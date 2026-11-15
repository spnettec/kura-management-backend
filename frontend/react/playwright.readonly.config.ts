import { defineConfig } from "@playwright/test";
export default defineConfig({testDir:"tests",testMatch:"nm-readonly.spec.ts",workers:1,fullyParallel:false,timeout:20000,
use:{baseURL:"http://127.0.0.1:18764",locale:"zh-CN",viewport:{width:1440,height:980},
launchOptions:{executablePath:"/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"}},
webServer:{command:"node node_modules/vite/bin/vite.js --host 127.0.0.1 --port 18764 --strictPort",url:"http://127.0.0.1:18764",reuseExistingServer:false,timeout:30000},
reporter:[["list"],["json",{outputFile:process.env.YOFC_READONLY_REPORT}]],outputDir:process.env.YOFC_READONLY_TEST_RESULTS});
