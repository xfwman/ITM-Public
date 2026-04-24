"use client";

import {
  ActionIcon,
  Button,
  Modal,
  Stack,
  Tooltip,
  useComputedColorScheme,
  useMantineColorScheme,
} from "@mantine/core";
import {
  IconCheck,
  IconMoonStars,
  IconSettings2,
  IconSunHigh,
} from "@tabler/icons-react";
import { useDisclosure } from "@mantine/hooks";

interface ThemeToggleProps {
  content: {
    toggle: string;
    light: string;
    dark: string;
  };
  className?: string;
}

export function ThemeToggle({ content, className }: ThemeToggleProps) {
  const { setColorScheme } = useMantineColorScheme();
  const [opened, { close, open }] = useDisclosure(false);
  const computedColorScheme = useComputedColorScheme("light", {
    getInitialValueInEffect: true,
  });

  function handleSelect(nextColorScheme: "light" | "dark") {
    setColorScheme(nextColorScheme);
    close();
  }

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
          color: "#02114b",
        }}
        title={content.toggle}
      >
        <Stack gap="sm">
          <Button
            fullWidth
            justify="space-between"
            leftSection={
              computedColorScheme === "light" ? <IconCheck size={16} /> : <IconSunHigh size={16} />
            }
            onClick={() => handleSelect("light")}
            variant={computedColorScheme === "light" ? "filled" : "light"}
          >
            {content.light}
          </Button>
          <Button
            fullWidth
            justify="space-between"
            leftSection={
              computedColorScheme === "dark" ? <IconCheck size={16} /> : <IconMoonStars size={16} />
            }
            onClick={() => handleSelect("dark")}
            variant={computedColorScheme === "dark" ? "filled" : "light"}
          >
            {content.dark}
          </Button>
        </Stack>
      </Modal>
    </>
  );
}
