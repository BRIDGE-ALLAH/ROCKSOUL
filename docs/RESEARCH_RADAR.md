# RESEARCH RADAR — WHAT THE SIGNALS ARE TELLING US

**Snapshot date: 10 November 2026**  
**Scope:** curated scan of GitHub Trending signals (AI agents and Rust), selected fast-rising repositories, and research on agent harnesses, memory, long-horizon work, and computer use.

> Ini bukan klaim sudah membaca seluruh GitHub Trending, semua paper, atau seluruh repositori yang viral. Itu corpus yang berubah terus dan terlalu luas untuk satu audit. Ini adalah radar terkurasi: tren yang paling nyambung dengan produk Rocksoul, lalu diterjemahkan jadi keputusan dan eksperimen.

## 1. GitHub signal — jangan cuma lihat angka bintang

The GitHub trend snapshots collected on 10 Nov 2026 show several patterns:

| Signal | Snapshot | Why Rocksoul should care |
|---|---|---|
| `anthropics/skills` | AI-agent list, +1,038 stars that day | Reusable, file-based skill instructions are becoming a visible distribution surface. |
| `vercel-labs/agent-browser` | Rust + AI-agent list, +916 stars that day | Browser automation is moving toward compact, semantic, agent-facing interfaces. |
| `vercel-labs/agent-desktop` | +12,120 stars for the week in the same collector | Computer-use is becoming a runtime capability, not just a screenshot demo. |
| `max-sixty/worktrunk` | +2,395 stars for the week | Parallel agent work needs isolated Git worktrees and clean handoff/merge flows. |
| `thedotmack/claude-mem` | +5,130 stars for the week | Continuity across sessions is a real pain point and a product surface. |
| `jub0t/Concat` | Rust trend, +493 stars that day | Local-first creative software with scriptable APIs/MCP is attracting attention. |
| `ruvnet/RuView` | Rust trend, +172 stars that day | Interesting signal for non-screen sensing and privacy-aware spatial interfaces; not a direct v0.1 feature. |

