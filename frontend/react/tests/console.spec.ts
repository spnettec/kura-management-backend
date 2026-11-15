import { test, expect, type Page } from "@playwright/test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
const dev = process.env.KURA_BROWSER_FIXTURE || resolve(import.meta.dirname, "../../../.phase1-dev");
async function login(page: Page, user: string) {
  await page.goto("/login");
  await page.getByLabel("账号", { exact: true }).fill(user);
  await page
    .getByLabel("密码", { exact: true })
    .fill(readFileSync(`${dev}/${user}.password`, "utf8"));
  await page.getByRole("button", { name: /登\s*录/ }).click();
  await expect(
    page.getByRole("heading", { name: "网关总览", exact: true }),
  ).toBeVisible();
}
test("HTTPS login, live gateway, task result, refresh and logout", async ({
  page,
}) => {
  const errors: string[] = [];
  page.on("pageerror", (e) => errors.push(e.message));
  await page.goto("/login");
  await page.screenshot({
    path: `../../target/${process.env.EVIDENCE_PREFIX || "phase1"}-login.png`,
    fullPage: true,
  });
  await page.getByLabel("账号", { exact: true }).fill("admin");
  await page.getByLabel("密码", { exact: true }).fill("incorrect");
  await page.getByRole("button", { name: /登\s*录/ }).click();
  await expect(page.getByRole("alert")).toBeVisible();
  await login(page, "admin");
  await expect(
    page.getByRole("button", { name: "本机开发网关 dev-gateway" }),
  ).toBeVisible();
  await expect(page.getByText("实时连接正常").first()).toBeVisible();
  await page.screenshot({
    path: `../../target/${process.env.EVIDENCE_PREFIX || "phase1"}-gateways.png`,
    fullPage: true,
  });
  await page
    .getByRole("row")
    .filter({ hasText: "dev-gateway" })
    .getByRole("button", { name: "查询状态", exact: true })
    .click();
  await page.getByRole("link", { name: "任务记录" }).click();
  await expect(
    page.getByRole("heading", { name: "任务记录", exact: true }),
  ).toBeVisible();
  await expect(page.getByText("已完成").first()).toBeVisible();
  await page.reload();
  await expect(
    page.getByRole("heading", { name: "任务记录", exact: true }),
  ).toBeVisible();
  await page.getByRole("button", { name: /退\s*出/ }).click();
  await expect(
    page.getByRole("heading", { name: "登录工作空间" }),
  ).toBeVisible();
  expect(errors).toEqual([]);
});
test("viewer actions disabled; other tenant sees no gateway", async ({
  page,
}) => {
  await login(page, "viewer");
  await expect(
    page
      .getByRole("row")
      .filter({ hasText: "dev-gateway" })
      .getByRole("button", { name: "查询状态", exact: true }),
  ).toBeDisabled();
  await page.getByRole("button", { name: /退\s*出/ }).click();
  await login(page, "other");
  await expect(page.getByText("还没有已登记的网关")).toBeVisible();
  await expect(
    page.getByRole("button", { name: "本机开发网关 dev-gateway" }),
  ).toHaveCount(0);
});
test("network sources are observed through the live agent without enabling writes", async ({
  page,
}) => {
  const errors: string[] = [];
  page.on("pageerror", (e) => errors.push(e.message));
  await login(page, "admin");
  await page.getByRole("button", { name: "本机开发网关 dev-gateway" }).click();
  const panel = page.getByRole("region", { name: "有线网络管理" });
  await panel.getByRole("button", { name: "读取网络状态" }).click();
  await expect(
    panel.getByText("仅观测，尚未授权接口接管", { exact: false }),
  ).toBeVisible();
  await expect(panel.getByText(/^已确认版本/)).toBeVisible();
  await expect(panel.getByRole("button", { name: "应用候选配置" })).toHaveCount(
    0,
  );
  await page.screenshot({
    path: `../../target/${process.env.EVIDENCE_PREFIX || "phase2"}-network-observe.png`,
    fullPage: true,
  });
  expect(errors).toEqual([]);
});
