# FIRST LOCAL MISSION SLICE

**Current implementation branch:** `feat/first-local-mission-slice`

This slice connects a real local project to a durable mission, performs one bounded read-only operation, and stores a result with evidence.

## What is implemented

1. **Native folder selection** — the Tauri dialog lets the user choose a directory; Rust canonicalizes and validates it before creating the project.
2. **Local project registry** — project name and canonical workspace path are stored in the app-local SQLite database.
3. **Mission drafting** — mission title/objective are validated and stored with a `draft` status.
4. **Workspace Scout** — traverses a bounded directory tree, respects Git ignore rules, does not follow symlinks, and only reads file metadata (path, size, extension).
5. **Sensitive-name exclusions** — common secrets/config files, build output, package directories, and local Rocksoul state are omitted.
6. **Artifact creation** — the workspace inventory is stored as structured JSON and hashed with SHA-256.
7. **Evidence receipt** — operation summary, scope, file count, limits, and no-content/no-write assertions are stored in the same database transaction as the artifact.
8. **Human review and approval** — a successful inventory moves the mission to `review`. The user can inspect the artifact and receipts, then explicitly approve it. Approval is itself recorded as evidence and moves the mission to `completed`.

## What the Scout does not do

- It does not read source code or document contents.
- It does not run commands, scripts, tests, or Git hooks.
- It does not call a model or send any data to a provider.
- It does not modify, create, or delete files in the selected project.
- It does not claim that the project is healthy, secure, correct, or fully understood.

The inventory is bounded to 300 file entries and depth 12. A truncated inventory is explicitly marked. Permission errors are retained as bounded warnings rather than being represented as successful reads.

## Local storage

The Tauri host stores `rocksoul.sqlite3` in the operating-system app-local data directory. SQLite uses foreign keys, WAL journaling, a five-second busy timeout, and versioned migrations.

The test suite uses a temporary database and verifies the project, mission, artifact, and evidence survive a Store reconnect.

## Build and run

Requirements:
- Rust 1.99.0 (rustup)
- Bun 1.4.4
- Windows WebView2 for the first desktop target

Commands:
```bash
bun install
bun run ui:check
bun run ui:build
cargo fmt --all -- --check
cargo test -p rocksoul-core
cargo clippy -p rocksoul-core --all-targets -- -D warnings
bun run desktop:dev
```

The desktop shell is not yet packaged for distribution. Tauri bundling remains disabled until installer signing, update metadata, and clean-machine installation checks are implemented.

## Known limits and next gates

- The native folder picker is wired through Tauri's `dialog:allow-open` capability; interactive OS-level testing is still pending.
- No source-content reading or provider egress exists yet.
- No action approval system exists because the Scout is metadata-only and read-only.
- No event ledger, backup UI, restore UI, or database encryption has been implemented.
- Platform validation is not complete. Windows is the first target; macOS/Linux need separate CI and runtime acceptance.
- The GitHub CI workflow must pass before this slice is treated as build-verified.

## Acceptance checklist

- [x] Rust domain and SQLite migration created.
- [x] Project/mission CRUD implemented in core.
- [x] Bounded read-only inventory implemented.
- [x] Artifact and evidence commit atomically.
- [x] Unit tests cover persistence and safety exclusions.
- [x] Tauri command boundary added.
- [x] React desktop UI connected to commands.
- [x] Native directory picker wired through a scoped dialog capability.
- [x] Human approval gate records a separate evidence receipt.
- [x] CI workflow added.
- [ ] GitHub CI green.
- [ ] Interactive Windows desktop smoke test completed.
- [ ] Backup/restore and clean-install checks added.
