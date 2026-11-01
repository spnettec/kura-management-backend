import { Button, Drawer, Empty, Space, Table, Tag } from "antd";
import { useState } from "react";
import type { Gateway, Session } from "../../api/client";
import { NetworkPanel } from "./NetworkPanel";
import { LanguageSwitcher, useI18n } from "../../i18n/I18n";
import { capabilityLabel } from "../../i18n/core";
import { ConfigurationPanel } from "../configuration/ConfigurationPanel";
export function Gateways({
  items,
  user,
  onCommand,
}: {
  items: Gateway[];
  user: Session;
  onCommand: (id: string, op: string) => void;
}) {
  const { t, dateTime } = useI18n();
  const [selectedId, setSelectedId] = useState<string>();
  const selected = items.find(g => g.id === selectedId);
  const canOperate = ["operator", "admin"].includes(user.role);
  return (
    <>
      <Table<Gateway>
        rowKey="id"
        dataSource={items}
        pagination={false}
        locale={{ emptyText: <Empty description={t("gateway.empty")} /> }}
        columns={[
          {
            title: t("gateway.name"),
            key: "name",
            render: (_, g) => (
                <button className="gateway-link" onClick={() => setSelectedId(g.id)}>
                <strong>{g.name}</strong>
                <span>{g.id}</span>
              </button>
            ),
          },
          {
            title: t("gateway.channel"),
            key: "online",
            render: (_, g) => (
              <Tag color={g.online ? "green" : "default"}>
                {g.online ? t("gateway.online") : t("gateway.offline")}
              </Tag>
            ),
          },
          {
            title: "Kura",
            key: "kura",
            render: (_, g) => (
              <span className={g.kuraAvailable && g.online ? "" : "muted"}>
                {g.kuraAvailable && g.online
                  ? t("gateway.connected")
                  : t("gateway.unavailable")}
              </span>
            ),
          },
          {
            title: t("gateway.heartbeat"),
            dataIndex: "lastSeen",
            render: (v: string | null) =>
              v ? dateTime(v) : t("gateway.waiting"),
          },
          {
            title: t("gateway.actions"),
            key: "actions",
            render: (_, g) => (
              <Button
                disabled={!canOperate || !g.online}
                onClick={() => onCommand(g.id, "agent.status")}
              >
                {t("gateway.status")}
              </Button>
            ),
          },
        ]}
      />
      <Drawer
        extra={<LanguageSwitcher />}
        title={selected?.name}
        open={!!selected}
          onClose={() => setSelectedId(undefined)}
        width={720}
      >
        {selected && (
          <>
            <p className="eyebrow">{selected.id}</p>
            <h3>{t("gateway.capabilities")}</h3>
            <Space wrap>
              {selected.capabilities.map((c) => (
                <Tag key={c} title={c}>
                  {capabilityLabel(t, c)}
                </Tag>
              ))}
            </Space>
            <p className="muted">
              Agent {selected.agentVersion ?? t("gateway.disconnected")} ·{" "}
              {t("gateway.revision")} {selected.revision}
            </p>
            <Space direction="vertical">
              <Button
                disabled={!canOperate || !selected.online}
                onClick={() => onCommand(selected.id, "kura.snapshot.list")}
              >
                {t("gateway.snapshots")}
              </Button>
            </Space>
              {selected.capabilities.includes("kura.configuration.list") && <ConfigurationPanel
                key={`configuration:${user.id}:${selected.id}`} gateway={selected} user={user} />}
            {selected.capabilities.includes("network.inspect") && (
              <NetworkPanel
                key={`${user.id}:${selected.id}`}
                gatewayId={selected.id}
                user={user}
              />
            )}
          </>
        )}
      </Drawer>
    </>
  );
}
