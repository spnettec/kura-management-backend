import { defineConfig } from "@playwright/test";
export default defineConfig({
  testDir: "tests", testMatch: "tls-activation.spec.ts", workers: 1,
  fullyParallel: false,
  use: { baseURL: process.env.YOFC_TLS_BROWSER_URL || "http://127.0.0.1:18610", locale: "zh-CN",
    viewport: { width: 1440, height: 1100 }, ignoreHTTPSErrors: true,
    launchOptions: { executablePath: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" } },
  outputDir: process.env.YOFC_TLS_BROWSER_OUTPUT || "../../target/tls-activation-browser",
  reporter: [["list"], ["json", { outputFile: process.env.YOFC_TLS_BROWSER_REPORT || "../../target/tls-activation-browser.json" }]],
});
