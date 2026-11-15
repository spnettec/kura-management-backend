import { defineConfig } from '@playwright/test';
export default defineConfig({
  testDir:'tests',testMatch:'host-installer.spec.ts',workers:1,fullyParallel:false,
  use:{baseURL:'http://127.0.0.1:18472',viewport:{width:1280,height:1300},
    launchOptions:{executablePath:'/Applications/Google Chrome.app/Contents/MacOS/Google Chrome'}},
  webServer:{command:'python3 -m http.server 18472 --bind 127.0.0.1 --directory ../../packaging/cockpit',url:'http://127.0.0.1:18472',reuseExistingServer:false},
  outputDir:process.env.YOFC_INSTALLER_OUTPUT || '../../target/host-installer-browser',
  reporter:[['list'],['json',{outputFile:process.env.YOFC_INSTALLER_REPORT || '../../target/host-installer-browser.json'}]],
});
