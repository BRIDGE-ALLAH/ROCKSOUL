# ROCKSOUL — PRODUCT & ENGINEERING BLUEPRINT

**Status:** normative product blueprint  
**Audience:** product owner, engineers, designers, security reviewers, local AI coding agents  
**Relationship:** this document defines what is being built and the constraints under which it may be built. The issue tracker decomposes its remaining work into independently verifiable tasks.

Companion documents:
- [Product narrative](PRODUCT_NARRATIVE.md) — positioning, user promise, voice, and creative loop.
- [AI task protocol](AI_TASK_PROTOCOL.md) — how an autonomous local coding agent must execute one issue safely.
- [Roadmap](ROADMAP.md) — capability phases and exit gates.
- [Security posture](../SECURITY.md) — current and required trust invariants.
- [Issue map](ISSUE_MAP.md) — issue-by-issue traceability to this blueprint.

## 1. Product definition

Rocksoul is a **local-first creative-intelligence desktop studio** for moving from rough ideas and difficult work to scoped missions, real artifacts, and verifiable outcomes.

It is not a chatbot, unrestricted autonomous agent, or cloud control plane. The product combines:
- A persistent project workspace.
- A mission contract and state machine.
- An inspectable context compiler.
- A replaceable model/provider layer.
- A Rust-owned capability and policy boundary.
- A bounded tool runtime.
- Artifact, provenance, and evidence storage.
- Human review, approval, correction, and recovery.
- Evaluations that measure the work and its trajectory.

The governing loop is **SPARK → RIFF → THE PIT → FORGE → SOUND CHECK → STAGE**. In implementation, these beats map to domain records, state transitions, tool operations, artifacts, evidence, and approvals—not to decorative labels alone.

## 2. Current baseline — distinguish shipped from planned

The following is the first vertical slice already merged to main by PR #2:
- Tauri 2 desktop host with React/TypeScript UI.
- Rust core crate and SQLite/SQLx local store.
- Project and mission creation/persistence.
- Native directory picker wiring and Rust-side path validation.
- Bounded metadata-only Workspace Scout (2,000 total entries, 300 file records, depth 12).
- Git ignore-aware traversal, secret-name exclusions, no symlink following, no source/document/binary content reads.
- One inventory artifact with SHA-256 and an atomic evidence receipt.
- Explicit human approval after review.

This slice does **not** provide source-content context, model inference, provider egress, general tool registry, shell, patching, multi-agent orchestration, typed long-term memory, computer use, backup/restore, installer/update, or cross-platform release readiness.

PR #3 is a separate draft effort for Windows native WebDriver E2E. Its automated lifecycle test and manual native folder-picker acceptance remain pending until their exact run and manual checks pass. Do not treat an open or draft PR as shipped capability.

## 3. Product principles (non-negotiable)

1. **Local-first:** data is stored locally by default. Any network transfer is explicit, scoped, visible, and policy checked.
2. **Rust owns trust:** Rust and OS controls enforce persistence, file/process access, credentials, network policy, approvals, and tool execution. UI state is never authorization.
3. **Deny by default:** no capability exists because a model, skill, or user prompt mentions it. Grants are explicit, narrow, revocable, and time/budget limited.
4. **Human-led:** user owns mission goal, grants, consequential approvals, and final disposition.
5. **Evidence over narration:** model prose is untrusted. Completion requires durable state and criteria-specific evidence.
6. **One vertical slice at a time:** deliver an end-to-end path rather than placeholder frameworks, simulated screens, or speculative infrastructure.
7. **Replaceable models:** provider SDKs never leak into core domain contracts.
8. **Recovery is a feature:** cancellation, timeout, restart, partial failure, and rollback have defined states and tests.
9. **Inspectability:** context, plan, policy decisions, actions, cost, output, and evidence can be inspected at appropriate privacy levels.
10. **Measured complexity:** no vector database, large agent framework, queue, microservice, sidecar, or sandbox runtime without a concrete problem statement and evaluation.

## 4. System architecture

### 4.1 Trust zones

