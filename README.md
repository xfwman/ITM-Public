# IT-Management Public Website

This repository contains the public-facing website for **IT-Management**.  
It is a multilingual marketing site built to present services, experience, and contact options with a calm, credible B2B tone.

## Tech Stack

- `Next.js` with the App Router
- `TypeScript`
- `Mantine` for UI primitives, theming, forms, and responsive composition
- Locale-based routing with English and Danish message catalogs
- Structured content modules for services and cases

## Run The Project

```bash
npm install
npm run dev
```

Open `http://localhost:3000`.

## Build And Validate

```bash
npm run lint
npm run typecheck
npm run build
```

## i18n Approach

- Routes are locale-prefixed: `/en/...` and `/da/...`
- A root proxy redirects `/` to the best locale based on cookie or browser language
- All user-facing copy is externalized in:
  - [src/messages/en.ts](C:/source/itm-public/src/messages/en.ts)
  - [src/messages/da.ts](C:/source/itm-public/src/messages/da.ts)
- Shared locale helpers live in:
  - [src/i18n/config.ts](C:/source/itm-public/src/i18n/config.ts)
  - [src/i18n/dictionaries.ts](C:/source/itm-public/src/i18n/dictionaries.ts)
  - [src/i18n/routing.ts](C:/source/itm-public/src/i18n/routing.ts)

## Content Updates

- Update service definitions in [src/content/services.ts](C:/source/itm-public/src/content/services.ts)
- Update case definitions in [src/content/cases.ts](C:/source/itm-public/src/content/cases.ts)
- Update visible text in the locale files under [src/messages](C:/source/itm-public/src/messages)
- Update shared site details such as email, URL, and location in [src/lib/site.ts](C:/source/itm-public/src/lib/site.ts)

## Contact Form

The contact form currently uses a placeholder submission flow with an explicit backend boundary:

- Client form: [src/components/forms/ContactForm.tsx](C:/source/itm-public/src/components/forms/ContactForm.tsx)
- Route handler: [src/app/api/contact/route.ts](C:/source/itm-public/src/app/api/contact/route.ts)
- Shared validation/submission logic: [src/lib/contact.ts](C:/source/itm-public/src/lib/contact.ts)

This makes it straightforward to replace the mock handling with email, CRM, or workflow integration later.

## Notes

- The current content is production-structured, but some company details are intentionally placeholders and should be replaced before launch.
- SEO basics are included through localized metadata, `robots`, and `sitemap`.
- The theme supports both light and dark mode out of the box.
