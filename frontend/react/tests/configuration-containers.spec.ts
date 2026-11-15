import { expect, test } from "@playwright/test";

test("typed container projection keeps empty distinct from unset and masks password vectors in both languages (stubbed tasks)", async ({ page }) => {
  const tasks = new Map<string, object>();
  await page.route("**/api/v1/session", route => route.fulfill({ json: {
    id: "container-contract", username: "admin", tenantId: "fixture", role: "admin", csrf: "fixture",
  } }));
  await page.route("**/api/v1/gateways?*", route => route.fulfill({ json: { items: [{
    id: "container-gateway", name: "Container fixture", online: true, enabled: true, kuraAvailable: true,
    capabilities: ["agent.status", "kura.configuration.list", "kura.configuration.get"],
    revision: 0, lastSeen: null, agentVersion: "fixture", uptimeSeconds: 1,
  }] } }));
  await page.route("**/api/v1/events", () => {});
  await page.route(/\/api\/v1\/tasks(?:\?.*)?$/, route => route.fulfill({ json: { items: [] } }));
  await page.route("**/api/v1/tasks/*", route => route.fulfill({ json: tasks.get(route.request().url().split("/").at(-1)!) }));
  await page.route("**/api/v1/gateways/container-gateway/tasks", async route => {
    const body = route.request().postDataJSON();
    const id = crypto.randomUUID();
    tasks.set(id, { id, gatewayId: "container-gateway", state: "SUCCEEDED", error: null,
      operation: body.operation === "kura.configuration.list" ? 2 : 11,
      result: body.operation === "kura.configuration.list" ? ["containers"] : {
        pid: "containers", locale: body.kuraConfiguration.locale, writable: false,
        properties: [
          { id: "ints", type: "INTEGER", present: true, redacted: false, value: ["-3", "4"] },
          { id: "empty", type: "STRING", present: true, redacted: false, value: [] },
          { id: "unset", type: "STRING", present: false, redacted: false, value: null },
          { id: "secrets", type: "PASSWORD", present: true, redacted: true, redaction: "SECRET" },
        ],
      } });
    await route.fulfill({ status: 202, json: { id, state: "QUEUED" } });
  });
  await page.goto("/gateways");
  await page.getByRole("button", { name: "Container fixture container-gateway" }).click();
  const panel = page.getByRole("region", { name: /Kura (配置浏览|configuration browser)/ });
  await panel.getByRole("button", { name: "列出 Kura 配置" }).click();
  await panel.getByRole("button", { name: "读取配置详情" }).click();
  await expect(panel.getByText("[]", { exact: true })).toBeVisible();
  await expect(panel.getByText("未设置", { exact: true })).toBeVisible();
  await expect(panel.getByText("已脱敏", { exact: true })).toBeVisible();
  await page.getByRole("dialog").getByRole("combobox", { name: "语言 / Language" }).selectOption("en-US");
  await expect(panel.getByText("[]", { exact: true })).toBeVisible();
  await expect(panel.getByText("Not set", { exact: true })).toBeVisible();
  await expect(panel.getByText("Redacted", { exact: true })).toBeVisible();
  await expect(panel.getByRole("button", { name: /Submit configuration update/ })).toHaveCount(0);
  await page.screenshot({ path: "../../target/component-applicability-20261107/container-browser-en.png", fullPage: true });
});
