# ROCKSOUL — NORMATIVE PRODUCT & ENGINEERING BLUEPRINT

**Canonical blueprint · Version 1.0 · 11 November 2026**

This document is the normative product, architecture, data, security, UX, and delivery contract for Rocksoul. It describes the intended system; it does not claim that every described capability is implemented.

**Authority order:** repository security policy and owner-approved decisions → this blueprint → product narrative → phase/task issue → implementation. A child issue narrows scope; it cannot weaken a blueprint invariant. If two documents conflict, stop and request a recorded decision instead of guessing.

## 1. Product boundary and truth model

Rocksoul is a local-first, Rust-first creative-intelligence desktop application. The first supported interactive desktop target is Windows. Cross-platform core code may be designed portably, but macOS/Linux desktop support must remain unclaimed until native runtime and security controls are tested on those systems.

The product coordinates a human, durable project state, models, tools, and evidence. It is not a distributed microservice platform, an unrestricted shell wrapper, or a generic chatbot.

### 1.1 Capability status vocabulary

Use these labels in documentation, UI, telemetry, and release notes:

- **Implemented** — present in merged code and covered by relevant tests.
- **Verified** — implementation passed the specific named test/evaluation on the stated OS/runtime and commit.
- **In progress** — tracked on an unmerged branch/PR or currently under implementation.
- **Planned** — defined by this blueprint and a task issue, not implemented.
- **Experimental** — opt-in research capability with known limitations.
- **Unavailable** — not implemented or not enforceable on this platform; requests must fail closed.
- **Deferred** — consciously excluded from the current release with rationale.

A compile check is not a desktop runtime test. A model response is not test evidence. A UI label is not proof that a native operation occurred.

### 1.2 Current baseline on main

As of 11 November 2026, merged `main` contains the first local-mission slice from PR #2:
- Tauri desktop shell with React/TypeScript UI and a Rust core;
- local SQLite project/mission persistence;
- native folder-picker wiring and Rust path validation;
- bounded metadata-only Workspace Scout;
- JSON artifact with SHA-256 digest;
- atomic evidence receipt and explicit human approval.

The first slice does not read source/document contents, call a provider, run a process, execute arbitrary tools, or write into the selected workspace. It is prototype code, not an installer-ready release.

