import { test, expect, type Page } from "@playwright/test";
import { readFileSync } from "node:fs";

const username = process.env.PACKAGE_BROWSER_USERNAME;
const passwordFile = process.env.PACKAGE_BROWSER_PASSWORD_FILE;
const gatewayId = process.env.PACKAGE_BROWSER_GATEWAY_ID;
const gatewayName = process.env.PACKAGE_BROWSER_GATEWAY_NAME;
const evidencePrefix = process.env.EVIDENCE_PREFIX || "package-center";

test.skip(!username || !passwordFile || !gatewayId || !gatewayName, "package-center credentials are required");

test.use({ locale: "zh-CN" });

async function submit(page: Page, password: string) {
  await page.getByLabel("密码", { exact: true }).fill(password);
  await page.getByRole("button", { name: /登\s*录/ }).click();
}

test("installed package payload supports bilingual login, gateway status and task history", async ({ page }) => {
  const errors: string[] = [];
  page.on("pageerror", (error) => errors.push(error.message));
  const password = readFileSync(passwordFile!, "utf8").trim();

  await page.goto("/login");
  await expect(page).toHaveTitle("YOFC · 网关管理中心");
  await page.screenshot({
    path: `../../target/${evidencePrefix}-login-zh.png`,
    fullPage: true,
  });
  await page.getByLabel("账号", { exact: true }).fill(username!);
  await submit(page, "wrong-package-password");
  await expect(page.getByRole("alert")).toContainText("账号或密码错误");
  await page.getByRole("combobox", { name: "语言 / Language" }).selectOption("en-US");
  await expect(page.getByRole("alert")).toContainText("Incorrect username or password.");

  await page.getByLabel("Username", { exact: true }).fill(username!);
  await page.getByLabel("Password", { exact: true }).fill(password);
  await page.getByRole("button", { name: "Sign in", exact: true }).click();
  await expect(page.getByRole("heading", { name: "Gateways", exact: true })).toBeVisible();

  const row = page.getByRole("row").filter({ hasText: gatewayId! });
  await expect(row).toContainText(gatewayName!);
  await expect(row.getByText("Online", { exact: true })).toBeVisible();
  await page.screenshot({
    path: `../../target/${evidencePrefix}-gateways-en.png`,
    fullPage: true,
  });
  await row.getByRole("button", { name: "Query status", exact: true }).click();
  await page.getByRole("link", { name: "Tasks", exact: false }).click();
  await expect(page.getByRole("heading", { name: "Tasks", exact: true })).toBeVisible();
  await expect(page.getByText("Completed", { exact: true }).first()).toBeVisible();
  await page.screenshot({
    path: `../../target/${evidencePrefix}-tasks-en.png`,
    fullPage: true,
  });

  await page.getByRole("combobox", { name: "语言 / Language" }).selectOption("zh-CN");
  await expect(page.getByRole("heading", { name: "任务记录", exact: true })).toBeVisible();
  await page.reload();
  await expect(page.getByRole("heading", { name: "任务记录", exact: true })).toBeVisible();
  await page.getByRole("button", { name: /退\s*出/ }).click();
  await expect(page.getByRole("heading", { name: "登录工作空间" })).toBeVisible();
  expect(errors).toEqual([]);
});
