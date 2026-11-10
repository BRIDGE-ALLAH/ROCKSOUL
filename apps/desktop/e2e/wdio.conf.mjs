import path from "node:path";
import { mkdir } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import { browser } from "@wdio/globals";

const here = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(here, "../../..");
const appBinaryPath = path.join(
  repositoryRoot,
  "target",
  "debug",
  "rocksoul-desktop.exe",
);
const testResults = path.join(here, "test-results");

export const config = {
  runner: "local",
  specs: ["./specs/**/*.spec.mjs"],
  maxInstances: 1,
  injectGlobals: false,
  capabilities: [
    {
      browserName: "tauri",
      "wdio:enforceWebDriverClassic": true,
      "tauri:options": {
        application: appBinaryPath,
      },
    },
  ],
  services: [
    [
      "@wdio/tauri-service",
      {
        appBinaryPath,
        driverProvider: "external",
        autoInstallTauriDriver: true,
        autoDownloadEdgeDriver: true,
        // App-side log forwarding requires the optional tauri-plugin-wdio;
        // screenshots and WebDriver logs remain available without it.
        startTimeout: 120_000,
        commandTimeout: 60_000,
      },
    ],
  ],
  framework: "mocha",
  mochaOpts: {
    ui: "bdd",
    timeout: 180_000,
  },
  reporters: ["spec"],
  logLevel: "info",
  waitforTimeout: 15_000,
  connectionRetryTimeout: 120_000,
  connectionRetryCount: 2,
  afterTest: async function (test, _context, { error }) {
    if (!error || !browser.sessionId) return;

    await mkdir(testResults, { recursive: true });
    const safeName = String(test.title).replace(/[^a-z0-9-_]+/gi, "-");
    try {
      await browser.saveScreenshot(
        path.join(testResults, `${Date.now()}-${safeName}.png`),
      );
    } catch (screenshotError) {
      console.warn("Could not capture the failed E2E screen:", screenshotError);
    }
  },
};
