import { defineConfig } from '@playwright/test';
export default defineConfig({
  testDir:'tests',testMatch:'initial-setup-defaults.spec.ts',workers:1,fullyParallel:false,
  use:{locale:'en-US',baseURL:'http://127.0.0.1:18475',
    launchOptions:{executablePath:'/Applications/Google Chrome.app/Contents/MacOS/Google Chrome'}},
  webServer:{command:'npx vite --host 127.0.0.1 --port 18475 --strictPort',url:'http://127.0.0.1:18475',reuseExistingServer:false},
  outputDir:process.env.YOFC_SETUP_OUTPUT || '../../target/setup-default-contracts',
  reporter:[['list'],['json',{outputFile:process.env.YOFC_SETUP_REPORT || '../../target/setup-default-contracts.json'}]],
});
