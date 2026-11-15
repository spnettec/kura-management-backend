import { test, expect } from "@playwright/test";
const base = process.env.KURA_GWT_CONFIGURATION_FIXTURE_URL;
test.skip(!base, "Requires the independent production-widget/fake-provider GWT fixture");
for (const locale of ["zh", "en"] as const) {
  test(`GWT conditional editor messages and fresh saves (${locale}; fake provider)`, async ({ page }) => {
    await page.goto(`${base}/?locale=${locale}`);
    const field = page.getByRole("textbox");
    await expect(field).toHaveValue("initial");
    const apply = async () => {
      await field.press("Tab");
      await page.getByRole("button", { name: locale === "en" ? /^.*Apply$/ : /^.*应用$/ }).click();
      await page.getByRole("button", { name: locale === "en" ? "Yes" : "是", exact: true }).click();
    };
    await field.fill("stale-local-edit");
    await page.getByRole("button", { name: "Remote update", exact: true }).click();
    await apply();
    await expect(page.getByText(locale === "en"
      ? "This configuration has changed. Reload it before applying your edits."
      : "配置已被修改。请重新加载后再应用编辑。", { exact: true })).toBeVisible();
    await expect(page.getByText("attempts=1 accepted=0", { exact: true })).toBeVisible();
    await expect(field).toHaveValue("stale-local-edit");
    await page.screenshot({ path: `../../target/gwt-concurrent-configuration-20261107/browser/conflict-${locale}.png` });
    await page.getByRole("button", { name: "Reload configuration", exact: true }).click();
    await expect(field).toHaveValue("remote-value");
    await field.fill("first-fresh-save"); await apply();
    await expect(page.getByText("attempts=2 accepted=1", { exact: true })).toBeVisible();
    // The normal listener reload runs after the production widget's two-second defer.
    await page.waitForTimeout(2500);
    await field.fill("second-fresh-save"); await apply();
    await expect(page.getByText("attempts=3 accepted=2", { exact: true })).toBeVisible();
    await page.waitForTimeout(2500);
    await page.getByRole("button", { name: "Next result unknown", exact: true }).click();
    await field.fill("unknown-submission"); await apply();
    await expect(page.getByText(locale === "en"
      ? "The result of this update is unknown. Do not apply it again; inspect the current configuration and recovery state."
      : "此次更新结果未知。请勿再次提交；请检查当前配置与恢复状态。", { exact: true })).toBeVisible();
    await page.waitForTimeout(2300);
    await expect(page.getByText("attempts=4 accepted=3", { exact: true })).toBeVisible();
    await page.screenshot({ path: `../../target/gwt-concurrent-configuration-20261107/browser/unknown-${locale}.png` });
    // An explicit stale retry also cannot add another accepted write.
    await apply();
    await expect(page.getByText("attempts=5 accepted=3", { exact: true })).toBeVisible();
  });
}
