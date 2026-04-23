import {
  Badge,
  Card,
  Group,
  List,
  ListItem,
  Stack,
  Text,
  ThemeIcon,
} from "@mantine/core";
import type { TablerIcon } from "@tabler/icons-react";

import classes from "@/components/sections/ServiceCard.module.css";

interface ServiceCardProps {
  icon: TablerIcon;
  sectionLabel: string;
  title: string;
  summary: string;
  description: string;
  deliverables: readonly string[];
  outcomes: readonly string[];
  audienceTags: readonly string[];
  engagementTags: readonly string[];
  labels: {
    deliverables: string;
    outcomes: string;
  };
  compact?: boolean;
}

export function ServiceCard({
  icon: Icon,
  sectionLabel,
  title,
  summary,
  description,
  deliverables,
  outcomes,
  audienceTags,
  engagementTags,
  labels,
  compact = false,
}: ServiceCardProps) {
  return (
    <Card className={classes.card} h="100%" withBorder>
      <Stack gap="md" h="100%">
        <Group justify="space-between" wrap="nowrap">
          <ThemeIcon color="brand" radius="md" size={42} variant="light">
            <Icon size={20} />
          </ThemeIcon>
          <Text className={classes.label}>{sectionLabel}</Text>
        </Group>

        <Stack gap={6}>
          <Text className={classes.title} fw={600} size="xl">
            {title}
          </Text>
          <Text className={classes.summary}>{summary}</Text>
        </Stack>

        {!compact ? (
          <Text className={classes.description} size="sm">
            {description}
          </Text>
        ) : null}

        <Group gap="xs">
          {audienceTags.map((item) => (
            <Badge color="gray" key={item} variant="outline">
              {item}
            </Badge>
          ))}
        </Group>

        <Group gap="xs">
          {engagementTags.map((item) => (
            <Badge color="sand" key={item} variant="light">
              {item}
            </Badge>
          ))}
        </Group>

        <Stack gap="xs">
          {!compact ? (
            <Text className={classes.subheading} fw={600} size="sm">
              {labels.deliverables}
            </Text>
          ) : null}
          {compact ? (
            <List className={classes.list} spacing="xs" size="sm">
              {outcomes.slice(0, 2).map((item) => (
                <ListItem key={item}>{item}</ListItem>
              ))}
            </List>
          ) : (
            <List className={classes.list} spacing="xs" size="sm">
              {deliverables.map((item) => (
                <ListItem key={item}>{item}</ListItem>
              ))}
            </List>
          )}
        </Stack>

        {!compact ? (
          <Stack gap="xs">
            <Text className={classes.subheading} fw={600} size="sm">
              {labels.outcomes}
            </Text>
            <List className={classes.list} spacing="xs" size="sm">
              {outcomes.map((item) => (
                <ListItem key={item}>{item}</ListItem>
              ))}
            </List>
          </Stack>
        ) : null}
      </Stack>
    </Card>
  );
}
