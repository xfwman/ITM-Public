import { Grid, GridCol, Group, Paper, SimpleGrid, Stack, Text } from "@mantine/core";
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

import classes from "./home.module.css";

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
  const navigation = messages.shared.navigation;
  const featuredCases = caseDefinitions.filter((caseItem) => caseItem.featured);

  const heroSpotlights = [
    {
      label: navigation.services,
      title: messages.content.services[featuredServices[0].id].title,
      description: messages.content.services[featuredServices[0].id].summary,
      href: localizePath(locale, "/services"),
    },
    {
      label: navigation.cases,
      title: messages.content.cases[featuredCases[0].id].title,
      description: messages.content.cases[featuredCases[0].id].summary,
      href: localizePath(locale, "/cases"),
    },
    {
      label: navigation.cases,
      title: messages.content.cases[featuredCases[1].id].title,
      description: messages.content.cases[featuredCases[1].id].summary,
      href: localizePath(locale, "/cases"),
    },
  ] as const;

  return (
    <>
      <SectionShell>
        <HeroSection
          description={page.hero.description}
          eyebrow={page.hero.eyebrow}
          highlights={page.hero.highlights}
          metrics={page.proof.stats}
          panelItems={page.hero.panelItems}
          panelTitle={page.hero.panelTitle}
          primaryHref={localizePath(locale, "/contact")}
          primaryLabel={page.hero.primaryCta}
          secondaryHref={localizePath(locale, "/services")}
          secondaryLabel={page.hero.secondaryCta}
          spotlights={heroSpotlights}
          title={page.hero.title}
        />
      </SectionShell>

      <SectionShell>
        <Paper className={classes.capabilityStrip} p={{ base: "lg", md: "xl" }} radius="xl">
          <Stack gap="lg" pos="relative">
            <Text className={classes.capabilityLabel}>{page.services.eyebrow}</Text>
            <Group gap="sm">
              {featuredServices.map((service) => (
                <Paper
                  className={classes.capabilityItem}
                  key={service.id}
                  p={{ base: "sm", md: "md" }}
                  radius="xl"
                >
                  <Text fw={600}>{messages.content.services[service.id].title}</Text>
                </Paper>
              ))}
            </Group>
          </Stack>
        </Paper>
      </SectionShell>

      <SectionShell
        description={page.services.description}
        eyebrow={page.services.eyebrow}
        title={page.services.title}
      >
        <Paper className={classes.lightFrame} p={{ base: "lg", md: "2rem" }} radius="xl">
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
                  sectionLabel={navigation.services}
                  summary={content.summary}
                  title={content.title}
                />
              );
            })}
          </SimpleGrid>
        </Paper>
      </SectionShell>

      <SectionShell
        description={page.proof.description}
        eyebrow={page.proof.eyebrow}
        title={page.proof.title}
      >
        <Paper className={classes.proofPanel} p={{ base: "lg", md: "2rem" }} radius="xl">
          <Grid gap="lg" pos="relative">
            <GridCol span={{ base: 12, lg: 5 }}>
              <SimpleGrid cols={{ base: 1, sm: 3, lg: 1 }} spacing="lg">
                {page.proof.stats.map((item) => (
                  <Paper
                    className={classes.statCard}
                    key={item.label}
                    p="xl"
                    radius="xl"
                    shadow="sm"
                    withBorder
                  >
                    <Stack gap="xs">
                      <Text c="white" fw={700} size="2rem">
                        {item.value}
                      </Text>
                      <Text c="rgba(239, 243, 247, 0.62)">{item.label}</Text>
                    </Stack>
                  </Paper>
                ))}
              </SimpleGrid>
            </GridCol>
            <GridCol span={{ base: 12, lg: 7 }}>
              <SimpleGrid cols={{ base: 1, md: 3 }} spacing="lg">
                {page.proof.pillars.map((item) => (
                  <Paper
                    className={classes.pillarCard}
                    key={item.title}
                    p="xl"
                    radius="xl"
                    shadow="sm"
                    withBorder
                  >
                    <Stack gap="sm">
                      <Text c="white" fw={600} size="lg">
                        {item.title}
                      </Text>
                      <Text c="rgba(239, 243, 247, 0.66)">{item.text}</Text>
                    </Stack>
                  </Paper>
                ))}
              </SimpleGrid>
            </GridCol>
          </Grid>
        </Paper>
      </SectionShell>

      <SectionShell
        description={page.cases.description}
        eyebrow={page.cases.eyebrow}
        title={page.cases.title}
      >
        <Grid gap="lg">
          <GridCol span={{ base: 12, lg: 7 }}>
            {featuredCases[0] ? (
              <CaseCard
                approach={messages.content.cases[featuredCases[0].id].approach}
                challenge={messages.content.cases[featuredCases[0].id].challenge}
                context={messages.content.cases[featuredCases[0].id].context}
                icon={featuredCases[0].icon}
                labels={labels}
                outcome={messages.content.cases[featuredCases[0].id].outcome}
                sectionLabel={navigation.cases}
                summary={messages.content.cases[featuredCases[0].id].summary}
                title={messages.content.cases[featuredCases[0].id].title}
              />
            ) : null}
          </GridCol>
          <GridCol span={{ base: 12, lg: 5 }}>
            <Stack gap="lg">
              {featuredCases.slice(1).map((caseItem) => {
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
                    sectionLabel={navigation.cases}
                    summary={content.summary}
                    title={content.title}
                  />
                );
              })}
            </Stack>
          </GridCol>
        </Grid>
      </SectionShell>

      <SectionShell
        description={page.principles.description}
        eyebrow={page.principles.eyebrow}
        title={page.principles.title}
      >
        <SimpleGrid cols={{ base: 1, md: 3 }} spacing="lg">
          {page.principles.items.map((item) => (
            <Paper
              className={classes.principleCard}
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
