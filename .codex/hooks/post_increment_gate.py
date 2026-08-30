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

REPORT_SCHEMA_VERSION = 1
STATE_SCHEMA_VERSION = 3
LEGACY_STATE_SCHEMA_VERSION = 1
LEGACY_FAILED_STATE_SCHEMA_VERSION = 2
# Retained for report-fixture and external read-only validator compatibility.
SCHEMA_VERSION = REPORT_SCHEMA_VERSION
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
    "## Scope and boundaries",
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

# D-098 is a deliberately source-bound, one-time governance recovery. These
# constants are not a general failed-gate override surface: callers cannot
# select any of these identities, reports, paths, or outcomes.
FAILED_DISPOSITION_INCREMENT_ID = "v0-xcode-developer-id-recovery-execution"
FAILED_DISPOSITION_PREDECESSOR_REPORT_PATH = (
    "docs/reviews/"
    "2026-08-28-v0-xcode-developer-id-recovery-execution-post-increment-review.md"
)
FAILED_DISPOSITION_PREDECESSOR_REPORT_SHA256 = (
    "712df03ac11cff026be4badad2c8d22a023377f40c99981b90937c95ae165087"
)
FAILED_DISPOSITION_PREDECESSOR_STATE_SHA256 = (
    "dfa11a0784c10edffab71e0ae1859db397d47ec035801f69a249bdded8ff4dc3"
)
FAILED_DISPOSITION_PREDECESSOR_HEAD_COMMIT = (
    "0931df66c389bdc13c705d1259706c4d3770761c"
)
FAILED_DISPOSITION_BASE_COMMIT = (
    "a417e5f1c1c602b917ca27c65af71480e3db6a45"
)
FAILED_DISPOSITION_RECOVERY_ID = (
    "v0-terminal-failed-successor-disposition-recovery"
)
FAILED_DISPOSITION_RECOVERY_REPORT_PATH = (
    "docs/reviews/2026-08-29-v0-terminal-failed-successor-disposition-recovery-"
    "post-increment-review.md"
)
FAILED_DISPOSITION_SUCCESSOR_INCREMENT_ID = (
    "personal-assistant-v0-signing-security-prerequisite-planning"
)
FAILED_DISPOSITION_BLOCKING_FINDING_SHA256 = frozenset(
    {
        "68947bd7d76fcc7cca2e1d2c03e1756ca62b80779ee3d6cb5faaa3ba3a6f406d",
        "06b43d394531f9abf3b65f03d1b2b79d9b47056b1a5c4aa5663b8bd4d24ff318",
        "0574f237e81e652e5bca224046f99cf73667ae89469e360c264fd7a9052ec86e",
    }
)
FAILED_DISPOSITION_RECOVERY_ALLOWED_PATHS = (
    ".agents/skills/post-increment-gate/SKILL.md",
    ".agents/skills/verified-increment/SKILL.md",
    ".codex/hooks/post_increment_gate.py",
    ".codex/hooks/tests/test_post_increment_gate.py",
    "AGENTS.md",
    "ARCHITECTURE.md",
    "CHANGELOG.md",
    "CODE_REVIEW.md",
    "DECISIONS.md",
    "ENGINEERING_GUIDE.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "ROADMAP.md",
    "SECURITY.md",
    "TESTING_GUIDE.md",
    "TROUBLESHOOTING_LOG.md",
    "docs/increments/v0-terminal-failed-successor-disposition-recovery.md",
    "docs/plans/2026-08-29-v0-terminal-failed-successor-disposition-recovery.md",
    "docs/reviews/2026-08-29-v0-terminal-failed-successor-disposition-recovery-post-increment-review.md",
    "docs/templates/POST_INCREMENT_REVIEW_TEMPLATE.md",
)
FAILED_DISPOSITION_SUCCESSOR_ALLOWED_PATHS = (
    "ARCHITECTURE.md",
    "CHANGELOG.md",
    "DECISIONS.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "ROADMAP.md",
    "SECURITY.md",
    "SECURITY_CHECKLIST.md",
    "TESTING_GUIDE.md",
    "TROUBLESHOOTING_LOG.md",
    "docs/increments/personal-assistant-v0-signing-security-prerequisite-planning.md",
    "docs/plans/2026-08-29-personal-assistant-v0-signing-security-prerequisite-planning.md",
    "docs/reviews/2026-08-29-personal-assistant-v0-signing-security-prerequisite-planning-post-increment-review.md",
)

