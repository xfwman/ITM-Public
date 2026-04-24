import "server-only";

import { readFile } from "node:fs/promises";
import path from "node:path";
import matter from "gray-matter";
import { cache } from "react";

import type { Locale } from "@/i18n/config";
import type {
  AudienceId,
  EngagementModelId,
  ServiceId,
} from "@/data/services";
import type { CaseId } from "@/data/cases";

interface PageMeta {
  title: string;
  description: string;
}

interface SharedContent {
  brand: {
    name: string;
    strapline: string;
  };
  navigation: {
    home: string;
    services: string;
    about: string;
    cases: string;
    contact: string;
    privacy: string;
    contactCta: string;
    menu: string;
  };
  actions: {
    learnMore: string;
    viewServices: string;
    viewCases: string;
    contact: string;
    sendMessage: string;
    readMore: string;
  };
  language: {
    label: string;
    options: {
      en: string;
      da: string;
    };
  };
  theme: {
    toggle: string;
    light: string;
    dark: string;
  };
  labels: {
    deliverables: string;
    outcomes: string;
    context: string;
    challenge: string;
    approach: string;
    outcome: string;
  };
  footer: {
    description: string;
    reachOutHeading: string;
    locationLabel: string;
    emailLabel: string;
    sitemapHeading: string;
    rights: string;
    note: string;
  };
}

interface HomePageFile {
  meta: PageMeta;
  hero: {
    eyebrow: string;
    title: string;
    description: string;
    primaryCta: string;
    secondaryCta: string;
    highlights: string[];
    panelTitle: string;
    panelItems: string[];
  };
  services: {
    eyebrow: string;
    title: string;
    description: string;
  };
  proof: {
    eyebrow: string;
    title: string;
    description: string;
    stats: Array<{
      value: string;
      label: string;
    }>;
    pillars: Array<{
      title: string;
      text: string;
    }>;
  };
  cases: {
    eyebrow: string;
    title: string;
    description: string;
    cta: string;
  };
  principles: {
    eyebrow: string;
    title: string;
    description: string;
    items: Array<{
      title: string;
      text: string;
    }>;
  };
  cta: {
    title: string;
    description: string;
    primaryCta: string;
    secondaryCta: string;
  };
}

interface ServicesPageFile {
  meta: PageMeta;
  hero: {
    eyebrow: string;
    title: string;
    description: string;
  };
  outcomes: {
    eyebrow: string;
    title: string;
    description: string;
    items: string[];
  };
  customers: {
    eyebrow: string;
    title: string;
    description: string;
  };
  models: {
    eyebrow: string;
    title: string;
    description: string;
  };
  cta: {
    title: string;
    description: string;
    primaryCta: string;
  };
}

interface AboutPageFile {
  meta: PageMeta;
  hero: {
    eyebrow: string;
    title: string;
    description: string;
  };
  story: {
    eyebrow: string;
    title: string;
    paragraphs: string[];
  };
  profile: {
    eyebrow: string;
    title: string;
    role: string;
    summary: string;
    focusAreas: string[];
  };
  principles: {
    eyebrow: string;
    title: string;
    description: string;
    items: Array<{
      title: string;
      text: string;
    }>;
  };
  experience: {
    eyebrow: string;
    title: string;
    description: string;
    stats: Array<{
      value: string;
      label: string;
    }>;
  };
  cta: {
    title: string;
    description: string;
    primaryCta: string;
  };
}

interface CasesPageFile {
  meta: PageMeta;
  hero: {
    eyebrow: string;
    title: string;
    description: string;
  };
  overview: {
    eyebrow: string;
    title: string;
    description: string;
    items: string[];
  };
  cta: {
    title: string;
    description: string;
    primaryCta: string;
  };
}

interface ContactPageFile {
  meta: PageMeta;
  hero: {
    eyebrow: string;
    title: string;
    description: string;
  };
  details: {
    eyebrow: string;
    title: string;
    description: string;
    emailLabel: string;
    locationLabel: string;
    responseLabel: string;
    responseValue: string;
    bookingLabel: string;
    bookingValue: string;
  };
  form: {
    title: string;
    description: string;
    fields: {
      name: string;
      email: string;
      company: string;
      message: string;
      honeypot: string;
    };
    placeholders: {
      name: string;
      email: string;
      company: string;
      message: string;
    };
    validation: {
      nameRequired: string;
      emailRequired: string;
      emailInvalid: string;
      messageRequired: string;
    };
    submit: string;
    submitting: string;
    privacyNote: string;
    successTitle: string;
    successMessage: string;
    failureTitle: string;
    failureMessage: string;
  };
  expectations: {
    title: string;
    items: string[];
  };
}

