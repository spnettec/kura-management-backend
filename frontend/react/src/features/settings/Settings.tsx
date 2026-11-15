import { Alert, Button, Card, Checkbox, Input, Space, Tag, Typography } from "antd";
import { useCallback, useEffect, useRef, useState } from "react";
import { api, apiDownload, ApiError, certificateFingerprint, saveBlob, type Session } from "../../api/client";
import { errorText, LanguageSwitcher, useI18n } from "../../i18n/I18n";

type TlsStatus = {
  certificateFingerprint?: string | null;
  pendingFingerprint?: string | null;
  restartRequired?: boolean;
};
type CaStatus = TlsStatus & { caFingerprint?: string | null; registeredGateways?: number };
type TaskKeyStatus = { keyFingerprint?: string; pendingRestoreStaged?: boolean };

function PemFileInput({ label, disabled, onStart, onEnd, onRead, onError }: { label: string; disabled: boolean; onStart: () => void; onEnd: () => void; onRead: (value: string) => void; onError: (error: unknown) => void }) {
  const [reading, setReading] = useState(false);
  return <label>{label}<input type="file" aria-label={label} accept=".pem,.crt,.cer,.key" disabled={disabled || reading} onChange={async e => {
    const file = e.currentTarget.files?.[0];
    if (!file) return;
    onStart();
    setReading(true);
    try {
      if (file.size > 65536) throw new ApiError("FILE_TOO_LARGE", 413);
      const value = await file.text();
      if (new TextEncoder().encode(value).length > 65536) throw new ApiError("FILE_TOO_LARGE", 413);
      onRead(value);
    } catch (error) { onError(error); }
    finally { setReading(false); onEnd(); }
  }} /></label>;
}

function Fingerprint({ label, value }: { label: string; value?: string | null }) {
  if (!value) return null;
  return (
    <p className="muted" style={{ overflowWrap: "anywhere" }}>
      {label}
      <br />
      <code>{value}</code>
    </p>
  );
}

function RestartHint({ staged }: { staged: boolean }) {
  const { t } = useI18n();
  if (!staged) return null;
  return <Alert type="success" title={t("settings.staged")} description={t("settings.restart")} />;
}

