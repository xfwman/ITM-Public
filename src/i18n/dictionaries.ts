import "server-only";

import type { Locale } from "@/i18n/config";

type DeepWiden<T> = T extends string
  ? string
  : T extends number
    ? number
    : T extends boolean
      ? boolean
      : T extends readonly (infer U)[]
        ? ReadonlyArray<DeepWiden<U>>
        : T extends object
          ? {
              [K in keyof T]: DeepWiden<T[K]>;
            }
          : T;

export type Messages = DeepWiden<typeof import("@/messages/en").default>;

const dictionaries = {
  en: () => import("@/messages/en").then((mod) => mod.default),
  da: () => import("@/messages/da").then((mod) => mod.default),
} satisfies Record<Locale, () => Promise<Messages>>;

export async function getMessages(locale: Locale): Promise<Messages> {
  return dictionaries[locale]();
}