interface PrivacyPageFile {
  meta: PageMeta;
  hero: {
    eyebrow: string;
    title: string;
    description: string;
  };
  sections: Array<{
    title: string;
    paragraphs: string[];
  }>;
}

interface EntryCollection<TEntries> {
  entries: TEntries;
}

interface ServiceEntry {
  title: string;
  summary: string;
  description: string;
  deliverables: string[];
  outcomes: string[];
}

interface CaseEntry {
  title: string;
  summary: string;
  context: string;
  challenge: string;
  approach: string;
  outcome: string;
}

export interface LocaleMessages {
  shared: SharedContent;
  meta: {
    pages: {
      home: PageMeta;
      services: PageMeta;
      about: PageMeta;
      cases: PageMeta;
      contact: PageMeta;
      privacy: PageMeta;
    };
  };
  content: {
    audiences: Record<AudienceId, string>;
    engagementModels: Record<EngagementModelId, string>;
    services: Record<ServiceId, ServiceEntry>;
    cases: Record<CaseId, CaseEntry>;
  };
  pages: {
    home: Omit<HomePageFile, "meta">;
    services: Omit<ServicesPageFile, "meta">;
    about: Omit<AboutPageFile, "meta">;
    cases: Omit<CasesPageFile, "meta">;
    contact: Omit<ContactPageFile, "meta">;
    privacy: Omit<PrivacyPageFile, "meta">;
  };
}

const CONTENT_ROOT = path.join(process.cwd(), "content");

async function readFrontmatter<TFrontmatter>(relativePath: string) {
  const filePath = path.join(CONTENT_ROOT, relativePath);
  const source = await readFile(filePath, "utf8");
  const parsed = matter(source);

  return parsed.data as TFrontmatter;
}

export const getLocaleMessages = cache(async (locale: Locale) => {
  const [
    shared,
    home,
    servicesPage,
    about,
    casesPage,
    contact,
    privacy,
    audiences,
    engagementModels,
    services,
    cases,
  ] = await Promise.all([
    readFrontmatter<SharedContent>(`shared/site.${locale}.md`),
    readFrontmatter<HomePageFile>(`pages/home/index.${locale}.md`),
    readFrontmatter<ServicesPageFile>(`pages/services/index.${locale}.md`),
    readFrontmatter<AboutPageFile>(`pages/about/index.${locale}.md`),
    readFrontmatter<CasesPageFile>(`pages/cases/index.${locale}.md`),
    readFrontmatter<ContactPageFile>(`pages/contact/index.${locale}.md`),
    readFrontmatter<PrivacyPageFile>(`pages/privacy/index.${locale}.md`),
    readFrontmatter<EntryCollection<Record<AudienceId, string>>>(
      `catalog/audiences/index.${locale}.md`,
    ),
    readFrontmatter<EntryCollection<Record<EngagementModelId, string>>>(
      `catalog/engagement-models/index.${locale}.md`,
    ),
    readFrontmatter<EntryCollection<Record<ServiceId, ServiceEntry>>>(
      `catalog/services/index.${locale}.md`,
    ),
    readFrontmatter<EntryCollection<Record<CaseId, CaseEntry>>>(
      `catalog/cases/index.${locale}.md`,
    ),
  ]);

  const { meta: homeMeta, ...homePage } = home;
  const { meta: servicesMeta, ...servicesContent } = servicesPage;
  const { meta: aboutMeta, ...aboutPage } = about;
  const { meta: casesMeta, ...casesContent } = casesPage;
  const { meta: contactMeta, ...contactPage } = contact;
  const { meta: privacyMeta, ...privacyPage } = privacy;

  return {
    shared,
    meta: {
      pages: {
        home: homeMeta,
        services: servicesMeta,
        about: aboutMeta,
        cases: casesMeta,
        contact: contactMeta,
        privacy: privacyMeta,
      },
    },
    content: {
      audiences: audiences.entries,
      engagementModels: engagementModels.entries,
      services: services.entries,
      cases: cases.entries,
    },
    pages: {
      home: homePage,
      services: servicesContent,
      about: aboutPage,
      cases: casesContent,
      contact: contactPage,
      privacy: privacyPage,
    },
  } satisfies LocaleMessages;
});
