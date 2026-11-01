import {
  createContext,
  useContext,
  useEffect,
  useMemo,
  useState,
  type ReactNode,
} from "react";
import { ConfigProvider, type FormInstance } from "antd";
import zhCN from "antd/locale/zh_CN";
import enUS from "antd/locale/en_US";
import { ApiError } from "../api/client";
import {
  localeStorageKey,
  resolveLocale,
  translatedCode,
  translator,
  type Locale,
  type Translate,
} from "./core";

type I18n = {
  locale: Locale;
  setLocale: (value: Locale) => void;
  t: Translate;
  dateTime: (value: string | null) => string;
};
const Context = createContext<I18n | null>(null);

function initialLocale(): Locale {
  let saved: string | null = null;
  try {
    saved = localStorage.getItem(localeStorageKey);
  } catch {
    /* Storage may be disabled. */
  }
  return resolveLocale(saved, navigator.languages ?? [navigator.language]);
}

export function I18nProvider({ children }: { children: ReactNode }) {
  const [locale, setLocale] = useState(initialLocale);
  const t = useMemo(() => translator(locale), [locale]);
  useEffect(() => {
    document.documentElement.lang = locale;
    document.title = t("app.title");
    try {
      localStorage.setItem(localeStorageKey, locale);
    } catch {
      /* Switching still works in memory. */
    }
  }, [locale, t]);
  const value = useMemo(
    () => ({
      locale,
      setLocale,
      t,
      dateTime: (value: string | null) => {
        const date = value ? new Date(value) : undefined;
        return date && !Number.isNaN(date.getTime())
          ? new Intl.DateTimeFormat(locale, {
              dateStyle: "medium",
              timeStyle: "medium",
            }).format(date)
          : "—";
      },
    }),
    [locale, t],
  );
  return (
    <Context.Provider value={value}>
      <ConfigProvider
        locale={locale === "zh-CN" ? zhCN : enUS}
        theme={{
          token: {
            colorPrimary: "#1b7059",
            borderRadius: 6,
            fontFamily:
              'Inter, -apple-system, BlinkMacSystemFont, "PingFang SC", sans-serif',
            colorBgLayout: "#f5f7f5",
          },
          components: {
            Table: { headerBg: "#f7f9f7", headerColor: "#64716b" },
          },
        }}
      >
        {children}
      </ConfigProvider>
    </Context.Provider>
  );
}

export function useI18n(): I18n {
  const value = useContext(Context);
  if (!value) throw new Error("I18nProvider is required");
  return value;
}

export function LanguageSwitcher() {
  const { locale, setLocale } = useI18n();
  return (
    <label className="language-switcher">
      <span>语言 / Language</span>
      <select
        aria-label="语言 / Language"
        value={locale}
        onChange={(event) => setLocale(event.target.value as Locale)}
      >
        <option value="zh-CN" lang="zh-CN">
          简体中文
        </option>
        <option value="en-US" lang="en-US">
          English
        </option>
      </select>
    </label>
  );
}

// Revalidate only fields already showing errors; switching never clears typed values.
export function useLocalizedForm(form: FormInstance) {
  const { locale } = useI18n();
  useEffect(() => {
    const names = form
      .getFieldsError()
      .filter((field) => field.errors.length)
      .map((field) => field.name);
    if (names.length) void form.validateFields(names).catch(() => {});
  }, [locale, form]);
}

export function errorText(error: unknown, t: Translate): string {
  if (error instanceof ApiError) return translatedCode(t, "error", error.code);
  if (error instanceof TypeError) return t("error.network");
  return t("error.unknown");
}
