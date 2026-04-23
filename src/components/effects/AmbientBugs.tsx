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
  set_dark_mode(darkMode: boolean): void;
  set_viewport(scrollX: number, scrollY: number): void;
  set_obstacles(rects: Float32Array): void;
}

interface AmbientBugsModule {
  default(options?: {
    module_or_path?: string;
  }): Promise<unknown>;
  BugField: new (canvas: HTMLCanvasElement, seed: number) => BugFieldInstance;
}

const OBSTACLE_SELECTOR = "main .mantine-Paper-root, main .mantine-Card-root";

function collectObstacleRects(): number[] {
  const rects: number[] = [];
  const pageX = window.scrollX;
  const pageY = window.scrollY;

  for (const element of document.querySelectorAll<HTMLElement>(OBSTACLE_SELECTOR)) {
    const rect = element.getBoundingClientRect();

    if (rect.width < 48 || rect.height < 32) {
      continue;
    }

    rects.push(rect.left + pageX, rect.top + pageY, rect.width, rect.height);
  }

  return rects;
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
    let obstacleDirty = true;
    let boundsDirty = true;
    let scene: BugFieldInstance | null = null;
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

    function syncObstacles() {
      scene?.set_obstacles(new Float32Array(collectObstacleRects()));
      obstacleDirty = false;
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
        syncObstacles();

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

          if (obstacleDirty) {
            syncObstacles();
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
      obstacleDirty = true;
    };

    const handleScroll = () => {
      viewportDirty = true;
    };

    window.addEventListener("resize", handleResize, { passive: true });
    window.addEventListener("scroll", handleScroll, { passive: true });

    resizeObserver = new ResizeObserver(() => {
      boundsDirty = true;
      viewportDirty = true;
      obstacleDirty = true;
    });

    resizeObserver.observe(document.documentElement);
    resizeObserver.observe(document.body);

    for (const element of document.querySelectorAll<HTMLElement>(OBSTACLE_SELECTOR)) {
      resizeObserver.observe(element);
    }

    mutationObserver = new MutationObserver(() => {
      boundsDirty = true;
      viewportDirty = true;
      obstacleDirty = true;
      syncColorScheme();

      resizeObserver?.disconnect();
      for (const element of document.querySelectorAll<HTMLElement>(OBSTACLE_SELECTOR)) {
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
      resizeObserver?.disconnect();
      mutationObserver?.disconnect();
    };
  }, []);

  return <canvas aria-hidden="true" className={classes.canvas} ref={canvasRef} />;
}
