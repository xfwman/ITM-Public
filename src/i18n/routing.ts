import { isLocale, type Locale } from "@/i18n/config";

export function localizePath(locale: Locale, path: string): string {
  if (path === "/") {
    return `/${locale}`;
  }

  return `/${locale}${path.startsWith("/") ? path : `/${path}`}`;
}

export function stripLocaleFromPathname(pathname: string): string {
  const segments = pathname.split("/");
  const localeCandidate = segments[1];

  if (!isLocale(localeCandidate)) {
    return pathname || "/";
  }

  const remainder = `/${segments.slice(2).join("/")}`.replace(/\/+/g, "/");
  return remainder === "/" ? "/" : remainder.replace(/\/$/, "");
}

export function replaceLocaleInPathname(
  pathname: string,
  locale: Locale,
): string {
  return localizePath(locale, stripLocaleFromPathname(pathname));
}
