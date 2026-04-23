import { Grid, GridCol, List, ListItem, Paper, Stack, Text } from "@mantine/core";
import { IconCalendarTime, IconMail, IconMapPin } from "@tabler/icons-react";
import type { Metadata } from "next";

import { ContactForm } from "@/components/forms/ContactForm";
import { PageIntro } from "@/components/sections/PageIntro";
import { SectionShell } from "@/components/shared/SectionShell";
import { getPageContext } from "@/i18n/server";
import { createPageMetadata } from "@/lib/metadata";
import { siteConfig } from "@/lib/site";

interface ContactPageProps {
  params: Promise<{
    locale: string;
  }>;
}

export async function generateMetadata({
  params,
}: ContactPageProps): Promise<Metadata> {
  const { locale, messages } = await getPageContext(params);
  return createPageMetadata(locale, messages, "contact", "/contact");
}

export default async function ContactPage({ params }: ContactPageProps) {
  const { messages } = await getPageContext(params);
  const page = messages.pages.contact;

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
        description={page.details.description}
        eyebrow={page.details.eyebrow}
        title={page.details.title}
      >
        <Grid gap="xl">
          <GridCol span={{ base: 12, md: 5 }}>
            <Paper bg="var(--panel-surface-strong)" p="xl" radius="md" shadow="sm" withBorder>
              <Stack gap="lg">
                <Stack gap="sm">
                  <IconMail size={20} />
                  <Text c="dimmed" size="sm">
                    {page.details.emailLabel}
                  </Text>
                  <Text fw={600}>{siteConfig.email}</Text>
                </Stack>
                <Stack gap="sm">
                  <IconMapPin size={20} />
                  <Text c="dimmed" size="sm">
                    {page.details.locationLabel}
                  </Text>
                  <Text fw={600}>{siteConfig.location}</Text>
                </Stack>
                <Stack gap="sm">
                  <IconCalendarTime size={20} />
                  <Text c="dimmed" size="sm">
                    {page.details.responseLabel}
                  </Text>
                  <Text fw={600}>{page.details.responseValue}</Text>
                </Stack>
                <Stack gap="sm">
                  <Text c="dimmed" size="sm">
                    {page.details.bookingLabel}
                  </Text>
                  <Text>{page.details.bookingValue}</Text>
                </Stack>
              </Stack>
            </Paper>
          </GridCol>

          <GridCol span={{ base: 12, md: 7 }}>
            <ContactForm content={page.form} />
          </GridCol>
        </Grid>
      </SectionShell>

      <SectionShell title={page.expectations.title}>
        <Paper bg="var(--panel-surface)" p="xl" radius="md" shadow="sm" withBorder>
          <List spacing="sm">
            {page.expectations.items.map((item) => (
              <ListItem key={item}>{item}</ListItem>
            ))}
          </List>
        </Paper>
      </SectionShell>
    </>
  );
}
