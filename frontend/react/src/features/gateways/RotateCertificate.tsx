import { Alert, Button, Checkbox, Input, Modal, Radio, Space, Tag } from "antd";
import { useEffect, useRef, useState } from "react";
import { api, apiDownload, ApiError, certificateFingerprint, saveBlob, type Gateway, type Session } from "../../api/client";
import { errorText, LanguageSwitcher, useI18n } from "../../i18n/I18n";

type Intent = {
  operationId: string;
  id: string;
  previousCertificateSha256: string;
  previousBindingGeneration?: number;
  certificateSha256: string;
  certificatePemSha256: string;
  bundleRequested: boolean;
  bundleDownloadStarted?: boolean;
  bundleReceived?: boolean;
};
type RotationReceipt = {
  id: string;
  operationId: string;
  state: "APPLIED" | "REJECTED" | "UNKNOWN";
  previousCertificateSha256?: string;
  certificateSha256?: string | null;
  certificatePemSha256?: string;
  bundleRequested?: boolean;
  revision?: number | null;
  rejectionCode?: string | null;
  identityBundleToken?: string;
  receiptVersion?: number;
  previousBindingGeneration?: number | null;
  bindingGeneration?: number | null;
};
const uuid = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/;
const digest = /^[0-9a-f]{64}$/;
function validIntent(value: Intent, id: string): boolean {
  return !!value && uuid.test(value.operationId) && value.id === id
    && [value.previousCertificateSha256, value.certificateSha256, value.certificatePemSha256].every(v => typeof v === "string" && digest.test(v))
    && typeof value.bundleRequested === "boolean"
    && (value.previousBindingGeneration === undefined || safeGeneration(value.previousBindingGeneration))
    && (value.bundleDownloadStarted === undefined || typeof value.bundleDownloadStarted === "boolean")
    && (value.bundleReceived === undefined || value.bundleReceived === true && value.bundleDownloadStarted === true);
}
function safeGeneration(value: unknown): value is number {
  return typeof value === "number" && Number.isSafeInteger(value) && value >= 1 && value < Number.MAX_SAFE_INTEGER;
}
function generationReceipt(value: RotationReceipt): boolean {
  return value.receiptVersion === 2 && safeGeneration(value.previousBindingGeneration)
    && value.bindingGeneration === value.previousBindingGeneration + 1;
}

