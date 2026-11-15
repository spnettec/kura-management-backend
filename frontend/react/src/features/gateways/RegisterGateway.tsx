import { Alert, Button, Checkbox, Input, Modal, Radio, Space, Tag } from "antd";
import { useState } from "react";
import { api, apiDownload, ApiError, saveBlob, type Gateway, type Session } from "../../api/client";
import { errorText, LanguageSwitcher, useI18n } from "../../i18n/I18n";
import type { components } from "../../api/schema";
type Receipt = components["schemas"]["GatewayRegistration"];

export function RegisterGateway({ user, onRegistered }: { user: Session; onRegistered: () => void }) {
  const { t } = useI18n();
  const [open, setOpen] = useState(false);
  const [id, setId] = useState("");
  const [name, setName] = useState("");
  const [pem, setPem] = useState("");
  const [key, setKey] = useState("");
  const [withKey, setWithKey] = useState(false);
  const [bundleEndpoint, setBundleEndpoint] = useState("");
  const [bundleDelivered, setBundleDelivered] = useState(false);
  const [fingerprint, setFingerprint] = useState("");
  const [confirmed, setConfirmed] = useState(false);
  const [busy, setBusy] = useState(false);
  const [uncertain, setUncertain] = useState(false);
  const [receipt, setReceipt] = useState<Receipt>();
  const [error, setError] = useState<unknown>();
  const [absent, setAbsent] = useState(false);
  const frozen = busy || uncertain || !!receipt;
  function edit(change: () => void) { change(); setFingerprint(""); setConfirmed(false); setError(undefined); setAbsent(false); }
  async function review() {
    setError(undefined); setBusy(true); setConfirmed(false);
    try {
      if (!/^[A-Za-z0-9_-]{1,128}$/.test(id) || !name.trim() || new TextEncoder().encode(name).length > 256 || /[\x00-\x1f\x7f]/.test(name) || pem.length > 32768) throw new ApiError("INVALID_ARGUMENT", 400);
      if (withKey && (!/^-----BEGIN (RSA |EC |DSA )?PRIVATE KEY-----[A-Za-z0-9+/=\s]+-----END (RSA |EC |DSA )?PRIVATE KEY-----$/.test(key.trim()) || key.length > 32768)) throw new ApiError("INVALID_ARGUMENT", 400);
      const match = pem.trim().match(/^-----BEGIN CERTIFICATE-----([A-Za-z0-9+/=\s]+)-----END CERTIFICATE-----$/);
      if (!match) throw new ApiError("INVALID_ARGUMENT", 400);
      const encoded = match[1].replace(/\s/g, "");
      const raw = atob(encoded);
      if (!raw.length || btoa(raw) !== encoded) throw new ApiError("INVALID_ARGUMENT", 400);
      const hash = await crypto.subtle.digest("SHA-256", Uint8Array.from(raw, c => c.charCodeAt(0)));
      setFingerprint(Array.from(new Uint8Array(hash), b => b.toString(16).padStart(2, "0")).join(""));
    } catch (e) { setFingerprint(""); setError(e); }
    finally { setBusy(false); }
  }
  function matches(value: { id: string; name: string; certificateSha256?: string; bindingGeneration?: number }) {
    return value.id === id && value.name === name && value.certificateSha256 === fingerprint && value.bindingGeneration === 1;
  }
  async function register() {
    if (!confirmed || !fingerprint || frozen) return;
    setBusy(true); setConfirmed(false); setError(undefined); setAbsent(false);
    try {
      const body: Record<string, unknown> = { id, name, clientCertificatePem: pem };
      if (withKey) body.clientKeyPem = key;
      const value = await api<Receipt>("/gateways", "POST", body, user.csrf);
      if (!matches(value)) throw new Error("receipt mismatch");
      if (value.identityBundleToken) {
        const host = location.hostname;
        setBundleEndpoint(`https://${host}:18444`);
      }
      setReceipt(value); onRegistered();
    } catch (e) { setUncertain(true); setError(e); }
    finally { setBusy(false); }
  }
  async function downloadBundle() {
    if (!receipt?.identityBundleToken) return;
    setBusy(true); setError(undefined);
    try {
      const query = `token=${encodeURIComponent(receipt.identityBundleToken)}&endpoint=${encodeURIComponent(bundleEndpoint)}`;
      const blob = await apiDownload(`/gateways/${encodeURIComponent(receipt.id)}/identity-bundle?${query}`);
      saveBlob(blob, `${receipt.id}-identity-bundle.tar`);
      setBundleDelivered(true);
    } catch (e) { setError(e); }
    finally { setBusy(false); }
  }
  async function reconcile() {
    setBusy(true); setError(undefined);
    try {
      const value = await api<Gateway>(`/gateways/${encodeURIComponent(id)}`);
      if (!matches(value)) throw new ApiError("CONFLICT", 409);
      setReceipt({ id, name, certificateSha256: fingerprint, bindingGeneration: 1 });
      setUncertain(false); onRegistered();
    } catch (e) {
      if (e instanceof ApiError && e.status === 404) { setUncertain(false); setConfirmed(false); setAbsent(true); }
      else setError(e);
    } finally { setBusy(false); }
  }
  if (user.role !== "admin") return null;
  return <>
    <Button onClick={() => setOpen(true)}>{t("registration.open")}</Button>
    <Modal title={t("registration.open")} open={open} onCancel={() => setOpen(false)} footer={null} destroyOnHidden={false} closable={!busy} maskClosable={!busy}>
      <LanguageSwitcher />
      <p>{t("registration.description")}</p>
      <Space direction="vertical" style={{ width: "100%" }}>
        <label>{t("registration.id")}<Input aria-label={t("registration.id")} value={id} disabled={frozen} maxLength={128} onChange={e => edit(() => setId(e.target.value))} /></label>
        <label>{t("registration.name")}<Input aria-label={t("registration.name")} value={name} disabled={frozen} onChange={e => edit(() => setName(e.target.value))} /></label>
        <Radio.Group value={withKey} disabled={frozen} onChange={e => edit(() => setWithKey(e.target.value))}>
          <Radio value={false}>{t("bundle.modePaste")}</Radio>
          <Radio value={true}>{t("bundle.modeKey")}</Radio>
        </Radio.Group>
        <label>{t("registration.certificate")}<Input.TextArea aria-label={t("registration.certificate")} value={pem} disabled={frozen} maxLength={32768} rows={7} onChange={e => edit(() => setPem(e.target.value))} /></label>
        {withKey && <label>{t("bundle.key")}<Input.TextArea aria-label={t("bundle.key")} value={key} disabled={frozen} maxLength={32768} rows={5} onChange={e => edit(() => setKey(e.target.value))} /></label>}
        <Button disabled={frozen || !id || !name || !pem || (withKey && !key)} onClick={() => void review()}>{t("registration.review")}</Button>
        {fingerprint && <p>{t("registration.fingerprint")}<br /><code style={{ overflowWrap: "anywhere" }}>{fingerprint}</code></p>}
        <Checkbox checked={confirmed} disabled={!fingerprint || frozen} onChange={e => setConfirmed(e.target.checked)}>{t("registration.confirm")}</Checkbox>
        <Button type="primary" disabled={!confirmed || !fingerprint || frozen} loading={busy} onClick={() => void register()}>{t("registration.submit")}</Button>
        {error !== undefined && <Alert type="error" title={errorText(error, t)} />}
        {uncertain && <><Alert type="warning" title={t("registration.uncertain")} /><Button disabled={busy} onClick={() => void reconcile()}>{t("registration.reconcile")}</Button></>}
        {absent && <Alert type="info" title={t("registration.absent")} />}
        {receipt && <Button onClick={() => { setReceipt(undefined); setId(""); setName(""); setPem(""); setKey(""); setFingerprint(""); setConfirmed(false); setError(undefined); setAbsent(false); setBundleDelivered(false); }}>{t("registration.new")}</Button>}
        {receipt && <Alert type="success" title={t("registration.success")} description={<><div>{receipt.id}</div><code style={{ overflowWrap: "anywhere" }}>{receipt.certificateSha256}</code></>} />}
        {receipt?.identityBundleToken && !bundleDelivered && (
          <Alert type="warning" title={t("bundle.warning")} description={
            <Space direction="vertical" style={{ width: "100%" }}>
              <label>{t("bundle.endpoint")}<Input aria-label={t("bundle.endpoint")} value={bundleEndpoint} disabled={busy} maxLength={200} onChange={e => setBundleEndpoint(e.target.value)} /></label>
              <Button type="primary" disabled={!bundleEndpoint.startsWith("https://")} loading={busy} onClick={() => void downloadBundle()}>{t("bundle.download")}</Button>
              <Tag color="orange">{t("bundle.oneTime")}</Tag>
            </Space>
          } />
        )}
        {receipt?.identityBundleToken && bundleDelivered && <Alert type="info" title={t("bundle.downloaded")} />}
      </Space>
    </Modal>
  </>;
}
