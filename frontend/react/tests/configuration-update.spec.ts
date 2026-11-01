import { expect, test, type Page } from "@playwright/test";
import { configuration, configurationPatch } from "../src/features/configuration/model";

const revision = "a".repeat(64);
const view = () => ({ pid: "test.pid", locale: "zh", writable: true, updateEnabled: true, revision, properties: [
  { id: "counter", name: "counter", type: "LONG", cardinality: 0, present: true, redacted: false, value: "1", editable: true },
  { id: "password", name: "password", type: "PASSWORD", cardinality: 0, present: true, redacted: true, redaction: "SECRET", editable: true },
  { id: "constrained", name: "constrained", type: "STRING", present: true, redacted: false, value: "fixed", editable: false },
] });

test("typed patch preserves precision and password omission, rejects unsupported or ambiguous fields", () => {
  const current = configuration(view(), "test.pid");
  expect(configurationPatch(current, { counter: "9223372036854775807" })).toEqual({ pid: "test.pid", expectedRevision: revision,
    properties: [{ id: "counter", value: { text: "9223372036854775807" } }] });
  expect(() => configurationPatch(current, { constrained: "changed" })).toThrow("PROTOCOL_MISMATCH");
  expect(() => configurationPatch(current, {})).toThrow("INVALID_ARGUMENT");
  const array = configuration({ ...view(), properties: [{ ...view().properties[0], cardinality: 2, value: ["1"] }] }, "test.pid");
  expect(() => configurationPatch(array, { counter: "[9223372036854775807]" })).toThrow("INVALID_ARGUMENT");
  expect(configurationPatch(array, { counter: '["9223372036854775807"]' }).properties[0].value).toEqual({ textArray: { values: ["9223372036854775807"] } });
});

async function fixture(page: Page, role = "admin", capability = true, ambiguous = false) {
  const tasks = new Map<string, object>(); const writes: any[] = [];
  let outcome = "SUCCEEDED";
  await page.route("**/api/v1/session", route => route.fulfill({ json: { id: "write-contract", username: role, tenantId: "fixture", role, csrf: "fixture" } }));
  await page.route("**/api/v1/gateways?*", route => route.fulfill({ json: { items: [{ id: "write-gateway", name: "Write fixture", online: true,
    enabled: true, kuraAvailable: true, capabilities: ["kura.configuration.list", "kura.configuration.get", ...(capability ? ["kura.configuration.update"] : [])], revision: 0 }] } }));
  await page.route("**/api/v1/events", () => {});
  await page.route(/\/api\/v1\/tasks(?:\?.*)?$/, route => route.fulfill({ json: { items: [] } }));
  await page.route("**/api/v1/tasks/*", route => {
    const id = route.request().url().split("/").at(-1)!;
    if (id === "write-task") return route.fulfill({ json: { id, gatewayId: "write-gateway", state: outcome, operation: 12,
      error: outcome === "UNKNOWN" ? "OUTCOME_UNKNOWN" : null,
      result: outcome === "SUCCEEDED" ? { pid: "test.pid", submitted: true, previousRevision: revision } : {} } });
    return route.fulfill({ json: tasks.get(id) });
  });
  await page.route("**/api/v1/gateways/write-gateway/tasks", async route => {
    const body = route.request().postDataJSON();
    if (body.operation === "kura.configuration.update") {
      writes.push(body);
      expect(route.request().headers()["x-csrf-token"]).toBe("fixture");
      return ambiguous ? route.fulfill({ status: 502, json: { error: { code: "INTERNAL" } } })
        : route.fulfill({ status: 202, json: { id: "write-task", state: "QUEUED" } });
    }
    const id = crypto.randomUUID(); tasks.set(id, { id, gatewayId: "write-gateway", state: "SUCCEEDED", operation: body.operation === "kura.configuration.list" ? 2 : 11,
      result: body.operation === "kura.configuration.list" ? ["test.pid"] : view() });
    return route.fulfill({ status: 202, json: { id, state: "QUEUED" } });
  });
  await page.goto("/gateways"); await page.getByRole("button", { name: "Write fixture write-gateway", exact: true }).click();
  const panel = page.getByRole("region", { name: /Kura (配置浏览|configuration browser)/ });
  await panel.getByRole("button", { name: "列出 Kura 配置" }).click();
  await panel.getByRole("button", { name: "读取配置详情" }).click();
  await expect(panel.getByRole("button", { name: "修改配置", exact: true })).toBeVisible();
  return { writes, panel, outcome: (state: string) => { outcome = state; } };
}

