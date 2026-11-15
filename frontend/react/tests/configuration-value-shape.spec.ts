import { expect, test } from "@playwright/test";
import { configuration, configurationPatch, isArrayProperty, type Configuration, type Property } from "../src/features/configuration/model";

const property = (cardinality: number, valueShape: Property["valueShape"], value: Property["value"]): Property => ({
  id: "field", type: "STRING", cardinality, valueShape, value, present: true, redacted: false, editable: true,
});
const view = (field: Property): Configuration => ({ pid: "fixture", locale: "en", writable: true,
  updateEnabled: true, revision: "a".repeat(64), properties: [field] });

test("legacy +/-1 scalar and native one-element containers submit distinct wire shapes", () => {
  for (const cardinality of [0, 1, -1]) {
    const field = property(cardinality, "SCALAR", " spaced ");
    expect(isArrayProperty(field)).toBe(false);
    expect(configurationPatch(configuration(view(field), "fixture"), { field: " spaced " }).properties)
      .toEqual([{ id: "field", value: { text: " spaced " } }]);
  }
  for (const [cardinality, shape] of [[1, "ARRAY"], [-1, "COLLECTION"]] as const) {
    const field = property(cardinality, shape, [" spaced "]);
    expect(isArrayProperty(field)).toBe(true);
    expect(configurationPatch(configuration(view(field), "fixture"), { field: '[" spaced "]' }).properties)
      .toEqual([{ id: "field", value: { textArray: { values: [" spaced "] } } }]);
    expect(() => configurationPatch(view(field), { field: '["a", "b"]' })).toThrow();
  }
});

test("secret and absent values retain explicit shape without exposing values", () => {
  for (const [shape, array] of [["SCALAR", false], ["COLLECTION", true]] as const) {
    const field: Property = { id: "field", type: "PASSWORD", cardinality: -1, valueShape: shape,
      present: false, redacted: true, redaction: "SECRET", editable: true };
    const decoded = configuration(view(field), "fixture");
    expect(isArrayProperty(decoded.properties[0])).toBe(array);
    const result = configurationPatch(decoded, { field: array ? '[""]' : "" });
    expect(result.properties[0].value).toEqual(array ? { textArray: { values: [""] } } : { text: "" });
    expect(decoded.properties[0]).not.toHaveProperty("value");
  }
});

test("impossible representation metadata fails before a write can be assembled", () => {
  for (const field of [property(0, "ARRAY", []), property(1, "COLLECTION", []),
    property(-1, "ARRAY", []), property(2, "SCALAR", "x"), property(1, "SCALAR", ["x"])]) {
    expect(() => configuration(view(field), "fixture")).toThrow();
  }
});


test("bilingual configuration editor keeps scalar and one-element container fields distinct (stubbed tasks)", async ({ page }) => {
  const tasks = new Map<string, object>(); const writes: unknown[] = [];
  const fields: Property[] = [
    { ...property(1, "SCALAR", "old"), id: "legacy", name: "legacy" },
    { ...property(1, "ARRAY", ["one"]), id: "singleArray", name: "singleArray" },
    { ...property(-1, "COLLECTION", ["one"]), id: "singleList", name: "singleList" },
  ];
  await page.route("**/api/v1/session", route => route.fulfill({ json: { id: "shape", username: "admin", role: "admin", tenantId: "fixture", csrf: "fixture" } }));
  await page.route("**/api/v1/gateways?*", route => route.fulfill({ json: { items: [{ id: "shape", name: "Shape fixture", online: true,
    enabled: true, kuraAvailable: true, revision: 0, capabilities: ["kura.configuration.list", "kura.configuration.get", "kura.configuration.update"] }] } }));
  await page.route("**/api/v1/events", () => {});
  await page.route(/\/api\/v1\/tasks(?:\?.*)?$/, route => route.fulfill({ json: { items: [] } }));
  await page.route("**/api/v1/tasks/*", route => route.fulfill({ json: tasks.get(route.request().url().split("/").at(-1)!) }));
  await page.route("**/api/v1/gateways/shape/tasks", async route => {
    const body = route.request().postDataJSON(); const id = crypto.randomUUID();
    const write = body.operation === "kura.configuration.update";
    if (write) writes.push(body.kuraConfigurationUpdate.properties);
    tasks.set(id, { id, gatewayId: "shape", state: "SUCCEEDED", operation: write ? 12 : 11,
      result: write ? { pid: "fixture", submitted: true, previousRevision: "a".repeat(64) }
        : body.operation === "kura.configuration.list" ? ["fixture"]
        : { ...view(fields[0]), locale: body.kuraConfiguration.locale, properties: fields } });
    await route.fulfill({ status: 202, json: { id, state: "QUEUED" } });
  });
  await page.goto("/gateways");
  await page.getByRole("button", { name: "Shape fixture shape", exact: true }).click();
  const panel = page.getByRole("region", { name: /Kura (配置浏览|configuration browser)/ });
  await panel.getByRole("button", { name: "列出 Kura 配置" }).click();
  await panel.getByRole("button", { name: "读取配置详情" }).click();
  await panel.getByRole("button", { name: "修改配置", exact: true }).click();
  const editor = page.getByRole("dialog", { name: /^(修改配置|Edit configuration)$/ });
  for (const id of ["legacy", "singleArray", "singleList"]) await editor.getByRole("checkbox", { name: new RegExp(id+".*设置新值") }).check();
  await expect(editor.getByLabel("legacy", { exact: true })).toHaveValue("old");
  await expect(editor.getByLabel("singleArray", { exact: true })).toHaveValue('["one"]');
  await expect(editor.getByLabel("singleList", { exact: true })).toHaveValue('["one"]');
  await editor.getByLabel("legacy", { exact: true }).fill(" spaced ");
  await editor.getByLabel("singleArray", { exact: true }).fill('["array"]');
  await editor.getByLabel("singleList", { exact: true }).fill('["list"]');
  await page.getByRole("dialog", { name: "Shape fixture", exact: true }).getByRole("combobox", { name: "语言 / Language" }).selectOption("en-US");
  await expect(editor.getByLabel("legacy", { exact: true })).toHaveValue(" spaced ");
  await editor.getByRole("checkbox", { name: "I confirm updating 3 properties of fixture" }).check();
  await editor.getByRole("button", { name: "Confirm and submit" }).click();
  await expect(panel.getByText(/Configuration submission completed/)).toBeVisible();
  expect(writes).toEqual([[{ id: "legacy", value: { text: " spaced " } },
    { id: "singleArray", value: { textArray: { values: ["array"] } } },
    { id: "singleList", value: { textArray: { values: ["list"] } } }]]);
  await page.screenshot({ path: "../../target/configuration-scalar-edit-20261107/react-value-shape-en.png", fullPage: true });
});
