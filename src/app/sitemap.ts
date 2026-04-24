import type { MetadataRoute } from "next";

import { sitemapRoutes } from "@/data/navigation";
import { locales } from "@/i18n/config";
import { localizePath } from "@/i18n/routing";
import { siteConfig } from "@/lib/site";

export default function sitemap(): MetadataRoute.Sitemap {
  const lastModified = new Date();

  return sitemapRoutes.flatMap((route) =>
    locales.map((locale) => ({
      url: `${siteConfig.url}${localizePath(locale, route)}`,
      lastModified,
      alternates: {
        languages: {
          en: `${siteConfig.url}${localizePath("en", route)}`,
          da: `${siteConfig.url}${localizePath("da", route)}`,
        },
      },
    })),
  );
}
