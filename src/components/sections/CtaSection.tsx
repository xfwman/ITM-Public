import Link from "next/link";
import {
  Button,
  Group,
  Paper,
  Stack,
  Text,
  Title,
} from "@mantine/core";
import { IconArrowRight } from "@tabler/icons-react";

import classes from "@/components/sections/CtaSection.module.css";

interface CtaSectionProps {
  title: string;
  description: string;
  primaryLabel: string;
  primaryHref: string;
  secondaryLabel?: string;
  secondaryHref?: string;
}

export function CtaSection({
  title,
  description,
  primaryLabel,
  primaryHref,
  secondaryLabel,
  secondaryHref,
}: CtaSectionProps) {
  return (
    <Paper className={classes.panel} p={{ base: "xl", md: "2rem" }} radius="xl">
      <Stack gap="lg" pos="relative">
        <Stack gap="sm">
          <Title c="white" order={2}>
            {title}
          </Title>
          <Text c="rgba(255, 255, 255, 0.82)" maw={700}>
            {description}
          </Text>
        </Stack>

        <Group>
          <Link href={primaryHref} style={{ textDecoration: "none" }}>
            <Button
              color="sand"
              component="span"
              rightSection={<IconArrowRight size={18} />}
              variant="white"
            >
              {primaryLabel}
            </Button>
          </Link>

          {secondaryLabel && secondaryHref ? (
            <Link href={secondaryHref} style={{ textDecoration: "none" }}>
              <Button color="gray" component="span" variant="transparent">
                {secondaryLabel}
              </Button>
            </Link>
          ) : null}
        </Group>
      </Stack>
    </Paper>
  );
}
