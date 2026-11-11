#!/usr/bin/env python3
"""Fail-closed policy checks for a CycloneDX SBOM and Grype JSON report.

The scanner produces evidence; this script makes the repository's policy
explicit and testable. It never fetches source code or contacts a scanner.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import sys
from collections import Counter
from datetime import date
from pathlib import Path
from typing import Any


def read_json(path: str | Path) -> dict[str, Any]:
    with Path(path).open(encoding="utf-8") as handle:
        value = json.load(handle)
    if not isinstance(value, dict):
        raise ValueError(f"{path} must contain a JSON object")
    return value


def exception_is_expired(exception: dict[str, Any], today: date) -> bool:
    expires = exception.get("expires")
    if not isinstance(expires, str):
        return True
    try:
        return date.fromisoformat(expires) < today
    except ValueError:
        return True


def validate_exceptions(policy: dict[str, Any], today: date) -> list[str]:
    errors: list[str] = []
    for key in ("license_exceptions", "vulnerability_exceptions"):
        items = policy.get(key, [])
        if not isinstance(items, list):
            errors.append(f"Policy field {key} must be an array.")
            continue
        for index, item in enumerate(items):
            if not isinstance(item, dict):
                errors.append(f"{key}[{index}] must be an object.")
                continue
            missing = [
                field
                for field in ("rationale", "owner", "expires")
                if not str(item.get(field, "")).strip()
            ]
            if missing:
                errors.append(
                    f"{key}[{index}] is missing required field(s): {', '.join(missing)}."
                )
            if exception_is_expired(item, today):
                errors.append(
                    f"{key}[{index}] has an invalid or expired exception date "
                    f"({item.get('expires')!r}); remove or renew it with review."
                )
    return errors


def license_atoms(expression: str) -> list[str]:
    cleaned = expression.replace("(", " ").replace(")", " ")
    return [
        token
        for token in re.split(r"\s+(?:AND|OR)\s+|\s+", cleaned.strip())
        if token and token.upper() not in {"WITH"}
    ]


def classify_license(expression: str, policy: dict[str, Any]) -> str:
    approved = set(policy.get("approved_spdx", []))
    disallowed_exact = set(policy.get("disallowed_exact", []))
    prefixes = tuple(policy.get("disallowed_prefixes", []))
    atoms = license_atoms(expression)
    if not atoms:
        return "review"
    if any(atom in disallowed_exact or atom.startswith(prefixes) for atom in atoms):
        return "disallowed"
    if all(atom in approved for atom in atoms):
        return "approved"
    return "review"


def extract_component_licenses(component: dict[str, Any]) -> list[str]:
    found: list[str] = []
    for item in component.get("licenses", []) or []:
        if not isinstance(item, dict):
            continue
        expression = item.get("expression")
        license_data = item.get("license")
        if isinstance(expression, str) and expression.strip():
            found.append(expression.strip())
        elif isinstance(license_data, dict):
            identifier = license_data.get("id") or license_data.get("name")
            if isinstance(identifier, str) and identifier.strip():
                found.append(identifier.strip())
    return found


def license_exception_matches(
    exception: dict[str, Any],
    component: dict[str, Any],
    license_expression: str,
) -> bool:
    return (
        exception.get("component") == component.get("name")
        and str(exception.get("version", "")) == str(component.get("version", ""))
        and exception.get("license") == license_expression
    )


def assess_licenses(
    sbom: dict[str, Any], policy: dict[str, Any]
) -> tuple[Counter[str], list[str]]:
    counts: Counter[str] = Counter()
    errors: list[str] = []
    components = sbom.get("components", []) or []
    exceptions = policy.get("license_exceptions", []) or []

    for component in components:
        if not isinstance(component, dict):
            continue
        name = str(component.get("name") or "(unnamed component)")
        version = str(component.get("version") or "(version unknown)")
        expressions = extract_component_licenses(component)
        if not expressions:
            expressions = ["NOASSERTION"]

        for expression in expressions:
            status = classify_license(expression, policy)
            exception = next(
                (
                    entry
                    for entry in exceptions
                    if license_exception_matches(entry, component, expression)
                ),
                None,
            )
            if exception is not None:
                status = "approved-by-exception"

            counts[status] += 1
            subject = f"{name}@{version} ({expression})"
            if status == "disallowed":
                errors.append(f"Disallowed license: {subject}.")
            elif status == "review" and not exception:
                counts["review-required"] += 0
    return counts, errors


def vulnerability_exception_matches(
    exception: dict[str, Any],
    vulnerability_id: str,
    package_name: str,
    package_version: str,
) -> bool:
    return (
        exception.get("vulnerability_id") == vulnerability_id
        and exception.get("package") == package_name
        and str(exception.get("version", "")) == package_version
    )


def assess_vulnerabilities(
    grype: dict[str, Any], policy: dict[str, Any]
) -> tuple[Counter[str], list[str]]:
    counts: Counter[str] = Counter()
    errors: list[str] = []
    exceptions = policy.get("vulnerability_exceptions", []) or []

    for match in grype.get("matches", []) or []:
        if not isinstance(match, dict):
            continue
        vulnerability = match.get("vulnerability") or {}
        artifact = match.get("artifact") or {}
        severity = str(vulnerability.get("severity") or "unknown").lower()
        vulnerability_id = str(vulnerability.get("id") or "UNKNOWN")
        package_name = str(artifact.get("name") or "UNKNOWN")
        package_version = str(artifact.get("version") or "UNKNOWN")
        counts[severity] += 1

        if severity not in {"high", "critical"}:
            continue

        excepted = any(
            vulnerability_exception_matches(
                exception, vulnerability_id, package_name, package_version
            )
            for exception in exceptions
        )
        if not excepted:
            errors.append(
                f"{severity.upper()} vulnerability {vulnerability_id} affects "
                f"{package_name}@{package_version}; no exact, time-bounded exception exists."
            )
        else:
            counts["high-critical-excepted"] += 1

    return counts, errors


def build_report(
    sbom: dict[str, Any],
    grype: dict[str, Any],
    license_counts: Counter[str],
    vulnerability_counts: Counter[str],
    exception_errors: list[str],
    policy_errors: list[str],
    release_gate: bool,
) -> str:
    packages = sbom.get("components", []) or []
    descriptor = grype.get("descriptor", {}) or {}
    db = descriptor.get("db", {}) or {}
    lines = [
        "# ROCKSOUL supply-chain report",
        "",
        f"- Release gate: {'yes' if release_gate else 'no (PR/push audit)'}",
        f"- SBOM format: {sbom.get('bomFormat', 'unknown')} "
        f"{sbom.get('specVersion', 'unknown')}",
        f"- Components recorded: {len(packages)}",
        f"- Syft version requested: {os.environ.get('SYFT_VERSION', 'not recorded')}",
        f"- Grype version reported: {descriptor.get('version', os.environ.get('GRYPE_VERSION', 'not recorded'))}",
        f"- Grype database built: {db.get('built', 'not reported')}",
        f"- Grype database status: {db.get('status', 'not reported')}",
        f"- Grype database checksum: {db.get('checksum', 'not reported')}",
        "",
        "## License inventory",
        "",
    ]
    for status in ("approved", "approved-by-exception", "disallowed", "review"):
        lines.append(f"- {status}: {license_counts.get(status, 0)}")
    lines.extend(["", "## Vulnerability inventory", ""])
    for severity in ("critical", "high", "medium", "low", "negligible", "unknown"):
        lines.append(f"- {severity}: {vulnerability_counts.get(severity, 0)}")
    lines.append(
        f"- high/critical covered by exact exception: "
        f"{vulnerability_counts.get('high-critical-excepted', 0)}"
    )
    if exception_errors or policy_errors:
        lines.extend(["", "## Policy findings", ""])
        lines.extend(f"- {item}" for item in exception_errors + policy_errors)
    else:
        lines.extend(["", "## Policy findings", "", "- No blocking policy findings."])
    lines.extend(
        [
            "",
            "## Interpretation",
            "",
            "A clean report is evidence about the dependency snapshot and vulnerability "
            "database used for this run. It is not a general claim that ROCKSOUL is secure.",
            "The generated Cargo.lock and bun.lock are retained with this run's evidence.",
            "Unknown license metadata is reported for owner review on PR/push and blocks "
            "a published release unless covered by a current exact-component exception.",
        ]
    )
    return "\n".join(lines) + "\n"


def run_policy(
    sbom: dict[str, Any],
    grype: dict[str, Any],
    policy: dict[str, Any],
    today: date | None = None,
    release_gate: bool = False,
) -> tuple[str, list[str]]:
    today = today or date.today()
    exception_errors = validate_exceptions(policy, today)
    license_counts, license_errors = assess_licenses(sbom, policy)
    vulnerability_counts, vulnerability_errors = assess_vulnerabilities(grype, policy)

    if release_gate and license_counts.get("review", 0):
        license_errors.append(
            f"{license_counts['review']} component license(s) are unknown or not approved; "
            "published releases require explicit review or a current exact exception."
        )
    errors = exception_errors + license_errors + vulnerability_errors
    report = build_report(
        sbom,
        grype,
        license_counts,
        vulnerability_counts,
        exception_errors,
        license_errors + vulnerability_errors,
        release_gate,
    )
    return report, errors


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--sbom", required=True)
    parser.add_argument("--grype", required=True)
    parser.add_argument("--policy", required=True)
    parser.add_argument("--report", required=True)
    args = parser.parse_args()

    try:
        sbom = read_json(args.sbom)
        grype = read_json(args.grype)
        policy = read_json(args.policy)
        release_gate = os.environ.get("RELEASE_GATE", "false").lower() == "true"
        report, errors = run_policy(sbom, grype, policy, release_gate=release_gate)
        Path(args.report).write_text(report, encoding="utf-8")
        print(report)
        if errors:
            print("\n".join(f"::error::{error}" for error in errors), file=sys.stderr)
            return 1
        return 0
    except (OSError, ValueError, json.JSONDecodeError) as error:
        print(f"::error::Supply-chain policy could not run: {error}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
