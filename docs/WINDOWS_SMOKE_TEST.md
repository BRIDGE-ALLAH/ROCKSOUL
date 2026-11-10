# Windows desktop smoke test

**Status snapshot: 11 November 2026.** This checklist separates automated WebView interaction from the native Windows shell dialog and from release packaging. Passing one gate must not be represented as passing the others.

## Automated Windows native E2E

The automated job is intended to launch the real Rocksoul Tauri executable in the Rust **debug profile** through WebdriverIO and Microsoft Edge WebDriver. It exercises the actual Rust IPC handlers and local SQLite vault. It is not browser-preview mode, and application commands or persistence must not be mocked.

The fixture is synthetic and contains:
- a README with a sentinel string;
- a source path with a sentinel string;
- a `.gitignore` rule for `target/`;
- a `.env` file with a secret sentinel;
- ignored generated build output.

The automated acceptance scenario must prove:

- [ ] The native executable starts and the UI reports a successful connection to the local SQLite vault.
- [ ] A project is created from the synthetic local directory.
- [ ] A mission is drafted and persisted.
- [ ] Workspace Scout runs through the UI and the real Rust IPC boundary.
- [ ] The JSON inventory artifact contains allowed metadata paths, sizes, and extensions only.
- [ ] The artifact excludes `.env` and ignored build output and contains none of the README/source/secret sentinel contents.
- [ ] A read-only inventory evidence receipt is stored.
- [ ] Explicit human approval records a second receipt and moves the mission to completed.
- [ ] The native application process is restarted; the project, completed mission, artifact, and both receipts are restored from SQLite.
- [ ] The synthetic workspace sentinel file is byte-for-byte unchanged.
- [ ] Test failures retain actionable application logs and sanitized screenshots.

### Current observed result

The latest observed PR-triggered run is [#42851122894](https://github.com/BRIDGE-ALLAH/ROCKSOUL/actions/runs/42851122894), on commit `9874f4337145b20afb1a65ce87a5be9e7590fa6b`.

- Rust core tests, formatting, and Clippy: passed.
- UI typecheck and production build: passed.
- Windows native executable build and WebDriver session creation: passed.
- Windows E2E scenario: **failed; not accepted**. The installed Tauri WebDriver service aborts its worker `before()` hook in `patchBrowserExecute` with `TypeError: Cannot read properties of undefined (reading 'bind')`. The later `vault-status` timeout is a downstream symptom; this run does not establish that the SQLite vault itself failed.
- Real Windows shell folder-picker interaction: not automated and not yet manually accepted.

Track automated harness completion in [P0-F01 / issue #5](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/5). Do not close that issue until the PR-triggered Windows job and the Linux Rust/UI jobs pass on the same commit.

## Manual native folder-picker acceptance

This test requires a real, interactive Windows desktop session. WebDriver's WebView DOM does not automate the Windows shell's native directory picker.

Record the app commit, Windows edition/build, WebView2 runtime, operator, date, results, and sanitized screenshots. Use a synthetic test folder only.

- [ ] Launch the app and wait until the header truthfully reports **LOCAL VAULT**.
- [ ] Enter a valid test path, then click **BROWSE** and select a different existing directory.
- [ ] Confirm the selected absolute path appears in the project form.
- [ ] Reopen **BROWSE**, cancel the dialog, and confirm the existing path is unchanged.
- [ ] Select a valid directory and link it; confirm Rust stores the canonical path.
- [ ] Try a nonexistent or invalid path; confirm a visible actionable error and no project creation.
- [ ] Create a mission and run the read-only Sound Check.
- [ ] Confirm the artifact contains metadata only; common secret files and ignored build output are absent.
- [ ] Confirm the second human-approval receipt is visible after approval.
- [ ] Exit the entire app process and relaunch it; confirm project, completed mission, artifact, and both receipts remain.
- [ ] Compare the test workspace before/after and confirm no files were changed.

Track this manual OS-dialog gate independently in [P0-F02 / issue #6](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/6).

## Local commands

From the repository root, build the native debug executable used by the automated interaction test:

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

Run the E2E test from `apps/desktop/e2e` with Node.js 24:

~~~powershell
npm install
npm test
~~~

## Release boundary

This smoke test is not installer acceptance. Tauri bundling remains disabled. A releasable Windows build still needs a reproducible installer, signing and signature verification, clean-machine install, update/rollback, backup/restore, uninstall, and security/supply-chain gates. Those are tracked by the separate P0 and OPS task issues; do not treat this checklist as a substitute.
