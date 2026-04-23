import type { ReactNode } from "react";
import { Container, Stack, Text, Title } from "@mantine/core";

interface SectionShellProps {
  eyebrow?: string;
  title?: string;
  description?: string;
  children: ReactNode;
}

export function SectionShell({
  eyebrow,
  title,
  description,
  children,
}: SectionShellProps) {
  return (
    <Container py={{ base: 52, md: 72 }} size="xl">
      <Stack gap="xl">
        {eyebrow || title || description ? (
          <Stack gap="sm">
            {eyebrow ? (
              <Text c="brand.6" fw={600} size="sm" tt="uppercase">
                {eyebrow}
              </Text>
            ) : null}
            {title ? <Title order={2}>{title}</Title> : null}
            {description ? (
              <Text c="dimmed" maw={760} size="lg">
                {description}
              </Text>
            ) : null}
          </Stack>
        ) : null}
        {children}
      </Stack>
    </Container>
  );
}
