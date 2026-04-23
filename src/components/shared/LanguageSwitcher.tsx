"use client";

import { useRouter } from "next/navigation";
import { Button, Menu } from "@mantine/core";
import { IconCheck, IconWorld } from "@tabler/icons-react";

import type { Locale } from "@/i18n/config";
import { replaceLocaleInPathname } from "@/i18n/routing";

interface LanguageSwitcherProps {
  locale: Locale;
  pathname: string;
  content: {
    label: string;
    options: {
      en: string;
      da: string;
    };
  };
}

export function LanguageSwitcher({
  locale,
  pathname,
  content,
}: LanguageSwitcherProps) {
  const router = useRouter();

  function handleChange(nextLocale: Locale) {
    router.push(replaceLocaleInPathname(pathname, nextLocale));
  }

  return (
    <Menu shadow="md" width={180}>
      <Menu.Target>
        <Button
          leftSection={<IconWorld size={16} />}
          radius="xl"
          variant="default"
        >
          {content.options[locale]}
        </Button>
      </Menu.Target>

      <Menu.Dropdown>
        <Menu.Label>{content.label}</Menu.Label>
        {(["en", "da"] as const).map((option) => (
          <Menu.Item
            key={option}
            leftSection={locale === option ? <IconCheck size={14} /> : null}
            onClick={() => handleChange(option)}
          >
            {content.options[option]}
          </Menu.Item>
        ))}
      </Menu.Dropdown>
    </Menu>
  );
}
