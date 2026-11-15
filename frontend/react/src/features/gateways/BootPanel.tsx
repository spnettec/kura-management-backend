import { Alert, Button, Checkbox, Space } from "antd";
import { useRef, useState } from "react";
import { ApiError, api, type Gateway, type Session, type Task } from "../../api/client";
import { errorText, useI18n } from "../../i18n/I18n";

type Transaction = { id?: string; state: string; targetUnit?: string; retiredUnits?: string[] };
type View = { transaction: Transaction; confirmedPolicy: Transaction | null };
type Plan = { plan: Transaction; confirmToken: string; targetBefore: { unit: string; enablement: string; pid: number }; retiredBefore: { unit: string; enablement: string }[]; proposedActions: string[]; ordinaryGrantSha256: string; expiresAt: number };

export function BootPanel({ gateway, user }: { gateway: Gateway; user: Session }) {
  const { t } = useI18n();
  const [view, setView] = useState<View>();
  const [plan, setPlan] = useState<Plan>();
  const [busy, setBusy] = useState(false);
  const [confirmed, setConfirmed] = useState(false);
  const [error, setError] = useState<unknown>();
  const [taskId, setTaskId] = useState<string>();
  const [uncertain, setUncertain] = useState(false);
  const intent = useRef<{ signature: string; key: string } | undefined>(undefined);
  const storage = `yofc.boot.prepare:${user.id}:${gateway.id}`;
  const allowed = user.role === "admin" && gateway.online && gateway.capabilities.includes("ordinary.boot.v1");
  async function task(operation: string, action?: { transactionId: string; confirmationToken: string }) {
    const signature = JSON.stringify({ operation, action });
    if (!intent.current || intent.current.signature !== signature) intent.current = { signature, key: crypto.randomUUID() };
    const accepted = await api<{ id: string }>(`/gateways/${encodeURIComponent(gateway.id)}/tasks`, "POST",
      { operation, idempotencyKey: intent.current.key, ...(action ? { ordinaryBootAction: action } : {}) }, user.csrf);
    if (operation !== "ordinary.boot.status") setTaskId(accepted.id);
    if (operation === "ordinary.boot.prepare") sessionStorage.setItem(storage, accepted.id);
    const deadline = Date.now() + 35000;
    while (Date.now() < deadline) {
      const result = await api<Task>(`/tasks/${encodeURIComponent(accepted.id)}`);
      if (result.state === "UNKNOWN") {
        // Read this exact task while writes stay gated; never resend it.
        setUncertain(true); setError(new ApiError("OUTCOME_UNKNOWN", 0));
      } else if (!["QUEUED", "RUNNING"].includes(result.state)) {
        if (result.state !== "SUCCEEDED") throw new ApiError(result.error ?? result.state, 0);
        intent.current = undefined;
        setError(undefined);
        return result.result;
      }
      await new Promise(resolve => setTimeout(resolve, 500));
    }
    throw new ApiError("TASK_PENDING", 0);
  }
  async function load() {
    if (!allowed || busy) return;
    setBusy(true); setError(undefined);
    try {
      const next = await task("ordinary.boot.status") as View;
      if (!next?.transaction || !("confirmedPolicy" in next)) throw new ApiError("INTERNAL", 0);
      setView(next); setPlan(undefined); setConfirmed(false);
      if (next.transaction.state === "BOOT_POLICY_PREPARED") {
        const id = sessionStorage.getItem(storage);
        if (id) {
          const original = await api<Task>(`/tasks/${encodeURIComponent(id)}`);
          const candidate = original.result as unknown as Plan;
          if (original.gatewayId === gateway.id && original.operation === 16 && original.state === "SUCCEEDED" && candidate?.plan?.id === next.transaction.id) setPlan(candidate);
        }
      }
    } catch (e) { setError(e); } finally { setBusy(false); }
  }
  async function act(kind: "prepare" | "execute" | "cancel" | "reconcile") {
    if (!allowed || busy || ((kind === "execute" || kind === "reconcile") && !confirmed)) return;
    setBusy(true); setError(undefined);
    try {
      const action = (kind === "execute" || kind === "cancel") && plan?.plan.id
        ? { transactionId: plan.plan.id, confirmationToken: plan.confirmToken } : undefined;
      const result = await task(`ordinary.boot.${kind}`, action);
      if (kind === "prepare") { const next=result as unknown as Plan; setPlan(next); setView(v => ({transaction:next.plan,confirmedPolicy:v?.confirmedPolicy ?? null})); }
      else { setPlan(undefined); setView(v => ({transaction:result as unknown as Transaction,confirmedPolicy:kind==="execute"||kind==="reconcile" ? result as unknown as Transaction : v?.confirmedPolicy ?? null})); }
      setConfirmed(false); setUncertain(false);
    } catch (e) { setError(e); setUncertain(true); } finally { setBusy(false); }
  }
  const prepared = view?.transaction.state === "BOOT_POLICY_PREPARED";
  const settled = view && ["UNCONFIGURED", "CANCELLED", "BOOT_POLICY_COMMITTED"].includes(view.transaction.state);
  return <section aria-label={t("boot.region")}>
    <h3>{t("boot.title")}</h3><p className="muted">{t("boot.scope")}</p>
    {error != null && <Alert type="error" showIcon title={errorText(error, t)} />}
    {uncertain && <Alert type="warning" title={t("boot.unknown")} />}
    {taskId && <p>{t("boot.task")}: <code>{taskId}</code></p>}
    <Button disabled={!allowed || busy} loading={busy} onClick={() => void load()}>{t("boot.status")}</Button>
    {view && <>
      <p>{t("boot.latest")}: <code>{view.transaction.state}</code> {view.transaction.targetUnit}</p>
      <p>{t("boot.confirmedPolicy")}: {view.confirmedPolicy ? <><code>{view.confirmedPolicy.state}</code> {view.confirmedPolicy.targetUnit}</> : t("boot.none")}</p>
      <Button disabled={!allowed || busy || !settled || uncertain} onClick={() => void act("prepare")}>{t("boot.prepare")}</Button>
    </>}
    {plan && prepared && <>
      <p>{t("boot.target")}: {plan.targetBefore.unit} ({plan.targetBefore.enablement}, PID {plan.targetBefore.pid})</p>
      <p>{t("boot.retire")}: {plan.retiredBefore.map(v => `${v.unit} (${v.enablement})`).join(", ") || t("boot.none")}</p>
      <ul>{plan.proposedActions.map(v => <li key={v}><code>{v}</code></li>)}</ul>
      <p>{t("boot.grant")}: <code>{plan.ordinaryGrantSha256}</code></p>
      <p>{t("boot.expires")}: {new Date(plan.expiresAt).toLocaleString()}</p>
      <Checkbox disabled={busy} checked={confirmed} onChange={e => setConfirmed(e.target.checked)}>{t("boot.approve")}</Checkbox>
      <Space><Button disabled={!allowed || busy || !confirmed || uncertain || Date.now() >= plan.expiresAt} onClick={() => void act("execute")}>{t("boot.execute")}</Button>
        <Button disabled={!allowed || busy} onClick={() => void act("cancel")}>{t("boot.cancel")}</Button></Space>
    </>}
    {view && ["RECOVERY_REQUIRED", "APPLYING_BOOT_POLICY"].includes(view.transaction.state) && <>
      <Checkbox disabled={busy} checked={confirmed} onChange={e => setConfirmed(e.target.checked)}>{t("boot.reconcileWarning")}</Checkbox>
      <Button disabled={!allowed || busy || !confirmed} onClick={() => void act("reconcile")}>{t("boot.reconcile")}</Button>
    </>}
  </section>;
}
