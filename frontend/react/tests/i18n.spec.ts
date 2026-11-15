import { test, expect, type Page } from "@playwright/test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

test.use({ locale: "en-US" });
const password = () =>
  readFileSync(
    resolve(process.env.KURA_BROWSER_FIXTURE || resolve(import.meta.dirname, "../../../.phase1-dev"), "admin.password"),
    "utf8",
  );
const switcher = (page: Page) =>
  page.getByRole("combobox", { name: "语言 / Language" });

async function login(page: Page) {
  await page.goto("/login");
  await page.getByLabel("Username", { exact: true }).fill("admin");
  await page.getByLabel("Password", { exact: true }).fill(password());
  await page.getByRole("button", { name: "Sign in", exact: true }).click();
  await expect(
    page.getByRole("heading", { name: "Gateways", exact: true }),
  ).toBeVisible();
}

test("English detection, Chinese switching, existing errors and live login/tasks survive navigation", async ({
  page,
}) => {
  const errors: string[] = [];
  page.on("pageerror", (error) => errors.push(error.message));
  await page.goto("/login");
  await expect(page.locator("html")).toHaveAttribute("lang", "en-US");
  await expect(page).toHaveTitle("YOFC · Gateway Management");
  await page.getByRole("button", { name: "Sign in", exact: true }).click();
  await expect(
    page.getByText("Enter your username", { exact: true }),
  ).toBeVisible();
  await switcher(page).selectOption("zh-CN");
  await expect(page.getByText("请输入账号", { exact: true })).toBeVisible();
  await page.getByLabel("账号", { exact: true }).fill("admin");
  await page.getByLabel("密码", { exact: true }).fill("incorrect-i18n");
  await page.getByRole("button", { name: /登\s*录/ }).click();
  await expect(page.getByRole("alert")).toContainText("账号或密码错误");
  await switcher(page).selectOption("en-US");
  await expect(page.getByRole("alert")).toContainText(
    "Incorrect username or password.",
  );
  await expect(page.getByLabel("Username", { exact: true })).toHaveValue(
    "admin",
  );
  await page.getByLabel("Password", { exact: true }).fill(password());
  await page.getByRole("button", { name: "Sign in", exact: true }).click();
  await expect(
    page.getByRole("heading", { name: "Gateways", exact: true }),
  ).toBeVisible();
  await expect(page.getByText("Live updates connected").first()).toBeVisible();
  await expect(page.getByText("Administrator", { exact: true })).toBeVisible();
  await page.screenshot({
    path: `../../target/${process.env.EVIDENCE_PREFIX || "i18n"}-gateways-en.png`,
    fullPage: true,
  });
  await page
    .getByRole("row")
    .filter({ hasText: "dev-gateway" })
    .getByRole("button", { name: "Query status", exact: true })
    .click();
  await page.getByRole("link", { name: "Tasks", exact: false }).click();
  await expect(
    page.getByRole("heading", { name: "Tasks", exact: true }),
  ).toBeVisible();
  await expect(
    page.getByText("Completed", { exact: true }).first(),
  ).toBeVisible();
  await expect(
    page.getByRole("columnheader", { name: "Submitted at", exact: true }),
  ).toBeVisible();
  await page.reload();
  await expect(
    page.getByRole("heading", { name: "Tasks", exact: true }),
  ).toBeVisible();
  await switcher(page).selectOption("zh-CN");
  await expect(
    page.getByRole("heading", { name: "任务记录", exact: true }),
  ).toBeVisible();
  await page.reload();
  await expect(
    page.getByRole("heading", { name: "任务记录", exact: true }),
  ).toBeVisible();
  await page.getByRole("button", { name: /退\s*出/ }).click();
  await expect(
    page.getByRole("heading", { name: "登录工作空间" }),
  ).toBeVisible();
  expect(errors).toEqual([]);
});

test("live network observation switches language in its drawer without losing results", async ({
  page,
}) => {
  await login(page);
  await page.getByRole("button", { name: "本机开发网关 dev-gateway" }).click();
  const drawer = page.getByRole("dialog");
  await drawer.getByRole("button", { name: "Read network status" }).click();
  await expect(
    drawer.getByText("Observation only; interface takeover is not authorized", {
      exact: false,
    }),
  ).toBeVisible();
  await drawer
    .getByRole("combobox", { name: "语言 / Language" })
    .selectOption("zh-CN");
  await expect(
    drawer.getByRole("region", { name: "有线网络管理" }),
  ).toBeVisible();
  await expect(
    drawer.getByText("仅观测，尚未授权接口接管", { exact: false }),
  ).toBeVisible();
  await expect(
    drawer.getByRole("button", { name: "应用候选配置" }),
  ).toHaveCount(0);
  await page.screenshot({
    path: `../../target/${process.env.EVIDENCE_PREFIX || "i18n"}-network-zh.png`,
    fullPage: true,
  });
});

