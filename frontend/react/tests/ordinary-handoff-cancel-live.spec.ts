import { expect, test } from "@playwright/test";
import { existsSync, readFileSync, writeFileSync } from "node:fs";

const b = process.env.KURA_CANCEL_LIVE_ROOT;
if (!b) throw new Error("explicit new cancellation evidence root required");
const host = JSON.parse(readFileSync(`${b}/host-stack.json`, "utf8"));
const orb = JSON.parse(readFileSync(`${b}/cancel-live-fixture.json`, "utf8"));
const environment = process.env.KURA_CANCEL_ENVIRONMENT ?? "orb";
const resumed = process.env.KURA_CANCEL_LIVE_RESUME === "1";
const save = (name: string, value: unknown) => writeFileSync(`${b}/${name}.json`, JSON.stringify(value, null, 2), { flag: "wx", mode: 0o600 });

test("actual center cancels expired unissued handoffs without changing the approved Kura parent", async ({ page }) => {
  test.setTimeout(300000);
  expect(orb.state).toBe("approved-parent-ready-for-center-cancel");
  if (resumed) {
    expect(existsSync(`${b}/browser-started.json`)).toBe(true);
    expect(existsSync(`${b}/center-cancel-admitted.json`)).toBe(false);
    expect(existsSync(`${b}/center-prepare-admitted-second.json`)).toBe(false);
    save(`browser-resume-${process.env.KURA_CANCEL_RESUME_ATTEMPT ?? "1"}-started`, { gateway: orb.gateway, originalPrepareOnly: true });
  } else save("browser-started", { gateway: orb.gateway, neverReplayThisEntry: true });
  const requests: { operation: string; idempotencyKey: string }[] = [];
  page.on("request", r => {
    if (r.method() === "POST" && r.url().endsWith(`/gateways/${orb.gateway}/tasks`)) requests.push(r.postDataJSON());
  });
  const task = async (id: string) => (await page.request.get(`/api/v1/tasks/${id}`)).json();
  const panel = page.getByRole("region", { name: /Kura (普通配置交接|ordinary profile handoff)/ });
  await page.goto("/login");
  await page.getByLabel("账号", { exact: true }).fill(host.testAdmin);
  await page.getByLabel("密码", { exact: true }).fill(readFileSync(`${host.fixture}/${host.testAdmin}.password`, "utf8"));
  await page.getByRole("button", { name: /登\s*录/ }).click();
  await page.getByRole("button", { name: `本轮未执行交接计划取消 ${orb.gateway}`, exact: true }).click();
  if (resumed) {
    const session = await (await page.request.get("/api/v1/session")).json();
    expect(typeof session.id).toBe("string");
    const admitted = JSON.parse(readFileSync(`${b}/center-prepare-admitted.json`, "utf8"));
    // Restore only the recorded task ID, as the panel does after a refresh.
    await page.evaluate(({ key, id }) => sessionStorage.setItem(key, id), { key: `yofc.handoff:${session.id}:${orb.gateway}:prepare`, id: admitted.id });
  }
  await panel.getByRole("button", { name: "读取交接状态与原任务" }).click();
  await expect(panel).toContainText(resumed ? "ORDINARY_PREPARED" : "COMPLETED_APPROVED_CHECKS", { timeout: 40000 });

  async function prepare(suffix: string) {
    const accepted = page.waitForResponse(r => r.status() === 202 && r.url().endsWith(`/gateways/${orb.gateway}/tasks`) && r.request().postDataJSON().operation === "recovery.ordinary.prepare");
    await panel.getByRole("button", { name: "准备并审阅普通配置交接" }).click();
    const id: string = (await (await accepted).json()).id;
    save(`center-prepare-admitted${suffix}`, { id });
    await expect.poll(async () => (await task(id)).state, { timeout: 60000 }).toBe("SUCCEEDED");
    const original = await task(id);
    expect(original.result.state).toBe("ORDINARY_PREPARED");
    expect(original.result.unit).toBe(orb.ordinaryUnit);
    expect(await page.evaluate(() => Object.values(sessionStorage).join(" "))).not.toContain(original.result.confirmToken);
    await expect(panel).toContainText(orb.ordinaryUnit);
    return { id, original };
  }
  async function cancel(suffix: string, transactionId: string) {
    await expect(panel.getByRole("checkbox")).not.toBeChecked();
    const accepted = page.waitForResponse(r => r.status() === 202 && r.url().endsWith(`/gateways/${orb.gateway}/tasks`) && r.request().postDataJSON().operation === "recovery.ordinary.cancel");
    await panel.getByRole("button", { name: "取消尚未执行的交接计划" }).click();
    const id: string = (await (await accepted).json()).id;
    save(`center-cancel-admitted${suffix}`, { id, transactionId });
    await expect.poll(async () => (await task(id)).state, { timeout: 60000 }).toBe("SUCCEEDED");
    const done = await task(id);
    expect(done.operation).toBe(29);
    expect(done.result).toMatchObject({ id: transactionId, state: "ORDINARY_CANCELLED", cancelled: true, issued: false, ordinaryUnitHandedOff: false, scope: "administrator-selected-checks", kuraRecoveryVerified: false, normalBootVerified: false });
    await expect(panel).toContainText("ORDINARY_CANCELLED");
    await expect(panel.getByRole("button", { name: "准备并审阅普通配置交接" })).toBeEnabled();
    return { id, done };
  }

  const first = resumed ? await (async () => {
    const { id } = JSON.parse(readFileSync(`${b}/center-prepare-admitted.json`, "utf8"));
    const original = await task(id);
    expect(original.state).toBe("SUCCEEDED");
    expect(original.operation).toBe(21);
    await expect(panel).toContainText(orb.ordinaryUnit);
    return { id, original };
  })() : await prepare("");
  if (!resumed) save("center-plan-ready", { prepareId: first.id, expiresAtUnixMs: first.original.result.expiresAtUnixMs, transactionId: first.original.result.transactionId });
  else expect(JSON.parse(readFileSync(`${b}/center-plan-ready.json`, "utf8")).prepareId).toBe(first.id);
  // A separate root fixture controller waits for real expiry and reopens only
  // this new agent, checking the unchanged Kura PID and systemd invocation.
  await expect.poll(() => existsSync(`${b}/${environment}-expired-parent-reopened.json`), { timeout: 140000 }).toBe(true);
  await expect(panel.getByRole("button", { name: "读取交接状态与原任务" })).toBeEnabled({ timeout: 40000 });
  await panel.getByRole("button", { name: "读取交接状态与原任务" }).click();
  await expect(panel.getByRole("button", { name: "取消尚未执行的交接计划" })).toBeEnabled({ timeout: 40000 });
  await panel.getByRole("checkbox").check();
  await expect(panel.getByRole("button", { name: "确认普通配置交接", exact: true })).toBeDisabled();
  await panel.getByRole("checkbox").uncheck();
  const firstCancelled = await cancel("", first.original.result.transactionId);
  await page.screenshot({ path: `${b}/cancel-live-zh.png`, fullPage: true, animations: "disabled" });
  await page.getByRole("dialog").getByRole("combobox", { name: "语言 / Language" }).selectOption("en-US");
  await expect(panel.getByRole("button", { name: "Prepare and review ordinary handoff" })).toBeEnabled();
  await expect(panel).toContainText("ORDINARY_CANCELLED");
  await page.screenshot({ path: `${b}/cancel-live-en.png`, fullPage: true, animations: "disabled" });
  await page.getByRole("dialog").getByRole("combobox", { name: "语言 / Language" }).selectOption("zh-CN");

  const second = await prepare("-second");
  expect(second.original.result.transactionId).not.toBe(first.original.result.transactionId);
  const secondCancelled = await cancel("-second", second.original.result.transactionId);
  expect((await task(first.id)).result).toEqual(first.original.result);
  expect((await task(firstCancelled.id)).result).toEqual(firstCancelled.done.result);
  expect(requests.filter(v => v.operation === "recovery.ordinary.prepare")).toHaveLength(resumed ? 1 : 2);
  expect(requests.filter(v => v.operation === "recovery.ordinary.cancel")).toHaveLength(2);
  expect(requests.filter(v => v.operation === "recovery.ordinary.execute")).toHaveLength(0);
  save("center-browser-live", { status: "passed", gateway: orb.gateway, firstPrepare: first.id, firstCancel: firstCancelled.id, secondPrepare: second.id, secondCancel: secondCancelled.id, firstTransaction: first.original.result.transactionId, secondTransaction: second.original.result.transactionId, actualHttpsGrpcAgent: true, expiredAgentReopen: true, explicitNewReviews: 2, cancellationRequests: 2, executeRequests: 0, originalTasksRetained: true, bilingual: true, kuraRecoveryVerified: false, normalBootVerified: false });
  await expect.poll(() => existsSync(`${b}/native-post-cancel-proof.json`), { timeout: 90000 }).toBe(true);

  await page.goto(orb.http + "/admin/console");
  await page.getByPlaceholder("输入用户名").fill("admin");
  await page.getByPlaceholder("输入密码").fill("admin");
  await page.locator('button[type="submit"]').filter({ hasText: "登录" }).click();
  await page.getByRole("link", { name: /Configuration revision test fixture/ }).click();
  await expect(page.locator("form").getByRole("textbox").first()).toHaveValue("9");
  await expect(page.locator("body")).toContainText(/登陆名\s*admin/);
  await page.screenshot({ path: `${b}/gwt-after-cancel.png`, fullPage: true, animations: "disabled" });
  save("gwt-after-cancel", { status: "passed", login: "admin/admin", count: 9, parentPid: orb.parentPid });
});
