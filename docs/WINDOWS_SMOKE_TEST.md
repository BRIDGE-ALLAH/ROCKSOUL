# Windows desktop smoke test

**Status snapshot: 11 November 2026.** Automated native lifecycle E2E passed on commit `02c258c3cc2f17fa429d61bbe8aa830158aa6d33` in [GitHub Actions run #42894857711](https://github.com/BRIDGE-ALLAH/ROCKSOUL/actions/runs/42894857711). This checklist separates automated WebView interaction from the native Windows shell dialog and release packaging. Passing one gate must not be represented as passing the others.

## Automated Windows native E2E

The automated job is intended to launch the real Rocksoul Tauri executable in the Rust **debug profile** through WebdriverIO and Microsoft Edge WebDriver. It exercises the actual Rust IPC handlers and local SQLite vault. It is not browser-preview mode, and application commands or persistence must not be mocked.

The fixture is synthetic and contains:
- a README with a sentinel string;
- a source path with a sentinel string;
- a `.gitignore` rule for `target/`;
- a `.env` file with a secret sentinel;
- ignored generated build output.

The automated acceptance scenario must prove:

- [x] The native executable starts and the UI reports a successful connection to the local SQLite vault.
- [x] A project is created from the synthetic local directory.
- [x] A mission is drafted and persisted.
- [x] Workspace Scout runs through the UI and the real Rust IPC boundary.
- [x] The JSON inventory artifact contains allowed metadata paths, sizes, and extensions only.
- [x] The artifact excludes `.env` and ignored build output and contains none of the README/source/secret sentinel contents.
- [x] A read-only inventory evidence receipt is stored.
- [x] Explicit human approval records a second receipt and moves the mission to completed.
- [x] The native application process is restarted; the project, completed mission, artifact, and both receipts are restored from SQLite.
- [x] The synthetic workspace sentinel file is byte-for-byte unchanged.
- [x] Failure diagnostics are retained as a WebDriver workflow log and sanitized screenshot artifact (verified on a failing CI attempt).

### Current observed result

The latest passing PR-triggered run is [#42894857711](https://github.com/BRIDGE-ALLAH/ROCKSOUL/actions/runs/42894857711), on commit `02c258c3cc2f17fa429d61bbe8aa830158aa6d33`.

- Rust core tests, formatting, and Clippy: passed.
- UI typecheck and production build: passed using the corrected Bun scripts.
- Windows native executable build and WebDriver session creation: passed with the Vite `dist` assets embedded through Tauri's `custom-protocol` feature.
- Windows E2E lifecycle: **passed** — native app startup, SQLite vault connection, project/mission creation, metadata-only Scout, artifact and first receipt, explicit approval and second receipt, native process restart, and restoration of project/completed mission/artifact/receipts.
- Workspace sentinel: unchanged.
- Real Windows shell folder-picker interaction: not automated and not yet manually accepted; it remains [P0-F02 / issue #6](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/6).

The automated P0-F01 harness is tracked in [issue #5](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/5). The manual OS dialog is a separate acceptance gate; no automated E2E result is a substitute for it.

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

## P0-F12 — Verified backup export (manual native Save dialog)

- [ ] On a real Windows desktop, activate **CREATE VERIFIED BACKUP** and confirm the native Save dialog opens with the `.rocksoul-backup` bundle filter and a suggested filename.
- [ ] Select a new destination name in a writable folder. Confirm the UI shows a busy/verification state and reports success only after Rust returns the finalized bundle.
- [ ] Confirm the selected name is the resulting bundle directory and it contains `database.sqlite3` plus `manifest.json`.
- [ ] Read `manifest.json`; confirm `integrityResult` is `ok`, the SHA-256 is 64 hexadecimal characters, and `databaseBytes` matches the database file size.
- [ ] Independently calculate SHA-256 for `database.sqlite3` and compare it with the manifest value.
- [ ] Start another backup, cancel the native Save dialog, and confirm the app returns to idle without an error or a new backup bundle.
- [ ] Select an existing destination name; confirm the operation fails with a useful message, does not overwrite the existing bundle, and the live vault remains usable.
- [ ] Simulate a destination permission/I/O failure where practical; confirm the partial staging folder is removed, the UI returns to idle, and the live vault remains usable.
- [ ] Record Windows version/build, app commit, selected destination outcome, manifest/hash verification, and sanitized screenshots. Do not include private workspace contents in the evidence.

WebDriver cannot control the native Windows Save dialog outside the WebView DOM. The native E2E job therefore verifies app launch and vault lifecycle but does not substitute for this manual Save-dialog acceptance.