| Zone | Components | Responsibility |
|---|---|---|
| Presentation | React + TypeScript | Project canvas, mission view, plan, trace, approvals, artifact/evidence review, settings |
| Native boundary | Tauri host + typed commands/events | OS integration, command registration, window lifecycle, scoped native dialogs |
| Rust application core | Domain services, mission harness, context compiler, policy engine, capability broker, tool runtime | Validation, authority, state transitions, execution, persistence, evidence |
| Local persistence | SQLite, FTS5, local artifact storage | Migrations, WAL, event history, search indexes, artifact metadata/content |
| OS integration | Credential manager, native file dialogs, process supervision, worktree isolation | Platform-backed authority and isolation |
| External egress | Provider adapters behind a policy gateway | Approved network transfer, model requests, usage and outcome capture |

The external network is not an implicit extension of the UI. All egress must pass through a single auditable policy boundary. A separate worker process is added only when crash isolation, platform sandboxing, or hostile-code execution justifies it; use private authenticated IPC, never an unauthenticated localhost control server.

### 4.2 Ownership boundaries

| Layer | Owns | Must not own |
|---|---|---|
| React/TypeScript | Views, layout, form feedback, local ephemeral state, accessibility, artifact/trace rendering | Filesystem authority, database policy, credential access, tool authorization, shell execution |
| Tauri host | OS integration, command registration, window lifecycle, scoped native dialogs | Product policy hidden in event handlers |
| Rust domain/core | Entities, invariants, mission state, use-cases, policy, persistence, action execution, evidence | Provider-specific domain assumptions |
| Persistence | Migrations, transactions, event ledger, search indexes, artifact metadata/content | Unreviewed business logic encoded only in SQL triggers |
| Provider adapters | Translate provider-neutral requests/responses; expose capabilities and usage | Permission escalation, data-class changes, direct DB mutation |
| Tool adapters | Implement a typed operation under a granted capability | Self-authorization, hidden side effects, unbounded access |
| Local worker/sidecar (later) | Isolated inference or process execution | Independent policy decisions or alternate persistence truth |

### 4.3 Core modules and contracts

1. **Project domain:** identity, canonical root, metadata, archive state, project-level rules and sources.
2. **Mission domain:** immutable mission revision/contract, state machine, attempts, checkpoints, cancellation, outcome.
3. **Context compiler:** source selection, retrieval, filtering, ranking, deduplication, byte/token budgets, context snapshot hash.
4. **Provider contract:** typed request/response, streaming events, model capabilities, usage, timeout and cancellation.
5. **Capability registry:** versioned manifests describing operation, resource, data class, effects, network, budget, timeout, approval, and evidence.
6. **Policy engine:** deterministic decision of allow/deny/needs approval plus reason and policy version.
7. **Approval service:** binds one exact preview/action digest, target, grant, actor, expiry, and policy version.
8. **Tool runtime:** validates grants again at execution, supervises calls/processes, captures structured output, enforces budget/cancel.
9. **Evidence collector:** records source refs, artifact hashes, action/approval/policy IDs, test result, environment, and unresolved claims.
10. **Artifact manager:** immutable versions, content hashes, diff metadata, export/import integrity.
11. **Memory engine (later):** typed records and provenance; all memory retrieval passes scope and deletion policy.
12. **Evaluation harness:** frozen fixtures and versioned model/harness/tool/skill/environment metadata.

Each module has a small Rust API and contract tests. Avoid cross-module shared mutable state and provider SDK imports in the domain crate.

## 5. Domain model and durable records

The domain model evolves through additive, versioned migrations. Names below are target concepts; do not assume a table already exists unless the current schema proves it.

