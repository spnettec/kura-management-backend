import { Alert, Button, Card, Input, Space, Typography } from "antd";
import { useEffect, useId, useState } from "react";
import type { FormEvent } from "react";
import { api, ApiError, type Session } from "../../api/client";
import type { components } from "../../api/schema";
import { errorText, useI18n } from "../../i18n/I18n";
import { savePublicRotationReceipt, type RotationReceipt } from "./RotateCertificate";

type Receipt = components["schemas"]["GatewayEnrollmentBatch"];
type Request = components["schemas"]["GatewayEnrollmentBatchRequest"];
type Saved = { id: string; pending?: "propose" | "approve" };
const path = "/gateway-enrollment-batches";
const uuidPattern = "[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}";
const publicPem = /^-----BEGIN CERTIFICATE-----[A-Za-z0-9+/=\s]+-----END CERTIFICATE-----$/;
const maximum = 32;

export function BulkEnrollment({ user, onRegistered, reviewId }: { user: Session; onRegistered: () => void; reviewId?: string }) {
  const { t } = useI18n();
  const label = useId();
  const key = `yofc.console.gateway-enrollment-batch.v1:${user.id}`;
  const [open, setOpen] = useState(false);
  const [rows, setRows] = useState(() => [crypto.randomUUID(), crypto.randomUUID()]);
  const [saved, setSaved] = useState<Saved | null>();
  const [id, setId] = useState("");
  const [receipt, setReceipt] = useState<Receipt>();
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<unknown>();

  useEffect(() => {
    if (user.role !== "admin") return;
    let cancelled = false;
    try {
      const raw = localStorage.getItem(key);
      if (!raw) {
        setSaved(null);
        return;
      }
      const entry: Saved = JSON.parse(raw);
      if (!entry || !new RegExp(`^${uuidPattern}$`).test(entry.id)
        || (entry.pending !== undefined && !["propose", "approve"].includes(entry.pending))) {
        throw new Error("Invalid saved enrollment batch");
      }
      setSaved(entry);
      setId(entry.id);
      setOpen(!!entry.pending);
      void api<Receipt>(`${path}/${entry.id}`).then(result => {
        if (cancelled) return;
        confirmReadback(result, entry);
      }).catch(problem => { if (!cancelled) setError(problem); });
    } catch (problem) { setError(problem); }
    return () => { cancelled = true; };
  }, [key, user.role]);

  function confirmReadback(result: Receipt, entry?: Saved) {
    setReceipt(result);
    if (entry?.id === result.operationId && (entry.pending === "propose" || result.viewerApproved || result.state !== "PENDING")) {
      const known = { id: result.operationId };
      localStorage.setItem(key, JSON.stringify(known));
      setSaved(known);
    }
  }

  async function observe() {
    setBusy(true);
    setError(undefined);
    try {
      const result = await api<Receipt>(`${path}/${id}`);
      confirmReadback(result, saved ?? undefined);
      if (result.state === "APPLIED") onRegistered();
    } catch (problem) { setError(problem); }
    finally { setBusy(false); }
  }

  async function dispatch(entry: Saved, body?: Request) {
    const previous = localStorage.getItem(key);
    if (previous && (JSON.parse(previous) as Saved).pending) throw new ApiError("CA_BATCH_OBSERVE_REQUIRED", 409);
    // If a write's response is lost, reopening the console can only read this ID.
    localStorage.setItem(key, JSON.stringify(entry));
    setSaved(entry);
    setId(entry.id);
    setReceipt(undefined);
    try {
      const url = entry.pending === "approve" ? `${path}/${entry.id}/approve` : path;
      const result = await api<Receipt>(url, "POST", body, user.csrf);
      confirmReadback(result, entry);
      if (result.state === "APPLIED") onRegistered();
    } catch (problem) {
      if (problem instanceof ApiError && [
        "CA_BATCH_CONFLICT", "INVALID_ARGUMENT", "CA_BATCH_CERTIFICATE_INVALID",
      ].includes(problem.code)) {
        const known = { id: entry.id };
        localStorage.setItem(key, JSON.stringify(known));
        setSaved(known);
      }
      throw problem;
    }
  }

  async function propose(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    if (busy || saved === undefined || saved?.pending) return;
    const form = event.currentTarget;
    const values = new FormData(form);
    setBusy(true);
    setError(undefined);
    try {
      const entries = [];
      for (const row of rows) {
        const file = values.get(`certificate-${row}`);
        if (!(file instanceof File) || file.size > 32768) throw new ApiError("CA_BATCH_PUBLIC_FILE", 400);
        const pem = (await file.text()).trim();
        if (!publicPem.test(pem)) throw new ApiError("CA_BATCH_PUBLIC_FILE", 400);
        entries.push({
          id: String(values.get(`id-${row}`)),
          name: String(values.get(`name-${row}`)),
          clientCertificatePem: pem,
        });
      }
      const operationId = crypto.randomUUID();
      await dispatch({ id: operationId, pending: "propose" }, { operationId, entries });
      form.reset();
    } catch (problem) {
      setError(problem);
    } finally { setBusy(false); }
  }

  async function approve(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    if (!receipt || busy || saved === undefined || saved?.pending) return;
    setBusy(true);
    setError(undefined);
    try { await dispatch({ id: receipt.operationId, pending: "approve" }); }
    catch (problem) { setError(problem); }
    finally { setBusy(false); }
  }

  async function downloadRenewal(entry: Receipt["entries"][number]) {
    const renewal = entry.renewal;
    const operationId = renewal?.operationId;
    if (busy || receipt?.state !== "APPLIED" || !renewal || !operationId) return;
    setBusy(true);
    setError(undefined);
    try {
      const original = await api<RotationReceipt>(
        `/gateways/${encodeURIComponent(entry.id)}/certificate-operations/${operationId}`);
      const previous = renewal.binding;
      if (original.id !== entry.id || original.operationId !== operationId
        || original.previousCertificateSha256 !== previous.previousCertificateSha256
        || original.previousBindingGeneration !== previous.previousBindingGeneration
        || original.certificateSha256 !== entry.certificateSha256 || original.bundleRequested !== false) {
        throw new ApiError("RECOVERY_REQUIRED", 409);
      }
      savePublicRotationReceipt(original);
    } catch (problem) { setError(problem); }
    finally { setBusy(false); }
  }

  useEffect(() => {
    if (open && error) document.getElementById(`${label}-error`)?.focus();
  }, [open, error, label]);

  const canReview = saved !== undefined && !saved?.pending;
  useEffect(() => {
    if (!reviewId || !canReview) return;
    setOpen(true);
    setId(reviewId);
    setReceipt(undefined);
    setError(undefined);
  }, [reviewId, canReview]);

  if (user.role !== "admin") return null;
  const disabled = busy || saved === undefined || !!saved?.pending || user.mustChangePassword;
  return <>
    <Button aria-expanded={open} aria-controls={label} onClick={() => setOpen(!open)}>{t("batchEnroll.title")}</Button>
    {open && <section id={label}>
      <Card title={t("batchEnroll.title")}>
        <Space direction="vertical" style={{ width: "100%" }}>
          <p>{t("batchEnroll.description")}</p>
          <form method="post" action={`/api/v1${path}`} aria-label={t("batchEnroll.propose")} onSubmit={event => void propose(event)}>
            <Space direction="vertical" style={{ width: "100%" }}>
              {rows.map((row, index) => <fieldset key={row} disabled={disabled} style={{ maxWidth: "36rem" }}>
                <legend>{t("batchEnroll.gateway")} {index + 1}</legend>
                <Space direction="vertical" style={{ width: "100%" }}>
                  <label htmlFor={`${row}-id`}>{t("registration.id")}</label>
                  <Input id={`${row}-id`} name={`id-${row}`} required pattern="[A-Za-z0-9_\-]{1,128}"
                    autoCorrect="off" autoCapitalize="none" spellCheck={false} />
                  <label htmlFor={`${row}-name`}>{t("registration.name")}</label>
                  <Input id={`${row}-name`} name={`name-${row}`} required maxLength={256} />
                  <label htmlFor={`${row}-cert`}>{t("batchEnroll.certificate")}</label>
                  <input id={`${row}-cert`} name={`certificate-${row}`} type="file" required accept=".pem,.crt" />
                  {rows.length > 2 && <Button htmlType="button" onClick={() => setRows(current => current.filter(value => value !== row))}>
                    {t("batchEnroll.remove")}
                  </Button>}
                </Space>
              </fieldset>)}
              <Space wrap>
                <Button htmlType="button" disabled={disabled || rows.length >= maximum}
                  onClick={() => setRows(current => [...current, crypto.randomUUID()])}>{t("batchEnroll.add")}</Button>
                <Button htmlType="submit" disabled={disabled}>{t("batchEnroll.propose")}</Button>
              </Space>
            </Space>
          </form>
          <form method="get" action={`/api/v1${path}/${id}`} aria-labelledby={`${label}-id-label`}
            onSubmit={event => { event.preventDefault(); void observe(); }}>
            <Space direction="vertical" style={{ width: "100%", maxWidth: "24rem" }}>
              <label id={`${label}-id-label`} htmlFor={`${label}-id`}>{t("batchEnroll.id")}</label>
              <Input id={`${label}-id`} name="operationId" value={id} readOnly={!!saved?.pending}
                required pattern={uuidPattern} autoCorrect="off" autoCapitalize="none" spellCheck={false}
                disabled={busy} onChange={event => { setId(event.target.value); setReceipt(undefined); }} />
              <Button htmlType="submit" disabled={busy}>{t("batchEnroll.observe")}</Button>
            </Space>
          </form>
          {saved?.pending && <Alert type="warning" title={t("batchEnroll.unknown")} />}
          {receipt && <>
            <div role="status">
              <p><Typography.Text copyable code>{receipt.operationId}</Typography.Text></p>
              <p>{t(receipt.state === "APPLIED" && receipt.entries.some(entry => entry.renewal)
                ? "batchEnroll.RENEWED" : `batchEnroll.${receipt.state}`)} · {receipt.approvedCount}/{receipt.requiredCount}</p>
              <p>{t("batchEnroll.anchor")} <Typography.Text code style={{ overflowWrap: "anywhere" }}>{receipt.caFingerprint}</Typography.Text></p>
              {receipt.signingManifestSha256 && <p>{t("caSigning.manifest")}{" "}
                <Typography.Text code style={{overflowWrap: "anywhere"}}>{receipt.signingManifestSha256}</Typography.Text>
              </p>}
              <ul>{receipt.entries.map(entry => <li key={entry.id}>{entry.name} ({entry.id}){" "}
                <Typography.Text code style={{ overflowWrap: "anywhere" }}>{entry.certificateSha256}</Typography.Text>
                {entry.renewal && <div>
                  {t("batchEnroll.pinnedGeneration")} {entry.renewal.binding.previousBindingGeneration}{" "}
                  {entry.renewal.operationId && <>· {t("batchEnroll.receiptGeneration")} {entry.renewal.binding.previousBindingGeneration + 1}{" "}</>}
                  <Typography.Text code style={{overflowWrap: "anywhere"}}>{entry.renewal.binding.previousCertificateSha256}</Typography.Text>
                  {entry.renewal.operationId && <Button disabled={busy} onClick={() => void downloadRenewal(entry)}>
                    {t("batchEnroll.downloadRenewal")} · {entry.id}
                  </Button>}
                </div>}
              </li>)}</ul>
              {receipt.rejectionCode && <p>{t(`batchEnroll.${receipt.rejectionCode}`)}</p>}
            </div>
            {receipt.state === "PENDING" && !receipt.viewerApproved &&
              <form method="post" action={`/api/v1${path}/${receipt.operationId}/approve`}
                aria-label={t("batchEnroll.approve")} onSubmit={event => void approve(event)}>
                <Button htmlType="submit" type="primary" disabled={disabled}>{t("batchEnroll.approve")}</Button>
              </form>}
          </>}
          {error !== undefined && <div id={`${label}-error`} tabIndex={-1}>
            <Alert type="error" title={errorText(error, t)} />
          </div>}
        </Space>
      </Card>
    </section>}
  </>;
}
