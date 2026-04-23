import type { ReactNode } from "react";
import { mantineHtmlProps } from "@mantine/core";
import "@mantine/core/styles.css";
import { Plus_Jakarta_Sans, Sora } from "next/font/google";
import Script from "next/script";

import "@/app/globals.css";
import { getMantineColorSchemeScript } from "@/theme/color-scheme-script";

const bodyFont = Plus_Jakarta_Sans({
  subsets: ["latin"],
  variable: "--font-body",
  weight: ["400", "500", "600", "700"],
});

const headingFont = Sora({
  subsets: ["latin"],
  variable: "--font-heading",
  weight: ["400", "600", "700"],
});

interface RootLayoutProps {
  children: ReactNode;
  params: Promise<{
    locale?: string;
  }>;
}

export default async function RootLayout({
  children,
  params,
}: RootLayoutProps) {
  const { locale } = await params;

  return (
    <html
      {...mantineHtmlProps}
      className={`${bodyFont.variable} ${headingFont.variable}`}
      dir="ltr"
      lang={locale ?? "en"}
    >
      <body>
        <Script id="mantine-color-scheme" strategy="beforeInteractive">
          {getMantineColorSchemeScript("auto")}
        </Script>
        {children}
      </body>
    </html>
  );
}
