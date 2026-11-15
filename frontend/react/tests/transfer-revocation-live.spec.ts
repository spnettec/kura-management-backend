import { test, expect, type Page } from "@playwright/test";
import { readFileSync } from "node:fs";

const username = process.env.TRANSFER_BROWSER_USERNAME;
const passwordFile = process.env.TRANSFER_BROWSER_PASSWORD_FILE;
const gatewayId = process.env.TRANSFER_BROWSER_GATEWAY_ID;
const gatewayName = process.env.TRANSFER_BROWSER_GATEWAY_NAME;
const activePrefix = process.env.TRANSFER_BROWSER_ACTIVE_PREFIX;
const evidencePrefix = process.env.EVIDENCE_PREFIX || "transfer-revocation";

test.skip(
  !username || !passwordFile || !gatewayId || !gatewayName || !activePrefix,
  "transfer-revocation live credentials are required",
);

test.use({ locale: "zh-CN" });

test("issued transfer authorizations are visible and revocable on the live console", async ({ page }) => {
  const errors: string[] = [];
  page.on("pageerror", (error) => errors.push(error.message));
  const password = readFileSync(passwordFile!, "utf8").trim();

  await page.goto("/login");
  await page.getByLabel("账号", { exact: true }).fill(username!);
  await page.getByLabel("密码", { exact: true }).fill(password);
  await page.getByRole("button", { name: /登\s*录/ }).click();
  await expect(page.getByRole("heading", { name: /网关/ })).toBeVisible();

  const row = page.getByRole("row").filter({ hasText: gatewayId });
  await expect(row).toContainText(gatewayName!);
  await page
    .getByRole("button", { name: `${gatewayName} ${gatewayId}`, exact: true })
    .click();
  const panel = page.getByRole("region", { name: /跨中心转移审批|Cross-center transfer approval/ });
  await expect(panel).toBeVisible();

  const issued = panel.getByRole("table").filter({ hasText: activePrefix! });
  await expect(issued).toBeVisible();
  await expect(panel.getByText("已签发转移授权")).toBeVisible();
  await expect(panel.getByText(activePrefix!, { exact: false }).first()).toBeVisible();
  await expect(panel.getByText("撤销只阻止尚未消费的一次性授权；已完成的转移不受影响，审批不会重新打开。")).toBeVisible();
  await page.screenshot({
    path: `../../target/${evidencePrefix}-issuances-zh.png`,
    fullPage: true,
  });

  await panel.getByRole("button", { name: "撤销授权" }).click();
  await expect(panel.getByText("已撤销").first()).toBeVisible();
  await expect(panel.getByRole("button", { name: "撤销授权" })).toHaveCount(0);
  await page.screenshot({
    path: `../../target/${evidencePrefix}-revoked-zh.png`,
    fullPage: true,
  });

  await page
    .getByRole("dialog", { name: gatewayName!, exact: true })
    .getByRole("combobox", { name: "语言 / Language" })
    .selectOption("en-US");
  await expect(panel.getByText("Issued transfer authorizations")).toBeVisible();
  await expect(panel.getByText("Revoked").first()).toBeVisible();
  await page.screenshot({
    path: `../../target/${evidencePrefix}-revoked-en.png`,
    fullPage: true,
  });

  expect(errors).toEqual([]);
});
