import { messages, type MessageKey } from "./messages";

export type Locale = "zh-CN" | "en-US";
export const localeStorageKey = "yofc.console.locale";
export type Translate = (
  key: MessageKey,
  values?: Record<string, string | number>,
) => string;

export function resolveLocale(
  saved: string | null,
  languages: readonly string[],
): Locale {
  if (saved === "zh-CN" || saved === "en-US") return saved;
  for (const language of languages) {
    if (/^zh(?:-|$)/i.test(language)) return "zh-CN";
    if (/^en(?:-|$)/i.test(language)) return "en-US";
  }
  return "zh-CN";
}

export function translator(locale: Locale): Translate {
  return (key, values = {}) =>
    messages[key][locale === "zh-CN" ? 0 : 1].replace(
      /\{(\w+)\}/g,
      (token, name: string) => String(values[name] ?? token),
    );
}

export function translatedCode(
  t: Translate,
  prefix: "state" | "operation" | "role" | "error",
  code: string | number,
): string {
  const key = `${prefix}.${code}`;
  if (Object.hasOwn(messages, key)) return t(key as MessageKey);
  if (prefix === "error") return t("error.unknown");
  if (prefix === "operation") return t("task.unknown");
  return String(code);
}

export function capabilityLabel(t: Translate, capability: string): string {
  if (capability === "task.reconcile.v1") return t("capability.reconcile");
  if (capability === "task.network-outcome.v1") return t("capability.networkOutcome");
  const operations: Record<string, number> = {
    "agent.status": 1,
    "kura.configuration.list": 2,
    "kura.snapshot.list": 3,
    "network.inspect": 4,
    "network.prepare": 5,
    "network.confirm": 6,
    "network.rollback": 7,
    "network.transaction": 8,
    "network.takeover.plan": 9,
    "network.takeover.prepare": 10,
    "kura.configuration.get": 11,
      "kura.configuration.update": 12,
  };
  return Object.hasOwn(operations, capability)
    ? translatedCode(t, "operation", operations[capability])
    : capability;
}
