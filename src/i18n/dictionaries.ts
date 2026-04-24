import "server-only";

import type { Locale } from "@/i18n/config";
import {
  getLocaleMessages,
  type LocaleMessages,
} from "@/lib/content-loader";

export type Messages = LocaleMessages;

export async function getMessages(locale: Locale): Promise<Messages> {
  return getLocaleMessages(locale);
}
