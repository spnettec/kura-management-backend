import { expect, test, type Page } from "@playwright/test";

async function open(page: Page, mustChange: boolean) {
  let rotated = 0;
  await page.route("**/api/v1/session", route => route.fulfill({ json: {
    id: "11111111-1111-4111-8111-111111111111", username: "admin",
    tenantId: "default", role: "admin", csrf: "fixture",
    mustChangePassword: mustChange,
  }}));
  await page.route("**/api/v1/gateways?*", route => route.fulfill({ json: { items: [] } }));
  await page.route("**/api/v1/events", () => {});
  await page.route("**/api/v1/auth/password", async route => {
    const body = route.request().postDataJSON();
    rotated++;
    if (body.currentPassword !== undefined || body.newPassword.length < 12)
      return route.fulfill({ status: 400, json: { error: { code: "INVALID_ARGUMENT" } } });
    mustChange = false;
    return route.fulfill({ json: { ok: true } });
  });
  // The mocked session decides the view directly: /login renders the gate
  // when a session is already present.
  await page.goto("/login");
  return { rotated: () => rotated };
}

test("seeded admin must rotate the initial password before using the console", async ({ page }) => {
  const fixture = await open(page, true);
  const dialog = page.getByRole("heading", { name: "设置管理员密码" });
  await expect(dialog).toBeVisible();
  // mismatch guard
  await expect(page.getByLabel("当前密码", { exact: true })).toHaveCount(0);
  await page.getByLabel("新密码（至少 12 个字符）", { exact: true }).fill("rotate-me-2026!");
  await page.getByLabel("确认新密码", { exact: true }).fill("different-value!");
  await page.getByRole("button", { name: "设置密码并继续" }).click();
  await expect(page.getByText("两次输入的新密码不一致。")).toBeVisible();
  // success path
  await page.getByLabel("确认新密码", { exact: true }).fill("rotate-me-2026!");
  await page.getByRole("button", { name: "设置密码并继续" }).click();
  await expect(page.getByRole("heading", { name: /网关/ }).first()).toBeVisible();
  expect(fixture.rotated()).toBe(1);

});

test("rotated admins go straight to the console", async ({ page }) => {
  await open(page, false);
  await expect(page.getByRole("heading", { name: /网关/ }).first()).toBeVisible();
  await expect(page.getByRole("heading", { name: "设置管理员密码" })).toHaveCount(0);
});
