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

interface ServiceCardProps {
  icon: TablerIcon;
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
    <Card
      bg="var(--panel-surface)"
      h="100%"
      shadow="sm"
      withBorder
    >
      <Stack gap="md" h="100%">
        <ThemeIcon color="brand" radius="xl" size={42} variant="light">
          <Icon size={20} />
        </ThemeIcon>

        <Stack gap={6}>
          <Text fw={600} size="xl">
            {title}
          </Text>
          <Text c="dimmed">{summary}</Text>
        </Stack>

        {!compact ? <Text size="sm">{description}</Text> : null}

        <Group gap="xs">
          {audienceTags.map((item) => (
            <Badge color="brand" key={item} variant="light">
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
            <Text fw={600} size="sm">
              {labels.deliverables}
            </Text>
          ) : null}
          {compact ? (
            <List spacing="xs" size="sm">
              {outcomes.slice(0, 2).map((item) => (
                <ListItem key={item}>{item}</ListItem>
              ))}
            </List>
          ) : (
            <List spacing="xs" size="sm">
              {deliverables.map((item) => (
                <ListItem key={item}>{item}</ListItem>
              ))}
            </List>
          )}
        </Stack>

        {!compact ? (
          <Stack gap="xs">
            <Text fw={600} size="sm">
              {labels.outcomes}
            </Text>
            <List spacing="xs" size="sm">
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
