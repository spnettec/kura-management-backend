import { Alert, Button, Card, Input, Space, Typography } from "antd";
import { useEffect, useId, useState, type FormEvent } from "react";
import { api, apiDownload, ApiError, saveBlob, type Session } from "../../api/client";
import type { components } from "../../api/schema";
import { errorText, useI18n } from "../../i18n/I18n";

type Signing = components["schemas"]["GatewaySigningRequest"];
type PublicCertificates = components["schemas"]["GatewayEnrollmentBatch"];
type CsrEntries = components["schemas"]["PrepareGatewaySigningRequest"]["entries"];
type CertificateEntries = components["schemas"]["GatewayEnrollmentBatchRequest"]["entries"];
type Marker = { id: string; pending?: "prepare" | "certificates" };
const batchPath = "/gateway-enrollment-batches";
const signingPath = `${batchPath}/signing-requests`;
const uuidPattern = "[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}";

async function publicPem(data: FormData, name: string, label: "CERTIFICATE REQUEST" | "CERTIFICATE") {
  const file = data.get(name);
  if (!(file instanceof File) || file.size > 32768) throw new ApiError("CA_SIGNING_FILE", 400);
  const value = (await file.text()).trim();
  const pattern = new RegExp(`^-----BEGIN ${label}-----[A-Za-z0-9+/=\\s]+-----END ${label}-----$`);
  if (!pattern.test(value)) throw new ApiError("CA_SIGNING_FILE", 400);
  return value;
}

