import type { Metadata } from "next";

import type { Locale } from "@/i18n/config";
import { localizePath } from "@/i18n/routing";
import type { Messages } from "@/i18n/dictionaries";
import { siteConfig } from "@/lib/site";

type PageKey = keyof Messages["meta"]["pages"];

function buildAbsoluteUrl(path: string) {
  return new URL(path, siteConfig.url).toString();
}

export function createPageMetadata(
  locale: Locale,
  messages: Messages,
  pageKey: PageKey,
  path: string,
): Metadata {
  const pageMeta = messages.meta.pages[pageKey];
  const localizedPath = localizePath(locale, path);

  return {
    metadataBase: new URL(siteConfig.url),
    title: pageMeta.title,
    description: pageMeta.description,
    applicationName: siteConfig.name,
    alternates: {
      canonical: localizedPath,
      languages: {
        en: buildAbsoluteUrl(localizePath("en", path)),
        da: buildAbsoluteUrl(localizePath("da", path)),
      },
    },
    openGraph: {
      type: "website",
      siteName: siteConfig.name,
      title: pageMeta.title,
      description: pageMeta.description,
      url: buildAbsoluteUrl(localizedPath),
      locale,
    },
    twitter: {
      card: "summary_large_image",
      title: pageMeta.title,
      description: pageMeta.description,
    },
  };
}