Sources:
- [AI agents trend snapshot](https://githubtrending.today/topic/ai-agents/)
- [Rust trend snapshot](https://githubtrending.today/language/rust/)
- [Weekly trend overview](https://topgit.dev/trending/weekly)
- [Anthropic Skills](https://github.com/anthropics/skills)
- [Vercel Agent Browser](https://github.com/vercel-labs/agent-browser)
- [Vercel Agent Desktop](https://github.com/vercel-labs/agent-desktop)
- [Worktrunk](https://github.com/max-sixty/worktrunk)
- [Concat](https://github.com/jub0t/Concat)

**Data caveat:** githubtrending.today states that it is not affiliated with GitHub and copies rankings/stars from github.com/trending at the collection time. Star velocity is attention, not quality, security, maintenance, or product-market fit. Always inspect license, last activity, issue health, code, release discipline, and threat model before adopting a dependency.

## 2. The trend shift

### From model worship to harness engineering

The interesting question is no longer just "which model is smartest?". The model is one component inside a harness: task definition, context selection, persistent state, tools, policy, environment, verification, and intervention.

A July 2026 source-code study examined eleven production coding harnesses. It reports recurring patterns around context, tools, permissions, orchestration, skills, and state. Its abstract also reports that general-purpose agent frameworks and vector-based code retrieval were absent from the studied runtimes, while file-based skills appeared in 9 of 11 systems and MCP in 8 of 11. This is a study of a selected corpus—not a universal rule—but it argues against blindly adding a giant framework or vector database on day one.

### From chat history to durable, typed memory

A long context window is not the same as long-term memory. Projects need continuity across days, restarts, models, and people. Memory should preserve events and decisions, distinguish observation from interpretation, record provenance, and allow correction.

### From isolated tools to hybrid computer work

Useful agents need to move across browser, desktop UI, CLI, code, and files. A screenshot-only approach is brittle. Prefer accessibility/semantic interfaces first, screenshots and vision as fallback, then confirm the result with observable artifacts and logs.

### From "agent swarm" to governed crews

Parallel agents are useful only when each one has a bounded mission, isolated workspace, tool grants, budget, timeout, and acceptance criteria. Otherwise, concurrency just multiplies noise and merge conflicts.

### From output-only evaluation to trajectory evidence

A final answer can look good while the work is wrong. Evaluate the path: actions, intermediate artifacts, commands, logs, test output, source coverage, human interventions, and final deliverable.

## 3. Papers worth following

These papers are selected for direct relevance. Preprints are labelled as such; a preprint is not automatically peer-reviewed evidence.

| Paper | Date / status | What it says | Product implication |
|---|---|---|---|
| [Harness Engineering: Anatomy, Architecture, and Evolution of Coding Agents](https://arxiv.org/abs/2609.00006) | 15 Jul 2026, arXiv preprint | Source-code study of 11 production harnesses and recurring runtime patterns. | Build a thin, inspectable harness; don't start by wrapping a framework zoo. |
| [AI Harness Engineering: A Runtime Substrate for Foundation-Model Software Agents](https://arxiv.org/abs/2605.13357) | 13 May 2026, arXiv preprint | Capability emerges from model + harness + environment; identifies task, context, tools, memory, state, observability, verification, permissions, and intervention responsibilities. | Treat the harness as the product's core engineering asset. |
| [Methodological Harness in Agentic Software Engineering](https://arxiv.org/abs/2609.32014) | 25 Sep 2026, arXiv preprint | Mining study of 5,435 repositories from a 116,211-repository dataset; integrated harness mechanisms are still uncommon. | Make the mechanisms work together, not as disconnected docs and flags. |
| [Toward Evidence-Based Agentic Software Engineering](https://arxiv.org/abs/2610.35348) | 26 Oct 2026, arXiv preprint | Annotates 325 experiments from 146 peer-reviewed papers; many omit exact harness versions or full traces. | Version the harness, publish reproducible eval configs, and store traces. |
| [The Horizon Gap](https://arxiv.org/abs/2608.06663) | 7 Aug 2026, arXiv survey | Surveys 1,547 papers and separates long-horizon tasks, long context, and persistent memory. Outcome-only signals become weak as tasks lengthen. | Persist task state, checkpoint progress, and evaluate intermediate steps. |
| [PROJECTMEM](https://arxiv.org/abs/2606.12329) | 10 Jun 2026, arXiv preprint | Local-first append-only events projected into compact memory; memory can warn before repeating failed actions. | Treat memory as a governance input, not just retrieval. |
| [WeaveBench](https://www.microsoft.com/en-us/research/publication/weavebench-a-long-horizon-real-world-benchmark-for-computer-use-agents-with-hybrid-interfaces/) | Jun 2026, research benchmark | 114 tasks across eight domains combine GUI, CLI, code, and external tools; grades real artifacts and traces. | Our own evals must test mixed-interface work, not toy tool calls. |
| [APEX-EM](https://aclanthology.org/2026.findings-emnlp.1996/) | Findings of EMNLP 2026 | Procedural-episodic experience replay with typed memory and multiple retrieval paths. | Explore structured experience reuse, but keep human review and source attribution. |
| [Rethinking Memory Mechanisms of Foundation Agents in the Second Half](https://arxiv.org/abs/2602.06052) | 14 Jan 2026, arXiv survey | Maps memory substrate, cognitive mechanism, and memory subject for foundation agents. | Design memory types and lifecycle before picking a vector store. |

## 4. What we should actually adopt

### Adopt now

1. **A small Rust harness.** It owns state, tools, permissions, budgets, retries, cancellation, and evidence.
2. **Skills as portable folders.** Human-readable `SKILL.md` plus versioned metadata, tests, allowed tools, and examples.
3. **Event-based project memory.** Append-only facts/events; derived summaries are replaceable projections.
4. **Semantic computer-use adapters.** Accessibility tree and stable element references before pixel coordinates.
5. **Parallel worktree isolation.** One agent, one mission, one branch/worktree; merge through a gated review lane.
6. **Trajectory-based evaluation.** Freeze the model, harness, tools, prompt/skill versions, task set, and environment when comparing results.

### Prototype, don't bet the product on it yet

- Multi-agent "idea collision" and adversarial review.
- On-device model routing and hybrid local/cloud execution.
- Event reconsolidation and memory decay based on explicit policies.
- Wasm-based skill/plugin sandbox.
- Visual world model of a project or workflow.
- Multimodal continuous context: voice, images, video, documents, and screen state.

### Ignore for now

- "AGI achieved" badges.
- Unbounded self-improving agents.
- A 50-agent swarm as the default experience.
- A vector database before we can show retrieval beats FTS/path/symbol search.
- A universal agent framework that dictates every workflow.
- Microservices, Kubernetes, Kafka, and distributed infra for a single-user desktop MVP.

## 5. Research questions Rocksoul can own

1. Does a typed event ledger reduce repeated work better than raw transcript replay?
2. Can a "Skeptic" agent improve factual and engineering quality without making latency/cost explode?
3. Can a desktop agent complete one real mission across GUI + CLI + code with trace-level verification?
4. Does deterministic retrieval beat embeddings for repository-local tasks at small and medium scale?
5. How should a skill declare required capabilities, data egress, and risk before installation?
6. Can we measure user agency: approvals, reversals, edits, interventions, and trust calibration?
7. Can an agent safely suggest improvements to its own skills without being allowed to rewrite policy or grant itself permissions?

## 6. Living radar rules

- Revisit GitHub trends monthly; a daily spike is not a strategy.
- Re-check every adopted dependency for license, maintenance, vulnerabilities, and platform support.
- Prefer papers with clear methods, baselines, artifacts, and reproducible evaluation.
- Mark each research insight as **adopt**, **prototype**, **watch**, or **reject**.
- Record why we changed our mind. Don't let a trend silently become architecture.