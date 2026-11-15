import { Alert, Button, Card, Checkbox, Input, Radio, Space, Typography } from "antd";
import { useCallback, useEffect, useRef, useState } from "react";
import { api, ApiError, certificateFingerprint, type Session } from "../../api/client";
import { errorText, LanguageSwitcher, useI18n } from "../../i18n/I18n";
import { Login } from "./Login";
import { setupApi } from "../../api/setup";
import { ServerCsr, type CsrSelection } from "../settings/ServerCsr";

type Status = { configured: boolean; maintenance?: boolean; authenticated?: boolean; installationId?: string; certificateFingerprint?: string; csrf?: string; blockingOperationId?: string | null; operationId?: string; origin?: string; listenAddress?: string };
type Original = { operationId: string; state: "PENDING" | "APPLIED" | "UNKNOWN" | "REJECTED"; origin: string; observedConfiguration: boolean };
type Saved = { operationId: string; installationId: string; origin: string };
type Credential = { schemaVersion: number; installationId: string; token: string; certificateFingerprint: string };
const savedKey = "yofc.console.initial-setup.v1";

export function InitialSetup() {
  const { t } = useI18n();
  const [status, setStatus] = useState<Status>();
  const [user, setUser] = useState<Session | null>();
  const [credential, setCredential] = useState<Credential>();
  const [trustReviewed, setTrustReviewed] = useState(false);
  const [saved, setSaved] = useState<Saved | null>();
  const [operation, setOperation] = useState<Original>();
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<unknown>();
  const [draft, setDraft] = useState({ origin: location.origin, grpc: "0.0.0.0:18444", dbHost: "localhost", dbPort: "5432", dbName: "console", dbUser: "console", dbPassword: "", username: "", tenant: "", password: "" });
  const [pem, setPem] = useState({ cert: "", key: "", ca: "" });
  const [identityMode, setIdentityMode] = useState("generated");
  const [csrId, setCsrId] = useState<string>();
  const [csrBlocked, setCsrBlocked] = useState(true);
  const [fingerprints, setFingerprints] = useState<{ cert: string; ca: string }>();
  const [confirmed, setConfirmed] = useState(false);
  const revision = useRef(0);
  const defaultApplied = useRef(false);
  const csrChanged = useCallback((selection: CsrSelection | undefined, blocked: boolean) => {
    setCsrBlocked(blocked); setCsrId(selection?.requestId);
    setPem(v => ({ ...v, cert: selection?.certificatePem ?? "", key: "" }));
    revision.current++; setConfirmed(false); setFingerprints(undefined);
  }, []);
  const refresh = useCallback(async () => {
    try {
      const value = await setupApi<Status>("/status"); setStatus(value);
        if (!value.configured && !defaultApplied.current) {
          const match = /^(\[[^\]]+\]|[0-9.]+):([0-9]+)$/.exec(value.listenAddress ?? "");
          if (match) {
            const port = Number(match[2]);
            if (Number.isInteger(port) && port > 0 && port <= 65535) {
              setDraft(v => ({ ...v, grpc: `${match[1].startsWith("[") ? "[::]" : "0.0.0.0"}:${port === 65535 ? 65534 : port + 1}` }));
              defaultApplied.current = true;
            }
          }
        }
      if (value.configured && !value.maintenance) {
        try { setUser(await api<Session>("/session")); }
        catch (e) { if (e instanceof ApiError && e.status === 401) setUser(null); else throw e; }
      }
      const raw = localStorage.getItem(savedKey), record: Saved | null = raw ? JSON.parse(raw) : null;
      if (record && (!/^[0-9a-f-]{36}$/i.test(record.operationId) || typeof record.origin !== "string")) throw new Error("Invalid saved setup");
      setSaved(record && record.installationId === value.installationId ? record : null);
    } catch (e) { setError(e); }
  }, []);
  useEffect(() => { void refresh(); }, [refresh]);
  const observe = useCallback(async (id: string, configured: boolean) => {
    try {
      const result = configured ? await api<Original>(`/settings/initial-setup/${id}`) : await setupApi<Original>(`/operations/${id}`);
      setOperation(result); setError(undefined);
    } catch (e) { setError(e); await refresh(); } // Read only after the TLS handoff.
  }, [refresh]);
  const originalId = saved?.operationId ?? status?.blockingOperationId ?? status?.operationId;
  useEffect(() => {
    if (!originalId || !status || (status.configured ? user?.role !== "admin" : !status.authenticated)) return;
    void observe(originalId, status.configured);
  }, [originalId, status?.configured, status?.authenticated, user?.role, observe]);
  useEffect(() => {
    if (!originalId || operation?.state !== "PENDING") return;
    const timer = setTimeout(() => void observe(originalId, !!status?.configured), 600);
    return () => clearTimeout(timer);
  }, [originalId, operation, status?.configured, observe]);
  function changed(key: keyof typeof draft, value: string) { revision.current++; setDraft(v => ({ ...v, [key]: value })); setConfirmed(false); setFingerprints(undefined); }
  async function readFile(file: File, limit: number) {
    if (file.size > limit) throw new ApiError("FILE_TOO_LARGE", 413);
    const value = await file.text();
    if (new TextEncoder().encode(value).length > limit) throw new ApiError("FILE_TOO_LARGE", 413);
    return value;
  }
  async function pair() {
    if (!credential || !trustReviewed || busy) return;
    setBusy(true); setError(undefined);
    try { await setupApi("/pair", "POST", { installationId: credential.installationId, token: credential.token }); setCredential(undefined); await refresh(); }
    catch (e) { setError(e); } finally { setBusy(false); }
  }
  async function review() {
    setBusy(true); setError(undefined); setConfirmed(false); const current = ++revision.current;
    try {
      const [cert, ca] = await Promise.all([certificateFingerprint(pem.cert), certificateFingerprint(pem.ca)]);
      if (!cert || !ca) throw new ApiError("INVALID_ARGUMENT", 400);
      if (current === revision.current) setFingerprints({ cert, ca });
    } catch (e) { setError(e); } finally { setBusy(false); }
  }
  async function submit() {
    if (!status?.installationId || !status.csrf || saved !== null || status.blockingOperationId || !confirmed || busy) return;
    const request = { operationId: crypto.randomUUID(), installationId: status.installationId, origin: draft.origin };
    const host = draft.dbHost.includes(":") && !draft.dbHost.startsWith("[") ? `[${draft.dbHost}]` : draft.dbHost;
    setBusy(true); setConfirmed(false); setError(undefined);
    try {
      localStorage.setItem(savedKey, JSON.stringify(request)); setSaved(request); // Before dispatch; no secrets stored.
      setOperation(await setupApi<Original>("/operations", "POST", { operationId: request.operationId, origin: draft.origin,
        grpcAddress: draft.grpc, databaseUrl: `postgres://${encodeURIComponent(draft.dbUser)}:${encodeURIComponent(draft.dbPassword)}@${host}:${draft.dbPort}/${encodeURIComponent(draft.dbName)}`,
        username: draft.username, tenantId: draft.tenant, password: draft.password, serverPem: pem.cert, serverKey: pem.key, ...(csrId ? { serverCsrId: csrId } : {}), clientCaPem: pem.ca }, status.csrf));
    } catch (e) { setError(e); }
    finally { setPem({ cert: "", key: "", ca: "" }); setDraft(v => ({ ...v, password: "", dbPassword: "" })); setBusy(false); }
  }
  function acknowledgeRejection() {
    if (operation?.state !== "REJECTED") return;
    try { localStorage.removeItem(savedKey); setSaved(null); setOperation(undefined); setConfirmed(false); void refresh(); } catch (e) { setError(e); }
  }
  if (status?.configured && !status.maintenance && user === null) return <Login onLogin={refresh} />;
  const field = (key: keyof typeof draft, label: string, secret = false) => <label>{label}
    {secret ? <Input.Password aria-label={label} value={draft[key]} maxLength={256} disabled={busy || !!originalId} autoComplete="new-password" onChange={e => changed(key, e.target.value)} />
      : <Input aria-label={label} value={draft[key]} maxLength={256} disabled={busy || !!originalId} onChange={e => changed(key, e.target.value)} />}</label>;
  const file = (key: keyof typeof pem, label: string) => <label>{label}<input type="file" aria-label={label} accept=".pem,.crt,.cer,.key" disabled={busy || !!originalId} onChange={async event => {
    const input = event.currentTarget, f = input.files?.[0]; if (!f) return;
    setBusy(true); setError(undefined); setConfirmed(false); setFingerprints(undefined); revision.current++;
    try { const value = await readFile(f, key === "key" ? 32768 : 65536); setPem(v => ({ ...v, [key]: value })); }
    catch (e) { setError(e); } finally { input.value = ""; setBusy(false); }
  }} /></label>;
  const nextOrigin = operation?.origin ?? saved?.origin ?? status?.origin;
  if (status?.maintenance) return <main style={{ maxWidth:940,margin:"32px auto",padding:20 }}><Space direction="vertical"><LanguageSwitcher /><Alert type="warning" title={t("error.MAINTENANCE")} /><Button onClick={()=>void refresh()}>{t("installation.observe")}</Button></Space></main>;
  return <main style={{ maxWidth: 940, margin: "32px auto", padding: 20 }}><Space direction="vertical" style={{ width: "100%" }}>
    <LanguageSwitcher /><Typography.Title level={1}>{t("installation.title")}</Typography.Title>
    {status?.configured && <Alert type="success" title={t("installation.configured")} />}
    {!status?.configured && !status?.authenticated && <Card title={t("installation.access")}>
      <p>{t("installation.trust")}</p><Typography.Text code style={{ overflowWrap: "anywhere" }}>{credential?.certificateFingerprint ?? status?.certificateFingerprint}</Typography.Text>
      <Space direction="vertical" style={{ width: "100%" }}>
        <label>{t("installation.credential")}<input type="file" aria-label={t("installation.credential")} accept=".json" disabled={busy} onChange={async event => {
          const input = event.currentTarget, f = input.files?.[0]; if (!f) return; setBusy(true); setError(undefined); setTrustReviewed(false);
          try {
            const value: Credential = JSON.parse(await readFile(f, 65536));
            if (value.schemaVersion !== 1 || !/^[0-9a-f]{64}$/.test(value.token) || value.installationId !== status?.installationId || value.certificateFingerprint !== status?.certificateFingerprint) throw new ApiError("INVALID_ARGUMENT", 400);
            setCredential(value);
          } catch (e) { setError(e); } finally { input.value = ""; setBusy(false); }
        }} /></label>
        <Checkbox checked={trustReviewed} disabled={busy || !credential} onChange={e => setTrustReviewed(e.target.checked)}>{t("installation.trustConfirm")}</Checkbox>
        <Button type="primary" disabled={busy || !credential || !trustReviewed} onClick={() => void pair()}>{t("installation.pair")}</Button>
      </Space>
    </Card>}
    {!status?.configured && status?.authenticated && !originalId && <>
      <Card title={t("installation.center")}><Space direction="vertical" style={{ width: "100%" }}>
        {field("origin", t("installation.origin"))}{field("grpc", t("installation.grpc"))}<Alert type="info" title={t("installation.emptyDatabase")} />
        {field("dbHost", t("installation.dbHost"))}{field("dbPort", t("installation.dbPort"))}{field("dbName", t("installation.dbName"))}{field("dbUser", t("installation.dbUser"))}{field("dbPassword", t("installation.dbPassword"), true)}
      </Space></Card>
      <Card title={t("installation.certificates")}><Space direction="vertical" style={{ width: "100%" }}><p>{t("installation.externalCa")}</p>
        <Radio.Group aria-label={t("csr.mode")} value={identityMode} disabled={busy || csrBlocked} onChange={e => { setIdentityMode(e.target.value); setCsrId(undefined); setPem(v => ({ ...v, cert: "", key: "" })); setConfirmed(false); setFingerprints(undefined); revision.current++; }}>
          <Radio.Button value="generated">{t("csr.generated")}</Radio.Button><Radio.Button value="import">{t("csr.existing")}</Radio.Button>
        </Radio.Group>
        {identityMode === "generated" ? <ServerCsr setup scope={status.installationId!} origin={draft.origin} csrf={status.csrf!} disabled={busy} onChange={csrChanged} />
          : <>{file("cert", t("installation.serverCert"))}{file("key", t("installation.serverKey"))}</>}
        {file("ca", t("installation.clientCa"))}
      </Space></Card>
      <Card title={t("installation.administrator")}><Space direction="vertical" style={{ width: "100%" }}>
        {field("tenant", t("installation.tenant"))}{field("username", t("installation.username"))}{field("password", t("installation.password"), true)}
        <Button disabled={busy || !pem.cert || (!pem.key && !csrId) || !pem.ca || !draft.username || !draft.tenant || new TextEncoder().encode(draft.password).length < 12} onClick={() => void review()}>{t("installation.review")}</Button>
        {fingerprints && <><p>{t("installation.serverCert")}<br /><code>{fingerprints.cert}</code></p><p>{t("installation.clientCa")}<br /><code>{fingerprints.ca}</code></p></>}
        <Checkbox checked={confirmed} disabled={busy || !fingerprints} onChange={e => setConfirmed(e.target.checked)}>{t("installation.confirm")}</Checkbox>
        <Button type="primary" disabled={busy || saved !== null || !confirmed || !fingerprints} loading={busy} onClick={() => void submit()}>{t("installation.submit")}</Button>
      </Space></Card>
    </>}
    {originalId && <Card title={t("installation.original")}><Space direction="vertical" style={{ width: "100%" }}>
      <Typography.Text code copyable>{originalId}</Typography.Text>
      <Alert type={operation?.state === "APPLIED" ? "success" : "warning"} title={operation?.state === "APPLIED" ? t("installation.applied") : operation?.state === "PENDING" ? t("installation.pending") : operation?.state === "REJECTED" ? t("installation.rejected") : t("installation.unknown")} />
      <Button disabled={busy} onClick={() => void observe(originalId, !!status?.configured)}>{t("installation.observe")}</Button>
      <Button disabled={busy} onClick={() => void refresh()}>{t("installation.reconnect")}</Button>
      {operation?.state === "REJECTED" && <Button onClick={acknowledgeRejection}>{t("installation.edit")}</Button>}
    </Space></Card>}
    {nextOrigin?.startsWith("https://") && <Button href={`${nextOrigin}/setup${originalId ? `?operationId=${encodeURIComponent(originalId)}` : ""}`}>{t("installation.openCenter")}</Button>}
    {status?.configured && user?.role === "admin" && <Button href="/">{t("installation.enter")}</Button>}
    {error !== undefined && <Alert type="error" title={errorText(error, t)} />}
  </Space></main>;
}
