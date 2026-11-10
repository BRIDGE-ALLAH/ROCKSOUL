# ROCKSOUL — DELIVERY ROADMAP

**Build the engine before the fireworks.**

This roadmap is a capability-gated delivery sequence, not a marketing calendar. The canonical detail and security rules live in [BLUEPRINT.md](BLUEPRINT.md); this file is the phase-level navigation.

## Authority and task tracking

- [Product narrative](PRODUCT_NARRATIVE.md) defines the promise and experience.
- [Normative blueprint](BLUEPRINT.md) defines architecture, contracts, data, security, and exit gates.
- [AI task protocol](AI_TASK_PROTOCOL.md) defines how one small local agent executes one task.
- [Issue map](ISSUE_MAP.md) indexes the **100 task issues** and major dependency gates.
- [Program tracker #4](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/4) is the umbrella checklist.

No phase checkbox is complete merely because its issues are created. Exit gates require merged implementation and named verification evidence.

## Current repository state — 11 November 2026

The first local-mission slice is merged into `main` through [PR #2](https://github.com/BRIDGE-ALLAH/ROCKSOUL/pull/2). Its main-branch CI [run #42832504466](https://github.com/BRIDGE-ALLAH/ROCKSOUL/actions/runs/42832504466) passed Rust formatting, five core tests, Clippy, UI typecheck/production build, and Windows compile checking.

**Implemented on main:** Tauri desktop shell; React/TypeScript interface; local SQLite project/mission persistence; native folder-picker wiring with Rust path validation; bounded metadata-only Workspace Scout; SHA-256 JSON artifact; atomic evidence receipt; explicit human approval.

**Automated Windows native lifecycle verified:** [PR #3](https://github.com/BRIDGE-ALLAH/ROCKSOUL/pull/3) passes the native WebDriver scenario on commit `02c258c3cc2f17fa429d61bbe8aa830158aa6d33` in [run #42894857711](https://github.com/BRIDGE-ALLAH/ROCKSOUL/actions/runs/42894857711). The run proves native launch, SQLite vault connection, project/mission creation, Scout, approval, process restart, and restored artifact/evidence. [P0-F01 #5](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/5) owns this automated gate. [P0-F02 #6](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/6) separately owns manual OS-native folder-picker select/cancel/invalid-path acceptance; that interactive test remains unverified. The current checklist is in [WINDOWS_SMOKE_TEST.md](WINDOWS_SMOKE_TEST.md).

**Phase 0 is not complete.** Automated Windows lifecycle is verified, but manual folder-picker acceptance, rename/archive, event history, search, backup/restore, startup/shutdown recovery, keyboard/accessibility acceptance, deletion/reset, data-at-rest review and implementation, design-system/navigation, installer/signing/update/clean-install remain open.

**Phase 1 is not complete.** There is no bounded source-content compiler, approved provider adapter, general tool registry, resumable mission runner, or criterion-backed provider mission. The current metadata Scout is not a general-purpose autonomous agent.

## Phase 0 — FOUNDATION & DESKTOP RELIABILITY

**Target:** a dependable, accessible Windows desktop foundation with honest state and recoverable local data.

Scope includes:
- project lifecycle: create, rename, archive/unarchive, safe unlink/delete;
- local SQLite schema, migrations, event ledger, metadata FTS5 search;
- consistent online backup, export, preflight, restore, and startup/shutdown recovery;
- keyboard and screen-reader semantics, contrast, reduced motion;
- canonical design tokens, shared accessible UI primitives, real-workspace navigation;
- data-at-rest threat model, deletion/reset semantics, Windows native runtime/picker acceptance;
- reproducible Windows installer, signing, update, rollback, and clean-machine gates.

Exit gate:
- [ ] Fresh clone builds under CI on the stated toolchain.
- [ ] Native Windows app launches/exits and reconnects to its local vault.
- [ ] Project/mission lifecycle and folder-picker choose/cancel/invalid-path flows are accepted on a real Windows desktop.
- [ ] App restart and unclean-exit recovery preserve or reconcile durable state.
- [ ] Backup/restore round-trip, integrity, and corruption/failure recovery pass.
- [ ] Keyboard-only, screen-reader, contrast, and reduced-motion checks pass.
- [ ] Installer, signing, update, rollback, uninstall, and clean-machine checks pass before public release.

## Phase 1 — ONE RELIABLE READ-ONLY MISSION

**Target:** one real repository question answered with inspectable context and criterion-specific evidence.

Scope includes:
- versioned mission contract/state/checkpoints/cancellation;
- explicit source catalog and bounded read-only text extraction;
- deterministic context compiler, source search, and context inspector;
- data classification, fail-closed egress, credential store, and one reviewed provider;
- versioned read-only tool manifests and Workspace Scout registration;
- structured plan preview and criterion-specific evidence gates;
- prompt-injection/untrusted-source boundary;
- hypothesis/route/experiment records and a branchable Chaos Canvas that promotes to draft only.

Exit gate:
- [ ] A real repository question is answered by a real provider only after exact context/egress policy permits it.
- [ ] The user can inspect included/excluded/truncated sources and the exact context digest.
- [ ] Only registered read-only tools execute in this phase.
- [ ] Untrusted source content cannot alter grants, policy, egress, approval, or mission scope.
- [ ] Every mandatory criterion maps to fresh, inspectable evidence.
- [ ] Mission state and evidence survive app restart and provider failure.

## Phase 2 — SAFE ACTIONS & EXECUTION AUTHORITY

**Target:** limited writes and command operations in an enforceable OS boundary.

Scope includes:
- typed capability grants, default-deny policy, revocation, and exact-digest approvals;
- canonical filesystem grants and adversarial path/TOCTOU tests;
- supervised child processes and a small typed command catalog—not arbitrary shell;
- reviewed Windows process isolation profile, enforcement, and escape regression suite;
- isolated Git worktree per coding mission;
- immutable patch proposals, exact approval, durable apply journal, rollback, and crash recovery;
- bounded allowlisted test runner with structured evidence.

Exit gate:
- [ ] No operation runs without a current policy Allow and the required grant/approval.
- [ ] Windows filesystem/network/process/resource restrictions are enforced at OS/runtime boundary.
- [ ] Failure to initialize mandatory isolation denies execution; no unsandboxed fallback exists.
- [ ] Traversal, link, TOCTOU, command injection, environment, network, descendant, and resource tests pass.
- [ ] Timeout/cancellation/crash recovery reconcile side effects and preserve an audit trail.
- [ ] Patch application is bound to an exact digest and is recoverable/rollback-capable.

## Phase 3 — PROOF-CARRYING CREATION

**Target:** deliverables that another machine can independently inspect.

Scope includes:
- immutable content-addressed artifact versions and diff review;
- versioned provenance manifests;
- evidence mapped to each acceptance criterion;
- portable export pack and offline integrity verifier.

Exit gate:
- [ ] Every completed mission has artifact, contract, sources, tool/provider versions, approvals, test results, and limitations in its provenance.
- [ ] A second machine can verify exported artifact/evidence integrity offline.
- [ ] A model cannot self-certify a criterion.
- [ ] Failure and unresolved uncertainty remain visible.

## Phase 4 — ADAPTIVE MEMORY, SKILLS & CREW

**Target:** durable continuity and small, bounded specialist collaboration.

Scope includes:
- typed working/episodic/semantic/procedural/preference memory;
- user review, correction, scope, deletion, and evaluated retrieval;
- versioned portable skills and inspect-before-install;
- worker/verifier role contracts, isolated assignments, small crew orchestration, disagreement/critique.

Exit gate:
- [ ] Each memory item has provenance, trust, scope, age, and correction/deletion controls.
- [ ] Retrieval is evaluated on frozen fixtures and respects scope/deletion.
- [ ] Skills cannot grant themselves authority or silently change policy.
- [ ] Worker and verifier operate with separate scope, budget, evidence, and writable workspace.
- [ ] Disagreement is preserved rather than forced into false consensus.

## Phase 5 — REALITY LOOM

**Target:** controlled browser/desktop work integrated with files and approved commands.

Scope includes:
- semantic-first isolated browser adapter and domain allowlists;
- Windows desktop accessibility-tree read adapter;
- consented screenshot/vision fallback;
- fresh-state/precondition/action/post-state verification;
- reproducible hybrid GUI/browser/CLI/file benchmark.

Exit gate:
- [ ] Browser/desktop sessions are explicitly scoped, isolated, and auditable.
- [ ] Each action uses a fresh state snapshot and verifies its postcondition.
- [ ] Missing/ambiguous/stale targets stop safely with human intervention.
- [ ] Non-allowed apps/domains cannot be accessed.
- [ ] Benchmark scores both final artifacts and traces; safety violations are hard failures.

## Phase 6 — MODEL DJ & LOCAL INTELLIGENCE

**Target:** policy-first choice between verified cloud and local model profiles.

Scope includes:
- versioned provider/model capability profiles;
- second provider adapter;
- privacy-first routing;
- local usage/cost/latency accounting;
- optional supervised local inference sidecar;
- explicit offline/local-only mode;
- reproducible provider comparison.

Exit gate:
- [ ] User can inspect why a provider/model was selected.
- [ ] Sensitive or unknown data cannot route to a disallowed endpoint.
- [ ] Local-only mode is functional and states unavailable features honestly.
- [ ] Capability claims are test-verified and versioned.
- [ ] Model changes do not silently change mission harness or evidence semantics.

## Phase 7 — CREATIVE COLLIDER & RESEARCH LAB

**Target:** explicit-consent multimodal work and evaluated long-horizon experiments.

Scope includes:
- creative asset catalog and reference-first import;
- consented image/audio/video workflows;
- bounded counterfactual/simulation sandbox;
- human-reviewed skill improvement proposals;
- opt-in local human-agency/intervention metrics;
- AGILITY long-horizon benchmark.

Exit gate:
- [ ] Every media input/output has source selection, data limits, classification, consent, provenance, and retention semantics.
- [ ] Source assets remain unchanged unless explicit import/copy is chosen.
- [ ] Simulation results show assumptions and uncertainty, not prediction claims.
- [ ] Skill changes require human review and preserve rollback.
- [ ] Metrics are opt-in, local, transparent, and deletable.
- [ ] Benchmarks are reproducible and never marketed as AGI/ASI scores.

## OPS — CROSS-CUTTING DELIVERY

- [ ] OPS-01 defines per-OS/per-capability supported, experimental, or unavailable status; native runtime claims are backed by native tests.
- [ ] OPS-02 generates SBOMs and reviewed license/vulnerability reports with pinned scanner/database versions and time-bounded exceptions.

## Release gates

Every public release must pass the relevant blueprint gates: Rust format/lint/unit/integration/property tests; UI typecheck/component/accessibility tests; native platform runtime tests; migration/backup/restore/recovery; filesystem/process/IPC/credential/egress security; exact-version evaluation; dependency SBOM/license/vulnerability review; signed artifact and update verification; clean-install/rollback/uninstall; and honest known-limitations disclosure.

## Explicit non-goals before v1

- Unrestricted shell or desktop control.
- Agent self-granting or self-modifying policy.
- Autonomous public cloud publishing.
- Always-on camera, microphone, or screen capture.
- Marketplace for arbitrary native plugins.
- Multi-tenant team server.
- Claims of AGI, ASI, consciousness, or guaranteed human-level capability.