test("bilingual network form, consent and transaction UI contract (stubbed task results)", async ({
  page,
}) => {
  const tasks: object[] = [];
  await page.route("**/api/v1/session", (route) =>
    route.fulfill({
      json: {
        id: "ui-contract",
        username: "admin",
        tenantId: "fixture",
        role: "admin",
        csrf: "fixture",
      },
    }),
  );
  await page.route("**/api/v1/gateways?*", (route) =>
    route.fulfill({
      json: {
        items: [
          {
            id: "dev-gateway",
            name: "本机开发网关",
            online: true,
            kuraAvailable: false,
            lastSeen: "2026-11-01T00:00:00Z",
            agentVersion: "fixture",
            revision: 1,
            capabilities: ["agent.status", "network.inspect"],
          },
        ],
      },
    }),
  );
  // This contract has no live event stream; live SSE is covered by the other tests.
  await page.route("**/api/v1/events", () => {});

  await page.route(/\/api\/v1\/tasks(?:\?.*)?$/, (route) =>
    route.fulfill({ json: { items: tasks } }),
  );
  await page.route("**/api/v1/gateways/dev-gateway/tasks", async (route) => {
    const body = route.request().postDataJSON();
    const id = crypto.randomUUID();
    let result: object;
    if (body.operation === "network.inspect")
      result = {
        revision: 1,
        sourceDigest: "a".repeat(64),
        sources: [],
        interfaces: [],
        transaction: null,
        writeEnabled: true,
      };
    else if (body.operation === "network.takeover.plan")
      result = {
        eligible: true,
        interface: body.networkTakeover.interface,
        revision: 1,
        sourceDigest: "a".repeat(64),
        planDigest: "b".repeat(64),
        sourcePath: "etc/netplan/40-existing.yaml",
        sourceMode: "0640",
        preservedInterface: { dhcp4: false },
        preservedOtherInterfaces: ["other0"],
      };
    else if (body.operation === "network.takeover.prepare")
      result = {
        id,
        state: "WAIT_CONFIRM",
        revision: 1,
        deadlineUnixMs: Date.now() + 120000,
        confirmationToken: "fixture",
        diagnostic: null,
      };
    else {
      await route.fulfill({
        status: 409,
        json: { error: { code: "CONFLICT" } },
      });
      return;
    }
    tasks.unshift({ id, state: "SUCCEEDED", result });
    await route.fulfill({ status: 202, json: { id, state: "QUEUED" } });
  });
  await page.goto("/gateways");
  await page.getByRole("button", { name: "本机开发网关 dev-gateway" }).click();
  const drawer = page.getByRole("dialog");
  await drawer.getByRole("button", { name: "Read network status" }).click();
  await drawer.getByLabel("Interface name", { exact: true }).fill("fixture0");
  await drawer.getByRole("button", { name: "Add route" }).click();
  await drawer
    .getByLabel("Route destination", { exact: true })
    .fill("192.0.2.0/24");
  await drawer
    .getByRole("combobox", { name: "语言 / Language" })
    .selectOption("zh-CN");
  await expect(drawer.getByLabel("接口名称", { exact: true })).toHaveValue(
    "fixture0",
  );
  await expect(drawer.getByLabel("路由目标", { exact: true })).toHaveValue(
    "192.0.2.0/24",
  );
  await drawer.getByLabel("待接管接口", { exact: true }).fill("fixture0");
  await drawer.getByRole("button", { name: "预览接管计划" }).click();
  await expect(
    drawer.getByRole("button", { name: "应用接管候选" }),
  ).toBeDisabled();
  await drawer
    .getByRole("combobox", { name: "语言 / Language" })
    .selectOption("en-US");
  await expect(
    drawer.getByRole("button", { name: "Apply takeover candidate" }),
  ).toBeDisabled();
  await drawer.getByRole("checkbox", { name: /I have reviewed/ }).check();
  await drawer
    .getByRole("button", { name: "Apply takeover candidate" })
    .click();
  await expect(
    drawer.getByText("Awaiting confirmation", { exact: true }),
  ).toBeVisible();
  await expect(
    drawer.getByRole("button", { name: "Confirm connectivity" }),
  ).toBeVisible();
  await drawer.getByRole("button", { name: "Refresh transaction" }).click();
  await expect(drawer.getByRole("alert").first()).toContainText(
    "The request conflicts",
  );
  await drawer
    .getByRole("combobox", { name: "语言 / Language" })
    .selectOption("zh-CN");
  await expect(drawer.getByRole("alert").first()).toContainText(
    "请求与当前状态",
  );
  await expect(drawer.getByText("等待确认", { exact: true })).toBeVisible();
  await page.screenshot({
    path: `../../target/${process.env.EVIDENCE_PREFIX || "i18n"}-transaction-zh.png`,
    fullPage: true,
  });
});

test("language selection works when storage is blocked and fits a mobile login", async ({
  page,
}) => {
  await page.addInitScript(() =>
    Object.defineProperty(window, "localStorage", {
      get() {
        throw new Error("blocked for test");
      },
    }),
  );
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto("/login");
  await expect(
    page.getByRole("heading", { name: "Sign in to your workspace" }),
  ).toBeVisible();
  await switcher(page).selectOption("zh-CN");
  await expect(
    page.getByRole("heading", { name: "登录工作空间" }),
  ).toBeVisible();
  expect(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= innerWidth,
    ),
  ).toBe(true);
  await switcher(page).selectOption("en-US");
  await page.screenshot({
    path: `../../target/${process.env.EVIDENCE_PREFIX || "i18n"}-login-mobile-en.png`,
    fullPage: true,
  });
});
