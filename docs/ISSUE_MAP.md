# ROCKSOUL — ISSUE MAP & DEPENDENCY INDEX

**Issue map · Version 1.0 · 11 November 2026**

This file indexes the GitHub execution backlog. It does not replace the task issue: each issue's `Dependencies`, `Scope`, `Design and engineering rules`, `Explicitly out of scope`, and `Acceptance criteria` sections are authoritative.

- **Program tracker:** [#4 — Execute the Rocksoul blueprint through small, verifiable tasks](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/4)
- **Canonical narrative:** [PRODUCT_NARRATIVE.md](PRODUCT_NARRATIVE.md)
- **Normative blueprint:** [BLUEPRINT.md](BLUEPRINT.md)
- **Local AI execution protocol:** [AI_TASK_PROTOCOL.md](AI_TASK_PROTOCOL.md)

## Backlog inventory

- **100 task issues** with stable IDs: P0 through P7 plus OPS.
- **1 umbrella issue** (#4) to track phase completion and maintain the backlog.
- Each child issue has a single bounded outcome, dependencies, explicit scope fence, engineering/design rules, checkable acceptance criteria, verification instructions, and an AI-agent handoff.
- The backlog is not a claim that any task is implemented. Close issues only after merge and evidence-backed acceptance.

## Phase index

### P0 — Foundation and desktop reliability (23 tasks)

| Task ID | Issue | Task | AC checks |
|---|---:|---|---:|
| `P0-F01` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Complete the real Windows native desktop E2E lifecycle smoke test | 6 |
| `P0-F02` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Accept the native Windows folder picker choose, cancel, and invalid-path behavior | 6 |
| `P0-F03` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Implement project rename as an atomic Rust-owned operation | 5 |
| `P0-F04` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Add project archive and unarchive without deleting workspace data | 6 |
| `P0-F05` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Allow safe editing of draft mission title and objective | 5 |
| `P0-F06` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Introduce an append-only typed event ledger schema | 6 |
| `P0-F07` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Record project and mission lifecycle changes in the event ledger | 5 |
| `P0-F08` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Expose a read-only project and mission event timeline | 5 |
| `P0-F09` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Add local FTS5 metadata indexing for project and mission search | 5 |
| `P0-F10` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Build a searchable project and mission finder UI | 6 |
| `P0-F11` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Implement consistent SQLite online backup and integrity verification | 6 |
| `P0-F12` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Add a local backup export flow to the desktop UI | 6 |
| `P0-F13` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Implement safe database backup preflight and atomic restore | 6 |
| `P0-F14` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Add structured startup, shutdown, and unclean-exit diagnostics | 6 |
| `P0-F15` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Make the core desktop workflow fully keyboard navigable | 6 |
| `P0-F16` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Add screen-reader semantics, contrast checks, and reduced-motion accessibility tests | 6 |
| `P0-F17` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Produce a reproducible unsigned Windows installer artifact | 6 |
| `P0-F18` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Define and verify Windows installer signing and update rollback gates | 6 |
| `P0-F19` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Define and validate the local data-at-rest protection strategy | 7 |
| `P0-F20` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Define the Rocksoul visual design tokens and visual language contract | 7 |
| `P0-F21` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Build accessible shared UI primitives for Rocksoul desktop surfaces | 8 |
| `P0-F22` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Define the desktop information architecture and implement real-workspace navigation | 8 |
| `P0-F23` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Add an explicit and recoverable local-data reset and deletion workflow | 8 |

### P1 — One reliable read-only mission (23 tasks)

| Task ID | Issue | Task | AC checks |
|---|---:|---|---:|
| `P1-M01` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Define a versioned mission contract model and migration | 5 |
| `P1-M02` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Build the mission contract editor for scope, constraints, and acceptance criteria | 6 |
| `P1-M03` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Enforce the typed mission state machine in Rust | 6 |
| `P1-M04` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Add persisted run checkpoints, cancellation, and timeout handling | 6 |
| `P1-M05` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Add a project source catalog for explicit files, notes, and links | 5 |
| `P1-M06` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Implement bounded, read-only text extraction for explicitly selected sources | 5 |
| `P1-M07` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Build an explicit-scope FTS5 content index and source search API | 5 |
| `P1-M08` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Implement a deterministic budgeted context compiler | 5 |
| `P1-M09` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Build a context inspector and explicit provider-egress review surface | 6 |
| `P1-M10` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Define a provider-neutral model request and response contract | 5 |
| `P1-M11` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Add a Windows Credential Manager-backed provider secret store | 5 |
| `P1-M12` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Implement data classification and fail-closed provider egress policy | 6 |
| `P1-M13` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Record the first provider selection as a reviewed architecture decision | 5 |
| `P1-M14` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Implement the first real provider adapter behind the neutral contract | 6 |
| `P1-M15` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Add provider streaming, cancellation, and resumable failure handling | 5 |
| `P1-M16` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Define a versioned read-only tool registry and manifest schema | 5 |
| `P1-M17` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Register Workspace Scout as a bounded read-only tool | 5 |
| `P1-M18` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Add a structured mission plan model and human-readable plan preview | 5 |
| `P1-M19` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Gate mission completion on criterion-specific evidence | 5 |
| `P1-M20` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Ship one end-to-end read-only repository research mission | 8 |
| `P1-M21` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Enforce an untrusted-source and prompt-injection boundary in mission context | 7 |
| `P1-M22` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Add typed hypothesis, route, and experiment records to the mission domain | 7 |
| `P1-M23` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Build the branchable Chaos Canvas for hypotheses and experiment review | 8 |

### P2 — Safe actions and execution authority (16 tasks)

| Task ID | Issue | Task | AC checks |
|---|---:|---|---:|
| `P2-A01` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Define the versioned capability grant model and validator | 5 |
| `P2-A02` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Implement the Rust default-deny capability policy engine | 5 |
| `P2-A03` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Add user-managed capability grant lifecycle and revocation | 6 |
| `P2-A04` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Bind action previews and human approvals to an exact digest | 6 |
| `P2-A05` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Enforce canonical project-root filesystem grants for read/write operations | 5 |
| `P2-A06` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Add adversarial filesystem regression tests for traversal, links, and TOCTOU | 5 |
| `P2-A07` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Build a supervised cancellable child-process runtime | 6 |
| `P2-A08` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Define a small typed command catalog with no unrestricted shell | 5 |
| `P2-A09` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Create and manage one isolated Git worktree per coding mission | 6 |
| `P2-A10` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Generate immutable patch proposals and reviewable diffs without applying them | 5 |
| `P2-A11` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Apply an approved patch with a durable journal and rollback | 6 |
| `P2-A12` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Recover interrupted tool and patch operations after app restart | 5 |
| `P2-A13` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Register a bounded test-runner tool with structured evidence | 6 |
| `P2-A14` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Select and specify the Windows execution-isolation security profile | 6 |
| `P2-A15` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Enforce the reviewed Windows sandbox profile for approved command execution | 9 |
| `P2-A16` | [#104](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/104) | Add adversarial regression tests for Windows process-sandbox escapes | 7 |

### P3 — Proof-carrying creation (5 tasks)

| Task ID | Issue | Task | AC checks |
|---|---:|---|---:|
| `P3-A01` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Add immutable content-addressed artifact versions | 5 |
| `P3-A02` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Build artifact version history and diff review UI | 5 |
| `P3-A03` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Define a versioned artifact provenance manifest | 5 |
| `P3-A04` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Show evidence mapped to each mission acceptance criterion | 5 |
| `P3-A05` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Create a portable evidence export pack with an offline integrity verifier | 6 |

### P4 — Adaptive memory, skills, and crew (10 tasks)

| Task ID | Issue | Task | AC checks |
|---|---:|---|---:|
| `P4-CR01` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Define typed agent-role contracts for worker and verifier jobs | 5 |
| `P4-CR02` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Implement an independent verifier role for mission evidence | 5 |
| `P4-CR03` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Orchestrate a small isolated specialist crew with per-role budgets | 6 |
| `P4-CR04` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Build a crew disagreement and critique review surface | 5 |
| `P4-MEM01` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Add typed project memory records with provenance and trust state | 5 |
| `P4-MEM02` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Build user correction, review, and deletion controls for memory | 6 |
| `P4-MEM03` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Implement deterministic scope-aware memory retrieval | 5 |
| `P4-MEM04` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Create a frozen memory retrieval evaluation set and scorer | 5 |
| `P4-SK01` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Define and validate the portable Rocksoul skill manifest | 5 |
| `P4-SK02` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Build a skill catalog with inspect-before-install review | 6 |

### P5 — Reality Loom and controlled computer use (6 tasks)

| Task ID | Issue | Task | AC checks |
|---|---:|---|---:|
| `P5-R01` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Implement a semantic-first read-only browser adapter | 6 |
| `P5-R02` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Add browser domain allowlists, isolated sessions, and action traces | 6 |
| `P5-R03` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Add a Windows desktop accessibility-tree read adapter | 5 |
| `P5-R04` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Add a consented screenshot and vision fallback for inaccessible UI | 5 |
| `P5-R05` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Enforce re-read, precondition, and post-action verification for computer-use operations | 5 |
| `P5-R06` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Create a reproducible hybrid GUI, browser, CLI, and file-work benchmark | 6 |

### P6 — Model DJ and local intelligence (7 tasks)

| Task ID | Issue | Task | AC checks |
|---|---:|---|---:|
| `P6-D01` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Add a versioned model capability and profile registry | 5 |
| `P6-D02` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Implement a second provider adapter using the neutral contract | 6 |
| `P6-D03` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Implement privacy-first model routing across verified profiles | 5 |
| `P6-D04` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Track local model usage, cost estimates, and latency per mission | 6 |
| `P6-D05` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Add an optional supervised local inference sidecar | 6 |
| `P6-D06` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Add an explicit local-only and offline operating mode | 6 |
| `P6-D07` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Build a reproducible model and provider comparison evaluation suite | 5 |

### P7 — Creative Collider and long-horizon research (8 tasks)

| Task ID | Issue | Task | AC checks |
|---|---:|---|---:|
| `P7-C01` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Add a local creative-asset catalog and explicit import workflow | 5 |
| `P7-C02` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Add an explicitly consented image creation workflow | 5 |
| `P7-C03` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Add an explicitly consented audio workflow | 6 |
| `P7-C04` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Add a bounded video workflow with frame and provenance controls | 6 |
| `P7-C05` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Build a bounded counterfactual scenario sandbox | 6 |
| `P7-C06` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Add human-reviewed skill improvement proposals | 6 |
| `P7-C07` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Add opt-in local human-agency and intervention metrics | 6 |
| `P7-C08` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Build the AGILITY long-horizon evaluation benchmark | 7 |

### OPS — Cross-cutting platform and supply-chain operations (3 tasks)

| Task ID | Issue | Task | AC checks |
|---|---:|---|---:|
| `OPS-01` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Define the supported OS matrix and add cross-platform core/runtime CI | 7 |
| `OPS-02` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Add dependency governance, SBOM, license, and vulnerability release gates | 7 |
| `PROGRAM` | [#undefined](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/undefined) | Execute the Rocksoul blueprint through small, verifiable tasks | 0 |

## Dependency sequencing

The task issues carry the exact edge list. The following are the major sequencing gates; they are not substitutes for the individual dependency sections.

### P0 — Desktop foundation

- Native runtime baseline: [P0-F01 #5](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/5) uses the existing WebDriver/Tauri PR #3; [P0-F02 #6](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/6) separately validates the native Windows shell folder picker.
- Event history: [F06 #10](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/10) → [F07 #11](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/11) → [F08 #12](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/12).
- Search: [F09 #13](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/13) → [F10 #14](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/14).
- Database recovery: [F11 #15](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/15) → [F12 #16](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/16) and [F13 #17](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/17); [F14 #18](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/18) participates in startup/restore recovery.
- Installer: [F17 #21](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/21) → [F18 #22](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/22).
- UX foundation: [F20 #95](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/95) → [F21 #96](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/96) → [F22 #97](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/97).
- Data lifecycle: [F19 #92](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/92) and backup semantics [F11 #15](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/15) → [F23 #98](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/98).

### P1 — One reliable read-only mission

- Mission contract and state: [M01 #23](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/23) → [M02 #24](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/24), [M03 #25](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/25), and [M04 #26](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/26).
- Context pipeline: [M05 #27](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/27) → [M06 #28](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/28) → [M07 #29](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/29) → [M08 #30](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/30) → [M09 #31](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/31).
- Provider and egress: [M10 #32](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/32) → [M11 #33](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/33), [M12 #34](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/34), [M13 #35](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/35) → [M14 #36](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/36) → [M15 #37](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/37).
- Read-only harness: [M16 #38](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/38) → [M17 #39](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/39); plan [M18 #40](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/40); evidence gate [M19 #41](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/41); complete vertical slice [M20 #42](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/42).
- Untrusted context: [M21 #99](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/99) depends on source/context/egress contracts; its adversarial fixtures protect M06/M08/M09 and provider/tool boundaries.
- Creative exploration: [M22 #100](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/100) → [M23 #101](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/101); branch promotion creates a draft only.

### P2 — Safe actions

- Authority: [A01 #43](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/43) → [A02 #44](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/44) → [A03 #45](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/45) → [A04 #46](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/46).
- Filesystem: [A05 #47](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/47) → [A06 #48](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/48).
- Process/command: [A07 #49](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/49) → [A08 #50](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/50) → [A09 #51](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/51) → [A10 #52](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/52) → [A11 #53](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/53); recovery [A12 #54](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/54); test runner [A13 #55](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/55).
- Windows isolation: [A14 #102](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/102) ADR → [A15 #103](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/103) enforcement → [A16 #104](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/104) adversarial regression suite. This is distinct from process-tree supervision in A07.

### P3–P7 and OPS

- P3 artifact/version/provenance/evidence/export progresses from [A01 #56](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/56) through [A05 #60](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/60).
- P4 memory, skills, and crew issues declare their data/role dependencies individually; the verifier and crew must follow capability policy and evidence contracts.
- P5 browser/desktop surfaces depend on policy, grants, process/session isolation, and exact action verification before the hybrid benchmark.
- P6 provider/model routing depends on the provider-neutral contract, first reviewed provider, egress policy, and verified model profiles.
- P7 media and simulations reuse the same asset, mission, egress, artifact, and evidence contracts; they are not a separate security model.
- OPS-01 (#93) governs per-platform support claims. OPS-02 (#94) governs dependency/SBOM/license/vulnerability release gates.

## Issue authoring standard

All task issues must retain these headings:
1. Outcome
2. Blueprint trace
3. Dependencies
4. Scope and implementation specification
5. Design and engineering rules
6. Explicitly out of scope
7. Acceptance criteria (checkboxes)
8. Verification
9. AI-agent handoff

Do not create a second issue for an existing task ID. If scope is missing, open a proposed new ID only after updating this map and the program tracker. New tasks must have a unique owner boundary and must not duplicate an existing task's acceptance criteria.

## Counting and maintenance

| Category | Task count |
|---|---:|
| P0 — Foundation | 23 |
| P1 — Reliable mission | 23 |
| P2 — Safe actions | 16 |
| P3 — Proof-carrying creation | 5 |
| P4 — Memory and crew | 10 |
| P5 — Reality Loom | 6 |
| P6 — Model DJ | 7 |
| P7 — Creative Collider | 8 |
| OPS — Cross-cutting | 3 |
| **Total** | **101** |

Update this file whenever issues are created, superseded, split, merged, or closed. Closed work remains listed with its status and evidence; never delete historical task IDs.
