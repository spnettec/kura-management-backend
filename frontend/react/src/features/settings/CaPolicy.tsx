import { Alert, Button, Card, Checkbox, Input, Space, Typography } from "antd";
import { useCallback, useEffect, useId, useState } from "react";
import { api, ApiError, type Session } from "../../api/client";
import type { components } from "../../api/schema";
import { errorText, useI18n } from "../../i18n/I18n";

type Overview = components["schemas"]["CaPolicyOverview"];
type Proposal = components["schemas"]["CaPolicyProposal"];
type Pending = { id: string; action: "propose" | "approve" };
const path = "/settings/client-ca/policy";
const uuidPattern = "[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}";

export function CaPolicy({ user, onPublished }: { user: Session; onPublished: () => void }) {
  const { t } = useI18n();
  const label = useId();
  const storageKey = `yofc.console.ca-policy.v1:${user.id}`;
  const [data, setData] = useState<Overview>();
  const [count, setCount] = useState(2);
  const [roster, setRoster] = useState<string[]>([]);
  const [id, setId] = useState("");
  const [proposal, setProposal] = useState<Proposal>();
  const [pending, setPending] = useState<Pending | null>();
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<unknown>();
  const [invalid, setInvalid] = useState(false);

  const refresh = useCallback(async () => {
    const next = await api<Overview>(path);
    setData(next);
    setCount(next.policy.requiredCount);
    setRoster(next.eligibleAdmins
      .filter(admin => next.policy.custodianIds?.includes(admin.id) ?? true)
      .map(admin => admin.id));
  }, []);

  const accept = useCallback((result: Proposal, original: Pending | null) => {
    setProposal(result);
    if (original?.id === result.change.operationId &&
        (original.action === "propose" || result.viewerApproved || result.appliedRevision !== null || result.stale)) {
      localStorage.removeItem(storageKey);
      setPending(null);
    }
    if (result.appliedRevision !== null) {
      onPublished();
      void refresh().catch(setError);
    }
  }, [onPublished, refresh, storageKey]);

  useEffect(() => {
    void refresh().catch(setError);
    try {
      const raw = localStorage.getItem(storageKey);
      if (!raw) {
        setPending(null);
        return;
      }
      const saved: Pending = JSON.parse(raw);
      if (!saved || !new RegExp(`^${uuidPattern}$`).test(saved.id)
          || !["propose", "approve"].includes(saved.action)) throw new Error("Invalid saved CA policy reference");
      setId(saved.id);
      setPending(saved);
      void api<Proposal>(`${path}/proposals/${saved.id}`).then(result => accept(result, saved)).catch(setError);
    } catch (e) { setError(e); }
  }, [accept, refresh, storageKey]);
  useEffect(() => {
    if (invalid) document.getElementById(`${label}-roster-error`)?.focus();
  }, [invalid, label]);

  async function observe() {
    setBusy(true);
    setError(undefined);
    setProposal(undefined);
    try { accept(await api<Proposal>(`${path}/proposals/${id}`), pending ?? null); }
    catch (e) { setError(e); }
    finally { setBusy(false); }
  }

  async function submit(original: Pending, body?: components["schemas"]["CaPolicyChange"]) {
    if (busy || pending !== null) return;
    setBusy(true);
    setError(undefined);
    try {
      // Preserve the original identifier before sending any write.
      if (localStorage.getItem(storageKey)) {
        throw new ApiError("CA_POLICY_OBSERVE_REQUIRED", 409);
      }
      localStorage.setItem(storageKey, JSON.stringify(original));
      setPending(original);
      setId(original.id);
      setProposal(undefined);
      const url = original.action === "approve" ? `${path}/proposals/${original.id}/approve` : `${path}/proposals`;
      accept(await api<Proposal>(url, "POST", body, user.csrf), original);
    } catch (e) {
      setError(e);
      if (e instanceof ApiError && (e.code === "CA_POLICY_CONFLICT" || e.code === "INVALID_INPUT")) {
        try {
          localStorage.removeItem(storageKey);
          setPending(null);
        } catch (storageError) { setError(storageError); }
      }
    } finally { setBusy(false); }
  }

  const names = (ids: string[]) => ids.map(id => data?.eligibleAdmins.find(admin => admin.id === id)?.username ?? id).join(", ");
  const formBusy = busy || pending !== null || !data?.viewerCanPropose;
  return <Card title={t("settings.caPolicy.title")}>
    <Space direction="vertical" style={{ width: "100%" }}>
      {data && <>
        <p>{t("settings.caPolicy.current")}{` ${data.policy.revision} · ${data.policy.requiredCount}`}</p>
        <p>{data.policy.custodianIds ? names(data.policy.custodianIds) : t("settings.caPolicy.defaultRoster")}</p>
        <Button disabled={busy} onClick={() => { setError(undefined); void refresh().catch(setError); }}>
          {t("settings.activation.refresh")}
        </Button>
        {data.viewerCanPropose && <form method="post" action={`/api/v1${path}/proposals`}
          aria-label={t("settings.caPolicy.propose")} onSubmit={event => {
            event.preventDefault();
            if (roster.length < count) {
              setInvalid(true);
              return;
            }
            const change = { operationId: crypto.randomUUID(), expectedRevision: data.policy.revision,
              requiredCount: count, custodianIds: roster };
            void submit({ id: change.operationId, action: "propose" }, change);
          }}>
          <Space direction="vertical" style={{ width: "100%" }}>
            <label htmlFor={`${label}-count`}>{t("settings.caPolicy.requiredCount")}</label>
            <Input id={`${label}-count`} name="requiredCount" type="number" min={2} max={32767} step={1}
              required disabled={formBusy} value={Number.isFinite(count) ? count : ""}
              onChange={event => { setCount(event.target.valueAsNumber); setInvalid(false); }} />
            <fieldset disabled={formBusy} aria-describedby={invalid ? `${label}-roster-error` : undefined}>
              <legend>{t("settings.caPolicy.roster")}</legend>
              <Space direction="vertical">
                {data.eligibleAdmins.map(admin => <Checkbox key={admin.id} value={admin.id} name="custodianIds"
                  checked={roster.includes(admin.id)} disabled={formBusy} onChange={event => {
                    setInvalid(false);
                    setRoster(ids => event.target.checked ? [...ids, admin.id] : ids.filter(id => id !== admin.id));
                  }}>{admin.username}</Checkbox>)}
              </Space>
            </fieldset>
            {invalid && <p id={`${label}-roster-error`} tabIndex={-1} role="alert">{t("settings.caPolicy.rosterTooSmall")}</p>}
            <Button htmlType="submit" disabled={formBusy}>{t("settings.caPolicy.propose")}</Button>
          </Space>
        </form>}
      </>}
      <form method="get" action={`/api/v1${path}/proposals/${id}`} aria-labelledby={`${label}-id-label`}
        onSubmit={event => { event.preventDefault(); void observe(); }}>
        <Space direction="vertical">
          <label id={`${label}-id-label`} htmlFor={`${label}-id`}>{t("settings.caPolicy.proposalId")}</label>
          <Input id={`${label}-id`} name="operationId" required pattern={uuidPattern}
            autoCorrect="off" autoCapitalize="none" spellCheck={false}
            disabled={busy || !!pending} value={id} onChange={event => { setId(event.target.value); setProposal(undefined); }} />
          <Button htmlType="submit" disabled={busy}>{t("settings.caPolicy.observe")}</Button>
        </Space>
      </form>
      {pending && <>
        <Typography.Text copyable code>{pending.id}</Typography.Text>
        <Alert type="warning" title={t("settings.caPolicy.unknown")} />
      </>}
      {proposal && <>
        <div role="status">
          <Typography.Text copyable code>{proposal.change.operationId}</Typography.Text>
          <p>{t("settings.caPolicy.newThreshold")}{` ${proposal.change.requiredCount} · ${names(proposal.change.custodianIds)}`}</p>
          <p>{t("settings.activation.approvalCount")}{` ${proposal.approvedCount}/${proposal.approvalRequiredCount}`}</p>
          {proposal.appliedRevision !== null && <p>{t("settings.caPolicy.applied")}{` ${proposal.appliedRevision}`}</p>}
          {proposal.stale && <p>{t("settings.caPolicy.stale")}</p>}
          {proposal.viewerApproved && <p>{t("settings.caPolicy.voted")}</p>}
        </div>
        {!proposal.stale && proposal.appliedRevision === null && !proposal.viewerApproved && data?.viewerCanPropose &&
          <Button type="primary" disabled={formBusy}
            onClick={() => void submit({ id: proposal.change.operationId, action: "approve" })}>
            {t("settings.caPolicy.approve")}
          </Button>}
      </>}
      {error !== undefined && <Alert type="error" title={errorText(error, t)} />}
    </Space>
  </Card>;
}
