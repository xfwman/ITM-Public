import {
  Grid,
  GridCol,
  Paper,
  SimpleGrid,
  Stack,
  Text,
  ThemeIcon,
} from "@mantine/core";
import { IconArrowRight } from "@tabler/icons-react";
import type { Metadata } from "next";

import { CtaSection } from "@/components/sections/CtaSection";
import { PageIntro } from "@/components/sections/PageIntro";
import { SectionShell } from "@/components/shared/SectionShell";
import { getPageContext } from "@/i18n/server";
import { localizePath } from "@/i18n/routing";
import { createPageMetadata } from "@/lib/metadata";

interface AboutPageProps {
  params: Promise<{
    locale: string;
  }>;
}

export async function generateMetadata({
  params,
}: AboutPageProps): Promise<Metadata> {
  const { locale, messages } = await getPageContext(params);
  return createPageMetadata(locale, messages, "about", "/about");
}

export default async function AboutPage({ params }: AboutPageProps) {
  const { locale, messages } = await getPageContext(params);
  const page = messages.pages.about;

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
        description={page.story.paragraphs[0]}
        eyebrow={page.story.eyebrow}
        title={page.story.title}
      >
        <Grid gap="xl">
          <GridCol span={{ base: 12, md: 7 }}>
            <Stack gap="lg">
              {page.story.paragraphs.map((paragraph) => (
                <Text key={paragraph} size="lg">
                  {paragraph}
                </Text>
              ))}
            </Stack>
          </GridCol>
          <GridCol span={{ base: 12, md: 5 }}>
            <Paper bg="var(--panel-surface-strong)" p="xl" radius="md" shadow="sm" withBorder>
              <Stack gap="sm">
                <Text c="brand.6" fw={600} size="sm" tt="uppercase">
                  {page.profile.eyebrow}
                </Text>
                <Text fw={700} size="xl">
                  {page.profile.title}
                </Text>
                <Text c="dimmed">{page.profile.role}</Text>
                <Text>{page.profile.summary}</Text>
                <Stack gap="sm" mt="sm">
                  {page.profile.focusAreas.map((item) => (
                    <Paper
                      bg="var(--panel-surface)"
                      key={item}
                      p="sm"
                      radius="sm"
                      withBorder
                    >
                      <Text size="sm">{item}</Text>
                    </Paper>
                  ))}
                </Stack>
              </Stack>
            </Paper>
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
              bg="var(--panel-surface)"
              key={item.title}
              p="xl"
              radius="md"
              shadow="sm"
              withBorder
            >
              <Stack gap="sm">
                <ThemeIcon color="brand" radius="md" size={42} variant="light">
                  <IconArrowRight size={18} />
                </ThemeIcon>
                <Text fw={600} size="lg">
                  {item.title}
                </Text>
                <Text c="dimmed">{item.text}</Text>
              </Stack>
            </Paper>
          ))}
        </SimpleGrid>
      </SectionShell>

      <SectionShell
        description={page.experience.description}
        eyebrow={page.experience.eyebrow}
        title={page.experience.title}
      >
        <SimpleGrid cols={{ base: 1, md: 3 }} spacing="lg">
          {page.experience.stats.map((item) => (
            <Paper
              bg="var(--panel-surface)"
              key={item.label}
              p="xl"
              radius="md"
              shadow="sm"
              withBorder
            >
              <Stack gap="xs">
                <Text fw={700} size="xl">
                  {item.value}
                </Text>
                <Text c="dimmed">{item.label}</Text>
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
          title={page.cta.title}
        />
      </SectionShell>
    </>
  );
}
