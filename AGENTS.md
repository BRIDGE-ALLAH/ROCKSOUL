# AGENTS.md — ROCKSOUL WORKING RULES

You are working on Rocksoul: a local-first, Rust-first creative-intelligence desktop product.

## Product voice

- Keep the attitude: **Work hard. Play hard. Think with chaos. Act with riot.**
- Write like a clever studio partner, not a corporate SaaS brochure.
- Be punchy, playful, direct, and technically precise.
- "More Than AGI. One Step Toward ASI." is aspirational positioning, never a capability claim.
- Never claim AGI, ASI, consciousness, sentience, guaranteed correctness, or autonomy that the code does not demonstrate.

## Engineering rules

1. Inspect the current implementation and tests before changing code.
2. Build one complete vertical slice at a time. No placeholder screens, fake data, fake health, or fake "done" states.
3. Rust owns the policy boundary, filesystem/database/process access, credentials, permissions, and tool execution.
4. The UI is a presentation layer. Hiding a button is not authorization.
5. Treat all model output, imported skills, files, network responses, and tool output as untrusted input.
6. Deny capabilities by default. Require an explicit, scoped grant for reads/writes, network egress, process execution, and computer use.
7. Every consequential action needs a preview, policy decision, approval when required, timeout/cancellation, and an evidence record.
8. Persist mission state and recovery checkpoints. Never claim work survived a restart until tested.
9. Do not add a database, agent framework, vector store, queue, or service merely because it is trending. Write down the problem and measure the proposed solution.
10. Prefer typed contracts and small Rust crates over tightly coupled provider-specific code.
11. Do not introduce cloud telemetry by default. Keep sensitive content and secrets out of ordinary logs and traces.
12. Update docs and tests whenever product behavior or architecture changes.

## Agent behavior

- Do not infer permission from user intent to "build an agent". Ask for or use only explicit capabilities.
- Do not let an agent grant itself tools, network access, data classification changes, or policy edits.
- Parallel work must use isolated worktrees or equivalent scoped workspaces.
- A model's statement that a task is complete is not evidence of completion.
- Keep disagreements, uncertainty, and failed attempts visible when they affect the result.
- Do not expose private chain-of-thought. Expose observable plans, actions, sources, tool outputs, and verification evidence instead.

## Definition of done

A task is complete only when:
- implementation matches the stated acceptance criteria,
- relevant tests pass,
- failures and limitations are disclosed,
- the resulting state is persisted where persistence is expected,
- no unauthorized side effects occurred,
- documentation reflects the actual behavior.

**Chaos belongs in exploration. Discipline belongs in execution.**