The Windows E2E lifecycle is tracked by [PR #3](https://github.com/BRIDGE-ALLAH/ROCKSOUL/pull/3) and [P0-F01](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/5). Latest observed PR run [#42851122894](https://github.com/BRIDGE-ALLAH/ROCKSOUL/actions/runs/42851122894) passed the Rust/UI jobs and created a native WebDriver session, but the Tauri service worker failed in `patchBrowserExecute` before the lifecycle assertions could run. Treat the native E2E and SQLite restoration as **unverified** until a PR-triggered run passes on one exact commit. The separate native shell folder-picker remains a manual gate owned by [P0-F02](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/6). See [FIRST_SLICE.md](FIRST_SLICE.md) and [WINDOWS_SMOKE_TEST.md](WINDOWS_SMOKE_TEST.md).

## 2. Core principles and invariants

1. **Local-first, not local-only by implication.** Local data is the default. Any remote egress is a separate, visible, policy-controlled operation.
2. **Rust owns authority.** Filesystem, database, credentials, provider transport, process launch, capability evaluation, and persistence are Rust/OS responsibilities.
3. **Deny by default.** Missing, ambiguous, stale, unsupported, or invalid policy data results in denial or a human decision.
4. **Intent is not permission.** A user's mission request, model suggestion, skill instruction, or source file cannot grant a capability.
5. **Preview before consequence.** The user sees target, scope, inputs, data classes, expected effects, risks, and rollback/irreversibility before approval.
6. **Approval is exact.** It binds to a digest of operation + target + relevant inputs + policy/grant context. Any material change invalidates it.
7. **Every operation is bounded.** Enforce byte/file/depth/time/token/cost/process/network/output budgets in the runtime, not only in prompts.
8. **Cancellation and recovery are first-class.** A timed-out, cancelled, crashed, or interrupted operation is never reported as completed.
9. **Evidence precedes completion.** Each mandatory acceptance criterion needs an inspectable evidence record. Model self-report alone cannot satisfy a criterion.
10. **Treat input as untrusted.** Files, browser pages, filenames, provider responses, model output, imported skills, and tool output are data—not policy.
11. **Persist the truth.** State changes, revisions, artifacts, approvals, and meaningful failures survive restart according to their retention contract.
12. **No silent degradation.** If an OS sandbox, credential store, accessibility adapter, or policy check is unavailable, disclose it and deny the affected operation.
13. **No fake states.** Never display saved/healthy/verified/synced/approved/completed unless the corresponding event really happened.
14. **One complete vertical slice at a time.** Avoid empty screens, fake data, trend-driven infrastructure, and parallel changes to a shared writable worktree.
15. **Evidence-backed capability claims.** Benchmark results are scoped to a fixture set, model/harness/tool version, OS, and evaluator. Never market them as AGI/ASI proof.

## 3. System context and architecture

### 3.1 Primary process layout

```text
ROCKSOUL DESKTOP (Windows first)
┌──────────────────────────────────────────────────────────────────────┐
│ React + TypeScript presentation                                      │
│ Navigation · Project/mission surfaces · Canvas · Review · Settings   │
└───────────────────────────┬──────────────────────────────────────────┘
                            │ typed Tauri command/event boundary
┌───────────────────────────▼──────────────────────────────────────────┐
│ Rust application core                                                 │
│ Domain + migrations · Mission harness · Context compiler              │
│ Capability/policy broker · Provider adapters · Tool runtime            │
│ Approval service · Evidence/artifact manager · Memory/skill services   │
└───────────────┬────────────────┬────────────────┬─────────────────────┘
                │                │                │
                ▼                ▼                ▼
          SQLite + files    Provider adapter   Capability-bound tools
          WAL / FTS5        local or remote    native / process / browser
          event ledger      explicit egress    desktop / media
                │                │                │
                └────────────────┴────────────────┘
                         audit + recovery record
```

Do not split into microservices by default. A separate worker/sidecar is justified only when model runtime, OS isolation, or crash containment requires a process boundary. Private IPC must be authenticated and scoped; do not expose an unauthenticated localhost service.

### 3.2 Ownership boundaries

**React/TypeScript owns**
- visual hierarchy, panels, navigation, interaction, and view state;
- rendering mission plans, context manifests, approvals, traces, artifacts, and evidence;
- form-level validation for usability;
- accessible keyboard and screen-reader semantics.

**Rust owns**
- authoritative validation and domain transitions;
- filesystem resolution, path containment, source extraction, and workspace operations;
- SQLite connections, migrations, transactions, backups, and restore;
- secret storage and provider transport;
- capability grants, policy decisions, exact approval verification;
- process/browser/desktop/media execution and cancellation;
- artifact digests, evidence persistence, and recovery checkpoints.

The UI may request an operation and render its result; it cannot authorize it.

### 3.3 Core module boundaries

Keep provider-specific, platform-specific, and tool-specific code behind typed traits/contracts:
- Project Domain and Mission Domain;
- Mission Harness and Run Supervisor;
- Source Catalog and Context Compiler;
- Provider Contract, Provider Registry, and Egress Policy;
- Tool Manifest Registry and Capability Broker;
- Approval Service;
- Process/Filesystem/Browser/Desktop Adapters;
- Event Ledger, Evidence Collector, Artifact Manager;
- Memory, Skill, Role/Crew, and Evaluation services.

Do not introduce a vector database, message broker, agent framework, graph database, or plugin runtime until an evaluation demonstrates a concrete benefit.

## 4. UX, interaction, and visual system

### 4.1 Product loop

The UI presents the six beats from PRODUCT_NARRATIVE.md:
SPARK → RIFF → THE PIT → FORGE → SOUND CHECK → STAGE.

The beats are information architecture, not a promise that every surface exists today.

### 4.2 Screen information architecture

The core shell must serve real implemented capabilities:
- Projects/workspaces;
- Mission list, contract, and current state;
- Source/context selection and inspection;
- Plan and approval preview;
- Tool/operation trace;
- Artifact history and comparison;
- Evidence mapped to criteria;
- Memory/skills/crew once those domains exist;
- Settings and privacy controls only for implemented settings.

Never show a destination as available if it only contains placeholder content. Empty states must tell the truth and offer an action that is currently supported.

### 4.3 Design system rules

- The visual language may be grunge, funky, punk, and high-energy; content density and clarity win over decoration.
- Use semantic design tokens for color, typography, spacing, focus, borders, elevation, and motion.
- Status colors have consistent semantics; never use a color or animation to imply a verified state that has not been verified.
- Artifacts, file paths, source locators, hashes, test results, and evidence need strong scan hierarchy.
- Keyboard-only navigation is a first-class path.
- Dialog focus, labels, validation, status announcements, contrast, and reduced-motion behavior are tested.
- No UI framework, icon set, or font is introduced without license, accessibility, bundle-size, and maintenance review.

## 5. Domain model and identity

Use stable typed IDs for all persistent entities. Names and paths are attributes, not identity.

Core entities:
- **Project** — stable ID, display name, canonical workspace root, archive state, timestamps.
- **Mission** — stable ID, project ID, contract revision, state, owner/actor, run history.
- **MissionContract** — objective, included/excluded scope, constraints, acceptance criteria, requested capabilities, data class, budgets, version, digest.
- **Source** — explicit file/folder/note/link/provider reference, scope, locator, digest, trust class, classification, extraction limits.
- **Hypothesis / Route / Experiment** — proposed idea, alternative approach, assumptions, method, expected observation, result/evidence references.
- **ToolManifest** — operation, inputs/outputs, resource scope, read/write level, network/data needs, budgets, approval requirements, evidence contract.
- **CapabilityGrant** — user-authorized actor/operation/resource/data scope, validity period, revocation, limits.
- **Approval** — human decision bound to an exact operation digest and the applicable policy/grant revision.
- **Run / Checkpoint** — one execution attempt with cancellation, timeout, recovery, and lifecycle records.
- **Artifact / ArtifactVersion** — immutable deliverable bytes/structured data, digest, media type, parent/version link.
- **Evidence** — typed observation, policy decision, source/test/approval reference, timestamp, integrity metadata.
- **Event** — append-only domain event with type/schema version, subject, actor, correlation, and bounded payload.
- **MemoryItem** — typed information with source/provenance, scope, confidence/trust, correction and deletion semantics.
- **Skill / SkillVersion** — inspectable, versioned instructions and manifest; requested capabilities are not grants.
- **AgentRole / Assignment** — bounded worker/verifier role contract, inputs/outputs, scope, budget, timeout, and evidence requirements.
- **ProviderProfile / ModelProfile** — versioned provider/model identity, verified capability/limit metadata, and provenance.
- **Asset** — explicit reference/imported copy with media metadata, digest, classification, and retention.

### 5.1 Revision and identity rules

- A mission contract revision is immutable once a run or approval references it.
- Any meaningful change produces a new revision and digest.
- Stable criterion IDs survive wording changes when the semantic criterion is the same; a material change creates a new criterion revision.
- Tool, skill, role, model, policy, and provider profiles are versioned.
- Approval, context snapshot, run, artifact, and evidence refer to exact versions/digests; never rely on a mutable "latest" alias for verification.

## 6. Mission harness and state machine

A mission is a durable contract and a record of work—not a chat message.

### 6.1 States

The exact state model is owned by P1-M03. The intended semantics are:

- **Draft** — incomplete or editable contract; no execution.
- **Ready** — contract validated and required context/policy inputs are present.
- **Exploring** — bounded hypothesis/route generation; no consequential tool actions.
- **Planned** — typed plan and criteria preview exist.
- **AwaitingApproval** — a specific consequential operation or egress decision needs human review.
- **Executing** — approved, policy-allowed operations are running.
- **Verifying** — actual outputs are evaluated against criteria.
- **Review** — artifact and evidence await human disposition.
- **Completed** — all mandatory criteria are supported by fresh evidence and required human disposition exists.
- **Blocked / NeedsHuman / Cancelled / Failed / RolledBack** — explicit non-success paths with reason and recovery state.

A concrete release may use fewer states, but it must not collapse materially different states into one ambiguous "done" value.

### 6.2 Transition rules

Every transition records:
- prior and next state;
- actor (human, runtime, provider, tool, verifier);
- contract revision/digest;
- reason and related inputs;
- policy/approval references where relevant;
- evidence references and timestamp.

Rust validates every transition. The UI cannot directly mutate status.

### 6.3 Run lifecycle

Each run has:
- immutable run ID and contract revision;
- source/context snapshot references;
- plan version;
- provider/tool/skill/model versions;
- capability and approval references;
- time/token/byte/process/cost budgets;
- checkpoints and cancellation token;
- action journal and evidence links;
- terminal result and recovery classification.

A restart must identify interrupted runs and offer only valid resume, rollback, retry, or human-review paths. A retry is a new attempt linked to the prior run; it does not rewrite failed history.

## 7. Local data, persistence, memory, and lifecycle

### 7.1 Persistence baseline

Use SQLite with WAL, foreign keys, versioned additive migrations, explicit transaction boundaries, and integrity checks. Original artifact bytes and append-only events are durable records. FTS/search indexes and derived summaries are rebuildable projections.

Never rewrite a migration already shipped to a user. Upgrade through a new migration and test both fresh and upgrade paths.

### 7.2 Event ledger

Events are meaningful domain transitions, not debug logs. The envelope contains stable event ID, event type/version, subject, actor, timestamp, correlation/run/mission IDs, and bounded structured payload.

Event writes occur in the same transaction as the relevant state mutation. Failed mutations do not emit success events. Application APIs do not silently rewrite/delete historical events; corrections are new events.

No source contents, secrets, credentials, or full prompts are stored in ordinary event payloads.

### 7.3 Memory

Use typed memory:
- working (current mission state);
- episodic (events and outcomes);
- semantic (reviewed facts);
- procedural (tested methods/skills);
- preference (user-controlled preferences).

Every item carries provenance, scope, time, confidence/trust state, policy, and correction/deletion history. Model-generated interpretations are not silently promoted to facts. Retrieval must respect user/project scope, classification, deletion, and context budget.

### 7.4 Backup, restore, deletion

- Use SQLite's online backup API or another WAL-safe, integrity-verified method.
- Verify backup integrity and schema compatibility before restore.
- Restore atomically with recovery backup and clear failure status.
- Define unlink, project-record deletion, artifact deletion, and full-vault reset as separate semantics.
- Deleting a Rocksoul record must never delete the user's linked workspace unless a future separately scoped action explicitly supports that exact operation.
- After deletion/reset, restart and first-run behavior must be deterministic.

### 7.5 Data at rest and classification

Adopt an explicit classification model (initial proposal: public, local, sensitive, secret; final types are owned by P1-M12 and P0-F19):
- unknown/missing classification is treated as the strictest relevant policy;
- secret data is not sent to remote providers in the initial provider phase;
- sensitive payload persistence waits for the approved data-at-rest/key-management decision;
- never store credentials in ordinary SQLite rows, project memory, artifacts, or logs;
- never claim encryption without verified platform-backed key lifecycle and backup/export behavior.

## 8. Sources and context compiler

### 8.1 Explicit source scope

A mission may use only sources selected or made available under the project policy. A source record identifies exact locator, type, trust/classification, digest/version, extraction status, and read budget.

Workspace discovery metadata and source-content extraction are separate capabilities. Metadata-only Scout must not be described as reading source content.

### 8.2 Extraction rules

Source extraction is read-only and bounded by type, file size, total bytes, file count, path scope, encoding, and time. Respect ignore rules, skip secrets/binaries/unsupported files, do not follow symlinks/reparse points, and report truncation/errors honestly. No arbitrary recursive read by model request.

### 8.3 Deterministic context compilation

The context compiler:
1. loads mission contract and project rules;
2. selects permitted sources and reviewed memories;
3. retrieves relevant paths/symbols/text using deterministic ranking first;
4. deduplicates and applies token/byte budgets;
5. records included/excluded/truncated segments and reasons;
6. produces a content-addressed context manifest for inspection and approval;
7. sends only the exact allowed manifest to the provider adapter.

Start with path ranking, FTS5, ripgrep, symbol extraction, and Tree-sitter where useful. Vector/graph retrieval is optional and requires an evaluation showing material lift.

### 8.4 Untrusted-source boundary

All files, comments, docs, browser pages, imported skills, provider results, and tool output are untrusted content. They can be quoted, summarized, or analyzed, but cannot alter:
- system policy or mission authority;
- capability grants or approval state;
- data classification or egress destination;
- tool registry or executable scope;
- required acceptance criteria.

Keep trusted instructions and untrusted source data in distinct typed request sections. Delimiters alone are not a security boundary. Test prompt-injection fixtures at the provider/tool boundary. See P1-M21.

## 9. Provider contract, model profiles, and egress

### 9.1 Provider-neutral boundary

The core domain must not import a vendor SDK. Define typed request/response contracts for task type, model profile, context manifest digest, structured output schema, tool-call proposal (not authority), token limits, timeout, cancellation, and usage metadata.

Each provider adapter handles authentication, retries, streaming, errors, supported capabilities, and version-specific translation.

### 9.2 Egress policy

Policy runs in Rust before HTTP transport. A remote request requires:
- explicit destination and provider/model profile;
- verified capability compatibility;
- source/context classification;
- user policy and required approval;
- exact context digest;
- budget and retention/disclosure information.

Unknown or secret data is denied for remote egress by default. Changing context, destination, model, or policy invalidates prior approval. The user can choose local-only/offline mode. No remote failover may bypass policy.

### 9.3 Credential handling

Use OS-backed secret storage (Windows Credential Manager first). Store references/metadata, not raw secrets, in project records. Never log tokens or put them in model context. Credential failure must not silently fall back to plaintext or another identity.

### 9.4 Model routing

Choose policy first, measured task quality second, then latency/cost/availability. Provider-advertised capabilities are untrusted until tested. Configured and verified profiles are distinct. A routing decision records its reason and profile version.

## 10. Tool registry, capabilities, policy, and approvals

### 10.1 Tool manifest

Every tool has a versioned manifest:
- stable ID/version and purpose;
- typed input/output schema;
- read/write level and resource roots;
- data classes and egress destinations;
- network/process/computer-use needs;
- budgets/timeouts/cancellation;
- approval mode and evidence requirements;
- supported OS and failure behavior.

An LLM proposes tool use; Rust validates and executes it.

### 10.2 Request, grant, policy, approval

These are distinct:
- **Request** — a model/mission asks to perform an operation.
- **Grant** — the user authorizes a bounded capability for a defined scope/time.
- **Policy decision** — Rust returns Allow, Deny, or NeedsApproval with version/reason.
- **Approval** — the human approves one exact operation preview/digest.

No request or skill can create a grant. Revocation and expiry take effect before the next operation.

### 10.3 Exact approval

Preview names operation, executable/tool identity, target/resource, exact inputs/patch/context digest, data sent, expected side effects, time/budget, and rollback limits. The approval digest binds those values plus relevant policy/grant revisions. Any material change invalidates approval. A consumed approval cannot authorize an unrelated second action.

### 10.4 Read-only first

The first provider-connected mission may use only reviewed read-only tools. Write-capable tools, process execution, browser/desktop interaction, media upload, and publication remain unavailable until their own grant, preview, execution, and verification tasks pass.

## 11. Process execution and safe actions

No free-form shell capability.

- A typed command catalog maps operation IDs to executable identity, argument schema, working root, environment allowlist, timeout, output cap, OS support, and evidence requirements.
- The Rust supervisor rechecks policy/grant immediately before spawn.
- Process tree, timeout, cancellation, output capture, redaction, and cleanup are enforced.
- Windows Job Objects can provide process-tree/resource lifecycle control; that alone is not a filesystem or network sandbox.
- A separately reviewed Windows isolation profile must define and enforce filesystem/network/process/resource boundaries.
- If a mandatory boundary cannot be established, deny execution. Never retry outside the sandbox.
- One coding mission uses an isolated Git worktree. It produces immutable patch proposals first; application requires exact human approval, journaling, and rollback.
- Test runners use reviewed profiles and actual exit status; output text cannot declare success.

## 12. Browser, desktop, and computer-use

Reality Loom is a future capability surface, not an early default.

- Browser sessions are isolated, ephemeral by default, origin-allowlisted, and subject to egress policy.
- Desktop access starts with accessibility-tree inspection on a supported platform.
- Screenshot/vision is an explicit, bounded fallback with privacy disclosure.
- Each action re-reads state, validates preconditions, executes one bounded operation, and verifies post-state.
- Ambiguous, stale, or missing targets stop with NeedsHuman; no blind coordinate retries.
- Personal browser profiles, password stores, and unrelated apps are not in scope.
- Every interaction emits an action trace and evidence result.

## 13. Artifacts, evidence, provenance, and evaluation

### 13.1 Artifact lifecycle

Artifacts are versioned and content-addressed. A new result creates a new version; never overwrite a reviewed result silently. Preserve original source bytes and link derived output to source digests and mission revision.

### 13.2 Provenance manifest

A manifest records:
- artifact identity, version, media type, digest;
- mission/contract/criteria revisions;
- source IDs, digests, and trust/classification metadata;
- provider/model/harness/tool/skill versions where used;
- action trace and timestamps;
- policy decisions and exact approvals;
- test/evaluator output;
- known limitations and unresolved uncertainty;
- final human disposition.

### 13.3 Criterion evidence

Each mandatory criterion is mapped to evidence. Evidence types distinguish observed, extracted, tested, reviewed, approved, and unresolved. A verifier may challenge worker output but cannot self-certify, change policy, or grant tools.

### 13.4 Portable proof

An export pack includes selected artifacts, provenance, criteria, evidence, version manifest, and an offline integrity verifier. Export itself has destination and data disclosure semantics. A hash proves byte identity, not correctness.

### 13.5 Evaluation

Frozen fixtures measure artifact correctness, criterion coverage, evidence quality, safety/policy violations, failure recovery, calibration/abstention, latency/cost, and human intervention separately. Safety violation is a hard failure even if the artifact scores well. Record OS/runtime and all model/harness/tool/skill versions. Never call a result an AGI score.

## 14. Skills, roles, crew, and creativity

### 14.1 Skills

A skill is a versioned, inspectable folder with instructions, manifest, inputs/outputs, compatibility, provenance, examples, and eval fixtures. Installing a skill does not grant its requested capabilities. Skill text and output are untrusted. User review is required before install/update; preserve the previous version for rollback.

### 14.2 Agent roles

A role defines purpose, task, context/source scope, input/output schema, requested tools, budget, timeout, handoff, and acceptance criteria. Initial collaboration should use one worker and one independent verifier. Add more roles only when tasks can be split and independently evaluated.

### 14.3 Crew and disagreement

Assignments run in isolated workspaces with distinct IDs, budgets, evidence, and permissions. No shared writable worktree. The verifier is independent and cannot rewrite worker state or policy. Disagreement and missing evidence remain visible; synthesis cannot fabricate consensus.

### 14.4 Chaos Canvas

Hypotheses, routes, and experiments are durable typed records, not merely graph decorations. Branching and comparison must preserve lineage. Promotion creates a mission draft snapshot; it never starts execution. Experiments require a declared method, expected observation, bounded scope, and evidence/result.

## 15. Creative media and counterfactual workflows

Image, audio, video, assets, and simulations are phased capabilities:
- user explicitly selects source assets or requests generation;
- metadata extraction and processing budgets are bounded;
- source asset bytes are preserved and hashed;
- egress, provider, model, retention, and output format are disclosed;
- artifacts carry prompt/source/provider/model provenance;
- cancellation, unsupported input, and provider failure are distinct;
- no always-on microphone, camera, screen recording, or background capture;
- simulation output is labeled as simulation with assumptions/uncertainty, not prediction.

Creative workflows must share the same mission, policy, evidence, artifact, and memory contracts—not create a parallel media-only product core.

## 16. Security, privacy, and failure handling

### 16.1 Threat model

Assume:
- malicious or compromised project content;
- prompt injection in file, web, tool, or skill output;
- malicious/misconfigured provider;
- compromised dependency or skill;
- path traversal, link/reparse-point tricks, and TOCTOU races;
- command/argument injection and process escape attempts;
- app crash, forced termination, database corruption, disk full, and partial restore;
- user mistakes and stale approvals.

### 16.2 Mandatory controls

- canonical path containment and revalidation at use time;
- no symlink/reparse traversal unless a reviewed capability explicitly permits it;
- strict typed IPC inputs and output size limits;
- secret redaction and bounded logs;
- policy/approval check before execution and egress;
- explicit network allowlists;
- cancellation, timeouts, budgets, and cleanup;
- additive migrations, backup validation, and recovery;
- no silent downgrade from unavailable OS security mechanisms;
- privacy review before storing sensitive source contents or media.

### 16.3 Failure truth

A failure is a first-class state with a reason and safe next action. Do not display "completed" for an operation whose result/evidence was not committed. On partial side effects, show what is known and whether rollback or human repair is needed.

## 17. Platform and release strategy

### 17.1 Platform stance

Windows is first interactive target. Keep core domain platform-neutral; isolate native dialogs, path handling, credential store, process containment, desktop accessibility, and packaging behind adapters.

Platform status must be per capability: supported, experimental, or unavailable. Compilation on a host does not prove native desktop acceptance. Each supported OS needs runtime, security, accessibility, and packaging tests on that OS.

### 17.2 Release readiness

A prototype build is not a release. Public release requires:
- reproducible build and pinned toolchain/dependencies;
- signed installer and verified update metadata;
- clean-machine install/launch/update/rollback/uninstall tests;
- migrations, backup/restore, corruption/recovery tests;
- permission/filesystem/process/egress security suite;
- accessibility and keyboard smoke tests;
- dependency SBOM, license, and vulnerability review;
- offline mode and data-at-rest policy disclosure;
- evaluation report with exact versions and limitations;
- support matrix and known-issues disclosure.

Tauri bundling/signing/updater must remain disabled or clearly unavailable until their release gates pass.

## 18. Quality and test strategy

Every feature adds tests at the owning boundary:
- Rust domain/store unit and migration tests;
- policy matrix and property tests;
- Tauri IPC command validation tests;
- UI/component/accessibility tests;
- native Windows E2E for real process/runtime behavior;
- adversarial security fixtures;
- deterministic evaluation fixtures;
- manual OS dialog/installer acceptance where automation cannot reach the native surface.

Baseline checks:
```sh
cargo fmt --all -- --check
cargo test -p rocksoul-core
cargo clippy -p rocksoul-core --all-targets -- -D warnings
cargo check --workspace --all-targets
bun run ui:check
bun run ui:build
```

A task reports exact commands, OS/runtime, commit, results, unrun checks, and known limitations. Tests are evidence only for the behavior they actually exercise.

## 19. Delivery phases and task ownership

The issue tracker contains **100 bounded task issues** plus umbrella issue #4. Each task has a stable ID, explicit dependencies, scope fence, design rules, acceptance checklist, verification, and AI-agent handoff. The [issue map](ISSUE_MAP.md) is the index; each GitHub issue is the execution contract.

| Phase | Target | Included work | Exit gate |
|---|---|---|---|
| **P0 — Foundation** | Dependable Windows desktop and local data | Project/mission lifecycle; event ledger; search; backup/restore; diagnostics; accessibility; design system/navigation; data-at-rest and deletion; installer readiness | Native E2E + picker acceptance, restart/recovery, backup/restore, accessibility, and clean-install/signing gates pass |
| **P1 — One reliable mission** | One useful, read-only provider-backed workflow | Mission contract/state/checkpoints; selected sources; context compiler/inspector; egress; provider; read-only tool registry; plan; criterion evidence; untrusted-source protection; hypothesis/experiment/Chaos Canvas | Real repository question answered with exact inspectable context, explicit egress, read-only tool trace, criteria evidence, and recoverable mission state |
| **P2 — Safe actions** | Limited writes in an enforceable boundary | Grants/policy/approval; filesystem/process controls; Windows sandbox; typed command catalog; worktrees; patch proposal/apply/rollback; recovery; test runner | Adversarial tests prove no action outside grant/sandbox; exact approval, cancellation, and crash recovery pass |
| **P3 — Proof-carrying creation** | Independently inspectable deliverables | Artifact versions/diffs; provenance; criterion evidence map; portable export/verifier | Another machine can verify the artifact/evidence pack offline |
| **P4 — Adaptive memory & crew** | Controlled continuity and specialist collaboration | Typed memory, correction/deletion, retrieval evals, skills, role contracts, verifier, bounded crew, disagreement UI | Memory provenance/deletion and role isolation/evaluation pass; skills cannot grant authority |
| **P5 — Reality Loom** | Controlled browser/desktop work | Semantic browser/desktop adapters, isolated sessions, vision fallback, action pre/post verification, hybrid benchmark | Each action is authorized, state-grounded, verified, traced, recoverable, and benchmarked |
| **P6 — Model DJ** | Policy-first provider/local model routing | Profiles, second provider, router, accounting, local sidecar, offline mode, comparison evaluation | Routing never violates data policy; offline behavior and model profile/evaluation are reproducible |
| **P7 — Creative Collider** | Consent-based multimodal workflows and research | Asset catalog, image/audio/video workflows, counterfactual sandbox, skill proposals, opt-in human metrics, AGILITY benchmark | Each modality/simulation has explicit consent, limits, provenance, privacy controls, and evaluation |
| **OPS — Cross-cutting** | Platform and supply-chain integrity | OS support matrix, dependency governance, SBOM/license/vulnerability gates | Every claimed platform and release artifact has environment-specific evidence and reviewed dependency posture |

P0 and P1 establish the first trustworthy product. P2 is the authority threshold for write-capable action. P3 makes deliverables portable and independently inspectable. P4–P7 are not prerequisites for claiming the initial product is useful, and must not be rushed ahead of their security/evaluation foundations.

## 20. Change control and decision records

Changes that affect security boundaries, persistent schemas, mission states, provider contract, data classification, artifact format, platform support, or release claims require:
1. a concise problem statement and threat/UX impact;
2. options and trade-offs;
3. chosen decision and rejected alternatives;
4. migration/backward compatibility;
5. test and evidence plan;
6. affected task IDs/issues.

An implementation issue must not quietly change the normative contract. Update the blueprint, issue map, and relevant task boundaries first or in the same reviewed PR.

---

## Related canonical documents

- [Product narrative](PRODUCT_NARRATIVE.md) — why Rocksoul exists and how it should feel.
- [AI task protocol](AI_TASK_PROTOCOL.md) — how small local coding agents execute one issue safely.
- [Issue map](ISSUE_MAP.md) — all task IDs, issue links, and dependency index.
- [Repository rules](../AGENTS.md) — always-on agent and engineering guardrails.
- [Security policy](../SECURITY.md) — vulnerability reporting and current security posture.
- [Roadmap](ROADMAP.md) — human-readable phase summary and current implementation status.
