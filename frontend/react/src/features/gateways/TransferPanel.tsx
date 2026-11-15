import {
  Alert,
  Button,
  Checkbox,
  Form,
  Input,
  Space,
  Table,
  Tag,
} from "antd";
import { useCallback, useEffect, useRef, useState } from "react";
import {
  api,
  type Gateway,
  type Session,
  type TransferApproval,
  type TransferIssuance,
  type TransferSigner,
} from "../../api/client";
import { errorText, useI18n, useLocalizedForm } from "../../i18n/I18n";
import type { MessageKey } from "../../i18n/messages";

type SignerFields = {
  signerKeyId: string;
  signerPublicKeyPem: string;
};
type RequestFields = {
  newEndpoint: string;
  newGatewayId: string;
  newClientCaPem: string;
  newClientCertificatePem: string;
};
const statusKeys: Record<string, MessageKey> = {
  ACTIVE: "transfer.status.ACTIVE",
  REVOKED: "transfer.status.REVOKED",
  PENDING: "transfer.status.PENDING",
  APPROVED: "transfer.status.APPROVED",
  REJECTED: "transfer.status.REJECTED",
  ISSUED: "transfer.status.ISSUED",
};
function statusColor(status: string) {
  return status === "ACTIVE" || status === "APPROVED"
    ? "green"
    : status === "PENDING"
      ? "blue"
      : status === "ISSUED"
        ? "purple"
        : "default";
}
export function TransferPanel({
  gateway,
  user,
}: {
  gateway: Gateway;
  user: Session;
}) {
  const { t, dateTime } = useI18n();
  const [signers, setSigners] = useState<TransferSigner[]>([]);
  const [approvals, setApprovals] = useState<TransferApproval[]>([]);
  const [issuances, setIssuances] = useState<TransferIssuance[]>([]);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<unknown>(null);
  const [confirmed, setConfirmed] = useState(false);
  const mounted = useRef(true);
  const [signerForm] = Form.useForm<SignerFields>();
  const [requestForm] = Form.useForm<RequestFields>();
  useLocalizedForm(signerForm);
  useLocalizedForm(requestForm);
  const load = useCallback(async () => {
    const [authority, request, issued] = await Promise.all([
      api<{ items: TransferSigner[] }>("/transfer/signers"),
      api<{ items: TransferApproval[] }>("/transfer/approvals"),
      api<{ items: TransferIssuance[] }>("/transfer/issuances"),
    ]);
    if (!mounted.current) return;
    setSigners(authority.items);
    setApprovals(
      request.items.filter(
        (item) =>
          item.gatewayId === gateway.id ||
          item.newGatewayId === gateway.id,
      ),
    );
    setIssuances(
      issued.items.filter(
        (item) =>
          item.gatewayId === gateway.id ||
          item.newGatewayId === gateway.id,
      ),
    );
  }, [gateway.id]);
  useEffect(() => {
    mounted.current = true;
    void load().catch((value) => {
      if (mounted.current) setError(value);
    });
    return () => {
      mounted.current = false;
    };
  }, [load]);
  async function run(work: () => Promise<void>) {
    setBusy(true);
    setError(null);
    try {
      await work();
      await load();
    } catch (value) {
      if (mounted.current) setError(value);
    } finally {
      if (mounted.current) setBusy(false);
    }
  }
  async function registerSigner(fields: SignerFields) {
    await api(
      "/transfer/signers",
      "POST",
      {
        keyId: fields.signerKeyId,
        publicKeyPem: fields.signerPublicKeyPem,
      },
      user.csrf,
    );
    signerForm.resetFields();
  }
  async function requestTransfer(fields: RequestFields) {
    await api(
      `/gateways/${encodeURIComponent(gateway.id)}/transfer-approval`,
      "POST",
      fields,
      user.csrf,
    );
    requestForm.resetFields();
    setConfirmed(false);
  }
  async function action(id: string, name: "approve" | "reject" | "revoke") {
    await api(`/transfer/approvals/${id}/${name}`, "POST", {}, user.csrf);
  }
  return (
    <section className="panel" aria-label={t("transfer.title")}>
      <p className="eyebrow">{t("transfer.region")}</p>
      <h3>{t("transfer.title")}</h3>
      <p className="muted">{t("transfer.description")}</p>
      {error ? (
        <Alert type="error" showIcon description={errorText(error, t)} />
      ) : null}
      <Form<SignerFields> form={signerForm} layout="vertical" onFinish={(values) => void run(() => registerSigner(values))}>
        <Form.Item
          name="signerKeyId"
          label={t("transfer.signerKeyId")}
          rules={[{ required: true, max: 128 }]}
        >
          <Input autoComplete="off" placeholder="center-2026-11" />
        </Form.Item>
        <Form.Item
          name="signerPublicKeyPem"
          label={t("transfer.signerPublicKey")}
          rules={[{ required: true }]}
        >
          <Input.TextArea
            rows={4}
            spellCheck={false}
            placeholder="-----BEGIN PUBLIC KEY-----"
          />
        </Form.Item>
        <Button type="primary" htmlType="submit" loading={busy}>
          {t("transfer.registerSigner")}
        </Button>
      </Form>
      <Table<TransferSigner>
        size="small"
        rowKey="keyId"
        dataSource={signers}
        pagination={false}
        locale={{ emptyText: t("transfer.noSigners") }}
        columns={[
          { title: t("transfer.keyId"), dataIndex: "keyId" },
          {
            title: t("transfer.fingerprint"),
            dataIndex: "publicKeySha256",
            render: (value: string) => <code>{value.slice(0, 16)}…</code>,
          },
          {
            title: t("transfer.status"),
            dataIndex: "status",
            render: (value: string) => (
              <Tag color={statusColor(value)}>
                {t(statusKeys[value] ?? "transfer.status.ACTIVE")}
              </Tag>
            ),
          },
          {
            title: t("transfer.actions"),
            render: (_, signer) =>
              signer.status === "ACTIVE" && (
                <Button
                  danger
                  size="small"
                  disabled={busy}
                  onClick={() => void run(() => api(`/transfer/signers/${encodeURIComponent(signer.keyId)}/revoke`, "POST", {}, user.csrf))}
                >
                  {t("transfer.revokeSigner")}
                </Button>
              ),
          },
        ]}
      />
      <Form<RequestFields> form={requestForm} layout="vertical" onFinish={(values) => void run(() => requestTransfer(values))}>
        <p className="eyebrow">{t("transfer.requestTitle")}</p>
        <Form.Item
          name="newEndpoint"
          label={t("transfer.newEndpoint")}
          rules={[{ required: true, max: 512 }]}
        >
          <Input autoComplete="off" placeholder="https://center2.example:28444" />
        </Form.Item>
        <Form.Item
          name="newGatewayId"
          label={t("transfer.newGatewayId")}
          rules={[{ required: true, max: 128 }]}
        >
          <Input autoComplete="off" />
        </Form.Item>
        <Form.Item
          name="newClientCaPem"
          label={t("transfer.newCa")}
          rules={[{ required: true }]}
        >
          <Input.TextArea rows={4} spellCheck={false} placeholder="-----BEGIN CERTIFICATE-----" />
        </Form.Item>
        <Form.Item
          name="newClientCertificatePem"
          label={t("transfer.newCertificate")}
          rules={[{ required: true }]}
        >
          <Input.TextArea rows={4} spellCheck={false} placeholder="-----BEGIN CERTIFICATE-----" />
        </Form.Item>
        <Checkbox checked={confirmed} onChange={(event) => setConfirmed(event.target.checked)}>
          {t("transfer.confirmRequest", {
            from: String(gateway.bindingGeneration),
            to: String(gateway.bindingGeneration + 1),
          })}
        </Checkbox>
        <div className="actions">
          <Button type="primary" danger htmlType="submit" disabled={!confirmed} loading={busy}>
            {t("transfer.request")}
          </Button>
        </div>
      </Form>
      <Table<TransferApproval>
        size="small"
        rowKey="id"
        dataSource={approvals}
        pagination={false}
        locale={{ emptyText: t("transfer.noApprovals") }}
        columns={[
          {
            title: t("transfer.approval"),
            dataIndex: "id",
            render: (value: string) => <code>{value.slice(0, 8)}…</code>,
          },
          {
            title: t("transfer.destination"),
            render: (_, approval) => (
              <span>
                {approval.newGatewayId}
                <br />
                <span className="muted">{approval.newEndpoint}</span>
              </span>
            ),
          },
          {
            title: t("transfer.generations"),
            render: (_, approval) => `${approval.expectedGeneration} → ${approval.toGeneration}`,
          },
          {
            title: t("transfer.status"),
            dataIndex: "status",
            render: (value: string) => (
              <Tag color={statusColor(value)}>
                {t(statusKeys[value] ?? "transfer.status.ACTIVE")}
              </Tag>
            ),
          },
          {
            title: t("transfer.approvers"),
            render: (_, approval) => (
              <span>
                {approval.requestedByName}
                <br />
                <span className="muted">{approval.approvedByName ?? "—"}</span>
              </span>
            ),
          },
          {
            title: t("transfer.time"),
            dataIndex: "createdAt",
            render: (value: string) => dateTime(value),
          },
          {
            title: t("transfer.actions"),
            render: (_, approval) => (
              <Space direction="vertical" size={4}>
                {approval.status === "PENDING" && approval.requestedBy !== user.id && (
                  <Button size="small" type="primary" disabled={busy} onClick={() => void run(() => action(approval.id, "approve"))}>
                    {t("transfer.approve")}
                  </Button>
                )}
                {["PENDING", "APPROVED"].includes(approval.status) && (
                  <Space size={4}>
                    <Button size="small" danger disabled={busy} onClick={() => void run(() => action(approval.id, "reject"))}>
                      {t("transfer.reject")}
                    </Button>
                    <Button size="small" danger disabled={busy} onClick={() => void run(() => action(approval.id, "revoke"))}>
                      {t("transfer.revoke")}
                    </Button>
                  </Space>
                )}
              </Space>
            ),
          },
        ]}
      />
      <p className="eyebrow">{t("transfer.issuanceTitle")}</p>
      <Table<TransferIssuance>
        size="small"
        rowKey="transferId"
        dataSource={issuances}
        pagination={false}
        locale={{ emptyText: t("transfer.noIssuances") }}
        columns={[
          {
            title: t("transfer.issuance"),
            dataIndex: "transferId",
            render: (value: string) => <code>{value.slice(0, 8)}…</code>,
          },
          {
            title: t("transfer.destination"),
            render: (_, issuance) => (
              <span>
                {issuance.newGatewayId}
                <br />
                <span className="muted">{issuance.newEndpoint}</span>
              </span>
            ),
          },
          {
            title: t("transfer.generations"),
            render: (_, issuance) => `${issuance.expectedGeneration} → ${issuance.toGeneration}`,
          },
          {
            title: t("transfer.issuer"),
            render: (_, issuance) => (
              <span>
                {issuance.issuedByName}
                <br />
                <span className="muted">{issuance.revokedByName ?? "—"}</span>
              </span>
            ),
          },
          {
            title: t("transfer.time"),
            dataIndex: "issuedAt",
            render: (value: string) => dateTime(value),
          },
          {
            title: t("transfer.status"),
            dataIndex: "status",
            render: (value: string) => (
              <Tag color={statusColor(value)}>
                {t(statusKeys[value] ?? "transfer.status.ACTIVE")}
              </Tag>
            ),
          },
          {
            title: t("transfer.actions"),
            render: (_, issuance) =>
              issuance.status === "ACTIVE" && (
                <Button
                  danger
                  size="small"
                  disabled={busy}
                  onClick={() => void run(() => api(`/transfer/issuances/${encodeURIComponent(issuance.transferId)}/revoke`, "POST", {}, user.csrf))}
                >
                  {t("transfer.revokeIssuance")}
                </Button>
              ),
          },
        ]}
      />
      <Alert type="info" showIcon title={t("transfer.revokeNote")} />
      <Alert type="info" showIcon title={t("transfer.issueNote")} />
    </section>
  );
}
