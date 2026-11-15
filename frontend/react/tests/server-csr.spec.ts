import { test, expect, type Page } from "@playwright/test";
import { readFileSync, mkdirSync } from "node:fs";
import { resolve } from "node:path";
import { createHash } from "node:crypto";
const id = "f4cf9e82-4508-45f0-80d8-31bb730ed341";
const installation = "08271197-47a6-4b3e-a123-853925071244";
const publicCsr = "-----BEGIN CERTIFICATE REQUEST-----\\nPUBLIC-UI-CONTRACT-ONLY\\n-----END CERTIFICATE REQUEST-----\\n";
const leaf = readFileSync(resolve(import.meta.dirname, "../../../crates/console/tests/fixtures/tls-live/server-one.pem"), "utf8");
const ca = readFileSync(resolve(import.meta.dirname, "../../../crates/console/tests/fixtures/tls-live/ca-one.pem"), "utf8");
const labels = {
  "en-US": { generate: "Generate server CSR", observe: "Observe original CSR request", download: "Download public CSR", upload: "Upload signed server certificate", unknown: "Outcome unknown.", signed: "Signed certificate validated", staged: "Certificate staged", existing: "Import existing server certificate and key", submit: "Upload server certificate", confirm: "I verified that this server key belongs to this certificate", ready: "CSR ready", title: "Server certificate" },
  "zh-CN": { generate: "生成服务器 CSR", observe: "查询原 CSR 请求", download: "下载公开 CSR", upload: "上传签回的服务器证书", unknown: "结果未确定", signed: "签回证书已验证", staged: "证书已暂存", existing: "导入已有服务器证书和私钥", submit: "上传服务器证书", confirm: "我已核对服务器私钥与该证书匹配", ready: "CSR 已生成", title: "服务器证书" },
};
type Mode = "normal" | "lost-create" | "lost-import" | "missing" | "server-unknown" | "rejected-certificate" | "installed" | "lost-rejected-receipt" | "lost-applied-receipt" | "mismatched-import-receipt" | "mismatched-import-digest";
async function fixture(page: Page, language: keyof typeof labels, setup = false, mode: Mode = "normal") {
  const writes: { method: string; url: string; body: any }[] = [], queries: string[] = [];
  const origin = "https://localhost:28453";
  const storage = `yofc.console.server-csr.v1:${setup ? "setup" : "settings"}:${setup ? installation : `${origin}:csr-admin`}`;
  let record: any = mode === "server-unknown" ? { requestId: id, state: "UNKNOWN" } : mode === "installed" ? { requestId:id, state:"INSTALLED", origin, installationOperationId:installation } : undefined;
  const configuration: any[] = [];
  const importReceipts = new Map<string, any>();
  await page.addInitScript(language => localStorage.setItem("yofc.console.locale", language), language);
  await page.route("**/api/setup/status", r => r.fulfill({ json: { configured: false, authenticated: true, csrf: "csr-setup-csrf", installationId: installation, listenAddress: "127.0.0.1:28453", blockingOperationId: null } }));
  await page.route("**/api/v1/session", r => r.fulfill({ json: { id: "csr-admin", username: "csr-admin", tenantId: "review", role: "admin", csrf: "csr-admin-csrf", mustChangePassword: false } }));
  await page.route("**/api/v1/gateways?*", r => r.fulfill({ json: { items: [] } }));
  await page.route("**/api/v1/tasks?*", r => r.fulfill({ json: { items: [] } }));
  await page.route("**/api/v1/events", r => r.fulfill({ contentType: "text/event-stream", body: ": csr-contract\\n\\n" }));
  await page.route("**/api/v1/settings/tls", r => r.fulfill({ json: { origin, certificateFingerprint: "1".repeat(64), pendingFingerprint: record?.state === "STAGED" ? "a".repeat(64) : null } }));
  await page.route("**/api/v1/settings/client-ca", r => r.fulfill({ json: { caFingerprint: "2".repeat(64), registeredGateways: 0 } }));
  await page.route("**/api/v1/settings/task-key", r => r.fulfill({ json: { keyFingerprint: "3".repeat(64) } }));
  await page.route("**/api/v1/settings/tls-activation", r => r.fulfill({ json: { generation: id, healthy: true, certificateFingerprint: "1".repeat(64), caFingerprint: "2".repeat(64), pendingRevision: "a".repeat(64), activationRequired: false, registeredGateways: 0, blockingOperationId: null } }));
  await page.route("**/api/setup/operations", r => { configuration.push(r.request().postDataJSON()); return r.fulfill({ json: { operationId: configuration[0].operationId, state: "UNKNOWN", origin, observedConfiguration: false } }); });
  await page.route("**/api/setup/operations/*", r => r.fulfill({ json: { operationId: configuration[0]?.operationId, state: "UNKNOWN", origin } }));
  const base = setup ? "/api/setup/server-csr" : "/api/v1/settings/server-csr";
  await page.route(`**${base}**`, async route => {
    const req = route.request(), path = new URL(req.url()).pathname.slice(base.length);
    if (req.method() === "GET") {
      if (!path) return route.fulfill({ json: { current: record ?? null } });
      queries.push(path.slice(1));
      if (mode === "missing") return route.fulfill({ status: 404, json: { error: { code: "NOT_FOUND" } } });
      if (path.includes("/imports/")) {
        const receipt = importReceipts.get(path.split("/").at(-1)!);
        return receipt ? route.fulfill({ json: { ...record, importOperation: receipt } })
          : route.fulfill({ status: 404, json: { error: { code: "NOT_FOUND" } } });
      }
      return route.fulfill({ json: record });
    }
    const body = req.postData() ? req.postDataJSON() : undefined;
    const stored = await page.evaluate(storage => JSON.parse(localStorage.getItem(storage)!), storage);
    expect(stored.requestId).toBe(path ? path.split("/")[1] : body.requestId);
    expect(stored.phase).toBe(path.endsWith("certificate") ? "IMPORT" : path.endsWith("stage") ? "STAGE" : path.endsWith("retire") ? "RETIRE" : "CREATE");
    writes.push({ method: req.method(), url: path, body });
    expect(JSON.stringify(stored)).not.toContain("PRIVATE KEY");
    if (!path) {
      record = { requestId: body.requestId, origin: body.origin, state: "READY", csrPem: publicCsr, csrSha256: "b".repeat(64) };
      if (mode === "lost-create" || mode === "missing") { if (mode === "lost-create") record = { requestId: body.requestId, state: "UNKNOWN" }; return route.abort("connectionreset"); }
    } else if (path.endsWith("certificate")) {
      expect(Object.keys(body).sort()).toEqual(body.operationId ? ["operationId", "serverPem"] : ["serverPem"]);
      const receipt = body.operationId ? { operationId: body.operationId, requestId: record.requestId,
        certificateSha256: createHash("sha256").update(body.serverPem).digest("hex"), state: "APPLIED" } : undefined;
      if (receipt) {
        expect(stored.importOperationId).toBe(body.operationId);
        expect(stored.certificateSha256).toBe(receipt.certificateSha256);
      }
      if (mode === "lost-import") return route.abort("connectionreset");
      if ((mode === "rejected-certificate" || mode === "lost-rejected-receipt") && writes.length === 2) {
        if (receipt) importReceipts.set(body.operationId, { ...receipt, state: "REJECTED", code: "INVALID_ARGUMENT" });
        if (mode === "lost-rejected-receipt") return route.abort("connectionreset");
        return route.fulfill({ json: { ...record, certificateRejected: true, importOperation: importReceipts.get(body.operationId) } });
      }
      record = { ...record, state: "SIGNED", certificatePem: body.serverPem, certificateFingerprint: "a".repeat(64) };
      if (receipt) {
        importReceipts.set(body.operationId, mode === "mismatched-import-receipt" ? { ...receipt, operationId: id }
          : mode === "mismatched-import-digest" ? { ...receipt, certificateSha256: "0".repeat(64) } : receipt);
        record.importOperation = importReceipts.get(body.operationId);
      }
      if (["lost-applied-receipt", "mismatched-import-receipt", "mismatched-import-digest"].includes(mode)) return route.abort("connectionreset");
    } else if (path.endsWith("stage")) record = { ...record, state: "STAGED" };
    else if (path.endsWith("retire")) record = { ...record, state: "RETIRED" };
    return route.fulfill({ json: record });
  });
  await page.goto(setup ? "/setup" : "/settings");
  const l = labels[language];
  const panel = setup ? page.locator("main") : page.locator(".ant-card").filter({ has: page.locator(".ant-card-head-title", { hasText: new RegExp(`^${l.title}$`) }) });
  return { panel, l, writes, queries, storage, configuration };
}
for (const language of ["zh-CN", "en-US"] as const) {
  test(`public CSR download, signed upload and explicit staging preserve private keys in ${language}`, async ({ page }) => {
    const f = await fixture(page, language);
    await expect(f.panel.getByRole("button", { name: f.l.generate, exact: true })).toBeEnabled();
    await f.panel.getByRole("button", { name: f.l.generate, exact: true }).click();
    await expect(f.panel).toContainText(f.l.ready);
    const download = page.waitForEvent("download");
    await f.panel.getByRole("button", { name: f.l.download, exact: true }).click();
    const file = await download; expect(file.suggestedFilename()).toMatch(/\.csr$/);
    expect(readFileSync((await file.path())!, "utf8")).toBe(publicCsr);
    await f.panel.getByLabel(f.l.upload, { exact: true }).setInputFiles({ name: "signed.pem", mimeType: "application/x-pem-file", buffer: Buffer.from(leaf) });
    await expect(f.panel).toContainText(f.l.signed);
    await expect(f.panel.getByRole("button", { name: f.l.submit, exact: true })).toBeDisabled();
    await f.panel.getByRole("checkbox").check();
    await f.panel.getByRole("button", { name: f.l.submit, exact: true }).click();
    await expect(f.panel).toContainText(f.l.staged);
    expect(f.writes.map(v => v.method)).toEqual(["POST", "PUT", "POST"]);
    expect(f.configuration).toEqual([]);
    await page.reload(); await expect(f.panel).toContainText(f.l.staged); expect(f.writes).toHaveLength(3);
    expect(await page.locator("input,textarea").evaluateAll(nodes => nodes.map(n => (n as HTMLInputElement).value).join(" "))).not.toContain("PRIVATE KEY");
    const evidence = process.env.YOFC_CSR_BROWSER_EVIDENCE;
    if (evidence) { mkdirSync(evidence, { recursive: true }); await f.panel.screenshot({ path: resolve(evidence, `csr-settings-${language}.png`) }); }
  });
  for (const mode of ["lost-create", "missing"] as const) test(`CSR ${mode} retains the original ID after reload in ${language}`, async ({ page }) => {
    const f = await fixture(page, language, false, mode);
    await f.panel.getByRole("button", { name: f.l.generate, exact: true }).click();
    await expect(f.panel).toContainText(f.l.unknown);
    const original = f.writes[0].body.requestId;
    await f.panel.getByRole("button", { name: f.l.observe, exact: true }).click();
    await page.reload(); await expect(f.panel).toContainText(f.l.unknown);
    await expect(f.panel.getByRole("button", { name: f.l.generate, exact: true })).toHaveCount(0);
    await expect(f.panel.getByRole("radio", { name: f.l.existing, exact: true })).toBeDisabled();
    expect(f.writes).toHaveLength(1); expect(f.queries.every(v => v === original)).toBe(true);
  });
}
test("lost import with a still-READY server is UNKNOWN and never reuploads or retires", async ({ page }) => {
  const f = await fixture(page, "en-US", false, "lost-import");
  await f.panel.getByRole("button", { name: f.l.generate, exact: true }).click();
  await f.panel.getByLabel(f.l.upload, { exact: true }).setInputFiles({ name: "signed.pem", mimeType: "application/x-pem-file", buffer: Buffer.from(leaf) });
  await expect(f.panel).toContainText(f.l.unknown); await page.reload(); await expect(f.panel).toContainText(f.l.unknown);
  await expect(f.panel.locator('input[type="file"]:enabled')).toHaveCount(0);
  await expect(f.panel.getByRole("button", { name: "Retire confirmed CSR", exact: true })).toHaveCount(0);
  expect(f.writes).toHaveLength(2);
});
test("a definite validation rejection allows a corrected certificate on the same CSR", async ({ page }) => {
  const f = await fixture(page, "en-US", false, "rejected-certificate");
  await f.panel.getByRole("button", { name: f.l.generate, exact: true }).click();
  const input = f.panel.getByLabel(f.l.upload, { exact: true });
  await input.setInputFiles({ name: "wrong.pem", mimeType: "application/x-pem-file", buffer: Buffer.from("invalid certificate") });
  await expect(input).toBeEnabled();
  await input.setInputFiles({ name: "signed.pem", mimeType: "application/x-pem-file", buffer: Buffer.from(leaf) });
  await expect(f.panel).toContainText(f.l.signed);
  expect(f.writes).toHaveLength(3); expect(new Set(f.writes.slice(1).map(v => v.url.split("/")[1])).size).toBe(1);
});
for (const language of ["zh-CN", "en-US"] as const) for (const setup of [false, true]) {
  test(`original rejected upload receipt restores correction without resend ${language} setup=${setup}`, async ({ page }) => {
    const f = await fixture(page, language, setup, "lost-rejected-receipt");
    await f.panel.getByRole("button", { name: f.l.generate, exact: true }).click();
    await expect(f.panel).toContainText(f.l.ready);
    await f.panel.getByLabel(f.l.upload, { exact: true }).setInputFiles({ name: "wrong.pem", mimeType: "application/x-pem-file", buffer: Buffer.from("invalid certificate") });
    await expect.poll(() => f.writes.length).toBe(2);
    await expect(f.panel).toContainText(f.l.unknown);
    expect(f.writes[1].body.operationId).toMatch(/^[0-9a-f-]{36}$/);
    const original = f.writes[1].body.operationId;
    await page.reload();
    await expect(f.panel.getByLabel(f.l.upload, { exact: true })).toBeEnabled();
    expect(f.queries.some(path => path.endsWith(`/imports/${original}`))).toBe(true);
    expect(f.writes).toHaveLength(2);
    await f.panel.getByLabel(f.l.upload, { exact: true }).setInputFiles({ name: "signed.pem", mimeType: "application/x-pem-file", buffer: Buffer.from(leaf) });
    await expect(f.panel).toContainText(f.l.signed);
    expect(f.writes).toHaveLength(3);
    expect(f.writes[2].body.operationId).not.toBe(original);
    expect(f.writes[2].url).toBe(f.writes[1].url);
  });
  test(`original applied upload receipt restores success without resend ${language} setup=${setup}`, async ({ page }) => {
    const f = await fixture(page, language, setup, "lost-applied-receipt");
    await f.panel.getByRole("button", { name: f.l.generate, exact: true }).click();
    await expect(f.panel).toContainText(f.l.ready);
    await f.panel.getByLabel(f.l.upload, { exact: true }).setInputFiles({ name: "signed.pem", mimeType: "application/x-pem-file", buffer: Buffer.from(leaf) });
    await expect.poll(() => f.writes.length).toBe(2);
    await expect(f.panel).toContainText(f.l.unknown);
    await page.reload(); await expect(f.panel).toContainText(f.l.signed);
    expect(f.writes).toHaveLength(2);
    expect(f.queries.some(path => path.endsWith(`/imports/${f.writes[1].body.operationId}`))).toBe(true);
  });
}
for (const mode of ["mismatched-import-receipt", "mismatched-import-digest"] as const) test(`${mode} cannot unlock an upload`, async ({ page }) => {
  const f = await fixture(page, "en-US", false, mode);
  await f.panel.getByRole("button", { name: f.l.generate, exact: true }).click();
  await expect(f.panel).toContainText(f.l.ready);
  await f.panel.getByLabel(f.l.upload, { exact: true }).setInputFiles({ name: "signed.pem", mimeType: "application/x-pem-file", buffer: Buffer.from(leaf) });
  await expect.poll(() => f.writes.length).toBe(2);
  await page.reload(); await expect(f.panel).toContainText(f.l.unknown);
  await expect(f.panel.getByRole("button", { name: f.l.submit, exact: true })).toHaveCount(0);
  expect(f.writes).toHaveLength(2);
});
test("a legacy pending import without its own operation identity stays UNKNOWN even if a certificate is now signed", async ({ page }) => {
  const f = await fixture(page, "en-US");
  await f.panel.getByRole("button", { name: f.l.generate, exact: true }).click();
  await expect(f.panel).toContainText(f.l.ready);
  await f.panel.getByLabel(f.l.upload, { exact: true }).setInputFiles({ name: "signed.pem", mimeType: "application/x-pem-file", buffer: Buffer.from(leaf) });
  await expect(f.panel).toContainText(f.l.signed);
  await page.evaluate(storage => { const value = JSON.parse(localStorage.getItem(storage)!); value.phase = "IMPORT";
    delete value.importOperationId; delete value.certificateSha256; localStorage.setItem(storage, JSON.stringify(value)); }, f.storage);
  await page.reload(); await expect(f.panel).toContainText(f.l.unknown);
  await expect(f.panel.getByRole("button", { name: f.l.submit, exact: true })).toHaveCount(0);
  await expect(f.panel.getByRole("button", { name: "Retire confirmed CSR", exact: true })).toHaveCount(0);
  expect(f.writes).toHaveLength(2);
});
test("upload identity persistence failure prevents the certificate PUT", async ({ page }) => {
  const f = await fixture(page, "en-US");
  await f.panel.getByRole("button", { name: f.l.generate, exact: true }).click();
  await expect(f.panel).toContainText(f.l.ready);
  await page.evaluate(() => { const set = Storage.prototype.setItem; Storage.prototype.setItem = function(key, value) {
    if (key.startsWith("yofc.console.server-csr") && JSON.parse(value).phase === "IMPORT") throw new Error("owned import persistence failure"); return set.call(this, key, value); }; });
  await f.panel.getByLabel(f.l.upload, { exact: true }).setInputFiles({ name: "signed.pem", mimeType: "application/x-pem-file", buffer: Buffer.from(leaf) });
  await expect(f.panel.getByRole("alert").last()).toBeVisible();
  expect(f.writes).toHaveLength(1);
});
test("the serialized upload envelope is bounded before saving an intent or sending a PUT", async ({ page }) => {
  const f = await fixture(page, "en-US");
  await f.panel.getByRole("button", { name: f.l.generate, exact: true }).click();
  await expect(f.panel).toContainText(f.l.ready);
  await f.panel.getByLabel(f.l.upload, { exact: true }).setInputFiles({ name: "oversized-envelope.pem", mimeType: "application/x-pem-file", buffer: Buffer.from("\n".repeat(40000)) });
  await expect(f.panel.getByRole("alert").last()).toBeVisible();
  expect(f.writes).toHaveLength(1);
  expect(await page.evaluate(storage => JSON.parse(localStorage.getItem(storage)!).phase, f.storage)).toBe("IDLE");
});
test("initial setup selects the signed local identity without uploading its private key", async ({ page }) => {
  const f = await fixture(page, "en-US", true);
  await f.panel.getByRole("button", { name: f.l.generate, exact: true }).click();
  await f.panel.getByLabel(f.l.upload, { exact: true }).setInputFiles({ name: "signed.pem", mimeType: "application/x-pem-file", buffer: Buffer.from(leaf) });
  await expect(f.panel).toContainText(f.l.signed);
  await page.getByLabel("Gateway public CA certificate file", { exact: true }).setInputFiles({ name: "ca.pem", mimeType: "application/x-pem-file", buffer: Buffer.from(ca) });
  await page.getByLabel("Organization identifier", { exact: true }).fill("csr-review");
  await page.getByLabel("Administrator account", { exact: true }).fill("csr-admin");
  await page.getByLabel("Administrator password (at least 12 bytes)", { exact: true }).fill("A-strong-CSR-password");
  await page.getByRole("button", { name: "Review this configuration and certificate fingerprints", exact: true }).click();
  await page.getByRole("checkbox", { name: "I reviewed the console address, certificates and organization account, and approve initializing the dedicated empty database.", exact: true }).check();
  await page.getByRole("button", { name: "Initialize the management console", exact: true }).click();
  await expect.poll(() => f.configuration.length).toBe(1);
  expect(f.configuration[0].serverCsrId).toBe(f.writes[0].body.requestId);
  expect(f.configuration[0].serverKey).toBe("");expect(JSON.stringify(f.configuration[0])).not.toContain("PRIVATE KEY");
});
test("another setup browser restores a server UNKNOWN without creating a new request", async ({ page }) => {
  const f = await fixture(page, "en-US", true, "server-unknown");
  await expect(f.panel).toContainText(id);await expect(f.panel).toContainText(f.l.unknown);
  await expect(f.panel.getByRole("radio", { name: f.l.existing, exact: true })).toBeDisabled();expect(f.writes).toEqual([]);
});
test("storage failure prevents CSR creation dispatch", async ({ page }) => {
  const f = await fixture(page, "en-US");
  await expect(f.panel.getByRole("button", { name: f.l.generate, exact: true })).toBeEnabled();
  await page.evaluate(() => { const set = Storage.prototype.setItem; Storage.prototype.setItem = function(key, value) { if (key.startsWith("yofc.console.server-csr")) throw new Error("owned storage failure"); return set.call(this, key, value); }; });
  await f.panel.getByRole("button", { name: f.l.generate, exact: true }).click();await expect(f.panel.getByRole("alert").last()).toBeVisible();expect(f.writes).toEqual([]);
});

test("an installed original is terminal and never offered for staging or retirement", async ({ page }) => {
  const f = await fixture(page, "en-US", false, "installed");
  await expect(f.panel).toContainText("This certificate was used for initial installation");
  await expect(f.panel.getByRole("button", { name:"Retire confirmed CSR", exact:true })).toHaveCount(0);
  await expect(f.panel.getByRole("button", { name:f.l.submit, exact:true })).toHaveCount(0);
  expect(f.writes).toEqual([]);
});

test("a noncanonical center origin cannot dispatch key generation", async ({ page }) => {
  const f=await fixture(page,"en-US",true);
  await page.getByLabel("Console HTTPS address",{exact:true}).fill("https://localhost:28453/path");
  await expect(f.panel.getByRole("button",{name:f.l.generate,exact:true})).toBeDisabled();
  expect(f.writes).toEqual([]);
});
