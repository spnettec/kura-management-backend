import { expect, test, type Page } from "@playwright/test";

async function open(page: Page, role = "admin", capability = true) {
  const writes: any[] = [];
  await page.route("**/api/v1/session", route => route.fulfill({ json: { id: "snapshot-user", username: role, tenantId: "fixture", role, csrf: "fixture" } }));
  await page.route("**/api/v1/gateways?*", route => route.fulfill({ json: { items: [{ id: "snapshot-gateway", name: "Snapshot fixture", online: true,
    enabled: true, kuraAvailable: true, capabilities: ["kura.snapshot.list", ...(capability ? ["kura.snapshot.write"] : [])], revision: 0 }] } }));
  await page.route("**/api/v1/events", () => {});
  await page.route(/\/api\/v1\/tasks(?:\?.*)?$/, route => route.fulfill({ json: { items: [] } }));
  await page.route("**/api/v1/tasks/*", route => route.fulfill({ json: { id: "list-task", gatewayId: "snapshot-gateway", state: "SUCCEEDED", operation: 3, error: null, result: ["4", "2"] } }));
  await page.route("**/api/v1/gateways/snapshot-gateway/tasks", async route => {
    const body = route.request().postDataJSON();
    writes.push(body);
    expect(route.request().headers()["x-csrf-token"]).toBe("fixture");
    return route.fulfill({ status: 202, json: { id: "list-task", state: "QUEUED" } });
  });
  await page.goto("/gateways");
  await page.getByRole("button", { name: "Snapshot fixture snapshot-gateway", exact: true }).click();
  return { writes, panel: page.getByRole("region", { name: /Kura (快照|snapshots)/ }) };
}

test("snapshot panel explains non-selective rollback and submits only after bilingual confirmation", async ({ page }) => {
  const f = await open(page);
  await expect(f.panel.getByText(/不是通用配置编辑器/)).toBeVisible();
  await f.panel.getByRole("button", { name: "读取快照列表" }).click();
  await expect(f.panel.getByText("4", { exact: true })).toBeVisible();
  await f.panel.getByRole("button", { name: "保存当前快照" }).click();
  const dialog = page.getByRole("dialog", { name: "保存当前快照" });
  await expect(dialog.getByRole("button", { name: "确认并提交" })).toBeDisabled();
  await page.getByRole("dialog", { name: "Snapshot fixture", exact: true }).getByRole("combobox", { name: "语言 / Language" }).selectOption("en-US");
  await expect(page.getByRole("dialog", { name: "Save current snapshot" }).getByText(/does not change the running configuration or rewind system network/)).toBeVisible();
  await page.getByRole("checkbox", { name: /does not roll back system network/ }).check();
  await page.getByRole("button", { name: "Confirm and submit" }).click();
  await expect(page.getByRole("dialog", { name: "Save current snapshot" })).toBeHidden();
  expect(f.writes.at(-1)).toMatchObject({ operation: "kura.snapshot.save", kuraSnapshotSave: { expectedLatestId: "4" } });
  await f.panel.getByRole("button", { name: "Read snapshot list" }).click();
  await f.panel.locator("li").filter({ hasText: /^2/ }).getByRole("button", { name: "Roll back to this snapshot" }).click();
  await expect(page.getByRole("dialog", { name: "Roll back to this snapshot" }).locator("p")).toContainText("may return to defaults");
  await page.getByRole("checkbox", { name: /missing components may return to defaults/ }).check();
  await page.getByRole("button", { name: "Confirm and submit" }).click();
  expect(f.writes.at(-1)).toMatchObject({ operation: "kura.snapshot.rollback", kuraSnapshotRollback: { snapshotId: "2", expectedLatestId: "4" } });
});

test("snapshot writes stay hidden without the explicit capability", async ({ page }) => {
  const f = await open(page, "admin", false);
  await expect(f.panel).toHaveCount(0);
  await expect(page.getByRole("button", { name: "保存当前快照" })).toHaveCount(0);
});