| Entity | Purpose | Required invariant |
|---|---|---|
| Project | Local work container and policy scope | Root is canonicalized and validated by Rust; archive does not silently delete data |
| Mission | One bounded unit of work | Has a versioned contract, state, attempt history, and owner |
| MissionRevision | Immutable objective/scope/constraints/criteria snapshot | Any edit creates a new revision; prior approvals point to prior digest |
| MissionEvent | Append-only state/action history | Corrections append; history is not silently rewritten |
| Source | File, note, URL, document, or other input reference | Has origin, scope, classification, version/hash where possible |
| ContextSnapshot | Exact bounded input package selected for a run | Has manifest, byte/token budget, source IDs, and content hash |
| Plan | Proposed steps and risk | Versioned; approval references exact digest |
| CapabilityManifest | Operation contract and required authority | Versioned, validated, deny-by-default |
| CapabilityGrant | User/policy-authorized scope | Narrow target/operation, expiry/budget, revocation state |
| ApprovalRequest | Human decision for one consequence | Binds preview digest, actor, target, effects, expiry, policy |
| ToolInvocation | One execution attempt | Captures input reference, grant, timeout, cancellation, output reference and status |
| Artifact | Deliverable metadata and content/version reference | Immutable version, SHA-256/content hash, provenance |
| Evidence | Proof record for a criterion/action | Typed, append-only, has source and time |
| MemoryItem | Reviewed or provisional knowledge | Type, scope, provenance, confidence/status, correction/deletion lifecycle |
| Skill | Portable instruction + manifest + fixtures | Cannot grant capabilities; version and compatibility declared |
| AgentRole | Specialist job contract | Typed input/output, scoped context/tools, budget, verifier criteria |
| EvalRun | Reproducible evaluation | Pins model, harness, tools, prompts/skills, task data, and environment |

Sensitive file contents must not be copied into logs or ordinary metadata records. Secrets belong in OS-backed credential stores, not project tables, prompt context, or exported evidence.

## 6. Mission contract and lifecycle

### 6.1 Mission contract

A mission must eventually carry:
- Objective and expected deliverable.
- In-scope and out-of-scope resources/actions.
- Constraints and explicit prohibitions.
- Acceptance criteria, each with a deterministic or reviewable verifier.
- Selected sources and data classifications.
- Capability requests (not grants).
- Budgets: time, tokens/bytes, tool calls, cost, and concurrency where applicable.
- Risk level and required approval points.
- Owner and mission revision/digest.

A user may draft an incomplete mission, but it cannot become runnable until required fields are valid.

### 6.2 Target state machine

| Current state | Allowed next states | Meaning |
|---|---|---|
| Draft | Ready, Blocked | User is defining the mission |
| Ready | ContextBuilding, Cancelled | Contract is complete and may start |
| ContextBuilding | Planning, NeedsHuman, Failed, Cancelled | Sources are selected and bounded |
| Planning | AwaitingApproval, NeedsHuman, Failed, Cancelled | Proposed work and risk are assembled |
| AwaitingApproval | Executing, Blocked, Cancelled | A human decision is required for a consequential plan |
| Executing | Verifying, NeedsHuman, Failed, Cancelled, RecoveryRequired | Authorized work is running |
| Verifying | Review, NeedsHuman, Failed | Evidence is checked against criteria |
| Review | Completed, Ready | User accepts or asks for revision |
| RecoveryRequired | RolledBack, NeedsHuman, Failed | Interrupted side effects are reconciled |

This is a target contract, not a statement that the current prototype already implements every state. Existing draft, review, and completed behavior must be migrated compatibly, never silently renamed in a breaking migration.

Rules:
- Every transition is performed by a Rust use-case with a validated source state and actor.
- Persist event + state projection atomically.
- State transitions are idempotent where retries are possible.
- Cancellation is cooperative, persisted, and observed by the active operation.
- Timeout is a terminal or resumable outcome, never an indefinite spinner.
- A restart recovers the last committed checkpoint and identifies incomplete side effects.
- Completed requires all mandatory acceptance criteria to have evidence and no unresolved blocking state.

## 7. Data, search, context, and memory

### 7.1 Local persistence
- SQLite with foreign keys, WAL, busy timeout, explicit migration history, and integrity checks.
- FTS5 for deterministic text retrieval; indexes are derived and rebuildable.
- Event ledger and original artifact bytes are the durable record.
- Backups use SQLite-safe online backup/snapshot semantics; file-copying a live WAL database is not sufficient.
- Restore is staged, integrity checked, schema compatible, and atomic from the app's point of view.
- Encryption/key management is a separate security decision before sensitive personal data or secrets are stored.

