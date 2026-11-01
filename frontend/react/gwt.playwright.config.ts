import { defineConfig } from "@playwright/test";
const runId = process.env.KURA_GWT_RUN_ID;
if (runId && !/^[a-z0-9][a-z0-9-]{0,47}$/.test(runId)) throw new Error("Invalid KURA_GWT_RUN_ID");
export default defineConfig({
  testDir: "gwt-tests",
  workers: 1,
  fullyParallel: false,
  timeout: 45000,
  expect: { timeout: 10000 },
  use: {
    baseURL: "http://127.0.0.1:18080",
    viewport: { width: 1440, height: 1100 },
    locale: "en-US",
    launchOptions: { executablePath: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" },
  },
  reporter: [["list"], ["json", { outputFile: `../../target/gwt-network-${runId ? `${runId}-` : ""}browser.json` }]],
});
