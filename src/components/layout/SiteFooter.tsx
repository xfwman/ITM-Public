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

import { footerNavigation } from "@/data/navigation";
import { localizePath } from "@/i18n/routing";
import type { Locale } from "@/i18n/config";
import { siteConfig } from "@/lib/site";
import classes from "@/components/layout/SiteFooter.module.css";

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
    <Box
      className={classes.root}
      component="footer"
      data-ambient-cover="shelter"
      pb="xl"
      pt={88}
    >
      <Container size="xl">
        <Stack gap="xl" pos="relative">
          <Divider color="rgba(239, 243, 247, 0.08)" />
          <Grid gap="xl">
            <GridCol span={{ base: 12, md: 5 }}>
              <Stack gap="sm">
                <Title order={3}>{brand.name}</Title>
                <Text c="rgba(239, 243, 247, 0.64)">{brand.strapline}</Text>
                <Text maw={520}>{content.description}</Text>
                <Text c="rgba(239, 243, 247, 0.56)" size="sm">
                  {content.note}
                </Text>
              </Stack>
            </GridCol>

            <GridCol span={{ base: 12, sm: 6, md: 3 }}>
              <Stack gap="sm">
                <Text fw={600}>{content.sitemapHeading}</Text>
                {footerNavigation.map((item) => (
                  <Link
                    className={classes.link}
                    href={localizePath(locale, item.href)}
                    key={item.key}
                    style={{ textDecoration: "none" }}
                  >
                    <Text component="span">
                      {navigationLabels[item.key]}
                    </Text>
                  </Link>
                ))}
              </Stack>
            </GridCol>

            <GridCol span={{ base: 12, sm: 6, md: 4 }}>
              <Stack gap="sm">
                <Text fw={600}>{content.reachOutHeading}</Text>
                <Text c="rgba(239, 243, 247, 0.56)" size="sm">
                  {content.emailLabel}
                </Text>
                <a className={classes.link} href={`mailto:${siteConfig.email}`}>
                  {siteConfig.email}
                </a>
                <Text c="rgba(239, 243, 247, 0.56)" mt="sm" size="sm">
                  {content.locationLabel}
                </Text>
                <Text>{siteConfig.location}</Text>
              </Stack>
            </GridCol>
          </Grid>

          <Divider color="rgba(239, 243, 247, 0.08)" />
          <Text c="rgba(239, 243, 247, 0.56)" size="sm">
            {new Date().getFullYear()} {brand.name}. {content.rights}
          </Text>
        </Stack>
      </Container>
    </Box>
  );
}
