import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { mergeConfig } from "vite";
import base from "./vite.config.ts";
// Isolated browser contracts use an existing test leaf, never a CA private key.
const fixtures = resolve(import.meta.dirname, "../../crates/console/tests/fixtures/tls-live");
export default mergeConfig(base, { server: { host: "127.0.0.1", port: 28453, strictPort: true,
  https: { cert: readFileSync(resolve(fixtures, "server-one.pem")), key: readFileSync(resolve(fixtures, "server-one.key")) } } });
