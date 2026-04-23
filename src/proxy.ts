import { NextRequest, NextResponse } from "next/server";

import {
  defaultLocale,
  isLocale,
  localeCookieName,
  type Locale,
} from "@/i18n/config";
import { localizePath } from "@/i18n/routing";

function getPreferredLocale(acceptLanguageHeader: string | null): Locale {
  if (!acceptLanguageHeader) {
    return defaultLocale;
  }

  const preferredLanguages = acceptLanguageHeader
    .split(",")
    .map((entry) => entry.trim().split(";")[0]?.toLowerCase())
    .filter(Boolean);

  for (const language of preferredLanguages) {
    if (language === "da" || language.startsWith("da-")) {
      return "da";
    }

    if (language === "en" || language.startsWith("en-")) {
      return "en";
    }
  }

  return defaultLocale;
}

export function proxy(request: NextRequest) {
  const { pathname } = request.nextUrl;
  const pathnameLocale = pathname.split("/")[1];

  if (isLocale(pathnameLocale)) {
    const response = NextResponse.next();
    response.cookies.set(localeCookieName, pathnameLocale, {
      path: "/",
      maxAge: 60 * 60 * 24 * 365,
      sameSite: "lax",
    });
    return response;
  }

  const cookieLocale = request.cookies.get(localeCookieName)?.value;
  const locale: Locale = isLocale(cookieLocale ?? "")
    ? (cookieLocale as Locale)
    : getPreferredLocale(request.headers.get("accept-language"));
  const url = request.nextUrl.clone();

  url.pathname = localizePath(locale, pathname);

  const response = NextResponse.redirect(url);
  response.cookies.set(localeCookieName, locale, {
    path: "/",
    maxAge: 60 * 60 * 24 * 365,
    sameSite: "lax",
  });

  return response;
}

export const config = {
  matcher: ["/((?!api|_next|.*\\..*).*)"],
};
