import { RegisterGateway } from "./RegisterGateway";
import { BulkEnrollment } from "./BulkEnrollment";
import { RotateCertificate } from "./RotateCertificate";
import { Button, Drawer, Empty, Space, Table, Tag } from "antd";
import { useState } from "react";
import type { Gateway, Session } from "../../api/client";
import { BootPanel } from "./BootPanel";
import { HandoffPanel } from "./HandoffPanel";
import { FirewallPanel } from "./FirewallPanel";
import { NmPanel } from "./NmPanel";
import { ImportedFirewallPanel } from "./ImportedFirewallPanel";
import { SystemControlPanel } from "./SystemControlPanel";
import { NetworkPanel } from "./NetworkPanel";
import { TransferPanel } from "./TransferPanel";
import { LanguageSwitcher, useI18n } from "../../i18n/I18n";
import { capabilityLabel } from "../../i18n/core";
import { ConfigurationPanel } from "../configuration/ConfigurationPanel";
import { SnapshotPanel } from "../snapshots/SnapshotPanel";
export function Gateways({
  items,
  user,
  onCommand,
  onRegistered,
}: {
  onRegistered: () => void;
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
      <RegisterGateway key={user.id} user={user} onRegistered={onRegistered} />
      <BulkEnrollment key={`bulk:${user.id}`} user={user} onRegistered={onRegistered} />
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
            <Space wrap>
              <Button
                disabled={!canOperate || !selected.online}
                onClick={() => onCommand(selected.id, "kura.snapshot.list")}
              >
                {t("gateway.snapshots")}
              </Button>
              {user.role === "admin" && (
                <RotateCertificate key={`rotate:${user.id}:${selected.id}`} gateway={selected} user={user} />
              )}
            </Space>
              {selected.capabilities.includes("kura.configuration.list") && <ConfigurationPanel
                key={`configuration:${user.id}:${selected.id}`} gateway={selected} user={user} />}
            {selected.capabilities.includes("kura.snapshot.write") && <SnapshotPanel key={`snapshot:${user.id}:${selected.id}`} gateway={selected} user={user} />}
            {selected.capabilities.includes("network.inspect") && (
              <NetworkPanel
                key={`${user.id}:${selected.id}`}
                gatewayId={selected.id}
                user={user}
              />
            )}
            {user.role === "admin" && selected.capabilities.includes("ordinary.boot.v1") && <BootPanel key={`boot:${user.id}:${selected.id}`} gateway={selected} user={user} />}
            {user.role === "admin" && selected.capabilities.includes("recovery.ordinary.v1") && <HandoffPanel key={`handoff:${user.id}:${selected.id}`} gateway={selected} user={user} />}
            {user.role === "admin" && selected.capabilities.includes("firewall.remote.v1") && <FirewallPanel key={`firewall:${user.id}:${selected.id}:${selected.bindingGeneration}`} gateway={selected} user={user} />}
            {user.role === "admin" && selected.capabilities.includes("nm.remote.v1") && <NmPanel key={`nm:${user.id}:${selected.id}:${selected.bindingGeneration}`} gateway={selected} user={user} />}
              {user.role === "admin" && selected.capabilities.includes("firewall.imported.original.v1") && <ImportedFirewallPanel key={`imported:${user.id}:${selected.id}:${selected.bindingGeneration}`} gateway={selected} user={user} />}
            {user.role === "admin" && selected.capabilities.includes("system.control.v1") && <SystemControlPanel key={`system:${user.id}:${selected.id}`} gateway={selected} user={user} />}
            {user.role === "admin" && (
              <TransferPanel
                key={`transfer:${user.id}:${selected.id}`}
                gateway={selected}
                user={user}
              />
            )}
          </>
        )}
      </Drawer>
    </>
  );
}
