import { defineConfig } from '@playwright/test';
export default defineConfig({
  testDir:'tests',testMatch:'native-package.spec.ts',workers:1,
  use:{baseURL:'http://127.0.0.1:28472',viewport:{width:1280,height:1350},
    launchOptions:{executablePath:'/Applications/Google Chrome.app/Contents/MacOS/Google Chrome'}},
  webServer:{command:'python3 -m http.server 28472 --bind 127.0.0.1 --directory ../../packaging/cockpit',url:'http://127.0.0.1:28472',reuseExistingServer:false},
  outputDir:process.env.YOFC_NATIVE_OUTPUT || '../../target/native-package-browser',
  reporter:[['list'],['json',{outputFile:process.env.YOFC_NATIVE_REPORT || '../../target/native-package-browser.json'}]],
});
