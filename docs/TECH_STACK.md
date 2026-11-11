# ROCKSOUL — TECH STACK

**Rule:** modern, boring where reliability matters, experimental where it creates an edge.

Don't pick a framework because it is loud on social media. Pick it because the project can ship, test, secure, and maintain it.

## Recommended baseline

| Layer | Pick | Why |
|---|---|---|
| Desktop shell | **Tauri 2.12.5** | Native Rust host + system WebView; small footprint, good OS integration, explicit capability permissions. |
| Native core | **Rust 1.99.0, Edition 2024** | Strong types, memory safety, native performance, mature async and systems ecosystem. |
| UI | **React 19.3.0 + TypeScript** | Fast iteration for a dense creative desktop UI; large component, editor, and accessibility ecosystem. |
| Build | **Vite 8.3.7 + Rolldown** | Fast frontend workflow; Vite 8 moved to a unified Rust-based Rolldown bundler. |
| JS toolchain | **Bun 1.4.4** | Workspaces, package manager, test runner, scripts. Use it for UI/tooling—not as a replacement for the native security boundary. |
| Async runtime | **Tokio** | Background jobs, streaming, provider calls, cancellation, and process supervision. |
| Local database | **SQLite + WAL + FTS5** | Durable local store, full-text search, migrations, portable backups. |
| Rust DB layer | **SQLx 0.9.x** | Async typed SQL layer with SQLite support. |
| Serialization | **Serde + JSON/JSON Schema** | Typed contracts between UI, core, providers, and tools. |
| HTTP/providers | **Reqwest + Rustls** | Provider adapters with TLS and explicit timeouts. |
| Observability | **tracing** | Structured spans for missions, tools, provider requests, and failures. |
| UI primitives | **Radix UI / React Aria + Tailwind CSS** | Accessible interaction primitives plus flexible styling for the Rocksoul visual system. |
| Code understanding | **Tree-sitter + ripgrep** | Deterministic structural and lexical repo inspection before semantic/vector retrieval. |
| Plugin sandbox (later) | **Wasmtime / WASI Components** | Potential portable sandbox for third-party extensions; introduce only with a tested capability model. |

Implementation pin snapshot on 10 Nov 2026: Rust 1.99.0; Tauri 2.12.5 with tauri-build 2.7.4, CLI 2.12.4 and JS API 2.12.4; React 19.3.0; Vite 8.3.7 with @vitejs/plugin-react 6.3.0; Bun 1.4.4; SQLx 0.9.0. These are starting pins, not a substitute for a committed lockfile, CI build, vulnerability audit, or compatibility test. Refresh and pin exact transitive dependency versions before shipping.

