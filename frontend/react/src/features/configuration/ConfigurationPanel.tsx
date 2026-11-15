import { Alert, Button, Empty, Select, Space, Table, Tag } from "antd";
import { useEffect, useRef, useState } from "react";
import type { Gateway, Session } from "../../api/client";
import { errorText, useI18n } from "../../i18n/I18n";
import { configuration, configurationPids, readTask, type Configuration, type Property } from "./model";
import { ConfigurationEditor } from "./ConfigurationEditor";

export function ConfigurationPanel({ gateway, user }: { gateway: Gateway; user: Session }) {
  const { t, locale } = useI18n();
  const [pids, setPids] = useState<string[]>();
  const [pid, setPid] = useState<string>();
  const [view, setView] = useState<Configuration>();
  const [error, setError] = useState<unknown>(null);
  const [busy, setBusy] = useState(false);
  const pending = useRef<AbortController | null>(null);
  useEffect(() => () => pending.current?.abort(), []);
  const allowed = ["operator", "admin"].includes(user.role) && gateway.online;

  async function read(list: boolean) {
    const controller = new AbortController();
    pending.current?.abort();
    pending.current = controller;
    setBusy(true); setError(null); setView(undefined);
    try {
      const result = await readTask(gateway.id, user, list ? "kura.configuration.list" : "kura.configuration.get",
        list ? undefined : { pid: pid!, locale: locale === "zh-CN" ? "zh" : "en" }, controller.signal);
      controller.signal.throwIfAborted();
      if (list) {
        const values = configurationPids(result);
        setPids(values);
        setPid(previous => previous && values.includes(previous) ? previous : values[0]);
      } else setView(configuration(result, pid!));
    } catch (e) {
      if (!controller.signal.aborted) setError(e);
    } finally {
      if (!controller.signal.aborted) setBusy(false);
    }
  }
  return (
    <section className="configuration-panel" aria-label={t("configuration.region")}>
      <h3>{t("configuration.title")}</h3>
      <p className="muted">{t("configuration.note")}</p>
      {error != null && <Alert type="error" showIcon title={errorText(error, t)} />}
      <Space orientation="vertical" style={{ width: "100%" }}>
        <Button disabled={!allowed || busy} loading={busy} onClick={() => void read(true)}>{t("gateway.configurations")}</Button>
        {pids?.length === 0 && <Empty description={t("configuration.empty")} />}
        {!!pids?.length && <>
          <label htmlFor="configuration-pid">{t("configuration.pid")}</label>
          <Select id="configuration-pid" aria-label={t("configuration.pid")} showSearch
            placeholder={t("configuration.choose")} value={pid} disabled={busy}
            style={{ width: "100%" }} options={pids.map(value => ({ value, label: value }))}
            onChange={value => { setPid(value); setView(undefined); setError(null); }} />
          <Button disabled={!allowed || busy || !pid || !gateway.capabilities.includes("kura.configuration.get")}
            onClick={() => void read(false)}>{t("configuration.read")}</Button>
        </>}
      </Space>
      {view && <>
        <h4>{view.name || view.pid}</h4>
        <p className="muted">{view.description}</p>
        <p className="muted">{t("configuration.metadata", { language: t(view.locale === "zh" ? "configuration.zh" : "configuration.en") })}</p>
        <ConfigurationEditor key={`${gateway.id}:${view.pid}:${view.revision ?? "read"}`} view={view} gateway={gateway} user={user} />
        <Table<Property> rowKey="id" size="small" dataSource={view.properties} pagination={{ pageSize: 20 }}
          scroll={{ x: 580 }} locale={{ emptyText: t("configuration.noProperties") }} columns={[
            { title: t("configuration.field"), key: "field", width: 230, render: (_, field) => <div>
              <strong>{field.name || field.id}</strong><div className="muted">{field.id}</div><div>{field.description}</div>
            </div> },
            { title: t("configuration.type"), dataIndex: "type", width: 110 },
            { title: t("configuration.applicability"), key: "applicability", width: 200, render: (_, field) =>
              field.readOnlyReason ? <span className="muted">{t(`configuration.readOnly.${field.readOnlyReason}`)}</span>
                : <span>{t(field.editable === true ? "configuration.fieldSupported" : "configuration.fieldReadOnly")}</span> },
            { title: t("configuration.value"), key: "value", render: (_, field) => field.redacted
              ? <Tag>{t(field.redaction === "SECRET" ? "configuration.redacted" : "configuration.unsupported")}</Tag>
              : !field.present ? <span className="muted">{t("configuration.absent")}</span>
              : <pre className="configuration-value">{JSON.stringify(field.value, null, 2)}</pre> },
          ]} />
      </>}
    </section>
  );
}
