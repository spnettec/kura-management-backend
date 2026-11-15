import { defineConfig } from "@playwright/test";
export default defineConfig({
  testDir: "tests", testMatch: process.env.YOFC_INITIAL_SETUP_LANGUAGE === "en" ? "initial-configuration-en.spec.ts" : "initial-configuration.spec.ts", workers: 1, fullyParallel: false,
  use: { baseURL: process.env.YOFC_INITIAL_SETUP_URL || "https://localhost:18464", locale: "zh-CN",
    viewport: { width: 1440, height: 1100 }, ignoreHTTPSErrors: true,
    launchOptions: { executablePath: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" } },
  outputDir: process.env.YOFC_INITIAL_SETUP_OUTPUT || "../../target/initial-configuration-browser",
  reporter: [["list"], ["json", { outputFile: process.env.YOFC_INITIAL_SETUP_REPORT || "../../target/initial-configuration-browser.json" }]],
});
