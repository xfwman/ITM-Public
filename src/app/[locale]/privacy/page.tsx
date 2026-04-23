import { Paper, Stack, Text, Title } from "@mantine/core";
import type { Metadata } from "next";

import { PageIntro } from "@/components/sections/PageIntro";
import { SectionShell } from "@/components/shared/SectionShell";
import { getPageContext } from "@/i18n/server";
import { createPageMetadata } from "@/lib/metadata";

interface PrivacyPageProps {
  params: Promise<{
    locale: string;
  }>;
}

export async function generateMetadata({
  params,
}: PrivacyPageProps): Promise<Metadata> {
  const { locale, messages } = await getPageContext(params);
  return createPageMetadata(locale, messages, "privacy", "/privacy");
}

export default async function PrivacyPage({ params }: PrivacyPageProps) {
  const { messages } = await getPageContext(params);
  const page = messages.pages.privacy;

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
        <Paper
          bg="var(--panel-surface-strong)"
          className="surface-outer-shadow"
          p={{ base: "xl", md: "2rem" }}
          radius="md"
          withBorder
        >
          <Stack gap="xl">
            {page.sections.map((section) => (
              <Stack gap="sm" key={section.title}>
                <Title order={3}>{section.title}</Title>
                {section.paragraphs.map((paragraph) => (
                  <Text c="dimmed" key={paragraph}>
                    {paragraph}
                  </Text>
                ))}
              </Stack>
            ))}
          </Stack>
        </Paper>
      </SectionShell>
    </>
  );
}
