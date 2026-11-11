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

## P0-F14 — Startup, shutdown, and unclean-exit diagnostics

Automated Rust tests cover the active-session marker lifecycle and fixed error-code redaction. The native desktop smoke below still requires an interactive Windows machine for process termination and startup-failure injection.

Record the app commit, Windows edition/build, WebView2 runtime, operator, date, and sanitized evidence. Do not include real app-data paths or private project contents in logs or screenshots.

- [ ] First launch with no prior session marker; confirm the vault reports CONNECTING, then LOCAL VAULT only after SQLite connection and all migrations succeed.
- [ ] Exit Rocksoul normally. Confirm shutdown logs show database pool close completed before the active-session marker is cleared.
- [ ] Relaunch after the clean exit; confirm there is no previous-session warning.
- [ ] Start Rocksoul, create a synthetic mission, then terminate the process forcibly (Task Manager / `Stop-Process -Force`) without closing the app. Relaunch and confirm the warning `A previous session ended...` is visible while the database is re-opened and checked.
- [ ] Confirm the warning does not claim an interrupted operation succeeded and does not automatically replay or mutate any mission.
- [ ] With a synthetic test profile, make the app database unavailable or use a copied corrupt database fixture. Confirm the UI reports VAULT ERROR, does not report LOCAL VAULT, and project/mission commands remain unavailable.
- [ ] Restore the test database fixture and relaunch. Confirm startup can recover and the warning clears after a later graceful exit.
- [ ] Review structured logs: startup stages distinguish session marker, database connection, migration, UI host readiness, pool close, and marker cleanup.
- [ ] Confirm logs contain only fixed stage/error codes and never include application-data paths, project paths, source/document contents, prompts, credentials, or tokens.

A forced-termination check must be performed on a real Windows process. A WebDriver session reload is a graceful lifecycle action and is not evidence of crash detection.

Structured logs are written as daily JSONL files under the app-local data directory's `logs` folder (filename prefix `rocksoul.jsonl`). The writer is flushed after the database pool is closed and the clean-shutdown marker is processed. If the log directory cannot be created, the host falls back to stderr where the platform exposes it; the UI still reports the actual database startup status.
