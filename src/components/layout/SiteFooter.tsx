import Link from "next/link";
import {
  Box,
  Container,
  Divider,
  Grid,
  GridCol,
  Stack,
  Text,
  Title,
} from "@mantine/core";

import { footerNavigation } from "@/content/navigation";
import { localizePath } from "@/i18n/routing";
import type { Locale } from "@/i18n/config";
import { siteConfig } from "@/lib/site";

interface SiteFooterProps {
  locale: Locale;
  content: {
    description: string;
    reachOutHeading: string;
    locationLabel: string;
    emailLabel: string;
    sitemapHeading: string;
    rights: string;
    note: string;
  };
  navigation: {
    services: string;
    about: string;
    cases: string;
    contact: string;
    privacy: string;
  };
  brand: {
    name: string;
    strapline: string;
  };
}

export function SiteFooter({
  locale,
  content,
  navigation,
  brand,
}: SiteFooterProps) {
  const navigationLabels = {
    services: navigation.services,
    about: navigation.about,
    cases: navigation.cases,
    contact: navigation.contact,
    privacy: navigation.privacy,
  };

  return (
    <Box component="footer" pb="xl" pt={72}>
      <Container size="xl">
        <Stack gap="xl">
          <Divider opacity={0.3} />
          <Grid gap="xl">
            <GridCol span={{ base: 12, md: 5 }}>
              <Stack gap="sm">
                <Title order={3}>{brand.name}</Title>
                <Text c="dimmed">{brand.strapline}</Text>
                <Text maw={520}>{content.description}</Text>
                <Text c="dimmed" size="sm">
                  {content.note}
                </Text>
              </Stack>
            </GridCol>

            <GridCol span={{ base: 12, sm: 6, md: 3 }}>
              <Stack gap="sm">
                <Text fw={600}>{content.sitemapHeading}</Text>
                {footerNavigation.map((item) => (
                  <Link
                    href={localizePath(locale, item.href)}
                    key={item.key}
                    style={{ textDecoration: "none" }}
                  >
                    <Text c="dimmed" component="span">
                      {navigationLabels[item.key]}
                    </Text>
                  </Link>
                ))}
              </Stack>
            </GridCol>

            <GridCol span={{ base: 12, sm: 6, md: 4 }}>
              <Stack gap="sm">
                <Text fw={600}>{content.reachOutHeading}</Text>
                <Text c="dimmed" size="sm">
                  {content.emailLabel}
                </Text>
                <a href={`mailto:${siteConfig.email}`}>
                  {siteConfig.email}
                </a>
                <Text c="dimmed" mt="sm" size="sm">
                  {content.locationLabel}
                </Text>
                <Text>{siteConfig.location}</Text>
              </Stack>
            </GridCol>
          </Grid>

          <Divider opacity={0.25} />
          <Text c="dimmed" size="sm">
            {new Date().getFullYear()} {brand.name}. {content.rights}
          </Text>
        </Stack>
      </Container>
    </Box>
  );
}
