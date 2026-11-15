import { test, expect } from "@playwright/test";
import { readFileSync, mkdirSync, readdirSync } from "node:fs";
import { resolve } from "node:path";

test("Fresh console initializes through UI and switches service in the same process", async ({ page }) => {
  test.skip(!process.env.YOFC_INITIAL_SETUP_SCENE, "A distinct unentered scene must be selected explicitly");
  const scene = process.env.YOFC_INITIAL_SETUP_SCENE!;
  if (readdirSync(resolve(scene,"live-center/state/setup-operations")).some(name=>/\.(accepted|intent|receipt)\.json$/.test(name)))
    throw new Error("This one-time setup scene is already entered; only read its original operation, never replay initialization");
  const data = JSON.parse(readFileSync(resolve(scene, "live-center/browser-input.json"), "utf8"));
  const fixture = resolve(import.meta.dirname, "../../../crates/console/tests/fixtures/tls-live");
  const dispatched: string[] = [];
  page.on("request", async request => {
    if (request.method() !== "POST" || !request.url().endsWith("/api/setup/operations")) return;
    const body = request.postDataJSON();
    const saved = await page.evaluate(() => JSON.parse(localStorage.getItem("yofc.console.initial-setup.v1")!));
    expect(saved.operationId).toBe(body.operationId);
    expect(saved).not.toHaveProperty("password"); expect(saved).not.toHaveProperty("databaseUrl"); expect(saved).not.toHaveProperty("serverKey");
    dispatched.push(body.operationId);
  });
  await page.goto("/setup");
  await page.getByLabel("安装凭据文件（JSON）", { exact: true }).setInputFiles(resolve(scene, "live-center/state/setup/installation-credential.json"));
  await page.getByRole("checkbox", { name: "我已核对浏览器证书指纹与可信安装凭据一致。", exact: true }).check();
  await page.getByRole("button", { name: "解锁首次配置", exact: true }).click();
  for (const [label, value] of [
    ["中心 HTTPS 地址", "https://localhost:18464"], ["代理通道监听地址", "127.0.0.1:18465"],
    ["数据库主机", data.databaseHost], ["数据库端口", data.databasePort], ["数据库名称", data.databaseName],
    ["数据库账号", data.databaseUser], ["数据库密码", data.databasePassword],
    ["组织标识", data.tenant], ["管理员账号", data.username], ["管理员密码（至少12字节）", data.password],
  ]) await page.getByLabel(label, { exact: true }).fill(value);
  for (const [label, file] of [["中心服务器证书文件", "server-one.pem"], ["中心服务器私钥文件", "server-one.key"], ["网关公共 CA 证书文件", "ca-one.pem"]]) {
    const input = page.getByLabel(label, { exact: true }); await expect(input).toBeEnabled(); await input.setInputFiles(resolve(fixture, file));
  }
  const review = page.getByRole("button", { name: "核对本次配置与证书指纹", exact: true }); await expect(review).toBeEnabled(); await review.click();
  await page.getByRole("checkbox", { name: "我已核对中心地址、证书和组织账号，确认初始化专用空数据库。", exact: true }).check();
  await page.getByRole("button", { name: "初始化并进入管理中心", exact: true }).click();
  // The old TLS connection is closed. The page only reads the original, detects
  // configured mode and asks for the administrator chosen above.
  await expect(page.getByRole("heading", { name: "登录工作空间", exact: true })).toBeVisible({ timeout: 15000 });
  await page.getByLabel("账号", { exact: true }).fill(data.username);
  await page.getByLabel("密码", { exact: true }).fill(data.password);
  await page.getByRole("button", { name: /^登\s*录$/ }).click();
  await expect(page.getByText("原初始化已记录完成回执。请使用配置的组织管理员账号登录。", { exact: true })).toBeVisible();
  expect(dispatched).toHaveLength(1);
  const original = await page.request.get(`/api/v1/settings/initial-setup/${dispatched[0]}`);
  expect(original.status()).toBe(200); expect((await original.json()).state).toBe("APPLIED");
  await page.reload();
  await expect(page.getByText("原初始化已记录完成回执。请使用配置的组织管理员账号登录。", { exact: true })).toBeVisible();
  expect(dispatched).toHaveLength(1);
  const ready = await page.request.get("/health/ready"); expect(ready.status()).toBe(200);
  mkdirSync(resolve(scene, "browser"), { recursive: true });
  await page.screenshot({ path: resolve(scene, "browser/initial-configuration-live.png"), fullPage: true });
});
