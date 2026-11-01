import { test, expect, type Page, type BrowserContext } from "@playwright/test";
import { readFileSync } from "node:fs";
const runId = process.env.KURA_GWT_RUN_ID;
const evidence = `../../target/gwt-network-${runId ? `${runId}-` : ""}`;

// This suite requires the explicit one-shot IDEA fixture profile. It is never selected by the React suite.
test.describe.serial("original GWT network transactions with a test-only network owner", () => {
  let page: Page;
  let context: BrowserContext;
  const errors: string[] = [];
  const writes: string[] = [];
  let capturedToken = false;
  async function openNetwork(target: Page) {
    await target.getByRole("link", { name: /Network|网络/ }).first().click();
    await expect(target.getByRole("cell", { name: "fixture0", exact: true })).toBeVisible();
    await target.getByRole("cell", { name: "fixture0", exact: true }).click();
    await expect(target.locator("#ip")).toHaveValue("192.0.2.2");
  }
  async function login(target: Page) {
    await target.goto("/");
    await target.locator("input[type=text]").first().fill("admin");
    await target.locator("input[type=password]").first().fill("admin");
    await target.locator("#login-button").click();
    await expect(target.getByRole("link", { name: /Network|网络/ }).first()).toBeVisible();
    await openNetwork(target);
  }
  async function mtu(value: string) {
    await page.getByRole("link", { name: /Advanced|高级/, exact: true }).click();
    await page.locator("#mtu").fill(value);
    await page.locator("#mtu").press("Tab");
  }
  async function apply() {
    await expect(page.locator("#btnNetworkApply")).toBeEnabled();
    await page.locator("#btnNetworkApply").click();
    await page.getByRole("button", { name: /^(Yes|是)$/ }).click();
  }
  test.beforeAll(async ({ browser }) => {
    const fixture = readFileSync(`${evidence}fixture.txt`, "utf8");
    expect(fixture).toContain("testOnly=true");
    expect(fixture).toContain("prepares=0\n");
    context = await browser.newContext({ locale: "en-US", viewport: { width: 1440, height: 1100 } });
    await context.addCookies([{ name: "GWT_LOCALE", value: "zh_CN", url: "http://127.0.0.1:18080" }]);
    page = await context.newPage();
    page.on("pageerror", error => errors.push(error.message));
    page.on("request", request => {
      const body = request.postData() || "";
      for (const method of ["prepareNetworkTransaction", "confirmNetworkTransaction", "rollbackNetworkTransaction"])
        if (body.includes(method)) writes.push(method);
    });
    page.on("response", async response => {
      if (response.url().endsWith("/network")) {
        const body = await response.text().catch(() => "");
        if (body.includes("confirmationToken")) capturedToken = true;
      }
    });
    await login(page);
  });
  test.afterAll(async () => { await context?.close(); });

  test("Chinese and English compiled views preserve read-only boundaries", async () => {
    await expect(page.getByText(/网络由 agent 管理/)).toBeVisible();
    await expect(page.locator("#status")).toBeDisabled();
    await expect(page.locator("#priority")).toBeDisabled();
    await expect(page.locator("#configure")).toBeEnabled();
    await page.getByRole("cell", { name: "fixture-ro0", exact: true }).click();
    await expect(page.locator("#configure")).toBeDisabled();
    await expect(page.locator("#ip")).toBeDisabled();
    await page.screenshot({ path: `${evidence}readonly-zh.png`, fullPage: true });
    // LoginUi writes its own cookie path. Replace that preference without duplicating cookie names.
    await context.clearCookies({ name: "GWT_LOCALE" });
    await context.addCookies([{ name: "GWT_LOCALE", value: "en", url: "http://127.0.0.1:18080" }]);
    await page.reload();
    await openNetwork(page);
    await expect(page.getByText(/Agent-managed network/)).toBeVisible();
    await expect(page.locator("#btnNetworkConfirmTransaction")).toHaveText("Confirm network change");
    await expect(page.getByRole("link", { name: "IPv6", exact: true })).toHaveCount(0);
    await mtu("1400");
    await expect(page.locator("#promisc")).toBeDisabled();
    await expect(page.locator("#ip6Mtu")).toBeDisabled();
    expect(errors).toEqual([]);
  });
  test("candidate requires path verification and cannot be confirmed by another session", async ({ browser }) => {
    await apply();
    await expect(page.getByText(/Waiting for confirmation/)).toBeVisible();
    await expect(page.locator("#btnNetworkConfirmTransaction")).toBeDisabled();
    await expect(page.locator("#btnNetworkRollbackTransaction")).toBeEnabled();
    await expect(page.locator("#btnNetworkApply")).toBeDisabled();
    const other = await browser.newContext({ locale: "en-US" });
    try {
      await other.addCookies([{ name: "GWT_LOCALE", value: "en", url: "http://127.0.0.1:18080" }]);
      const view = await other.newPage();
      await login(view);
      await expect(view.getByText(/Waiting for confirmation/)).toBeVisible();
      await expect(view.locator("#btnNetworkConfirmTransaction")).toBeDisabled();
      await expect(view.locator("#btnNetworkRollbackTransaction")).toBeDisabled();
    } finally { await other.close(); }
    await page.getByRole("checkbox", { name: /I have reconnected/ }).check();
    await Promise.all([
      page.waitForResponse(response => response.url().endsWith("/network") &&
        (response.request().postData() || "").includes("findNetInterfaceConfigurations")),
      page.locator("#btnNetworkConfirmTransaction").click(),
    ]);
    await expect(page.getByText(/Network transaction: Confirmed\./)).toBeVisible();
      await expect(page.getByText("Network transaction: Confirmed.", { exact: true })).toBeVisible();
      await expect(page.getByText(/Confirmation time remaining/)).toHaveCount(0);
      await page.screenshot({ path: `${evidence}confirmed-en.png`, fullPage: true });
    expect(writes).toEqual(["prepareNetworkTransaction", "confirmNetworkTransaction"]);
    expect(capturedToken).toBe(false);
  });
  test("explicit rollback keeps the previously confirmed settings", async () => {
    await mtu("1410");
    await apply();
    await expect(page.getByText(/Waiting for confirmation/)).toBeVisible();
    await expect(page.locator("#btnNetworkConfirmTransaction")).toBeDisabled();
    await Promise.all([
      page.waitForResponse(response => response.url().endsWith("/network") &&
        (response.request().postData() || "").includes("findNetInterfaceConfigurations")),
      page.locator("#btnNetworkRollbackTransaction").click(),
    ]);
    await expect(page.getByText(/Network transaction: Rolled back\./)).toBeVisible();
      await expect(page.getByText("Network transaction: Rolled back.", { exact: true })).toBeVisible();
      await expect(page.getByText(/Confirmation time remaining/)).toHaveCount(0);
      await context.clearCookies({ name: "GWT_LOCALE" });
      await context.addCookies([{ name: "GWT_LOCALE", value: "zh_CN", url: "http://127.0.0.1:18080" }]);
      await page.reload();
      await openNetwork(page);
      await expect(page.getByText("网络事务：已回退。", { exact: true })).toBeVisible();
      await expect(page.getByText(/剩余确认时间/)).toHaveCount(0);
      await page.screenshot({ path: `${evidence}rolled-back-zh.png`, fullPage: true });
      await context.clearCookies({ name: "GWT_LOCALE" });
      await context.addCookies([{ name: "GWT_LOCALE", value: "en", url: "http://127.0.0.1:18080" }]);
      await page.reload();
      await openNetwork(page);
    await mtu("1301");
    await apply();
    await expect(page.getByText(/INVALID_ARGUMENT/)).toBeVisible();
    await expect(page.locator("#btnNetworkApply")).toBeEnabled();
    expect(writes.filter(v => v === "prepareNetworkTransaction")).toHaveLength(3);
    expect(errors).toEqual([]);
  });
  test("accepted prepare with a lost receipt is observed without retry or token recovery", async () => {
    await mtu("1302");
    await apply();
    await expect(page.getByText(/TIMEOUT/)).toBeVisible();
    await expect(page.getByText(/Waiting for confirmation/)).toBeVisible();
    await expect(page.locator("#btnNetworkApply")).toBeDisabled();
    await expect(page.locator("#btnNetworkConfirmTransaction")).toBeDisabled();
    await expect(page.locator("#btnNetworkRollbackTransaction")).toBeDisabled();
    // Wait for a subsequent observation, not a fixed delay, then prove no write was replayed.
    await page.waitForResponse(response => response.url().endsWith("/network") &&
      (response.request().postData() || "").includes("findNetworkTransaction"));
    expect(writes.filter(v => v === "prepareNetworkTransaction")).toHaveLength(4);
    expect(capturedToken).toBe(false);
    expect(errors).toEqual([]);
      await page.screenshot({ path: `${evidence}lost-receipt-en.png`, fullPage: true });
  });
});
