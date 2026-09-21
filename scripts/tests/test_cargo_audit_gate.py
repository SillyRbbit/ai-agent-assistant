from __future__ import annotations

import importlib.util
import sys
import unittest
from pathlib import Path


def load_module():
    module_path = Path(__file__).resolve().parents[1] / "cargo_audit_gate.py"
    spec = importlib.util.spec_from_file_location("cargo_audit_gate", module_path)
    if spec is None or spec.loader is None:
        raise RuntimeError("cargo_audit_gate module could not be loaded")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


gate = load_module()


EXPECTED_ACCEPTED_WARNINGS = frozenset(
    {
        ("RUSTSEC-2024-0370", "proc-macro-error", "1.0.4"),
        ("RUSTSEC-2024-0429", "glib", "0.18.5"),
        ("RUSTSEC-2025-0075", "unic-char-range", "0.9.0"),
        ("RUSTSEC-2025-0080", "unic-common", "0.9.0"),
        ("RUSTSEC-2025-0081", "unic-char-property", "0.9.0"),
        ("RUSTSEC-2025-0098", "unic-ucd-version", "0.9.0"),
        ("RUSTSEC-2025-0100", "unic-ucd-ident", "0.9.0"),
        ("RUSTSEC-2026-0190", "anyhow", "1.0.102"),
    }
)

WITHDRAWN_GTK_WARNINGS = (
    ("RUSTSEC-2024-0411", "gdkwayland-sys", "0.18.2"),
    ("RUSTSEC-2024-0412", "gdk", "0.18.2"),
    ("RUSTSEC-2024-0413", "atk", "0.18.2"),
    ("RUSTSEC-2024-0414", "gdkx11-sys", "0.18.2"),
    ("RUSTSEC-2024-0415", "gtk", "0.18.2"),
    ("RUSTSEC-2024-0416", "atk-sys", "0.18.2"),
    ("RUSTSEC-2024-0417", "gdkx11", "0.18.2"),
    ("RUSTSEC-2024-0418", "gdk-sys", "0.18.2"),
    ("RUSTSEC-2024-0419", "gtk3-macros", "0.18.2"),
    ("RUSTSEC-2024-0420", "gtk-sys", "0.18.2"),
)


def entry(advisory_id: str, package: str = "quick-xml", version: str = "0.39.4"):
    return {
        "advisory": {"id": advisory_id},
        "package": {"name": package, "version": version},
    }


def report(vulnerabilities=None, warnings=None):
    return {
        "vulnerabilities": {"list": [] if vulnerabilities is None else vulnerabilities},
        "warnings": {} if warnings is None else warnings,
    }


def accepted_warnings():
    return [
        entry(advisory_id, package, version)
        for advisory_id, package, version in sorted(gate.ACCEPTED_WARNINGS)
    ]


class CargoAuditGateTests(unittest.TestCase):
    def test_accepted_warning_set_is_exact(self) -> None:
        self.assertEqual(gate.ACCEPTED_WARNINGS, EXPECTED_ACCEPTED_WARNINGS)

    def test_accepts_clean_success(self) -> None:
        self.assertEqual(gate.validate_report(report(), 0), ())

    def test_accepts_exact_reviewed_baseline(self) -> None:
        payload = report(
            [entry("RUSTSEC-2026-0194"), entry("RUSTSEC-2026-0195")],
            {"informational": accepted_warnings()},
        )

        self.assertEqual(gate.validate_report(payload, 1), ())

    def test_rejects_new_vulnerability(self) -> None:
        payload = report(
            [
                entry("RUSTSEC-2026-0194"),
                entry("RUSTSEC-2026-0195"),
                entry("RUSTSEC-2099-0001", "new-package", "1.2.3"),
            ]
        )

        findings = gate.validate_report(payload, 1)

        self.assertTrue(any("RUSTSEC-2099-0001" in finding for finding in findings))

    def test_rejects_changed_baseline_package_version(self) -> None:
        payload = report(
            [entry("RUSTSEC-2026-0194", version="0.40.0"), entry("RUSTSEC-2026-0195")]
        )

        findings = gate.validate_report(payload, 1)

        self.assertTrue(any("0.40.0" in finding for finding in findings))
        self.assertTrue(any("baseline changed" in finding for finding in findings))

    def test_rejects_new_warning(self) -> None:
        payload = report(
            warnings={
                "informational": accepted_warnings()
                + [entry("RUSTSEC-2099-0002", "new-package", "1.2.3")]
            }
        )

        findings = gate.validate_report(payload, 0)

        self.assertTrue(any("RUSTSEC-2099-0002" in finding for finding in findings))

    def test_rejects_each_withdrawn_gtk_warning_if_it_reappears(self) -> None:
        for advisory_id, package, version in WITHDRAWN_GTK_WARNINGS:
            with self.subTest(advisory_id=advisory_id):
                payload = report(
                    warnings={
                        "informational": accepted_warnings()
                        + [entry(advisory_id, package, version)]
                    }
                )

                findings = gate.validate_report(payload, 0)

                self.assertTrue(
                    any(
                        f"unexpected warning {advisory_id} in {package} {version}"
                        in finding
                        for finding in findings
                    )
                )

    def test_rejects_changed_warning_package_version(self) -> None:
        warnings = accepted_warnings()
        warnings[0] = entry("RUSTSEC-2024-0370", "proc-macro-error", "2.0.0")
        payload = report(warnings={"informational": warnings})

        findings = gate.validate_report(payload, 0)

        self.assertTrue(any("2.0.0" in finding for finding in findings))
        self.assertTrue(any("warning baseline changed" in finding for finding in findings))

    def test_rejects_tool_failure(self) -> None:
        payload = report()

        findings = gate.validate_report(payload, 2)

        self.assertTrue(any("unexpected status" in finding for finding in findings))

    def test_rejects_malformed_report(self) -> None:
        findings = gate.validate_report({"vulnerabilities": {"list": "invalid"}}, 0)

        self.assertTrue(any("invalid cargo-audit report" in finding for finding in findings))


if __name__ == "__main__":
    unittest.main()
