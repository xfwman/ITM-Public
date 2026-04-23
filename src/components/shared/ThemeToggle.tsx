"use client";

import { ActionIcon, Tooltip, useComputedColorScheme, useMantineColorScheme } from "@mantine/core";
import { IconMoonStars, IconSunHigh } from "@tabler/icons-react";

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
  const colorScheme = useComputedColorScheme("light", {
    getInitialValueInEffect: true,
  });
  const nextColorScheme = colorScheme === "dark" ? "light" : "dark";

  return (
    <Tooltip
      label={nextColorScheme === "dark" ? content.dark : content.light}
      withArrow
    >
      <ActionIcon
        aria-label={content.toggle}
        className={className}
        onClick={() => setColorScheme(nextColorScheme)}
        radius="md"
        size="lg"
        variant="subtle"
      >
        {colorScheme === "dark" ? (
          <IconSunHigh size={18} />
        ) : (
          <IconMoonStars size={18} />
        )}
      </ActionIcon>
    </Tooltip>
  );
}
