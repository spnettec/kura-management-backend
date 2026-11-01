import { createI18n } from 'vue-i18n'
import { messages, type SupportedLocale } from './messages'

const storageKey = 'kura-management-locale'
const fallbackLocale: SupportedLocale = 'zh-CN'

function detectLocale(): SupportedLocale {
  const saved = localStorage.getItem(storageKey)
  if (saved === 'zh-CN' || saved === 'en-US') {
    return saved
  }
  return fallbackLocale
}

export const i18n = createI18n({
  legacy: false,
  locale: detectLocale(),
  fallbackLocale,
  messages,
})

export function setLocale(locale: SupportedLocale) {
  i18n.global.locale.value = locale
  localStorage.setItem(storageKey, locale)
}

export const supportedLocales: SupportedLocale[] = ['zh-CN', 'en-US']

