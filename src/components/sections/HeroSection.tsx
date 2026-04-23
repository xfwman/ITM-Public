import Link from "next/link";
import {
  Badge,
  Button,
  Grid,
  GridCol,
  Group,
  List,
  ListItem,
  Paper,
  SimpleGrid,
  Stack,
  Text,
  ThemeIcon,
  Title,
} from "@mantine/core";
import { IconArrowRight, IconArrowUpRight, IconCheck } from "@tabler/icons-react";

import classes from "@/components/sections/HeroSection.module.css";

interface HeroSectionProps {
  eyebrow: string;
  title: string;
  description: string;
  primaryLabel: string;
  primaryHref: string;
  secondaryLabel: string;
  secondaryHref: string;
  highlights: readonly string[];
  metrics: readonly {
    value: string;
    label: string;
  }[];
  panelTitle: string;
  panelItems: readonly string[];
  spotlights: readonly {
    label: string;
    title: string;
    description: string;
    href: string;
  }[];
}

export function HeroSection({
  eyebrow,
  title,
  description,
  primaryLabel,
  primaryHref,
  secondaryLabel,
  secondaryHref,
  highlights,
  metrics,
  panelTitle,
  panelItems,
  spotlights,
}: HeroSectionProps) {
  return (
    <Paper className={classes.heroCard} p={{ base: "xl", md: "2rem" }} radius="md">
      <Grid align="center" gap="xl" pos="relative">
        <GridCol span={{ base: 12, md: 7 }}>
          <Stack gap="lg">
            <Badge color="sand" variant="filled">
              {eyebrow}
            </Badge>

            <Stack gap="md">
              <Title c="white" order={1}>
                {title}
              </Title>
              <Text className={classes.description} maw={680} size="lg">
                {description}
              </Text>
            </Stack>

            <Group>
              <Link href={primaryHref} style={{ textDecoration: "none" }}>
                <Button
                  className={classes.primaryButton}
                  color="sand"
                  component="span"
                  rightSection={<IconArrowRight size={18} />}
                  size="md"
                >
                  {primaryLabel}
                </Button>
              </Link>
              <Link href={secondaryHref} style={{ textDecoration: "none" }}>
                <Button
                  className={classes.secondaryButton}
                  component="span"
                  size="md"
                  variant="default"
                >
                  {secondaryLabel}
                </Button>
              </Link>
            </Group>

            <List
              icon={
                <ThemeIcon color="sand" radius="md" size={24} variant="light">
                  <IconCheck size={14} />
                </ThemeIcon>
              }
              spacing="sm"
            >
              {highlights.map((highlight) => (
                <ListItem c="rgba(239, 243, 247, 0.84)" key={highlight}>
                  {highlight}
                </ListItem>
              ))}
            </List>

            <SimpleGrid cols={{ base: 1, sm: 3 }} spacing="sm">
              {metrics.map((metric) => (
                <Paper className={classes.metricCard} key={metric.label} p="md" radius="md">
                  <Stack gap={4}>
                    <Text className={classes.metricValue}>{metric.value}</Text>
                    <Text c="rgba(239, 243, 247, 0.6)" size="sm">
                      {metric.label}
                    </Text>
                  </Stack>
                </Paper>
              ))}
            </SimpleGrid>
          </Stack>
        </GridCol>

        <GridCol span={{ base: 12, md: 5 }}>
          <Stack gap="md">
            <Paper className={classes.heroPanel} p="xl" radius="md">
              <Stack gap="md">
                <Text c="white" fw={600}>
                  {panelTitle}
                </Text>
                <List c="rgba(239, 243, 247, 0.78)" spacing="sm" type="ordered">
                  {panelItems.map((item) => (
                    <ListItem key={item}>{item}</ListItem>
                  ))}
                </List>
              </Stack>
            </Paper>

            <SimpleGrid cols={{ base: 1, sm: 2, md: 1 }} spacing="md">
              {spotlights.map((spotlight) => (
                <Link
                  className={classes.spotlightLink}
                  href={spotlight.href}
                  key={spotlight.title}
                >
                  <Paper className={classes.spotlightCard} p="lg" radius="md">
                    <Stack gap="sm">
                      <Group justify="space-between" wrap="nowrap">
                        <Text className={classes.spotlightLabel}>
                          {spotlight.label}
                        </Text>
                        <ThemeIcon color="sand" radius="md" size={30} variant="light">
                          <IconArrowUpRight size={16} />
                        </ThemeIcon>
                      </Group>
                      <Text className={classes.spotlightTitle} size="lg">
                        {spotlight.title}
                      </Text>
                      <Text c="rgba(239, 243, 247, 0.68)" size="sm">
                        {spotlight.description}
                      </Text>
                    </Stack>
                  </Paper>
                </Link>
              ))}
            </SimpleGrid>
          </Stack>
        </GridCol>
      </Grid>
    </Paper>
  );
}
