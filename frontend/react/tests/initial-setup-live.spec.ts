import { test, expect } from "@playwright/test";
import { randomBytes } from "node:crypto";
import { openSync, writeFileSync, closeSync, readFileSync, existsSync } from "node:fs";
import { resolve } from "node:path";

test("fresh installed admin/admin is gated until its password is set, then enters bilingually", async ({ page }) => {
  const fixture = process.env.KURA_BROWSER_INITIAL_SETUP_FIXTURE;
  if (!fixture) throw new Error("An explicitly named new initial-setup fixture is required");
  const resume = process.env.KURA_BROWSER_INITIAL_SETUP_RESUME === "before-password";
  let password: string;
  if (resume) {
    if (!existsSync(resolve(fixture, "initial-setup.started")) || existsSync(resolve(fixture, "initial-setup.json"))) {
      throw new Error("Only an inspected, unfinished pre-password checkpoint may resume");
    }
    password = readFileSync(resolve(fixture, "admin.password"), "utf8");
  } else {
    closeSync(openSync(resolve(fixture, "initial-setup.started"), "wx", 0o600));
    password = randomBytes(24).toString("base64url");
    const descriptor = openSync(resolve(fixture, "admin.password"), "wx", 0o600);
    writeFileSync(descriptor, password); closeSync(descriptor);
  }
  let submissions = 0;
  page.on("request", request => {
    if (request.method() === "POST" && request.url().endsWith("/api/v1/auth/password")) {
      expect(Object.keys(request.postDataJSON())).toEqual(["newPassword"]); submissions++;
    }
  });
  await page.goto("/login");
  await page.getByLabel("账号", { exact: true }).fill("admin");
  await page.getByLabel("密码", { exact: true }).fill("admin");
  await page.getByRole("button", { name: /登\s*录/ }).click();
  await expect(page.getByRole("heading", { name: "设置管理员密码", exact: true })).toBeVisible();
  const denied = await page.evaluate(async () => {
    const response = await fetch("/api/v1/gateways");
    return { status: response.status, body: await response.json() };
  });
  expect(denied.status).toBe(403); expect(denied.body.error.code).toBe("PASSWORD_CHANGE_REQUIRED");
  await expect(page.getByLabel("当前密码", { exact: true })).toHaveCount(0);
  await page.reload();
  await expect(page.getByRole("heading", { name: "设置管理员密码", exact: true })).toBeVisible();
  await page.getByLabel("新密码（至少 12 个字符）", { exact: true }).fill(password);
  await page.getByLabel("确认新密码", { exact: true }).fill(password);
  await page.screenshot({ path: resolve(fixture, "initial-setup-form.png"), fullPage: true });
  await page.getByRole("button", { name: "设置密码并继续", exact: true }).click();
  await expect(page.getByRole("heading", { name: "网关总览", exact: true })).toBeVisible();
  expect(await page.evaluate(async () => (await (await fetch("/api/v1/session")).json()).mustChangePassword)).toBe(false);
  expect(submissions).toBe(1);
  await page.screenshot({ path: resolve(fixture, "initial-setup-zh.png"), fullPage: true });
  await page.getByRole("button", { name: /退\s*出/ }).click();
  await page.getByLabel("账号", { exact: true }).fill("admin");
  await page.getByLabel("密码", { exact: true }).fill("admin");
  await page.getByRole("button", { name: /登\s*录/ }).click();
  await expect(page.getByRole("alert")).toContainText("账号或密码错误");
  await page.getByLabel("密码", { exact: true }).fill(password);
  await page.getByRole("button", { name: /登\s*录/ }).click();
  await expect(page.getByRole("heading", { name: "网关总览", exact: true })).toBeVisible();
  await page.getByRole("combobox", { name: "语言 / Language" }).selectOption("en-US");
  await expect(page.getByRole("heading", { name: "Gateways", exact: true })).toBeVisible();
  await page.screenshot({ path: resolve(fixture, "initial-setup-en.png"), fullPage: true });
  writeFileSync(resolve(fixture, "initial-setup.json"), JSON.stringify({ status: "passed", initialCredentials: "admin/admin", gatewayApiGated: true, setupFields: ["newPassword", "confirm"], directEntryAfterSetup: true, defaultPasswordRejectedAfterSetup: true, bilingual: true, passwordSubmissions: submissions, secretValuesIncluded: false }, null, 2));
});
