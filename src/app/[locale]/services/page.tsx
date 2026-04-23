import { List, ListItem, Paper, SimpleGrid, Text } from "@mantine/core";
import type { Metadata } from "next";

import { CtaSection } from "@/components/sections/CtaSection";
import { PageIntro } from "@/components/sections/PageIntro";
import { ServiceCard } from "@/components/sections/ServiceCard";
import { SectionShell } from "@/components/shared/SectionShell";
import { serviceDefinitions } from "@/content/services";
import { getPageContext } from "@/i18n/server";
import { localizePath } from "@/i18n/routing";
import { createPageMetadata } from "@/lib/metadata";

interface ServicesPageProps {
  params: Promise<{
    locale: string;
  }>;
}

export async function generateMetadata({
  params,
}: ServicesPageProps): Promise<Metadata> {
  const { locale, messages } = await getPageContext(params);
  return createPageMetadata(locale, messages, "services", "/services");
}

export default async function ServicesPage({ params }: ServicesPageProps) {
  const { locale, messages } = await getPageContext(params);
  const page = messages.pages.services;
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

      <SectionShell>
        <SimpleGrid cols={{ base: 1, md: 2 }} spacing="lg">
          {serviceDefinitions.map((service) => {
            const content = messages.content.services[service.id];

            return (
              <ServiceCard
                audienceTags={service.audienceIds.map(
                  (audienceId) => messages.content.audiences[audienceId],
                )}
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
      </SectionShell>

      <SectionShell
        description={page.outcomes.description}
        eyebrow={page.outcomes.eyebrow}
        title={page.outcomes.title}
      >
        <Paper bg="var(--panel-surface-strong)" p="xl" radius="md" shadow="sm" withBorder>
          <List spacing="sm">
            {page.outcomes.items.map((item) => (
              <ListItem key={item}>{item}</ListItem>
            ))}
          </List>
        </Paper>
      </SectionShell>

      <SectionShell
        description={page.customers.description}
        eyebrow={page.customers.eyebrow}
        title={page.customers.title}
      >
        <SimpleGrid cols={{ base: 1, md: 2 }} spacing="lg">
          {Object.values(messages.content.audiences).map((audience) => (
            <Paper
              bg="var(--panel-surface)"
              key={audience}
              p="xl"
              radius="md"
              shadow="sm"
              withBorder
            >
              <Text>{audience}</Text>
            </Paper>
          ))}
        </SimpleGrid>
      </SectionShell>

      <SectionShell
        description={page.models.description}
        eyebrow={page.models.eyebrow}
        title={page.models.title}
      >
        <SimpleGrid cols={{ base: 1, md: 2, lg: 4 }} spacing="lg">
          {Object.values(messages.content.engagementModels).map((model) => (
            <Paper
              bg="var(--panel-surface)"
              key={model}
              p="xl"
              radius="md"
              shadow="sm"
              withBorder
            >
              <Text fw={600}>{model}</Text>
            </Paper>
          ))}
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
