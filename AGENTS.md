# AGENTS.md

## Project Purpose

This repository is the public website for **IT-Management**.  
Its purpose is to communicate services, credibility, and contact opportunities through a professional multilingual marketing site.

## Technology Constraints

- Use `Next.js` with the App Router and `TypeScript`
- Use `Mantine` as the primary UI framework
- Follow Mantine-native composition patterns and treat `ui.mantine.dev` as the preferred source of UI ideas
- Preserve the existing multi-language architecture with locale-based routing
- Do not bypass shared theme, layout, or section primitives without a clear reason

## Architecture Rules

- Keep all visible user-facing copy externalized in root-level markdown files under `content/`
- Reuse shared components before introducing new visual variants
- Keep page sections composable and content-driven
- Preserve accessibility, keyboard usability, and responsive behavior
- Keep services, cases, shared site copy, and page content structured in hierarchical markdown modules with locale suffixes like `*.en.md`

## Design System Guidance

- Maintain a calm, credible B2B visual tone
- Avoid gratuitous visual noise, novelty effects, or heavy animation
- Prefer Mantine-native styling and theme tokens over ad hoc custom styling
- Keep both light and dark themes working

## Implementation Rules

- No hardcoded user-facing strings in reusable components
- Prefer shared theme tokens, Mantine props, and existing CSS modules before writing new one-off styles
- Avoid large new dependencies unless they are clearly justified
- Keep forms, CTAs, and interaction patterns simple and understandable
- Prefer server-first rendering patterns for public marketing content

## Content Rules

- The root `content/` directory is the headless source of truth for editorial content
- Use frontmatter markdown files with locale suffixes like `index.en.md` and `index.da.md`
- Organize content hierarchically in subdirectories such as `content/pages/...`, `content/catalog/...`, and `content/shared/...`
- Services and cases must stay structured and reusable
- English and Danish translations must stay synchronized
- Placeholder content must be clearly recognizable and easy to replace
- Metadata, navigation, footer text, and form labels must remain localizable

## Quality Gates

- `npm run lint` passes
- `npm run typecheck` passes
- `npm run build` passes
- No obvious accessibility regressions are introduced

## Change Discipline

- Prefer small cohesive commits
- Preserve repo readability and folder clarity
- Document significant architectural deviations in `README.md` or nearby source comments when necessary
