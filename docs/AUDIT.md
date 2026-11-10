# ROCKSOUL — REPOSITORY & ECOSYSTEM AUDIT

**Audit date:** 10 November 2026  
**Method:** repository metadata and selected project manifests/README/configuration files reviewed through GitHub. Findings are static inspection unless explicitly noted.

> This is not a runtime test report. No Windows installer, desktop launch, model call, sandbox exploit, or live provider execution was performed as part of this document.

## 1. Target repository: ROCKSOUL

**Repository:** [BRIDGE-ALLAH/ROCKSOUL](https://github.com/BRIDGE-ALLAH/ROCKSOUL)

At audit time:
- Public repository exists.
- Default branch is `main`.
- Repository is empty: no README, license, Cargo manifest, package manifest, or code tree was present.

**Conclusion:** clean slate. There is no legacy implementation to preserve, so establish product scope and architecture before adding dependencies.

## 2. Sibling repositories

### ROCKSOUL-EX

**Repository:** [BRIDGE-ALLAH/ROCKSOUL-EX](https://github.com/BRIDGE-ALLAH/ROCKSOUL-EX)

Static review of README and package manifest shows a VS Code engineering companion, version 0.3.0, with:
- Read-only repository inspection (Witness).
- Local project DNA and mission/handoff records.
- OpenAI Responses API adapter.
- Structured patch proposals and VS Code diff review.
- Second explicit approval before applying changes.
- Command verification with bounded evidence.
- Workspace Trust restrictions and SecretStorage provider credentials.
- No required server/database/telemetry according to the README.

**Keep / reuse as design patterns:**
- Understand before changing.
- Scope a mission and patch.
- Separate patch generation, patch approval, and command execution.
- Preserve project memory in human-readable form.
- Treat evidence as part of completion.

**Do not assume:** this extension is a reusable Rust library or a desktop runtime. It is a separate Node/VS Code extension product. Reuse contracts and lessons first; extract code only after reviewing implementation, tests, license, and maintenance boundaries.

### ROCKSOUL-WX

**Repository:** [BRIDGE-ALLAH/ROCKSOUL-WX](https://github.com/BRIDGE-ALLAH/ROCKSOUL-WX)

Static review of README and Cargo workspace shows a Rust 2024 workspace with Axum 0.8 and Dioxus 0.7. It describes:
- Typed workload contracts.
- Deterministic, fail-closed privacy/egress policy.
- Execution planning and provider ranking.
- Local/private/public execution locations.
- Authentication, rate limits, CORS, provider timeouts, and metadata receipts.
- Dioxus Command Deck and Windows gateway packaging.

Its own README also discloses important limitations:
- Single-operator, not multi-tenant.
- In-memory per-key rate limiting.
- No streaming, multimodal inputs, or embeddings.
- No automatic provider failover after execution error.
- Hard cost reservation not implemented.
- Human approval mode blocked until a workflow exists.
- Operator-maintained provider quality/latency/price values.

**Keep / reuse as design patterns:**
- Policy-first provider routing.
- Typed workload contract.
- Explicit data classification and egress.
- Provider adapter boundary.
- Clear disclosure of what "passed" means.

**Do not assume:** WX is already a general-purpose agent engine or desktop client. Its gateway should remain optional behind a documented contract. Do not duplicate its code or deploy it as a required dependency for the first Rocksoul release.

## 3. MOONWITNESS monorepo — lessons from prior deep review

**Repository:** [BRIDGE-ALLAH/MOONWITNESS](https://github.com/BRIDGE-ALLAH/MOONWITNESS)

The earlier static deep audit is tracked in [Issue #11](https://github.com/BRIDGE-ALLAH/MOONWITNESS/issues/11). The desktop-specific findings are summarized here because they directly affect what Rocksoul must avoid inheriting.

### High-priority desktop findings from static inspection

| Area | Finding | Consequence |
|---|---|---|
| React providers | SyncStatusProvider was absent from the component provider tree while a layout component used its hook. | UI may fail during render; prove the shell mounts before feature work. |
| Authentication | Windows Hello/PAT flows were placeholders; JWT generation path attempted incompatible key handling; login page and Zustand auth state were disconnected. | No production auth claim until end-to-end identity tests pass. |
| Local database | Desktop health check was unconditional and local SurrealDB used in-memory `Mem`. | Data durability and health reporting were not real. |
| Sidecars | Health probes did not match each service's actual HTTP/UDP protocol; manager was not wired to mesh commands. | Reported service health and actions could be fictional. |
| Processes | Piped stdout/stderr were not drained; shutdown/supervision was incomplete. | Child processes can hang or deadlock. |
| Sync | Sync engine had no durable outbox/configuration; CRDT was simplified; table names were interpolated into queries. | Sync correctness, injection resistance, and restart recovery were not established. |
| Native commands | Multiple pages/commands served hard-coded demo data; export UI could claim a ZIP was saved without generating one. | Product trust is damaged by fake completion. |
| Licensing | TPM/fingerprint checks were stubs or simplified; UI hard-coded license-valid state. | Not production licensing. |
| Packaging | MSI/EXE/MSIX paths, external binaries, updater endpoint/public key, plugin declarations, scripts, and app versions were inconsistent. | Release/update path not ready. |

### Additional architectural inconsistency

The monorepo root uses Bun 1.4.4 and workspaces. The desktop UI manifest declares React 18.3, while the shared UI package declares React 19 as its peer dependency. The same UI manifest pins Vite 5.2-era tooling, while the new Rocksoul proposal uses Vite 8.x. This is a dependency alignment and upgrade risk—not proof of a runtime failure by itself.

### What Rocksoul should learn

- Start with one actual app shell and persistent local project.
- Health status must be measured against the real service contract and protocol.
- Authentication must be one connected flow, not several UI placeholders.
- Sidecar lifecycle needs supervision, log draining, health, cancellation, and shutdown.
- Don't introduce a database without proving persistence, backup, and migration.
- Never show "saved", "verified", "synced", or "healthy" unless the operation really happened.
- Package/update signing and release tests are architecture, not last-minute polish.

## 4. Risks if we combine repositories too early

1. **Release coupling:** a desktop product gets tied to monorepo service release cadence.
2. **Dependency collisions:** different React versions, Rust toolchains, and UI stacks leak across workspaces.
3. **Unclear ownership:** it becomes unclear which repo owns provider policy, memory, auth, and agent execution.
4. **False production confidence:** a service being present in Compose or a UI showing a status does not prove an end-to-end capability.
5. **Security inheritance:** unfinished placeholder flows and broad local privileges spread into a new product.

## 5. Decision

**ROCKSOUL should be a separate product repository.**

- ROCKSOUL owns desktop UX, mission harness, project domain, local memory, artifacts, skills, tool capability broker, and evals.
- ROCKSOUL-EX remains a VS Code engineering companion.
- ROCKSOUL-WX remains a provider/workload gateway.
- MOONWITNESS remains the broader infrastructure/service ecosystem.

Integrate through versioned API and event contracts. Extract shared Rust crates only when a real stable shared boundary exists.

## 6. Audit follow-up gates

- [ ] Fresh clone and install on Windows.
- [ ] App starts and exits cleanly; repeated launch is safe.
- [ ] Project data persists across app restart and OS restart.
- [ ] Backup/restore round-trip passes.
- [ ] File scope, traversal, symlink, hard-link, and TOCTOU tests pass.
- [ ] Secrets are kept outside model context and ordinary logs.
- [ ] Tool grants are enforced in Rust, not only in UI.
- [ ] Process output is drained; timeout, cancellation, crash, and restart are tested.
- [ ] Provider egress policy tested with local mock servers.
- [ ] Every completion state is backed by actual evidence.
- [ ] Signed installer, update metadata, and rollback tested on a clean machine.
- [ ] Accessibility and keyboard paths tested in a real desktop session.

## 7. Evidence baseline

The project begins with no runtime code, so no implementation checklist should be marked complete yet. The findings above are architecture and repository lessons from source inspection, not fresh test results.