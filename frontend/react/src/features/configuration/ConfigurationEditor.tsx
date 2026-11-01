import { Alert, Button, Checkbox, Input, Modal, Select, Space } from "antd";
import { useEffect, useRef, useState } from "react";
import { ApiError, api, type Gateway, type Session, type Task } from "../../api/client";
import { errorText, useI18n } from "../../i18n/I18n";
import { configurationPatch, configurationSubmission, type Configuration, type Property } from "./model";

export function ConfigurationEditor({ view, gateway, user }: { view: Configuration; gateway: Gateway; user: Session }) {
  const { t } = useI18n();
  const [open, setOpen] = useState(false);
  const [values, setValues] = useState<Record<string, string>>({});
  const [confirmed, setConfirmed] = useState(false);
  const [busy, setBusy] = useState(false);
  const [blocked, setBlocked] = useState(false);
  const [error, setError] = useState<unknown>();
  const [receipt, setReceipt] = useState<Task>();
  const [taskId, setTaskId] = useState<string>();
  const [uncertain, setUncertain] = useState(false);
  const request = useRef<AbortController | null>(null);
  useEffect(() => () => request.current?.abort(), []);
  const allowed = user.role === "admin" && gateway.online && gateway.capabilities.includes("kura.configuration.update") &&
    view.writable === true && view.updateEnabled === true && !!view.revision;
  const fields = view.properties.filter(field => field.editable === true && field.type !== "UNKNOWN");
  const selected = Object.keys(values);
  const isArray = (field: Property) => field.cardinality !== undefined && field.cardinality !== 0;
  function initial(field: Property): string {
    if (field.redacted || !field.present) return isArray(field) ? "[]" : field.type === "BOOLEAN" ? "false" : "";
    return isArray(field) ? JSON.stringify(field.value) : String(field.value ?? "");
  }
  function change(id: string, value: string) { setConfirmed(false); setValues(previous => ({ ...previous, [id]: value })); }
  function observe(task: Task) {
    if (task.id !== taskId && taskId !== undefined) throw new ApiError("PROTOCOL_MISMATCH", 0);
    configurationSubmission(task, gateway.id, view.pid, view.revision!);
    if (!["QUEUED", "RUNNING", "UNKNOWN"].includes(task.state)) setUncertain(false);
    setReceipt(task);
  }
  async function check() {
    if (!taskId) return;
    setBusy(true); setError(undefined);
    try { observe(await api<Task>(`/tasks/${encodeURIComponent(taskId)}`)); }
    catch (e) { setError(e); } finally { setBusy(false); }
  }
  async function save() {
    if (!allowed || busy || blocked || !confirmed) return;
    let patch;
    try { patch = configurationPatch(view, values); } catch (e) { setError(e); return; }
    const controller = new AbortController(); request.current = controller;
    setBusy(true); setError(undefined);
    let acceptedId: string | undefined;
    try {
      // One user-confirmed POST. No retry after an ambiguous HTTP response.
      const accepted = await api<{ id: string }>(`/gateways/${encodeURIComponent(gateway.id)}/tasks`, "POST",
        { operation: "kura.configuration.update", idempotencyKey: crypto.randomUUID(), kuraConfigurationUpdate: patch }, user.csrf, controller.signal);
      if (typeof accepted.id !== "string" || !accepted.id) throw new ApiError("PROTOCOL_MISMATCH", 0);
      acceptedId = accepted.id; setTaskId(accepted.id); setBlocked(true); setValues({}); setOpen(false);
      const deadline = Date.now() + 35000;
      while (Date.now() < deadline) {
        const task = await api<Task>(`/tasks/${encodeURIComponent(accepted.id)}`, "GET", undefined, undefined, controller.signal);
        if (task.id !== accepted.id) throw new ApiError("PROTOCOL_MISMATCH", 0);
        configurationSubmission(task, gateway.id, view.pid, view.revision!);
        setReceipt(task);
        if (!["QUEUED", "RUNNING"].includes(task.state)) break;
        await new Promise(resolve => setTimeout(resolve, 500));
      }
    } catch (e) {
      if (!controller.signal.aborted) {
        setError(e);
        // Even a server error may follow a committed admission. Keep drafts out
        // of retries; the user can inspect the task or the gateway task list.
        if (acceptedId || !(e instanceof ApiError) || e.status >= 500 || e.status === 0) {
          setUncertain(true); setBlocked(true); setValues({}); setOpen(false);
        }
      }
    } finally { if (!controller.signal.aborted) setBusy(false); }
  }
  const unknown = uncertain || receipt?.state === "UNKNOWN";
  const pending = receipt && ["QUEUED", "RUNNING"].includes(receipt.state);
  return <div className="configuration-editor">
    <p className="muted">{allowed ? t("configuration.writeScope") : t("configuration.writeDisabled")}</p>
    {error != null && <Alert type="error" showIcon title={errorText(error, t)} />}
    {(receipt || unknown) && <Alert showIcon type={unknown || pending ? "warning" : receipt?.state === "SUCCEEDED" ? "success" : "error"}
      title={unknown ? t("configuration.writeUnknown") : pending ? t("configuration.writePending") : receipt?.state === "SUCCEEDED"
        ? t("configuration.writeSubmitted") : errorText(new ApiError(receipt?.error ?? receipt?.state ?? "INTERNAL", 0), t)}
      description={<Space orientation="vertical">
        <span>{t("configuration.writeReadAgain")}</span>
        {taskId && <span>{t("configuration.taskId")}: {taskId}</span>}
        {taskId && <Button loading={busy} disabled={busy} onClick={() => void check()}>{t("configuration.checkSubmission")}</Button>}
      </Space>} />}
    <Button disabled={!allowed || blocked || busy || fields.length === 0} onClick={() => setOpen(true)}>{t("configuration.edit")}</Button>
    <Modal open={open} title={t("configuration.edit")} onCancel={() => { if (!busy) { setOpen(false); setValues({}); setConfirmed(false); } }}
      footer={<Space><Button disabled={busy} onClick={() => { setOpen(false); setValues({}); setConfirmed(false); }}>{t("configuration.cancel")}</Button>
        <Button type="primary" loading={busy} disabled={!confirmed || selected.length === 0 || blocked || busy} onClick={() => void save()}>{t("configuration.submit")}</Button></Space>}>
      <p>{t("configuration.writeKeep")}</p>
      <p className="muted">{t("configuration.writeValues")}</p>
      {error != null && <Alert type="error" showIcon title={errorText(error, t)} />}
      <div style={{ maxHeight: "48vh", overflowY: "auto" }}>
        {fields.map(field => <div key={field.id} style={{ marginBlock: 12 }}>
          <Checkbox disabled={busy} checked={Object.hasOwn(values, field.id)} onChange={e => {
            setConfirmed(false); setValues(previous => { if (e.target.checked) return { ...previous, [field.id]: initial(field) }; const next = { ...previous }; delete next[field.id]; return next; });
          }}>{field.name || field.id} · {field.id} · {t("configuration.setValue")}</Checkbox>
          {Object.hasOwn(values, field.id) && (isArray(field)
            ? <Input.TextArea aria-label={field.id} disabled={busy} value={values[field.id]} onChange={e => change(field.id, e.target.value)} />
            : field.type === "BOOLEAN" ? <Select aria-label={field.id} disabled={busy} value={values[field.id]} style={{ width: "100%" }}
              options={[{ value: "true", label: "true" }, { value: "false", label: "false" }]} onChange={value => change(field.id, value)} />
              : field.redacted ? <Input.Password aria-label={field.id} autoComplete="new-password" disabled={busy} value={values[field.id]} onChange={e => change(field.id, e.target.value)} />
                : <Input aria-label={field.id} disabled={busy} value={values[field.id]} onChange={e => change(field.id, e.target.value)} />)}
        </div>)}
      </div>
      <Checkbox checked={confirmed} disabled={busy || selected.length === 0} onChange={e => setConfirmed(e.target.checked)}>
        {t("configuration.confirmWrite", { pid: view.pid, count: selected.length })}
      </Checkbox>
    </Modal>
  </div>;
}
