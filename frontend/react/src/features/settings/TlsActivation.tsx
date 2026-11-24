import { Alert, Button, Card, Checkbox, Input, Space, Typography } from "antd";
import { useCallback, useEffect, useId, useState } from "react";
import { api, type Session } from "../../api/client";
import { errorText, useI18n } from "../../i18n/I18n";

type ActivationStatus = {
  generation: string;
  healthy: boolean;
  certificateFingerprint: string;
  caFingerprint: string;
  pendingTlsFingerprint: string | null;
  pendingCaFingerprint: string | null;
  pendingRevision: string;
  activationRequired: boolean;
  registeredGateways: number;
  blockingOperationId: string | null;
  blockingCsrRequestId?: string | null;
};
type ActivationRequest = { operationId: string; expectedGeneration: string; pendingRevision: string };
type Approval = ActivationRequest & { approvedCount: number; requiredCount: number; quorumMet: boolean };
type Operation = { operationId: string; state: "APPLIED" | "UNKNOWN" | "REJECTED"; observedActive: boolean; request: ActivationRequest };
const activationPath = "/settings/tls-activation";
const uuidPattern = "[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}";

export function TlsActivation({ user, revision, onApplied }: { user: Session; revision: number; onApplied: () => void }) {
  const { t } = useI18n();
  const storageKey = `yofc.console.tls-activation.v1:${user.username}`;
  const [status, setStatus] = useState<ActivationStatus>();
  // Undefined means storage has not been read successfully. Dispatch stays off.
  const [request, setRequest] = useState<ActivationRequest | null>();
  const [operation, setOperation] = useState<Operation>();
  const [confirmed, setConfirmed] = useState(false);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<unknown>();
  const [proposalId, setProposalId] = useState("");
  const [approval, setApproval] = useState<Approval>();
  const proposalInput = useId();
  const refresh = useCallback(async () => {
    setConfirmed(false);
    try { setStatus(await api<ActivationStatus>("/settings/tls-activation")); }
    catch (e) { setError(e); }
  }, []);
  const observe = useCallback(async (saved: ActivationRequest) => {
    try {
      const result = await api<Operation>(`/settings/tls-activation/${saved.operationId}`);
      setOperation(result);
      if (result.state === "APPLIED") onApplied();
      setError(undefined);
      await refresh();
    } catch (e) { setError(e); }
  }, [refresh, onApplied]);
  useEffect(() => { void refresh(); }, [refresh, revision]);
  useEffect(() => {
    if (!status?.blockingOperationId || request !== null) return;
    let cancelled = false;
    void api<Operation>(`/settings/tls-activation/${status.blockingOperationId}`).then(original => {
      if (cancelled) return;
      localStorage.setItem(storageKey, JSON.stringify(original.request));
      setRequest(original.request);
      setOperation(original);
    }).catch(setError);
    return () => { cancelled = true; };
  }, [status?.blockingOperationId, request, storageKey]);
  useEffect(() => {
    function restore() {
      try {
        const raw = localStorage.getItem(storageKey);
        if (!raw) { setRequest(null); return; }
        const saved: ActivationRequest = JSON.parse(raw);
        const uuid = new RegExp(`^${uuidPattern}$`);
        if (!saved || !uuid.test(saved.operationId) || !uuid.test(saved.expectedGeneration)
          || !/^[0-9a-f]{64}$/.test(saved.pendingRevision)) throw new Error("Invalid saved activation");
        setRequest(saved);
        void observe(saved); // Read the original operation; never repeat POST.
      } catch (e) { setRequest(undefined); setError(e); }
    }
    restore();
    const changed = (event: StorageEvent) => { if (event.key === storageKey) restore(); };
    window.addEventListener("storage", changed);
    return () => window.removeEventListener("storage", changed);
  }, [storageKey, observe]);

  const canReview = !busy && request === null && status?.healthy && status.activationRequired
    && !status.blockingOperationId && !status.blockingCsrRequestId;
  const currentApproval = approval?.operationId === proposalId
    && approval.expectedGeneration === status?.generation
    && approval.pendingRevision === status?.pendingRevision ? approval : undefined;

  async function reviewApproval() {
    setBusy(true);
    setError(undefined);
    setConfirmed(false);
    try {
      const review = await api<Approval>(`${activationPath}/approvals/${proposalId}`);
      setProposalId(review.operationId);
      setApproval(review);
    } catch (e) { setError(e); }
    finally { setBusy(false); }
  }

  async function approve() {
    if (!canReview || !status?.pendingCaFingerprint || !confirmed || (proposalId && !currentApproval)) return;
    setBusy(true);
    setError(undefined);
    setConfirmed(false);
    const proposal = {
      operationId: currentApproval?.operationId ?? crypto.randomUUID(),
      expectedGeneration: status.generation,
      pendingRevision: status.pendingRevision,
    };
    setProposalId(proposal.operationId);
    try {
      setApproval(await api<Approval>(`${activationPath}/approvals`, "POST", proposal, user.csrf));
    } catch (e) { setError(e); }
    finally { setBusy(false); }
  }

  async function activate() {
    if (!status || !canReview || !confirmed || (status.pendingCaFingerprint && !currentApproval?.quorumMet)) return;
    setBusy(true);
    setError(undefined);
    setConfirmed(false);
    const saved = {
      operationId: status.pendingCaFingerprint ? currentApproval!.operationId : crypto.randomUUID(),
      expectedGeneration: status.generation,
      pendingRevision: status.pendingRevision,
    };
    try {
      // Saving must succeed before dispatch. A lost response keeps this id across
      // reloads; even a subsequent 404 never authorizes a second activation.
      localStorage.setItem(storageKey, JSON.stringify(saved));
      setRequest(saved);
      const result = await api<Operation>("/settings/tls-activation", "POST", saved, user.csrf);
      setOperation(result);
      if (result.state === "APPLIED") onApplied();
      await refresh();
    } catch (e) { setError(e); }
    finally { setBusy(false); }
  }
  function finish() {
    if (!operation || !["APPLIED", "REJECTED"].includes(operation.state)) return;
    try {
      localStorage.removeItem(storageKey);
      setRequest(null);
      setOperation(undefined);
      setError(undefined);
      setProposalId("");
      setApproval(undefined);
      void refresh();
      onApplied();
    } catch (e) { setError(e); }
  }
  const fingerprint = (label: string, value?: string | null) => value && <p>
    {label}<br /><Typography.Text code style={{ overflowWrap: "anywhere" }}>{value}</Typography.Text>
  </p>;
  return <Card title={t("settings.activation.title")}>
    <Space direction="vertical" style={{ width: "100%" }}>
      <Alert type="info" title={t("settings.activation.description")} />
      {status && <>
        {fingerprint(t("settings.activation.tls"), status.pendingTlsFingerprint ?? status.certificateFingerprint)}
        {fingerprint(t("settings.activation.ca"), status.pendingCaFingerprint ?? status.caFingerprint)}
        <p>{t("settings.ca.gateways")}: <strong>{status.registeredGateways}</strong></p>
        {!status.healthy && <Alert type="error" title={t("settings.activation.degraded")} />}
          {status.blockingCsrRequestId && <Alert type="warning" title={t("csr.UNKNOWN")} description={status.blockingCsrRequestId} />}
      </>}
      <Button disabled={busy} onClick={() => void refresh()}>{t("settings.activation.refresh")}</Button>
      {status?.pendingCaFingerprint && request === null && <>
        <form method="get" action={`/api/v1${activationPath}/approvals/${proposalId}`}
          onSubmit={event => { event.preventDefault(); void reviewApproval(); }}>
          <Space direction="vertical">
            <label htmlFor={proposalInput}>{t("settings.activation.proposalId")}</label>
            <Input id={proposalInput} name="operationId" value={proposalId} required pattern={uuidPattern}
              autoCorrect="off" autoCapitalize="none" spellCheck={false} disabled={busy}
              onChange={event => { setProposalId(event.target.value); setApproval(undefined); setConfirmed(false); }} />
            <Button htmlType="submit" disabled={busy}>{t("settings.activation.reviewApproval")}</Button>
          </Space>
        </form>
        {proposalId && currentApproval && <p role="status">
          <Typography.Text copyable code>{proposalId}</Typography.Text>{" "}
          {t("settings.activation.approvalCount")}: {currentApproval.approvedCount}/{currentApproval.requiredCount}
        </p>}
        {approval && !currentApproval && <Alert type="warning" title={t("settings.activation.approvalStale")} />}
      </>}
      <Checkbox checked={confirmed} disabled={!canReview}
        onChange={event => setConfirmed(event.target.checked)}>{t("settings.activation.confirm")}</Checkbox>
      {status?.pendingCaFingerprint && request === null &&
        <Button disabled={!canReview || !confirmed || !!(proposalId && !currentApproval)} onClick={() => void approve()}>
          {t("settings.activation.approve")}
        </Button>}
      <Button type="primary" loading={busy} disabled={!canReview || !confirmed || !!(status?.pendingCaFingerprint && !currentApproval?.quorumMet)}
        onClick={() => void activate()}>{t("settings.activation.apply")}</Button>
      {request && <>
        <Typography.Text copyable code>{request.operationId}</Typography.Text>
        <Alert type={operation?.state === "APPLIED" ? "success" : "warning"}
          title={operation?.state === "APPLIED" ? t("settings.activation.applied")
            : operation?.state === "REJECTED" ? t("settings.activation.rejected") : t("settings.activation.unknown")}
          description={operation?.observedActive ? t("settings.activation.observed") : undefined} />
        <Button disabled={busy} onClick={() => void observe(request)}>{t("settings.activation.observe")}</Button>
        {operation && ["APPLIED", "REJECTED"].includes(operation.state) &&
          <Button disabled={busy} onClick={finish}>{t("settings.activation.finish")}</Button>}
      </>}
      {error !== undefined && <Alert type="error" title={errorText(error, t)} />}
    </Space>
  </Card>;
}
