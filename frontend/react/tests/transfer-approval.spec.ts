import { expect, test, type Page } from "@playwright/test";
import type { MessageKey } from "../src/i18n/messages";

const signerPem = "-----BEGIN PUBLIC KEY-----\nTEST\n-----END PUBLIC KEY-----";
const caPem = "-----BEGIN CERTIFICATE-----\nCA\n-----END CERTIFICATE-----";
const certificatePem = "-----BEGIN CERTIFICATE-----\nCERT\n-----END CERTIFICATE-----";

async function open(page: Page) {
  const signerWrites: unknown[] = [];
  const requestWrites: unknown[] = [];
  const approvalWrites: unknown[] = [];
  const issuanceWrites: string[] = [];
  let issuances = [{
    transferId: "4c1dd0aa-1111-4111-8111-111111111111",
    approvalId: "2b6aa731-e56f-4d92-9690-827f6b28ec53",
    gatewayId: "transfer-gateway",
    expectedGeneration: 3,
    toGeneration: 4,
    newGatewayId: "center2-gateway",
    newEndpoint: "https://center2.example:28444",
    signerKeyId: "center-signer",
    issuedBy: "11111111-1111-4111-8111-111111111111",
    issuedByName: "admin",
    issuedAt: "2026-11-02T01:00:00Z",
    status: "ACTIVE",
    revokedAt: null,
    revokedByName: null,
  }];
  let signers: { keyId: string; publicKeySha256: string; status: "ACTIVE" | "REVOKED"; createdAt: string; revokedAt: null }[] = [];
  let approvals = [{
    id: "2b6aa731-e56f-4d92-9690-827f6b28ec53",
    gatewayId: "transfer-gateway",
    expectedGeneration: 3,
    toGeneration: 4,
    newGatewayId: "center2-gateway",
    newEndpoint: "https://center2.example:28444",
    newClientCaSha256: "a".repeat(64),
    newClientCertificateSha256: "c".repeat(64),
    requestedBy: "22222222-2222-4222-8222-222222222222",
    requestedByName: "requester",
    approvedBy: null,
    approvedByName: null,
    status: "PENDING",
    createdAt: "2026-11-02T00:00:00Z",
    approvedAt: null,
    closedAt: null,
  }];
  await page.route("**/api/v1/session", route => route.fulfill({ json: {
    id: "11111111-1111-4111-8111-111111111111", username: "admin",
    tenantId: "fixture", role: "admin", csrf: "fixture",
  }}));
  await page.route("**/api/v1/gateways?*", route => route.fulfill({ json: { items: [{
    id: "transfer-gateway", name: "Transfer fixture", online: true, enabled: true,
    kuraAvailable: true, capabilities: [], revision: 7, bindingGeneration: 3,
  }]} }));
  await page.route("**/api/v1/events", () => {});
  await page.route(/\/api\/v1\/tasks(?:\?.*)?$/, route => route.fulfill({ json: { items: [] } }));
  await page.route("**/api/v1/transfer/signers", async route => {
    const request = route.request();
    if (request.method() === "GET")
      return route.fulfill({ json: { items: signers } });
    const body = request.postDataJSON();
    signerWrites.push(body);
    expect(request.headers()["x-csrf-token"]).toBe("fixture");
    signers = [{ keyId: body.keyId, publicKeySha256: "a".repeat(64), status: "ACTIVE", createdAt: "2026-11-02T00:00:00Z", revokedAt: null }];
    return route.fulfill({ status: 201, json: { keyId: body.keyId, publicKeySha256: "a".repeat(64), status: "ACTIVE" } });
  });
  await page.route("**/api/v1/gateways/transfer-gateway/transfer-approval", async route => {
    const body = route.request().postDataJSON();
    requestWrites.push(body);
    expect(route.request().headers()["x-csrf-token"]).toBe("fixture");
    approvals = [{ ...approvals[0], id: "3b6aa731-e56f-4d92-9690-827f6b28ec53", status: "PENDING", requestedByName: "admin", requestedBy: "11111111-1111-4111-8111-111111111111" }];
    return route.fulfill({ status: 201, json: { id: approvals[0].id, status: "PENDING" } });
  });
  await page.route("**/api/v1/transfer/approvals", async route => {
    const request = route.request();
    if (request.method() === "GET")
      return route.fulfill({ json: { items: approvals } });
    const body = request.postDataJSON();
    requestWrites.push(body);
    expect(request.headers()["x-csrf-token"]).toBe("fixture");
    approvals = [{ ...approvals[0], id: "3b6aa731-e56f-4d92-9690-827f6b28ec53", status: "PENDING", requestedByName: "admin", requestedBy: "11111111-1111-4111-8111-111111111111" }];
    return route.fulfill({ status: 201, json: { id: approvals[0].id, status: "PENDING" } });
  });
  await page.route("**/api/v1/transfer/approvals/*/approve", async route => {
    approvalWrites.push(route.request().postDataJSON());
    expect(route.request().headers()["x-csrf-token"]).toBe("fixture");
    approvals = [{ ...approvals[0], status: "APPROVED", approvedByName: "admin", approvedBy: "11111111-1111-4111-8111-111111111111" }];
    return route.fulfill({ json: { id: approvals[0].id, gatewayId: approvals[0].gatewayId, status: "APPROVED" } });
  });
  await page.route("**/api/v1/transfer/issuances", async route => {
    if (route.request().method() === "GET")
      return route.fulfill({ json: { items: issuances } });
    return route.fulfill({ status: 405, json: { error: { code: "UNSUPPORTED" } } });
  });
  await page.route("**/api/v1/transfer/issuances/*/revoke", async route => {
    issuanceWrites.push(new URL(route.request().url()).pathname.split("/")[5]);
    expect(route.request().headers()["x-csrf-token"]).toBe("fixture");
    issuances = [{ ...issuances[0], status: "REVOKED", revokedAt: "2026-11-02T01:05:00Z", revokedByName: "admin" }];
    return route.fulfill({ json: { transferId: issuances[0].transferId, signerKeyId: issuances[0].signerKeyId, status: "REVOKED" } });
  });
  await page.goto("/gateways");
  await page.getByRole("button", { name: "Transfer fixture transfer-gateway", exact: true }).click();
  const panel = page.getByRole("region", { name: /跨中心转移审批|Cross-center transfer approval/ });
  return { panel, signerWrites, requestWrites, approvalWrites, issuanceWrites, signerPem, caPem, certificatePem };
}

