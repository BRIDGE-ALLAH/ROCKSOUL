# ROCKSOUL Desktop — first local mission slice

**Think with chaos. Act with riot.**

This folder contains the first runnable desktop implementation: a local studio, mission state machine, read-only workspace inventory, local artifacts, and SHA-256 evidence receipts.

## Requirements

- Rust stable **1.94 or newer** (Tauri 2.12 and SQLx 0.9 have higher MSRV than the earlier scaffold assumed).
- Bun 1.4+.
- Windows 10/11 with WebView2 Runtime for the first target build.
- Windows native build tools (Visual Studio C++ Build Tools / Windows SDK).

## Run on Windows

From this directory:

```powershell
bun install
bun run tauri dev
```

Build the frontend only:

```powershell
bun run check
bun run build
```

Run native tests and lint:

```powershell
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml
```

Build a Windows installer after the desktop build has been validated:

```powershell
bun run tauri build
```

The initial CI compiles the desktop executable without packaging an installer. Installer signing, clean-machine install/update/rollback, and smoke testing remain release gates.

## What this slice does

1. Creates a local studio/project.
2. Creates a mission with a short brief.
3. Starts the mission through a constrained state transition: `ready → active`.
4. Lets the user choose a folder using the native directory picker.
5. Inventories a bounded set of paths without opening file contents.
6. Skips common generated directories and sensitive filename patterns; does not follow symlinks.
7. Saves a user-authored artifact in SQLite and records its SHA-256 digest.
8. Moves the mission to review; only then can the user mark it complete.
9. Stores events and evidence locally so the UI can show what actually happened.

## Storage

The application database is created under Tauri's OS-specific application data directory as `rocksoul.sqlite` (with SQLite WAL files while the database is open).

Tables:
- `projects`
- `missions`
- `artifacts`
- `evidence`
- `events`

SQLite schema version is recorded using `PRAGMA user_version`. Backups/migrations beyond the initial schema are the next storage hardening task.

## Security boundaries in this slice

- Workspace selection is an explicit user action.
- The scanner has a 1,500-entry cap and maximum depth of five.
- It does not read file contents, execute commands, invoke an agent, or contact a model/provider.
- Symlinks and common credential file names are excluded.
- All persistence and mission transition checks live in Rust.
- Artifact content is limited to 1 MB.
- UI status is derived from actual native command responses.

The scanner is an initial conservative inventory, not a complete secrets detector or a sandbox. It must not be described as a security audit of the selected repository.

## Known limits

- One artifact can be saved per mission lifecycle in this first slice; multiple versions and richer review are planned.
- Workspace scanning is inventory-only; content-aware indexing is not enabled.
- No model provider, agent, shell, browser automation, cloud sync, telemetry, or publishing is included.
- Evidence currently proves local artifact persistence and its digest, not semantic correctness of artifact content.
- Windows runtime/installer validation depends on the CI result and a clean-machine manual pass.

## Product direction

See the root [product vision](../../docs/PRODUCT_VISION.md), [architecture](../../docs/ARCHITECTURE.md), [tech stack](../../docs/TECH_STACK.md), and [roadmap](../../docs/ROADMAP.md).