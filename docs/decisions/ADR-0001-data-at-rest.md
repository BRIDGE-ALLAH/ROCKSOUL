# ADR-0001: Local data-at-rest protection

- **Status:** Proposed — product-owner and security review required before sensitive payload persistence
- **Date:** 2026-11-11
- **Owners:** Rocksoul product and security maintainers
- **Related work:** [P0-F19 / Issue #92](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/92), [P0-F11 / Issue #15](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/15), [P0-F13 / Issue #17](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/17), [P1-M06 / Issue #28](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/28), [P1-M07 / Issue #29](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/29), [P1-M08 / Issue #30](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/30), [P4-MEM01 / Issue #61](https://github.com/BRIDGE-ALLAH/ROCKSOUL/issues/61)

## 1. Context and current facts

Rocksoul is a local-first desktop application. Local-first does not mean encrypted by default.

The current prototype stores the following in a SQLite database opened in WAL mode by SQLx:

- project names, canonical absolute workspace paths, and timestamps;
- mission titles, objectives, status, and timestamps;
- metadata-only Workspace Scout artifact JSON, including paths and directory metadata;
- evidence summaries, source paths, and JSON receipt details.

The current store creates the database under the app-local-data directory and configures SQLite WAL, foreign keys, a five-connection pool, and a five-second busy timeout. The current schema and content are **not encrypted by Rocksoul**. This ADR must not be read as a claim that existing local data is protected by application-level encryption.

The workspace itself remains user-owned and outside Rocksoul's data-at-rest boundary. A metadata-only scan still reveals local names and paths. Future source extraction, prompts, notes, memory, generated assets, and imported media can materially increase sensitivity; those features must not persist sensitive payloads until the required protection and review gates are satisfied.

## 2. Threat model

### Protected assets

1. Local project and mission metadata, including absolute paths and objectives.
2. Artifact and evidence content, provenance, and future imported source text.
3. Typed memory, user notes, model-generated interpretations, and future media.
4. Database files and sidecars, backups, exports, temporary files, caches, diagnostics, and crash reports.
5. Encryption keys, recovery material, provider credentials, and other secrets.

### Adversaries and trust boundaries

The design is intended to reduce exposure from:
- theft or offline inspection of a powered-off device or copied app-data directory;
- another local account or process that does not possess the user's OS key access;
- accidental disclosure through exported backups, temporary files, or ordinary logs;
- loss or corruption during key rotation, migration, backup, or restore.

The design does **not** promise protection from:
- malware or an attacker already executing as the logged-in user while the vault is unlocked;
- administrator/root compromise, kernel compromise, debugger/instrumentation access, keyloggers, or memory scraping;
- a user deliberately exporting or copying plaintext;
- sensitive files left in the user's workspace, cloud-synced folders, OS snapshots, or backups outside Rocksoul's control;
- weak account credentials, unlocked sessions, or an OS key store that is already unlocked.

Full-disk encryption (for example, BitLocker, FileVault, or LUKS) remains recommended as a device baseline. It complements, but does not replace, application-level encryption for copied database files and portable backups.

## 3. Data classification and storage rules

Classification is assigned at ingestion and follows content through derived artifacts, memory, caches, backups, and exports. Derived data may inherit the strictest classification of its inputs; a model summary or embedding is not automatically less sensitive than its source.

| Class | Examples | Storage rule |
|---|---|---|
| **Public** | Product documentation or content intentionally marked public | May be stored in ordinary local metadata stores. Remote egress still requires the mission's egress policy. |
| **Local** | Project name, workspace path, file names/extensions/sizes, mission labels, non-sensitive operation metadata | Local-only by default. Treat absolute paths and names as potentially identifying. Protect with the encrypted vault once the vault implementation is enabled. |
| **Sensitive** | User objectives/notes that reveal private work, selected source text, prompts containing private context, memory, generated content derived from private inputs, private media and detailed evidence | Do not persist in plaintext. Persist only after database/blob encryption and key lifecycle gates are implemented and reviewed. |
| **Secret** | Provider API keys, passwords, recovery secrets, encryption keys, authentication tokens | Store only in the platform secure-secret facility or a user-held recovery mechanism. Never store raw secrets in SQLite rows, artifacts, logs, prompts, caches, or exports. |

Unknown, malformed, or missing classification is treated as the strictest relevant class. A classification label is not an access grant and does not permit remote egress.

### Storage-surface rules

- **SQLite rows:** project/mission metadata is Local today. Future Sensitive fields require encrypted database storage. Secret values are never ordinary database fields.
- **Artifacts and evidence:** current inventory paths and receipts are Local and may reveal information. Future source excerpts, prompts, memory snapshots, and private media are Sensitive. The database encryption boundary must cover current artifact/evidence rows; detached blobs require their own authenticated encryption envelope.
- **Logs and traces:** operational metadata only. Redact source content, prompt bodies, credentials, keys, recovery material, and unnecessary full paths. Do not emit serialized database rows or provider payloads for debugging.
- **Caches and temporary files:** avoid persistent plaintext copies of Sensitive data. Prefer bounded in-memory processing. Any unavoidable temporary persistent representation must use the same encryption envelope, restrictive OS permissions, explicit lifetime, and best-effort cleanup. Never describe deletion from an SSD as guaranteed secure erasure.
- **Backups:** inherit the most restrictive classification included in the database. Every backup must be integrity-checked and encrypted. A raw copy of an active WAL database is not an acceptable backup procedure.
- **Exports:** require an explicit user action, a visible scope/contents preview, and an explicit protection choice. Portable Sensitive exports require a recovery passphrase or equivalent portable recovery envelope; a machine-bound key alone is insufficient.
- **Workspace sources:** Rocksoul's encrypted vault does not encrypt original files in the selected workspace. Read/import consent, source classification, and egress policy remain separate controls.

## 4. Platform key-storage comparison

Use a Rust-owned abstraction for key operations, with a native implementation per supported OS. Platform adapters must be tested on the actual supported OS/runtime, not inferred from compilation.

| Platform | Preferred root-key protection | Useful properties | Gaps and required behavior |
|---|---|---|---|
| Windows | DPAPI user scope for wrapping a random vault key; do not use machine-wide scope for a per-user vault | CryptProtectData protects a data blob and normally restricts decryption to the same user credentials and machine. | User/profile migration, administrator password reset, profile corruption, and device change can affect recovery. Do not represent DPAPI wrapping as a portable backup key. See [Microsoft DPAPI documentation](https://learn.microsoft.com/en-us/windows/win32/api/dpapi/nf-dpapi-cryptprotectdata). |
| macOS | Keychain Services item with explicit item access control for the app | Keychain provides encrypted storage for small secrets and cryptographic keys, with access-control facilities. | Login/keychain lock state, user authorization, app identity changes, and migration must be tested. See [Apple Keychain Services](https://developer.apple.com/documentation/security/keychain-services). |
| Linux | Freedesktop Secret Service through the active user's session | Standard interface for storing secrets in a user login session, with item/collection locking and unlock prompts. | A Secret Service implementation/session may be absent or unavailable in headless/minimal environments. Fail closed for Sensitive persistence; do not silently store plaintext. See [Secret Service API](https://specifications.freedesktop.org/secret-service/latest/description.html). |

Do not invent a common plaintext fallback when an OS facility is locked, unavailable, or errors. The UI must distinguish locked, unavailable, denied, and lost-key states. The user may use a separately documented recovery passphrase where supported; this is not a fallback to plaintext.

## 5. Decision

Adopt a staged **envelope-key architecture**, with the following implementation direction:

1. A cryptographically random vault master key (VMK) is generated by Rust. The VMK is never written in plaintext beside the database or artifacts.
2. The platform adapter stores/protects the VMK using the current user's native secure-secret facility. Store only non-secret key identifiers and format/version metadata in SQLite.
3. The database must be encrypted as a database, not by ad-hoc field-by-field encryption. The implementation spike should evaluate SQLCipher against the pinned SQLx/SQLite build and Tauri platforms. SQLCipher is the leading candidate, **not yet an approved implementation dependency**. Do not merge it until the spike proves supported Windows/macOS/Linux builds, key provisioning before any database access, schema migrations, integrity checks, WAL behavior, online backup, restore, and performance.
4. If a later feature stores detached artifact/media blobs, encrypt each blob with a unique data-encryption key (DEK) using an authenticated-encryption scheme selected through the implementation review. Wrap the DEK under a key derived from the VMK using domain-separated key derivation. Bind object ID, format version, and media/content type as authenticated metadata. Never reuse a nonce with the same AEAD key.
5. Backups and portable exports use an encrypted recovery envelope. The envelope wraps the VMK using a user-provided recovery passphrase processed by Argon2id with versioned salt and cost parameters selected through hardware/performance testing. The OS-wrapped VMK is not sufficient for cross-machine restore. Never store or log the passphrase.
6. A restore on a new user profile/device must first unlock the recovery envelope, then re-wrap the VMK into the destination OS key store. No network account or remote key service is required by this ADR.
7. Sensitive source extraction, persistent context, durable memory, and private media remain blocked from persistent storage until the relevant encryption, backup/restore, deletion, and product/security review gates pass.

This is a staged decision rather than a claim that SQLCipher or blob encryption is already implemented. The project must keep the current honest capability statement until those gates are met.

### Why a defer gate is part of the decision

The existing app uses SQLx SQLite and WAL. Selecting a database encryption library without proving it against the exact SQLite linkage, build targets, WAL handling, online backup, and restore path would create a false security guarantee and risk data loss. The SQLite Online Backup API supports consistent snapshots of a live database; the WAL file is part of persistent state and must not be separated from the database during raw file copying. Those behaviors must be tested with the selected encrypted build, not assumed.

References:
- [SQLite Online Backup API](https://www.sqlite.org/backup.html)
- [SQLite Write-Ahead Logging](https://www.sqlite.org/wal.html)
- [SQLCipher project documentation](https://www.zetetic.net/sqlcipher/)

## 6. Key lifecycle and recovery

### Create and access

- Generate VMK material using an OS cryptographic random source through a maintained Rust cryptographic API.
- Store only the OS-protected VMK and stable key/version identifiers in the secure-secret facility.
- Unlock only when the app needs vault access. Keep plaintext key material in memory only for the shortest practical period and clear owned buffers on drop where the runtime permits; do not claim this defeats process-memory inspection.
- The app must report key-store state truthfully. A missing/locked key must never be interpreted as a new empty database.

### Rotation

- Distinguish **rewrapping** (changing the OS/recovery wrapper while keeping the VMK) from **data-key rotation** (re-encrypting data under new keys).
- Rewrapping must be atomic and preserve a verified recovery path.
- Data-key rotation must use a staged new vault/database or blob version, verify integrity and record/content digests, sync to durable storage, then atomically switch. Retain a clearly identified recoverable old version until verification is complete; never overwrite the only usable copy.
- Version envelopes, wrapped keys, KDF parameters, and blob formats. Reject unknown versions rather than guessing.

### Loss and recovery

- If the OS key store is temporarily locked/unavailable, explain the state and allow retry. Do not create a new VMK.
- If the OS-protected VMK is genuinely lost, recover only through a previously created recovery envelope/passphrase. Without recovery material, encrypted data is unrecoverable; show this before users enable encryption or export a protected backup.
- A recovery passphrase must be chosen by the user, never generated and then silently discarded. Provide a one-time recovery confirmation flow without sending the passphrase to telemetry.
- On user-profile or device migration, restore the recovery envelope and re-wrap the VMK under the destination OS facility. Windows DPAPI user/machine binding makes a portable recovery envelope necessary.

### Deletion

- Removing the OS-wrapped VMK can provide cryptographic erasure only for copies whose payloads are actually encrypted under that key and only after external recovery envelopes/backups are accounted for.
- Explicit vault deletion must enumerate app-owned database, WAL/SHM, backup, export, cache, and temporary locations; preserve user-owned workspace files unless separately and explicitly selected.
- For legacy plaintext databases, removing a key does nothing. Do not claim secure erase of plaintext copies, OS snapshots, cloud-sync versions, or SSD blocks. Explain the limits and require explicit confirmation for destructive deletion.

## 7. Migration from the current unencrypted prototype

The current database is unencrypted. Migration must be explicit, recoverable, and interruption-safe:

1. Detect the legacy schema and display that the existing database is currently unencrypted by Rocksoul.
2. Before migration, ensure a verified recovery copy exists and explain whether that copy remains plaintext.
3. Require explicit user approval to migrate. Do not silently create a new encrypted vault and leave an undisclosed plaintext copy behind.
4. Create a new encrypted database in a staging location. Copy schema/data using a transactionally consistent method; preserve stable IDs, timestamps, relationships, and artifact digests.
5. Verify schema version, row counts by table, foreign-key checks, integrity result, and artifact/evidence digests. Close all connections before final replacement.
6. Flush staged data and perform an atomic same-volume replace/switch where supported. If any step fails, keep the old database intact and reopen it only with a clear unencrypted-state warning.
7. After success, show any retained plaintext recovery copy and let the user explicitly delete it or keep it with a warning. Do not imply that OS/SSD-level remnants have been securely erased.

Migration and restore tests must include interrupted migration at each phase, corrupt ciphertext, missing key, wrong recovery passphrase, low disk space, and rollback to the last verified copy.

## 8. Backup, WAL, restore, and performance requirements

- Use SQLite's Online Backup API or a proven WAL-safe equivalent. Never copy only the main .sqlite3 file while the app is active and assume it includes committed WAL pages.
- The encrypted backup must carry a non-secret manifest: format version, database schema version, Rocksoul app version, creation timestamp, cipher/KDF identifiers and parameters, key ID, integrity result, byte length, and digest. Do not include passwords, raw keys, source payloads, or prompt content in the manifest.
- Validate authentication/integrity before restore. Then validate SQLite integrity and schema compatibility before the new vault becomes active.
- Restore into a staging file, preserve a recoverable prior vault, sync, close pools, and switch atomically. A failed restore must not leave a half-open or silently empty vault.
- Benchmark startup, normal writes/reads, migrations, backup, and restore on minimum supported hardware and representative database sizes. Define memory/time budgets before selecting KDF parameters. Argon2id is the candidate for passphrase-derived wrapping keys; its parameters must be versioned and tuned to the supported desktop budget. See [RFC 9106](https://www.rfc-editor.org/rfc/rfc9106.html).
- Include WAL sidecars, temporary files, crash recovery, locked key stores, and disk-full conditions in the platform matrix.

## 9. Release and feature gates

No feature may claim Sensitive data is protected at rest until all applicable gates are evidenced:

- [ ] Security/product review of this ADR and approved threat-model residual risks.
- [ ] SQLCipher/SQLx compatibility spike passes on Windows, macOS, and Linux targets supported for the release, or a reviewed alternative is selected.
- [ ] Key-store adapter integration tests prove create/lock/unlock/denied/missing/loss behavior on each platform.
- [ ] Database migration, WAL, integrity, backup, restore, rewrap, and recovery tests pass with encrypted data.
- [ ] Blob/export encryption tests pass before detached Sensitive payloads are persisted or exported.
- [ ] Legacy plaintext migration, interrupted migration, and user-visible deletion warnings are tested.
- [ ] Logs, crash reports, caches, and temporary-file review shows no Sensitive plaintext or Secret leakage.
- [ ] Product copy and documentation state precise limits; no unverified "encrypted" or "secure erase" claim.

Until those gates pass:
- continue to classify current local SQLite data as unencrypted by Rocksoul;
- do not persist Sensitive source text, prompts, memory, or private media in ordinary SQLite/artifact/cache/log files;
- keep Secret values out of ordinary stores;
- keep unknown/secret remote egress denied by default.

## 10. Accepted residual risks

Even after implementation, local application encryption cannot protect data while the user's session and app are compromised or the vault is unlocked. Decrypted values and keys necessarily exist in process memory during use. OS key-store security depends on OS configuration and account strength. Portable recovery depends on the user's recovery secret; loss can mean permanent data loss. Workspace originals, exports, screenshots, external backups, OS snapshots, filesystem journaling, and SSD wear-leveling can retain copies outside Rocksoul's control. This ADR does not claim remote key escrow, multi-device sync, enterprise key recovery, or protection against a privileged local attacker.

## 11. Review record

- **Product-owner review:** Pending.
- **Security review:** Pending.
- **Implementation status:** No encryption code, schema migration, or key-store integration is included in this ADR.
- **Sensitive-persistence gate:** Not approved until product-owner/security review and the release gates above are evidenced.
