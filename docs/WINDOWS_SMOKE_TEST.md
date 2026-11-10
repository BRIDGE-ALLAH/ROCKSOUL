# Windows desktop smoke test

This is the native desktop acceptance checklist for the first local-mission slice.

## Automated Windows E2E

The GitHub Actions Windows job compiles and launches the actual Tauri desktop executable in the Rust **debug profile** through WebDriverIO and Microsoft Edge WebDriver. It still exercises the real Rust IPC handlers and local SQLite vault; this is not browser preview mode and no application commands or persistence are mocked. The unoptimized profile keeps the E2E job focused on behavior rather than release-profile thin-LTO link time.

It creates a synthetic workspace and checks:

- Native app startup and local SQLite vault connection.
- Creating a project from a real local directory path.
- Creating a mission.
- Running the Rust Workspace Scout through the UI.
- Saving and rendering the metadata-only inventory artifact.
- Excluding .env and ignored build output; not copying file contents into the artifact.
- Saving the read-only inventory evidence receipt.
- Explicitly approving the mission and recording a second evidence receipt.
- Closing and relaunching the native Tauri process, then reloading the project, completed mission, artifact, and both receipts from SQLite.
- Confirming the original workspace README remains unchanged.

The test uses the editable folder-path field to keep the test deterministic. It does not automate the Windows shell's native folder-picker dialog because that dialog is outside the WebView DOM.

## Manual native folder-picker acceptance

Run this on a real interactive Windows desktop session:

1. Start the app with **bun run desktop:dev** or launch the built executable.
2. Wait for the header to show **LOCAL VAULT**.
3. Click **BROWSE** and select an existing test workspace folder.
4. Confirm the selected absolute path appears in the project form.
5. Open **BROWSE** again and cancel the dialog. Confirm the current path is unchanged.
6. Link the project, create a mission, and run the Sound Check.
7. Confirm the artifact shows paths, sizes, and extensions only; it must not contain file contents.
8. Confirm ignored build output and common secret files are omitted.
9. Approve the mission and confirm that the second receipt is visible.
10. Close the entire application process and relaunch it. Confirm the project, completed mission, artifact, and both receipts remain visible.
11. Try linking a non-existent path. Confirm the error is shown and no project is created.
12. Confirm the files in the selected workspace were not changed.

## Local commands

From the repository root, build and launch the native debug executable used by the automated interaction test:

~~~powershell
bun install
bun run ui:build
cargo build -p rocksoul-desktop
.\target\debug\rocksoul-desktop.exe
~~~

To validate the optimized release executable separately:

~~~powershell
cargo build --release -p rocksoul-desktop
.\target\release\rocksoul-desktop.exe
~~~

The CI E2E command is run from apps/desktop/e2e with Node.js 24:

~~~powershell
npm install
npm test
~~~

## Current release boundary

The E2E test is not an installer acceptance test. Tauri bundling remains disabled. Signed installer, clean-machine installation, backup/restore, updater, rollback, and uninstall are separate release gates.
