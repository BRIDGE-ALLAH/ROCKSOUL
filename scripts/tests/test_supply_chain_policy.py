import sys
import unittest
from datetime import date
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from supply_chain_policy import run_policy


BASE_POLICY = {
    "approved_spdx": ["MIT", "Apache-2.0", "BSD-3-Clause"],
    "disallowed_exact": ["BUSL-1.1"],
    "disallowed_prefixes": ["GPL-", "AGPL-", "SSPL-"],
    "license_exceptions": [],
    "vulnerability_exceptions": [],
}


def sbom_with_license(license_id):
    licenses = [] if license_id is None else [{"license": {"id": license_id}}]
    return {
        "bomFormat": "CycloneDX",
        "specVersion": "1.6",
        "components": [
            {"name": "demo-package", "version": "1.2.3", "licenses": licenses}
        ],
    }


def grype_with_match(severity="Low", vulnerability_id="CVE-2026-0001"):
    return {
        "descriptor": {
            "version": "v0.118.0",
            "db": {"built": "2026-11-10T00:00:00Z", "status": "valid"},
        },
        "matches": [
            {
                "vulnerability": {"id": vulnerability_id, "severity": severity},
                "artifact": {"name": "demo-package", "version": "1.2.3"},
            }
        ],
    }


class SupplyChainPolicyTests(unittest.TestCase):
    today = date(2026, 11, 11)

    def test_approved_license_and_low_vulnerability_pass(self):
        _, errors = run_policy(
            sbom_with_license("MIT"),
            grype_with_match("Low"),
            BASE_POLICY,
            today=self.today,
        )
        self.assertEqual(errors, [])

    def test_disallowed_copyleft_license_fails_on_pr(self):
        _, errors = run_policy(
            sbom_with_license("GPL-3.0-only"),
            {"matches": []},
            BASE_POLICY,
            today=self.today,
        )
        self.assertTrue(any("Disallowed license" in error for error in errors))

    def test_unknown_license_is_review_on_pr_and_blocked_on_release(self):
        sbom = sbom_with_license(None)
        _, pr_errors = run_policy(
            sbom, {"matches": []}, BASE_POLICY, today=self.today, release_gate=False
        )
        _, release_errors = run_policy(
            sbom, {"matches": []}, BASE_POLICY, today=self.today, release_gate=True
        )
        self.assertEqual(pr_errors, [])
        self.assertTrue(any("unknown" in error for error in release_errors))

    def test_exact_license_exception_requires_owner_rationale_and_expiry(self):
        policy = dict(BASE_POLICY)
        policy["license_exceptions"] = [
            {
                "component": "demo-package",
                "version": "1.2.3",
                "license": "NOASSERTION",
                "rationale": "Upstream has not published an SPDX identifier.",
                "owner": "maintainer",
                "expires": "2026-12-01",
            }
        ]
        _, errors = run_policy(
            sbom_with_license(None),
            {"matches": []},
            policy,
            today=self.today,
            release_gate=True,
        )
        self.assertEqual(errors, [])

    def test_expired_exception_fails_even_if_finding_disappears(self):
        policy = dict(BASE_POLICY)
        policy["vulnerability_exceptions"] = [
            {
                "vulnerability_id": "CVE-2026-0001",
                "package": "demo-package",
                "version": "1.2.3",
                "rationale": "Temporary mitigation while upstream fix is prepared.",
                "owner": "maintainer",
                "expires": "2026-11-10",
            }
        ]
        _, errors = run_policy(
            sbom_with_license("MIT"),
            {"matches": []},
            policy,
            today=self.today,
        )
        self.assertTrue(any("expired" in error for error in errors))

    def test_high_vulnerability_blocks_without_exception(self):
        _, errors = run_policy(
            sbom_with_license("MIT"),
            grype_with_match("High"),
            BASE_POLICY,
            today=self.today,
        )
        self.assertTrue(any("HIGH vulnerability" in error for error in errors))

    def test_critical_vulnerability_requires_exact_exception(self):
        policy = dict(BASE_POLICY)
        policy["vulnerability_exceptions"] = [
            {
                "vulnerability_id": "CVE-2026-0001",
                "package": "demo-package",
                "version": "1.2.3",
                "rationale": "Temporary mitigation with a tracked upstream fix.",
                "owner": "security-reviewer",
                "expires": "2026-12-01",
            }
        ]
        _, errors = run_policy(
            sbom_with_license("MIT"),
            grype_with_match("Critical"),
            policy,
            today=self.today,
        )
        self.assertEqual(errors, [])

    def test_exception_for_different_package_does_not_suppress_finding(self):
        policy = dict(BASE_POLICY)
        policy["vulnerability_exceptions"] = [
            {
                "vulnerability_id": "CVE-2026-0001",
                "package": "another-package",
                "version": "1.2.3",
                "rationale": "The exception belongs to another dependency.",
                "owner": "security-reviewer",
                "expires": "2026-12-01",
            }
        ]
        _, errors = run_policy(
            sbom_with_license("MIT"),
            grype_with_match("High"),
            policy,
            today=self.today,
        )
        self.assertTrue(any("HIGH vulnerability" in error for error in errors))


if __name__ == "__main__":
    unittest.main()