export function RotateCertificate({ gateway, user }: { gateway: Gateway; user: Session }) {
  const { t } = useI18n();
  const storageKey = JSON.stringify(["yofc.gateway-certificate-operation.v1", user.tenantId, user.id, gateway.id]);
  const [open, setOpen] = useState(false);
  const [pem, setPem] = useState("");
  const [key, setKey] = useState("");
  const [withKey, setWithKey] = useState(false);
  const [fingerprint, setFingerprint] = useState("");
  const [reviewedPem, setReviewedPem] = useState("");
  const reviewSequence = useRef(0);
  const [currentFingerprint, setCurrentFingerprint] = useState(gateway.certificateSha256);
  const [currentGeneration, setCurrentGeneration] = useState(gateway.bindingGeneration);
  const [confirmed, setConfirmed] = useState(false);
  const [busy, setBusy] = useState(false);
  const [restoring, setRestoring] = useState(true);
  const [uncertain, setUncertain] = useState(false);
  const [intent, setIntent] = useState<Intent | null>(null);
  const intentRef = useRef<Intent | null>(null);
  const writer = useRef(false);
  const epoch = useRef(0);
  const [receipt, setReceipt] = useState<RotationReceipt>();
  const [error, setError] = useState<unknown>();
  const [storageError, setStorageError] = useState(false);
  const frozen = busy || restoring || intent !== null || uncertain;
  const delivered = intent?.bundleReceived === true;
  const bundleUnavailable = receipt?.state === "APPLIED" && intent?.bundleRequested && !receipt.identityBundleToken && !delivered;

  function isCurrentOperation(operationId: string): boolean {
    return intentRef.current?.operationId === operationId;
  }

  function persist(value: Intent) {
    localStorage.setItem(storageKey, JSON.stringify(value)); // Public identity only, before every dispatch.
    intentRef.current = value;
    setIntent(value);
  }
  function accept(value: RotationReceipt, identity: Intent) {
    if (value.id !== identity.id || value.operationId !== identity.operationId || !["APPLIED", "REJECTED", "UNKNOWN"].includes(value.state)) throw new Error("receipt mismatch");
    if (value.state === "UNKNOWN") { setReceipt(undefined); setUncertain(true); return; }
    if (value.previousCertificateSha256 !== identity.previousCertificateSha256 || value.certificatePemSha256 !== identity.certificatePemSha256
      || value.bundleRequested !== identity.bundleRequested
      || value.state === "APPLIED" && (value.certificateSha256 !== identity.certificateSha256 || !Number.isSafeInteger(value.revision) || Number(value.revision) < 1)
      || value.state === "REJECTED" && value.certificateSha256 != null && value.certificateSha256 !== identity.certificateSha256) throw new Error("receipt mismatch");
    if (identity.previousBindingGeneration !== undefined && (!generationReceipt(value)
      || value.previousBindingGeneration !== identity.previousBindingGeneration)) throw new Error("generation receipt mismatch");
    if (value.identityBundleToken !== undefined && (!identity.bundleRequested || typeof value.identityBundleToken !== "string" || !/^[A-Za-z0-9_-]{20,512}$/.test(value.identityBundleToken))) throw new Error("receipt mismatch");
    setReceipt(value);
    setUncertain(false);
    if (value.state === "REJECTED") setError(new ApiError(value.rejectionCode ?? "CONFLICT", 409));
  }
  async function readOriginal(identity: Intent, selectedEpoch: number) {
    const value = await api<RotationReceipt>(`/gateways/${encodeURIComponent(identity.id)}/certificate-operations/${identity.operationId}`);
    if (epoch.current !== selectedEpoch || intentRef.current?.operationId !== identity.operationId) return;
    accept(value, identity);
  }
  useEffect(() => {
    let disposed = false;
    async function restore() {
      const selectedEpoch = ++epoch.current;
      setRestoring(true); setReceipt(undefined); setConfirmed(false); setError(undefined); setStorageError(false);
      try {
        const raw = localStorage.getItem(storageKey);
        if (raw) {
          const value: Intent = JSON.parse(raw);
          if (!validIntent(value, gateway.id)) throw new Error("invalid saved operation");
          intentRef.current = value; setIntent(value); setFingerprint(value.certificateSha256); setWithKey(value.bundleRequested); setUncertain(true);
          await readOriginal(value, selectedEpoch);
        } else { intentRef.current = null; setIntent(null); setUncertain(false); }
        if (!disposed && epoch.current === selectedEpoch) setRestoring(false);
      } catch (failure) {
        if (!disposed && epoch.current === selectedEpoch) { setUncertain(true); setError(failure); setRestoring(false); }
      }
    }
    void restore();
    const changed = (event: StorageEvent) => { if (event.key === storageKey) void restore(); };
    window.addEventListener("storage", changed);
    return () => { disposed = true; epoch.current++; window.removeEventListener("storage", changed); };
    // A session/gateway change invalidates all old asynchronous receipt reads.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [storageKey, gateway.id]);

  function edit(change: () => void) { reviewSequence.current++; change(); setFingerprint(""); setReviewedPem(""); setConfirmed(false); setError(undefined); }
  async function review() {
    const selected = ++reviewSequence.current, source = pem;
    setError(undefined); setConfirmed(false);
    try {
      const reviewed = (await certificateFingerprint(source)) ?? "";
      if (selected !== reviewSequence.current) return;
      setReviewedPem(source);
      setFingerprint(reviewed);
      if (!reviewed) setError(new ApiError("INVALID_ARGUMENT", 400));
    } catch (failure) { if (selected === reviewSequence.current) { setFingerprint(""); setReviewedPem(""); setError(failure); } }
  }
  async function rotate() {
    if (writer.current || !confirmed || !fingerprint || reviewedPem !== pem || frozen || intentRef.current) return;
    writer.current = true; setBusy(true); setConfirmed(false); setError(undefined);
    let published = false;
    try {
      if (localStorage.getItem(storageKey) !== null) throw new Error("saved operation exists");
      const previous = currentFingerprint;
      if (typeof previous !== "string" || !digest.test(previous)) throw new Error("invalid gateway baseline");
      if (!withKey && !safeGeneration(currentGeneration)) throw new Error("invalid gateway generation");
      const hash = await crypto.subtle.digest("SHA-256", new TextEncoder().encode(pem));
      const identity: Intent = { operationId: crypto.randomUUID(), id: gateway.id, previousCertificateSha256: previous,
        previousBindingGeneration: withKey ? undefined : currentGeneration,
        certificateSha256: fingerprint, certificatePemSha256: Array.from(new Uint8Array(hash), v => v.toString(16).padStart(2, "0")).join(""), bundleRequested: withKey };
      if (!validIntent(identity, gateway.id)) throw new Error("invalid operation identity");
      persist(identity); published = true;
      const body: Record<string, unknown> = { operationId: identity.operationId, previousCertificateSha256: identity.previousCertificateSha256,
        previousBindingGeneration: identity.previousBindingGeneration, clientCertificatePem: pem };
      if (withKey) body.clientKeyPem = key;
      const submittedEpoch = epoch.current;
      const value = await api<RotationReceipt>(`/gateways/${encodeURIComponent(gateway.id)}/certificate`, "POST", body, user.csrf);
      if (submittedEpoch === epoch.current && isCurrentOperation(identity.operationId)) accept(value, identity);
    } catch (failure) {
      if (published) setUncertain(true);
      setStorageError(!published); setError(failure);
    } finally { setKey(""); setBusy(false); writer.current = false; }
  }
  async function reconcile() {
    const identity = intentRef.current;
    if (writer.current || !identity) return;
    writer.current = true; setBusy(true); setError(undefined);
    try { await readOriginal(identity, epoch.current); }
    catch (failure) { setUncertain(true); setError(failure); }
    finally { setBusy(false); writer.current = false; }
  }
  async function downloadBundle() {
    const identity = intentRef.current;
    if (writer.current || !identity || identity.bundleDownloadStarted || !receipt?.identityBundleToken) return;
    writer.current = true; setBusy(true); setError(undefined);
    try {
      persist({ ...identity, bundleDownloadStarted: true });
      const query = `token=${encodeURIComponent(receipt.identityBundleToken)}`;
      const blob = await apiDownload(`/gateways/${encodeURIComponent(receipt.id)}/identity-bundle?${query}`);
      saveBlob(blob, `${receipt.id}-rotation-bundle.tar`);
      persist({ ...identity, bundleDownloadStarted: true, bundleReceived: true });
    } catch (failure) { setError(failure); }
    finally { setBusy(false); writer.current = false; }
  }
  function downloadReceipt() {
    if (busy || uncertain || receipt?.state !== "APPLIED" || !generationReceipt(receipt) || !intent
      || !isCurrentOperation(receipt.operationId)) return;
    // A public allowlist also strips the one-time private token from POST responses.
    const value = { schema: "yofc.console.gateway-certificate-rotation.v2", receiptVersion: 2,
      id: receipt.id, operationId: receipt.operationId, state: receipt.state,
      previousCertificateSha256: receipt.previousCertificateSha256, certificateSha256: receipt.certificateSha256,
      certificatePemSha256: receipt.certificatePemSha256, previousBindingGeneration: receipt.previousBindingGeneration,
      bindingGeneration: receipt.bindingGeneration, revision: receipt.revision, bundleRequested: receipt.bundleRequested,
      identityBundleTokenRecoverable: false };
    saveBlob(new Blob([JSON.stringify(value, null, 2) + "\n"], { type: "application/json" }), `${receipt.id}-${receipt.operationId}-certificate-receipt.json`);
  }
  async function again() {
    if (writer.current || uncertain || !receipt || receipt.state === "UNKNOWN" || bundleUnavailable || receipt.state === "APPLIED" && intent?.bundleRequested && !delivered) return;
    writer.current = true; setBusy(true); setError(undefined);
    const originalId = intentRef.current?.operationId, selectedEpoch = epoch.current;
    try {
      // This read selects the baseline for a NEW confirmed operation; it never settles the previous one.
      const current = await api<Gateway>(`/gateways/${encodeURIComponent(gateway.id)}`);
      if (typeof current.certificateSha256 !== "string" || !digest.test(current.certificateSha256)) throw new Error("invalid gateway fingerprint");
      if (!safeGeneration(current.bindingGeneration)) throw new Error("invalid gateway generation");
      if (selectedEpoch !== epoch.current || intentRef.current?.operationId !== originalId) return;
      localStorage.removeItem(storageKey); epoch.current++;
      intentRef.current = null; setIntent(null); setReceipt(undefined); setCurrentFingerprint(current.certificateSha256);
      setCurrentGeneration(current.bindingGeneration);
      reviewSequence.current++; setReviewedPem(""); setPem(""); setKey(""); setFingerprint(""); setConfirmed(false); setUncertain(false);
    } catch (failure) { setError(failure); }
    finally { setBusy(false); writer.current = false; }
  }
  return <>
    <Button onClick={() => setOpen(true)}>{t("rotation.open")}</Button>
    <Modal title={t("rotation.open")} open={open} onCancel={() => setOpen(false)} footer={null} destroyOnHidden={false} closable={!busy} maskClosable={!busy}>
      <LanguageSwitcher />
      <p className="muted">{t("rotation.description")}</p>
      <Space direction="vertical" style={{ width: "100%" }}>
        <p className="muted" style={{ overflowWrap: "anywhere" }}>{t(intent ? "rotation.previous" : "rotation.current")}<br /><code>{intent?.previousCertificateSha256 ?? currentFingerprint}</code></p>
        <Radio.Group value={withKey} disabled={frozen} onChange={e => edit(() => setWithKey(e.target.value))}>
          <Radio value={false}>{t("rotation.modePem")}</Radio><Radio value={true}>{t("rotation.modeBundle")}</Radio>
        </Radio.Group>
        <label>{t("rotation.newCertificate")}<Input.TextArea aria-label={t("rotation.newCertificate")} value={pem} disabled={frozen} maxLength={32768} rows={6} onChange={e => edit(() => setPem(e.target.value))} /></label>
        {withKey && <label>{t("rotation.newKey")}<Input.TextArea aria-label={t("rotation.newKey")} value={key} disabled={frozen} maxLength={32768} rows={5} onChange={e => edit(() => setKey(e.target.value))} /></label>}
        <Button disabled={frozen || !pem || withKey && !key} onClick={() => void review()}>{t("settings.review")}</Button>
        {fingerprint && <p>{t("settings.fingerprint")}<br /><code style={{ overflowWrap: "anywhere" }}>{fingerprint}</code></p>}
        <Checkbox checked={confirmed} disabled={!fingerprint || reviewedPem !== pem || frozen} onChange={e => setConfirmed(e.target.checked)}>{t("rotation.confirm")}</Checkbox>
        <Button type="primary" aria-label={t("rotation.submit")} aria-busy={busy} disabled={!confirmed || !fingerprint || reviewedPem !== pem || frozen} loading={busy} onClick={() => void rotate()}>{t("rotation.submit")}</Button>
        {intent && <p>{t("rotation.operation")}<br /><code>{intent.operationId}</code></p>}
        {error !== undefined && <Alert type="error" title={storageError ? t("rotation.storageFailed") : errorText(error, t)} />}
        {uncertain && <><Alert type="warning" title={t("rotation.uncertain")} /><Button disabled={busy || !intent} onClick={() => void reconcile()}>{t("registration.reconcile")}</Button></>}
        {receipt?.state === "APPLIED" && !intent?.bundleRequested && <Alert type="success" title={t("rotation.doneCenterOnly")} description={<code style={{ overflowWrap: "anywhere" }}>{receipt.certificateSha256}</code>} />}
        {receipt?.state === "REJECTED" && <Alert type="info" title={t("rotation.rejected")} />}
        {receipt?.state === "APPLIED" && generationReceipt(receipt) && <Button disabled={busy || uncertain} onClick={downloadReceipt}>{t("rotation.downloadReceipt")}</Button>}
        {receipt?.state === "APPLIED" && !generationReceipt(receipt) && <Alert type="warning" title={t("rotation.legacyReceipt")} />}
        {bundleUnavailable && <Alert type="warning" title={t("rotation.bundleUnavailable")} />}
        {receipt?.state === "APPLIED" && receipt.identityBundleToken && !delivered && <Alert type="warning" title={t("rotation.bundleWarning")} description={<Space direction="vertical" style={{ width: "100%" }}>
          <p className="muted">{t("rotation.bundleSteps")}</p>
          <Button type="primary" disabled={busy || intent?.bundleDownloadStarted} onClick={() => void downloadBundle()}>{t("rotation.bundleDownload")}</Button><Tag color="orange">{t("bundle.oneTime")}</Tag>
          {intent?.bundleDownloadStarted && <p>{t("rotation.bundleUnknown")}</p>}
        </Space>} />}
        {receipt?.state === "APPLIED" && delivered && <Alert type="info" title={t("bundle.downloaded")} />}
        {receipt && receipt.state !== "UNKNOWN" && <Button disabled={busy || uncertain || !!bundleUnavailable || receipt.state === "APPLIED" && !!intent?.bundleRequested && !delivered} onClick={() => void again()}>{t("rotation.again")}</Button>}
      </Space>
    </Modal>
  </>;
}
