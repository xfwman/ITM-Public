"use client";

import {
  ActionIcon,
  Group,
  Modal,
  Slider,
  Stack,
  Text,
  Tooltip,
} from "@mantine/core";
import { IconSettings2 } from "@tabler/icons-react";
import { useDisclosure } from "@mantine/hooks";
import { useEffect, useState } from "react";

import {
  ambientObjectControls,
  getAmbientObjectCounts,
  setAmbientObjectCount,
  subscribeAmbientObjectCounts,
  type AmbientObjectCounts,
  type AmbientObjectKey,
} from "@/components/effects/ambientSettings";

interface ThemeToggleProps {
  content: {
    toggle: string;
    count: string;
    objects: Record<AmbientObjectKey, string>;
  };
  className?: string;
}

export function ThemeToggle({ content, className }: ThemeToggleProps) {
  const [opened, { close, open }] = useDisclosure(false);
  const [counts, setCounts] = useState<AmbientObjectCounts>(() => getAmbientObjectCounts());

  useEffect(() => subscribeAmbientObjectCounts(setCounts), []);

  return (
    <>
      <Tooltip label={content.toggle} withArrow>
        <ActionIcon
          aria-label={content.toggle}
          className={className}
          onClick={open}
          radius="md"
          size="lg"
          variant="subtle"
        >
          <IconSettings2 size={18} />
        </ActionIcon>
      </Tooltip>

      <Modal
        centered
        onClose={close}
        opened={opened}
        overlayProps={{
          backgroundOpacity: 0.55,
          blur: 10,
          className: "ambient-cover-barrier",
          color: "#02114b",
        }}
        title={content.toggle}
      >
        <Stack gap="lg">
          {ambientObjectControls.map((control) => (
            <Stack gap={6} key={control.key}>
              <Group justify="space-between" wrap="nowrap">
                <Text fw={600} size="sm">
                  {content.objects[control.key]}
                </Text>
                <Text c="dimmed" ff="monospace" size="sm">
                  {content.count}: {counts[control.key]}
                </Text>
              </Group>
              <Slider
                aria-label={content.objects[control.key]}
                label={(value) => value}
                max={control.max}
                min={control.min}
                onChange={(value) => setAmbientObjectCount(control.key, value)}
                step={control.step}
                value={counts[control.key]}
              />
            </Stack>
          ))}
        </Stack>
      </Modal>
    </>
  );
}
