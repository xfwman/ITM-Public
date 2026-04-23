import type { Metadata } from "next";
import type { ReactNode } from "react";
import { MantineProvider } from "@mantine/core";

import { AmbientStage } from "@/components/layout/AmbientStage";
import { SiteFooter } from "@/components/layout/SiteFooter";
import { SiteHeader } from "@/components/layout/SiteHeader";
import { locales } from "@/i18n/config";
import { getPageContext } from "@/i18n/server";
import { siteConfig } from "@/lib/site";
import { theme } from "@/theme";

export const metadata: Metadata = {
  metadataBase: new URL(siteConfig.url),
  applicationName: siteConfig.name,
  title: {
    default: siteConfig.name,
    template: `%s | ${siteConfig.name}`,
  },
  description: siteConfig.description,
  openGraph: {
    siteName: siteConfig.name,
    type: "website",
  },
};

export function generateStaticParams() {
  return locales.map((locale) => ({
    locale,
  }));
}

export const dynamicParams = false;

interface LocaleLayoutProps {
  children: ReactNode;
  params: Promise<{
    locale: string;
  }>;
}

export default async function LocaleLayout({
  children,
  params,
}: LocaleLayoutProps) {
  const { locale, messages } = await getPageContext(params);

  return (
    <MantineProvider defaultColorScheme="auto" theme={theme}>
      <SiteHeader
        brand={messages.shared.brand}
        language={messages.shared.language}
        locale={locale}
        navigation={messages.shared.navigation}
        theme={messages.shared.theme}
      />
      <AmbientStage>
        <main>{children}</main>
      </AmbientStage>
      <SiteFooter
        brand={messages.shared.brand}
        content={messages.shared.footer}
        locale={locale}
        navigation={messages.shared.navigation}
      />
    </MantineProvider>
  );
}
