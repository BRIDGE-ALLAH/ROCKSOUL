# ROCKSOUL — PRODUCT NARRATIVE

**Canonical narrative · Version 1.0 · 11 November 2026**

> **WORK HARD. PLAY HARD.**
>
> **THINK WITH CHAOS. ACT WITH RIOT.**
>
> **MORE THAN AGI. ONE STEP TOWARD ASI.**

The tagline is an ambition and a creative posture, never a claim that Rocksoul has achieved AGI, surpassed AGI, reached ASI, or demonstrated consciousness.

## 1. The one-line story

**Rocksoul is a local-first creative-intelligence studio that turns messy ideas and real project work into explicit missions, inspectable artifacts, and evidence-backed outcomes—with the human keeping authority over consequential actions.**

It is not a chatbot with a punk jacket. It is a place to think, research, design, build, test, and hand off work without losing context or pretending that a model's confident sentence is proof.

## 2. The problem

Useful work rarely starts as a clean prompt. It starts as fragments: a repo, a sketch, a voice note, a half-written brief, a broken build, competing hypotheses, old decisions, and constraints scattered across files and tools.

Chat-only workflows make people repeat context, blur facts with guesses, lose the history of decisions, and struggle to verify what was actually changed. Unbounded agents create a different problem: they can act faster than the user can inspect their authority or recover from mistakes.

Rocksoul exists to close that gap:

- from loose intent to an explicit mission;
- from scattered information to selected, inspectable context;
- from one answer to competing routes and testable experiments;
- from model narration to concrete artifacts and receipts;
- from opaque automation to scoped authority, visible approvals, and recovery.

## 3. Who it is for

Rocksoul is for people who move between disciplines and want to make things:

- **Builders** working across code, product, design, and research.
- **Developers** who need repository-aware assistance without risking their primary workspace.
- **Creators** combining text, image, audio, video, and prototypes.
- **Researchers** tracking sources, assumptions, experiments, and decisions.
- **Founders and operators** who want ideas tested against reality rather than polished into a slide.
- **Small teams** that need specialist collaboration with clear ownership and review.

The first product target is an individual desktop workflow. Multi-tenant team SaaS, chatbot companionship, CRM replacement, and unrestricted digital-employee automation are not the initial market or architecture target.

## 4. The AGILITY mindset

AGILITY describes how Rocksoul should work now; it is not a benchmark claim.

- **Adaptive** — revise the route when evidence or requirements change.
- **Generative** — produce alternatives and connect ideas across disciplines.
- **Intelligent** — reason over sources, constraints, trade-offs, and uncertainty.
- **Grounded** — make important claims traceable to sources, tool results, tests, or human review.
- **Integrated** — coordinate models, tools, files, applications, and people through explicit contracts.
- **Tactical** — choose when to explore, focus, ask, act, or stop.
- **Human-led** — keep goals, grants, approvals, and consequential decisions under human control.

The product should become more adaptive through measurable reliability—not by turning a slogan into a capability badge.

## 5. The creative loop

The product experience follows six beats. These are user-facing concepts; they map to concrete domain records and runtime gates in the blueprint.

1. **SPARK — capture the spark.** Bring an idea, question, repo, link, sketch, note, or problem into a project.
2. **RIFF — shape the mission.** Define the objective, included and excluded scope, constraints, sources, data class, budgets, and acceptance criteria.
3. **THE PIT — assemble the crew.** Select the provider, specialist roles, skills, and tools appropriate to the mission. Every role and tool has a bounded contract; no role grants itself authority.
4. **FORGE — make something real.** Research, prototype, write, design, or produce a patch/artifact through typed, policy-checked operations.
5. **SOUND CHECK — challenge the result.** Compare against criteria, inspect sources and traces, run allowed checks, preserve disagreement, and expose unresolved uncertainty.
6. **STAGE — decide what happens next.** Approve, export, publish, hand off, revise, or stop. Consequential external actions require their own exact authorization.

Exploration may be divergent. Execution is scoped. Completion is evidence-gated.

## 6. What makes Rocksoul different

### Chaos-to-Order

Rocksoul should not force every problem into one linear prompt and one immediate plan. It should preserve hypotheses, alternatives, assumptions, and experiments, then help the user converge on a route supported by evidence.

### A crew that can disagree

