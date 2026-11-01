import { test, expect } from "@playwright/test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

test("Parallels Ubuntu 26.04 reports real interfaces with observation-only permissions", async ({ page }) => {
  test.setTimeout(60000);
  const errors: string[] = [];
  page.on("pageerror", (e) => errors.push(e.message));
  await page.goto("/login");
  await page.getByLabel("账号", { exact: true }).fill("admin");
  await page.getByLabel("密码", { exact: true }).fill(
    readFileSync(resolve(import.meta.dirname, "../../../.phase1-dev/admin.password"), "utf8"),
  );
  await page.getByRole("button", { name: /登\s*录/ }).click();
  await page.getByRole("button", { name: "Parallels Ubuntu 26.04 独立只读验证 parallels-2604-gateway" }).click();
  const panel = page.getByRole("region", { name: "有线网络管理" });
  await panel.getByRole("button", { name: "读取网络状态" }).click();
  await expect(panel.getByText("仅观测，尚未授权接口接管", { exact: false })).toBeVisible();
  await expect(panel.getByText(/^已确认版本/)).toBeVisible();
  await expect(panel.getByText("eth0", { exact: true })).toBeVisible();
  await expect(panel.getByText("eth1", { exact: true })).toBeVisible();
  await expect(panel.getByRole("button", { name: "应用候选配置" })).toHaveCount(0);
  await page.screenshot({ path: `../../target/${process.env.EVIDENCE_PREFIX ? `${process.env.EVIDENCE_PREFIX}-` : ""}parallels-network-observe.png`, fullPage: true });
  expect(errors).toEqual([]);
});
