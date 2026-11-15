import { defineConfig } from "@playwright/test";
export default defineConfig({
  testDir:"tests",testMatch:"gateway-certificate.spec.ts",workers:1,
  outputDir:process.env.YOFC_ROTATION_BROWSER_OUTPUT??"../../target/gateway-certificate-browser/output",
  use:{baseURL:"https://localhost:28453",ignoreHTTPSErrors:true,viewport:{width:1440,height:1100},launchOptions:{executablePath:"/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"}},
  webServer:{command:process.env.YOFC_ROTATION_REVIEW_BASELINE_COMMAND??"npm exec vite -- --config vite.csr.config.ts",url:"https://localhost:28453/gateways",ignoreHTTPSErrors:true,reuseExistingServer:false},
  reporter:[["list"],["json",{outputFile:process.env.YOFC_ROTATION_BROWSER_REPORT??"../../target/gateway-certificate-browser/report.json"}]],
});
