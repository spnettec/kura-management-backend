import { test, expect } from "@playwright/test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
test("real isolated Linux candidate and explicit rollback from the browser", async ({
  page,
}) => {
  test.setTimeout(60000);
  const errors: string[] = [];
  page.on("pageerror", (e) => errors.push(e.message));
  await page.goto("/login");
  await page.getByLabel("账号", { exact: true }).fill("admin");
  await page
    .getByLabel("密码", { exact: true })
    .fill(
      readFileSync(
        resolve(import.meta.dirname, "../../../.phase1-dev/admin.password"),
        "utf8",
      ),
    );
  await page.getByRole("button", { name: /登\s*录/ }).click();
  await page
    .getByRole("button", { name: "网络事务隔离验证 network-gateway" })
    .click();
  const panel = page.getByRole("region", { name: "有线网络管理" });
  await panel.getByRole("button", { name: "读取网络状态" }).click();
  await expect(
    panel.getByText("已限定接管接口", { exact: false }),
  ).toBeVisible();
  await panel.getByLabel("接口名称").fill("yofc-test0");
  await panel.getByLabel("IPv4 DHCP 客户端").click();
  await panel
    .getByLabel("静态 IPv4 地址 / 前缀（逗号分隔）")
    .fill("192.0.2.8/24");
  await panel.getByRole("button", { name: "应用候选配置" }).click();
  await expect(
    panel.getByText("等待新管理连接确认", { exact: false }),
  ).toBeVisible({ timeout: 35000 });
  await page.screenshot({
    path: "../../target/phase2-network-candidate.png",
    fullPage: true,
  });
  await panel.getByRole("button", { name: "立即回滚" }).click();
  await expect(panel.getByText("ROLLED_BACK", { exact: true })).toBeVisible({
    timeout: 35000,
  });
  await expect(
    panel.getByRole("button", { name: "应用候选配置" }),
  ).toBeEnabled();
  expect(errors).toEqual([]);
});
