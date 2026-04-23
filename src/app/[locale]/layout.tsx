import type { Metadata } from "next";
import type { ReactNode } from "react";
import { ColorSchemeScript, MantineProvider, mantineHtmlProps } from "@mantine/core";
import "@mantine/core/styles.css";
import { IBM_Plex_Sans, Merriweather } from "next/font/google";

import "@/app/globals.css";
import { SiteFooter } from "@/components/layout/SiteFooter";
import { SiteHeader } from "@/components/layout/SiteHeader";
import { locales } from "@/i18n/config";
import { getPageContext } from "@/i18n/server";
import { siteConfig } from "@/lib/site";
import { theme } from "@/theme";

const bodyFont = IBM_Plex_Sans({
  subsets: ["latin"],
  variable: "--font-body",
  weight: ["400", "500", "600", "700"],
});

const headingFont = Merriweather({
  subsets: ["latin"],
  variable: "--font-heading",
  weight: ["400", "700"],
});

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
    <html
      {...mantineHtmlProps}
      className={`${bodyFont.variable} ${headingFont.variable}`}
      dir="ltr"
      lang={locale}
    >
      <head>
        <ColorSchemeScript defaultColorScheme="auto" />
      </head>
      <body>
        <MantineProvider defaultColorScheme="auto" theme={theme}>
          <SiteHeader
            brand={messages.shared.brand}
            language={messages.shared.language}
            locale={locale}
            navigation={messages.shared.navigation}
            theme={messages.shared.theme}
          />
          <main>{children}</main>
          <SiteFooter
            brand={messages.shared.brand}
            content={messages.shared.footer}
            locale={locale}
            navigation={messages.shared.navigation}
          />
        </MantineProvider>
      </body>
    </html>
  );
}
