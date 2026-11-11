# ROCKSOUL — PLATFORM SUPPORT MATRIX

**Status vocabulary:** Supported means the named capability has repeatable tests on that platform. Experimental means some implementation or partial automated evidence exists but at least one acceptance gate is still open. Unavailable means the capability is not implemented or is deliberately denied; it must not silently fall back to a weaker implementation.

This matrix is capability-specific. It does not imply full feature parity or general desktop support for an operating system.

## Current matrix

| Capability | Windows | macOS | Linux | Evidence / boundary |
|---|---|---|---|---|
| Rust core domain, SQLite store, metadata-only Scout | Experimental until the cross-OS CI matrix is green | Experimental until the cross-OS CI matrix is green | Experimental until the cross-OS CI matrix is green | Rust format, tests, and Clippy run on the three CI host families; passing CI is required before changing these cells to Supported |
| Native Tauri desktop launch and first local mission lifecycle | Experimental | Unavailable | Unavailable | Windows native E2E passes on the real Tauri executable and SQLite vault; macOS/Linux native runtime jobs and acceptance do not exist |
| Native folder picker select/cancel/invalid-path | Experimental | Unavailable | Unavailable | Windows dialog is wired, but the manual interactive shell-dialog acceptance in issue #6 remains open |
| OS-backed credential storage | Unavailable | Unavailable | Unavailable | No platform key-store adapter is implemented |
| Enforced process / filesystem execution sandbox | Unavailable | Unavailable | Unavailable | No platform enforcement profile is implemented; no unsandboxed fallback is permitted |
| Browser / desktop computer-use adapters | Unavailable | Unavailable | Unavailable | No supported adapter or native runtime acceptance exists |
| Screen-reader, contrast, reduced-motion acceptance | Experimental | Unavailable | Unavailable | UI includes native semantic elements, but accessibility acceptance and Windows Narrator smoke remain open |
| Installer, signing, updates, rollback, clean-machine install | Unavailable | Unavailable | Unavailable | Prototype bundling/release gates are not complete |

## CI coverage

The Rust core CI job runs formatting, core unit/integration tests, and Clippy on:

- Ubuntu hosted runner (`ubuntu-latest`)
- macOS hosted runner (`macos-latest`)
- Windows hosted runner (`windows-2022`)

The Windows native desktop job remains separate and runs on `windows-2022`. It builds the actual Tauri debug executable, starts the app through WebDriver, exercises the local mission lifecycle, restarts the process, and verifies persisted SQLite state. That native E2E evidence is not inferred from a cross-platform core compile.

The UI typecheck/build job remains on Ubuntu. It verifies the React/TypeScript application bundle but is not a native desktop runtime test.

## Rules for changing a status

1. A compile check alone cannot promote a desktop capability to Supported.
2. Every OS-specific integration needs a named test on the OS where support is claimed.
3. Manual OS dialogs, accessibility tools, credential stores, process isolation, and packaging require the appropriate native/manual evidence; CI on another OS cannot substitute.
4. An unavailable capability must return an explicit error or remain inaccessible. It must never fall back to unrestricted filesystem access, arbitrary shell execution, plaintext credential storage, or a weaker sandbox.
5. Update this matrix, README, TECH_STACK, and ROADMAP together when a capability's evidence changes.

## Current known gaps

- Issue #6: interactive Windows shell folder-picker acceptance.
- Issues #16–#17: desktop backup export and safe restore UI/use case.
- Issue #92 / PR #108: reviewed data-at-rest strategy; current prototype database is not encrypted by Rocksoul.
- Issues #21–#22: installer, signing, update, rollback, and clean-machine release gates.
- No macOS/Linux native desktop runtime test exists yet.
