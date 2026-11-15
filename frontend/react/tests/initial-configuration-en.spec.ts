import { test, expect } from "@playwright/test";
import { readFileSync, mkdirSync, readdirSync } from "node:fs";
import { resolve } from "node:path";

test("English fresh console initializes through UI with current database binding guard", async ({ page }) => {
  test.skip(!process.env.YOFC_INITIAL_SETUP_SCENE, "A distinct unentered scene must be selected explicitly");
  const scene = process.env.YOFC_INITIAL_SETUP_SCENE!;
  if (readdirSync(resolve(scene,"live-center-en/state/setup-operations")).some(name=>/\.(accepted|intent|receipt)\.json$/.test(name)))
    throw new Error("This one-time setup scene is already entered; only read its original operation, never replay initialization");
  const data = JSON.parse(readFileSync(resolve(scene, "live-center-en/browser-input.json"), "utf8"));
  const fixture = resolve(import.meta.dirname, "../../../crates/console/tests/fixtures/tls-live");
  const dispatched: string[] = [];
  page.on("request", async request => {
    if (request.method() !== "POST" || !request.url().endsWith("/api/setup/operations")) return;
    const body = request.postDataJSON();
    const saved = await page.evaluate(() => JSON.parse(localStorage.getItem("yofc.console.initial-setup.v1")!));
    expect(saved.operationId).toBe(body.operationId);
    expect(saved).not.toHaveProperty("password"); expect(saved).not.toHaveProperty("databaseUrl"); expect(saved).not.toHaveProperty("serverKey");
    dispatched.push(body.operationId);
  });
  await page.addInitScript(()=>localStorage.setItem("yofc.console.locale","en-US"));
  await page.goto("/setup");
  await page.getByLabel("Installation credential file (JSON)", { exact: true }).setInputFiles(resolve(scene, "live-center-en/state/setup/installation-credential.json"));
  await page.getByRole("checkbox", { name: "I checked the browser certificate fingerprint against the trusted installation credential.", exact: true }).check();
  await page.getByRole("button", { name: "Unlock initial setup", exact: true }).click();
  for (const [label, value] of [
    ["Console HTTPS address", "https://localhost:18466"], ["Agent channel listen address", "127.0.0.1:18467"],
    ["Database host", data.databaseHost], ["Database port", data.databasePort], ["Database name", data.databaseName],
    ["Database account", data.databaseUser], ["Database password", data.databasePassword],
    ["Organization identifier", data.tenant], ["Administrator account", data.username], ["Administrator password (at least 12 bytes)", data.password],
  ]) await page.getByLabel(label, { exact: true }).fill(value);
  for (const [label, file] of [["Console server certificate file", "server-one.pem"], ["Console server private key file", "server-one.key"], ["Gateway public CA certificate file", "ca-one.pem"]]) {
    const input = page.getByLabel(label, { exact: true }); await expect(input).toBeEnabled(); await input.setInputFiles(resolve(fixture, file));
  }
  const review = page.getByRole("button", { name: "Review this configuration and certificate fingerprints", exact: true }); await expect(review).toBeEnabled(); await review.click();
  await page.getByRole("checkbox", { name: "I reviewed the console address, certificates and organization account, and approve initializing the dedicated empty database.", exact: true }).check();
  await page.getByRole("button", { name: "Initialize the management console", exact: true }).click();
  // The old TLS connection is closed. The page only reads the original, detects
  // configured mode and asks for the administrator chosen above.
  await expect(page.getByRole("heading", { name: "Sign in to your workspace", exact: true })).toBeVisible({ timeout: 15000 });
  await page.getByLabel("Username", { exact: true }).fill(data.username);
  await page.getByLabel("Password", { exact: true }).fill(data.password);
  await page.getByRole("button", { name: /^Sign in$/ }).click();
  await expect(page.getByText("The original initialization has a completion receipt. Sign in with the organization administrator account you configured.", { exact: true })).toBeVisible();
  expect(dispatched).toHaveLength(1);
  const original = await page.request.get(`/api/v1/settings/initial-setup/${dispatched[0]}`);
  expect(original.status()).toBe(200); expect((await original.json()).state).toBe("APPLIED");
  await page.reload();
  await expect(page.getByText("The original initialization has a completion receipt. Sign in with the organization administrator account you configured.", { exact: true })).toBeVisible();
  expect(dispatched).toHaveLength(1);
  const ready = await page.request.get("/health/ready"); expect(ready.status()).toBe(200);
  mkdirSync(resolve(scene, "browser"), { recursive: true });
  await page.screenshot({ path: resolve(scene, "browser/initial-configuration-live-en.png"), fullPage: true });
});
