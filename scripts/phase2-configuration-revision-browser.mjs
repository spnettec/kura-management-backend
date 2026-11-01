// Local acceptance only: fresh fixture's existing development account.
import { chromium } from "../frontend/react/node_modules/playwright/index.mjs";
import { writeFile } from "node:fs/promises";
import assert from "node:assert/strict";

const [url, output] = process.argv.slice(2);
const parsed = new URL(url);
assert.equal(parsed.hostname, "127.0.0.1");
assert.equal(parsed.protocol, "http:");
assert.ok(output && !output.includes(".."));
const browser = await chromium.launch({ executablePath: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome", headless: true });
try {
  const page = await browser.newPage({ locale: "en-US", viewport: { width: 1440, height: 980 } });
  await page.goto(new URL('/admin/auth', url).href);
  await page.getByPlaceholder("Enter username").fill("admin");
  await page.getByPlaceholder("Enter password").fill("admin");
  await page.locator("button").filter({ hasText: /^Login$/ }).click();
  await page.waitForURL("**/admin/console");
  await page.waitForFunction(() => {
    const visible = document.body.innerText;
    return /admin/.test(visible) && /Device Info|设备信息/.test(visible)
      && /OpenJDK 64-Bit Server VM/.test(visible);
  });
  await page.screenshot({ path: output + ".png", fullPage: true });
  await writeFile(output + ".json", JSON.stringify({ status: "passed", checks: 4, url: page.url(), originalConsoleLogin: true, deviceStatusVisible: true, fixtureOnly: true }, null, 2) + "\n", { flag: "wx" });
} finally { await browser.close(); }
