import { notFound } from "next/navigation";

import { isLocale, type Locale } from "@/i18n/config";
import { getMessages } from "@/i18n/dictionaries";

type LocaleParams =
  | Promise<{
      locale: string;
    }>
  | {
      locale: string;
    };

export async function getLocaleFromParams(params: LocaleParams): Promise<Locale> {
  const { locale } = await params;

  if (!isLocale(locale)) {
    notFound();
  }

  return locale;
}

export async function getPageContext(params: LocaleParams) {
  const locale = await getLocaleFromParams(params);
  const messages = await getMessages(locale);

  return {
    locale,
    messages,
  };
}
