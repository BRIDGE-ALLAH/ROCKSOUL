# ROCKSOUL — ISSUE MAP & DEPENDENCY INDEX

**Issue map · Version 1.0 · 11 November 2026**

This file indexes the GitHub execution backlog. It does not replace the task issue: each issue's `Dependencies`, `Scope`, `Design and engineering rules`, `Explicitly out of scope`, and `Acceptance criteria` sections are authoritative.

- **Program tracker:** [#4 — Execute the Rocksoul blueprint through small, verifiable tasks](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/4)
- **Canonical narrative:** [PRODUCT_NARRATIVE.md](PRODUCT_NARRATIVE.md)
- **Normative blueprint:** [BLUEPRINT.md](BLUEPRINT.md)
- **Local AI execution protocol:** [AI_TASK_PROTOCOL.md](AI_TASK_PROTOCOL.md)
- **Repository rules:** [AGENTS.md](../AGENTS.md)

## Backlog inventory

- **100 task issues** with unique stable IDs covering P0–P7 and OPS.
- **1 umbrella issue** (#4) tracks phase-level completion.
- Every child issue has one bounded outcome, dependencies, implementation scope, design/engineering rules, exclusions, checkbox acceptance criteria, verification steps, and an AI-agent handoff.
- The backlog is a plan, not an implementation claim. Close tasks only after merge and evidence-backed acceptance.

## Phase index

### P0 — Foundation and desktop reliability (23 tasks)

| Task ID | Issue | Task | AC checks | State |
|---|---:|---|---:|---|
| `P0-F01` | [#5](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/5) | Complete the real Windows native desktop E2E lifecycle smoke test | 6 | open |
| `P0-F02` | [#6](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/6) | Accept the native Windows folder picker choose, cancel, and invalid-path behavior | 6 | open |
| `P0-F03` | [#7](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/7) | Implement project rename as an atomic Rust-owned operation | 5 | open |
| `P0-F04` | [#8](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/8) | Add project archive and unarchive without deleting workspace data | 6 | open |
| `P0-F05` | [#9](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/9) | Allow safe editing of draft mission title and objective | 5 | open |
| `P0-F06` | [#10](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/10) | Introduce an append-only typed event ledger schema | 6 | open |
| `P0-F07` | [#11](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/11) | Record project and mission lifecycle changes in the event ledger | 5 | open |
| `P0-F08` | [#12](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/12) | Expose a read-only project and mission event timeline | 5 | open |
| `P0-F09` | [#13](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/13) | Add local FTS5 metadata indexing for project and mission search | 5 | open |
| `P0-F10` | [#14](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/14) | Build a searchable project and mission finder UI | 6 | open |
| `P0-F11` | [#15](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/15) | Implement consistent SQLite online backup and integrity verification | 6 | open |
| `P0-F12` | [#16](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/16) | Add a local backup export flow to the desktop UI | 6 | open |
| `P0-F13` | [#17](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/17) | Implement safe database backup preflight and atomic restore | 6 | open |
| `P0-F14` | [#18](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/18) | Add structured startup, shutdown, and unclean-exit diagnostics | 6 | open |
| `P0-F15` | [#19](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/19) | Make the core desktop workflow fully keyboard navigable | 6 | open |
| `P0-F16` | [#20](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/20) | Add screen-reader semantics, contrast checks, and reduced-motion accessibility tests | 6 | open |
| `P0-F17` | [#21](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/21) | Produce a reproducible unsigned Windows installer artifact | 6 | open |
| `P0-F18` | [#22](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/22) | Define and verify Windows installer signing and update rollback gates | 6 | open |
| `P0-F19` | [#92](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/92) | Define and validate the local data-at-rest protection strategy | 7 | open |
| `P0-F20` | [#95](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/95) | Define the Rocksoul visual design tokens and visual language contract | 7 | open |
| `P0-F21` | [#96](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/96) | Build accessible shared UI primitives for Rocksoul desktop surfaces | 8 | open |
| `P0-F22` | [#97](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/97) | Define the desktop information architecture and implement real-workspace navigation | 8 | open |
| `P0-F23` | [#98](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/98) | Add an explicit and recoverable local-data reset and deletion workflow | 8 | open |

### P1 — One reliable read-only mission (23 tasks)

| Task ID | Issue | Task | AC checks | State |
|---|---:|---|---:|---|
| `P1-M01` | [#23](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/23) | Define a versioned mission contract model and migration | 5 | open |
| `P1-M02` | [#24](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/24) | Build the mission contract editor for scope, constraints, and acceptance criteria | 6 | open |
| `P1-M03` | [#25](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/25) | Enforce the typed mission state machine in Rust | 6 | open |
| `P1-M04` | [#26](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/26) | Add persisted run checkpoints, cancellation, and timeout handling | 6 | open |
| `P1-M05` | [#27](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/27) | Add a project source catalog for explicit files, notes, and links | 5 | open |
| `P1-M06` | [#28](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/28) | Implement bounded, read-only text extraction for explicitly selected sources | 5 | open |
| `P1-M07` | [#29](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/29) | Build an explicit-scope FTS5 content index and source search API | 5 | open |
| `P1-M08` | [#30](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/30) | Implement a deterministic budgeted context compiler | 5 | open |
| `P1-M09` | [#31](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/31) | Build a context inspector and explicit provider-egress review surface | 6 | open |
| `P1-M10` | [#32](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/32) | Define a provider-neutral model request and response contract | 5 | open |
| `P1-M11` | [#33](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/33) | Add a Windows Credential Manager-backed provider secret store | 5 | open |
| `P1-M12` | [#34](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/34) | Implement data classification and fail-closed provider egress policy | 6 | open |
| `P1-M13` | [#35](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/35) | Record the first provider selection as a reviewed architecture decision | 5 | open |
| `P1-M14` | [#36](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/36) | Implement the first real provider adapter behind the neutral contract | 6 | open |
| `P1-M15` | [#37](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/37) | Add provider streaming, cancellation, and resumable failure handling | 5 | open |
| `P1-M16` | [#38](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/38) | Define a versioned read-only tool registry and manifest schema | 5 | open |
| `P1-M17` | [#39](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/39) | Register Workspace Scout as a bounded read-only tool | 5 | open |
| `P1-M18` | [#40](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/40) | Add a structured mission plan model and human-readable plan preview | 5 | open |
| `P1-M19` | [#41](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/41) | Gate mission completion on criterion-specific evidence | 5 | open |
| `P1-M20` | [#42](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/42) | Ship one end-to-end read-only repository research mission | 8 | open |
| `P1-M21` | [#99](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/99) | Enforce an untrusted-source and prompt-injection boundary in mission context | 7 | open |
| `P1-M22` | [#100](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/100) | Add typed hypothesis, route, and experiment records to the mission domain | 7 | open |
| `P1-M23` | [#101](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/101) | Build the branchable Chaos Canvas for hypotheses and experiment review | 8 | open |

### P2 — Safe actions and execution authority (16 tasks)

| Task ID | Issue | Task | AC checks | State |
|---|---:|---|---:|---|
| `P2-A01` | [#43](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/43) | Define the versioned capability grant model and validator | 5 | open |
| `P2-A02` | [#44](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/44) | Implement the Rust default-deny capability policy engine | 5 | open |
| `P2-A03` | [#45](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/45) | Add user-managed capability grant lifecycle and revocation | 6 | open |
| `P2-A04` | [#46](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/46) | Bind action previews and human approvals to an exact digest | 6 | open |
| `P2-A05` | [#47](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/47) | Enforce canonical project-root filesystem grants for read/write operations | 5 | open |
| `P2-A06` | [#48](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/48) | Add adversarial filesystem regression tests for traversal, links, and TOCTOU | 5 | open |
| `P2-A07` | [#49](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/49) | Build a supervised cancellable child-process runtime | 6 | open |
| `P2-A08` | [#50](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/50) | Define a small typed command catalog with no unrestricted shell | 5 | open |
| `P2-A09` | [#51](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/51) | Create and manage one isolated Git worktree per coding mission | 6 | open |
| `P2-A10` | [#52](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/52) | Generate immutable patch proposals and reviewable diffs without applying them | 5 | open |
| `P2-A11` | [#53](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/53) | Apply an approved patch with a durable journal and rollback | 6 | open |
| `P2-A12` | [#54](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/54) | Recover interrupted tool and patch operations after app restart | 5 | open |
| `P2-A13` | [#55](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/55) | Register a bounded test-runner tool with structured evidence | 6 | open |
| `P2-A14` | [#102](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/102) | Select and specify the Windows execution-isolation security profile | 6 | open |
| `P2-A15` | [#103](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/103) | Enforce the reviewed Windows sandbox profile for approved command execution | 9 | open |
| `P2-A16` | [#104](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/104) | Add adversarial regression tests for Windows process-sandbox escapes | 7 | open |

### P3 — Proof-carrying creation (5 tasks)

| Task ID | Issue | Task | AC checks | State |
|---|---:|---|---:|---|
| `P3-A01` | [#56](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/56) | Add immutable content-addressed artifact versions | 5 | open |
| `P3-A02` | [#57](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/57) | Build artifact version history and diff review UI | 5 | open |
| `P3-A03` | [#58](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/58) | Define a versioned artifact provenance manifest | 5 | open |
| `P3-A04` | [#59](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/59) | Show evidence mapped to each mission acceptance criterion | 5 | open |
| `P3-A05` | [#60](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/60) | Create a portable evidence export pack with an offline integrity verifier | 6 | open |

### P4 — Adaptive memory, skills, and crew (10 tasks)

| Task ID | Issue | Task | AC checks | State |
|---|---:|---|---:|---|
| `P4-CR01` | [#67](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/67) | Define typed agent-role contracts for worker and verifier jobs | 5 | open |
| `P4-CR02` | [#68](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/68) | Implement an independent verifier role for mission evidence | 5 | open |
| `P4-CR03` | [#69](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/69) | Orchestrate a small isolated specialist crew with per-role budgets | 6 | open |
| `P4-CR04` | [#70](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/70) | Build a crew disagreement and critique review surface | 5 | open |
| `P4-MEM01` | [#61](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/61) | Add typed project memory records with provenance and trust state | 5 | open |
| `P4-MEM02` | [#62](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/62) | Build user correction, review, and deletion controls for memory | 6 | open |
| `P4-MEM03` | [#63](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/63) | Implement deterministic scope-aware memory retrieval | 5 | open |
| `P4-MEM04` | [#64](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/64) | Create a frozen memory retrieval evaluation set and scorer | 5 | open |
| `P4-SK01` | [#65](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/65) | Define and validate the portable Rocksoul skill manifest | 5 | open |
| `P4-SK02` | [#66](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/66) | Build a skill catalog with inspect-before-install review | 6 | open |

### P5 — Reality Loom and controlled computer use (6 tasks)

| Task ID | Issue | Task | AC checks | State |
|---|---:|---|---:|---|
| `P5-R01` | [#71](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/71) | Implement a semantic-first read-only browser adapter | 6 | open |
| `P5-R02` | [#72](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/72) | Add browser domain allowlists, isolated sessions, and action traces | 6 | open |
| `P5-R03` | [#73](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/73) | Add a Windows desktop accessibility-tree read adapter | 5 | open |
| `P5-R04` | [#74](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/74) | Add a consented screenshot and vision fallback for inaccessible UI | 5 | open |
| `P5-R05` | [#75](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/75) | Enforce re-read, precondition, and post-action verification for computer-use operations | 5 | open |
| `P5-R06` | [#76](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/76) | Create a reproducible hybrid GUI, browser, CLI, and file-work benchmark | 6 | open |

### P6 — Model DJ and local intelligence (7 tasks)

| Task ID | Issue | Task | AC checks | State |
|---|---:|---|---:|---|
| `P6-D01` | [#77](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/77) | Add a versioned model capability and profile registry | 5 | open |
| `P6-D02` | [#78](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/78) | Implement a second provider adapter using the neutral contract | 6 | open |
| `P6-D03` | [#79](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/79) | Implement privacy-first model routing across verified profiles | 5 | open |
| `P6-D04` | [#80](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/80) | Track local model usage, cost estimates, and latency per mission | 6 | open |
| `P6-D05` | [#81](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/81) | Add an optional supervised local inference sidecar | 6 | open |
| `P6-D06` | [#82](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/82) | Add an explicit local-only and offline operating mode | 6 | open |
| `P6-D07` | [#83](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/83) | Build a reproducible model and provider comparison evaluation suite | 5 | open |

### P7 — Creative Collider and long-horizon research (8 tasks)

| Task ID | Issue | Task | AC checks | State |
|---|---:|---|---:|---|
| `P7-C01` | [#84](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/84) | Add a local creative-asset catalog and explicit import workflow | 5 | open |
| `P7-C02` | [#85](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/85) | Add an explicitly consented image creation workflow | 5 | open |
| `P7-C03` | [#86](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/86) | Add an explicitly consented audio workflow | 6 | open |
| `P7-C04` | [#87](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/87) | Add a bounded video workflow with frame and provenance controls | 6 | open |
| `P7-C05` | [#88](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/88) | Build a bounded counterfactual scenario sandbox | 6 | open |
| `P7-C06` | [#89](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/89) | Add human-reviewed skill improvement proposals | 6 | open |
| `P7-C07` | [#90](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/90) | Add opt-in local human-agency and intervention metrics | 6 | open |
| `P7-C08` | [#91](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/91) | Build the AGILITY long-horizon evaluation benchmark | 7 | open |

### OPS — Cross-cutting platform and supply-chain operations (2 tasks)

| Task ID | Issue | Task | AC checks | State |
|---|---:|---|---:|---|
| `OPS-01` | [#93](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/93) | Define the supported OS matrix and add cross-platform core/runtime CI | 7 | open |
| `OPS-02` | [#94](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/94) | Add dependency governance, SBOM, license, and vulnerability release gates | 7 | open |

## Dependency sequencing

The task issues carry the exact dependency edge list. The following are major gates, not substitutes for the individual issue dependencies.

### P0 — Desktop foundation
- Native runtime baseline: [P0-F01 #5](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/5) finishes the existing WebDriver/Tauri PR #3; [P0-F02 #6](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/6) separately validates the native Windows folder-picker dialog.
- Event history: [F06 #10](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/10) → [F07 #11](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/11) → [F08 #12](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/12).
- Search: [F09 #13](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/13) → [F10 #14](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/14).
- Database recovery: [F11 #15](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/15) → [F12 #16](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/16) and [F13 #17](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/17); [F14 #18](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/18) participates in recovery.
- Installer: [F17 #21](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/21) → [F18 #22](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/22).
- UX foundation: [F20 #95](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/95) → [F21 #96](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/96) → [F22 #97](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/97).
- Data lifecycle: [F19 #92](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/92) plus backup [F11 #15](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/15) → [F23 #98](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/98).

### P1 — One reliable read-only mission
- Mission contract/state: [M01 #23](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/23) → [M02 #24](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/24), [M03 #25](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/25), and [M04 #26](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/26).
- Context pipeline: [M05 #27](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/27) → [M06 #28](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/28) → [M07 #29](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/29) → [M08 #30](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/30) → [M09 #31](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/31).
- Provider/egress: [M10 #32](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/32) → [M11 #33](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/33), [M12 #34](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/34), [M13 #35](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/35) → [M14 #36](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/36) → [M15 #37](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/37).
- Read-only harness: [M16 #38](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/38) → [M17 #39](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/39); plan [M18 #40](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/40); evidence gate [M19 #41](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/41); complete slice [M20 #42](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/42).
- Untrusted context: [M21 #99](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/99) builds on source/context/egress contracts and protects provider/tool boundaries.
- Creative exploration: [M22 #100](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/100) → [M23 #101](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/101); route promotion creates a draft only.

### P2 — Safe actions
- Authority: [A01 #43](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/43) → [A02 #44](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/44) → [A03 #45](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/45) → [A04 #46](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/46).
- Filesystem: [A05 #47](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/47) → [A06 #48](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/48).
- Process/command: [A07 #49](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/49) → [A08 #50](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/50) → [A09 #51](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/51) → [A10 #52](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/52) → [A11 #53](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/53); recovery [A12 #54](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/54); test runner [A13 #55](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/55).
- Windows isolation is separate from process-tree supervision: [A14 #102](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/102) ADR → [A15 #103](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/103) enforcement → [A16 #104](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/104) adversarial tests.

### P3–P7 and OPS
- P3: artifact versioning, diffs, provenance, criterion evidence, and offline export (#56–#60).
- P4: typed memory, correction/deletion, retrieval evaluation, skill manifests/review, role contracts, independent verifier, bounded crew, disagreement (#61–#70).
- P5: semantic browser/desktop adapters, session policy, screenshot fallback, per-action verification, hybrid benchmark (#71–#76).
- P6: verified model profiles, provider adapters, privacy routing, usage, local sidecar/offline mode, model evaluation (#77–#83).
- P7: asset catalog, explicit image/audio/video, simulation, skill-improvement proposal, opt-in human metrics, AGILITY benchmark (#84–#91).
- OPS-01 (#93) defines per-platform support claims; OPS-02 (#94) defines dependency/SBOM/license/vulnerability release gates.

## Issue authoring standard

Every task issue retains:
1. Outcome
2. Blueprint trace
3. Dependencies
4. Scope and implementation specification
5. Design and engineering rules
6. Explicitly out of scope
7. Acceptance criteria with checkboxes
8. Verification
9. AI-agent handoff

Do not create a second issue for an existing task ID. A new task needs a unique owner boundary and must not duplicate acceptance criteria already owned by another task.

## Counts

| Phase | Tasks |
|---|---:|
| P0 — Foundation and desktop reliability | 23 |
| P1 — One reliable read-only mission | 23 |
| P2 — Safe actions and execution authority | 16 |
| P3 — Proof-carrying creation | 5 |
| P4 — Adaptive memory, skills, and crew | 10 |
| P5 — Reality Loom and controlled computer use | 6 |
| P6 — Model DJ and local intelligence | 7 |
| P7 — Creative Collider and long-horizon research | 8 |
| OPS — Cross-cutting platform and supply-chain operations | 2 |
| **Total** | **100** |

Update this file whenever an issue is created, split, superseded, merged, or closed. Retain historical task IDs with their final state and evidence.
