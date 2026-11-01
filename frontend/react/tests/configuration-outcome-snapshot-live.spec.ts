import { expect, test, type Page } from "@playwright/test";
import { existsSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { readdir, readFile } from "node:fs/promises";
import { join, resolve } from "node:path";

type OutcomeRecord = {
  source: string; commandId: string; operation: number; pid: string;
  previousRevision: string; submitted: boolean; paramsSha256: string;
};

async function outcomeRecords(fixture: string): Promise<OutcomeRecord[]> {
  const directory = resolve(fixture, "kura-home/data/management-outcomes");
  if (!existsSync(directory)) return [];
  const names = (await readdir(directory)).filter(name => name.endsWith(".json")).sort();
  return Promise.all(names.map(async name => JSON.parse(await readFile(join(directory, name), "utf8"))));
}

async function taskJson(page: Page, id: string): Promise<any> {
  return page.evaluate(async taskId => await (await fetch(`/api/v1/tasks/${taskId}`, { credentials: "same-origin" })).json(), id);
}

async function waitTask(page: Page, accepted: { id: string } | undefined, state: "SUCCEEDED" | "FAILED") {
  expect(accepted?.id, "accepted task id").toBeTruthy();
  return expect.poll(() => taskJson(page, accepted!.id), { timeout: 35000 }).toMatchObject({ id: accepted!.id, state });
}

async function readEditor(page: Page, localeButton: string, readButton: string) {
  const panel = page.getByRole("region", { name: /Kura (配置浏览|configuration browser)/ });
  await panel.getByRole("button", { name: "列出 Kura 配置" }).click();
  const pid = panel.getByRole("combobox", { name: /配置 PID|Configuration PID/ });
  await pid.fill("org.eclipse.kura.management.revisionFixture");
  await pid.press("ArrowDown"); await pid.press("Enter");
  await panel.getByRole("button", { name: readButton }).click();
  await expect(panel.getByRole("button", { name: localeButton, exact: true })).toBeVisible({ timeout: 15000 });
  return { panel, editor: page.getByRole("dialog", { name: /^(修改配置|Edit configuration)$/ }) };
}

test("real bilingual writes persist outcomes and explicit snapshots stay scoped", async ({ page }) => {
  test.setTimeout(180000);
  const fixture = resolve(import.meta.dirname, "../../../.phase1-dev/configuration-outcome-snapshot-idea-20261101");
  const startedPath = resolve(fixture, "outcome-snapshot-live-started");
  const resumed = existsSync(startedPath);
  if (!resumed) writeFileSync(startedPath, "one-use live flow\n", { flag: "wx" });
  const currentStage = resumed ? readFileSync(resolve(fixture, "stage"), "utf8").trim() : "initial";
  const savedProofPath = resolve(fixture, "snapshot-save-proof.json");
  const savedProof = existsSync(savedProofPath) ? JSON.parse(readFileSync(savedProofPath, "utf8")) : null;
  let records: OutcomeRecord[] = [];
  expect(JSON.parse(readFileSync(resolve(fixture, "initial-proof.json"), "utf8"))).toMatchObject({
    status: "passed", checks: 9, readOnlyObserver: true, snapshotSecretAbsent: true, snapshotMode600: true,
  });
  const errors: string[] = []; page.on("pageerror", error => errors.push(error.message));
  const acceptedTasks: { id: string }[] = [];
  page.on("response", async response => {
    if (/\/api\/v1\/gateways\/[^/]+\/tasks$/.test(new URL(response.url()).pathname) && response.request().method() === "POST") {
      acceptedTasks.push(await response.json());
    }
  });

  await page.goto("/login");
  await page.getByLabel("账号", { exact: true }).fill("admin");
  await page.getByLabel("密码", { exact: true }).fill(readFileSync(resolve(fixture, "../admin.password"), "utf8"));
  await page.getByRole("button", { name: /登\s*录/ }).click();
  await page.getByRole("button", { name: "Outcome snapshot IDEA fixture configuration-outcome-snapshot-idea-20261101", exact: true }).click();

  // First write: keep the synthetic password while setting count to LONG_MAX.
  let flow = await readEditor(page, "修改配置", "读取配置详情");
  if (currentStage === "initial") {
  await flow.panel.getByRole("button", { name: "修改配置", exact: true }).click();
  await flow.editor.getByRole("checkbox", { name: /count.*设置新值/ }).check();
  await flow.editor.getByLabel("count", { exact: true }).fill("9223372036854775807");
  await flow.editor.getByRole("checkbox", { name: /我确认修改.*的 1 项属性/ }).check();
  await flow.editor.getByRole("button", { name: "确认并提交" }).click();
  await expect(flow.panel.getByText(/配置提交完成/)).toBeVisible({ timeout: 35000 });
  writeFileSync(resolve(fixture, "stage"), "keep\n");
  await expect.poll(() => existsSync(resolve(fixture, "keep-proof.json")), { timeout: 15000 }).toBe(true);
  expect(JSON.parse(readFileSync(resolve(fixture, "keep-proof.json"), "utf8"))).toMatchObject({
    status: "passed", countMatchesExpected: true, passwordMatchesExpected: true, cmChangeCount: 8,
  });
  records = await outcomeRecords(fixture);
  expect(records).toHaveLength(1);
  expect(records[0]).toMatchObject({ source: "configuration.journal.v1", operation: 12, pid: "org.eclipse.kura.management.revisionFixture", submitted: true });
  expect(Object.keys(records[0]).sort()).toEqual(["commandId", "operation", "paramsSha256", "pid", "previousRevision", "source", "submitted"]);
  expect(statSync(resolve(fixture, "kura-home/data/management-outcomes", `${records[0].commandId}.json`)).mode & 0o777).toBe(0o600);
  await page.screenshot({ path: "../../target/outcome-snapshot-live-zh-20261101.png", fullPage: true });
  }

  if (currentStage !== "set") {
  // Second write: explicitly set the password in English and prove typed PASSWORD handling.
  const outer = page.getByRole("dialog", { name: "Outcome snapshot IDEA fixture", exact: true });
  await outer.getByRole("combobox", { name: "语言 / Language" }).selectOption("en-US");
  const englishPanel = page.getByRole("region", { name: /Kura (configuration browser)/ });
  await englishPanel.getByRole("button", { name: "Read configuration details" }).click();
  await englishPanel.getByRole("button", { name: "Edit configuration", exact: true }).click();
  flow = { panel: englishPanel, editor: page.getByRole("dialog", { name: /^(修改配置|Edit configuration)$/ }) };
  await flow.editor.getByRole("checkbox", { name: /password.*Set new value/ }).check();
  await flow.editor.getByLabel("password", { exact: true }).fill("fixture-remote-set");
  await flow.editor.getByRole("checkbox", { name: /I confirm updating 1 properties/ }).check();
  await flow.editor.getByRole("button", { name: "Confirm and submit" }).click();
  await expect(flow.panel.getByText(/Configuration submission completed/)).toBeVisible({ timeout: 35000 });
  writeFileSync(resolve(fixture, "stage"), "set\n");
  await expect.poll(() => existsSync(resolve(fixture, "set-proof.json")), { timeout: 15000 }).toBe(true);
  expect(JSON.parse(readFileSync(resolve(fixture, "set-proof.json"), "utf8"))).toMatchObject({
    status: "passed", countMatchesExpected: true, passwordMatchesExpected: true, cmChangeCount: 9,
  });
  records = await outcomeRecords(fixture);
  expect(records).toHaveLength(2);
  expect(new Set(records.map(record => record.commandId)).size).toBe(2);
  for (const record of records) {
    expect(record).toMatchObject({ source: "configuration.journal.v1", operation: 12, pid: "org.eclipse.kura.management.revisionFixture", submitted: true });
    expect(JSON.stringify(record)).not.toMatch(/fixture-(initial|remote-set)|9223372036854775807/);
  }
  await page.screenshot({ path: "../../target/outcome-snapshot-live-en-20261101.png", fullPage: true });
  }
  expect(JSON.parse(readFileSync(resolve(fixture, "set-proof.json"), "utf8"))).toMatchObject({
    status: "passed", countMatchesExpected: true, passwordMatchesExpected: true, cmChangeCount: 9,
  });
  records = await outcomeRecords(fixture);
  expect(records).toHaveLength(2);
  expect(new Set(records.map(record => record.commandId)).size).toBe(2);

  // Explicit save: current latest is 0. The one-use UI confirmation is asserted by default browser tests.
  const outer = page.getByRole("dialog", { name: "Outcome snapshot IDEA fixture", exact: true });
  await outer.getByRole("combobox", { name: "语言 / Language" }).selectOption("en-US");
  const snapshots = page.getByRole("region", { name: /Kura (快照|snapshots)/ });
  const snapshotRow = (id: string) => snapshots.locator("li").filter({ has: page.getByText(id, { exact: true }) });
  await page.screenshot({ path: "../../target/outcome-snapshot-live-before-snapshot.png", fullPage: true });
  await expect(snapshots).toHaveCount(1);
  await expect(snapshots.getByRole("button", { name: "Read snapshot list" })).toBeVisible();
  await snapshots.getByRole("button", { name: "Read snapshot list" }).click();
  await expect(snapshotRow("0")).toBeVisible({ timeout: 35000 });
  if (currentStage === "set" && !savedProof) {
    await snapshots.getByRole("button", { name: "Save current snapshot" }).click();
    const saveDialog = page.getByRole("dialog", { name: "Save current snapshot" });
    await expect(saveDialog.getByText(/does not change the running configuration or rewind system network/)).toBeVisible();
    await saveDialog.getByRole("checkbox", { name: /does not roll back system network/ }).check();
    await saveDialog.getByRole("button", { name: "Confirm and submit" }).click();
    await expect(saveDialog).toBeHidden();
    await waitTask(page, acceptedTasks.at(-1), "SUCCEEDED");
    saved = await taskJson(page, acceptedTasks.at(-1)!.id);
  } else {
    saved = savedProof ? await taskJson(page, savedProof.taskId) : null;
  }
  expect(saved).toMatchObject({ state: "SUCCEEDED", result: { saved: true, networkRewound: false } });
  expect(Number(saved.result.snapshotId)).toBeGreaterThan(0);
  const saveProof = { taskId: saved.id, result: saved.result };
  if (savedProof) expect(savedProof).toMatchObject(saveProof);
  else writeFileSync(savedProofPath, JSON.stringify(saveProof, null, 2) + "\n");

  // A stale optimistic version is rejected without retrying the original successful action.
  const stale = await page.evaluate(async expected => {
    const session = await (await fetch("/api/v1/session", { credentials: "same-origin" })).json();
    const response = await fetch("/api/v1/gateways/configuration-outcome-snapshot-idea-20261101/tasks", {
      method: "POST", credentials: "same-origin",
      headers: { "Content-Type": "application/json", "x-csrf-token": session.csrf },
      body: JSON.stringify({ operation: "kura.snapshot.save", idempotencyKey: crypto.randomUUID(), kuraSnapshotSave: { expectedLatestId: expected } }),
    });
    return { status: response.status, body: await response.json().catch(() => null) };
  }, "0");
  expect(stale.status).toBe(202);
  expect(stale.body?.id).toBeTruthy();
  await expect.poll(() => taskJson(page, stale.body.id), { timeout: 35000 }).toMatchObject({ state: "FAILED", error: "CONFLICT" });

  // Roll back to the initial snapshot; system network is explicitly not rewound.
  await snapshots.getByRole("button", { name: "Read snapshot list" }).click();
  const initialRow = snapshotRow("0");
  await expect(initialRow).toBeVisible({ timeout: 35000 });
  await initialRow.getByRole("button", { name: "Roll back to this snapshot" }).click();
  const rollbackDialog = page.getByRole("dialog", { name: "Roll back to this snapshot" });
  await expect(rollbackDialog.getByText(/missing components may return to defaults/)).toBeVisible();
  await rollbackDialog.getByRole("checkbox", { name: /missing components may return to defaults/ }).check();
  await rollbackDialog.getByRole("button", { name: "Confirm and submit" }).click();
  await expect(rollbackDialog).toBeHidden({ timeout: 35000 });
  await waitTask(page, acceptedTasks.at(-1), "SUCCEEDED");
  const rolledBack = await taskJson(page, acceptedTasks.at(-1)!.id);
  expect(rolledBack).toMatchObject({ state: "SUCCEEDED", result: { rolledBack: true, snapshotId: "0", networkRewound: false } });
  writeFileSync(resolve(fixture, "outcome-snapshot-live-completed"), "two writes, save, stale conflict, rollback\n", { flag: "wx" });
  expect(errors).toEqual([]);
});
