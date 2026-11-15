import {defineConfig} from "@playwright/test";
import {resolve} from "node:path";
const evidence=process.env.YOFC_IMPORTED_EVIDENCE_DIR||resolve(import.meta.dirname,"../../target/imported-ui");
export default defineConfig({
  testDir:"tests",testMatch:"imported-firewall.spec.ts",workers:1,fullyParallel:false,
  outputDir:resolve(evidence,"artifacts"),
  use:{baseURL:"http://127.0.0.1:51151",viewport:{width:1440,height:1080},locale:"zh-CN",
    launchOptions:{executablePath:"/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"}},
  webServer:{command:"npm run dev -- --port 51151 --strictPort",url:"http://127.0.0.1:51151",reuseExistingServer:false},
  reporter:[["list"],["json",{outputFile:resolve(evidence,"report.json")}]],
});
