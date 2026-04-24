export const primaryNavigation = [
  {
    key: "services",
    href: "/services",
  },
  {
    key: "about",
    href: "/about",
  },
  {
    key: "cases",
    href: "/cases",
  },
  {
    key: "contact",
    href: "/contact",
  },
] as const;

export const footerNavigation = [
  ...primaryNavigation,
  {
    key: "privacy",
    href: "/privacy",
  },
] as const;

export const sitemapRoutes = [
  "/",
  "/services",
  "/about",
  "/cases",
  "/contact",
  "/privacy",
] as const;