References:
- [Tauri architecture](https://v2.tauri.app/concept/architecture/)
- [Tauri 2.12.5 release](https://github.com/tauri-apps/tauri/releases/tag/tauri-v2.12.5)
- [Tauri build 2.7.4](https://docs.rs/tauri-build/2.7.4/tauri_build/)
- [React versions](https://react.dev/versions)
- [Vite releases](https://github.com/vitejs/vite/releases)
- [Bun 1.4.4](https://github.com/oven-sh/bun/releases/tag/bun-v1.4.4)
- [SQLx 0.9.0](https://docs.rs/sqlx/0.9.0/sqlx/)
- [Vite 8 announcement](https://vite.dev/blog/announcing-vite8)
- [Vite releases](https://vite.dev/releases)
- [Bun docs](https://bun.sh/docs)
- [SQLx](https://docs.rs/sqlx/latest/sqlx/)
- [SQLite FTS5](https://www.sqlite.org/fts5.html)
- [Wasmtime](https://docs.rs/wasmtime/latest/wasmtime/)

## Why not "all Rust UI"?

All-Rust UI is a valid option, not automatically a better product. Dioxus, Slint, Iced, egui, and GPUI are worth tracking. But Rocksoul's interface will likely need a rich canvas, command palette, editor surfaces, accessible dialogs, drag/drop, virtualized lists, and a large set of polished interactions.

**Default decision:** Rust for native core and trust boundary; React/TypeScript for product UI. Keep the boundary typed and narrow.

Run a 2-week UI spike comparing Tauri + React with Dioxus/Slint on:
- keyboard-first navigation,
- large virtualized project canvas,
- accessible controls,
- file drag/drop,
- state restoration,
- memory and startup time,
- Windows packaging and installer update.

Switch only if the alternative wins on measured developer experience, UX quality, binary behavior, and maintenance.

## Architecture rules

### Rust owns

- Filesystem and repository access.
- Database and migrations.
- Mission/task state and persistence.
- Permission checks and action previews.
- Provider credentials and secret access.
- Child process supervision and sandboxing.
- Tool execution and evidence capture.
- Model routing policy.

### TypeScript owns

- Screens, panels, interactions, layout.
- Local view state and optimistic UI where safe.
- Rendering artifact previews and trace timelines.
- Form validation and user-facing error recovery.

The UI must never be the only place where a security rule is enforced.

## Local data design

**Start with SQLite, not a distributed database.**

- WAL mode for normal concurrent read/write usage.
- Foreign keys and explicit migrations.
- FTS5 for text search.
- Structured tables for projects, missions, events, artifacts, grants, approvals, and eval runs.
- Append-only event ledger for important changes; read models can be rebuilt.
- Content-addressed artifact blobs and explicit backup/restore.
- Encryption strategy reviewed before storing secrets or sensitive personal data.

Vector search is an optional index, not the memory system. Begin with FTS5, path ranking, file metadata, symbol search, and Tree-sitter. Add local embeddings only if an evaluation dataset proves a meaningful retrieval lift.

## Agent runtime

Do not give an LLM a direct filesystem or shell handle.

Build a compact harness around:
1. Mission contract and acceptance criteria.
2. Context compiler with a token/byte budget.
3. Typed plan and branch proposals.
4. Model/provider adapter.
5. Capability broker and policy gate.
6. Tool runner with timeout, cancellation, and sandbox.
7. Evidence collector and verifier.
8. Persistent event log and replayable action trace.

Keep the core loop understandable. Add subagents only when tasks can be separated and independently verified.

## Provider strategy

- Provider-neutral internal request/response contract.
- Separate adapters for OpenAI-compatible APIs, native provider APIs, and local inference.
- Explicit capability discovery: tool calls, structured output, vision, audio, streaming, context limits.
- Per-model eval profile—not hard-coded "best model".
- Route by task, quality measured on our evals, latency, cost, privacy class, and availability.
- Fail closed when no provider satisfies data/egress policy.
- Store provider metadata, not secrets, in project records.

Local inference can use an isolated sidecar such as llama.cpp or another maintained runtime. Don't embed model weights into the GUI process in v0.1.

## Tool and extension strategy

- Native tools for trusted built-in operations.
- MCP adapter for compatibility with external tool ecosystems.
- Human-readable Skills folders with `SKILL.md`, manifest, version, examples, and tests.
- Skills declare capabilities and data needs; a skill cannot grant itself permissions.
- Use OS sandboxing for processes. Use Wasmtime/WASI for portable untrusted plugins when a concrete plugin API exists.
- Do not run arbitrary scripts merely because a model generated them.

## Things we deliberately won't add in v0.1

| Technology/pattern | Why we're holding it |
|---|---|
| Kubernetes / microservices | A single-user desktop does not need distributed ops overhead. |
| Kafka / NATS / Redis | A local SQLite-backed event loop is enough initially. |
| SurrealDB as primary desktop DB | A graph-shaped data model does not justify introducing a second persistence risk before product queries are measured. |
| Vector DB on day one | Deterministic retrieval is simpler to inspect and often sufficient for repository context. |
| LangChain/LangGraph as the core | Keep the harness thin and model-independent; integrate selected adapters, not framework lock-in. |
| Full multi-agent orchestration | Start with one reliable agent, one verifier, and isolated optional specialists. |
| Raw shell execution | Too much authority; use narrow capabilities and an approval gate. |
| Bun as system runtime | Rust owns native process, filesystem, and policy boundaries. |

## Quality and security gates

- `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`.
- Type-check, lint, and test the UI.
- Contract tests between UI and Rust commands.
- Property tests for policy, path resolution, budgets, and migration invariants.
- Security tests for path traversal, symlinks, hard links, TOCTOU, command injection, and secret leakage.
- Golden fixtures for trace/evidence output.
- E2E tests on Windows first, then Linux/macOS.
- SBOM, dependency/license review, signed releases, update metadata validation.

## Platform stance

The evidence-backed per-capability support status and CI coverage live in [PLATFORM_SUPPORT.md](PLATFORM_SUPPORT.md). A platform's inclusion in the design target list below is not a claim of implemented or supported runtime behavior.

Design for Windows first if that's the first actual user target, but keep the core platform-neutral. Platform integrations must be explicit adapters:
- Windows: WebView2, Job Objects, Credential Manager, named pipes.
- macOS: WKWebView, sandbox/container APIs, Keychain, XPC/Unix socket.
- Linux: WebKitGTK, namespaces/seccomp/bubblewrap where available, Secret Service, Unix socket.

Never claim that a sandbox is active until the platform-specific enforcement is implemented and tested.