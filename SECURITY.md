# Security

Rocksoul is a local-first desktop application. Security boundaries belong in the Rust core and operating system—not in UI visibility or model instructions.

## First-slice security posture

The Workspace Scout is deliberately metadata-only:
- It never opens file contents.
- It does not execute commands or child processes.
- It does not make network/provider requests.
- It does not modify the selected workspace.
- It does not follow symbolic links.
- It respects Git ignore and applies depth/file-count limits.
- It excludes common secret and credential names.
- It stores its inventory and evidence in the app-local database.

This is a conservative first slice, not a complete sandbox. Directory names and file metadata can still be sensitive. The inventory is local and is not encrypted by this feature.

## Important limitations

- The current UI accepts a filesystem path typed by the user. It does not yet provide a native folder-picker capability.
- The app has not yet implemented source-content reads, shell execution, browser automation, model-provider egress, or third-party skills.
- No production installer, updater, signature verification, or backup/restore workflow is enabled.
- The current tests do not replace OS-level security testing or a penetration test.

## Security design rules

1. Treat project content, provider responses, model output, and imported skills as untrusted.
2. Deny tool capabilities by default.
3. Enforce permissions in Rust, never only in React.
4. Scope file and process access to explicit project/capability grants.
5. Preview consequential actions before execution.
6. Bind approvals to a specific operation and target.
7. Keep secrets out of prompts, artifacts, and ordinary logs.
8. Make operations bounded, cancellable, and auditable.
9. Distinguish an actual test result from a model's claim that something passed.
10. Never label a project "safe" or "verified" based only on an inventory.

## Reporting

Please do not open public issues for suspected exploitable vulnerabilities. Contact the repository owner through GitHub's private security reporting channel where available. Include the affected commit, environment, reproduction steps, and impact. Do not attach real secrets or private user data.