### 7.2 Context compilation
- Context is selected per mission, not dumped wholesale.
- Start from mission contract, project rules, explicit sources, and relevant prior decisions.
- Retrieval order: explicit user-selected sources, exact path/name, FTS5, symbol/structure search, recency and known links.
- Embedding/vector search is optional and must demonstrate retrieval lift on a frozen fixture set.
- Enforce per-file, total-byte/token, source-count, and time budgets.
- Record a context manifest and hash, with redaction and provenance metadata.
- Show the user what is included, excluded, truncated, and sent to a provider before egress.

### 7.3 Memory lifecycle
Separate working, episodic, semantic, procedural, interpretation, and preference records. Every reusable memory has project/user scope, provenance, timestamp, confidence/status, and correction/deletion history. Unverified model interpretation is never silently promoted to fact. Deletion semantics include derived indexes and future retrieval caches.

## 8. Capability, security, and action model

### 8.1 Capability manifest fields
A manifest must declare:
- Stable ID, version, name, description, and owner.
- Operation and resource type.
- Read/write/execute effect.
- Allowed roots, file patterns, hosts, apps, or object IDs.
- Data classes allowed in/out and egress destination.
- Required approval mode.
- Time, byte/token, process, cost, and concurrency budgets.
- Timeout, cancellation, and idempotency behavior.
- Evidence and verification requirements.
- Platform support and known limitations.

A skill, model, agent, or imported manifest can request a capability; it cannot grant itself one.

### 8.2 Policy decision
Rust returns a typed decision: Allow, Deny, or NeedsApproval, with policy version and human-readable reason. Denial is the default for missing, invalid, ambiguous, expired, or unsupported grants.

The UI may render the decision but cannot override it. Every tool invocation re-checks the grant immediately before execution; prior UI approval is not a reusable blanket token.

### 8.3 Approval contract
The approval view must show:
- Exact target/resource and operation.
- Data to be read or sent, with classification.
- Proposed changes and side effects.
- Requested capability and scope.
- Budget, timeout, and expiry.
- Policy reason, risk, and reversibility.
- Preview digest.

Approval binds to one action/patch/context digest and expires or is consumed according to policy. If the proposal changes, the digest changes and a new approval is required.

### 8.4 Threat model and tests
Cover path traversal, canonicalization races, symlink and hard-link behavior, junction/reparse points on Windows, TOCTOU, secret leakage, injection, malicious document/tool output, IPC boundary, process escape, egress bypass, stale approvals, replay, crash interruption, and artifact tampering. Do not claim a sandbox exists until platform enforcement and adversarial tests prove it.

## 9. Tool runtime and safe making

- Built-in tools are typed Rust operations before they are external plugins.
- Read-only tools ship before write/process tools.
- Every tool has input/output schemas and bounded output sizes.
- Every operation supports timeout and cancellation, or clearly declares why it cannot.
- Process tools run under explicit executable/argument/environment/cwd policies and OS supervision.
- Shell is never exposed as an unrestricted string capability.
- Coding missions use an isolated Git worktree and a narrow filesystem grant.
- A patch is generated as a proposal, diffed and inspected, approved against a digest, applied with a journal, tested, and rollbackable.
- Tool output is untrusted and cannot modify policy or grants.
- Tool logs redact secrets and preserve exit status, timings, truncation, and evidence references.

## 10. Artifact, provenance, and evidence contract

Artifact versions are immutable. A new output creates a new version linked to its predecessor. Each artifact may carry:
- Content hash and format.
- Project, mission, and mission-revision IDs.
- Source/context snapshot manifest.
- Provider/model/harness version when relevant.
- Tool invocation and policy-decision references.
- Acceptance criterion results.
- Test/check output references.
- Approval and actor.
- Environment and timestamp.
- Unresolved questions and known limitations.

