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
  Stack,
  Text,
  ThemeIcon,
  Title,
} from "@mantine/core";
import { IconArrowRight, IconCheck } from "@tabler/icons-react";

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
  panelTitle: string;
  panelItems: readonly string[];
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
  panelTitle,
  panelItems,
}: HeroSectionProps) {
  return (
    <Paper className={classes.heroCard} p={{ base: "xl", md: "2rem" }} radius="xl">
      <Grid align="center" gap="xl" pos="relative">
        <GridCol span={{ base: 12, md: 7 }}>
          <Stack gap="lg">
            <Badge color="brand" variant="light">
              {eyebrow}
            </Badge>

            <Stack gap="md">
              <Title order={1}>{title}</Title>
              <Text c="dimmed" maw={680} size="lg">
                {description}
              </Text>
            </Stack>

            <Group>
              <Link href={primaryHref} style={{ textDecoration: "none" }}>
                <Button
                  component="span"
                  rightSection={<IconArrowRight size={18} />}
                  size="md"
                >
                  {primaryLabel}
                </Button>
              </Link>
              <Link href={secondaryHref} style={{ textDecoration: "none" }}>
                <Button component="span" size="md" variant="default">
                  {secondaryLabel}
                </Button>
              </Link>
            </Group>

            <List
              icon={
                <ThemeIcon color="brand" radius="xl" size={24} variant="light">
                  <IconCheck size={14} />
                </ThemeIcon>
              }
              spacing="sm"
            >
              {highlights.map((highlight) => (
                <ListItem key={highlight}>{highlight}</ListItem>
              ))}
            </List>
          </Stack>
        </GridCol>

        <GridCol span={{ base: 12, md: 5 }}>
          <Paper className={classes.heroPanel} p="xl" radius="xl">
            <Stack gap="md">
              <Text fw={600}>{panelTitle}</Text>
              <List spacing="sm" type="ordered">
                {panelItems.map((item) => (
                  <ListItem key={item}>{item}</ListItem>
                ))}
              </List>
            </Stack>
          </Paper>
        </GridCol>
      </Grid>
    </Paper>
  );
}
