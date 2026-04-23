import { Badge, Paper, Stack, Text, Title } from "@mantine/core";

import classes from "@/components/sections/PageIntro.module.css";

interface PageIntroProps {
  eyebrow: string;
  title: string;
  description: string;
}

export function PageIntro({ eyebrow, title, description }: PageIntroProps) {
  return (
    <Paper className={classes.panel} p={{ base: "xl", md: "2rem" }} radius="xl">
      <Stack gap="lg" pos="relative">
        <Badge color="brand" variant="filled">
          {eyebrow}
        </Badge>
        <Stack gap="md">
          <Title c="white" order={1}>
            {title}
          </Title>
          <Text c="rgba(239, 243, 247, 0.74)" maw={760} size="lg">
            {description}
          </Text>
        </Stack>
      </Stack>
    </Paper>
  );
}
