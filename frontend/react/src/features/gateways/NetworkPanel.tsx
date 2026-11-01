import {
  Alert,
  Button,
  Checkbox,
  Form,
  Input,
  InputNumber,
  Space,
  Switch,
  Table,
  Tag,
} from "antd";
import { useEffect, useRef, useState } from "react";
import { api, ApiError, type Session, type Task } from "../../api/client";
import type { components } from "../../api/schema";
import { errorText, useI18n, useLocalizedForm } from "../../i18n/I18n";
import { translatedCode } from "../../i18n/core";
type Change = components["schemas"]["NetworkChange"];
type Action = components["schemas"]["NetworkAction"];
type Takeover = components["schemas"]["NetworkTakeover"];
type TakeoverPlan = {
  eligible: boolean;
  interface: string;
  diagnostic?: string;
  revision?: number;
  sourceDigest?: string;
  planDigest?: string;
  sourcePath?: string;
  sourceMode?: string;
  preservedInterface?: unknown;
  preservedOtherInterfaces?: string[];
};
type Transaction = {
  id: string;
  state: string;
  revision: number;
  deadlineUnixMs: number;
  confirmationToken: string;
  diagnostic: string | null;
};
type Inspection = {
  revision: number;
  sourceDigest: string;
  sources: { path: string; sha256: string }[];
  interfaces: {
    name: string;
    mac: string;
    carrier: string;
    mtu: string;
    allowed: boolean;
    physical: boolean;
  }[];
  transaction: Transaction | null;
  writeEnabled: boolean;
};
type Fields = {
  interface: string;
  dhcp4: boolean;
  addresses: string;
  nameservers: string;
  search: string;
  mtu: number;
  dhcpMetric: number;
  optional: boolean;
  confirmSeconds: number;
  routes: { to: string; via: string; metric: number }[];
};
const split = (s = "") => s.split(/[,\s]+/).filter(Boolean);
export function NetworkPanel({
  gatewayId,
  user,
}: {
  gatewayId: string;
  user: Session;
}) {
  const { t } = useI18n();
  const [inspection, setInspection] = useState<Inspection>();
  const [transaction, setTransaction] = useState<Transaction | null>();
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<unknown>(null);
  const [now, setNow] = useState(Date.now());
  const [takeoverInterface, setTakeoverInterface] = useState("");
  const [takeoverPlan, setTakeoverPlan] = useState<TakeoverPlan>();
  const [reviewed, setReviewed] = useState(false);
  const [form] = Form.useForm<Fields>();
  useLocalizedForm(form);
  const mounted = useRef(false);
  const transactionRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (transaction) transactionRef.current?.scrollIntoView({ block: "start" });
  }, [transaction?.id, transaction?.state]);
  useEffect(() => {
    mounted.current = true;
    const timer = setInterval(() => setNow(Date.now()), 1000);
    return () => {
      mounted.current = false;
      clearInterval(timer);
    };
  }, []);
  async function task(
    operation: string,
    extra: {
      networkChange?: Change;
      networkAction?: Action;
      networkTakeover?: Takeover;
    } = {},
  ) {
    const accepted = await api<{ id: string }>(
      `/gateways/${encodeURIComponent(gatewayId)}/tasks`,
      "POST",
      { operation, idempotencyKey: crypto.randomUUID(), ...extra },
      user.csrf,
    );
    const deadline = Date.now() + 35000;
    while (mounted.current && Date.now() < deadline) {
      const list = await api<{ items: Task[] }>("/tasks");
      const result = list.items.find((t) => t.id === accepted.id);
      if (result && !["QUEUED", "RUNNING"].includes(result.state)) {
        if (result.state !== "SUCCEEDED")
          throw new ApiError(result.error ?? result.state, 0);
        return result.result;
      }
      await new Promise((resolve) => setTimeout(resolve, 800));
    }
    throw new ApiError("TASK_PENDING", 0);
  }
  async function run(work: () => Promise<void>) {
    setBusy(true);
    setError(null);
    try {
      await work();
    } catch (e) {
      if (mounted.current) setError(e);
    } finally {
      if (mounted.current) setBusy(false);
    }
  }
  async function inspect() {
    const value = (await task("network.inspect")) as Inspection;
    if (mounted.current) {
      setInspection(value);
      setTransaction(value.transaction);
    }
  }
  async function prepare(fields: Fields) {
    if (!inspection) return;
    const networkChange: Change = {
      expectedRevision: inspection.revision,
      sourceDigest: inspection.sourceDigest,
      confirmSeconds: fields.confirmSeconds,
      config: {
        interface: fields.interface,
        dhcp4: fields.dhcp4,
        addresses: split(fields.addresses),
        nameservers: split(fields.nameservers),
        searchDomains: split(fields.search),
        mtu: fields.mtu,
        optional: fields.optional,
        dhcpMetric: fields.dhcpMetric,
        routes: fields.routes ?? [],
      },
    };
    const value = (await task("network.prepare", {
      networkChange,
    })) as Transaction;
    if (mounted.current) setTransaction(value);
  }
  async function action(operation: string) {
    if (!transaction) return;
    const value = (await task(operation, {
      networkAction: {
        transactionId: transaction.id,
        confirmationToken: transaction.confirmationToken,
      },
    })) as Transaction;
    if (mounted.current) setTransaction(value);
    if (value.state === "COMMITTED" || value.state === "ROLLED_BACK")
      await inspect();
  }
  const seconds = transaction
    ? Math.max(0, Math.ceil((transaction.deadlineUnixMs - now) / 1000))
    : 0;
  return (
    <section aria-label={t("network.region")}>
      <h3>{t("network.title")}</h3>
      <p className="muted">{t("network.description")}</p>
      {error != null && <Alert type="error" title={errorText(error, t)} />}
      <Button
        loading={busy}
        disabled={!["operator", "admin"].includes(user.role)}
        onClick={() => void run(inspect)}
      >
        {t("network.inspect")}
      </Button>
      {transaction && (
        <div
          ref={transactionRef}
          style={{ marginTop: 16, scrollMarginTop: 16 }}
        >
          <Tag
            color={
              transaction.state === "COMMITTED"
                ? "green"
                : transaction.state === "WAIT_CONFIRM"
                  ? "gold"
                  : "default"
            }
          >
            <span>{translatedCode(t, "state", transaction.state)}</span>{" "}
            <code>{transaction.state}</code>
          </Tag>
          <p>{t("network.transaction", { id: transaction.id })}</p>
          {transaction.state === "WAIT_CONFIRM" && (
            <Alert
              type="warning"
              title={t("network.countdown", { seconds })}
              description={t("network.confirmNote")}
            />
          )}
          {transaction.diagnostic && (
            <Alert
              type="error"
              title={t("network.diagnostic")}
              description={
                <details>
                  <summary>{t("network.details")}</summary>
                  {transaction.diagnostic}
                </details>
              }
            />
          )}
          <Space wrap>
            <Button
              loading={busy}
              onClick={() => void run(() => action("network.transaction"))}
            >
              {t("network.refresh")}
            </Button>
            {user.role === "admin" && transaction.state === "WAIT_CONFIRM" && (
              <>
                <Button
                  type="primary"
                  disabled={busy || seconds === 0}
                  onClick={() => void run(() => action("network.confirm"))}
                >
                  {t("network.confirm")}
                </Button>
                <Button
                  disabled={busy}
                  onClick={() => void run(() => action("network.rollback"))}
                >
                  {t("network.rollback")}
                </Button>
              </>
            )}
          </Space>
        </div>
      )}
      {inspection && (
        <>
          <p>
            {t("gateway.revision")} <strong>{inspection.revision}</strong> ·{" "}
            {inspection.writeEnabled
              ? t("network.delegated")
              : t("network.observation")}
          </p>
          <Table
            rowKey="name"
            size="small"
            pagination={false}
            dataSource={inspection.interfaces}
            columns={[
              { title: t("network.interface"), dataIndex: "name" },
              { title: "MAC", dataIndex: "mac" },
              { title: t("network.carrier"), dataIndex: "carrier" },
              {
                title: t("network.permission"),
                render: (_, row) =>
                  row.allowed ? t("network.allowed") : t("network.readOnly"),
              },
            ]}
          />
          <details>
            <summary>
              {t("network.sources", { count: inspection.sources.length })}
            </summary>
            {inspection.sources.map((s) => (
              <p key={s.path}>
                {s.path}
                <br />
                <small>{s.sha256}</small>
              </p>
            ))}
          </details>
          {user.role === "admin" && inspection.writeEnabled && (
            <section
              aria-label={t("network.takeoverRegion")}
              style={{ marginTop: 16 }}
            >
              <h4>{t("network.takeoverTitle")}</h4>
              <p className="muted">{t("network.takeoverNote")}</p>
              <Space wrap>
                <Input
                  aria-label={t("network.takeoverInterface")}
                  placeholder={t("network.takeoverPlaceholder")}
                  maxLength={15}
                  value={takeoverInterface}
                  onChange={(e) => {
                    setTakeoverInterface(e.target.value);
                    setTakeoverPlan(undefined);
                    setReviewed(false);
                  }}
                />
                <Button
                  disabled={busy || !takeoverInterface}
                  onClick={() =>
                    void run(async () => {
                      const plan = (await task("network.takeover.plan", {
                        networkTakeover: { interface: takeoverInterface },
                      })) as TakeoverPlan;
                      if (mounted.current) {
                        setTakeoverPlan(plan);
                        setReviewed(false);
                      }
                    })
                  }
                >
                  {t("network.preview")}
                </Button>
              </Space>
              {takeoverPlan &&
                (takeoverPlan.eligible ? (
                  <>
                    <p>
                      {t("network.sourceSummary", {
                        path: takeoverPlan.sourcePath ?? "—",
                        mode: takeoverPlan.sourceMode ?? "—",
                        revision: takeoverPlan.revision ?? "—",
                      })}
                    </p>
                    <p>
                      {t("network.preserved", {
                        interfaces:
                          takeoverPlan.preservedOtherInterfaces?.join(", ") ||
                          t("common.none"),
                      })}
                    </p>
                    <pre className="result">
                      {JSON.stringify(takeoverPlan.preservedInterface, null, 2)}
                    </pre>
                    <Checkbox
                      checked={reviewed}
                      onChange={(e) => setReviewed(e.target.checked)}
                    >
                      {t("network.consent")}
                    </Checkbox>
                    <Button
                      disabled={
                        busy ||
                        !reviewed ||
                        (!!transaction &&
                          !["COMMITTED", "ROLLED_BACK"].includes(
                            transaction.state,
                          ))
                      }
                      onClick={() =>
                        void run(async () => {
                          const value = (await task(
                            "network.takeover.prepare",
                            {
                              networkTakeover: {
                                interface: takeoverPlan.interface,
                                expectedRevision: takeoverPlan.revision,
                                sourceDigest: takeoverPlan.sourceDigest,
                                planDigest: takeoverPlan.planDigest,
                                confirmSeconds: 120,
                              },
                            },
                          )) as Transaction;
                          if (mounted.current) {
                            setTransaction(value);
                            setTakeoverPlan(undefined);
                            setReviewed(false);
                          }
                        })
                      }
                    >
                      {t("network.applyTakeover")}
                    </Button>
                  </>
                ) : (
                  <Alert
                    type="warning"
                    title={t("network.ineligible")}
                    description={
                      takeoverPlan.diagnostic && (
                        <details>
                          <summary>{t("network.details")}</summary>
                          {takeoverPlan.diagnostic}
                        </details>
                      )
                    }
                  />
                ))}
            </section>
          )}
          {user.role === "admin" && inspection.writeEnabled && (
            <Form
              form={form}
              layout="vertical"
              initialValues={{
                dhcp4: true,
                addresses: "",
                nameservers: "",
                search: "",
                mtu: 1500,
                dhcpMetric: 100,
                optional: true,
                confirmSeconds: 120,
                routes: [],
              }}
              onFinish={(values) => void run(() => prepare(values))}
            >
              <Form.Item
                label={t("network.interfaceName")}
                name="interface"
                rules={[{ required: true }]}
              >
                <Input
                  maxLength={15}
                  placeholder={t("network.interfacePlaceholder")}
                />
              </Form.Item>
              <Form.Item
                label={t("network.dhcp")}
                name="dhcp4"
                valuePropName="checked"
              >
                <Switch />
              </Form.Item>
              <Form.Item label={t("network.addresses")} name="addresses">
                <Input placeholder="192.0.2.2/24" />
              </Form.Item>
              <Form.Item label={t("network.dns")} name="nameservers">
                <Input />
              </Form.Item>
              <Form.Item label={t("network.search")} name="search">
                <Input />
              </Form.Item>
              <Space>
                <Form.Item label="MTU" name="mtu">
                  <InputNumber min={576} max={9000} />
                </Form.Item>
                <Form.Item label={t("network.dhcpMetric")} name="dhcpMetric">
                  <InputNumber min={0} max={65535} />
                </Form.Item>
              </Space>
              <Form.Item
                label={t("network.optional")}
                name="optional"
                valuePropName="checked"
              >
                <Switch />
              </Form.Item>
              <Form.List name="routes">
                {(fields, { add, remove }) => (
                  <>
                    {fields.map((field) => (
                      <Space key={field.key} align="baseline">
                        <Form.Item
                          name={[field.name, "to"]}
                          rules={[{ required: true }]}
                        >
                          <Input
                            aria-label={t("network.routeTo")}
                            placeholder={t("network.routeToPlaceholder")}
                          />
                        </Form.Item>
                        <Form.Item
                          name={[field.name, "via"]}
                          rules={[{ required: true }]}
                        >
                          <Input
                            aria-label={t("network.routeVia")}
                            placeholder={t("network.routeViaPlaceholder")}
                          />
                        </Form.Item>
                        <Form.Item
                          name={[field.name, "metric"]}
                          rules={[{ required: true }]}
                        >
                          <InputNumber
                            aria-label={t("network.routeMetric")}
                            min={0}
                            max={65535}
                          />
                        </Form.Item>
                        <Button onClick={() => remove(field.name)}>
                          {t("network.remove")}
                        </Button>
                      </Space>
                    ))}
                    <Button
                      onClick={() =>
                        add({ to: "default", via: "", metric: 100 })
                      }
                    >
                      {t("network.addRoute")}
                    </Button>
                  </>
                )}
              </Form.List>
              <Form.Item label={t("network.window")} name="confirmSeconds">
                <InputNumber min={30} max={300} />
              </Form.Item>
              <Button
                htmlType="submit"
                loading={busy}
                disabled={
                  !!transaction &&
                  !["COMMITTED", "ROLLED_BACK"].includes(transaction.state)
                }
              >
                {t("network.apply")}
              </Button>
            </Form>
          )}
        </>
      )}
    </section>
  );
}
