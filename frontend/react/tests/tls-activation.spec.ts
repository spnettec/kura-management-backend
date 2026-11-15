import { test, expect, type Page } from "@playwright/test";
import { readFileSync, mkdirSync } from "node:fs";
import { resolve } from "node:path";

const generation = "11111111-1111-4111-8111-111111111111";
const originalId = "22222222-2222-4222-8222-222222222222";
const revision = "a".repeat(64);
const storage = "yofc.console.tls-activation.v1:tls-test";
const saved = { operationId: originalId, expectedGeneration: generation, pendingRevision: revision };
const evidence = process.env.YOFC_TLS_BROWSER_EVIDENCE;
const labels = {
  "zh-CN": { title: "证书与信任配置生效", apply: "使本批证书生效", observe: "查询原生效操作", confirm: "我已核对两项指纹和网关影响", refresh: "刷新待生效配置", unknown: "原操作结果尚未核定", done: "原操作已记录生效回执", finish: "已阅，准备下一批" },
  "en-US": { title: "Activate certificates and trust", apply: "Activate this certificate set", observe: "Read original activation", confirm: "I reviewed both fingerprints and the gateway impact", refresh: "Refresh pending settings", unknown: "The original outcome is unresolved", done: "The original activation has a recorded applied receipt", finish: "Acknowledge and review the next set" },
};

async function fixture(page: Page, language: keyof typeof labels, mode: "applied" | "lost" | "missing" | "blocked" = "applied", seed = false) {
  const writes: unknown[] = [], queries: string[] = [];
  await page.addInitScript(({ language, seed, saved, storage }) => {
    localStorage.setItem("yofc.console.locale", language);
    if (seed && !localStorage.getItem(storage)) localStorage.setItem(storage, JSON.stringify(saved));
  }, { language, seed, saved, storage });
  await page.route("**/api/v1/session", r => r.fulfill({ json: { id: "tls-test", username: "tls-test", tenantId: "tls-test", role: "admin", csrf: "test-csrf", mustChangePassword: false } }));
  await page.route("**/api/v1/gateways?*", r => r.fulfill({ json: { items: [] } }));
  await page.route("**/api/v1/tasks?*", r => r.fulfill({ json: { items: [] } }));
  await page.route("**/api/v1/events", r => r.fulfill({ contentType: "text/event-stream", body: ": test\n\n" }));
  await page.route("**/api/v1/settings/tls", r => r.fulfill({ json: { certificateFingerprint: "1".repeat(64), pendingFingerprint: "2".repeat(64), activationRequired: true, restartRequired: false } }));
  await page.route("**/api/v1/settings/client-ca", r => r.fulfill({ json: { caFingerprint: "3".repeat(64), pendingFingerprint: "4".repeat(64), registeredGateways: 2, activationRequired: true, restartRequired: false } }));
  await page.route("**/api/v1/settings/task-key", r => r.fulfill({ json: { keyFingerprint: "5".repeat(64), pendingRestoreStaged: false } }));
  let pending = true;
  await page.route("**/api/v1/settings/tls-activation", async r => {
    if (r.request().method() === "GET") return r.fulfill({ json: { generation, healthy: true, certificateFingerprint: "1".repeat(64), caFingerprint: "3".repeat(64), pendingTlsFingerprint: pending ? "2".repeat(64) : null, pendingCaFingerprint: pending ? "4".repeat(64) : null, pendingRevision: revision, activationRequired: pending, registeredGateways: 2, blockingOperationId: mode === "blocked" ? originalId : null } });
    const body = r.request().postDataJSON();
    expect(await page.evaluate(storage => JSON.parse(localStorage.getItem(storage)!), storage)).toEqual(body);
    writes.push(body); pending = false;
    if (mode === "lost") return r.abort("connectionreset");
    return r.fulfill({ json: { operationId: body.operationId, state: "APPLIED", observedActive: true, request: body } });
  });
  await page.route("**/api/v1/settings/tls-activation/*", r => {
    const id = r.request().url().split("/").at(-1)!; queries.push(id);
    if (mode === "missing") return r.fulfill({ status: 404, json: { error: { code: "NOT_FOUND" } } });
    const request = writes[0] || saved;
    return r.fulfill({ json: { operationId: id, state: mode === "applied" ? "APPLIED" : "UNKNOWN", observedActive: mode !== "missing", request } });
  });
  await page.goto("/settings");
  const l = labels[language];
  const panel = page.locator(".ant-card").filter({ has: page.locator(".ant-card-head-title", { hasText: l.title }) });
  await expect(panel.getByRole("button", { name: l.refresh, exact: true })).toBeEnabled();
  const approve = () => panel.getByRole("checkbox").check();
  return { panel, l, writes, queries, approve };
}