test("bilingual editor confirms only selected fields and keeps password unchanged (stubbed tasks)", async ({ page }) => {
  const f = await fixture(page);
  await f.panel.getByRole("button", { name: "修改配置", exact: true }).click();
  const editor = page.getByRole("dialog", { name: /^(修改配置|Edit configuration)$/ });
  await expect(editor.getByLabel("password", { exact: true })).toHaveCount(0);
  await expect(editor.getByRole("checkbox", { name: /constrained/ })).toHaveCount(0);
  await editor.getByRole("checkbox", { name: /counter.*设置新值/ }).check();
  await editor.getByLabel("counter", { exact: true }).fill("9223372036854775807");
  await expect(editor.getByRole("button", { name: "确认并提交" })).toBeDisabled();
  await page.getByRole("dialog", { name: "Write fixture", exact: true }).getByRole("combobox", { name: "语言 / Language" }).selectOption("en-US");
  await expect(editor.getByLabel("counter", { exact: true })).toHaveValue("9223372036854775807");
  await expect(editor.getByText(/Unchecked properties and passwords keep their values/)).toBeVisible();
  await editor.getByRole("checkbox", { name: "I confirm updating 1 properties of test.pid" }).check();
  await editor.getByRole("button", { name: "Confirm and submit" }).click();
  await expect(f.panel.getByText(/Configuration submission completed/)).toBeVisible();
  expect(f.writes).toHaveLength(1);
  expect(f.writes[0].kuraConfigurationUpdate.properties).toEqual([{ id: "counter", value: { text: "9223372036854775807" } }]);
  await f.panel.getByRole("button", { name: "Check submission" }).click();
  expect(f.writes).toHaveLength(1);
});

test("explicit password setting and UNKNOWN query never resubmit (stubbed tasks)", async ({ page }) => {
  const f = await fixture(page); f.outcome("UNKNOWN");
  await f.panel.getByRole("button", { name: "修改配置", exact: true }).click();
  const editor = page.getByRole("dialog", { name: /^(修改配置|Edit configuration)$/ });
  await editor.getByRole("checkbox", { name: /password.*设置新值/ }).check();
  await editor.getByLabel("password", { exact: true }).fill("new-test-secret");
  await editor.getByRole("checkbox", { name: "我确认修改 test.pid 的 1 项属性" }).check();
  await editor.getByRole("button", { name: "确认并提交" }).click();
  await expect(f.panel.getByText(/提交结果未知/)).toBeVisible();
  await expect(f.panel.getByRole("button", { name: "修改配置", exact: true })).toBeDisabled();
  expect(f.writes[0].kuraConfigurationUpdate.properties).toEqual([{ id: "password", value: { text: "new-test-secret" } }]);
  expect(await f.panel.innerText()).not.toContain("new-test-secret");
  await page.getByRole("dialog", { name: "Write fixture", exact: true }).getByRole("combobox", { name: "语言 / Language" }).selectOption("en-US");
  await expect(f.panel.getByText(/Submission outcome is unknown/)).toBeVisible();
  f.outcome("SUCCEEDED"); await f.panel.getByRole("button", { name: "Check submission" }).click();
  await expect(f.panel.getByText(/Configuration submission completed/)).toBeVisible();
  expect(f.writes).toHaveLength(1);
});

for (const [role, capability] of [["operator", true], ["admin", false]] as const) {
  test(`configuration write gate for ${role} capability=${capability} (stubbed tasks)`, async ({ page }) => {
    const f = await fixture(page, role, capability);
    await expect(f.panel.getByRole("button", { name: "修改配置", exact: true })).toBeDisabled();
    expect(f.writes).toHaveLength(0);
  });
}

test("ambiguous HTTP admission blocks another submission (stubbed tasks)", async ({ page }) => {
  const f = await fixture(page, "admin", true, true);
  await f.panel.getByRole("button", { name: "修改配置", exact: true }).click();
  const editor = page.getByRole("dialog", { name: /^(修改配置|Edit configuration)$/ });
  await editor.getByRole("checkbox", { name: /counter.*设置新值/ }).check();
  await editor.getByRole("checkbox", { name: "我确认修改 test.pid 的 1 项属性" }).check();
  await editor.getByRole("button", { name: "确认并提交" }).click();
  await expect(f.panel.getByText(/提交结果未知/)).toBeVisible();
  await expect(f.panel.getByRole("button", { name: "修改配置", exact: true })).toBeDisabled();
  expect(f.writes).toHaveLength(1);
});
