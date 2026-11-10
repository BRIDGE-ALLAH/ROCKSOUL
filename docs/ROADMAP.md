# ROCKSOUL — ROADMAP

**Build the engine before the fireworks.**

Roadmap ini berbasis capability gate, bukan tanggal marketing. Tiap fase harus punya demo yang bisa diulang dan acceptance criteria yang bisa diperiksa.

## Current repository state — 2026-11-10

The first local-mission slice is merged to `main` via [PR #2](https://github.com/BRIDGE-ALLAH/ROCKSOUL/pull/2). Verified by [GitHub Actions run #42831777183](https://github.com/BRIDGE-ALLAH/ROCKSOUL/actions/runs/42831777183): Rust formatting, 5 core tests, Clippy, UI typecheck/production build, and Windows desktop compilation all passed.

**Implemented now:** one Tauri desktop shell, one React UI, local SQLite project/mission persistence, bounded metadata-only workspace inventory, SHA-256 artifact, atomic evidence receipt, and explicit human approval.

**Still open in Phase 0:** rename/archive UX, FTS5/search, durable event ledger, backup/restore, structured crash-safe lifecycle, keyboard/accessibility review, interactive Windows launch/folder-picker smoke test, and packaging/clean-install checks. The first slice is build-verified, not a finished installer or a complete Phase 0 exit.

**Phase 1 is not complete:** there is no provider adapter, no source-content/context compiler, no general tool registry, no mission cancellation/checkpoints, and no provider egress. Do not present the metadata Scout as a general-purpose autonomous agent.

## Phase 0 — FOUNDATION

**Target:** repo, contracts, UI shell, durable local state.

Deliver:
- Rust workspace and Tauri desktop shell.
- React/TypeScript UI shell with Rocksoul identity.
- Project create/open/rename/archive.
- SQLite migrations, WAL, FTS5, backup and restore.
- Typed domain IDs and event log.
- Structured logs and crash-safe startup/shutdown.
- CI for Rust + UI and Windows packaging smoke.

Exit criteria:
- Fresh clone builds in CI.
- App starts, creates a project, restarts, and restores data.
- No fake health or completion state.
- Database migration and backup/restore round-trip pass.
- UI can be navigated by keyboard.

## Phase 1 — ONE RELIABLE MISSION

**Target:** complete useful work with read-only tools.

Deliver:
- Mission spec with objective, scope, constraints, acceptance criteria.
- Context compiler with file/path/FTS search.
- Provider adapter for one real model.
- Tool registry with read-only file/repository tools.
- Plan preview, cancellation, timeout, and persisted checkpoints.
- Evidence timeline and artifact export.

Exit criteria:
- A real repository question can be researched and answered with file/source references.
- User can inspect what context was sent to the provider.
- Mission state survives app restart.
- Provider failure does not lose the mission or corrupt project state.
- Completion is blocked if required evidence is missing.

## Phase 2 — SAFE ACTIONS

**Target:** limited write actions in isolated workspace.

Deliver:
- Capability manifests and policy engine.
- Exact action preview and approval.
- One action per approval grant; no blanket shell permission.
- Isolated Git worktree per coding mission.
- Patch preview, apply journal, rollback, and test run.
- Tool process supervision and structured logs.

Exit criteria:
- Traversal, symlink, hard-link, TOCTOU, command-injection, and secret-leak tests pass.
- Cancellation and timeout work while a command is running.
- Crash recovery can identify incomplete operations and offer safe rollback.
- No agent writes outside granted roots.
- Human approval is bound to the exact patch/action digest.

## Phase 3 — PROOF-CARRYING CREATION

**Target:** artifacts become trustworthy deliverables.

Deliver:
- Artifact versions and diffs.
- Source/provenance manifest.
- Evidence mapping to acceptance criteria.
- Independent verifier role.
- Export pack with artifact, evidence, and environment manifest.
- Clear statuses: draft, built, tested, reviewed, approved, published.

Exit criteria:
- A second machine can inspect an exported evidence pack.
- Every completed mission has verifiable acceptance evidence.
- Failed checks and unresolved uncertainty remain visible.
- "Verified" cannot be set by a model response alone.

## Phase 4 — ADAPTIVE MEMORY & CREW

**Target:** continuity and specialist collaboration.

Deliver:
- Working, episodic, semantic, procedural, and preference memory.
- Memory correction/deletion controls.
- Skill folders with manifests and eval fixtures.
- Two or three specialist roles, not a default giant swarm.
- Context budgeter and memory retrieval evaluation.
- Agent comparison/critique view.

Exit criteria:
- Memory retrieval quality measured against a fixture set.
- User can inspect source and scope for each memory.
- Agent disagreement is preserved until resolved.
- Skills cannot grant their own permissions.
- No memory fact is promoted silently from an unverified model guess.

## Phase 5 — REALITY LOOM

**Target:** controlled hybrid computer use.

Deliver:
- Browser adapter with semantic locator support.
- Desktop accessibility tree adapter for first target OS.
- Screenshot/vision fallback.
- GUI + CLI + file operations in one trace.
- Session isolation and explicit app/domain allowlists.
- WeaveBench-inspired internal tasks.

Exit criteria:
- Each action re-reads state and checks expected result.
- Failed or ambiguous actions stop safely.
- Agent cannot access non-allowed applications or domains.
- Evaluation scores final artifacts and traces, not just model narration.

## Phase 6 — MODEL DJ & LOCAL INTELLIGENCE

**Target:** policy-first model routing.

Deliver:
- Multiple cloud provider adapters.
- Optional local model sidecar.
- Capability registry and model profiles.
- Routing by data class, measured quality, latency, price, and availability.
- Provider comparison on frozen task suites.
- Cost/latency accounting and fail-closed egress.

Exit criteria:
- User sees why a model/provider was selected.
- Sensitive data cannot be routed to a disallowed endpoint.
- Local mode remains usable without network.
- Model changes do not silently change harness behavior.

## Phase 7 — CREATIVE COLLIDER / RESEARCH LAB

**Target:** exploratory multimodal workflows and counterfactual experiments.

Potential:
- Image/audio/video workflows.
- Simulation and "what-if" environments.
- More expressive idea graphs.
- Agent-generated skill proposals with human approval.
- Research reports on adaptability, collaboration, and evaluation.

This phase is not required for the first product release. The ASI phrase is a horizon, not a delivery plan.

## Release gates

Every public release must pass:
- Rust format, lint, unit, integration, and property tests.
- UI type check, lint, unit, and accessibility smoke.
- Windows clean-install, launch, update, rollback, and uninstall checks.
- SQLite migration, corruption recovery, backup, and restore checks.
- Security suite for filesystem, process, IPC, credentials, and network egress.
- Eval suite with exact model, harness, tool, skill, and environment versions.
- Dependency license and vulnerability checks.
- Signed artifacts and update metadata.
- No fake "done", "saved", "healthy", "synced", or "verified" states.

## Explicit non-goals before v1

- Autonomous public cloud publishing.
- Unrestricted shell/desktop control.
- Self-modifying permissions or policy.
- Marketplace and arbitrary third-party native plugins.
- Multi-tenant team server.
- Always-on screen/microphone recording.
- Claims of AGI, ASI, consciousness, or guaranteed human-level capability.