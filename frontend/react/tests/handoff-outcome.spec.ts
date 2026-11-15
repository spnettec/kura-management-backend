import { test, expect } from "@playwright/test";

test("original handoff task is reconciled without creating another task, in both languages", async ({ page }) => {
  const id = "60000000-0000-4000-8000-000000000006";
  let resolved = false;
  let writes = 0;
  page.on("request", request => { if (request.method() === "POST") writes++; });
  await page.addInitScript(() => {
    class Source extends EventTarget {
      onopen: (() => void) | null = null;
      constructor() { super(); (window as unknown as { bootSource: EventTarget }).bootSource = this; setTimeout(() => this.onopen?.(), 0); }
      close() {}
    }
    Object.defineProperty(window, "EventSource", { value: Source });
  });
  await page.route("**/api/v1/session", route => route.fulfill({ json: {
    id: "boot-proof-ui", username: "admin", tenantId: "fixture", role: "admin", csrf: "fixture",
  } }));
  await page.route("**/api/v1/gateways?*", route => route.fulfill({ json: { items: [] } }));
  await page.route(/\/api\/v1\/tasks(?:\?.*)?$/, route => route.fulfill({ json: { items: [{
    id, gatewayId: "fixture", operation: 22, state: resolved ? "SUCCEEDED" : "UNKNOWN",
    error: resolved ? null : "OUTCOME_UNKNOWN", createdAt: "2026-11-06T00:00:00Z",
    result: resolved ? { id: "handoff-11111111-1111-4111-8111-111111111111", state: "ORDINARY_ACTIVE_APPROVED_CHECKS",
      scope: "administrator-selected-checks", normalBootVerified: false, ordinaryUnitHandedOff: true, kuraRecoveryVerified: false, reconciliation: {
        source: "ordinary.handoff.journal.v1", commandId: id, operation: 22,
      } } : null,
  }] } }));
  await page.goto("/commands");
  await page.getByRole("button", { name: /执行普通配置交接 60000000/ }).click();
  await expect(page.getByRole("dialog").locator("pre")).toContainText('"state": "UNKNOWN"');
  resolved = true;
  await page.evaluate(() => (window as unknown as { bootSource: EventTarget }).bootSource.dispatchEvent(new Event("task")));
  await expect(page.getByRole("dialog").getByText("已依据原命令的普通交接决定核定", { exact: true })).toBeVisible();
  await expect(page.getByRole("dialog").getByText(/没有再次启停 Kura/)).toBeVisible();
  await expect(page.getByRole("dialog").locator("pre")).toContainText(id);
  await page.getByRole("dialog").getByRole("combobox", { name: "语言 / Language" }).selectOption("en-US");
  await expect(page.getByRole("dialog").getByText("Reconciled from the original ordinary handoff decision", { exact: true })).toBeVisible();
  await expect(page.getByRole("dialog").getByText(/did not start or stop Kura/)).toBeVisible();
  expect(writes).toBe(0);
  await page.screenshot({ path: "../../target/handoff-outcome-reconciliation-20261106/browser-proof-en.png", fullPage: true });
});

for (const invalid of ["wrong-command", "full-recovery-claim"] as const) {
  test(`handoff proof label rejects ${invalid}`, async ({ page }) => {
    const id = "70000000-0000-4000-8000-000000000007";
    await page.route("**/api/v1/session", route => route.fulfill({ json: {
      id: "boot-proof-negative", username: "admin", tenantId: "fixture", role: "admin", csrf: "fixture",
    } }));
    await page.route("**/api/v1/gateways?*", route => route.fulfill({ json: { items: [] } }));
    await page.addInitScript(() => {
      class Source extends EventTarget {
        onopen: (() => void) | null = null;
        constructor() { super(); setTimeout(() => this.onopen?.(), 0); }
        close() {}
      }
      Object.defineProperty(window, "EventSource", { value: Source });
    });
    await page.route(/\/api\/v1\/tasks(?:\?.*)?$/, route => route.fulfill({ json: { items: [{
      id, gatewayId: "fixture", operation: 22, state: "SUCCEEDED", error: null, createdAt: "2026-11-06T00:00:00Z",
      result: { state: "ORDINARY_ACTIVE_APPROVED_CHECKS", scope: "administrator-selected-checks", normalBootVerified: false, ordinaryUnitHandedOff: true, kuraRecoveryVerified: invalid === "full-recovery-claim", reconciliation: {
        source: "ordinary.handoff.journal.v1", commandId: invalid === "wrong-command" ? "different" : id, operation: 22,
      } },
    }] } }));
    await page.goto("/commands");
    await page.getByRole("button", { name: /执行普通配置交接 70000000/ }).click();
    await expect(page.getByRole("dialog").getByText("已依据原命令的普通交接决定核定", { exact: true })).toHaveCount(0);
  });
}
