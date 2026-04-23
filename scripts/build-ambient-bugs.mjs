import { copyFileSync, mkdirSync } from "node:fs";
import { execFileSync } from "node:child_process";
import path from "node:path";
import { fileURLToPath } from "node:url";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const rootDir = path.resolve(scriptDir, "..");
const crateDir = path.join(rootDir, "rust", "ambient-bugs");
const generatedDir = path.join(rootDir, "src", "generated", "ambient-bugs");
const publicDir = path.join(rootDir, "public", "ambient-bugs");
const wasmInput = path.join(
  crateDir,
  "target",
  "wasm32-unknown-unknown",
  "release",
  "ambient_bugs.wasm",
);

mkdirSync(generatedDir, { recursive: true });
mkdirSync(publicDir, { recursive: true });

execFileSync(
  "cargo",
  ["build", "--release", "--target", "wasm32-unknown-unknown"],
  {
    cwd: crateDir,
    stdio: "inherit",
  },
);

execFileSync(
  "wasm-bindgen",
  [
    "--target",
    "web",
    "--no-typescript",
    "--out-dir",
    generatedDir,
    wasmInput,
  ],
  {
    cwd: rootDir,
    stdio: "inherit",
  },
);

copyFileSync(
  path.join(generatedDir, "ambient_bugs_bg.wasm"),
  path.join(publicDir, "ambient_bugs_bg.wasm"),
);
