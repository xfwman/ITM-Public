"use client";

import { useEffect, useRef, useState, type CSSProperties } from "react";
import Link from "next/link";
import { usePathname } from "next/navigation";
import {
  Anchor,
  Box,
  Burger,
  Button,
  Container,
  Divider,
  Drawer,
  Group,
  Stack,
  Text,
} from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";

import { primaryNavigation } from "@/data/navigation";
import { replaceLocaleInPathname, stripLocaleFromPathname } from "@/i18n/routing";
import type { Locale } from "@/i18n/config";
import { LanguageSwitcher } from "@/components/shared/LanguageSwitcher";
import { ThemeToggle } from "@/components/shared/ThemeToggle";
import classes from "@/components/layout/SiteHeader.module.css";

interface SiteHeaderProps {
  locale: Locale;
  brand: {
    name: string;
    strapline: string;
  };
  navigation: {
    home: string;
    services: string;
    about: string;
    cases: string;
    contact: string;
    contactCta: string;
    menu: string;
  };
  language: {
    label: string;
    options: {
      en: string;
      da: string;
    };
  };
  theme: {
    toggle: string;
    light: string;
    dark: string;
  };
}

export function SiteHeader({
  locale,
  brand,
  navigation,
  language,
  theme,
}: SiteHeaderProps) {
  const pathname = usePathname();
  const [opened, { close, toggle }] = useDisclosure(false);
  const navTabsRef = useRef<HTMLDivElement | null>(null);
  const [uniformTabWidth, setUniformTabWidth] = useState<number | null>(null);
  const currentPath: string = stripLocaleFromPathname(pathname || `/${locale}`);

  const navigationLabels = {
    services: navigation.services,
    about: navigation.about,
    cases: navigation.cases,
    contact: navigation.contact,
  };

  useEffect(() => {
    function measureTabs() {
      const tabs = navTabsRef.current;

      if (!tabs || window.matchMedia("(max-width: 48em)").matches) {
        setUniformTabWidth(null);
        return;
      }

      const labels = Array.from(
        tabs.querySelectorAll<HTMLElement>("[data-nav-label='true']"),
      );
      const sampleLink = tabs.querySelector<HTMLElement>("[data-tab-link='true']");

      if (!labels.length || !sampleLink) {
        setUniformTabWidth(null);
        return;
      }

      const styles = window.getComputedStyle(sampleLink);
      const horizontalSpacing =
        Number.parseFloat(styles.paddingInlineStart) +
        Number.parseFloat(styles.paddingInlineEnd) +
        Number.parseFloat(styles.borderInlineStartWidth) +
        Number.parseFloat(styles.borderInlineEndWidth);

      const widestLabel = labels.reduce((maxWidth, label) => {
        return Math.max(maxWidth, label.getBoundingClientRect().width);
      }, 0);

      setUniformTabWidth(Math.ceil(widestLabel + horizontalSpacing));
    }

    const resizeObserver = new ResizeObserver(() => {
      measureTabs();
    });

    if (navTabsRef.current) {
      resizeObserver.observe(navTabsRef.current);
    }

    void document.fonts?.ready.then(() => {
      measureTabs();
    });

    measureTabs();
    window.addEventListener("resize", measureTabs, { passive: true });

    return () => {
      resizeObserver.disconnect();
      window.removeEventListener("resize", measureTabs);
    };
  }, [locale, pathname]);

  const navTabsStyle = uniformTabWidth
    ? ({ "--uniform-tab-width": `${uniformTabWidth}px` } as CSSProperties)
    : undefined;

  return (
    <Box className={classes.headerRoot}>
      <Container size="xl">
        <Group h={84} justify="space-between" wrap="nowrap">
          <Anchor
            className={classes.brandLink}
            component={Link}
            href={`/${locale}`}
            underline="never"
          >
            <Text className={classes.brandName}>{brand.name}</Text>
            <Text
              c="dimmed"
              className={classes.brandStrapline}
              size="sm"
              visibleFrom="sm"
            >
              {brand.strapline}
            </Text>
          </Anchor>

          <Group
            className={classes.navTabs}
            gap="xs"
            ref={navTabsRef}
            style={navTabsStyle}
            visibleFrom="md"
            wrap="nowrap"
          >
            {primaryNavigation.map((item) => (
              <Anchor
                className={classes.navLink}
                component={Link}
                data-active={currentPath === item.href || undefined}
                data-tab-link="true"
                href={`/${locale}${item.href}`}
                key={item.key}
                underline="never"
              >
                <span className={classes.navLabel} data-nav-label="true">
                  {navigationLabels[item.key]}
                </span>
              </Anchor>
            ))}
          </Group>

          <Group gap="sm" visibleFrom="md" wrap="nowrap">
            <ThemeToggle className={classes.utilityControl} content={theme} />
            <LanguageSwitcher
              buttonClassName={classes.utilityControl}
              content={language}
              dropdownClassName={classes.menuDropdown}
              locale={locale}
              pathname={pathname || `/${locale}`}
            />
            <Button
              className={classes.ctaButton}
              color="sand"
              component={Link}
              href={`/${locale}/contact`}
              variant="filled"
            >
              {navigation.contactCta}
            </Button>
          </Group>

          <Group gap="xs" hiddenFrom="md" wrap="nowrap">
            <ThemeToggle className={classes.utilityControl} content={theme} />
            <Burger
              aria-label={navigation.menu}
              className={classes.burger}
              color="white"
              onClick={toggle}
              opened={opened}
              size="sm"
            />
          </Group>
        </Group>
      </Container>

      <Drawer
        classNames={{
          content: classes.drawerContent,
        }}
        onClose={close}
        overlayProps={{
          backgroundOpacity: 0.55,
          blur: 6,
          color: "#02114b",
        }}
        opened={opened}
        padding="lg"
        position="right"
        title={<Text className={classes.drawerTitle}>{brand.name}</Text>}
      >
        <Stack gap="lg">
          <Anchor
            className={classes.navLink}
            component={Link}
            data-active={currentPath === "/" || undefined}
            href={`/${locale}`}
            onClick={close}
            underline="never"
          >
            <span className={classes.navLabel}>{navigation.home}</span>
          </Anchor>

          {primaryNavigation.map((item) => (
            <Anchor
              className={classes.navLink}
              component={Link}
              data-active={currentPath === item.href || undefined}
              href={replaceLocaleInPathname(item.href, locale)}
              key={item.key}
              onClick={close}
              underline="never"
            >
              <span className={classes.navLabel}>{navigationLabels[item.key]}</span>
            </Anchor>
          ))}

          <Divider opacity={0.28} />

          <Group grow>
            <LanguageSwitcher
              buttonClassName={classes.utilityControl}
              content={language}
              dropdownClassName={classes.menuDropdown}
              locale={locale}
              pathname={pathname || `/${locale}`}
            />
          </Group>

          <Button
            className={classes.ctaButton}
            color="sand"
            component={Link}
            href={`/${locale}/contact`}
            onClick={close}
          >
            {navigation.contactCta}
          </Button>
        </Stack>
      </Drawer>
    </Box>
  );
}
