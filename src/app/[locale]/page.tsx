import { Grid, GridCol, Paper, SimpleGrid, Stack, Text } from "@mantine/core";
import type { Metadata } from "next";

import { CaseCard } from "@/components/sections/CaseCard";
import { CtaSection } from "@/components/sections/CtaSection";
import { HeroSection } from "@/components/sections/HeroSection";
import { ServiceCard } from "@/components/sections/ServiceCard";
import { SectionShell } from "@/components/shared/SectionShell";
import { caseDefinitions } from "@/content/cases";
import { featuredServices } from "@/content/services";
import { getPageContext } from "@/i18n/server";
import { localizePath } from "@/i18n/routing";
import { createPageMetadata } from "@/lib/metadata";

interface HomePageProps {
  params: Promise<{
    locale: string;
  }>;
}

export async function generateMetadata({
  params,
}: HomePageProps): Promise<Metadata> {
  const { locale, messages } = await getPageContext(params);
  return createPageMetadata(locale, messages, "home", "/");
}

export default async function HomePage({ params }: HomePageProps) {
  const { locale, messages } = await getPageContext(params);
  const page = messages.pages.home;
  const labels = messages.shared.labels;

  return (
    <>
      <SectionShell>
        <HeroSection
          description={page.hero.description}
          eyebrow={page.hero.eyebrow}
          highlights={page.hero.highlights}
          panelItems={page.hero.panelItems}
          panelTitle={page.hero.panelTitle}
          primaryHref={localizePath(locale, "/contact")}
          primaryLabel={page.hero.primaryCta}
          secondaryHref={localizePath(locale, "/services")}
          secondaryLabel={page.hero.secondaryCta}
          title={page.hero.title}
        />
      </SectionShell>

      <SectionShell
        description={page.services.description}
        eyebrow={page.services.eyebrow}
        title={page.services.title}
      >
        <SimpleGrid cols={{ base: 1, md: 3 }} spacing="lg">
          {featuredServices.map((service) => {
            const content = messages.content.services[service.id];

            return (
              <ServiceCard
                audienceTags={service.audienceIds.map(
                  (audienceId) => messages.content.audiences[audienceId],
                )}
                compact
                deliverables={content.deliverables}
                description={content.description}
                engagementTags={service.engagementModelIds.map(
                  (engagementId) =>
                    messages.content.engagementModels[engagementId],
                )}
                icon={service.icon}
                key={service.id}
                labels={labels}
                outcomes={content.outcomes}
                summary={content.summary}
                title={content.title}
              />
            );
          })}
        </SimpleGrid>
      </SectionShell>

      <SectionShell
        description={page.proof.description}
        eyebrow={page.proof.eyebrow}
        title={page.proof.title}
      >
        <Grid gap="lg">
          <GridCol span={{ base: 12, lg: 5 }}>
            <SimpleGrid cols={{ base: 1, sm: 3, lg: 1 }} spacing="lg">
              {page.proof.stats.map((item) => (
                <Paper
                  bg="var(--panel-surface)"
                  key={item.label}
                  p="xl"
                  radius="xl"
                  shadow="sm"
                  withBorder
                >
                  <Stack gap="xs">
                    <Text fw={700} size="2rem">
                      {item.value}
                    </Text>
                    <Text c="dimmed">{item.label}</Text>
                  </Stack>
                </Paper>
              ))}
            </SimpleGrid>
          </GridCol>
          <GridCol span={{ base: 12, lg: 7 }}>
            <SimpleGrid cols={{ base: 1, md: 3 }} spacing="lg">
              {page.proof.pillars.map((item) => (
                <Paper
                  bg="var(--panel-surface-strong)"
                  key={item.title}
                  p="xl"
                  radius="xl"
                  shadow="sm"
                  withBorder
                >
                  <Stack gap="sm">
                    <Text fw={600} size="lg">
                      {item.title}
                    </Text>
                    <Text c="dimmed">{item.text}</Text>
                  </Stack>
                </Paper>
              ))}
            </SimpleGrid>
          </GridCol>
        </Grid>
      </SectionShell>

      <SectionShell
        description={page.cases.description}
        eyebrow={page.cases.eyebrow}
        title={page.cases.title}
      >
        <SimpleGrid cols={{ base: 1, md: 3 }} spacing="lg">
          {caseDefinitions
            .filter((caseItem) => caseItem.featured)
            .map((caseItem) => {
              const content = messages.content.cases[caseItem.id];

              return (
                <CaseCard
                  approach={content.approach}
                  challenge={content.challenge}
                  compact
                  context={content.context}
                  icon={caseItem.icon}
                  key={caseItem.id}
                  labels={labels}
                  outcome={content.outcome}
                  summary={content.summary}
                  title={content.title}
                />
              );
            })}
        </SimpleGrid>
      </SectionShell>

      <SectionShell
        description={page.principles.description}
        eyebrow={page.principles.eyebrow}
        title={page.principles.title}
      >
        <SimpleGrid cols={{ base: 1, md: 3 }} spacing="lg">
          {page.principles.items.map((item) => (
            <Paper
              bg="var(--panel-surface)"
              key={item.title}
              p="xl"
              radius="xl"
              shadow="sm"
              withBorder
            >
              <Stack gap="sm">
                <Text fw={600} size="lg">
                  {item.title}
                </Text>
                <Text c="dimmed">{item.text}</Text>
              </Stack>
            </Paper>
          ))}
        </SimpleGrid>
      </SectionShell>

      <SectionShell>
        <CtaSection
          description={page.cta.description}
          primaryHref={localizePath(locale, "/contact")}
          primaryLabel={page.cta.primaryCta}
          secondaryHref={localizePath(locale, "/services")}
          secondaryLabel={page.cta.secondaryCta}
          title={page.cta.title}
        />
      </SectionShell>
    </>
  );
}