DISPOSITION_KEYS = frozenset(
    {
        "allowed_paths",
        "baseline_commit",
        "disposition_id",
        "next_increment_readiness",
        "predecessor_state_sha256",
        "quality_gate",
        "report_path",
        "report_sha256",
        "successor_increment_id",
        "workspace_fingerprint",
    }
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


def _hash_bytes(payload: bytes) -> str:
    return hashlib.sha256(payload).hexdigest()


def _finding_sha256(finding: dict[str, Any]) -> str:
    payload = json.dumps(
        finding,
        ensure_ascii=False,
        separators=(",", ":"),
        sort_keys=True,
    ).encode("utf-8")
    return _hash_bytes(payload)


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


def current_head_commit(root: Path) -> str:
    try:
        commit = _run_git(root, "rev-parse", "HEAD").decode("ascii").strip()
    except UnicodeDecodeError as error:
        raise GateError("Git returned an invalid HEAD commit") from error
    if re.fullmatch(r"(?:[0-9a-f]{40}|[0-9a-f]{64})", commit) is None:
        _fail("Git returned an invalid HEAD commit")
    return commit


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


def _validate_disposition(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        _fail(f"{label} must be a JSON object")
    _require_exact_keys(value, DISPOSITION_KEYS, label)
    if value.get("disposition_id") != FAILED_DISPOSITION_RECOVERY_ID:
        _fail(f"{label} has an invalid disposition id")
    if (
        value.get("predecessor_state_sha256")
        != FAILED_DISPOSITION_PREDECESSOR_STATE_SHA256
    ):
        _fail(f"{label} has an invalid predecessor state digest")
    if value.get("report_path") != FAILED_DISPOSITION_RECOVERY_REPORT_PATH:
        _fail(f"{label} has an invalid recovery report path")
    if value.get("baseline_commit") != FAILED_DISPOSITION_BASE_COMMIT:
        _fail(f"{label} has an invalid baseline commit")
    if (
        value.get("successor_increment_id")
        != FAILED_DISPOSITION_SUCCESSOR_INCREMENT_ID
    ):
        _fail(f"{label} has an invalid successor increment")
    if value.get("quality_gate") not in {"PASS", "PASS WITH ADVISORIES"}:
        _fail(f"{label} has a non-passing recovery result")
    if value.get("next_increment_readiness") not in {
        "Ready",
        "Ready with advisories",
    }:
        _fail(f"{label} has an invalid successor readiness")
    for key in ("report_sha256", "workspace_fingerprint"):
        digest = value.get(key)
        if not isinstance(digest, str) or re.fullmatch(r"[0-9a-f]{64}", digest) is None:
            _fail(f"{label} has an invalid evidence digest")
    if value.get("allowed_paths") != list(
        FAILED_DISPOSITION_SUCCESSOR_ALLOWED_PATHS
    ):
        _fail(f"{label} has an invalid successor path allowlist")
    return value


def validate_state(state_value: dict[str, Any]) -> None:
    schema_version = state_value.get("schema_version")
    status_value = state_value.get("status")
    common_keys = {
        "baseline_fingerprint",
        "increment_id",
        "schema_version",
        "status",
    }
    if status_value == "active":
        active_keys = set(common_keys)
        if "predecessor_disposition" in state_value:
            if schema_version != STATE_SCHEMA_VERSION:
                _fail("predecessor lineage requires the current state schema")
            active_keys.add("predecessor_disposition")
            _validate_disposition(
                state_value["predecessor_disposition"],
                "predecessor disposition",
            )
            if (
                state_value.get("increment_id")
                != FAILED_DISPOSITION_SUCCESSOR_INCREMENT_ID
            ):
                _fail("predecessor lineage belongs to a different increment")
        _require_exact_keys(
            state_value, frozenset(active_keys), "active gate state"
        )
    elif status_value in {"complete", "failed"}:
        evidence_keys = {
            "quality_gate",
            "report_path",
            "report_sha256",
            "workspace_fingerprint",
        }
        if status_value == "failed":
            evidence_keys.add("next_increment_readiness")
            evidence_keys.add("head_commit")
        lineage_keys: set[str] = set()
        if "successor_disposition" in state_value:
            if status_value != "failed" or schema_version != STATE_SCHEMA_VERSION:
                _fail("successor disposition is invalid for this gate state")
            lineage_keys.add("successor_disposition")
            _validate_disposition(
                state_value["successor_disposition"], "successor disposition"
            )
            if state_value.get("increment_id") != FAILED_DISPOSITION_INCREMENT_ID:
                _fail("successor disposition belongs to a different failed increment")
        if "predecessor_disposition" in state_value:
            if schema_version != STATE_SCHEMA_VERSION:
                _fail("predecessor lineage requires the current state schema")
            lineage_keys.add("predecessor_disposition")
            _validate_disposition(
                state_value["predecessor_disposition"],
                "predecessor disposition",
            )
            if (
                state_value.get("increment_id")
                != FAILED_DISPOSITION_SUCCESSOR_INCREMENT_ID
            ):
                _fail("predecessor lineage belongs to a different increment")
        _require_exact_keys(
            state_value,
            frozenset(
                common_keys
                | evidence_keys
                | lineage_keys
                | ({"completion_marker"} if status_value == "complete" else set())
            ),
            f"{status_value} gate state",
        )
        if status_value == "complete":
            if state_value.get("completion_marker") != COMPLETION_MARKER:
                _fail("completed gate state has an invalid completion marker")
            if state_value.get("quality_gate") not in {
                "PASS",
                "PASS WITH ADVISORIES",
            }:
                _fail("completed gate state has a non-passing result")
        else:
            if state_value.get("quality_gate") != "FAIL":
                _fail("failed gate state must preserve a FAIL result")
            if state_value.get("next_increment_readiness") not in READINESS_RESULTS:
                _fail("failed gate state has an invalid readiness result")
            head_commit = state_value.get("head_commit")
            if not isinstance(head_commit, str) or re.fullmatch(
                r"(?:[0-9a-f]{40}|[0-9a-f]{64})", head_commit
            ) is None:
                _fail("failed gate state has an invalid HEAD commit")
        report_path = state_value.get("report_path")
        if not isinstance(report_path, str) or not report_path:
            _fail(f"{status_value} gate state has invalid evidence")
        for key in ("report_sha256", "workspace_fingerprint"):
            value = state_value.get(key)
            if not isinstance(value, str) or not re.fullmatch(r"[0-9a-f]{64}", value):
                _fail(f"{status_value} gate state has invalid evidence")
    else:
        _fail("post-increment state has an unknown status")

    supported_state_versions = {
        LEGACY_STATE_SCHEMA_VERSION,
        LEGACY_FAILED_STATE_SCHEMA_VERSION,
        STATE_SCHEMA_VERSION,
    }
    if schema_version not in supported_state_versions:
        _fail("post-increment state schema version is unsupported")
    if status_value == "failed" and schema_version == LEGACY_STATE_SCHEMA_VERSION:
        _fail("failed gate state requires schema version 2 or later")
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
    if existing_state is not None and existing_state["status"] == "failed":
        if not validate_failed_state(root, existing_state):
            _fail("terminal failed gate evidence is invalid")
        if existing_state["increment_id"] == increment_id:
            _fail("this increment already has a valid terminal failure record")
        disposition = existing_state.get("successor_disposition")
        if disposition is not None:
            if increment_id != disposition["successor_increment_id"]:
                _fail("terminal disposition permits only its recorded successor")
            if disposition["next_increment_readiness"] == "Blocked":
                _fail("terminal disposition blocks the recorded successor")
            if changed_paths(root):
                _fail("recorded successor must begin from a clean workspace")
            write_state(
                root,
                {
                    "baseline_fingerprint": workspace_fingerprint(root),
                    "increment_id": increment_id,
                    "predecessor_disposition": disposition,
                    "schema_version": STATE_SCHEMA_VERSION,
                    "status": "active",
                },
            )
            return
        if existing_state["next_increment_readiness"] == "Blocked":
            _fail("terminal failed gate evidence blocks the next increment")
        if changed_paths(root):
            _fail("terminal failed increment must have a clean workspace")
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
            "schema_version": STATE_SCHEMA_VERSION,
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


def _validate_report_evidence(
    root: Path, report_value: str, increment_id: str
) -> tuple[dict[str, Any], Path, str]:
    report_path = _safe_report_path(root, report_value, increment_id)
    report_text = _read_bounded_text(
        report_path, MAX_REPORT_BYTES, "post-increment report"
    )
    report_lines = report_text.splitlines()
    for section in REQUIRED_REPORT_SECTIONS:
        if sum(line == section for line in report_lines) != 1:
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
    if manifest["schema_version"] != REPORT_SCHEMA_VERSION:
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
    if any(finding["blocks_next_increment"] for finding in findings) and manifest[
        "next_increment_readiness"
    ] != "Blocked":
        _fail("next-increment readiness contradicts a blocking finding")
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
    report_digest = _hash_file(report_path).hex()
    return manifest, report_path, report_digest


def validate_report(
    root: Path, report_value: str, increment_id: str
) -> tuple[dict[str, Any], Path, str]:
    validated = _validate_report_evidence(root, report_value, increment_id)
    if validated[0]["quality_gate"] == "FAIL":
        _fail("post-increment report contains blocking evidence")
    return validated


def validate_failed_report(
    root: Path, report_value: str, increment_id: str
) -> tuple[dict[str, Any], Path, str]:
    validated = _validate_report_evidence(root, report_value, increment_id)
    if validated[0]["quality_gate"] != "FAIL":
        _fail("terminal failure requires a FAIL report")
    return validated


def _is_ancestor(root: Path, ancestor: str, descendant: str) -> bool:
    try:
        merge_base = _run_git(root, "merge-base", ancestor, descendant).decode(
            "ascii"
        ).strip()
    except UnicodeDecodeError as error:
        raise GateError("Git returned an invalid merge base") from error
    return merge_base == ancestor


def _validate_predecessor_report(root: Path) -> None:
    report_path = _safe_report_path(
        root,
        FAILED_DISPOSITION_PREDECESSOR_REPORT_PATH,
        FAILED_DISPOSITION_INCREMENT_ID,
    )
    report_text = _read_bounded_text(
        report_path, MAX_REPORT_BYTES, "predecessor post-increment report"
    )
    if _hash_file(report_path).hex() != (
        FAILED_DISPOSITION_PREDECESSOR_REPORT_SHA256
    ):
        _fail("predecessor post-increment report digest changed")
    committed_report = _run_git(
        root,
        "show",
        f"{FAILED_DISPOSITION_BASE_COMMIT}:"
        f"{FAILED_DISPOSITION_PREDECESSOR_REPORT_PATH}",
    )
    if _hash_bytes(committed_report) != FAILED_DISPOSITION_PREDECESSOR_REPORT_SHA256:
        _fail("published predecessor report digest does not match")

    manifest = _extract_manifest(report_text)
    if (
        manifest.get("increment_id") != FAILED_DISPOSITION_INCREMENT_ID
        or manifest.get("quality_gate") != "FAIL"
        or manifest.get("next_increment_readiness") != "Blocked"
    ):
        _fail("predecessor report no longer preserves terminal blocked failure")
    findings = manifest.get("findings")
    if not isinstance(findings, list) or not all(
        isinstance(finding, dict) for finding in findings
    ):
        _fail("predecessor report findings are invalid")
    blocker_hashes = frozenset(
        _finding_sha256(finding)
        for finding in findings
        if finding.get("blocks_next_increment") is True
    )
    if blocker_hashes != FAILED_DISPOSITION_BLOCKING_FINDING_SHA256:
        _fail("predecessor blocking finding identities changed")


def _validate_predecessor_state_for_disposition(
    root: Path, state_value: dict[str, Any]
) -> None:
    expected_keys = frozenset(
        {
            "baseline_fingerprint",
            "head_commit",
            "increment_id",
            "next_increment_readiness",
            "quality_gate",
            "report_path",
            "report_sha256",
            "schema_version",
            "status",
            "workspace_fingerprint",
        }
    )
    _require_exact_keys(state_value, expected_keys, "predecessor failed gate state")
    if state_value.get("schema_version") != LEGACY_FAILED_STATE_SCHEMA_VERSION:
        _fail("predecessor failed gate state must remain schema version 2")
    if (
        state_value.get("increment_id") != FAILED_DISPOSITION_INCREMENT_ID
        or state_value.get("status") != "failed"
        or state_value.get("quality_gate") != "FAIL"
        or state_value.get("next_increment_readiness") != "Blocked"
        or state_value.get("report_path")
        != FAILED_DISPOSITION_PREDECESSOR_REPORT_PATH
        or state_value.get("report_sha256")
        != FAILED_DISPOSITION_PREDECESSOR_REPORT_SHA256
        or state_value.get("head_commit")
        != FAILED_DISPOSITION_PREDECESSOR_HEAD_COMMIT
    ):
        _fail("predecessor failed gate evidence does not match D-098")
    if "completion_marker" in state_value:
        _fail("predecessor failed gate state must not have a completion marker")
    raw_state_digest = _hash_file(state_path(root)).hex()
    if raw_state_digest != FAILED_DISPOSITION_PREDECESSOR_STATE_SHA256:
        _fail("predecessor failed gate state digest changed")


def _disposed_predecessor_state_sha256(state_value: dict[str, Any]) -> str:
    predecessor_state = dict(state_value)
    predecessor_state.pop("successor_disposition", None)
    predecessor_state["schema_version"] = LEGACY_FAILED_STATE_SCHEMA_VERSION
    payload = (json.dumps(predecessor_state, indent=2, sort_keys=True) + "\n").encode(
        "utf-8"
    )
    return _hash_bytes(payload)


def _validate_recovery_report(
    root: Path,
) -> tuple[dict[str, Any], str]:
    manifest, _, report_digest = validate_report(
        root,
        FAILED_DISPOSITION_RECOVERY_REPORT_PATH,
        FAILED_DISPOSITION_RECOVERY_ID,
    )
    if manifest["next_increment_readiness"] == "Blocked":
        _fail("recovery report does not admit its exact successor")
    if any(
        entry["required"] and entry["status"] != "Passed"
        for entry in manifest["verification"]
    ) or any(
        entry["required"] and entry["status"] != "Passed"
        for entry in manifest["manual_verification"]
    ):
        _fail("recovery report has incomplete required evidence")
    if any(finding["blocks_next_increment"] for finding in manifest["findings"]):
        _fail("recovery report contains a next-blocking finding")
    return manifest, report_digest


def _validate_disposed_failed_state(
    root: Path, state_value: dict[str, Any]
) -> bool:
    try:
        validate_state(state_value)
        if has_merge_conflicts(root):
            return False
        if (
            state_value.get("increment_id") != FAILED_DISPOSITION_INCREMENT_ID
            or state_value.get("status") != "failed"
            or state_value.get("quality_gate") != "FAIL"
            or state_value.get("next_increment_readiness") != "Blocked"
            or state_value.get("report_path")
            != FAILED_DISPOSITION_PREDECESSOR_REPORT_PATH
            or state_value.get("report_sha256")
            != FAILED_DISPOSITION_PREDECESSOR_REPORT_SHA256
            or state_value.get("head_commit")
            != FAILED_DISPOSITION_PREDECESSOR_HEAD_COMMIT
            or "completion_marker" in state_value
        ):
            return False
        if (
            _disposed_predecessor_state_sha256(state_value)
            != FAILED_DISPOSITION_PREDECESSOR_STATE_SHA256
        ):
            return False
        _validate_predecessor_report(root)
        current_head = current_head_commit(root)
        if not _is_ancestor(root, FAILED_DISPOSITION_BASE_COMMIT, current_head):
            return False
        disposition = _validate_disposition(
            state_value["successor_disposition"], "successor disposition"
        )
        recovery_path = _safe_report_path(
            root,
            FAILED_DISPOSITION_RECOVERY_REPORT_PATH,
            FAILED_DISPOSITION_RECOVERY_ID,
        )
        recovery_text = _read_bounded_text(
            recovery_path, MAX_REPORT_BYTES, "disposition recovery report"
        )
        if _hash_file(recovery_path).hex() != disposition["report_sha256"]:
            return False
        if workspace_fingerprint(root) != disposition["workspace_fingerprint"]:
            return False
        repository_changes = changed_paths(root)
        if repository_changes and set(repository_changes) != set(
            FAILED_DISPOSITION_RECOVERY_ALLOWED_PATHS
        ):
            return False
        if suspicious_changed_paths(repository_changes):
            return False
        manifest = _extract_manifest(recovery_text)
        if (
            manifest.get("increment_id") != FAILED_DISPOSITION_RECOVERY_ID
            or manifest.get("quality_gate") != disposition["quality_gate"]
            or manifest.get("next_increment_readiness")
            != disposition["next_increment_readiness"]
        ):
            return False
    except (GateError, KeyError):
        return False
    return True


def record_failed_disposition(root: Path) -> None:
    if has_merge_conflicts(root):
        _fail("cannot record a failed disposition while merge conflicts exist")
    state_value = read_state(root)
    if state_value is None:
        _fail("no post-increment gate state exists")
    if "successor_disposition" in state_value:
        if _validate_disposed_failed_state(root, state_value):
            return
        _fail("existing failed disposition evidence is invalid")

    if current_head_commit(root) != FAILED_DISPOSITION_BASE_COMMIT:
        _fail("failed disposition must be recorded from its exact baseline commit")
    if not _is_ancestor(
        root,
        FAILED_DISPOSITION_PREDECESSOR_HEAD_COMMIT,
        FAILED_DISPOSITION_BASE_COMMIT,
    ):
        _fail("predecessor failed HEAD is not an ancestor of the recovery baseline")
    _validate_predecessor_state_for_disposition(root, state_value)
    _validate_predecessor_report(root)

    repository_changes = changed_paths(root)
    if set(repository_changes) != set(FAILED_DISPOSITION_RECOVERY_ALLOWED_PATHS):
        _fail("failed disposition change set does not match its exact allowlist")
    if suspicious_changed_paths(repository_changes):
        _fail("suspicious generated, credential, database, or build path detected")

    manifest, report_digest = _validate_recovery_report(root)
    disposition = {
        "allowed_paths": list(FAILED_DISPOSITION_SUCCESSOR_ALLOWED_PATHS),
        "baseline_commit": FAILED_DISPOSITION_BASE_COMMIT,
        "disposition_id": FAILED_DISPOSITION_RECOVERY_ID,
        "next_increment_readiness": manifest["next_increment_readiness"],
        "predecessor_state_sha256": FAILED_DISPOSITION_PREDECESSOR_STATE_SHA256,
        "quality_gate": manifest["quality_gate"],
        "report_path": FAILED_DISPOSITION_RECOVERY_REPORT_PATH,
        "report_sha256": report_digest,
        "successor_increment_id": FAILED_DISPOSITION_SUCCESSOR_INCREMENT_ID,
        "workspace_fingerprint": workspace_fingerprint(root),
    }
    disposed_state = dict(state_value)
    disposed_state["schema_version"] = STATE_SCHEMA_VERSION
    disposed_state["successor_disposition"] = disposition
    write_state(root, disposed_state)
    recorded_state = read_state(root)
    if recorded_state is None or not _validate_disposed_failed_state(
        root, recorded_state
    ):
        _fail("recorded failed disposition did not validate")


def _validate_lineage_change_scope(
    state_value: dict[str, Any], repository_changes: tuple[str, ...]
) -> None:
    disposition = state_value.get("predecessor_disposition")
    if disposition is None:
        return
    allowed_paths = frozenset(disposition["allowed_paths"])
    if not set(repository_changes).issubset(allowed_paths):
        _fail("successor change set exceeds its recorded path allowlist")


def finalize_gate(root: Path, increment_id: str, report_value: str) -> None:
    increment_id = validate_increment_id(increment_id)
    if has_merge_conflicts(root):
        _fail("cannot complete an increment while merge conflicts exist")
    state_value = read_state(root)
    if state_value is None:
        _fail("no post-increment gate state exists")
    if state_value["increment_id"] != increment_id:
        _fail("increment does not match finalize request")
    if state_value["status"] == "failed":
        _fail("a terminally failed increment cannot be completed")

    repository_changes = changed_paths(root)
    suspicious = suspicious_changed_paths(repository_changes)
    if suspicious:
        _fail("suspicious generated, credential, database, or build path detected")
    _validate_lineage_change_scope(state_value, repository_changes)

    manifest, report_path, report_digest = validate_report(
        root, report_value, increment_id
    )
    relative_report = report_path.relative_to(root).as_posix()
    completed_state = {
        "baseline_fingerprint": state_value["baseline_fingerprint"],
        "completion_marker": COMPLETION_MARKER,
        "increment_id": increment_id,
        "quality_gate": manifest["quality_gate"],
        "report_path": relative_report,
        "report_sha256": report_digest,
        "schema_version": STATE_SCHEMA_VERSION,
        "status": "complete",
        "workspace_fingerprint": workspace_fingerprint(root),
    }
    if "predecessor_disposition" in state_value:
        completed_state["predecessor_disposition"] = state_value[
            "predecessor_disposition"
        ]
    write_state(root, completed_state)


def close_failed_gate(root: Path, increment_id: str, report_value: str) -> None:
    increment_id = validate_increment_id(increment_id)
    if has_merge_conflicts(root):
        _fail("cannot close a failed increment while merge conflicts exist")
    state_value = read_state(root)
    if state_value is None:
        _fail("no post-increment gate state exists")
    if state_value["increment_id"] != increment_id:
        _fail("increment does not match close-failed request")
    if state_value["status"] == "complete":
        _fail("a completed increment cannot be changed to failed")
    if (
        state_value["status"] == "failed"
        and state_value["report_path"] != report_value
    ):
        _fail("terminal failure may be reclosed only with the same report path")
    if (
        state_value["status"] == "failed"
        and state_value["head_commit"] != current_head_commit(root)
    ):
        _fail("terminal failure cannot be reclosed after HEAD changes")

    repository_changes = changed_paths(root)
    suspicious = suspicious_changed_paths(repository_changes)
    if suspicious:
        _fail("suspicious generated, credential, database, or build path detected")
    _validate_lineage_change_scope(state_value, repository_changes)

    manifest, report_path, report_digest = validate_failed_report(
        root, report_value, increment_id
    )
    relative_report = report_path.relative_to(root).as_posix()
    failed_state = {
        "baseline_fingerprint": state_value["baseline_fingerprint"],
        "head_commit": current_head_commit(root),
        "increment_id": increment_id,
        "next_increment_readiness": manifest["next_increment_readiness"],
        "quality_gate": "FAIL",
        "report_path": relative_report,
        "report_sha256": report_digest,
        "schema_version": STATE_SCHEMA_VERSION,
        "status": "failed",
        "workspace_fingerprint": workspace_fingerprint(root),
    }
    if "predecessor_disposition" in state_value:
        failed_state["predecessor_disposition"] = state_value[
            "predecessor_disposition"
        ]
    write_state(root, failed_state)


def _validate_terminal_evidence(root: Path, state_value: dict[str, Any]) -> bool:
    if (
        state_value.get("status") == "failed"
        and "successor_disposition" in state_value
    ):
        return _validate_disposed_failed_state(root, state_value)
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
        report_digest = _hash_file(report_path).hex()
        if report_digest != state_value["report_sha256"]:
            return False
        if workspace_fingerprint(root) != state_value["workspace_fingerprint"]:
            return False
        if suspicious_changed_paths(changed_paths(root)):
            return False
        if state_value["status"] == "failed":
            manifest = _extract_manifest(report_text)
            if manifest.get("quality_gate") != "FAIL" or manifest.get(
                "next_increment_readiness"
            ) != state_value["next_increment_readiness"]:
                return False
    except GateError:
        return False
    return True


def validate_completed_state(root: Path, state_value: dict[str, Any]) -> bool:
    return state_value.get("status") == "complete" and _validate_terminal_evidence(
        root, state_value
    )


def validate_failed_state(root: Path, state_value: dict[str, Any]) -> bool:
    return state_value.get("status") == "failed" and _validate_terminal_evidence(
        root, state_value
    )


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
        if state_value["status"] == "failed":
            return StopDecision(
                should_continue=not validate_failed_state(root, state_value)
            )
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
    if state_value["status"] in {"complete", "failed"}:
        status_value["quality_gate"] = state_value["quality_gate"]
        status_value["report_path"] = state_value["report_path"]
        if state_value["status"] == "failed":
            status_value["next_increment_readiness"] = state_value[
                "next_increment_readiness"
            ]
            if "successor_disposition" in state_value:
                disposition = state_value["successor_disposition"]
                status_value["successor_disposition"] = {
                    "disposition_id": disposition["disposition_id"],
                    "next_increment_readiness": disposition[
                        "next_increment_readiness"
                    ],
                    "quality_gate": disposition["quality_gate"],
                    "report_path": disposition["report_path"],
                    "successor_increment_id": disposition[
                        "successor_increment_id"
                    ],
                }
            status_value["valid"] = validate_failed_state(root, state_value)
        else:
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

    failed_parser = subparsers.add_parser(
        "close-failed",
        help="validate a FAIL report and record terminal failure without completion",
    )
    failed_parser.add_argument("--increment", required=True)
    failed_parser.add_argument("--report", required=True)

    subparsers.add_parser(
        "record-failed-disposition",
        help="record the exact D-098 cumulative-evidence disposition",
    )

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
        elif arguments.command == "close-failed":
            close_failed_gate(root, arguments.increment, arguments.report)
            print(
                "post-increment-gate: recorded terminal failure for increment "
                f"{arguments.increment}"
            )
        elif arguments.command == "record-failed-disposition":
            record_failed_disposition(root)
            print(
                "post-increment-gate: recorded exact successor disposition for "
                f"{FAILED_DISPOSITION_INCREMENT_ID}"
            )
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
