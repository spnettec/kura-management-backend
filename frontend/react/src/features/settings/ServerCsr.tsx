import { Alert, Button, Checkbox, Space, Typography } from "antd";
import { useCallback, useEffect, useRef, useState } from "react";
import { api, ApiError, saveBlob } from "../../api/client";
import { setupApi } from "../../api/setup";
import { errorText, useI18n } from "../../i18n/I18n";

type Csr = { requestId: string; origin?: string; state: "READY" | "SIGNED" | "STAGED" | "INSTALLED" | "RETIRED" | "REJECTED" | "UNKNOWN";
  csrPem?: string; csrSha256?: string; certificatePem?: string; certificateFingerprint?: string; blockingRequestId?: string; certificateRejected?: boolean;
  importOperation?: { operationId: string; requestId: string; certificateSha256: string; state: "APPLIED" | "REJECTED" | "UNKNOWN"; code?: string } };
type Phase = "IDLE" | "CREATE" | "IMPORT" | "STAGE" | "RETIRE";
type Saved = { requestId: string; origin: string; phase: Phase; importOperationId?: string; certificateSha256?: string };
export type CsrSelection = { requestId: string; certificatePem: string };
const uuid = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;

export function ServerCsr({ setup = false, scope, origin, csrf, disabled = false, onChange, onStaged }:
  { setup?: boolean; scope: string; origin: string; csrf: string; disabled?: boolean;
    onChange?: (selection: CsrSelection | undefined, blocked: boolean) => void; onStaged?: () => void }) {
  const { t } = useI18n();
  const storageKey = `yofc.console.server-csr.v1:${setup ? "setup" : "settings"}:${scope}`;
  const [saved, setSaved] = useState<Saved | null>();
  const [record, setRecord] = useState<Csr>();
  const [busy, setBusy] = useState(false);
  const [confirmed, setConfirmed] = useState(false);
  const [error, setError] = useState<unknown>();
  const writing = useRef(false);
  const validOrigin = (() => { try { const value = new URL(origin); return value.protocol === "https:" && value.origin === origin; } catch { return false; } })();
  const request = useCallback(<T,>(path: string, method = "GET", body?: unknown) => setup
    ? setupApi<T>(`/server-csr${path}`, method, body, csrf)
    : api<T>(`/settings/server-csr${path}`, method, body, csrf), [setup, csrf]);
  const persist = useCallback((value: Saved) => {
    localStorage.setItem(storageKey, JSON.stringify(value)); // Public identity only, before dispatch.
    setSaved(value);
  }, [storageKey]);
  const accept = useCallback((value: Csr, identity: Saved) => {
    if (value.requestId !== identity.requestId || !["READY", "SIGNED", "STAGED", "INSTALLED", "RETIRED", "REJECTED", "UNKNOWN"].includes(value.state)) throw new ApiError("INVALID_ARGUMENT", 400);
    let importSettled = false;
    if (identity.phase === "IMPORT" && identity.importOperationId) {
      const receipt = value.importOperation;
      if (!receipt || receipt.operationId !== identity.importOperationId || receipt.requestId !== identity.requestId
        || receipt.certificateSha256 !== identity.certificateSha256 || !["APPLIED", "REJECTED", "UNKNOWN"].includes(receipt.state)
        || receipt.state === "APPLIED" && !["SIGNED", "STAGED", "INSTALLED", "UNKNOWN"].includes(value.state)) throw new ApiError("INVALID_ARGUMENT", 400);
      importSettled = receipt.state === "APPLIED" || receipt.state === "REJECTED";
    }
    setRecord(value);
    const settled = identity.phase === "CREATE" && ["READY", "SIGNED", "STAGED", "INSTALLED", "RETIRED", "REJECTED"].includes(value.state)
      || identity.phase === "IMPORT" && importSettled
      || identity.phase === "STAGE" && value.state === "STAGED"
      || identity.phase === "RETIRE" && value.state === "RETIRED";
    persist({ ...identity, phase: settled ? "IDLE" : identity.phase });
    setError(value.importOperation?.state === "REJECTED"
      ? new ApiError(value.importOperation.code === "INVALID_ARGUMENT" ? "INVALID_ARGUMENT" : "CONFLICT", 400)
      : value.certificateRejected ? new ApiError("INVALID_ARGUMENT", 400) : undefined);
  }, [persist]);
  const readOriginal = useCallback((identity: Saved) => request<Csr>(`/${identity.requestId}${identity.phase === "IMPORT" && identity.importOperationId ? `/imports/${identity.importOperationId}` : ""}`), [request]);
  const observe = useCallback(async (identity: Saved) => {
    try { accept(await readOriginal(identity), identity); }
    catch (e) { setError(e); } // Even 404 retains this identity and disables another POST.
  }, [readOriginal, accept]);
  useEffect(() => {
    let disposed = false;
    async function restore() {
      setSaved(undefined); setRecord(undefined); setConfirmed(false);
      try {
        const raw = localStorage.getItem(storageKey);
        if (raw) {
          const value: Saved = JSON.parse(raw);
          if (!uuid.test(value.requestId) || typeof value.origin !== "string" || !["IDLE", "CREATE", "IMPORT", "STAGE", "RETIRE"].includes(value.phase)) throw new ApiError("INVALID_ARGUMENT", 400);
            if (!(value.importOperationId === undefined && value.certificateSha256 === undefined)
              && (!value.importOperationId || !uuid.test(value.importOperationId) || !/^[0-9a-f]{64}$/.test(value.certificateSha256 ?? ""))) throw new ApiError("INVALID_ARGUMENT", 400);
          if (disposed) return;
          setSaved(value);
            const result = await readOriginal(value);
          if (!disposed) accept(result, value);
        } else {
          const { current } = await request<{ current: Csr | null }>("");
          if (disposed) return;
          if (current) {
            const identity: Saved = { requestId: current.requestId, origin: current.origin ?? "", phase: "IDLE" };
            accept(current, identity);
          } else setSaved(null);
        }
      } catch (e) { if (!disposed) setError(e); }
    }
    void restore();
    const changed = (event: StorageEvent) => { if (event.key === storageKey) void restore(); };
    window.addEventListener("storage", changed);
    return () => { disposed = true; window.removeEventListener("storage", changed); };
  }, [storageKey, request, accept, readOriginal]);
  const uncertain = !!saved && (!record || record.state === "UNKNOWN" || saved.phase !== "IDLE");
  const terminal = !!record && ["STAGED", "INSTALLED", "RETIRED", "REJECTED"].includes(record.state);
  const selected = record?.state === "SIGNED" && !uncertain && record.origin === origin && !!record.certificatePem;
  useEffect(() => {
    onChange?.(selected && record?.certificatePem ? { requestId: record.requestId, certificatePem: record.certificatePem } : undefined,
      saved === undefined || !!saved && !terminal || busy);
  }, [onChange, selected, record, saved, terminal, busy]);
  async function generate() {
    if (saved !== null || disabled || busy || writing.current || !validOrigin) return;
    const identity: Saved = { requestId: crypto.randomUUID(), origin, phase: "CREATE" };
    writing.current = true; setBusy(true); setConfirmed(false); setError(undefined);
    try { persist(identity); accept(await request<Csr>("", "POST", { requestId: identity.requestId, origin }), identity); }
    catch (e) { setError(e); } finally { writing.current = false; setBusy(false); }
  }
  async function mutate(phase: "IMPORT" | "STAGE" | "RETIRE", body?: unknown) {
    if (!saved || !record || uncertain || busy || writing.current || disabled) return;
    let identity = { ...saved, phase }, payload = body;
    const suffix = phase === "IMPORT" ? "certificate" : phase === "STAGE" ? "stage" : "retire";
    writing.current = true; setBusy(true); setConfirmed(false); setError(undefined);
    try {
      if (phase === "IMPORT") {
        const serverPem = (body as { serverPem: string }).serverPem, operationId = crypto.randomUUID();
        payload = { serverPem, operationId };
        const encoder = new TextEncoder();
        if (encoder.encode(JSON.stringify(payload)).length > 65536) throw new ApiError("FILE_TOO_LARGE", 413);
        const digest = await crypto.subtle.digest("SHA-256", encoder.encode(serverPem));
        identity = { ...identity, importOperationId: operationId,
          certificateSha256: Array.from(new Uint8Array(digest), byte => byte.toString(16).padStart(2, "0")).join("") };
      }
      persist(identity);
      const value = await request<Csr>(`/${identity.requestId}/${suffix}`, phase === "IMPORT" ? "PUT" : "POST", payload);
      accept(value, identity);
      if (phase === "STAGE" && value.state === "STAGED") onStaged?.();
    } catch (e) { setError(e); } finally { writing.current = false; setBusy(false); }
  }
  function finish() {
    if (!terminal || !saved || saved.phase !== "IDLE" || busy) return;
    try { localStorage.removeItem(storageKey); setSaved(undefined); setRecord(undefined); setError(undefined); }
    catch (e) { setError(e); return; }
    // Inspect any other administrator's original request before enabling creation.
    void request<{ current: Csr | null }>("").then(({ current }) => {
      if (current) accept(current, { requestId: current.requestId, origin: current.origin ?? "", phase: "IDLE" });
      else setSaved(null);
    }).catch(setError);
  }
  return <Space direction="vertical" style={{ width: "100%" }}>
    <Alert type="info" title={t("csr.description")} />
    <p>{t("installation.origin")}: <Typography.Text code>{record?.origin ?? origin}</Typography.Text></p>
    {!validOrigin && <Alert type="warning" title={t("csr.originInvalid")} />}
    {saved === null && <Button type="primary" disabled={busy || disabled || !validOrigin} onClick={() => void generate()}>{t("csr.generate")}</Button>}
    {saved && <>
      <Typography.Text copyable code>{saved.requestId}</Typography.Text>
        {saved.importOperationId && <p>{t("csr.importOperation")} <Typography.Text copyable code>{saved.importOperationId}</Typography.Text></p>}
        {saved.importOperationId && <p className="muted">{t("csr.uploadReceiptHint")}</p>}
      <Alert type={uncertain ? "warning" : record?.state === "SIGNED" || record?.state === "STAGED" || record?.state === "INSTALLED" ? "success" : "info"}
        title={t(uncertain ? "csr.UNKNOWN" : `csr.${record?.state ?? "UNKNOWN"}`)} />
      <Button disabled={busy} onClick={() => void observe(saved)}>{t("csr.observe")}</Button>
      {record?.csrPem && <Button disabled={busy} onClick={() => saveBlob(new Blob([record.csrPem!], { type: "application/pkcs10" }), `console-${saved.requestId}.csr`)}>{t("csr.download")}</Button>}
      {record?.state === "READY" && <label>{t("csr.upload")}<input type="file" aria-label={t("csr.upload")} accept=".pem,.crt,.cer" disabled={busy || disabled || uncertain} onChange={async e => {
        const input = e.currentTarget, file = input.files?.[0]; if (!file) return;
        setBusy(true); setError(undefined);
        let value: string;
        try { if (file.size > 65536) throw new ApiError("FILE_TOO_LARGE", 413); value = await file.text(); if (new TextEncoder().encode(value).length > 65536) throw new ApiError("FILE_TOO_LARGE", 413); }
        catch (error) { setError(error); return; } finally { input.value = ""; setBusy(false); }
        await mutate("IMPORT", { serverPem: value });
      }} /></label>}
      {record?.certificateFingerprint && <p>{t("settings.fingerprint")}<br /><Typography.Text code>{record.certificateFingerprint}</Typography.Text></p>}
      {record?.origin && record.origin !== origin && <Alert type="warning" title={t("csr.originChanged")} />}
      {!setup && selected && <>
        <Checkbox checked={confirmed} disabled={busy || disabled} onChange={e => setConfirmed(e.target.checked)}>{t("settings.tls.confirm")}</Checkbox>
        <Button type="primary" disabled={busy || disabled || !confirmed} onClick={() => void mutate("STAGE")}>{t("settings.tls.submit")}</Button>
      </>}
      {!uncertain && ["READY", "SIGNED"].includes(record?.state ?? "") && <>
        <p className="muted">{t("csr.retireHint")}</p>
        <Button disabled={busy || disabled} onClick={() => void mutate("RETIRE")}>{t("csr.retire")}</Button>
      </>}
      {terminal && saved.phase === "IDLE" && <Button disabled={busy} onClick={finish}>{t("csr.finish")}</Button>}
    </>}
    {saved === undefined && <Button disabled={busy} onClick={() => location.reload()}>{t("csr.observe")}</Button>}
    {error !== undefined && <Alert type="error" title={errorText(error, t)} />}
  </Space>;
}
