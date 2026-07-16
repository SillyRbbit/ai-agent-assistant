#!/usr/bin/env python3
"""Validate and enforce Cortexa's repository-local post-increment gate."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import stat
import sys
import tempfile
from dataclasses import dataclass
from pathlib import Path
from typing import Any, TextIO

from common import (
    MAX_HOOK_INPUT_BYTES,
    ExitCode,
    GateError,
    changed_paths,
    decode_git_paths as _decode_git_paths,
    fail as _fail,
    has_merge_conflicts,
    read_bounded_text as _read_bounded_text,
    read_hook_payload,
    repository_root,
    run_git as _run_git,
    suspicious_changed_paths,
    validate_relative_path,
    workspace_path as _workspace_path,
)

SCHEMA_VERSION = 1
STATE_RELATIVE_PATH = Path(".codex/state/post_increment_gate.json")
REPORTS_RELATIVE_PATH = Path("docs/reviews")
COMPLETION_MARKER = "POST_INCREMENT_GATE_COMPLETE"
CONTINUATION_PROMPT = (
    "Run $post-increment-gate for the active increment before ending the session."
)
MAX_REPORT_BYTES = 1024 * 1024
MAX_COMMAND_CHARACTERS = 4096
MANIFEST_START = "<!-- post-increment-gate-manifest\n"
MANIFEST_END = "\n-->"

INCREMENT_ID_PATTERN = re.compile(r"^[a-z0-9]+(?:-[a-z0-9]+)*$")
REPORT_DATE_PATTERN = r"\d{4}-\d{2}-\d{2}"

VERIFICATION_STATUSES = frozenset(
    {"Passed", "Failed", "Not run", "Manual verification pending"}
)
QUALITY_RESULTS = frozenset({"PASS", "PASS WITH ADVISORIES", "FAIL"})
READINESS_RESULTS = frozenset({"Ready", "Ready with advisories", "Blocked"})
SEVERITIES = frozenset({"Critical", "High", "Medium", "Low", "Advisory"})
FINDING_CATEGORIES = frozenset(
    {"Architecture", "Security", "Code health", "Technical debt", "Roadmap"}
)

REQUIRED_REPORT_SECTIONS = (
    "## Executive summary",
    "## Verification results",
    "## Architecture findings",
    "## Security findings",
    "## Code-health findings",
    "## Technical debt",
    "## Roadmap findings",
    "## Completion decision",
    "## Next-increment readiness",
    "## Exact files changed",
    "## Exact commands executed",
)


@dataclass(frozen=True)
class StopDecision:
    should_continue: bool


def _hash_file(path: Path) -> bytes:
    digest = hashlib.sha256()
    try:
        with path.open("rb") as handle:
            while chunk := handle.read(64 * 1024):
                digest.update(chunk)
    except OSError as error:
        raise GateError(
            "repository file could not be read", ExitCode.IO_ERROR
        ) from error
    return digest.digest()


def workspace_fingerprint(root: Path) -> str:
    repository_paths = _decode_git_paths(
        _run_git(
            root,
            "ls-files",
            "--cached",
            "--others",
            "--exclude-standard",
            "-z",
        )
    )
    digest = hashlib.sha256()
    for relative_path in sorted(set(repository_paths)):
        path = _workspace_path(root, relative_path)
        try:
            file_status = path.lstat()
        except FileNotFoundError:
            # Tracked deletions remain cached until commit but are absent afterward.
            continue
        except OSError as error:
            raise GateError(
                "repository file metadata could not be read", ExitCode.IO_ERROR
            ) from error

        encoded_path = relative_path.encode("utf-8")
        digest.update(len(encoded_path).to_bytes(8, "big"))
        digest.update(encoded_path)
        digest.update((file_status.st_mode & 0o111).to_bytes(2, "big"))
        if stat.S_ISLNK(file_status.st_mode):
            try:
                target = os.readlink(path).encode("utf-8")
            except (OSError, UnicodeEncodeError) as error:
                raise GateError("repository symlink could not be read") from error
            digest.update(b"symlink")
            digest.update(hashlib.sha256(target).digest())
        elif stat.S_ISREG(file_status.st_mode):
            digest.update(b"file")
            digest.update(_hash_file(path))
        elif stat.S_ISDIR(file_status.st_mode):
            digest.update(b"directory")
        else:
            _fail("repository contains an unsupported file type")
    return digest.hexdigest()


def state_path(root: Path) -> Path:
    return root / STATE_RELATIVE_PATH


def _require_exact_keys(
    value: dict[str, Any], expected: frozenset[str], label: str
) -> None:
    if frozenset(value) != expected:
        _fail(f"{label} has unexpected or missing fields")


def read_state(root: Path) -> dict[str, Any] | None:
    path = state_path(root)
    if not path.exists():
        return None
    if path.is_symlink():
        _fail("post-increment state path must not be a symlink")
    try:
        resolved_path = path.resolve(strict=True)
        resolved_path.relative_to(root)
    except (OSError, ValueError) as error:
        raise GateError("post-increment state path is unsafe") from error
    if not resolved_path.is_file():
        _fail("post-increment state is not a regular file")
    text = _read_bounded_text(
        resolved_path, MAX_HOOK_INPUT_BYTES, "post-increment state"
    )
    try:
        value = json.loads(text)
    except json.JSONDecodeError as error:
        raise GateError("post-increment state is not valid JSON") from error
    if not isinstance(value, dict):
        _fail("post-increment state must be a JSON object")
    validate_state(value)
    return value


def validate_state(state_value: dict[str, Any]) -> None:
    status_value = state_value.get("status")
    common_keys = {
        "baseline_fingerprint",
        "increment_id",
        "schema_version",
        "status",
    }
    if status_value == "active":
        _require_exact_keys(state_value, frozenset(common_keys), "active gate state")
    elif status_value == "complete":
        _require_exact_keys(
            state_value,
            frozenset(
                common_keys
                | {
                    "completion_marker",
                    "quality_gate",
                    "report_path",
                    "report_sha256",
                    "workspace_fingerprint",
                }
            ),
            "completed gate state",
        )
        if state_value.get("completion_marker") != COMPLETION_MARKER:
            _fail("completed gate state has an invalid completion marker")
        if state_value.get("quality_gate") not in {
            "PASS",
            "PASS WITH ADVISORIES",
        }:
            _fail("completed gate state has a non-passing result")
        for key in ("report_path", "report_sha256", "workspace_fingerprint"):
            if not isinstance(state_value.get(key), str) or not state_value[key]:
                _fail("completed gate state has invalid evidence")
    else:
        _fail("post-increment state has an unknown status")

    if state_value.get("schema_version") != SCHEMA_VERSION:
        _fail("post-increment state schema version is unsupported")
    validate_increment_id(state_value.get("increment_id"))
    baseline = state_value.get("baseline_fingerprint")
    if not isinstance(baseline, str) or not re.fullmatch(r"[0-9a-f]{64}", baseline):
        _fail("post-increment state has an invalid baseline fingerprint")


def write_state(root: Path, state_value: dict[str, Any]) -> None:
    validate_state(state_value)
    path = state_path(root)
    parent = path.parent
    try:
        parent.mkdir(parents=True, exist_ok=True)
        resolved_parent = parent.resolve(strict=True)
        resolved_parent.relative_to(root)
    except (OSError, ValueError) as error:
        raise GateError(
            "post-increment state directory is unsafe", ExitCode.IO_ERROR
        ) from error

    payload = json.dumps(state_value, indent=2, sort_keys=True) + "\n"
    temporary_path: Path | None = None
    try:
        with tempfile.NamedTemporaryFile(
            "w",
            encoding="utf-8",
            dir=resolved_parent,
            prefix=".post_increment_gate.",
            delete=False,
        ) as handle:
            temporary_path = Path(handle.name)
            handle.write(payload)
            handle.flush()
            os.fsync(handle.fileno())
        temporary_path.chmod(0o600)
        os.replace(temporary_path, path)
    except OSError as error:
        if temporary_path is not None:
            try:
                temporary_path.unlink(missing_ok=True)
            except OSError:
                pass
        raise GateError(
            "post-increment state could not be written", ExitCode.IO_ERROR
        ) from error


def validate_increment_id(value: Any) -> str:
    if (
        not isinstance(value, str)
        or len(value) > 64
        or INCREMENT_ID_PATTERN.fullmatch(value) is None
    ):
        _fail("increment id must be a lowercase kebab-case identifier")
    return value


def begin_gate(root: Path, increment_id: str) -> None:
    increment_id = validate_increment_id(increment_id)
    if has_merge_conflicts(root):
        _fail("cannot begin an increment while merge conflicts exist")
    existing_state = read_state(root)
    if existing_state is not None and existing_state["status"] == "active":
        if existing_state["increment_id"] == increment_id:
            return
        _fail("another increment is already active")
    if (
        existing_state is not None
        and existing_state["status"] == "complete"
        and existing_state["increment_id"] == increment_id
        and validate_completed_state(root, existing_state)
    ):
        _fail("this increment already has a valid completion marker")

    write_state(
        root,
        {
            "baseline_fingerprint": workspace_fingerprint(root),
            "increment_id": increment_id,
            "schema_version": SCHEMA_VERSION,
            "status": "active",
        },
    )


def _safe_report_path(root: Path, report_value: str, increment_id: str) -> Path:
    validate_relative_path(report_value)
    relative_path = Path(report_value)
    expected_name = re.compile(
        rf"^{REPORT_DATE_PATTERN}-{re.escape(increment_id)}-post-increment-review\.md$"
    )
    if relative_path.parent != REPORTS_RELATIVE_PATH:
        _fail("post-increment report must be directly under docs/reviews")
    if expected_name.fullmatch(relative_path.name) is None:
        _fail("post-increment report filename does not match the increment")
    path = root / relative_path
    if path.is_symlink():
        _fail("post-increment report must not be a symlink")
    try:
        resolved_report = path.resolve(strict=True)
        resolved_reports_root = (root / REPORTS_RELATIVE_PATH).resolve(strict=True)
        resolved_reports_root.relative_to(root)
        resolved_report.relative_to(resolved_reports_root)
    except (OSError, ValueError) as error:
        raise GateError("post-increment report path is unsafe") from error
    if not resolved_report.is_file():
        _fail("post-increment report is not a regular file")
    return resolved_report


def _string_list(value: Any, label: str) -> list[str]:
    if not isinstance(value, list) or not value:
        _fail(f"{label} must be a non-empty list")
    strings: list[str] = []
    for item in value:
        if (
            not isinstance(item, str)
            or not item.strip()
            or len(item) > MAX_COMMAND_CHARACTERS
        ):
            _fail(f"{label} contains an invalid entry")
        strings.append(item)
    if len(strings) != len(set(strings)):
        _fail(f"{label} contains duplicate entries")
    return strings


def _validate_verification_entries(
    value: Any, label: str, name_key: str
) -> list[dict[str, Any]]:
    if not isinstance(value, list):
        _fail(f"{label} must be a list")
    entries: list[dict[str, Any]] = []
    for entry in value:
        if not isinstance(entry, dict):
            _fail(f"{label} contains a non-object entry")
        _require_exact_keys(
            entry, frozenset({name_key, "required", "status"}), f"{label} entry"
        )
        name = entry[name_key]
        if (
            not isinstance(name, str)
            or not name.strip()
            or len(name) > MAX_COMMAND_CHARACTERS
        ):
            _fail(f"{label} contains an invalid name")
        if not isinstance(entry["required"], bool):
            _fail(f"{label} contains an invalid required flag")
        if entry["status"] not in VERIFICATION_STATUSES:
            _fail(f"{label} contains an invalid status")
        entries.append(entry)
    return entries


def _validate_findings(value: Any) -> list[dict[str, Any]]:
    if not isinstance(value, list):
        _fail("findings must be a list")
    findings: list[dict[str, Any]] = []
    expected_keys = frozenset(
        {
            "blocks_completion",
            "blocks_next_increment",
            "category",
            "effort",
            "milestone",
            "risk",
            "severity",
            "summary",
        }
    )
    for finding in value:
        if not isinstance(finding, dict):
            _fail("findings contains a non-object entry")
        _require_exact_keys(finding, expected_keys, "finding")
        if finding["category"] not in FINDING_CATEGORIES:
            _fail("finding has an invalid category")
        if finding["severity"] not in SEVERITIES:
            _fail("finding has an invalid severity")
        if not isinstance(finding["blocks_completion"], bool) or not isinstance(
            finding["blocks_next_increment"], bool
        ):
            _fail("finding has an invalid blocking flag")
        for key in ("effort", "milestone", "risk", "summary"):
            if not isinstance(finding[key], str) or not finding[key].strip():
                _fail("finding has incomplete debt metadata")
        findings.append(finding)
    return findings


def _extract_manifest(report_text: str) -> dict[str, Any]:
    if report_text.count(MANIFEST_START) != 1:
        _fail("post-increment report must contain exactly one machine manifest")
    start = report_text.index(MANIFEST_START) + len(MANIFEST_START)
    end = report_text.find(MANIFEST_END, start)
    if end < 0:
        _fail("post-increment report machine manifest is unterminated")
    try:
        manifest = json.loads(report_text[start:end])
    except json.JSONDecodeError as error:
        raise GateError("post-increment report manifest is not valid JSON") from error
    if not isinstance(manifest, dict):
        _fail("post-increment report manifest must be a JSON object")
    return manifest


def validate_report(
    root: Path, report_value: str, increment_id: str
) -> tuple[dict[str, Any], Path, str]:
    report_path = _safe_report_path(root, report_value, increment_id)
    report_text = _read_bounded_text(
        report_path, MAX_REPORT_BYTES, "post-increment report"
    )
    for section in REQUIRED_REPORT_SECTIONS:
        if report_text.count(section) != 1:
            _fail(f"post-increment report must contain exactly one {section} section")

    manifest = _extract_manifest(report_text)
    _require_exact_keys(
        manifest,
        frozenset(
            {
                "commands_executed",
                "files_changed",
                "findings",
                "increment_id",
                "manual_verification",
                "next_increment_readiness",
                "quality_gate",
                "schema_version",
                "verification",
            }
        ),
        "post-increment report manifest",
    )
    if manifest["schema_version"] != SCHEMA_VERSION:
        _fail("post-increment report schema version is unsupported")
    if manifest["increment_id"] != increment_id:
        _fail("post-increment report increment does not match active state")
    if manifest["quality_gate"] not in QUALITY_RESULTS:
        _fail("post-increment report has an invalid quality-gate result")
    if manifest["next_increment_readiness"] not in READINESS_RESULTS:
        _fail("post-increment report has an invalid readiness result")

    verification = _validate_verification_entries(
        manifest["verification"], "verification", "command"
    )
    if not verification:
        _fail("post-increment report must record verification commands")
    manual_verification = _validate_verification_entries(
        manifest["manual_verification"], "manual verification", "check"
    )
    findings = _validate_findings(manifest["findings"])
    files_changed = _string_list(manifest["files_changed"], "files changed")
    commands_executed = _string_list(
        manifest["commands_executed"], "commands executed"
    )

    normalized_files: list[str] = []
    for relative_path in files_changed:
        validate_relative_path(relative_path)
        normalized_files.append(Path(relative_path).as_posix())
    if len(normalized_files) != len(set(normalized_files)):
        _fail("files changed contains duplicate normalized paths")
    actual_changed_paths = set(changed_paths(root))
    if set(normalized_files) != actual_changed_paths:
        _fail("report file inventory does not match the complete Git change set")

    for entry in verification:
        if entry["command"] not in commands_executed:
            _fail("verification command is missing from commands executed")

    blocking_verification = any(
        entry["required"] and entry["status"] != "Passed"
        for entry in verification
    )
    blocking_manual = any(
        entry["required"] and entry["status"] != "Passed"
        for entry in manual_verification
    )
    blocking_finding = any(
        finding["blocks_completion"]
        and finding["severity"] in {"Critical", "High"}
        for finding in findings
    )
    if blocking_verification or blocking_manual or blocking_finding:
        computed_quality = "FAIL"
    else:
        has_advisory = (
            bool(findings)
            or any(entry["status"] != "Passed" for entry in verification)
            or any(entry["status"] != "Passed" for entry in manual_verification)
            or manifest["next_increment_readiness"] != "Ready"
        )
        computed_quality = "PASS WITH ADVISORIES" if has_advisory else "PASS"
    if manifest["quality_gate"] != computed_quality:
        _fail("declared quality-gate result does not match report evidence")
    if computed_quality == "FAIL":
        _fail("post-increment report contains blocking evidence")

    report_digest = hashlib.sha256(report_text.encode("utf-8")).hexdigest()
    return manifest, report_path, report_digest


def finalize_gate(root: Path, increment_id: str, report_value: str) -> None:
    increment_id = validate_increment_id(increment_id)
    if has_merge_conflicts(root):
        _fail("cannot complete an increment while merge conflicts exist")
    state_value = read_state(root)
    if state_value is None:
        _fail("no post-increment gate state exists")
    if state_value["increment_id"] != increment_id:
        _fail("increment does not match finalize request")

    suspicious = suspicious_changed_paths(changed_paths(root))
    if suspicious:
        _fail("suspicious generated, credential, database, or build path detected")

    manifest, report_path, report_digest = validate_report(
        root, report_value, increment_id
    )
    relative_report = report_path.relative_to(root).as_posix()
    write_state(
        root,
        {
            "baseline_fingerprint": state_value["baseline_fingerprint"],
            "completion_marker": COMPLETION_MARKER,
            "increment_id": increment_id,
            "quality_gate": manifest["quality_gate"],
            "report_path": relative_report,
            "report_sha256": report_digest,
            "schema_version": SCHEMA_VERSION,
            "status": "complete",
            "workspace_fingerprint": workspace_fingerprint(root),
        },
    )


def validate_completed_state(root: Path, state_value: dict[str, Any]) -> bool:
    try:
        validate_state(state_value)
        if has_merge_conflicts(root):
            return False
        report_path = _safe_report_path(
            root, state_value["report_path"], state_value["increment_id"]
        )
        report_text = _read_bounded_text(
            report_path, MAX_REPORT_BYTES, "post-increment report"
        )
        report_digest = hashlib.sha256(report_text.encode("utf-8")).hexdigest()
        if report_digest != state_value["report_sha256"]:
            return False
        if workspace_fingerprint(root) != state_value["workspace_fingerprint"]:
            return False
        if suspicious_changed_paths(changed_paths(root)):
            return False
    except GateError:
        return False
    return True


def evaluate_stop_payload(payload: Any) -> StopDecision:
    if not isinstance(payload, dict):
        return StopDecision(should_continue=True)
    if payload.get("hook_event_name") != "Stop":
        return StopDecision(should_continue=True)
    if not isinstance(payload.get("stop_hook_active"), bool):
        return StopDecision(should_continue=True)
    if payload["stop_hook_active"]:
        return StopDecision(should_continue=False)
    cwd_value = payload.get("cwd")
    if not isinstance(cwd_value, str) or not cwd_value:
        return StopDecision(should_continue=True)

    try:
        root = repository_root(Path(cwd_value))
        if has_merge_conflicts(root):
            return StopDecision(should_continue=True)
        state_value = read_state(root)
        if state_value is None:
            return StopDecision(should_continue=bool(changed_paths(root)))
        if state_value["status"] == "active":
            return StopDecision(should_continue=True)
        return StopDecision(
            should_continue=not validate_completed_state(root, state_value)
        )
    except GateError:
        return StopDecision(should_continue=True)


def run_stop_hook(input_stream: Any, output_stream: TextIO) -> ExitCode:
    decision = evaluate_stop_payload(read_hook_payload(input_stream))
    if decision.should_continue:
        json.dump(
            {"decision": "block", "reason": CONTINUATION_PROMPT},
            output_stream,
            sort_keys=True,
        )
        output_stream.write("\n")
    return ExitCode.OK


def redacted_status(root: Path) -> dict[str, Any]:
    state_value = read_state(root)
    if state_value is None:
        return {"status": "inactive"}
    status_value: dict[str, Any] = {
        "increment_id": state_value["increment_id"],
        "status": state_value["status"],
    }
    if state_value["status"] == "complete":
        status_value["quality_gate"] = state_value["quality_gate"]
        status_value["report_path"] = state_value["report_path"]
        status_value["valid"] = validate_completed_state(root, state_value)
    return status_value


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=__doc__)
    subparsers = parser.add_subparsers(dest="command", required=True)

    subparsers.add_parser("stop", help="process one Codex Stop-hook payload")

    begin_parser = subparsers.add_parser(
        "begin", help="record one active implementation increment"
    )
    begin_parser.add_argument("--increment", required=True)

    finalize_parser = subparsers.add_parser(
        "finalize", help="validate a report and write the completion marker"
    )
    finalize_parser.add_argument("--increment", required=True)
    finalize_parser.add_argument("--report", required=True)

    subparsers.add_parser("status", help="print redacted gate state")
    return parser


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    arguments = parser.parse_args(argv)
    if arguments.command == "stop":
        return int(run_stop_hook(sys.stdin.buffer, sys.stdout))

    try:
        root = repository_root(Path.cwd())
        if arguments.command == "begin":
            begin_gate(root, arguments.increment)
            print(f"post-increment-gate: active increment {arguments.increment}")
        elif arguments.command == "finalize":
            finalize_gate(root, arguments.increment, arguments.report)
            print(f"post-increment-gate: completed increment {arguments.increment}")
        elif arguments.command == "status":
            json.dump(redacted_status(root), sys.stdout, sort_keys=True)
            sys.stdout.write("\n")
        else:
            parser.error("unknown command")
    except GateError as error:
        print(f"post-increment-gate: {error}", file=sys.stderr)
        return int(error.exit_code)
    return int(ExitCode.OK)


if __name__ == "__main__":
    raise SystemExit(main())
