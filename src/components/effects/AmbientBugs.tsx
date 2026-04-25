"use client";

import { useEffect, useRef } from "react";

import classes from "@/components/effects/AmbientBugs.module.css";

interface BugFieldInstance {
  frame(timestamp: number): void;
  resize(
    viewportWidth: number,
    viewportHeight: number,
    pageWidth: number,
    pageHeight: number,
    dpr: number,
  ): void;
  set_cover_regions(regions: Float32Array): void;
  set_dark_mode(darkMode: boolean): void;
  splat_at(pageX: number, pageY: number): boolean;
  set_viewport(scrollX: number, scrollY: number): void;
  set_obstacles(rects: Float32Array): void;
}

interface AmbientBugsModule {
  default(options?: {
    module_or_path?: string;
  }): Promise<unknown>;
  BugField: new (canvas: HTMLCanvasElement, seed: number) => BugFieldInstance;
}

const enum CoverRegionKind {
  Shelter = 0,
  Barrier = 1,
  Canopy = 2,
}

interface CoverRegionSource {
  selector: string;
  kind: CoverRegionKind;
  priority: number;
  minimumWidth?: number;
  minimumHeight?: number;
}

const COVER_REGION_SOURCES: readonly CoverRegionSource[] = [
  {
    selector: "[data-ambient-cover='barrier'], .ambient-cover-barrier",
    kind: CoverRegionKind.Barrier,
    priority: 90,
  },
  {
    selector: "[data-ambient-cover='shelter']",
    kind: CoverRegionKind.Shelter,
    priority: 80,
  },
  {
    selector: "[data-ambient-cover='canopy']",
    kind: CoverRegionKind.Canopy,
    priority: 70,
  },
  {
    selector: "[data-modal-content='true'], .mantine-Drawer-content, [data-menu-dropdown='true']",
    kind: CoverRegionKind.Shelter,
    priority: 60,
  },
  {
    selector: "main .mantine-Paper-root, main .mantine-Card-root",
    kind: CoverRegionKind.Shelter,
    priority: 10,
  },
];

function isRenderableCoverRegion(
  element: HTMLElement,
  rect: DOMRect,
  minimumWidth: number,
  minimumHeight: number,
) {
  if (rect.width < minimumWidth || rect.height < minimumHeight) {
    return false;
  }

  const styles = window.getComputedStyle(element);
  return styles.display !== "none" && styles.visibility !== "hidden" && styles.opacity !== "0";
}

function collectCoverRegionEntries() {
  const entries = new Map<
    HTMLElement,
    {
      source: CoverRegionSource;
    }
  >();

  for (const source of COVER_REGION_SOURCES) {
    for (const element of document.querySelectorAll<HTMLElement>(source.selector)) {
      const existing = entries.get(element);
      if (!existing || source.priority >= existing.source.priority) {
        entries.set(element, { source });
      }
    }
  }

  return entries;
}

function collectCoverRegions(): number[] {
  const regions: number[] = [];
  const pageX = window.scrollX;
  const pageY = window.scrollY;

  for (const [element, { source }] of collectCoverRegionEntries()) {
    const rect = element.getBoundingClientRect();

    if (
      !isRenderableCoverRegion(
        element,
        rect,
        source.minimumWidth ?? 48,
        source.minimumHeight ?? 32,
      )
    ) {
      continue;
    }

    regions.push(
      source.kind,
      rect.left + pageX,
      rect.top + pageY,
      rect.width,
      rect.height,
    );
  }

  return regions;
}

function pointIsCovered(pageX: number, pageY: number, regions: ArrayLike<number>) {
  for (let index = 0; index <= regions.length - 5; index += 5) {
    const x = regions[index + 1];
    const y = regions[index + 2];
    const width = regions[index + 3];
    const height = regions[index + 4];

    if (pageX >= x && pageX <= x + width && pageY >= y && pageY <= y + height) {
      return true;
    }
  }

  return false;
}

function isInteractiveTarget(target: EventTarget | null) {
  if (!(target instanceof Element)) {
    return false;
  }

  return Boolean(
    target.closest(
      [
        "a",
        "button",
        "input",
        "select",
        "textarea",
        "label",
        "summary",
        "[role='button']",
        "[role='link']",
        "[role='menuitem']",
        "[data-ambient-cover]",
      ].join(", "),
    ),
  );
}

function getPageMetrics() {
  const root = document.documentElement;
  const body = document.body;

  return {
    viewportWidth: window.innerWidth,
    viewportHeight: window.innerHeight,
    pageWidth: Math.max(
      root.scrollWidth,
      root.clientWidth,
      body?.scrollWidth ?? 0,
      body?.clientWidth ?? 0,
      window.innerWidth,
    ),
    pageHeight: Math.max(
      root.scrollHeight,
      root.clientHeight,
      body?.scrollHeight ?? 0,
      body?.clientHeight ?? 0,
      window.innerHeight,
    ),
    scrollX: window.scrollX,
    scrollY: window.scrollY,
  };
}

