import { Alert, Drawer, Table, Tag } from "antd";
import { useState } from "react";
import type { Task } from "../../api/client";
import { LanguageSwitcher, useI18n } from "../../i18n/I18n";
import { translatedCode } from "../../i18n/core";
export function Commands({ items }: { items: Task[] }) {
  const { t, dateTime } = useI18n();
  const [selectedId, setSelectedId] = useState<string>();
  const selected = items.find(task => task.id === selectedId);
  const result = selected?.result as { id?: unknown; state?: unknown; bootVerified?: unknown; kuraRecoveryVerified?: unknown; plan?: { id?: unknown; state?: unknown; bootVerified?: unknown; kuraRecoveryVerified?: unknown }; submitted?: unknown; newManagementConnectionVerified?:unknown; reconciliation?: { source?: unknown; state?: unknown; commandId?: unknown; operation?: unknown; transactionId?:unknown; requestSha256?:unknown; journalSha256?:unknown } } | null;
  const proof = result?.reconciliation;
  const bootValue = selected?.operation === 16 ? result?.plan : result;
  const handoff = result as { scope?: unknown; normalBootVerified?: unknown; ordinaryUnitHandedOff?: unknown; cancelled?: unknown; issued?: unknown } | null;
  const handoffProof = proof?.source === "ordinary.handoff.journal.v1"
    && [21, 22, 29].includes(selected?.operation ?? 0) && selected?.state === "SUCCEEDED"
    && handoff?.scope === "administrator-selected-checks"
    && result?.kuraRecoveryVerified === false && handoff?.normalBootVerified === false
    && result?.state === (selected?.operation === 21 ? "ORDINARY_PREPARED" : selected?.operation === 29 ? "ORDINARY_CANCELLED" : "ORDINARY_ACTIVE_APPROVED_CHECKS")
    && (selected?.operation !== 22 || handoff?.ordinaryUnitHandedOff === true)
    && (selected?.operation !== 29 || handoff?.cancelled === true && handoff?.issued === false && handoff?.ordinaryUnitHandedOff === false);
  const bootProof = proof?.source === "ordinary.boot.journal.v1"
    && [16, 17, 18, 19].includes(selected?.operation ?? 0)
    && bootValue?.bootVerified === false && bootValue?.kuraRecoveryVerified === false
    && bootValue?.state === (selected?.operation === 16 ? "BOOT_POLICY_PREPARED" : selected?.operation === 18 ? "CANCELLED" : "BOOT_POLICY_COMMITTED");
  const systemProof = proof?.source === "system.control.journal.v1"
    && [24,25,26,27,28].includes(selected?.operation??0)
    && ["system-process-only","administrator-selected-checks"].includes(String(handoff?.scope))
    && ([25,27].includes(selected?.operation??0)||selected?.state==="SUCCEEDED")
    && result?.kuraRecoveryVerified===false && handoff?.normalBootVerified===false
    && result?.state===([24,26].includes(selected?.operation??0)?"PREPARED":selected?.operation===28?"CANCELLED":"COMPLETED");
  const firewallProof=proof?.source==="firewall.remote.journal.v1" && proof.transactionId===result?.id
    && [proof.requestSha256,proof.journalSha256].every(v=>typeof v==="string"&&/^[a-f0-9]{64}$/.test(v))
    && ((selected?.state==="SUCCEEDED" && ((selected.operation===31&&["WAIT_CONFIRM","COMMITTED"].includes(String(result?.state)))
      ||(selected.operation===32&&result?.state==="COMMITTED"&&result.newManagementConnectionVerified===true)
      ||(selected.operation===33&&result?.state==="ROLLED_BACK")))
      ||(selected?.state==="FAILED"&&[31,32].includes(selected.operation)&&result?.state==="ROLLED_BACK"));
  const safeSelected=selected&&[30,31,32,33].includes(selected.operation)&&selected.result&&typeof selected.result==="object"
    ? {...selected,result:Object.fromEntries(Object.entries(selected.result).filter(([key])=>key!=="confirmationToken"))}:selected;
  const proved = proof?.commandId === selected?.id && proof?.operation === selected?.operation
    && (selected?.state === "SUCCEEDED" || selected?.state === "FAILED")
    && ((proof?.source === "network.journal.v1"
      && [5, 6, 7, 10].includes(selected?.operation ?? 0)
      && proof?.state === result?.state
      && (proof?.state === "COMMITTED" || proof?.state === "ROLLED_BACK"))
      || (proof?.source === "configuration.journal.v1"
        && selected?.operation === 12 && result?.submitted === true) || bootProof || handoffProof || systemProof || firewallProof);
  const configurationProof = proved && proof?.source === "configuration.journal.v1";
  return (
    <>
      <Table<Task>
        rowKey="id"
        pagination={false}
        dataSource={items}
        columns={[
          {
            title: t("task.name"),
            render: (_, task) => (
              <button
                className="gateway-link"
                onClick={() => setSelectedId(task.id)}
              >
                <strong>
                  {translatedCode(t, "operation", task.operation)}
                </strong>
                <span>{task.id.slice(0, 8)}</span>
              </button>
            ),
          },
          { title: t("task.target"), dataIndex: "gatewayId" },
          {
            title: t("task.state"),
            render: (_, task) => (
              <Tag
                color={
                  task.state === "SUCCEEDED"
                    ? "green"
                    : task.state === "FAILED"
                      ? "red"
                      : task.state === "UNKNOWN" ? "orange" : "default"
                }
              >
                {[5, 6, 7, 10, 31].includes(task.operation) && task.state === "SUCCEEDED"
                  ? t("task.receipt")
                  : translatedCode(t, "state", task.state)}
              </Tag>
            ),
          },
          {
            title: t("task.error"),
            render: (_, task) =>
              task.error ? translatedCode(t, "error", task.error) : "—",
          },
          {
            title: t("task.created"),
            dataIndex: "createdAt",
            render: (s: string) => dateTime(s),
          },
        ]}
      />
      <Drawer
        extra={<LanguageSwitcher />}
        title={t("task.result")}
        open={!!selected}
        onClose={() => setSelectedId(undefined)}
        width={560}
      >
        {selected && [5, 6, 7, 10].includes(selected.operation) && (
          <p className="muted">{t("task.receiptNote")}</p>
        )}
        {selected && [20, 21, 22, 29].includes(selected.operation) && <Alert type="info" showIcon title={t("handoff.title")} description={t("handoff.scope")} />}
        {selected && [23,24,25,26,27,28].includes(selected.operation) && <Alert type="info" showIcon title={t("system.title")} description={t("system.scope")} />}
        {selected && [30,31,32,33].includes(selected.operation) && <Alert type="info" title={t("firewall.title")} description={t("firewall.scope")} />}
          {proved && (firewallProof
            ? <Alert type="info" showIcon title={t("firewall.proof")} />
            : systemProof
          ? <Alert type="info" showIcon title={t("task.systemProof")} />
          : handoffProof
          ? <Alert type="info" showIcon title={t("task.handoffOutcome")} description={t("task.handoffProofNote")} />
          : bootProof
          ? <Alert type="info" showIcon title={t("task.bootOutcome")} description={t("task.bootProofNote")} />
          : configurationProof
          ? <Alert type="info" showIcon title={t("task.configurationOutcome")} description={t("task.configurationProofNote")} />
          : <Alert type="info" showIcon title={t("task.provedOutcome", {
              state: translatedCode(t, "state", proof?.state as string),
            })} description={t("task.proofNote")} />)}
        <pre className="result">{JSON.stringify(safeSelected, null, 2)}</pre>
      </Drawer>
    </>
  );
}
