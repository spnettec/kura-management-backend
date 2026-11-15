import { test, expect } from "@playwright/test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

test("takeover review and explicit consent UI contract (stubbed task results)", async ({ page }) => {
  const tasks: object[] = [];
  await page.route("**/api/v1/tasks", async route => route.fulfill({ json: { items: tasks } }));
  await page.route("**/api/v1/gateways/dev-gateway/tasks", async route => {
    const body = route.request().postDataJSON();
    const id = crypto.randomUUID();
    let result: object;
    if (body.operation === "network.inspect") result = { revision: 1, sourceDigest: "a".repeat(64), sources: [], interfaces: [], transaction: null, writeEnabled: true };
    else if (body.operation === "network.takeover.plan") result = { eligible: true, interface: body.networkTakeover.interface, revision: 1, sourceDigest: "a".repeat(64), planDigest: "b".repeat(64), sourcePath: "etc/netplan/40-existing.yaml", sourceMode: "0640", preservedInterface: { dhcp4: false, addresses: ["192.0.2.90/24"] }, preservedOtherInterfaces: ["other0"] };
    else {
      expect(body.operation).toBe("network.takeover.prepare");
      expect(body.networkTakeover.planDigest).toBe("b".repeat(64));
      result = { id, state: "WAIT_CONFIRM", revision: 1, deadlineUnixMs: Date.now() + 120000, confirmationToken: "fixture", diagnostic: null };
    }
    tasks.unshift({ id, state: "SUCCEEDED", result });
    await route.fulfill({ status: 202, json: { id, state: "QUEUED" } });
  });
  await page.goto("/login");
  await page.getByLabel("账号", { exact: true }).fill("admin");
  await page.getByLabel("密码", { exact: true }).fill(readFileSync(resolve(process.env.KURA_BROWSER_FIXTURE || resolve(import.meta.dirname, "../../../.phase1-dev"), "admin.password"), "utf8"));
  await page.getByRole("button", { name: /登\s*录/ }).click();
  await page.getByRole("button", { name: /本机开发网关 dev-gateway/ }).click();
  const panel = page.getByRole("region", { name: "有线网络管理" });
  await panel.getByRole("button", { name: "读取网络状态" }).click();
  await panel.getByLabel("待接管接口").fill("fixture0");
  await panel.getByRole("button", { name: "预览接管计划" }).click();
  await expect(panel.getByText(/40-existing.yaml/)).toBeVisible();
  const apply = panel.getByRole("button", { name: "应用接管候选" });
  await expect(apply).toBeDisabled();
  await panel.getByRole("checkbox", { name: /我已核对接口与来源/ }).check();
  await expect(apply).toBeEnabled();
  await apply.click();
  await expect(panel.getByText("WAIT_CONFIRM", { exact: true })).toBeVisible();
});
