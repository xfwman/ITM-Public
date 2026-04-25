"use client";

export const ambientObjectControls = [
  {
    key: "dandelion",
    min: 0,
    max: 24,
    step: 1,
    defaultValue: 8,
  },
  {
    key: "strawberryPlant",
    min: 0,
    max: 24,
    step: 1,
    defaultValue: 6,
  },
  {
    key: "grassClump",
    min: 0,
    max: 40,
    step: 1,
    defaultValue: 12,
  },
  {
    key: "aphid",
    min: 0,
    max: 30,
    step: 1,
    defaultValue: 8,
  },
  {
    key: "leafBeetle",
    min: 0,
    max: 30,
    step: 1,
    defaultValue: 8,
  },
  {
    key: "groundBeetle",
    min: 0,
    max: 16,
    step: 1,
    defaultValue: 3,
  },
  {
    key: "centipede",
    min: 0,
    max: 8,
    step: 1,
    defaultValue: 1,
  },
  {
    key: "ladybug",
    min: 0,
    max: 16,
    step: 1,
    defaultValue: 3,
  },
] as const;

export type AmbientObjectControl = (typeof ambientObjectControls)[number];
export type AmbientObjectKey = AmbientObjectControl["key"];
export type AmbientObjectCounts = Record<AmbientObjectKey, number>;

type AmbientObjectCountsListener = (counts: AmbientObjectCounts) => void;

export const defaultAmbientObjectCounts = ambientObjectControls.reduce(
  (counts, control) => {
    counts[control.key] = control.defaultValue;
    return counts;
  },
  {} as AmbientObjectCounts,
);

let currentAmbientObjectCounts: AmbientObjectCounts = {
  ...defaultAmbientObjectCounts,
};

const listeners = new Set<AmbientObjectCountsListener>();

function clampCount(value: number, control: AmbientObjectControl) {
  if (!Number.isFinite(value)) {
    return control.defaultValue;
  }

  return Math.round(Math.min(Math.max(value, control.min), control.max));
}

function emitAmbientObjectCounts() {
  const counts = getAmbientObjectCounts();
  for (const listener of listeners) {
    listener(counts);
  }
}

export function getAmbientObjectCounts(): AmbientObjectCounts {
  return {
    ...currentAmbientObjectCounts,
  };
}

export function setAmbientObjectCount(key: AmbientObjectKey, value: number) {
  const control = ambientObjectControls.find((item) => item.key === key);
  if (!control) {
    return;
  }

  const nextValue = clampCount(value, control);
  if (currentAmbientObjectCounts[key] === nextValue) {
    return;
  }

  currentAmbientObjectCounts = {
    ...currentAmbientObjectCounts,
    [key]: nextValue,
  };
  emitAmbientObjectCounts();
}

export function subscribeAmbientObjectCounts(listener: AmbientObjectCountsListener) {
  listeners.add(listener);

  return () => {
    listeners.delete(listener);
  };
}

export function ambientObjectCountsToFloat32Array(counts: AmbientObjectCounts) {
  return new Float32Array(ambientObjectControls.map((control) => counts[control.key]));
}