export function BulkSigningRequest({ user, onReview }: { user: Session; onReview: (id: string) => void }) {
  const { t } = useI18n();
  const label = useId();
  const key = `yofc.console.gateway-signing.v1:${user.id}`;
  const [open, setOpen] = useState(false);
  const [rows, setRows] = useState(() => [crypto.randomUUID(), crypto.randomUUID()]);
  const [marker, setMarker] = useState<Marker | null>();
  const [id, setId] = useState("");
  const [signing, setSigning] = useState<Signing>();
  const [proposal, setProposal] = useState<PublicCertificates>();
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<unknown>();

  function remember(next: Marker) {
    localStorage.setItem(key, JSON.stringify(next));
    setMarker(next);
    setId(next.id);
  }
  async function readOriginal(original: string, previous: Marker | null | undefined) {
    const request = await api<Signing>(`${signingPath}/${original}`);
    setSigning(request);
    const pending = previous?.id === original ? previous.pending : undefined;
    try {
      const enrolled = await api<PublicCertificates>(`${batchPath}/${original}`);
      if (enrolled.signingManifestSha256 !== request.manifestSha256) throw new Error("Batch manifest differs");
      setProposal(enrolled);
    } catch (problem) {
      setProposal(undefined);
      if (pending === "certificates" || !(problem instanceof ApiError && problem.status === 404)) throw problem;
    }
    remember({ id: original });
  }

  useEffect(() => {
    if (user.role !== "admin") return;
    try {
      const existing = localStorage.getItem(key);
      if (!existing) {
        setMarker(null);
        return;
      }
      const saved: Marker = JSON.parse(existing);
      if (!saved || !new RegExp(`^${uuidPattern}$`).test(saved.id)
        || (saved.pending !== undefined && !["prepare", "certificates"].includes(saved.pending))) {
        throw new Error("Invalid saved signing request");
      }
      setMarker(saved);
      setId(saved.id);
      setOpen(!!saved.pending);
      setBusy(true);
      void readOriginal(saved.id, saved).catch(setError).finally(() => setBusy(false));
    } catch (problem) { setError(problem); }
  }, [key, user.role]);

  async function run(action: () => Promise<void>) {
    if (busy) return;
    setError(undefined);
    setBusy(true);
    try { await action(); }
    catch (problem) { setError(problem); }
    finally { setBusy(false); }
  }

  async function dispatch(next: Marker, write: () => Promise<void>) {
    const existing = localStorage.getItem(key);
    if (existing && (JSON.parse(existing) as Marker).pending) throw new ApiError("CA_BATCH_OBSERVE_REQUIRED", 409);
    remember(next);
    try {
      await write();
      remember({id: next.id});
    } catch (problem) {
      if (problem instanceof ApiError && [
        "CA_SIGNING_CSR_INVALID", "CA_SIGNING_MISMATCH", "CA_BATCH_CONFLICT",
        "INVALID_ARGUMENT", "CA_BATCH_CERTIFICATE_INVALID",
      ].includes(problem.code)) remember({id: next.id});
      throw problem;
    }
  }

  function prepare(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const form = event.currentTarget;
    const data = new FormData(form);
    void run(async () => {
      const entries: CsrEntries = [];
      for (const row of rows) {
        entries.push({
          id: String(data.get(`id-${row}`)),
          name: String(data.get(`name-${row}`)),
          csrPem: await publicPem(data, row, "CERTIFICATE REQUEST"),
        });
      }
      const operationId = crypto.randomUUID();
      await dispatch({id: operationId, pending: "prepare"}, async () => {
        setSigning(await api<Signing>(signingPath, "POST", {operationId, entries}, user.csrf));
        setProposal(undefined);
        form.reset();
      });
    });
  }

  function importSigned(event: FormEvent<HTMLFormElement>, request: Signing) {
    event.preventDefault();
    const data = new FormData(event.currentTarget);
    void run(async () => {
      const entries: CertificateEntries = [];
      for (const {id, name} of request.entries) {
        entries.push({id, name, clientCertificatePem: await publicPem(data, id, "CERTIFICATE")});
      }
      const operationId = request.operationId;
      await dispatch({id: operationId, pending: "certificates"}, async () => {
        const receipt = await api<PublicCertificates>(`${signingPath}/${operationId}/certificates`, "POST",
          {operationId, entries}, user.csrf);
        if (receipt.signingManifestSha256 !== request.manifestSha256) throw new Error("Batch manifest differs");
        setProposal(receipt);
      });
    });
  }

  useEffect(() => {
    if (error && open) document.getElementById(`${label}-error`)?.focus();
  }, [error, label, open]);

  if (user.role !== "admin") return null;
  const locked = busy || marker === undefined || !!marker?.pending || user.mustChangePassword;
  return <>
    <Button aria-expanded={open} aria-controls={label} onClick={() => setOpen(!open)}>{t("caSigning.title")}</Button>
    {open && <section id={label}><Card title={t("caSigning.title")}>
      <Space direction="vertical" style={{width: "100%"}}>
        <p>{t("caSigning.help")}</p>
        <form method="get" action={`/api/v1${signingPath}/${id}`} aria-label={t("caSigning.observe")}
          onSubmit={event => { event.preventDefault(); void run(() => readOriginal(id, marker)); }}>
          <Space direction="vertical" style={{width: "100%", maxWidth: "24rem"}}>
            <label htmlFor={`${label}-id`}>{t("batchEnroll.id")}</label>
            <Input id={`${label}-id`} name="operationId" required pattern={uuidPattern} value={id} readOnly={!!marker?.pending}
              disabled={busy} autoCorrect="off" autoCapitalize="none" spellCheck={false}
              onChange={event => { setId(event.target.value); setSigning(undefined); setProposal(undefined); }} />
            <Button htmlType="submit" disabled={busy}>{t("caSigning.observe")}</Button>
          </Space>
        </form>
        {marker?.pending && <Alert type="warning" title={t("batchEnroll.unknown")} />}
        {!signing && <form method="post" action={`/api/v1${signingPath}`} aria-label={t("caSigning.prepare")} onSubmit={prepare}>
          <Space direction="vertical">
            {rows.map((row, index) => <fieldset key={row} disabled={locked}>
              <legend>{t("batchEnroll.gateway")} {index + 1}</legend>
              <Space direction="vertical">
                <label htmlFor={`${row}-id`}>{t("registration.id")}</label>
                <Input id={`${row}-id`} name={`id-${row}`} required pattern="[A-Za-z0-9_\-]{1,128}"
                  autoCorrect="off" autoCapitalize="none" spellCheck={false} />
                <label htmlFor={`${row}-name`}>{t("registration.name")}</label>
                <Input id={`${row}-name`} name={`name-${row}`} required maxLength={256} />
                <label htmlFor={`${row}-request`}>{t("caSigning.csrFile")}</label>
                <input id={`${row}-request`} name={row} type="file" required accept=".pem,.csr" />
                {rows.length > 2 && <Button htmlType="button" onClick={() => setRows(current => current.filter(value => value !== row))}>
                  {t("batchEnroll.remove")}
                </Button>}
              </Space>
            </fieldset>)}
            <Space wrap>
              <Button htmlType="button" disabled={locked || rows.length >= 32}
                onClick={() => setRows(current => [...current, crypto.randomUUID()])}>{t("batchEnroll.add")}</Button>
              <Button htmlType="submit" disabled={locked}>{t("caSigning.prepare")}</Button>
            </Space>
          </Space>
        </form>}
        {signing && <>
          <div role="status">
            <Typography.Text copyable code>{signing.operationId}</Typography.Text>
            <p>{t("caSigning.manifest")} <Typography.Text code style={{overflowWrap: "anywhere"}}>{signing.manifestSha256}</Typography.Text></p>
            <ul>{signing.entries.map(entry => <li key={entry.id}>{entry.name} ({entry.id}) · {entry.path}{" "}
              <Typography.Text code style={{overflowWrap: "anywhere"}}>{entry.publicKeySha256}</Typography.Text>
            </li>)}</ul>
            {proposal && <p>{t(`batchEnroll.${proposal.state}`)} · {proposal.approvedCount}/{proposal.requiredCount}</p>}
          </div>
          <Space wrap>
            <Button disabled={busy} onClick={() => void run(async () => {
              const archive = await apiDownload(`${signingPath}/${signing.operationId}/bundle`);
              saveBlob(archive, `gateway-csrs-${signing.operationId}.tar`);
            })}>{t("caSigning.download")}</Button>
            <Button disabled={locked} onClick={() => {
              setSigning(undefined); setProposal(undefined); setId(""); setError(undefined);
            }}>{t("caSigning.new")}</Button>
          </Space>
          {proposal ? <Button type="primary" onClick={() => onReview(signing.operationId)}>{t("caSigning.review")}</Button>
            : <form method="post" action={`/api/v1${signingPath}/${signing.operationId}/certificates`}
                aria-label={t("caSigning.upload")} onSubmit={event => importSigned(event, signing)}>
                <fieldset disabled={locked}>
                  <legend>{t("caSigning.upload")}</legend>
                  <Space direction="vertical">
                    {signing.entries.map(entry => <label key={entry.id}>
                      {t("batchEnroll.certificate")} · {entry.id}{" "}
                      <input name={entry.id} type="file" required accept=".pem,.crt" />
                    </label>)}
                    <Button htmlType="submit" type="primary" disabled={locked}>{t("caSigning.upload")}</Button>
                  </Space>
                </fieldset>
            </form>}
        </>}
        {error !== undefined && <div id={`${label}-error`} tabIndex={-1}><Alert type="error" title={errorText(error, t)} /></div>}
      </Space>
    </Card></section>}
  </>;
}