function TlsCard({ user }: { user: Session }) {
  const { t } = useI18n();
  const [status, setStatus] = useState<TlsStatus>();
  const [pem, setPem] = useState("");
  const [key, setKey] = useState("");
  const [fingerprint, setFingerprint] = useState("");
  const revision = useRef(0);
  const [confirmed, setConfirmed] = useState(false);
  const [staged, setStaged] = useState(false);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<unknown>();
  const refresh = useCallback(async () => {
    try {
      setStatus(await api<TlsStatus>("/settings/tls"));
    } catch (e) {
      setError(e);
    }
  }, []);
  useEffect(() => {
    void refresh();
  }, [refresh]);
  async function review() {
    setError(undefined);
    setConfirmed(false);
    const version = ++revision.current;
    setFingerprint("");
    const value = (await certificateFingerprint(pem)) ?? "";
    if (version === revision.current) setFingerprint(value);
  }
  async function submit() {
    setBusy(true);
    setError(undefined);
    try {
      await api("/settings/tls", "PUT", { serverPem: pem, serverKey: key }, user.csrf);
      setStaged(true);
      setPem("");
      setKey("");
      setFingerprint("");
      setConfirmed(false);
      await refresh();
    } catch (e) {
      setError(e);
    } finally {
      setBusy(false);
    }
  }
  return (
    <Card title={t("settings.tls.title")} extra={<LanguageSwitcher />}>
      <p className="muted">{t("settings.tls.description")}</p>
      <Fingerprint label={t("settings.current")} value={status?.certificateFingerprint} />
      <Fingerprint label={t("settings.pending")} value={status?.pendingFingerprint} />
      <Space direction="vertical" style={{ width: "100%" }}>
        <PemFileInput label={`${t("settings.selectFile")}: ${t("settings.tls.cert")}`} disabled={busy} onStart={() => { setBusy(true); revision.current++; setFingerprint(""); setConfirmed(false); setError(undefined); }} onEnd={() => setBusy(false)} onRead={value => { revision.current++; setPem(value); setFingerprint(""); setConfirmed(false); setError(undefined); }} onError={setError} />
        <PemFileInput label={`${t("settings.selectFile")}: ${t("settings.tls.key")}`} disabled={busy} onStart={() => { setBusy(true); revision.current++; setFingerprint(""); setConfirmed(false); setError(undefined); }} onEnd={() => setBusy(false)} onRead={value => { revision.current++; setKey(value); setConfirmed(false); setError(undefined); }} onError={setError} />
        <label>
          {t("settings.tls.cert")}
          <Input.TextArea aria-label={t("settings.tls.cert")} value={pem} maxLength={65536} rows={6} disabled={busy} onChange={(e) => { revision.current++; setPem(e.target.value); setFingerprint(""); setConfirmed(false); setError(undefined); }} />
        </label>
        <label>
          {t("settings.tls.key")}
          <Input.TextArea aria-label={t("settings.tls.key")} value={key} maxLength={65536} rows={6} disabled={busy} onChange={(e) => { revision.current++; setKey(e.target.value); setConfirmed(false); setError(undefined); }} />
        </label>
        <Button disabled={busy || !pem || !key} onClick={() => void review()}>{t("settings.review")}</Button>
        {fingerprint && (
          <p>
            {t("settings.fingerprint")}
            <br />
            <code style={{ overflowWrap: "anywhere" }}>{fingerprint}</code>
          </p>
        )}
        <Checkbox checked={confirmed} disabled={busy || !fingerprint || !key} onChange={(e) => setConfirmed(e.target.checked)}>
          {t("settings.tls.confirm")}
        </Checkbox>
        <Button aria-label={t("settings.tls.submit")} type="primary" disabled={busy || !confirmed || !fingerprint || !key} loading={busy} onClick={() => void submit()}>
          {t("settings.tls.submit")}
        </Button>
        <RestartHint staged={staged} />
        {error !== undefined && <Alert type="error" title={errorText(error, t)} />}
      </Space>
    </Card>
  );
}

function CaCard({ user }: { user: Session }) {
  const { t } = useI18n();
  const [status, setStatus] = useState<CaStatus>();
  const [pem, setPem] = useState("");
  const [fingerprint, setFingerprint] = useState("");
  const revision = useRef(0);
  const [confirmed, setConfirmed] = useState(false);
  const [staged, setStaged] = useState(false);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<unknown>();
  const refresh = useCallback(async () => {
    try {
      setStatus(await api<CaStatus>("/settings/client-ca"));
    } catch (e) {
      setError(e);
    }
  }, []);
  useEffect(() => {
    void refresh();
  }, [refresh]);
  async function review() {
    setError(undefined);
    setConfirmed(false);
    const version = ++revision.current;
    setFingerprint("");
    const value = (await certificateFingerprint(pem)) ?? "";
    if (version === revision.current) setFingerprint(value);
  }
  async function submit() {
    setBusy(true);
    setError(undefined);
    try {
      await api("/settings/client-ca", "PUT", { clientCaPem: pem }, user.csrf);
      setStaged(true);
      setPem("");
      setFingerprint("");
      setConfirmed(false);
      await refresh();
    } catch (e) {
      setError(e);
    } finally {
      setBusy(false);
    }
  }
  return (
    <Card title={t("settings.ca.title")}>
      <p className="muted">{t("settings.ca.description")}</p>
      {status?.registeredGateways !== undefined && (
        <p>
          {t("settings.ca.gateways")}: <strong>{status.registeredGateways}</strong>
        </p>
      )}
      <Fingerprint label={t("settings.current")} value={status?.caFingerprint} />
      <Fingerprint label={t("settings.pending")} value={status?.pendingFingerprint} />
      <Space direction="vertical" style={{ width: "100%" }}>
        <PemFileInput label={`${t("settings.selectFile")}: ${t("settings.ca.pem")}`} disabled={busy} onStart={() => { setBusy(true); revision.current++; setFingerprint(""); setConfirmed(false); setError(undefined); }} onEnd={() => setBusy(false)} onRead={value => { revision.current++; setPem(value); setFingerprint(""); setConfirmed(false); setError(undefined); }} onError={setError} />
        <label>
          {t("settings.ca.pem")}
          <Input.TextArea aria-label={t("settings.ca.pem")} value={pem} maxLength={65536} rows={6} disabled={busy} onChange={(e) => { revision.current++; setPem(e.target.value); setFingerprint(""); setConfirmed(false); setError(undefined); }} />
        </label>
        <Button disabled={busy || !pem} onClick={() => void review()}>{t("settings.review")}</Button>
        {fingerprint && (
          <p>
            {t("settings.fingerprint")}
            <br />
            <code style={{ overflowWrap: "anywhere" }}>{fingerprint}</code>
          </p>
        )}
        <Checkbox checked={confirmed} disabled={busy || !fingerprint} onChange={(e) => setConfirmed(e.target.checked)}>
          {t("settings.ca.confirm")}
        </Checkbox>
        <Button aria-label={t("settings.ca.submit")} type="primary" disabled={busy || !confirmed || !fingerprint} loading={busy} onClick={() => void submit()}>
          {t("settings.ca.submit")}
        </Button>
        <RestartHint staged={staged} />
        {error !== undefined && <Alert type="error" title={errorText(error, t)} />}
      </Space>
    </Card>
  );
}

