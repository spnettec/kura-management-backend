import { expect, test, type Page } from "@playwright/test";
import { readFileSync } from "node:fs";
import { createHash } from "node:crypto";
const fixtures = new URL("../../../crates/console/tests/fixtures/", import.meta.url);
const ca = readFileSync(new URL("review-ca-two.pem", fixtures));
const leaf = readFileSync(new URL("review-not-ca.pem", fixtures));
const hash = (pem: Buffer) => createHash("sha256").update(Buffer.from(pem.toString().replace(/-----[^-]+-----|\s/g, ""), "base64")).digest("hex");
// This mock checks file transfer/confirmation only. Parsed key pairing and CA
// purpose admission use the real HTTP/private-PG contracts.
const key = Buffer.from("-----BEGIN PRIVATE KEY-----\nAAAA\n-----END PRIVATE KEY-----\n");
async function open(page: Page, role = "admin") {
  const writes: { path: string; value: unknown }[] = [];
  await page.route("**/api/v1/**", async route => {
    const path = new URL(route.request().url()).pathname;
    if (path === "/api/v1/session") return route.fulfill({ json: { id: "review-admin", username: "review", tenantId: "review", role, csrf: "review-csrf" } });
    if (route.request().method() === "PUT") {
      expect(route.request().headers()["x-csrf-token"]).toBe("review-csrf");
      writes.push({ path, value: route.request().postDataJSON() });
      return route.fulfill({ json: { staged: true, restartRequired: true } });
    }
    if (path.endsWith("/settings/client-ca")) return route.fulfill({ json: { caFingerprint: "1".repeat(64), registeredGateways: 3 } });
    if (path.endsWith("/settings/tls")) return route.fulfill({ json: { certificateFingerprint: "2".repeat(64) } });
    if (path.endsWith("/settings/task-key")) return route.fulfill({ json: { keyFingerprint: "3".repeat(64) } });
    return route.fulfill({ json: { items: [] } });
  });
  await page.goto("/settings");
  return writes;
}
test("CA file selection requires fingerprint review and confirmation before one upload", async ({ page }) => {
  const writes = await open(page);
  await page.getByLabel("选择文件: 客户端 CA 证书 PEM", { exact: true }).setInputFiles({ name: "gateway-ca.pem", mimeType: "application/x-pem-file", buffer: ca });
  await expect(page.getByLabel("客户端 CA 证书 PEM", { exact: true })).toHaveValue(ca.toString());
  expect(writes).toHaveLength(0);
  const card = page.getByLabel("客户端 CA 证书 PEM", { exact: true }).locator("xpath=ancestor::*[contains(@class,'ant-card ')]");
  await expect(card.getByRole("button", { name: /上传客户端 CA$/ })).toBeDisabled();
  await card.getByRole("button", { name: "核对 PEM 指纹", exact: true }).click();
  await expect(card.getByText(hash(ca), { exact: true })).toBeVisible();
  await card.getByRole("checkbox").check();
  await card.getByRole("button", { name: /上传客户端 CA$/ }).click();
  await expect.poll(() => writes.length).toBe(1);
  expect(writes).toEqual([{ path: "/api/v1/settings/client-ca", value: { clientCaPem: ca.toString() } }]);
});
test("TLS file replacement clears confirmation and file controls translate", async ({ page }) => {
  const writes = await open(page);
  await page.getByLabel("选择文件: 服务器证书 PEM", { exact: true }).setInputFiles({ name: "server.pem", mimeType: "application/x-pem-file", buffer: leaf });
  await page.getByLabel("选择文件: 服务器私钥 PEM", { exact: true }).setInputFiles({ name: "server.key", mimeType: "application/x-pem-file", buffer: key });
  const card = page.getByLabel("服务器证书 PEM", { exact: true }).locator("xpath=ancestor::*[contains(@class,'ant-card ')]");
  await card.getByRole("button", { name: "核对 PEM 指纹", exact: true }).click();
  await card.getByRole("checkbox").check();
  await expect(card.getByRole("button", { name: "上传服务器证书", exact: true })).toBeEnabled();
  await page.getByLabel("选择文件: 服务器私钥 PEM", { exact: true }).setInputFiles({ name: "replacement.key", mimeType: "application/x-pem-file", buffer: Buffer.from(key.toString() + "\n") });
  await expect(card.getByRole("checkbox")).not.toBeChecked();
  await expect(card.getByRole("button", { name: "上传服务器证书", exact: true })).toBeDisabled();
  await page.getByRole("combobox", { name: "语言 / Language" }).first().selectOption("en-US");
  await expect(page.getByLabel("Choose file: Server certificate PEM", { exact: true })).toBeVisible();
  expect(writes).toHaveLength(0);
});
test("oversized files are rejected locally without settings writes", async ({ page }) => {
  const writes = await open(page);
  await page.getByLabel("选择文件: 客户端 CA 证书 PEM", { exact: true }).setInputFiles({ name: "valid.pem", mimeType: "application/x-pem-file", buffer: ca });
  const card = page.getByLabel("客户端 CA 证书 PEM", { exact: true }).locator("xpath=ancestor::*[contains(@class,'ant-card ')]");
  await card.getByRole("button", { name: "核对 PEM 指纹", exact: true }).click();
  await card.getByRole("checkbox").check();
  await page.getByLabel("选择文件: 客户端 CA 证书 PEM", { exact: true }).setInputFiles({ name: "oversized.pem", mimeType: "application/x-pem-file", buffer: Buffer.alloc(65537, 65) });
  await expect(card.getByRole("checkbox")).not.toBeChecked();
  await expect(card.getByRole("button", { name: /上传客户端 CA$/ })).toBeDisabled();
  await expect(page.getByRole("alert").filter({ hasText: "文件过大" })).toBeVisible();
  expect(writes).toHaveLength(0);
});
test("nonadmin has no certificate file controls", async ({ page }) => {
  await open(page, "viewer");
  await expect(page.locator('input[type="file"]')).toHaveCount(0);
});

