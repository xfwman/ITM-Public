import { Card, Group, List, ListItem, Stack, Text, ThemeIcon } from "@mantine/core";
import { IconArrowRight } from "@tabler/icons-react";
import type { TablerIcon } from "@tabler/icons-react";

import classes from "@/components/sections/CaseCard.module.css";

interface CaseCardProps {
  icon: TablerIcon;
  sectionLabel: string;
  title: string;
  summary: string;
  context: string;
  challenge: string;
  approach: string;
  outcome: string;
  labels: {
    context: string;
    challenge: string;
    approach: string;
    outcome: string;
  };
  compact?: boolean;
}

export function CaseCard({
  icon: Icon,
  sectionLabel,
  title,
  summary,
  context,
  challenge,
  approach,
  outcome,
  labels,
  compact = false,
}: CaseCardProps) {
  return (
    <Card className={classes.card} h="100%" shadow="sm" withBorder>
      <Stack gap="md" h="100%">
        <Group justify="space-between" wrap="nowrap">
          <Stack gap={6}>
            <Text className={classes.label}>{sectionLabel}</Text>
            <ThemeIcon color="brand" radius="xl" size={42} variant="light">
              <Icon size={20} />
            </ThemeIcon>
          </Stack>
          <ThemeIcon color="sand" radius="xl" size={34} variant="light">
            <IconArrowRight size={16} />
          </ThemeIcon>
        </Group>

        <Stack gap={6}>
          <Text className={classes.title} fw={600} size="xl">
            {title}
          </Text>
          <Text c="dimmed">{summary}</Text>
        </Stack>

        {compact ? (
          <List className={classes.compactList} spacing="xs" size="sm">
            <ListItem>{outcome}</ListItem>
            <ListItem>{approach}</ListItem>
          </List>
        ) : (
          <Stack gap="sm">
            <Text className={classes.detailRow} size="sm">
              <Text component="span" fw={600}>
                {labels.context}:
              </Text>{" "}
              {context}
            </Text>
            <Text className={classes.detailRow} size="sm">
              <Text component="span" fw={600}>
                {labels.challenge}:
              </Text>{" "}
              {challenge}
            </Text>
            <Text className={classes.detailRow} size="sm">
              <Text component="span" fw={600}>
                {labels.approach}:
              </Text>{" "}
              {approach}
            </Text>
            <Text className={classes.detailRow} size="sm">
              <Text component="span" fw={600}>
                {labels.outcome}:
              </Text>{" "}
              {outcome}
            </Text>
          </Stack>
        )}
      </Stack>
    </Card>
  );
}