function TaskKeyCard({ user }: { user: Session }) {
  const { t } = useI18n();
  const [status, setStatus] = useState<TaskKeyStatus>();
  const [passphrase, setPassphrase] = useState("");
  const [restorePassphrase, setRestorePassphrase] = useState("");
  const [rotatePassphrase, setRotatePassphrase] = useState("");
  const [rotateConfirmed, setRotateConfirmed] = useState(false);
  const [rotated, setRotated] = useState(false);
  const [restoreBase64, setRestoreBase64] = useState("");
  const [restoreName, setRestoreName] = useState("");
  const [confirmed, setConfirmed] = useState(false);
  const [staged, setStaged] = useState(false);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<unknown>();
  const refresh = useCallback(async () => {
    try {
      setStatus(await api<TaskKeyStatus>("/settings/task-key"));
    } catch (e) {
      setError(e);
    }
  }, []);
  useEffect(() => {
    void refresh();
  }, [refresh]);
  async function backup() {
    setBusy(true);
    setError(undefined);
    try {
      const blob = await apiDownload("/settings/task-key", "POST", { passphrase }, user.csrf);
      saveBlob(blob, "task-key.backup");
      setPassphrase("");
    } catch (e) {
      setError(e);
    } finally {
      setBusy(false);
    }
  }
  async function rotate() {
    setBusy(true);
    setError(undefined);
    try {
      const blob = await apiDownload(
        "/settings/task-key/rotate",
        "POST",
        { passphrase: rotatePassphrase, confirmed: true },
        user.csrf,
      );
      saveBlob(blob, "task-key-rotated.backup");
      setRotated(true);
      setRotatePassphrase("");
      setRotateConfirmed(false);
      await refresh();
    } catch (e) {
      setError(e);
    } finally {
      setBusy(false);
    }
  }
  function pick(file: File | undefined) {
    setConfirmed(false);
    if (!file) return;
    setRestoreName(file.name);
    const reader = new FileReader();
    reader.onload = () => {
      const bytes = new Uint8Array(reader.result as ArrayBuffer);
      let binary = "";
      for (const byte of bytes) binary += String.fromCharCode(byte);
      setRestoreBase64(btoa(binary));
    };
    reader.readAsArrayBuffer(file);
  }
  async function restore() {
    setBusy(true);
    setError(undefined);
    try {
      await api("/settings/task-key/restore", "POST", { backupBase64: restoreBase64, passphrase: restorePassphrase }, user.csrf);
      setStaged(true);
      setRestoreBase64("");
      setRestoreName("");
      setRestorePassphrase("");
      setConfirmed(false);
      await refresh();
    } catch (e) {
      setError(e);
    } finally {
      setBusy(false);
    }
  }
  return (
    <Card title={t("settings.taskkey.title")}>
      <p className="muted">{t("settings.taskkey.description")}</p>
      <Fingerprint label={t("settings.taskkey.fingerprint")} value={status?.keyFingerprint} />
      {status?.pendingRestoreStaged && <Alert type="warning" title={t("settings.taskkey.pending")} />}
      <Space direction="vertical" style={{ width: "100%" }}>
        <Typography.Title level={5}>{t("settings.taskkey.backup")}</Typography.Title>
        <label>
          {t("settings.taskkey.passphrase")}
          <Input.Password aria-label={t("settings.taskkey.passphrase")} value={passphrase} maxLength={1024} onChange={(e) => { setPassphrase(e.target.value); setError(undefined); }} />
        </label>
        <Button disabled={passphrase.length < 16} loading={busy} onClick={() => void backup()}>
          {t("settings.taskkey.backupSubmit")}
        </Button>
        <Typography.Title level={5}>{t("settings.taskkey.rotate")}</Typography.Title>
        <label>
          {t("settings.taskkey.rotatePassphrase")}
          <Input.Password
            aria-label={t("settings.taskkey.rotatePassphrase")}
            value={rotatePassphrase}
            maxLength={1024}
            onChange={(e) => {
              setRotatePassphrase(e.target.value);
              setRotateConfirmed(false);
              setError(undefined);
            }}
          />
        </label>
        <Checkbox
          checked={rotateConfirmed}
          disabled={rotatePassphrase.length < 16 || busy}
          onChange={(e) => setRotateConfirmed(e.target.checked)}
        >
          {t("settings.taskkey.rotateConfirm")}
        </Checkbox>
        <Button
          type="primary"
          danger
          disabled={!rotateConfirmed || rotatePassphrase.length < 16}
          loading={busy}
          onClick={() => void rotate()}
        >
          {t("settings.taskkey.rotateSubmit")}
        </Button>
        {rotated && <Alert type="success" title={t("settings.taskkey.rotateDone")} />}
        <Typography.Title level={5}>{t("settings.taskkey.restore")}</Typography.Title>
        <input type="file" aria-label={t("settings.taskkey.restore")} disabled={busy} onChange={(e) => pick(e.target.files?.[0])} />
        {restoreName && <Tag>{restoreName}</Tag>}
        <label>
          {t("settings.taskkey.passphrase")}
          <Input.Password aria-label={t("settings.taskkey.passphrase")} value={restorePassphrase} maxLength={1024} onChange={(e) => { setRestorePassphrase(e.target.value); setConfirmed(false); setError(undefined); }} />
        </label>
        <Checkbox checked={confirmed} disabled={!restoreBase64 || restorePassphrase.length < 16} onChange={(e) => setConfirmed(e.target.checked)}>
          {t("settings.taskkey.confirm")}
        </Checkbox>
        <Button type="primary" disabled={!confirmed || !restoreBase64 || restorePassphrase.length < 16} loading={busy} onClick={() => void restore()}>
          {t("settings.taskkey.restoreSubmit")}
        </Button>
        <RestartHint staged={staged} />
        {error !== undefined && <Alert type="error" title={errorText(error, t)} />}
      </Space>
    </Card>
  );
}

export function Settings({ user }: { user: Session }) {
  const { t } = useI18n();
  if (user.role !== "admin") return <Alert type="warning" title={t("settings.adminOnly")} />;
  return (
    <Space direction="vertical" style={{ width: "100%" }}>
      <Alert type="info" title={t("settings.description")} />
      <TlsCard user={user} />
      <CaCard user={user} />
      <TaskKeyCard user={user} />
    </Space>
  );
}
