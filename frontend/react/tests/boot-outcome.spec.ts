import { test, expect } from "@playwright/test";

test("original boot task is reconciled without creating another task, in both languages", async ({ page }) => {
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
    id, gatewayId: "fixture", operation: 17, state: resolved ? "SUCCEEDED" : "UNKNOWN",
    error: resolved ? null : "OUTCOME_UNKNOWN", createdAt: "2026-11-06T00:00:00Z",
    result: resolved ? { id: "boot-11111111-1111-4111-8111-111111111111", state: "BOOT_POLICY_COMMITTED",
      bootVerified: false, kuraRecoveryVerified: false, reconciliation: {
        source: "ordinary.boot.journal.v1", commandId: id, operation: 17,
      } } : null,
  }] } }));
  await page.goto("/commands");
  await page.getByRole("button", { name: /确认启动策略 60000000/ }).click();
  await expect(page.getByRole("dialog").locator("pre")).toContainText('"state": "UNKNOWN"');
  resolved = true;
  await page.evaluate(() => (window as unknown as { bootSource: EventTarget }).bootSource.dispatchEvent(new Event("task")));
  await expect(page.getByRole("dialog").getByText("已依据原命令的启动策略决定核定", { exact: true })).toBeVisible();
  await expect(page.getByRole("dialog").getByText(/不代表实际开机或完整 Kura 业务恢复通过/)).toBeVisible();
  await expect(page.getByRole("dialog").locator("pre")).toContainText(id);
  await page.getByRole("dialog").getByRole("combobox", { name: "语言 / Language" }).selectOption("en-US");
  await expect(page.getByRole("dialog").getByText("Resolved from the original command's boot policy decision", { exact: true })).toBeVisible();
  await expect(page.getByRole("dialog").getByText(/did not execute the command again/)).toBeVisible();
  expect(writes).toBe(0);
  await page.screenshot({ path: "../../target/boot-outcome-reconciliation-20261106/browser-proof-en.png", fullPage: true });
});

for (const invalid of ["wrong-command", "full-recovery-claim"] as const) {
  test(`boot proof label rejects ${invalid}`, async ({ page }) => {
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
      id, gatewayId: "fixture", operation: 17, state: "SUCCEEDED", error: null, createdAt: "2026-11-06T00:00:00Z",
      result: { state: "BOOT_POLICY_COMMITTED", bootVerified: false, kuraRecoveryVerified: invalid === "full-recovery-claim", reconciliation: {
        source: "ordinary.boot.journal.v1", commandId: invalid === "wrong-command" ? "different" : id, operation: 17,
      } },
    }] } }));
    await page.goto("/commands");
    await page.getByRole("button", { name: /确认启动策略 70000000/ }).click();
    await expect(page.getByRole("dialog").getByText("已依据原命令的启动策略决定核定", { exact: true })).toHaveCount(0);
  });
}
