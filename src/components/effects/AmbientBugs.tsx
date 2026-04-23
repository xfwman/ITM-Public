"use client";

import { useEffect, useRef } from "react";

import classes from "@/components/effects/AmbientBugs.module.css";

interface BugFieldInstance {
  frame(timestamp: number): void;
  resize(width: number, height: number, dpr: number): void;
  set_dark_mode(darkMode: boolean): void;
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
  const viewportWidth = window.innerWidth;
  const viewportHeight = window.innerHeight;

  for (const element of document.querySelectorAll<HTMLElement>(OBSTACLE_SELECTOR)) {
    const rect = element.getBoundingClientRect();

    if (rect.width < 48 || rect.height < 32) {
      continue;
    }

    if (rect.bottom < -80 || rect.top > viewportHeight + 80) {
      continue;
    }

    if (rect.right < -80 || rect.left > viewportWidth + 80) {
      continue;
    }

    rects.push(rect.left, rect.top, rect.width, rect.height);
  }

  return rects;
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
    let obstacleDirty = true;
    let resizeDirty = true;
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

    function syncViewport() {
      scene?.resize(window.innerWidth, window.innerHeight, window.devicePixelRatio || 1);
      resizeDirty = false;
      obstacleDirty = true;
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
        syncViewport();
        syncObstacles();

        const step = (timestamp: number) => {
          if (cancelled || !scene) {
            return;
          }

          if (resizeDirty) {
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
      resizeDirty = true;
    };

    const handleScroll = () => {
      obstacleDirty = true;
    };

    window.addEventListener("resize", handleResize, { passive: true });
    window.addEventListener("scroll", handleScroll, { passive: true });

    resizeObserver = new ResizeObserver(() => {
      obstacleDirty = true;
    });

    for (const element of document.querySelectorAll<HTMLElement>(OBSTACLE_SELECTOR)) {
      resizeObserver.observe(element);
    }

    mutationObserver = new MutationObserver(() => {
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
