import { Alert, Button, Checkbox, Input, Modal, Radio, Space, Tag } from "antd";
import { useState } from "react";
import { api, apiDownload, ApiError, certificateFingerprint, saveBlob, type Gateway, type Session } from "../../api/client";
import { errorText, LanguageSwitcher, useI18n } from "../../i18n/I18n";

type RotationReceipt = {
  id: string;
  certificateSha256: string;
  revision: number;
  identityBundleToken?: string;
};

export function RotateCertificate({ gateway, user }: { gateway: Gateway; user: Session }) {
  const { t } = useI18n();
  const [open, setOpen] = useState(false);
  const [pem, setPem] = useState("");
  const [key, setKey] = useState("");
  const [withKey, setWithKey] = useState(true);
  const [fingerprint, setFingerprint] = useState("");
  const [confirmed, setConfirmed] = useState(false);
  const [busy, setBusy] = useState(false);
  const [uncertain, setUncertain] = useState(false);
  const [receipt, setReceipt] = useState<RotationReceipt>();
  const [delivered, setDelivered] = useState(false);
  const [error, setError] = useState<unknown>();
  const frozen = busy || uncertain || !!receipt;

  function edit(change: () => void) {
    change();
    setFingerprint("");
    setConfirmed(false);
    setError(undefined);
  }
  async function review() {
    setError(undefined);
    setConfirmed(false);
    const reviewed = (await certificateFingerprint(pem)) ?? "";
    setFingerprint(reviewed);
    if (!reviewed) setError(new ApiError("INVALID_ARGUMENT", 400));
  }
  async function rotate() {
    if (!confirmed || !fingerprint || frozen) return;
    setBusy(true);
    setConfirmed(false);
    setError(undefined);
    try {
      const body: Record<string, unknown> = {
        previousCertificateSha256: gateway.certificateSha256,
        clientCertificatePem: pem,
      };
      if (withKey) body.clientKeyPem = key;
      const value = await api<RotationReceipt>(
        `/gateways/${encodeURIComponent(gateway.id)}/certificate`,
        "POST",
        body,
        user.csrf,
      );
      if (value.certificateSha256 !== fingerprint) throw new Error("receipt mismatch");
      setReceipt(value);
    } catch (e) {
      setUncertain(true);
      setError(e);
    } finally {
      setBusy(false);
    }
  }
  async function reconcile() {
    setBusy(true);
    setError(undefined);
    try {
      const value = await api<Gateway>(`/gateways/${encodeURIComponent(gateway.id)}`);
      if (value.certificateSha256 !== fingerprint) throw new ApiError("CONFLICT", 409);
      setReceipt({
        id: gateway.id,
        certificateSha256: fingerprint,
        revision: value.revision,
      });
      setUncertain(false);
    } catch (e) {
      setError(e);
    } finally {
      setBusy(false);
    }
  }
  async function downloadBundle() {
    if (!receipt?.identityBundleToken) return;
    setBusy(true);
    setError(undefined);
    try {
      const query = `token=${encodeURIComponent(receipt.identityBundleToken)}`;
      const blob = await apiDownload(`/gateways/${encodeURIComponent(receipt.id)}/identity-bundle?${query}`);
      saveBlob(blob, `${receipt.id}-rotation-bundle.tar`);
      setDelivered(true);
    } catch (e) {
      setError(e);
    } finally {
      setBusy(false);
    }
  }
  return (
    <>
      <Button onClick={() => setOpen(true)}>
        {t("rotation.open")}
      </Button>
      <Modal
        title={t("rotation.open")}
        open={open}
        onCancel={() => setOpen(false)}
        footer={null}
        destroyOnHidden={false}
        closable={!busy}
        maskClosable={!busy}
      >
        <LanguageSwitcher />
        <p className="muted">{t("rotation.description")}</p>
        <Space direction="vertical" style={{ width: "100%" }}>
          <p className="muted" style={{ overflowWrap: "anywhere" }}>
            {t("rotation.current")}
            <br />
            <code>{gateway.certificateSha256}</code>
          </p>
          <Radio.Group value={withKey} disabled={frozen} onChange={(e) => edit(() => setWithKey(e.target.value))}>
            <Radio value={true}>{t("rotation.modeBundle")}</Radio>
            <Radio value={false}>{t("rotation.modePem")}</Radio>
          </Radio.Group>
          <label>
            {t("rotation.newCertificate")}
            <Input.TextArea
              aria-label={t("rotation.newCertificate")}
              value={pem}
              disabled={frozen}
              maxLength={32768}
              rows={6}
              onChange={(e) => edit(() => setPem(e.target.value))}
            />
          </label>
          {withKey && (
            <label>
              {t("rotation.newKey")}
              <Input.TextArea
                aria-label={t("rotation.newKey")}
                value={key}
                disabled={frozen}
                maxLength={32768}
                rows={5}
                onChange={(e) => edit(() => setKey(e.target.value))}
              />
            </label>
          )}
          <Button disabled={frozen || !pem || (withKey && !key)} onClick={() => void review()}>
            {t("settings.review")}
          </Button>
          {fingerprint && (
            <p>
              {t("settings.fingerprint")}
              <br />
              <code style={{ overflowWrap: "anywhere" }}>{fingerprint}</code>
            </p>
          )}
          <Checkbox checked={confirmed} disabled={!fingerprint || frozen} onChange={(e) => setConfirmed(e.target.checked)}>
            {t("rotation.confirm")}
          </Checkbox>
          <Button type="primary" disabled={!confirmed || !fingerprint || frozen} loading={busy} onClick={() => void rotate()}>
            {t("rotation.submit")}
          </Button>
          {error !== undefined && <Alert type="error" title={errorText(error, t)} />}
          {uncertain && (
            <>
              <Alert type="warning" title={t("rotation.uncertain")} />
              <Button disabled={busy} onClick={() => void reconcile()}>
                {t("registration.reconcile")}
              </Button>
            </>
          )}
          {receipt && !receipt.identityBundleToken && (
            <Alert type="success" title={t("rotation.doneCenterOnly")} description={<code style={{ overflowWrap: "anywhere" }}>{receipt.certificateSha256}</code>} />
          )}
          {receipt?.identityBundleToken && !delivered && (
            <Alert
              type="warning"
              title={t("rotation.bundleWarning")}
              description={
                <Space direction="vertical" style={{ width: "100%" }}>
                  <p className="muted">{t("rotation.bundleSteps")}</p>
                  <Button type="primary" loading={busy} onClick={() => void downloadBundle()}>
                    {t("rotation.bundleDownload")}
                  </Button>
                  <Tag color="orange">{t("bundle.oneTime")}</Tag>
                </Space>
              }
            />
          )}
          {receipt?.identityBundleToken && delivered && (
            <Alert type="info" title={t("bundle.downloaded")} />
          )}
          {receipt && (
            <Button
              onClick={() => {
                setReceipt(undefined);
                setPem("");
                setKey("");
                setFingerprint("");
                setConfirmed(false);
                setError(undefined);
                setUncertain(false);
                setDelivered(false);
              }}
            >
              {t("rotation.again")}
            </Button>
          )}
        </Space>
      </Modal>
    </>
  );
}