function loadAmbientBugsModule() {
  return import("@/generated/ambient-bugs/ambient_bugs.js") as Promise<AmbientBugsModule>;
}

export function AmbientBugs() {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);

  useEffect(() => {
    if (window.matchMedia("(prefers-reduced-motion: reduce)").matches) {
      return undefined;
    }

    let cancelled = false;
    let frameId = 0;
    let viewportDirty = true;
    let coverDirty = true;
    let boundsDirty = true;
    let scene: BugFieldInstance | null = null;
    let coverRegions = new Float32Array();
    let resizeObserver: ResizeObserver | null = null;
    let mutationObserver: MutationObserver | null = null;

    const canvas = canvasRef.current;
    if (!canvas) {
      return undefined;
    }

    function syncColorScheme() {
      scene?.set_dark_mode(
        document.documentElement.getAttribute("data-mantine-color-scheme") === "dark",
      );
    }

    function syncBounds() {
      const { pageHeight, pageWidth, viewportHeight, viewportWidth } = getPageMetrics();

      scene?.resize(
        viewportWidth,
        viewportHeight,
        pageWidth,
        pageHeight,
        window.devicePixelRatio || 1,
      );

      boundsDirty = false;
      viewportDirty = true;
    }

    function syncViewport() {
      const { scrollX, scrollY } = getPageMetrics();
      scene?.set_viewport(scrollX, scrollY);
      viewportDirty = false;
    }

    function syncCoverWorld() {
      coverRegions = new Float32Array(collectCoverRegions());
      scene?.set_cover_regions(coverRegions);
      coverDirty = false;
    }

    async function start() {
      try {
        const bugsModule = await loadAmbientBugsModule();
        await bugsModule.default({
          module_or_path: "/ambient-bugs/ambient_bugs_bg.wasm",
        });

        if (cancelled) {
          return;
        }

        scene = new bugsModule.BugField(
          canvas as HTMLCanvasElement,
          Math.floor(Math.random() * 2 ** 31),
        );
        syncColorScheme();
        syncBounds();
        syncViewport();
        syncCoverWorld();

        const step = (timestamp: number) => {
          if (cancelled || !scene) {
            return;
          }

          if (boundsDirty) {
            syncBounds();
          }

          if (viewportDirty) {
            syncViewport();
          }

          if (coverDirty) {
            syncCoverWorld();
          }

          scene.frame(timestamp);
          frameId = window.requestAnimationFrame(step);
        };

        frameId = window.requestAnimationFrame(step);
      } catch (error) {
        console.error("Failed to start ambient bug animation", error);
      }
    }

    const handleResize = () => {
      boundsDirty = true;
      coverDirty = true;
    };

    const handleScroll = () => {
      viewportDirty = true;
    };

    const handleWorldClick = (event: MouseEvent) => {
      if (event.defaultPrevented || event.button !== 0 || !scene) {
        return;
      }

      if (isInteractiveTarget(event.target)) {
        return;
      }

      if (boundsDirty) {
        syncBounds();
      }

      if (viewportDirty) {
        syncViewport();
      }

      if (coverDirty) {
        syncCoverWorld();
      }

      const pageX = event.clientX + window.scrollX;
      const pageY = event.clientY + window.scrollY;

      if (pointIsCovered(pageX, pageY, coverRegions)) {
        return;
      }

      scene.splat_at(pageX, pageY);
    };

    window.addEventListener("resize", handleResize, { passive: true });
    window.addEventListener("scroll", handleScroll, { passive: true });
    window.addEventListener("click", handleWorldClick, true);

    resizeObserver = new ResizeObserver(() => {
      boundsDirty = true;
      viewportDirty = true;
      coverDirty = true;
    });

    resizeObserver.observe(document.documentElement);
    resizeObserver.observe(document.body);

    for (const element of collectCoverRegionEntries().keys()) {
      resizeObserver.observe(element);
    }

    mutationObserver = new MutationObserver(() => {
      boundsDirty = true;
      viewportDirty = true;
      coverDirty = true;
      syncColorScheme();

      resizeObserver?.disconnect();
      resizeObserver?.observe(document.documentElement);
      resizeObserver?.observe(document.body);
      for (const element of collectCoverRegionEntries().keys()) {
        resizeObserver?.observe(element);
      }
    });

    mutationObserver.observe(document.body, {
      childList: true,
      subtree: true,
    });
    mutationObserver.observe(document.documentElement, {
      attributes: true,
      attributeFilter: ["data-mantine-color-scheme"],
    });

    void start();

    return () => {
      cancelled = true;
      window.cancelAnimationFrame(frameId);
      window.removeEventListener("resize", handleResize);
      window.removeEventListener("scroll", handleScroll);
      window.removeEventListener("click", handleWorldClick, true);
      resizeObserver?.disconnect();
      mutationObserver?.disconnect();
    };
  }, []);

  return <canvas aria-hidden="true" className={classes.canvas} ref={canvasRef} />;
}
