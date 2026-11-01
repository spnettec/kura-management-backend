import { test, expect } from "@playwright/test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { resolveLocale, translatedCode, translator } from "../src/i18n/core";
import { messages, type MessageKey } from "../src/i18n/messages";

test("locale selection honors a valid preference, browser priority and unsupported fallback", () => {
  expect(resolveLocale("zh-CN", ["en-US"])).toBe("zh-CN");
  expect(resolveLocale("en-US", ["zh-CN"])).toBe("en-US");
  expect(resolveLocale(null, ["fr-FR", "en-GB", "zh-TW"])).toBe("en-US");
  expect(resolveLocale("invalid", ["zh-TW", "en-US"])).toBe("zh-CN");
  expect(resolveLocale(null, ["de-DE"])).toBe("zh-CN");
  expect(resolveLocale(null, [])).toBe("zh-CN");
});

test("every message has two translations with matching interpolation fields", () => {
  const placeholders = (value: string) =>
    [...value.matchAll(/\{(\w+)\}/g)].map((match) => match[1]).sort();
  for (const [key, pair] of Object.entries(messages)) {
    expect(pair[0].trim(), `${key} Chinese`).not.toBe("");
    expect(pair[1].trim(), `${key} English`).not.toBe("");
    expect(placeholders(pair[0]), key).toEqual(placeholders(pair[1]));
  }
  expect(translator("en-US")("pagination.page", { page: 3 })).toBe(
    "Up to 100 items per page · Page 3",
  );
  expect(translator("zh-CN")("network.countdown", { seconds: 42 })).toContain(
    "42 秒",
  );
});

test("the authored HTTP contract's errors, roles, task states and operations all have translations", () => {
  const schema = JSON.parse(
    readFileSync(
      resolve(import.meta.dirname, "../../../docs/openapi-v1.json"),
      "utf8",
    ),
  ).components.schemas;
  const groups = {
    error: schema.Task.properties.error.enum.filter(
      (code: unknown) => code !== null,
    ),
    state: schema.Task.properties.state.enum,
    operation: schema.Task.properties.operation.enum,
    role: schema.Session.properties.role.enum,
  };
  for (const [prefix, codes] of Object.entries(groups))
    for (const code of codes as (string | number)[]) {
      expect(
        Object.hasOwn(messages, `${prefix}.${code}`),
        `${prefix}.${code}`,
      ).toBe(true);
      for (const locale of ["zh-CN", "en-US"] as const)
        expect(translator(locale)(`${prefix}.${code}` as MessageKey)).not.toBe(
          "",
        );
    }
  expect(translatedCode(translator("en-US"), "error", "FUTURE_CODE")).toBe(
    messages["error.unknown"][1],
  );
  expect(translatedCode(translator("zh-CN"), "state", "FUTURE_STATE")).toBe(
    "FUTURE_STATE",
  );
});
