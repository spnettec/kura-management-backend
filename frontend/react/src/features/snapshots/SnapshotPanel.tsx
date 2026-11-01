import { Alert, Button, Checkbox, Modal, Space } from "antd";
import { useState } from "react";
import { ApiError, api, type Gateway, type Session, type Task } from "../../api/client";
import { errorText, useI18n } from "../../i18n/I18n";

export function SnapshotPanel({ gateway, user }: { gateway: Gateway; user: Session }) {
  const { t } = useI18n();
  const [ids, setIds] = useState<string[]>();
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<unknown>();
  const [confirmed, setConfirmed] = useState(false);
  const [action, setAction] = useState<"save" | "rollback">();
  const [target, setTarget] = useState<string>();
  const allowed = user.role === "admin" && gateway.online && gateway.capabilities.includes("kura.snapshot.write");
  const latest = ids?.[0] ?? "0";
  async function load() {
    setBusy(true); setError(undefined);
    try {
      const accepted = await api<{ id: string }>(`/gateways/${encodeURIComponent(gateway.id)}/tasks`, "POST",
        { operation: "kura.snapshot.list", idempotencyKey: crypto.randomUUID() }, user.csrf);
      const deadline = Date.now() + 35000;
      while (Date.now() < deadline) {
        const task = await api<Task>(`/tasks/${encodeURIComponent(accepted.id)}`);
        if (!["QUEUED", "RUNNING"].includes(task.state)) {
          if (task.state !== "SUCCEEDED" || !Array.isArray(task.result)) throw new ApiError(task.error ?? task.state, 0);
          setIds([...new Set(task.result.map(String))].sort((a, b) => (BigInt(a) < BigInt(b) ? 1 : -1)));
          return;
        }
        await new Promise(resolve => setTimeout(resolve, 500));
      }
      throw new ApiError("TASK_PENDING", 0);
    } catch (e) { setError(e); } finally { setBusy(false); }
  }
  async function submit() {
    if (!allowed || !confirmed || !action || busy) return;
    setBusy(true); setError(undefined);
    try {
      await api(`/gateways/${encodeURIComponent(gateway.id)}/tasks`, "POST", {
        operation: action === "save" ? "kura.snapshot.save" : "kura.snapshot.rollback",
        idempotencyKey: crypto.randomUUID(),
        ...(action === "save" ? { kuraSnapshotSave: { expectedLatestId: latest } }
          : { kuraSnapshotRollback: { snapshotId: target, expectedLatestId: latest } }),
      }, user.csrf);
      setAction(undefined); setConfirmed(false); setIds(undefined);
    } catch (e) { setError(e); } finally { setBusy(false); }
  }
  return <section aria-label={t("snapshot.region")}>
    <h3>{t("snapshot.title")}</h3>
    <p className="muted">{t("snapshot.scope")}</p>
    {error != null && <Alert type="error" showIcon title={errorText(error, t)} />}
    <Space>
      <Button disabled={!gateway.online || busy} loading={busy} onClick={() => void load()}>{t("snapshot.list")}</Button>
      <Button disabled={!allowed || !ids || busy} onClick={() => { setAction("save"); setConfirmed(false); }}>{t("snapshot.save")}</Button>
    </Space>
    {ids && <ul>{ids.map(id => <li key={id}><Space>
      <span>{id}</span>
      <Button disabled={!allowed || id === latest || busy} onClick={() => { setTarget(id); setAction("rollback"); setConfirmed(false); }}>{t("snapshot.rollback")}</Button>
    </Space></li>)}</ul>}
    <Modal open={!!action} title={action === "save" ? t("snapshot.save") : t("snapshot.rollback")}
      onCancel={() => { if (!busy) { setAction(undefined); setConfirmed(false); } }}
      footer={<Space>
        <Button disabled={busy} onClick={() => { setAction(undefined); setConfirmed(false); }}>{t("configuration.cancel")}</Button>
        <Button type="primary" danger={action === "rollback"} disabled={!confirmed || busy} loading={busy} onClick={() => void submit()}>{t("configuration.submit")}</Button>
      </Space>}>
      <p>{action === "save" ? t("snapshot.saveWarning") : t("snapshot.rollbackWarning")}</p>
      <Checkbox checked={confirmed} disabled={busy} onChange={e => setConfirmed(e.target.checked)}>{t("snapshot.confirm")}</Checkbox>
    </Modal>
  </section>;
}
