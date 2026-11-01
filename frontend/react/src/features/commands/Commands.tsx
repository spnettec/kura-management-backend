import { Alert, Drawer, Table, Tag } from "antd";
import { useState } from "react";
import type { Task } from "../../api/client";
import { LanguageSwitcher, useI18n } from "../../i18n/I18n";
import { translatedCode } from "../../i18n/core";
export function Commands({ items }: { items: Task[] }) {
  const { t, dateTime } = useI18n();
  const [selectedId, setSelectedId] = useState<string>();
  const selected = items.find(task => task.id === selectedId);
  const result = selected?.result as { state?: unknown; submitted?: unknown; reconciliation?: { source?: unknown; state?: unknown; commandId?: unknown; operation?: unknown } } | null;
  const proof = result?.reconciliation;
  const proved = proof?.commandId === selected?.id && proof?.operation === selected?.operation
    && (selected?.state === "SUCCEEDED" || selected?.state === "FAILED")
    && ((proof?.source === "network.journal.v1"
      && [5, 6, 7, 10].includes(selected?.operation ?? 0)
      && proof?.state === result?.state
      && (proof?.state === "COMMITTED" || proof?.state === "ROLLED_BACK"))
      || (proof?.source === "configuration.journal.v1"
        && selected?.operation === 12 && result?.submitted === true));
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
                {[5, 6, 7, 10].includes(task.operation) && task.state === "SUCCEEDED"
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
        {proved && (configurationProof
          ? <Alert type="info" showIcon title={t("task.configurationOutcome")} description={t("task.configurationProofNote")} />
          : <Alert type="info" showIcon title={t("task.provedOutcome", {
              state: translatedCode(t, "state", proof.state as string),
            })} description={t("task.proofNote")} />)}
        <pre className="result">{JSON.stringify(selected, null, 2)}</pre>
      </Drawer>
    </>
  );
}
