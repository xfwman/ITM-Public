export const locales = ["en", "da"] as const;

export type Locale = (typeof locales)[number];

export const defaultLocale: Locale = "en";

export const localeCookieName = "itm-locale";

export function isLocale(value: string): value is Locale {
  return locales.includes(value as Locale);
}
