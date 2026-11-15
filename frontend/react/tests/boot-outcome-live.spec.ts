import { expect, test } from "@playwright/test";
import { readFileSync, writeFileSync } from "node:fs";
const b = process.env.KURA_BOOT_EVIDENCE_ROOT;
if (!b) throw new Error("explicit new boot outcome evidence root required");
const host = JSON.parse(readFileSync(`${b}/host-stack.json`, "utf8"));
const orb = JSON.parse(readFileSync(`${b}/orb.json`, "utf8"));

test("actual original boot task survives domain process exit and is proved by the production agent", async ({ page }) => {
  test.setTimeout(240000);
  const requests: { operation: string }[] = [];
  page.on("request", r => { if (r.method() === "POST" && r.url().endsWith(`/gateways/${orb.gateway}/tasks`)) requests.push(r.postDataJSON()); });
  await page.goto("/login");
  await page.getByLabel("账号", { exact: true }).fill(host.testAdmin);
  await page.getByLabel("密码", { exact: true }).fill(readFileSync(`${host.fixture}/${host.testAdmin}.password`, "utf8"));
  await page.getByRole("button", { name: /登\s*录/ }).click();
  await page.getByRole("button", { name: `本轮原命令启动策略核定 ${orb.gateway}`, exact: true }).click();
  const panel = page.getByRole("region", { name: /Kura (启动策略|boot policy)/ });
  const resume = process.env.KURA_BOOT_RESUME_PREPARE_TASK;
  if (resume) {
    const session = await (await page.request.get("/api/v1/session")).json();
    await page.evaluate(({ key, id }) => sessionStorage.setItem(key, id), { key: `yofc.boot.prepare:${session.id}:${orb.gateway}`, id: resume });
  }
  await panel.getByRole("button", { name: "读取策略状态" }).click();
  if (!resume) {
    await expect(panel).toContainText("UNCONFIGURED");
    await panel.getByRole("button", { name: "准备并审阅本机策略" }).click();
  }
  await expect(panel.getByText(`enable ${orb.ordinaryUnit}`, { exact: true })).toBeVisible({ timeout: 45000 });
  const prepareId = resume ?? await panel.locator("p").filter({ hasText: "操作任务" }).locator("code").innerText();
  await panel.getByRole("checkbox").check();
  const accepted = page.waitForResponse(r => r.status() === 202 && r.url().endsWith(`/gateways/${orb.gateway}/tasks`) && r.request().postDataJSON().operation === "ordinary.boot.execute");
  await panel.getByRole("button", { name: "确认启动策略", exact: true }).click();
  const id: string = (await (await accepted).json()).id;
  expect(id).not.toBe(prepareId);
  const task = async () => (await page.request.get(`/api/v1/tasks/${id}`)).json();
  await expect.poll(async () => (await task()).state, { timeout: 60000 }).toBe("UNKNOWN");
  await expect(panel.getByRole("button", { name: "准备并审阅本机策略" })).toBeDisabled();
  writeFileSync(`${b}/awaiting-production.json`, JSON.stringify({ taskId: id, prepareId, state: "UNKNOWN", executeRequests: requests.filter(r => r.operation === "ordinary.boot.execute").length }, null, 2), { mode: 0o600 });
  // The host observes the exited test fixture, then starts the production ELF once.
  // This browser only reads the original task. It never resubmits the write.
  await expect.poll(async () => (await task()).state, { timeout: 150000 }).toBe("SUCCEEDED");
  const resolved = await task();
  expect(resolved.result.reconciliation.source).toBe("ordinary.boot.journal.v1");
  expect(resolved.result.reconciliation.commandId).toBe(id);
  expect(resolved.result.bootVerified).toBe(false);
  expect(resolved.result.kuraRecoveryVerified).toBe(false);
  expect(requests.filter(r => r.operation === "ordinary.boot.execute")).toHaveLength(1);
  expect(requests.filter(r => r.operation === "ordinary.boot.prepare")).toHaveLength(resume ? 0 : 1);
  await page.goto("/commands");
  await page.getByRole("button", { name: new RegExp(`确认启动策略 ${id.slice(0, 8)}`) }).click();
  const dialog = page.getByRole("dialog");
  await expect(dialog.getByText("已依据原命令的启动策略决定核定", { exact: true })).toBeVisible();
  await expect(dialog.getByText(/不代表实际开机或完整 Kura 业务恢复通过/)).toBeVisible();
  await page.screenshot({ path: `${b}/center-outcome-zh.png`, fullPage: true });
  await dialog.getByRole("combobox", { name: "语言 / Language" }).selectOption("en-US");
  await expect(dialog.getByText("Resolved from the original command's boot policy decision", { exact: true })).toBeVisible();
  await page.screenshot({ path: `${b}/center-outcome-en.png`, fullPage: true });
  writeFileSync(`${b}/center-browser-live.json`, JSON.stringify({ status: "passed", gateway: orb.gateway, prepareId, executeTaskId: id, unknownObserved: true, originalCommandProved: true, browserExecuteRequests: 1, replacementPrepareRequests: 0, actualHttpsGrpcAgent: true, bilingual: true, bootVerified: false, kuraRecoveryVerified: false, processLossScope: "test-only transport process exits after native decision before execution receipt; production ELF performs only receipt query" }, null, 2));
});
