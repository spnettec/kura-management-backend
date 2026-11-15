import { defineConfig } from "@playwright/test";
export default defineConfig({
  testDir: "tests", testMatch: "server-csr.spec.ts", workers: 1,
  outputDir: process.env.YOFC_CSR_BROWSER_OUTPUT ?? "../../target/server-csr-browser/output",
  use: { baseURL: "https://localhost:28453", ignoreHTTPSErrors: true, viewport: { width: 1440, height: 1100 },
    launchOptions: { executablePath: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" } },
  webServer: { command: "npm exec vite -- --config vite.csr.config.ts", url: "https://localhost:28453/setup", ignoreHTTPSErrors: true, reuseExistingServer: false },
  reporter: [["list"], ["json", { outputFile: process.env.YOFC_CSR_BROWSER_REPORT ?? "../../target/server-csr-browser/report.json" }]],
});
