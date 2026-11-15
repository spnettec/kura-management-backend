import { defineConfig } from "@playwright/test";
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { resolve } from "node:path";
const dev = resolve(import.meta.dirname, "../../.phase1-dev");
const tlsCert = process.env.KURA_BROWSER_TLS_CERT || `${dev}/server.pem`;
const pubkey = execFileSync("openssl", [
  "x509",
  "-in",
  tlsCert,
  "-pubkey",
  "-noout",
]);
const der = execFileSync("openssl", ["pkey", "-pubin", "-outform", "DER"], {
  input: pubkey,
});
const spki = createHash("sha256").update(der).digest("base64");
const liveProfiles: Record<string, string> = {
  "firewall-stream-loss": "**/firewall-stream-loss-live.spec.ts",
  "firewall-linux": "**/firewall-linux-live.spec.ts",
  "firewall-host": "**/firewall-host-live.spec.ts",
  "ordinary-handoff-cancel": "**/ordinary-handoff-cancel-live.spec.ts",
  "system-control": "**/system-control-live.spec.ts",
  "handoff-outcome": "**/handoff-outcome-live.spec.ts",
  "ordinary-handoff": "**/ordinary-handoff-live.spec.ts",
  "boot-outcome": "**/boot-outcome-live.spec.ts",
  "ordinary-boot": "**/ordinary-boot-live.spec.ts",
  "registration": "**/gateway-registration-live.spec.ts",
  "initial-setup": "**/initial-setup-live.spec.ts",
  network: "**/network-write.spec.ts",
  parallels: "**/parallels-observe.spec.ts",
  takeover: "**/takeover-live.spec.ts",
  configuration: "**/configuration-live.spec.ts",
  "configuration-update": "**/configuration-update-live.spec.ts",
  "outcome-snapshot": "**/configuration-outcome-snapshot-live.spec.ts",
  "package-center": "**/package-center-live.spec.ts",
  "transfer-revocation": "**/transfer-revocation-live.spec.ts",
};
const liveProfile = process.env.KURA_BROWSER_LIVE_PROFILE;
if (liveProfile && !liveProfiles[liveProfile]) {
  throw new Error(`KURA_BROWSER_LIVE_PROFILE must be one of ${Object.keys(liveProfiles).join(", ")}`);
}
export default defineConfig({
  testDir: "tests",
  // Live fixtures have distinct lifecycle/preconditions and are explicitly selected after the local gate.
  ...(liveProfile
    ? { testMatch: liveProfiles[liveProfile] }
    : { testIgnore: Object.values(liveProfiles) }),
  workers: 1,
  fullyParallel: false,
  use: {
    locale: "zh-CN",
    baseURL: process.env.KURA_BROWSER_BASE_URL || "https://localhost:18443",
    viewport: { width: 1440, height: 980 },
    ignoreHTTPSErrors: !!process.env.KURA_BROWSER_IGNORE_TLS_ERRORS,
    launchOptions: {
      executablePath:
        "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
      args: process.env.KURA_BROWSER_IGNORE_TLS_ERRORS
        ? ["--ignore-certificate-errors"]
        : [`--ignore-certificate-errors-spki-list=${spki}`],
    },
  },
  reporter: [
    ["list"],
    ["json", { outputFile: process.env.KURA_BROWSER_REPORT_JSON || "../../target/phase1-browser.json" }],
  ],
});