test("transfer signer and approval UI require explicit bilingual authority", async ({ page }) => {
  const fixture = await open(page);
  const panel = fixture.panel;
  await expect(panel.getByText("批准只授权签发，不会直接转移网关。")).toBeVisible();
  await panel.getByLabel("签名方 ID").fill("center-signer");
  await panel.getByLabel("原中心 Ed25519 公钥 PEM").fill(fixture.signerPem);
  await panel.getByRole("button", { name: "登记签名方" }).click();
  await expect(panel.getByText("center-signer")).toBeVisible();
  await expect(panel.getByText("有效").first()).toBeVisible();

  await panel.getByLabel("新中心地址").fill("https://center2.example:28444");
  await panel.getByLabel("新中心网关 ID").fill("center2-gateway");
  await panel.getByLabel("新中心信任锚 PEM").fill(fixture.caPem);
  await panel.getByLabel("新网关客户端证书 PEM").fill(fixture.certificatePem);
  await expect(panel.getByRole("button", { name: "记录请求" })).toBeDisabled();
  await expect(panel.getByRole("button", { name: /批\s*准/ })).toBeVisible();
  await panel.getByRole("button", { name: /批\s*准/ }).click();
  await expect(panel.getByText("已批准")).toBeVisible();
  expect(fixture.approvalWrites).toHaveLength(1);

  await panel.getByRole("checkbox", { name: /generation 3 → 4/ }).check();
  await panel.getByRole("button", { name: "记录请求" }).click();
  await expect(panel.getByText("待批准")).toBeVisible();
  expect(fixture.signerWrites[0]).toMatchObject({ keyId: "center-signer", publicKeyPem: signerPem });
  expect(fixture.requestWrites[0]).toMatchObject({
    newEndpoint: "https://center2.example:28444",
    newGatewayId: "center2-gateway",
    newClientCaPem: caPem,
    newClientCertificatePem: certificatePem,
  });

  await page.getByRole("dialog", { name: "Transfer fixture", exact: true }).getByRole("combobox", { name: "语言 / Language" }).selectOption("en-US");
  await expect(panel.getByText("Approval only authorizes issuance; it does not transfer the gateway directly.")).toBeVisible();
  await expect(panel.getByText("Pending").first()).toBeVisible();
});

test("issued transfer authorizations list and revoke bilingually", async ({ page }) => {
  const fixture = await open(page);
  const panel = fixture.panel;
  await expect(panel.getByText("已签发转移授权")).toBeVisible();
  await expect(panel.getByText("撤销只阻止尚未消费的一次性授权；已完成的转移不受影响，审批不会重新打开。")).toBeVisible();
  await expect(panel.getByRole("button", { name: "撤销授权" })).toBeVisible();
  await panel.getByRole("button", { name: "撤销授权" }).click();
  await expect(panel.getByText("已撤销").first()).toBeVisible();
  await expect(panel.getByRole("button", { name: "撤销授权" })).toHaveCount(0);
  expect(fixture.issuanceWrites).toEqual(["4c1dd0aa-1111-4111-8111-111111111111"]);

  await page.getByRole("dialog", { name: "Transfer fixture", exact: true }).getByRole("combobox", { name: "语言 / Language" }).selectOption("en-US");
  await expect(panel.getByText("Issued transfer authorizations")).toBeVisible();
  await expect(panel.getByText("Revocation only blocks one-time authorizations not yet consumed; completed transfers are unaffected and the approval is not reopened.")).toBeVisible();
  await expect(panel.getByText("Revoked").first()).toBeVisible();
});
