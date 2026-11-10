# ROCKSOUL

**MORE THAN AGI. ONE STEP TOWARD ASI.**

**WORK HARD. PLAY HARD.**  
**THINK WITH CHAOS. ACT WITH RIOT.**  
**SMART AGILITY. BRUTAL ENERGY. FUNKY STYLE.**

Rocksoul is where wild ideas get their hands dirty.

It's a local-first desktop creative-intelligence studio for people who don't want to stop at asking questions. Bring a half-baked idea, a monster problem, a messy repo, a sketch, a research rabbit hole, or a "what if?". Rocksoul helps turn it into a mission, find the relevant context, make something concrete, and check whether it actually works.

**Less prompt-loop. More let's-build-this.**

"More Than AGI" is the ambition, not a claim that Rocksoul has achieved or surpassed AGI. "One Step Toward ASI" is a distant research horizon, not a promise or timeline. We build what we can prove.

## The Rocksoul mindset

**AGILITY is the move.** Think across disciplines. Switch gears when evidence changes. Try the weird route. Learn from the crash. Keep what works. Turn the next idea into action.

- **Think with chaos:** explore more than one hypothesis and route.
- **Act with riot:** challenge stale patterns and build the better thing.
- **Work hard:** care about details and finish the job.
- **Play hard:** stay curious and keep experimenting.
- **Stay smart:** move fast when safe; slow down when consequences matter.

AGILITY means Adaptive, Generative, Intelligent, Grounded, Integrated, Tactical, and Human-led. It describes the product's operating philosophy, not an AGI score or capability claim.

## The creative loop

1. **SPARK** — bring the idea, question, sketch, link, note, or problem.
2. **RIFF** — define context, objective, scope, constraints, and success criteria.
3. **THE PIT** — assemble the right human, model, specialist role, skill, and tool under explicit contracts.
4. **FORGE** — research, design, write, code, or create a concrete artifact.
5. **SOUND CHECK** — inspect sources, test criteria, challenge assumptions, and expose uncertainty.
6. **STAGE** — approve, export, publish, hand off, revise, or stop.

Go wild while exploring. Get precise before shipping. Every important operation should leave evidence that can be inspected.

## What makes Rocksoul different

- **Chaos-to-Order:** preserve competing hypotheses and routes, then converge using evidence.
- **A crew that can disagree:** roles have bounded tasks and budgets; disagreement is visible rather than hidden by fake consensus.
- **Memory with provenance:** events, facts, interpretations, procedures, and preferences remain distinct and correctable.
- **Proof-carrying creation:** artifacts carry source references, versions, tests, approvals, and limitations.
- **A controlled workbench:** local files, providers, browser, desktop, CLI, and media use typed capability boundaries—not an unrestricted agent shell.
- **Human-led authority:** user goals, grants, and consequential decisions remain under human control.

## First product wedge

The first useful product is one reliable local-first mission:
- keep projects and mission state on the device;
- select sources explicitly;
- compile a bounded, inspectable context;
- use a reviewed provider only when policy and consent allow it;
- propose a typed plan and use registered read-only tools;
- verify acceptance criteria with evidence;
- persist artifacts and history across restart.

The current `main` branch is earlier than this target. It contains the first local-mission slice: native folder selection wiring, local projects/missions, a bounded metadata-only Workspace Scout, a SHA-256 artifact, evidence receipt, and explicit human approval. It does not yet include source-content context compilation, a provider adapter, a general tool registry, or autonomous agent execution.

## Canonical product contract

These documents now define the single source of truth:

- [Product narrative](docs/PRODUCT_NARRATIVE.md) — product promise, audience, identity, experience, and honest capability claims.
- [Normative blueprint](docs/BLUEPRINT.md) — architecture, domain model, lifecycle, UX rules, security boundaries, data handling, test gates, and phase exit criteria.
- [AI task protocol](docs/AI_TASK_PROTOCOL.md) — how small local coding agents execute one issue safely and autonomously within its scope.
- [Issue map and dependency index](docs/ISSUE_MAP.md) — all task IDs, GitHub links, acceptance-check counts, and major sequencing gates.
- [Program tracker](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/4) — phase-level checklist for the full delivery program.

The blueprint is normative. Child issues narrow implementation scope but may not weaken its security or product invariants.

## Delivery tracker

The program is decomposed into **100 small task issues** across P0–P7 and OPS. Each task has a bounded outcome, dependencies, implementation specification, design/engineering rules, explicit exclusions, checkbox acceptance criteria, verification, and AI-agent handoff.

Start at [Issue #4](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/4), then choose the smallest open issue whose dependencies are already merged. Do not assign a small local agent a whole phase.

## Current implementation status

The first local-mission slice was merged into `main` via [PR #2](https://github.com/BRIDGE-ALLAH/ROCKSOUL/pull/2). [CI run #42832504466](https://github.com/BRIDGE-ALLAH/ROCKSOUL/actions/runs/42832504466) passed Rust formatting, five core tests, Clippy, UI typecheck/production build, and Windows compile checking.

The Windows native E2E lifecycle is tracked by [draft PR #3](https://github.com/BRIDGE-ALLAH/ROCKSOUL/pull/3) and [P0-F01 #5](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/5). The latest observed PR run, [#42851122894](https://github.com/BRIDGE-ALLAH/ROCKSOUL/actions/runs/42851122894), passed Rust core and UI jobs and launched the native app/WebDriver session, but the Windows E2E scenario **failed** in the Tauri service `patchBrowserExecute` hook (`TypeError: Cannot read properties of undefined (reading 'bind')`). This does not prove a vault/SQLite failure and is not accepted as an E2E pass. The real Windows shell folder-picker select/cancel check is a separate manual gate in [P0-F02 #6](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/6); its procedure is documented in [WINDOWS_SMOKE_TEST.md](docs/WINDOWS_SMOKE_TEST.md).

This is prototype code, not a release-ready installer. Backup/restore, installer signing, updates, clean-machine installation, recovery, and release security gates remain open.

## Technical direction

Rust owns persistence, filesystem boundaries, policy, credentials, provider transport, processes, tools, and evidence. React/TypeScript owns the desktop presentation and interaction layer. The exact dependencies and rationale live in [TECH_STACK.md](docs/TECH_STACK.md); the normative boundary lives in [BLUEPRINT.md](docs/BLUEPRINT.md).

## Existing engineering references

- [Architecture overview](docs/ARCHITECTURE.md)
- [Tech stack](docs/TECH_STACK.md)
- [Repository/ecosystem audit](docs/AUDIT.md)
- [Idea Lab](docs/IDEA_LAB.md)
- [Research Radar](docs/RESEARCH_RADAR.md)
- [First local mission slice](docs/FIRST_SLICE.md)
- [Roadmap](docs/ROADMAP.md)
- [Security posture](SECURITY.md)
- [Repository agent rules](AGENTS.md)

## License

No open-source license has been selected yet. Until the project owner publishes one, default copyright restrictions apply; do not assume permission to reuse or redistribute this code.

---

**ROCKSOUL. THINK WITH CHAOS. ACT WITH RIOT.**