for (const language of ["zh-CN", "en-US"] as const) {
  test(`TLS activation requires review and stores the original id before dispatch in ${language}`, async ({ page }) => {
    const f = await fixture(page, language);
    const apply = f.panel.getByRole("button", { name: f.l.apply, exact: true });
    await expect(apply).toBeDisabled(); expect(f.writes).toHaveLength(0);
    await f.approve(); await expect(apply).toBeEnabled();
    await f.panel.getByRole("button", { name: f.l.refresh, exact: true }).click();
    await expect(apply).toBeDisabled();
    await f.approve(); await apply.click();
    await expect(f.panel).toContainText(f.l.done);
    expect(f.writes).toHaveLength(1);
    await page.reload(); await expect(f.panel).toContainText(f.l.done);
    expect(f.writes).toHaveLength(1);
    if (evidence) { mkdirSync(evidence, { recursive: true }); await f.panel.screenshot({ path: resolve(evidence, `activation-${language}.png`) }); }
  });

  test(`Lost TLS response survives reload without another POST in ${language}`, async ({ page }) => {
    const f = await fixture(page, language, "lost");
    await f.approve(); await f.panel.getByRole("button", { name: f.l.apply, exact: true }).click();
    await expect(f.panel).toContainText(f.l.unknown);
    const id = (f.writes[0] as typeof saved).operationId;
    await f.panel.getByRole("button", { name: f.l.observe, exact: true }).click();
    await page.reload(); await expect(f.panel).toContainText(f.l.unknown);
    await expect(f.panel.getByRole("button", { name: f.l.apply, exact: true })).toBeDisabled();
    await expect(f.panel.getByRole("button", { name: f.l.finish, exact: true })).toHaveCount(0);
    expect(f.writes).toHaveLength(1);
    expect(f.queries.every(value => value === id)).toBe(true);
  });
}

test("Another browser restores the server's unresolved original id without dispatch", async ({ page }) => {
  const f = await fixture(page, "zh-CN", "blocked");
  await expect(f.panel).toContainText(originalId);
  await expect(f.panel).toContainText(f.l.unknown);
  await expect(f.panel.getByRole("checkbox")).toBeDisabled();
  expect(f.writes).toHaveLength(0); expect(f.queries).toContain(originalId);
});

test("An original 404 never clears the saved activation or enables a new one", async ({ page }) => {
  const f = await fixture(page, "zh-CN", "missing", true);
  await expect(f.panel).toContainText(originalId);
  await f.panel.getByRole("button", { name: f.l.observe, exact: true }).click();
  await page.reload(); await expect(f.panel).toContainText(originalId);
  await expect(f.panel.getByRole("button", { name: f.l.apply, exact: true })).toBeDisabled();
  expect(f.writes).toHaveLength(0); expect(f.queries.every(id => id === originalId)).toBe(true);
});

test("A failed local durable save prevents activation dispatch", async ({ page }) => {
  const f = await fixture(page, "zh-CN");
  await page.evaluate(() => {
    const original = Storage.prototype.setItem;
    Storage.prototype.setItem = function (key, value) {
      if (key.startsWith("yofc.console.tls-activation")) throw new Error("Test storage failure");
      return original.call(this, key, value);
    };
  });
  await f.approve(); await f.panel.getByRole("button", { name: f.l.apply, exact: true }).click();
  await expect(f.panel.getByRole("alert").last()).toBeVisible();
  expect(f.writes).toHaveLength(0);
});

test("Live console certificate upload and activation through the settings UI", async ({ page }) => {
  test.skip(!process.env.YOFC_TLS_LIVE_URL, "New isolated live center is explicitly selected");
  const url = process.env.YOFC_TLS_LIVE_URL!;
  const password = readFileSync(process.env.YOFC_TLS_LIVE_PASSWORD_FILE!, "utf8").trim();
  await page.goto(`${url}/settings`);
  await page.getByLabel(/账号|Username/).fill("tls-browser-admin");
  await page.getByLabel(/密码|Password/, { exact: true }).fill(password);
  await page.getByRole("button", { name: /^(登\s*录|Sign in)$/ }).click();
  const fixtureDir = resolve(import.meta.dirname, "../../../crates/console/tests/fixtures/tls-live");
  await page.getByLabel("选择文件: 服务器证书 PEM", { exact: true }).setInputFiles(resolve(fixtureDir, "server-two.pem"));
  await page.getByLabel("选择文件: 服务器私钥 PEM", { exact: true }).setInputFiles(resolve(fixtureDir, "server-two.key"));
  const tls = page.locator(".ant-card").filter({ has: page.locator(".ant-card-head-title", { hasText: /^服务器证书$/ }) });
  await tls.getByRole("button", { name: "核对 PEM 指纹", exact: true }).click();
  await tls.getByRole("checkbox").check();
  await tls.getByRole("button", { name: "上传服务器证书", exact: true }).click();
  await expect(tls).toContainText("已暂存");
  const ca = page.locator(".ant-card").filter({ has: page.locator(".ant-card-head-title", { hasText: "客户端 CA（网关信任锚）" }) });
  await page.getByLabel("选择文件: 客户端 CA 证书 PEM", { exact: true }).setInputFiles(resolve(fixtureDir, "ca-two.pem"));
  await ca.getByRole("button", { name: "核对 PEM 指纹", exact: true }).click();
  await ca.getByRole("checkbox").check();
  await ca.getByRole("button", { name: "上传客户端 CA", exact: true }).click();
  await expect(ca).toContainText("已暂存");
  const activation = page.locator(".ant-card").filter({ has: page.locator(".ant-card-head-title", { hasText: labels["zh-CN"].title }) });
  await activation.getByRole("checkbox").check();
  await activation.getByRole("button", { name: labels["zh-CN"].apply, exact: true }).click();
  await activation.getByRole("button", { name: labels["zh-CN"].observe, exact: true }).click();
  await expect(activation).toContainText(labels["zh-CN"].done);
  await page.reload(); await expect(activation).toContainText(labels["zh-CN"].done);
  if (evidence) await page.screenshot({ path: resolve(evidence, "activation-live-center.png"), fullPage: true });
});
