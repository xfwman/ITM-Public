import { List, ListItem, Paper, SimpleGrid } from "@mantine/core";
import type { Metadata } from "next";

import { CaseCard } from "@/components/sections/CaseCard";
import { CtaSection } from "@/components/sections/CtaSection";
import { PageIntro } from "@/components/sections/PageIntro";
import { SectionShell } from "@/components/shared/SectionShell";
import { caseDefinitions } from "@/content/cases";
import { getPageContext } from "@/i18n/server";
import { localizePath } from "@/i18n/routing";
import { createPageMetadata } from "@/lib/metadata";

interface CasesPageProps {
  params: Promise<{
    locale: string;
  }>;
}

export async function generateMetadata({
  params,
}: CasesPageProps): Promise<Metadata> {
  const { locale, messages } = await getPageContext(params);
  return createPageMetadata(locale, messages, "cases", "/cases");
}

export default async function CasesPage({ params }: CasesPageProps) {
  const { locale, messages } = await getPageContext(params);
  const page = messages.pages.cases;
  const labels = messages.shared.labels;
  const navigation = messages.shared.navigation;

  return (
    <>
      <SectionShell>
        <PageIntro
          description={page.hero.description}
          eyebrow={page.hero.eyebrow}
          title={page.hero.title}
        />
      </SectionShell>

      <SectionShell
        description={page.overview.description}
        eyebrow={page.overview.eyebrow}
        title={page.overview.title}
      >
        <Paper bg="var(--panel-surface-strong)" p="xl" radius="xl" shadow="sm" withBorder>
          <List spacing="sm">
            {page.overview.items.map((item) => (
              <ListItem key={item}>{item}</ListItem>
            ))}
          </List>
        </Paper>
      </SectionShell>

      <SectionShell>
        <SimpleGrid cols={{ base: 1, md: 2 }} spacing="lg">
          {caseDefinitions.map((caseItem) => {
            const content = messages.content.cases[caseItem.id];

            return (
              <CaseCard
                approach={content.approach}
                challenge={content.challenge}
                compact={false}
                context={content.context}
                icon={caseItem.icon}
                key={caseItem.id}
                labels={labels}
                outcome={content.outcome}
                sectionLabel={navigation.cases}
                summary={content.summary}
                title={content.title}
              />
            );
          })}
        </SimpleGrid>
      </SectionShell>

      <SectionShell>
        <CtaSection
          description={page.cta.description}
          primaryHref={localizePath(locale, "/contact")}
          primaryLabel={page.cta.primaryCta}
          title={page.cta.title}
        />
      </SectionShell>
    </>
  );
}
