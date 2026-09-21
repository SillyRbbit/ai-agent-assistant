#!/usr/bin/env python3
"""Validate cargo-audit JSON against Cortexa's exact accepted baseline."""

from __future__ import annotations

import argparse
import json
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Mapping, Sequence

ACCEPTED_VULNERABILITIES = frozenset(
    {
        ("RUSTSEC-2026-0194", "quick-xml", "0.39.4"),
        ("RUSTSEC-2026-0195", "quick-xml", "0.39.4"),
    }
)

ACCEPTED_WARNINGS = frozenset(
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


@dataclass(frozen=True, order=True)
class Advisory:
    advisory_id: str
    package: str
    version: str


def require_mapping(value: Any, label: str) -> Mapping[str, Any]:
    if not isinstance(value, dict):
        raise ValueError(f"{label} must be an object")
    return value


def require_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise ValueError(f"{label} must be an array")
    return value


def advisory_from_entry(entry: Any, label: str) -> Advisory:
    item = require_mapping(entry, label)
    advisory = require_mapping(item.get("advisory"), f"{label}.advisory")
    package = require_mapping(item.get("package"), f"{label}.package")
    advisory_id = advisory.get("id")
    package_name = package.get("name")
    version = package.get("version")
    if not all(isinstance(value, str) and value for value in (advisory_id, package_name, version)):
        raise ValueError(f"{label} identity fields must be non-empty strings")
    return Advisory(advisory_id, package_name, version)


def parse_report(report: Any) -> tuple[frozenset[Advisory], frozenset[Advisory]]:
    root = require_mapping(report, "report")
    vulnerabilities = require_mapping(root.get("vulnerabilities"), "vulnerabilities")
    vulnerability_entries = require_list(vulnerabilities.get("list"), "vulnerabilities.list")
    parsed_vulnerabilities = frozenset(
        advisory_from_entry(entry, f"vulnerabilities.list[{index}]")
        for index, entry in enumerate(vulnerability_entries)
    )

    warning_entries: list[Advisory] = []
    warnings = require_mapping(root.get("warnings", {}), "warnings")
    for category, entries in warnings.items():
        if not isinstance(category, str):
            raise ValueError("warning categories must be strings")
        for index, entry in enumerate(require_list(entries, f"warnings.{category}")):
            warning_entries.append(advisory_from_entry(entry, f"warnings.{category}[{index}]"))
    return parsed_vulnerabilities, frozenset(warning_entries)


def validate_report(report: Any, cargo_audit_exit: int) -> tuple[str, ...]:
    if cargo_audit_exit not in (0, 1):
        return (f"cargo-audit exited with unexpected status {cargo_audit_exit}",)
    try:
        vulnerabilities, warnings = parse_report(report)
    except ValueError as error:
        return (f"invalid cargo-audit report: {error}",)

    accepted_vulnerabilities = frozenset(
        Advisory(*entry) for entry in ACCEPTED_VULNERABILITIES
    )
    accepted_warnings = frozenset(Advisory(*entry) for entry in ACCEPTED_WARNINGS)
    findings: list[str] = []
    unexpected_vulnerabilities = vulnerabilities - accepted_vulnerabilities
    missing_vulnerabilities = accepted_vulnerabilities - vulnerabilities
    unexpected_warnings = warnings - accepted_warnings
    missing_warnings = accepted_warnings - warnings
    if unexpected_vulnerabilities:
        findings.extend(
            f"unexpected vulnerability {item.advisory_id} in {item.package} {item.version}"
            for item in sorted(unexpected_vulnerabilities)
        )
    if unexpected_warnings:
        findings.extend(
            f"unexpected warning {item.advisory_id} in {item.package} {item.version}"
            for item in sorted(unexpected_warnings)
        )
    if cargo_audit_exit == 0 and vulnerabilities:
        findings.append("cargo-audit returned success while reporting vulnerabilities")
    if cargo_audit_exit == 1 and not vulnerabilities:
        findings.append("cargo-audit returned a finding status without vulnerabilities")
    if vulnerabilities and missing_vulnerabilities:
        findings.extend(
            f"accepted baseline changed: missing {item.advisory_id} for {item.package} {item.version}"
            for item in sorted(missing_vulnerabilities)
        )
    if warnings and missing_warnings:
        findings.extend(
            f"accepted warning baseline changed: missing {item.advisory_id} for {item.package} {item.version}"
            for item in sorted(missing_warnings)
        )
    return tuple(findings)


def parse_arguments(arguments: Sequence[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("report", type=Path)
    parser.add_argument("--cargo-audit-exit", required=True, type=int)
    return parser.parse_args(arguments)


def main(arguments: Sequence[str] | None = None) -> int:
    parsed = parse_arguments(sys.argv[1:] if arguments is None else arguments)
    try:
        report = json.loads(parsed.report.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        print(f"cargo-audit-gate: FAIL: report could not be read: {error}", file=sys.stderr)
        return 2
    findings = validate_report(report, parsed.cargo_audit_exit)
    if findings:
        for finding in findings:
            print(f"cargo-audit-gate: {finding}", file=sys.stderr)
        return 1
    print("cargo-audit-gate: PASS (only exact accepted advisory baseline present)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
