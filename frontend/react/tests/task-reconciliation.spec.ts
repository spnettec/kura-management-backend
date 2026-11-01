import { test, expect } from "@playwright/test";

test("uncertain outcomes remain explicit in both languages (stubbed task receipts)", async ({ page }) => {
  await page.addInitScript(() => {
    class ControlledEventSource extends EventTarget {
      onopen: (() => void) | null = null;
      constructor(_url: string) {
        super();
        (window as unknown as { taskSource: EventTarget }).taskSource = this;
        setTimeout(() => this.onopen?.(), 0);
      }
      close() {}
    }
    Object.defineProperty(window, "EventSource", { value: ControlledEventSource });
  });
  await page.route("**/api/v1/session", route => route.fulfill({ json: {
    id: "reconciliation-ui", username: "operator", tenantId: "fixture", role: "operator", csrf: "fixture",
  } }));
  await page.route("**/api/v1/gateways?*", route => route.fulfill({ json: { items: [] } }));
  let reconciled = false;
  await page.route(/\/api\/v1\/tasks(?:\?.*)?$/, route => route.fulfill({ json: { items: [
    { id: "10000000-0000-4000-8000-000000000001", gatewayId: "fixture", operation: 5,
      state: reconciled ? "SUCCEEDED" : "UNKNOWN", error: reconciled ? null : "OUTCOME_UNKNOWN",
      result: reconciled ? { state: "COMMITTED", reconciliation: {
        source: "network.journal.v1", commandId: "10000000-0000-4000-8000-000000000001", operation: 5, state: "COMMITTED",
      } } : null, createdAt: "2026-11-01T00:00:00Z" },
    { id: "20000000-0000-4000-8000-000000000002", gatewayId: "fixture", operation: 11, state: "SUCCEEDED", error: null, result: {}, createdAt: "2026-11-01T00:00:00Z" },
  ] } }));
  await page.goto("/commands");
  await expect(page.getByText("未知状态", { exact: true })).toBeVisible();
  await expect(page.getByText(/执行结果尚不确定/)).toBeVisible();
  await expect(page.locator(".metrics > div").filter({ hasText: "未决任务" }).locator("strong")).toHaveText("01");
  // A configuration read is a completed read, not a network transaction receipt.
  await expect(page.getByText("已完成", { exact: true })).toBeVisible();
  await page.getByRole("combobox", { name: "语言 / Language" }).selectOption("en-US");
  await expect(page.getByText("Unknown status", { exact: true })).toBeVisible();
  await expect(page.getByText(/Do not submit the write again/)).toBeVisible();
  await page.getByRole("button", { name: /Apply network candidate 10000000/ }).click();
  await expect(page.getByRole("dialog").locator("pre")).toContainText('"state": "UNKNOWN"');
  await page.screenshot({ path: "../../target/network-outcome-uncertain-ui-20261101.png", fullPage: true });
  reconciled = true;
  await page.evaluate(() => (window as unknown as { taskSource: EventTarget }).taskSource.dispatchEvent(new Event("task")));
  await expect(page.getByRole("dialog").locator("pre")).toContainText('"state": "SUCCEEDED"');
  await expect(page.getByRole("dialog").locator("pre")).toContainText('"state": "COMMITTED"');
  await expect(page.getByRole("dialog").getByText("Resolved from the gateway transaction journal: Committed", { exact: true })).toBeVisible();
  await expect(page.getByRole("dialog").getByText(/Reconciliation did not execute/)).toBeVisible();
  await expect(page.locator(".metrics > div").filter({ hasText: "Unresolved tasks" }).locator("strong")).toHaveText("00");
  await page.getByRole("dialog").getByRole("combobox", { name: "语言 / Language" }).selectOption("zh-CN");
  await expect(page.getByText("任务结果", { exact: true })).toBeVisible();
  await expect(page.getByRole("dialog").getByText("已依据网关事务日志核定：已确认", { exact: true })).toBeVisible();
  await expect(page.getByRole("dialog").locator("pre")).toContainText('"state": "COMMITTED"');
  await page.screenshot({ path: "../../target/network-outcome-reconciled-ui-20261101.png", fullPage: true });
});

test("rolled-back preparation stays failed and unrelated proof is not labelled (stubbed receipts)", async ({ page }) => {
  await page.addInitScript(() => {
    class OpenEventSource extends EventTarget {
      onopen: (() => void) | null = null;
      constructor(_url: string) { super(); setTimeout(() => this.onopen?.(), 0); }
      close() {}
    }
    Object.defineProperty(window, "EventSource", { value: OpenEventSource });
  });
  await page.route("**/api/v1/session", route => route.fulfill({ json: {
    id: "proof-ui", username: "operator", tenantId: "fixture", role: "operator", csrf: "fixture",
  } }));
  await page.route("**/api/v1/gateways?*", route => route.fulfill({ json: { items: [] } }));
  const id = "30000000-0000-4000-8000-000000000003";
  const failed = { id, gatewayId: "fixture", operation: 5, state: "FAILED", error: "CONFLICT",
    result: { state: "ROLLED_BACK", reconciliation: { source: "network.journal.v1", commandId: id, operation: 5, state: "ROLLED_BACK" } },
    createdAt: "2026-11-01T00:00:00Z" };
  await page.route(/\/api\/v1\/tasks(?:\?.*)?$/, route => route.fulfill({ json: { items: [
    failed, { ...failed, id: "40000000-0000-4000-8000-000000000004" },
  ] } }));
  await page.goto("/commands");
  await page.getByRole("button", { name: /应用网络候选 30000000/ }).click();
  await expect(page.getByRole("dialog").getByText("已依据网关事务日志核定：已回滚", { exact: true })).toBeVisible();
  await expect(page.getByRole("dialog").locator("pre")).toContainText('"state": "FAILED"');
  await page.getByRole("dialog").getByRole("combobox", { name: "语言 / Language" }).selectOption("en-US");
  await expect(page.getByRole("dialog").getByText("Resolved from the gateway transaction journal: Rolled back", { exact: true })).toBeVisible();
  await page.getByRole("button", { name: "Close" }).click();
  await page.getByRole("button", { name: /Apply network candidate 40000000/ }).click();
  await expect(page.getByRole("dialog").getByText(/Resolved from the gateway transaction journal/)).toHaveCount(0);
});
