import assert from "node:assert/strict";
import { access, readFile } from "node:fs/promises";
import { browser, $, $$ } from "@wdio/globals";

const testId = (id) => `[data-testid="${id}"]`;
const projectName = `Rocksoul E2E ${Date.now()}`;
const missionTitle = "Verify local mission persistence";
const workspace = process.env.ROCKSOUL_E2E_WORKSPACE;

async function waitFor(selector, description, timeout = 30_000) {
  await browser.waitUntil(
    async () => {
      try {
        return await $(selector).isDisplayed();
      } catch {
        return false;
      }
    },
    {
      timeout,
      interval: 250,
      timeoutMsg: `Timed out waiting for ${description}`,
    },
  );
}

async function waitForStatus(fragment) {
  await browser.waitUntil(
    async () => {
      try {
        return (await $(testId("status-line")).getText()).includes(fragment);
      } catch {
        return false;
      }
    },
    {
      timeout: 30_000,
      interval: 250,
      timeoutMsg: `Status line never contained: ${fragment}`,
    },
  );
}

describe("Rocksoul Windows desktop — local mission lifecycle", () => {
  it("creates, scans, approves, closes, and reopens a real local mission", async () => {
    assert.ok(
      workspace,
      "ROCKSOUL_E2E_WORKSPACE must point to the synthetic test workspace",
    );
    await access(workspace);

    await waitFor(testId("vault-status"), "local vault status", 60_000);
    await browser.waitUntil(
      async () => (await $(testId("vault-status")).getText()).includes("LOCAL VAULT"),
      {
        timeout: 60_000,
        interval: 250,
        timeoutMsg: "The native application did not connect to its local SQLite vault",
      },
    );

    const browseButton = $(testId("browse-folder"));
    assert.equal(await browseButton.isDisplayed(), true);
    assert.equal(await browseButton.isEnabled(), true);

    await $(testId("project-name")).setValue(projectName);
    await $(testId("project-root-path")).setValue(workspace);
    await $(testId("create-project")).click();
    await waitForStatus(`Project linked: ${projectName}`);

    const projectSelector =
      `${testId("project-item")}[data-project-name="${projectName}"]`;
    await waitFor(projectSelector, "new local project");

    await $(testId("mission-title")).setValue(missionTitle);
    await $(testId("mission-objective")).setValue(
      "Inventory metadata only, preserve the workspace, and prove that approval survives an application restart.",
    );
    await $(testId("create-mission")).click();

    const missionSelector =
      `${testId("mission-item")}[data-mission-title="${missionTitle}"]`;
    await waitFor(missionSelector, "new mission draft");

    await $(testId("run-sound-check")).click();
    await waitForStatus("Sound check ready:");
    await waitFor(testId("artifact-preview"), "saved workspace inventory artifact");

    const artifact = await $(testId("artifact-preview")).getText();
    assert.match(artifact, /README\.md/, "The inventory should include README metadata");
    assert.match(artifact, /src\/index\.ts/, "The inventory should include source path metadata");
    assert.doesNotMatch(artifact, /\.env/, "Secret environment files must be excluded");
    assert.doesNotMatch(
      artifact,
      /target\/generated\.txt/,
      "Ignored build output must be excluded",
    );
    assert.doesNotMatch(
      artifact,
      /WORKSPACE_CONTENT_SENTINEL|SOURCE_CONTENT_SENTINEL|SECRET_VALUE_SENTINEL/,
      "The Scout must not open file contents",
    );

    let receipts = await $$(testId("evidence-item"));
    assert.equal(receipts.length, 1, "A scan must save one read-only inventory receipt");
    assert.equal(await receipts[0].getAttribute("data-evidence-kind"), "read_only_inventory");

    await $(testId("approve-mission")).click();
    await waitForStatus("Mission approved.");
    await browser.waitUntil(
      async () => (await $(missionSelector).getText()).includes("COMPLETED"),
      {
        timeout: 15_000,
        interval: 250,
        timeoutMsg: "The mission did not move to completed after explicit approval",
      },
    );

    const originalReadme = await readFile(
      `${workspace}\\README.md`,
      "utf8",
    );
    assert.equal(
      originalReadme,
      "WORKSPACE_CONTENT_SENTINEL — this content must remain untouched.",
      "Workspace Scout must not modify the selected project",
    );

    // Restart the native Tauri process, not just the React route, then prove
    // the SQLite project, mission, artifact, and evidence were persisted.
    await browser.reloadSession();
    await waitFor(testId("vault-status"), "reopened local vault", 60_000);
    await browser.waitUntil(
      async () => (await $(testId("vault-status")).getText()).includes("LOCAL VAULT"),
      {
        timeout: 60_000,
        interval: 250,
        timeoutMsg: "The relaunched application could not reopen its local vault",
      },
    );

    await waitFor(projectSelector, "persisted project after application restart");
    await $(projectSelector).click();
    await waitFor(missionSelector, "persisted mission after application restart");
    await $(missionSelector).click();

    await browser.waitUntil(
      async () => (await $(missionSelector).getText()).includes("COMPLETED"),
      {
        timeout: 15_000,
        interval: 250,
        timeoutMsg: "The completed state was not restored after application restart",
      },
    );
    await waitFor(testId("artifact-preview"), "persisted inventory artifact");
    const persistedArtifact = await $(testId("artifact-preview")).getText();
    assert.match(persistedArtifact, /README\.md/);
    assert.match(persistedArtifact, /src\/index\.ts/);

    receipts = await $$(testId("evidence-item"));
    assert.equal(
      receipts.length,
      2,
      "The inventory receipt and human approval receipt must survive application restart",
    );
    const firstKind = await receipts[0].getAttribute("data-evidence-kind");
    const secondKind = await receipts[1].getAttribute("data-evidence-kind");
    assert.deepEqual(
      new Set([firstKind, secondKind]),
      new Set(["read_only_inventory", "human_approval"]),
    );
  });
});
