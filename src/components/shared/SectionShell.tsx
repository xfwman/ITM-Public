import type { ReactNode } from "react";
import { Container, Paper, Stack, Text, Title } from "@mantine/core";

import classes from "@/components/shared/SectionShell.module.css";

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
          <Paper className={classes.intro} p={{ base: "lg", md: "xl" }} radius="md">
            <Stack gap="sm">
              {eyebrow ? (
                <Text className={classes.eyebrow} fw={600} size="sm" tt="uppercase">
                  {eyebrow}
                </Text>
              ) : null}
              {title ? (
                <Title className={classes.title} order={2}>
                  {title}
                </Title>
              ) : null}
              {description ? (
                <Text className={classes.description} maw={760} size="lg">
                  {description}
                </Text>
              ) : null}
            </Stack>
          </Paper>
        ) : null}
        {children}
      </Stack>
    </Container>
  );
}
