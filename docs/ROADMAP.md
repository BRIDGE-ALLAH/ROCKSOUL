# ROCKSOUL — ROADMAP & CAPABILITY GATES

**Build the engine before the fireworks.**

This roadmap is capability-gated, not date-driven. The normative product and engineering rules are in [BLUEPRINT.md](BLUEPRINT.md); the user-facing promise and language are in [PRODUCT_NARRATIVE.md](PRODUCT_NARRATIVE.md). The independently executable backlog is indexed in [ISSUE_MAP.md](ISSUE_MAP.md). A task is complete only when its own acceptance criteria and evidence are satisfied.

## Current repository state — 10 November 2026

The first local-mission slice is merged to main via [PR #2](https://github.com/BRIDGE-ALLAH/ROCKSOUL/pull/2). Its source tree passed [CI run #42832504466](https://github.com/BRIDGE-ALLAH/ROCKSOUL/actions/runs/42832504466): Rust formatting, 5 core tests, Clippy, UI typecheck/production build, and Windows desktop compilation.

**Implemented on main:**
- Tauri desktop host, React/TypeScript UI, Rust core.
- Local SQLite/SQLx project and mission persistence.
- Native folder-picker wiring and Rust-side path validation.
- Bounded metadata-only Workspace Scout (2,000 total entries, 300 file records, depth 12).
- Git-ignore-aware traversal, secret-name exclusions, no symlink following, no source/document/binary content reads.
- One inventory artifact with SHA-256 and an atomic evidence receipt.
- Explicit human review and approval.

**Not implemented on main:** source-content context compilation, model/provider inference or egress, general tool registry, mission cancellation/checkpoints, event ledger, FTS5 search, backup/restore, patching, shell/process tools, multi-agent orchestration, typed long-term memory, browser/desktop computer use, installer/update.

PR #3 is a draft branch for Windows native WebDriver E2E. Treat it as pending until its latest native test run is green. The native Windows shell folder-picker choose/cancel interaction is a separate manual acceptance gate; WebDriver DOM tests do not prove that OS-level dialog.

## Phase 0 — FOUNDATION HARDENING

**Target:** turn the first prototype slice into a dependable local desktop foundation.

Deliver: Windows native lifecycle smoke, folder-picker acceptance, project rename/archive, mission draft editing, append-only event ledger, project/mission timeline, FTS5 metadata search, SQLite-safe backup/restore, crash-safe startup/shutdown, keyboard/accessibility validation, and Windows packaging/update release gates.

Exit criteria:
- Windows native app launches, selects/cancels a folder, creates a project and mission, completes a Scout, restarts, and restores state.
- Project and mission lifecycle events are durable and queryable.
- Search results are deterministic, local, and do not index prohibited secrets.
- Backup/restore round-trip passes against a live WAL database; failed restore leaves the previous database intact.
- Startup failures and interrupted operations are diagnosable, not silently converted to success.
- Core flows are keyboard operable and pass the defined accessibility smoke.
- Installer, update, rollback, and uninstall gates are proven on a clean Windows machine before public release.

## Phase 1 — ONE RELIABLE MISSION

**Target:** finish one useful read-only research mission using a real model and inspectable context.

Deliver: full mission contract, guarded mission state machine, checkpoints/cancel/timeout, source catalog, scoped read-only content tools, deterministic FTS5/context compiler, context preview, provider-neutral contract, OS credential storage, data-egress policy, one real provider adapter, plan preview, read-only tool registry, criteria-specific verification.

Exit criteria:
- A user can ask a real repository question and receive an answer grounded in file/source references.
- The exact context package, omissions, limits, data class, and egress destination are visible before transmission.
- Provider failure, timeout, cancellation, and app restart do not lose or corrupt the mission.
- Completion is blocked when mandatory evidence is missing.
- No write, process, or computer-use capability is implied by a read-only mission.

## Phase 2 — SAFE ACTIONS

**Target:** allow narrow, reviewable, reversible changes in isolation.

Deliver: capability manifests/grants, deterministic policy engine, digest-bound action preview/approval, path-scoped filesystem grants and adversarial tests, process supervision, explicit command profiles, isolated Git worktrees, patch proposal/review/apply journal, test runner, rollback, crash recovery.

Exit criteria:
- Traversal, symlink, hard-link, junction, TOCTOU, injection, secret-leak, stale-approval, and replay tests pass for supported platforms.
- Every write is confined to the exact granted root and operation.
- Approval is bound to the exact action/patch digest; modified proposals require re-approval.
- Cancellation works while a process is running.
- A crash can recover or safely roll back incomplete operations.
- No unrestricted shell capability exists.

## Phase 3 — PROOF-CARRYING CREATION

**Target:** make every deliverable independently inspectable.

Deliver: immutable artifact versions, diffs, source/provenance manifest, criteria-to-evidence mapping, portable export pack, integrity verifier.

Exit criteria:
- A second machine can inspect and validate an export pack without the original app database.
- Every completed mission has verifiable evidence for every mandatory criterion.
- Tampering is detected.
- Failed checks, disagreement, and uncertainty remain visible.
- A model cannot set a verified status by assertion.

## Phase 4 — ADAPTIVE MEMORY & CREW

**Target:** add continuity and specialist collaboration without losing provenance or control.

Deliver: typed memory lifecycle, correction/deletion, evaluated deterministic retrieval, portable skill manifest and fixtures, single worker + verifier contracts, small bounded specialist crew, disagreement/critique view, isolated worktrees for parallel coding.

Exit criteria:
- Memory retrieval is evaluated on frozen fixtures; each result exposes source, scope, and status.
- User correction/deletion propagates to derived indexes.
- Skills declare required capabilities and cannot self-authorize.
- Agents have bounded context, budget, and capability grants; parallel workers never share a writable worktree.
- Unresolved disagreement is preserved instead of replaced by fake consensus.

## Phase 5 — REALITY LOOM

**Target:** controlled hybrid computer use.

Deliver: semantic-first browser adapter, domain allowlist, isolated sessions, Windows accessibility-tree adapter, screenshot/vision fallback, state re-read and per-action verification, hybrid GUI/CLI/file benchmark.

Exit criteria:
- Each action checks preconditions, re-reads state, verifies the result, and records a trace.
- Failure or ambiguity stops safely and can request a human.
- No app/domain outside the allowlist can be accessed.
- Screenshots and screen-derived content follow explicit privacy/retention rules.
- Evaluation scores final artifacts and traces, not model narration.

## Phase 6 — MODEL DJ & LOCAL INTELLIGENCE

**Target:** route to allowed models based on measured fit.

Deliver: versioned model capability profiles, multiple provider adapters, policy-first router, local usage/cost/latency accounting, optional local inference sidecar, offline/local-only mode, frozen model comparison suite.

Exit criteria:
- Privacy and egress constraints filter candidate models before price/quality optimization.
- The user sees why a model/provider was selected.
- Local-only mode works with network disabled for the supported feature set.
- Model comparisons pin exact model, harness, prompts, tools, and environment.
- Provider changes do not silently change policy or harness behavior.

## Phase 7 — CREATIVE COLLIDER / RESEARCH LAB

**Target:** exploratory multimodal workflows and counterfactual experiments.

Deliver: typed creative asset import, isolated image/audio/video adapters, counterfactual scenario sandbox, human-approved skill proposals, privacy-preserving human-signal metrics, AGILITY long-horizon evaluation suite.

Exit criteria:
- Every modality declares data handling, provider egress, asset provenance, retention, and approval behavior.
- Simulation output is labelled as simulated and exposes assumptions.
- User metrics are opt-in, local, inspectable, and deletable.
- No multimodal or benchmark result is presented as an AGI/ASI score.

## Cross-cutting release gates

These apply regardless of phase:
- Rust formatting, lint, unit, integration, migration, property, and security tests.
- UI typecheck, lint, component tests, and accessibility checks.
- Native desktop E2E on every supported target OS.
- Database backup/restore, corruption detection, and interrupted-operation recovery.
- Filesystem, IPC, credentials, process, egress, approval, and artifact-integrity suite.
- Dependency license/vulnerability review, SBOM, signed release artifacts, and update metadata validation.
- Windows clean install, update, rollback, uninstall, and recovery acceptance.
- No fake “done”, “saved”, “healthy”, “synced”, “safe”, or “verified” state.

## Explicit non-goals before v1

- Unrestricted shell/desktop control.
- Autonomous public-cloud publishing or arbitrary account actions.
- Self-modifying permissions or policy.
- Marketplace for arbitrary native plugins.
- Multi-tenant team server or distributed infrastructure.
- Always-on screen/microphone/camera recording.
- Default giant agent swarm.
- Vector database before deterministic retrieval has been evaluated.
- Claims of AGI, ASI, consciousness, sentience, or guaranteed human-level correctness.

## How to execute

- Pick one unblocked issue from [ISSUE_MAP.md](ISSUE_MAP.md).
- Follow [AI_TASK_PROTOCOL.md](AI_TASK_PROTOCOL.md).
- Do not work across multiple issues unless the issue explicitly permits it.
- Close an issue only after every acceptance checkbox has attached evidence.
