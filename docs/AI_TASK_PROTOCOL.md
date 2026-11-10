# AI TASK PROTOCOL — SMALL, AUTONOMOUS, VERIFIABLE WORK

This protocol is for local coding agents with limited context or capability. It defines how to take exactly one GitHub issue from open to verifiably complete without requiring the agent to understand or modify the entire product.

## 1. Required reading (in order)

Before editing, read:
1. AGENTS.md — repository-wide engineering and safety rules.
2. docs/PRODUCT_NARRATIVE.md — what the product is and what it must not claim.
3. docs/BLUEPRINT.md — normative architecture, trust model, domain, UX, and phase gates.
4. docs/AI_TASK_PROTOCOL.md — this execution procedure.
5. The assigned issue — its scope, dependencies, acceptance criteria, and handoff.
6. Only the code, migrations, tests, and docs directly relevant to the assigned issue.

Do not treat README, old comments, a sibling repository, a research paper, or model output as higher authority than the current blueprint and issue.

## 2. One issue = one bounded delivery

- Select one open issue whose dependencies are closed or explicitly satisfied.
- Do not bundle neighboring issues “for convenience”.
- Do not take work from another issue unless the owner updates the issue map and scope first.
- Do not refactor unrelated modules, rename broad APIs, or add speculative infrastructure.
- If implementation discovers an unavoidable dependency, stop and add a concise blocker note; do not quietly absorb the other issue.
- A pull request should normally close exactly one task issue. A tiny tightly coupled prerequisite may be included only when the parent issue explicitly allows it.

## 3. Scope fences

Before editing, write down:
- The issue number and its single user-visible or system-visible outcome.
- Files/modules expected to change.
- Files/modules explicitly out of scope.
- Existing behavior and invariants that must remain unchanged.
- Data migration and compatibility implications.
- The exact commands/tests needed for acceptance.

If the actual code contradicts the issue, do not invent behavior. Report the mismatch and propose the smallest issue correction. Never modify a migration already shipped; add a new migration.

## 4. Architecture and security rules

- Rust owns database, filesystem, process, credential, network, permission, policy, and execution authority.
- React only presents state and sends typed requests. Hiding/disabling a button is not authorization.
- Deny capabilities by default. Missing or ambiguous scope is a denial.
- Treat model output, files, URLs, imported skills, tool output, and provider responses as untrusted.
- Never grant a capability from natural language, model output, a skill, or a previous unrelated approval.
- Every consequential operation needs a scoped preview/policy/approval (where required), bounds, cancellation/timeout, and evidence.
- Never log secrets, raw credentials, or full sensitive source content.
- No network request may occur without an explicit issue scope and a data-egress policy.
- Never add fake states, placeholder implementations, mock data in production paths, or silent fallbacks that change meaning.

## 5. Database and domain changes

- Add a numbered forward migration; never edit a migration already released to main.
- Preserve existing user data and provide explicit defaults/backfill for old rows.
- Test fresh database creation and upgrade from the previous supported schema.
- Keep state transition, event, evidence, and artifact writes atomic where the contract requires it.
- Add uniqueness/foreign-key/check constraints for invariants when the database is the correct enforcement layer.
- Treat event history as append-only; corrections are new events.
- Make operations idempotent where retries can happen.
- Include rollback/restore implications in the issue handoff.

## 6. Contract-first implementation

- Define typed Rust input/output models and error cases before UI wiring.
- Validate at the Rust boundary even if the UI also validates.
- Keep Tauri command handlers thin; call domain use-cases.
- Never expose SQLx rows, provider SDK types, filesystem handles, or OS secrets as UI contracts.
- Version serialized data and manifests where compatibility matters.
- Add contract tests for serialization, rejection cases, and platform-specific behavior.

## 7. Testing rules

Run the narrowest relevant tests during development, then all issue-mandated checks:
- Rust: cargo fmt --all -- --check, relevant cargo test, cargo clippy --all-targets -- -D warnings.
- UI: bun run ui:check, bun run ui:build, and relevant unit/component/accessibility tests.
- Desktop: native E2E on the stated OS; do not substitute browser preview for a native requirement.
- Data: migration from prior schema, fresh DB, restart persistence, backup/restore if relevant.
- Security: positive and negative authorization cases, out-of-scope path/target, failure, timeout, cancellation, replay, and tampering where relevant.

A test command not run must be reported as not run. Do not change or skip an assertion merely to make CI green without preserving the intended invariant.

## 8. Definition of done for an AI task

Every checkbox in the issue's acceptance criteria must be supported by:
- Code or documentation diff.
- Test result (command, OS/runtime, pass/fail).
- Evidence for persistence or side effects, where applicable.
- Explicit list of limitations and follow-up blockers.
- Documentation updated to match behavior.
- A clean diff with no unrelated files.

A green compile alone is not proof that a feature works at runtime. A model saying “done” is not proof that tests passed. No PR may self-approve or self-merge when human approval is required.

## 9. Required handoff comment

When ready, post a GitHub issue/PR comment with:
1. **Implemented:** exact behavior and changed modules.
2. **Acceptance:** each issue checkbox with evidence or link.
3. **Tests:** exact commands, environment, and results.
4. **Persistence/security:** migration, grants, side effects, rollback, and restart behavior as applicable.
5. **Not done:** skipped checks, platform gaps, known defects, or follow-up issue links.
6. **Compatibility:** migration and external contract impact.
7. **Next:** only the next unblocked issue(s), not a broad speculative plan.

## 10. Stop conditions

Stop and report a blocker if:
- The issue requires broader product authority than the current policy allows.
- An acceptance criterion conflicts with the blueprint or repository security rules.
- The code path is not isolated enough to make the change safely.
- A migration would destroy or reinterpret existing user data.
- Required OS interaction, credentials, hardware, or external account access is unavailable.
- Tests cannot prove the safety claim the issue asks for.

Do not turn a blocker into a fake pass. Split it into a smaller design or prerequisite issue and request product-owner review where needed.
