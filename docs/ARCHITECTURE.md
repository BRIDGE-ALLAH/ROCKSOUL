# ROCKSOUL — ARCHITECTURE

> **Normative authority:** this file is an explanatory architecture overview. The versioned [BLUEPRINT.md](BLUEPRINT.md) is the authoritative architecture, domain, security, data, and release contract. The [PRODUCT_NARRATIVE.md](PRODUCT_NARRATIVE.md) defines product intent; [ISSUE_MAP.md](ISSUE_MAP.md) indexes the executable work.

## The big shape

Rocksoul is a **local-first desktop application with a Rust-owned execution core**. Not a microservice estate. Not a web app with an unrestricted shell bolted on.

```text
                    ROCKSOUL DESKTOP
 ┌────────────────────────────────────────────────────────────┐
 │ React + TypeScript UI                                      │
 │ THE WALL | THE FLOOR | THE CREW | AMP METER                │
 └──────────────────────────┬─────────────────────────────────┘
                            │ typed Tauri commands/events
 ┌──────────────────────────▼─────────────────────────────────┐
 │                    RUST APP CORE                           │
 │ Project Domain · Mission State · Event Ledger               │
 │ Context Compiler · Harness Runtime · Model Router           │
 │ Policy Engine · Capability Broker · Approval Queue          │
 │ Tool Runner · Evidence Collector · Artifact Manager          │
 └───────┬──────────────────┬───────────────────┬──────────────┘
         │                  │                   │
         ▼                  ▼                   ▼
    SQLite + FTS5      Provider adapters     Tool adapters
    event log          cloud / local         native / MCP /
    artifact index     model endpoints       skills / browser
         │                  │                   │
         └──────────────────┴───────────────────┘
                            │
                 isolated execution boundary
              OS process sandbox / Wasmtime later
```

Only add a separate worker process when isolation, model runtime, or crash containment justifies it. Use private IPC (stdio JSON-RPC, named pipes, or Unix sockets) rather than exposing an unauthenticated localhost server.

## Core modules

### 1. Project Domain

Owns the nouns and invariants:
- Project
- Mission
- Hypothesis
- Experiment
- Artifact
- Source
- AgentRole
- ToolCapability
- Approval
- Evidence
- MemoryItem

Keep this domain independent of any model/provider SDK.

### 2. Mission Harness

A mission is a durable state machine, not a prompt string.

Possible states:
`Draft → Ready → Exploring → AwaitingApproval → Executing → Verifying → Review → Completed`

Failure paths:
`Blocked`, `Cancelled`, `Failed`, `NeedsHuman`, `RolledBack`.

Persist every transition. Each transition must state who/what caused it, the input references, policy decision, and evidence. A restart resumes from a known checkpoint rather than pretending the last run never happened.

### 3. Chaos-to-Order Engine

Separate exploration from execution.

**Explore phase**
- Generate competing hypotheses and routes.
- Ask different specialist roles to critique independently.
- Record assumptions and unresolved questions.
- Run cheap, reversible experiments first.

**Converge phase**
- Compare candidates against constraints and evidence.
- Produce one proposed plan with risks and acceptance criteria.
- Ask for approval before consequential operations.

**Execute phase**
- Run only granted capabilities.
- Check budgets, timeouts, cancellation, and resource limits.
- Capture output and side effects.

**Verify phase**
- Evaluate the deliverable against deterministic criteria.
- Ask an independent reviewer to challenge it.
- Mark the mission complete only when required evidence exists.

Chaos lives in hypothesis generation. Execution remains deterministic where possible.

### 4. Capability Broker

This is the security heart.

A tool is not just a function name; it is a capability manifest:
- allowed operation,
- resource scope,
- read/write level,
- data classification and egress,
- network access,
- execution location,
- budget and timeout,
- approval mode,
- audit/evidence requirements.

Example: a repository-inspection tool may read tracked, non-secret text files under one workspace, respect ignore rules, and return bounded excerpts. It cannot write, access sibling directories, or upload source contents to a cloud provider by default.

### 5. Memory Engine

Use different memory types rather than one giant "memory" bucket:

| Type | Stores | Example |
|---|---|---|
| Working | Current mission state | Current plan and open questions |
| Episodic | Immutable events and experiences | Build failed with a specific error |
| Semantic | Reviewed project facts | This service owns the billing API |
| Procedural | Reusable methods and skills | How this repo runs integration tests |
| Preference | User-controlled interaction preferences | Prefer concise diffs before applying |

A memory item has source, timestamp, confidence/provenance, scope, policy, and correction history. Model-generated interpretations are not silently promoted to facts.

### 6. Context Compiler

Context is a budgeted, inspectable product function:
- Start with mission contract and relevant project rules.
- Retrieve exact files, symbols, sources, and prior decisions.
- Prefer deterministic ranking first: path, symbol, FTS, recency, graph links.
- Optionally add vector retrieval after evals justify it.
- Deduplicate and cap context.
- Expose a "what went into context?" view to the user.

Do not dump a repository or full transcript into every model call.

### 7. Model Router

Choose a provider by policy first, then measured utility:
- data/egress permission,
- required modality and tool capability,
- quality on the relevant eval suite,
- latency,
- price,
- availability.

The router may select local or cloud inference. It may not override a privacy or permission boundary to find a cheaper or more capable model.

### 8. Tool Runtime

Tool surfaces:
- Native Rust tools for core app functions.
- MCP for external interoperability.
- Skills for reusable instructions/workflows.
- Browser and desktop adapters for computer-use.
- CLI and process adapters for approved commands.
- Wasmtime/WASI for future portable extension sandboxing.

For browser and desktop operations, prefer accessibility trees and semantic element references; fall back to screenshots and vision when needed. Re-read state after each meaningful action.

### 9. Evidence and Artifact Engine

Every deliverable has an evidence envelope:
- artifact hash and version,
- mission/acceptance criteria,
- source list,
- model/provider/harness version,
- tool actions and timestamps,
- approvals and policy decisions,
- test/check output,
- unresolved uncertainty,
- final human disposition.

Evidence can prove that a test passed; it does not automatically prove the underlying claim is true.

## Local data model

Start with SQLite:
- `projects`
- `missions`
- `events` (append-only for meaningful changes)
- `artifacts`
- `sources`
- `memory_items`
- `memory_links`
- `tool_capabilities`
- `approval_requests`
- `provider_profiles`
- `skills`
- `eval_runs`

Use migrations and backups from the first release. Derived summaries and search indexes can be rebuilt. The event ledger and original artifact bytes are the durable record.

## Parallel work without agent spaghetti

When multiple agents can work independently:
- Create a dedicated Git worktree per mission/agent.
- Give each a narrow acceptance contract and tool grants.
- Never share a writable worktree among competing agents.
- Require tests and evidence before a merge request enters the review lane.
- Resolve conflicts with a human-owned merge decision.

This turns parallelism into isolation + review, not a swarm sharing one keyboard.

## Integration boundaries

Rocksoul is independent from the MOONWITNESS monorepo and the sibling products.

- **ROCKSOUL-EX:** may provide concepts and optional future adapter for repository Witness, mission memory, Forge patch review, and Proof.
- **ROCKSOUL-WX:** may provide a provider/policy gateway via documented workload contracts; no hard runtime dependency at first.
- **MOONWITNESS:** potential backend/service ecosystem. Consume stable, authenticated APIs only.
- **Third-party models/tools:** adapters behind versioned contracts, not imports spread through core domain code.

Do not use Git submodules or copy whole sibling repos into this repository. If code sharing becomes real, extract a small versioned Rust crate or API contract and own its lifecycle.

## Trust invariants

1. UI is never the enforcement boundary.
2. Model output is untrusted input.
3. Tools are denied by default.
4. Approval is scoped to one exact operation/target/version.
5. Every write is journaled and recoverable.
6. Secrets never enter prompt logs or ordinary project memory.
7. Cloud egress requires explicit data classification and user policy.
8. Tool execution is cancellable and bounded.
9. Completion requires criteria-specific evidence.
10. The system must tell the truth when it cannot verify something.