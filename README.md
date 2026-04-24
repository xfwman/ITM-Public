# IT-Management Public Website

This repository contains the public-facing website for **IT-Management**.  
It is a multilingual marketing site built to present services, experience, and contact options with a calm, credible B2B tone.

## Tech Stack

- `Next.js` with the App Router
- `TypeScript`
- `Mantine` for UI primitives, theming, forms, and responsive composition
- Locale-based routing with English and Danish markdown content files
- Headless content architecture with frontmatter markdown under root `content/`

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
- All user-facing copy is externalized in markdown files with locale suffixes such as:
  - [content/shared/site.en.md](C:/source/itm-public/content/shared/site.en.md)
  - [content/shared/site.da.md](C:/source/itm-public/content/shared/site.da.md)
  - [content/pages/home/index.en.md](C:/source/itm-public/content/pages/home/index.en.md)
  - [content/pages/home/index.da.md](C:/source/itm-public/content/pages/home/index.da.md)
  - [content/catalog/services/index.en.md](C:/source/itm-public/content/catalog/services/index.en.md)
  - [content/catalog/services/index.da.md](C:/source/itm-public/content/catalog/services/index.da.md)
- Shared locale helpers and the markdown loader live in:
  - [src/i18n/config.ts](C:/source/itm-public/src/i18n/config.ts)
  - [src/i18n/dictionaries.ts](C:/source/itm-public/src/i18n/dictionaries.ts)
  - [src/i18n/routing.ts](C:/source/itm-public/src/i18n/routing.ts)
  - [src/lib/content-loader.ts](C:/source/itm-public/src/lib/content-loader.ts)

## Headless Content Structure

Content is organized hierarchically under the root [content](C:/source/itm-public/content) directory:

- [content/shared](C:/source/itm-public/content/shared) for brand, navigation, footer, theme, and shared labels
- [content/pages](C:/source/itm-public/content/pages) for page-specific metadata and content blocks
- [content/catalog](C:/source/itm-public/content/catalog) for structured collections like services, cases, audiences, and engagement models

Every localized file uses a locale suffix, for example `site.en.md`, `site.da.md`, `index.en.md`, or `index.da.md`.

## Content Updates

- Update shared site copy in [content/shared](C:/source/itm-public/content/shared)
- Update page copy and metadata in [content/pages](C:/source/itm-public/content/pages)
- Update structured services, cases, audiences, and engagement models in [content/catalog](C:/source/itm-public/content/catalog)
- Update structural icon/route definitions in [src/data](C:/source/itm-public/src/data)
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