Builder, Skeptic, Researcher, Operator, Editor, and Guardian are example roles—not a mandatory swarm. Each role needs a clear task, context boundary, capability request, budget, output schema, and independent acceptance criteria. Disagreement stays visible until a human or a defined verifier resolves it.

### Memory with provenance

Project memory is not an unfiltered chat dump. It distinguishes working state, events, reviewed facts, interpretations, procedures, and user-controlled preferences. Every reusable item has provenance, scope, age, and correction/deletion semantics.

### Proof-carrying creation

Important outputs travel with their history: mission revision, sources, artifact digest, tool/provider versions, approvals, tests, evidence, and known limitations. A receipt proves what was observed or executed; it does not magically prove every claim is true.

### A controlled workbench across surfaces

A single mission may eventually use files, repository tools, browser, desktop accessibility APIs, command profiles, and media providers. These are separate capability surfaces connected by typed contracts and one auditable policy boundary—not an unrestricted shell hidden behind an agent.

## 7. Product promise and product honesty

The promise is not "the AI will do everything." The promise is:

- Rocksoul remembers the work it has actually persisted.
- It shows which sources informed a result.
- It distinguishes model output from observations and test results.
- It explains what a proposed action will touch.
- It asks before actions that require human authority.
- It can stop, fail, or request help without disguising that state as success.
- It preserves enough evidence to review and recover.

No fake health, fake telemetry, fake progress, fake completion, or "AGI online" screens.

## 8. The first useful product

The first useful release is deliberately narrower than the long-range vision. Its path is:

1. create a local project and durable mission;
2. inspect explicitly selected sources under read-only bounds;
3. compile a deterministic, inspectable context package;
4. use one reviewed provider only after egress policy and user consent permit it;
5. propose a plan with risks and acceptance criteria;
6. use only registered read-only tools;
7. verify criteria against actual evidence;
8. store a versioned artifact, evidence, and event history;
9. restart and recover the work without losing its record.

The current `main` branch is earlier than this target: it contains the first local-mission slice (project, mission, metadata-only Workspace Scout, local artifact, evidence receipt, and human approval). It does **not** yet contain source-content extraction, a provider adapter, a general tool registry, or autonomous agent execution. The Windows native E2E work is tracked separately and is not complete until its PR-triggered CI run passes. This status statement must be updated only from merged code and observed test results.

## 9. Brand and experience

Rocksoul should feel like a sharp studio partner: curious, brave, irreverent, and technically responsible.

Visual energy may be grunge, Gen-Z, funky, and high-contrast. The work surface itself must remain legible, accessible, calm under pressure, and predictable. Decoration never outranks evidence, data density, or clear action states.

Voice examples:

- "Got a spark? Let's make a mess worth keeping."
- "Three routes. One looks promising. Let's test it."
- "That take broke. Good catch. Let's fix it."
- "Good rhythm. Missing receipts."
- "Big horizon. Small honest steps."

## 10. Non-negotiable boundaries

- The Rust core and OS enforce authority; the UI only presents it.
- Tools, file-content reads, network egress, writes, process execution, browser/desktop control, and publishing are denied by default.
- A capability grant is distinct from a requested capability and from a one-time approval.
- Human approval binds to the exact operation, target, inputs, and version digest.
- Untrusted files, pages, imported skills, provider output, and model output cannot modify policy or grant authority.
- Consequential operations are bounded, cancellable, journaled, and recoverable.
- Secrets never enter ordinary logs, prompts, artifacts, or project memory.
- User data remains locally controlled, inspectable, exportable, and deletable under explicit semantics.
- Unsupported platform enforcement is unavailable—not silently downgraded.
- Capability claims are earned through repeatable evaluation and honest disclosure.

## 11. Long horizon

Rocksoul may explore multimodal creative workflows, richer agent crews, local inference, browser/desktop interaction, simulations, and long-horizon adaptability. AGI is a research horizon; ASI is a distant and uncertain question. There is no promised ladder, date, or guaranteed trajectory from today's prototype to either.

**The ambition stays huge. Every shipped claim stays small enough to prove.**

---

## Authority

This narrative defines the product identity, audience, experience, and promise. The normative engineering and security rules live in [BLUEPRINT.md](BLUEPRINT.md). Execution rules for local AI coding agents live in [AI_TASK_PROTOCOL.md](AI_TASK_PROTOCOL.md). Tracked work is indexed in [ISSUE_MAP.md](ISSUE_MAP.md).
