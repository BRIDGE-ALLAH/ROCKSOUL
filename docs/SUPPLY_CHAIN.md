# ROCKSOUL — SUPPLY-CHAIN GOVERNANCE

## Purpose and trust boundary

Every release candidate should carry a dependency inventory and vulnerability snapshot that another reviewer can inspect. A green scanner result is evidence about one resolved dependency snapshot and one vulnerability database build; it is not a general security certification.

The workflow runs Syft over the repository after resolving the Rust and Bun dependency lock snapshots. It produces a CycloneDX JSON SBOM, scans that SBOM with Grype, applies the repository's license/vulnerability policy, and retains the report plus lock snapshots as a GitHub Actions artifact for 90 days.

The scanner receives the generated SBOM, not project source files, credentials, or private workspace data. Syft runs in the GitHub runner. Grype evaluates the SBOM locally after downloading its vulnerability database.

## Pinned tooling

| Tool | Version | Purpose |
|---|---|---|
| `anchore/sbom-action` | `v0.24.3` | Run Syft and write SBOM |
| Syft | `v1.57.1` | Catalog Rust, Bun, and repository packages |
| `anchore/scan-action` | `v7.4.2` | Run Grype against the SBOM |
| Grype | `v0.118.0` | Vulnerability matching |
| Rust | `1.99.0` | Resolve Cargo workspace dependencies |
| Bun | `1.4.4` | Resolve JavaScript workspace dependencies |

Versions are explicit in `.github/workflows/supply-chain.yml`. Scanner upgrades are deliberate review changes; do not use floating `latest` versions.

Syft added support for the Bun `bun.lock` lockfile in v1.46.0, so the pinned v1.57.1 scanner can inventory the Bun workspace lockfile as well as Cargo dependencies. See the [Syft release notes](https://github.com/anchore/syft/releases) and its Bun lockfile support change [#4625](https://github.com/anchore/syft/pull/4625).

## Dependency snapshot

The current prototype repository does not yet commit `Cargo.lock` or `bun.lock`. For every run, CI therefore creates fresh lock snapshots using the pinned Rust and Bun toolchains, checks Cargo metadata against the generated lock, and retains both lockfiles alongside the SBOM and scan report. These files document the exact resolved versions for that run; they do not pin dependency resolution between separate runs. Committing reviewed lockfiles is a remaining reproducibility hardening step before a public release.

Bun installation uses `--ignore-scripts`; package lifecycle scripts are not executed during inventory generation. Cargo lock generation resolves metadata without building dependencies.

## License policy

The initial allowlist is intentionally narrow and records common permissive SPDX identifiers:

- 0BSD
- Apache-2.0
- BSD-2-Clause and BSD-3-Clause
- CC0-1.0
- ISC
- MIT
- Unlicense
- Zlib

GPL/AGPL/SSPL families and BUSL-1.1 are disallowed by default. Any license that is missing, unknown, or outside the approved list is marked for review on pull requests. A published release is blocked until every such component has an exact, current exception or an approved license classification.

A license exception must name the exact component, version, license identifier, rationale, owner, and ISO expiry date. Exceptions are not inherited by new dependency versions.

## Vulnerability policy

Grype reports are grouped by severity. HIGH and CRITICAL findings fail CI unless an exact exception exists for the vulnerability ID, package name, and package version. Each exception must have a rationale, accountable owner, and expiry date. Expired or malformed exceptions fail CI even when the original finding is no longer present; maintainers must remove or explicitly renew them.

The generated report captures the Grype version and database build/status/checksum fields when supplied by the scanner. The JSON scan output and SBOM remain available for independent inspection.

## CI and release behavior

- Pull requests and pushes generate SBOM, Grype JSON, and a report. Disallowed licenses and unexcepted HIGH/CRITICAL vulnerabilities fail the policy job.
- Unknown license metadata is visible for owner review on PRs. It blocks a published release unless an exact, non-expired exception is present.
- A published GitHub release runs the stricter release license gate. The SBOM, Grype JSON, report, Cargo.lock, and bun.lock are attached to that workflow run as retained artifacts.
- No dependency is automatically upgraded by this workflow. Scanner version upgrades and dependency updates are separate reviewed changes.

## Exception and update cadence

Review SBOM/vulnerability evidence on every PR and published release. Review unresolved license metadata and active exceptions at least once per release. Exceptions should expire within 30 days by default; a longer period requires explicit security-owner rationale. Remove exceptions immediately after upstream remediation is verified.

See `.github/supply-chain/policy.json` for the machine-readable allowlist and exception records, and `scripts/tests/test_supply_chain_policy.py` for policy fixtures covering allowed/disallowed licenses, unknown-license release blocking, exact exceptions, expiry, and vulnerability severities.
