import { expect, test, type Page } from "@playwright/test";
import { configuration, configurationPids } from "../src/features/configuration/model";

const view = (locale = "zh") => ({
  pid: "test.pid", locale, name: locale === "zh" ? "测试组件" : "Test component", description: "fixture metadata",
  properties: [
    { id: "counter", name: "counter", type: "LONG", present: true, redacted: false, value: "9223372036854775807" },
    { id: "password", name: "password", type: "PASSWORD", present: true, redacted: true, redaction: "SECRET" },
    { id: "empty", type: "STRING", present: true, redacted: false, value: "" },
    { id: "missing", type: "STRING", present: false, redacted: false, value: null },
    { id: "unsupported", type: "UNKNOWN", present: true, redacted: true, redaction: "UNSUPPORTED" },
    { id: "markup", type: "STRING", present: true, redacted: false, value: "<script>unsafe()</script>" },
  ],
});

test("configuration projection rejects unsafe values, mismatched targets and contradictory redaction", () => {
  expect(configuration(view(), "test.pid").properties[0].value).toBe("9223372036854775807");
  expect(() => configuration(view(), "other.pid")).toThrow("PROTOCOL_MISMATCH");
  expect(() => configuration({ ...view(), properties: [{ ...view().properties[1], value: "must-not-render" }] }, "test.pid")).toThrow("PROTOCOL_MISMATCH");
  expect(() => configuration({ ...view(), properties: [{ ...view().properties[0], value: 9223372036854775807 }] }, "test.pid")).toThrow("PROTOCOL_MISMATCH");
  expect(configurationPids(["b", "a", "a"])).toEqual(["a", "b"]);
  expect(() => configurationPids([{}])).toThrow("PROTOCOL_MISMATCH");
});

async function fixture(page: Page, role = "admin", getSupported = true) {
  const tasks = new Map<string, object>();
  const requests: Record<string, unknown>[] = [];
  let failure: string | null = null;
  await page.route("**/api/v1/session", route => route.fulfill({ json: {
    id: "configuration-contract", username: role, tenantId: "fixture", role, csrf: "fixture",
  } }));
  await page.route("**/api/v1/gateways?*", route => route.fulfill({ json: { items: [{
    id: "config-gateway", name: "Configuration fixture", online: true, enabled: true, kuraAvailable: true,
    capabilities: ["agent.status", "kura.configuration.list", ...(getSupported ? ["kura.configuration.get"] : [])],
    revision: 0, lastSeen: null, agentVersion: "fixture", uptimeSeconds: 1,
  }] } }));
  await page.route("**/api/v1/events", () => {});
  await page.route(/\/api\/v1\/tasks(?:\?.*)?$/, route => route.fulfill({ json: { items: [] } }));
  await page.route("**/api/v1/tasks/*", route => {
    const id = route.request().url().split("/").at(-1)!;
    return route.fulfill({ json: tasks.get(id) });
  });
  await page.route("**/api/v1/gateways/config-gateway/tasks", async route => {
    const body = route.request().postDataJSON(); requests.push(body);
    const id = crypto.randomUUID();
    tasks.set(id, { id, gatewayId: "config-gateway", state: failure ? "FAILED" : "SUCCEEDED", error: failure,
      operation: body.operation === "kura.configuration.list" ? 2 : 11,
      result: body.operation === "kura.configuration.list" ? ["test.pid"] : view(body.kuraConfiguration.locale) });
    await route.fulfill({ status: 202, json: { id, state: "QUEUED" } });
  });
  await page.goto("/gateways");
  await page.getByRole("button", { name: "Configuration fixture config-gateway" }).click();
  return { requests, fail: (code: string) => { failure = code; } };
}

test("bilingual configuration browser preserves values, masks secrets and translates existing errors (stubbed tasks)", async ({ page }) => {
  const errors: string[] = []; page.on("pageerror", e => errors.push(e.message));
  const f = await fixture(page);
  const panel = page.getByRole("region", { name: /Kura (配置浏览|configuration browser)/ });
  await panel.getByRole("button", { name: "列出 Kura 配置" }).click();
  await panel.getByRole("button", { name: "读取配置详情" }).click();
  await expect(panel.getByText("测试组件", { exact: true })).toBeVisible();
  await expect(panel.getByText('"9223372036854775807"', { exact: true })).toBeVisible();
  await expect(panel.getByText("已脱敏", { exact: true })).toBeVisible();
  await expect(panel.getByText("未设置", { exact: true })).toBeVisible();
  await expect(panel.getByText('""', { exact: true })).toBeVisible();
  expect(await panel.locator("script").count()).toBe(0);
  await page.getByRole("dialog").getByRole("combobox", { name: "语言 / Language" }).selectOption("en-US");
  await expect(panel.getByText("Redacted", { exact: true })).toBeVisible();
  await expect(panel.getByText('"9223372036854775807"', { exact: true })).toBeVisible();
  expect(f.requests).toHaveLength(2);
  await panel.getByRole("button", { name: "Read configuration details" }).click();
  await expect(panel.getByText("Test component", { exact: true })).toBeVisible();
  expect(f.requests.at(-1)?.kuraConfiguration).toEqual({ pid: "test.pid", locale: "en" });
  await page.screenshot({ path: "../../target/configuration-browser-en-20261101.png", fullPage: true });
  f.fail("KURA_UNAVAILABLE");
  await panel.getByRole("button", { name: "Read configuration details" }).click();
  await expect(panel.getByRole("alert")).toContainText("The agent remains manageable");
  await expect(panel.getByText("Test component", { exact: true })).toHaveCount(0);
  await page.getByRole("dialog").getByRole("combobox", { name: "语言 / Language" }).selectOption("zh-CN");
  await expect(panel.getByRole("alert")).toContainText("agent 仍可管理");
  expect(f.requests).toHaveLength(4);
  expect(errors).toEqual([]);
});

test("viewer configuration controls stay disabled (stubbed session)", async ({ page }) => {
  await fixture(page, "viewer");
  await expect(page.getByRole("region", { name: "Kura 配置浏览" }).getByRole("button", { name: "列出 Kura 配置" })).toBeDisabled();
});

test("legacy configuration list stays usable without get capability (stubbed tasks)", async ({ page }) => {
  await fixture(page, "admin", false);
  const panel = page.getByRole("region", { name: "Kura 配置浏览" });
  await panel.getByRole("button", { name: "列出 Kura 配置" }).click();
  await expect(panel.getByRole("combobox", { name: "配置 PID" })).toBeVisible();
  await expect(panel.getByRole("button", { name: "读取配置详情" })).toBeDisabled();
});