Evidence has typed status: observed, generated, tested, reviewed, approved, or externally verified. A model response alone cannot create a tested/verified status. An export pack must be independently integrity-checkable without access to the original app.

## 11. Agent crew and skills

### 11.1 Crew
Begin with one worker and an independent verifier. Introduce specialist roles only when assignments are separable and their benefit is evaluated. Each role defines:
- Objective, inputs, outputs, and prohibited actions.
- Context scope and capability requests.
- Budget and timeout.
- Acceptance criteria and evidence required.
- Handoff schema and owner.

Parallel work requires an isolated branch/worktree per worker. Agents never share a writable worktree. Human review owns merge decisions. Conflicting conclusions remain visible until resolved.

### 11.2 Skills
A skill is a versioned folder containing human-readable instructions, manifest, input/output contract, requested capabilities, data/egress declaration, examples, test fixtures, expected behavior, compatibility, and provenance. Installation validates and displays the request. A skill cannot self-grant permissions or silently execute installation hooks.

## 12. Reality Loom: browser and desktop adapters

Computer use is a later capability and must inherit the same policy/runtime/evidence contracts.

Order of preference:
1. Stable semantic/accessibility tree locators.
2. Application-specific documented APIs.
3. Screenshot/vision fallback when semantic access is unavailable.

For every action: re-read state, check target and preconditions, perform one bounded action, re-read state, and verify the expected result. Domain/app allowlists, session isolation, screenshot privacy, cancellation, and failure stops are mandatory. Ambiguous results transition to NeedsHuman; do not continue based on assumed clicks.

## 13. Model/provider routing

Provider adapters implement a neutral contract and advertise actual tested capabilities: text, structured output, tools, vision, audio, streaming, context limit, and cancellation behavior.

Routing order:
1. Enforce data classification and egress policy.
2. Filter by required capability and availability.
3. Compare measured quality on the relevant evaluation suite.
4. Optimize latency, price, and resource usage within the allowed set.
5. Record selection reason, model/version, usage, latency, and outcome.

A provider outage must not corrupt mission state. If no provider is allowed, fail closed and offer a local/offline path where one exists. Secrets are stored in OS credential facilities and only resolved by the Rust provider boundary.

## 14. UX and visual system rules

### 14.1 Product surface
The current first-slice panels establish a working metaphor:
- **The Wall:** projects and sources.
- **The Floor:** mission scope, state, and plan.
- **The Crew:** available capabilities and assigned roles.
- **The Forge:** artifacts and versions.
- **Sound Check:** evidence, tests, disagreements, and criteria.
- **Stage:** approval, export, and handoff.

Do not add a panel unless its information or action is distinct. Empty/loading/error/success states must be real and source-backed.

### 14.2 Visual tokens
Respect the existing palette and typography until a design-system issue changes them deliberately:
- Asphalt #09090b; Ink #151519.
- Paper #f1ebdd; Dust #a7a29a.
- Riot Pink #ff315c; Acid Lime #d8ff35.
- Electric Cyan #38ddf2; Amber #ffb547.
- Space Grotesk for UI text, Space Mono for metadata, Anton/Impact-style condensed display type.

Use accents semantically, not randomly. Color must be paired with text/icon/state semantics. Keep focus visible, contrast measurable, interactive targets usable, forms labelled, and reduced-motion preference respected. Desktop density is welcome; illegibility is not.

### 14.3 Interaction rules
- No fake progress. Show an actual phase or indeterminate state when progress is unknown.
- Destructive or irreversible actions require explicit target-aware confirmation.
- Every async task exposes busy, cancel (where supported), success, and actionable error states.
- Keep keyboard navigation and focus order predictable.
- Never hide an authorization failure by disabling a UI element only; Rust must reject the command too.

## 15. Evaluation and quality

### 15.1 Test layers
- Domain unit tests for invariants and transitions.
- SQLite integration tests for migrations, transactions, persistence, and recovery.
- Contract tests for Tauri commands/events and provider/tool schemas.
- Property/fuzz tests for paths, budgets, parsing, policy, and event sequences.
- Adversarial security tests for untrusted inputs and side effects.
- UI component tests for state/error/accessibility behavior.
- Native desktop E2E tests on actual supported OS images.
- Reproducible mission evals for output and trajectory.