test("a completed old fingerprint review cannot approve a newly selected file", async ({ page }) => {
  const writes = await open(page);
  await page.getByLabel("选择文件: 客户端 CA 证书 PEM", { exact: true }).setInputFiles({ name: "first.pem", mimeType: "application/x-pem-file", buffer: ca });
  await expect(page.getByLabel("客户端 CA 证书 PEM", { exact: true })).toHaveValue(ca.toString());
  await page.evaluate(() => {
    const original = crypto.subtle.digest.bind(crypto.subtle);
    let release!: () => void;
    const wait = new Promise<void>(resolve => { release = resolve; });
    Object.assign(window, { reviewDigestRelease: release, reviewDigestDone: false });
    crypto.subtle.digest = async (algorithm, data) => { await wait; const result = await original(algorithm, data); Object.assign(window, { reviewDigestDone: true }); return result; };
  });
  const card = page.getByLabel("客户端 CA 证书 PEM", { exact: true }).locator("xpath=ancestor::*[contains(@class,'ant-card ')]");
  await card.getByRole("button", { name: "核对 PEM 指纹", exact: true }).click();
  await page.getByLabel("选择文件: 客户端 CA 证书 PEM", { exact: true }).setInputFiles({ name: "second.pem", mimeType: "application/x-pem-file", buffer: leaf });
  await expect(page.getByLabel("客户端 CA 证书 PEM", { exact: true })).toHaveValue(leaf.toString());
  await page.evaluate(() => { (window as unknown as { reviewDigestRelease: () => void }).reviewDigestRelease(); });
  await page.waitForFunction(() => (window as unknown as { reviewDigestDone: boolean }).reviewDigestDone);
  await page.evaluate(() => new Promise<void>(resolve => requestAnimationFrame(() => requestAnimationFrame(() => resolve()))));
  await expect(card.getByText(hash(ca), { exact: true })).toHaveCount(0);
  await expect(card.getByRole("button", { name: /上传客户端 CA$/ })).toBeDisabled();
  expect(writes).toHaveLength(0);
});
