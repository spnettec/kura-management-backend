import { Alert, Button, Checkbox, Space } from "antd";
import { useState } from "react";
import { ApiError, api, type Gateway, type Session, type Task } from "../../api/client";
import { errorText, useI18n } from "../../i18n/I18n";

type Plan = { transactionId: string; confirmToken: string; state: string; unit: string; profileId: string; profileDigest: string; grantSha256: string; sourceJournalSha256: string; snapshotSha256: string; componentPids: string[]; checkIds: string[]; expiresAtUnixMs: number };
type View = { state: string | null; canPrepare: boolean; completionId: string | null; ordinary: { id: string; unit: string; ordinaryUnitHandedOff: boolean } | null };
type Pending = { kind: "prepare" | "execute" | "cancel"; key: string; id?: string };

export function HandoffPanel({ gateway, user }: { gateway: Gateway; user: Session }) {
  const { t } = useI18n();
  const storage = `yofc.handoff:${user.id}:${gateway.id}`;
  const [view, setView] = useState<View>();
  const [plan, setPlan] = useState<Plan>();
  const [busy, setBusy] = useState(false);
  const [approved, setApproved] = useState(false);
  const [error, setError] = useState<unknown>();
  const [taskId, setTaskId] = useState<string>();
  const [uncertain, setUncertain] = useState(() => !!sessionStorage.getItem(storage));
  const allowed = user.role === "admin" && gateway.online && gateway.capabilities.includes("recovery.ordinary.v1");

  function accept(task: Task, kind: Pending["kind"]) {
    if (task.gatewayId !== gateway.id || task.operation !== (kind === "prepare" ? 21 : kind === "cancel" ? 29 : 22)) throw new ApiError("CONFLICT", 0);
    if (["QUEUED", "RUNNING", "UNKNOWN"].includes(task.state)) return false;
    if (task.state !== "SUCCEEDED") {
      sessionStorage.removeItem(storage); setUncertain(false);
      throw new ApiError(task.error ?? task.state, 0);
    }
    const v = task.result as unknown as Plan & { ordinaryUnitHandedOff?: boolean; cancelled?: boolean; issued?: boolean; scope?: string; kuraRecoveryVerified?: boolean; normalBootVerified?: boolean };
    if (v?.scope !== "administrator-selected-checks" || v.kuraRecoveryVerified !== false || v.normalBootVerified !== false) throw new ApiError("CONFLICT", 0);
    if (kind === "prepare") {
      if (v.state !== "ORDINARY_PREPARED" || !v.transactionId || !v.confirmToken || !Array.isArray(v.checkIds) || !Array.isArray(v.componentPids)) throw new ApiError("CONFLICT", 0);
      setPlan(v); setView(old => ({ ...old, state: v.state, canPrepare: false, completionId: old?.completionId ?? null, ordinary: { id: v.transactionId, unit: v.unit, ordinaryUnitHandedOff: false } }));
    } else {
      if (kind === "cancel" ? v.state !== "ORDINARY_CANCELLED" || v.cancelled !== true || v.issued !== false || v.ordinaryUnitHandedOff !== false : v.state !== "ORDINARY_ACTIVE_APPROVED_CHECKS" || v.ordinaryUnitHandedOff !== true) throw new ApiError("CONFLICT", 0);
      setPlan(undefined); setView(old => old ? { ...old, state: v.state, canPrepare: kind === "cancel" } : undefined);
    }
    sessionStorage.removeItem(storage); setUncertain(false);
    setApproved(false); return true;
  }
  async function readTask(id: string, kind: Pending["kind"], wait: boolean) {
    setTaskId(id);
    const deadline = Date.now() + 240000;
    do {
      const value = await api<Task>(`/tasks/${encodeURIComponent(id)}`);
      if (accept(value, kind)) return;
      setUncertain(true);
      if (!wait) return;
      await new Promise(resolve => setTimeout(resolve, 750));
    } while (Date.now() < deadline);
    throw new ApiError("TASK_PENDING", 0);
  }
  async function load() {
    if (!allowed || busy) return;
    setBusy(true); setError(undefined); setApproved(false);
    try {
      const pending = sessionStorage.getItem(storage);
      if (pending) {
        const p = JSON.parse(pending) as Pending;
        if (!p.id && p.key && ["prepare", "execute", "cancel"].includes(p.kind)) {
          try {
            const original = await api<{ id: string }>(`/gateways/${encodeURIComponent(gateway.id)}/task-intents/${encodeURIComponent(p.key)}`);
            p.id = original.id; sessionStorage.setItem(storage, JSON.stringify(p));
            if (p.kind === "prepare") sessionStorage.setItem(`${storage}:prepare`, original.id);
          } catch (e) { if (!(e instanceof ApiError && e.status === 404)) throw e; }
        }
        if (p.id && ["prepare", "execute", "cancel"].includes(p.kind)) await readTask(p.id, p.kind, false);
      }
      const { id } = await api<{ id: string }>(`/gateways/${encodeURIComponent(gateway.id)}/tasks`, "POST", { operation: "recovery.ordinary.status", idempotencyKey: crypto.randomUUID() }, user.csrf);
      const deadline = Date.now() + 35000;
      let next: View | undefined;
      while (Date.now() < deadline) {
        const result = await api<Task>(`/tasks/${encodeURIComponent(id)}`);
        if (result.state === "SUCCEEDED") { next = result.result as unknown as View; break; }
        if (!["QUEUED", "RUNNING"].includes(result.state)) throw new ApiError(result.error ?? result.state, 0);
        await new Promise(resolve => setTimeout(resolve, 500));
      }
      if (!next || typeof next.canPrepare !== "boolean") throw new ApiError("TASK_PENDING", 0);
      setView(next); setPlan(undefined);
      const prepared = sessionStorage.getItem(`${storage}:prepare`);
      if (next.state === "ORDINARY_PREPARED" && prepared && !sessionStorage.getItem(storage)) {
        const original = await api<Task>(`/tasks/${encodeURIComponent(prepared)}`);
        const value = original.result as unknown as Plan;
        if (value?.transactionId === next.ordinary?.id) accept(original, "prepare");
      }
    } catch (e) { setError(e); } finally { setBusy(false); }
  }
  async function act(kind: Pending["kind"]) {
    if (!allowed || busy || uncertain || (kind === "execute" && (!approved || !plan || Date.now() >= plan.expiresAtUnixMs))) return;
    if (kind === "prepare" && !view?.canPrepare) return;
    if (kind !== "prepare" && (!plan || view?.state !== "ORDINARY_PREPARED" || view.ordinary?.id !== plan.transactionId)) return;
    if (kind === "cancel" && !gateway.capabilities.includes("recovery.ordinary.cancel.v1")) return;
    setBusy(true); setError(undefined);
    const pending: Pending = { kind, key: crypto.randomUUID() };
    // Store only an intent and task ID. A lost HTTP acknowledgement stays gated
    // across reload; no token or replacement command is placed in web storage.
    sessionStorage.setItem(storage, JSON.stringify(pending)); setUncertain(true);
    try {
      const body = { operation: `recovery.ordinary.${kind}`, idempotencyKey: pending.key,
        ...(kind !== "prepare" ? { ordinaryHandoffAction: { transactionId: plan!.transactionId, confirmationToken: plan!.confirmToken } } : {}) };
      const { id } = await api<{ id: string }>(`/gateways/${encodeURIComponent(gateway.id)}/tasks`, "POST", body, user.csrf);
      pending.id = id; sessionStorage.setItem(storage, JSON.stringify(pending));
      if (kind === "prepare") sessionStorage.setItem(`${storage}:prepare`, id);
      await readTask(id, kind, true);
    } catch (e) {
      setError(e);
      if (!pending.id && e instanceof ApiError && e.status >= 400 && e.status < 500 && e.status !== 408) {
        sessionStorage.removeItem(storage); setUncertain(false);
      }
    } finally { setBusy(false); }
  }
  return <section aria-label={t("handoff.region")}>
    <h3>{t("handoff.title")}</h3><p className="muted">{t("handoff.scope")}</p>
    {error != null && <Alert type="error" showIcon title={errorText(error, t)} />}
    {uncertain && <Alert type="warning" title={t("handoff.unknown")} />}
    {taskId && <p>{t("boot.task")}: <code>{taskId}</code></p>}
    <Button disabled={!allowed || busy} loading={busy} onClick={() => void load()}>{t("handoff.status")}</Button>
    {view && <><p>{t("boot.latest")}: <code>{view.state ?? t("boot.none")}</code></p>
      <p>{t("handoff.parent")}: <code>{view.completionId ?? t("boot.none")}</code></p>
      <Button disabled={!allowed || busy || uncertain || !view.canPrepare} onClick={() => void act("prepare")}>{t("handoff.prepare")}</Button>
    </>}
    {plan && view?.state === "ORDINARY_PREPARED" && <>
      <p>{t("boot.target")}: {plan.unit}</p><p>{t("handoff.profile")}: <code>{plan.profileId}</code></p>
      <p>{t("boot.grant")}: <code>{plan.grantSha256}</code></p><p>{t("handoff.snapshot")}: <code>{plan.snapshotSha256}</code></p>
      <p>{t("handoff.components")}: {plan.componentPids.join(", ")}</p><p>{t("handoff.checks")}: {plan.checkIds.join(", ")}</p>
      <p>{t("boot.expires")}: {new Date(plan.expiresAtUnixMs).toLocaleString()}</p>
      <Checkbox disabled={busy || uncertain} checked={approved} onChange={e => setApproved(e.target.checked)}>{t("handoff.approve")}</Checkbox>
        <Space><Button disabled={!allowed || busy || uncertain || !approved || Date.now() >= plan.expiresAtUnixMs} onClick={() => void act("execute")}>{t("handoff.execute")}</Button>
          {gateway.capabilities.includes("recovery.ordinary.cancel.v1") && <Button disabled={!allowed || busy || uncertain} onClick={() => void act("cancel")}>{t("handoff.cancel")}</Button>}
        </Space>
    </>}
  </section>;
}