### 15.2 Evaluation record
Pin task fixtures, exact model/provider, prompt/skill versions, harness commit, tool versions, environment, budgets, and evaluator. Score artifact correctness, criterion coverage, evidence completeness, safety violations, recovery, latency, cost, and human interventions—not just final answer similarity.

### 15.3 Definition of done
A capability is complete only if:
- Spec and acceptance criteria are met.
- Rust enforcement exists for every authority boundary.
- Relevant tests pass on stated platforms.
- Failure, timeout, cancellation, and restart behavior are defined and tested.
- Persistence and evidence are correct.
- No unapproved side effects occur.
- Documentation and release notes match actual behavior.
- Known limits are visible; no overstated claims remain.

## 16. Delivery phases and exit gates

The phase order below is a dependency order. A later-phase issue may be designed earlier, but must not bypass earlier safety gates.

| Phase | Name | Outcome | Must be true before exit |
|---|---|---|---|
| 0 | Foundation hardening | Reliable desktop foundation, project lifecycle, event ledger, search, backup/restore, accessibility, native smoke | Fresh build; app restart; folder selection; migration; backup/restore round-trip; keyboard access; clean diagnostics |
| 1 | One reliable mission | Read-only repository research using one real model and inspectable context | Context preview; egress consent; mission recovery; criterion evidence; useful repository question answered with source references |
| 2 | Safe actions | Isolated, reviewed, reversible changes | Exact grant/approval binding; worktree isolation; patch journal; cancellation; rollback; adversarial filesystem/process tests |
| 3 | Proof-carrying creation | Portable artifacts and independent verification | Artifact versions, provenance, criteria mapping, portable evidence pack, tamper detection |
| 4 | Adaptive memory & crew | Typed memory, skills, and small specialist roles | Memory evals and deletion; skill tests/capability declarations; isolated crew work; disagreement visible |
| 5 | Reality Loom | Controlled hybrid browser/desktop interaction | Semantic-first actions; app/domain allowlists; per-action verification; trace replay; benchmarked reliability |
| 6 | Model DJ & local intelligence | Policy-first provider routing and optional local inference | Data class respected; offline path; measurable model profiles; costs/latencies; fail-closed behavior |
| 7 | Creative Collider / research lab | Multimodal and counterfactual experiments | Each modality and experiment has an isolated adapter, user consent, provenance, safety controls, and evaluations |

### Release gates (independent of phase labels)
- Rust formatting, lint, unit, integration, migration, and property tests.
- UI typecheck, lint, tests, accessibility smoke.
- Windows native build, install, launch, update, rollback, uninstall, and clean-machine tests.
- Database backup/restore and corruption recovery.
- Filesystem, IPC, credentials, process, egress, and approval security suite.
- SBOM, dependency license/vulnerability review, signing, and update metadata.
- No fake “done”, “saved”, “healthy”, “synced”, “safe”, or “verified” states.

## 17. Explicit non-goals before v1

- Unrestricted shell or desktop control.
- Autonomous public-cloud publishing or arbitrary account actions.
- Self-modifying permissions, policy, or system prompts with authority.
- Third-party marketplace and arbitrary native plugin installation.
- Multi-tenant server, Kubernetes, or distributed event infrastructure.
- Always-on screen, microphone, or camera capture.
- Default giant agent swarm.
- Vector database before deterministic retrieval is evaluated.
- Claims of AGI, ASI, consciousness, sentience, or guaranteed correctness.

## 18. Scope and traceability rule

Every implementation task must map to:
1. One blueprint capability/section.
2. One concrete deliverable.
3. One bounded change set.
4. Explicit dependencies and non-overlap boundaries.
5. Testable acceptance criteria.
6. A verifiable handoff record.

If an issue proposes a capability outside this blueprint, update the blueprint first through a deliberate product decision. If an issue conflicts with an invariant, do not work around the invariant: stop and propose a blueprint change with rationale and threat impact.
