#!/usr/bin/env python3
"""Read-only repository health checks used locally and in GitHub Actions."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable, Sequence
from urllib.parse import unquote, urlsplit

MAX_TEXT_BYTES = 2 * 1024 * 1024

MARKDOWN_LINK = re.compile(r"!?\[[^\]]*\]\((?P<target>[^)]+)\)")
MARKDOWN_REFERENCE = re.compile(r"^\s*\[[^\]]+\]:\s*(?P<target>\S+)", re.MULTILINE)
HTML_REFERENCE = re.compile(r"\b(?:href|src)=[\"'](?P<target>[^\"']+)[\"']", re.IGNORECASE)
NPM_RUN_COMMAND = re.compile(r"\bnpm\s+run\s+(?P<script>[A-Za-z0-9:_-]+)")
ACTION_USE = re.compile(
    r"^\s*(?:-\s*)?uses:\s*(?P<reference>\S+)(?:\s+#.*)?$", re.MULTILINE
)
IMMUTABLE_ACTION = re.compile(r"^[^@\s]+@[0-9a-fA-F]{40}$")
IMMUTABLE_CONTAINER = re.compile(r"^docker://[^@\s]+@sha256:[0-9a-fA-F]{64}$")
WRITE_PERMISSION = re.compile(r"^\s*[a-z][a-z-]*:\s*write\s*$", re.MULTILINE)
SELF_HOSTED_RUNNER_SELECTOR = "runs-on: [self-hosted, Linux, X64, cortexa-ci]"
SELF_HOSTED_PUSH_BRANCHES = (
    "- main",
    '- "codex/**"',
    '- "feature/**"',
    '- "fix/**"',
    '- "refactor/**"',
    '- "meta/**"',
    '- "phase*/**"',
)

SECRET_PATTERNS = (
    ("private-key", re.compile(r"-{5}BEGIN (?:EC |OPENSSH |RSA )?PRIVATE KEY-{5}")),
    ("github-token", re.compile(r"\bgh[pousr]_[A-Za-z0-9]{36,}\b")),
    ("openai-key", re.compile(r"\bsk-(?:proj-)?[A-Za-z0-9_-]{32,}\b")),
    ("aws-access-key", re.compile(r"\b(?:AKIA|ASIA)[0-9A-Z]{16}\b")),
    ("slack-token", re.compile(r"\bxox[baprs]-[A-Za-z0-9-]{20,}\b")),
)

GENERATED_TOP_LEVEL_DIRECTORIES = frozenset(
    {".next", "build", "coverage", "dist", "node_modules", "out"}
)
GENERATED_SUFFIXES = (
    ".bak",
    ".backup",
    ".db",
    ".db-shm",
    ".db-wal",
    ".log",
    ".sqlite",
    ".sqlite-shm",
    ".sqlite-wal",
    ".sqlite3",
    ".sqlite3-shm",
    ".sqlite3-wal",
)
LICENSE_EVIDENCE = (
    "LICENSE",
    "LICENSE.md",
    "COPYING",
    "docs/github/LICENSING.md",
)
AUTHORITATIVE_COMMAND_DOCUMENTS = (
    "AGENTS.md",
    "CODE_REVIEW.md",
    "CONTRIBUTING.md",
    "ENGINEERING_GUIDE.md",
    "README.md",
    "RELEASE_CHECKLIST.md",
    "TESTING_GUIDE.md",
)
REQUIRED_SCRIPTS = frozenset(
    {
        "build",
        "docs:check",
        "format:check",
        "lint",
        "repository:check",
        "security:scan",
        "tauri",
        "test",
        "test:hooks",
        "test:integration",
        "test:repository",
        "test:unit",
        "typecheck",
        "verify",
    }
)


@dataclass(frozen=True)
class Finding:
    check: str
    path: str
    detail: str
    line: int | None = None

    def render(self) -> str:
        location = self.path if self.line is None else f"{self.path}:{self.line}"
        return f"{self.check}: {location}: {self.detail}"


def run_git(root: Path, *arguments: str) -> bytes:
    result = subprocess.run(
        ["git", *arguments],
        cwd=root,
        check=False,
        capture_output=True,
    )
    if result.returncode != 0:
        raise RuntimeError("Git repository inspection failed")
    return result.stdout


def repository_root(cwd: Path) -> Path:
    result = subprocess.run(
        ["git", "rev-parse", "--show-toplevel"],
        cwd=cwd,
        check=False,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        raise RuntimeError("Working directory is not inside a Git repository")
    return Path(result.stdout.strip()).resolve(strict=True)


def repository_paths(root: Path) -> tuple[str, ...]:
    payload = run_git(
        root,
        "ls-files",
        "--cached",
        "--others",
        "--exclude-standard",
        "-z",
    )
    paths: list[str] = []
    for raw_path in payload.split(b"\0"):
        if not raw_path:
            continue
        relative_path = raw_path.decode("utf-8")
        path = Path(relative_path)
        if path.is_absolute() or ".." in path.parts or ".git" in path.parts:
            raise RuntimeError("Git returned an unsafe repository path")
        if (root / path).is_file():
            paths.append(relative_path)
    return tuple(sorted(set(paths)))


def read_text(path: Path) -> str | None:
    try:
        payload = path.read_bytes()
    except OSError as error:
        raise RuntimeError(f"Repository file could not be read: {path}") from error
    if len(payload) > MAX_TEXT_BYTES or b"\0" in payload:
        return None
    try:
        return payload.decode("utf-8")
    except UnicodeDecodeError:
        return None


def without_fenced_code(text: str) -> str:
    output: list[str] = []
    fence: str | None = None
    for line in text.splitlines(keepends=True):
        stripped = line.lstrip()
        marker = "```" if stripped.startswith("```") else "~~~" if stripped.startswith("~~~") else None
        if marker is not None:
            fence = None if fence == marker else marker if fence is None else fence
            output.append("\n")
        elif fence is None:
            output.append(line)
        else:
            output.append("\n")
    return "".join(output)


def normalize_link_target(raw_target: str) -> str:
    target = raw_target.strip()
    if target.startswith("<") and ">" in target:
        return target[1 : target.index(">")]
    if " " in target or "\t" in target:
        return target.split(maxsplit=1)[0]
    return target


def local_link_target(source: Path, root: Path, raw_target: str) -> Path | None:
    root = root.resolve(strict=True)
    source = source.resolve(strict=True)
    target = normalize_link_target(raw_target)
    if not target or target.startswith("#"):
        return None
    parsed = urlsplit(target)
    if parsed.scheme or parsed.netloc:
        return None
    decoded_path = unquote(parsed.path)
    if not decoded_path:
        return None
    candidate = root / decoded_path.lstrip("/") if decoded_path.startswith("/") else source.parent / decoded_path
    try:
        resolved = candidate.resolve(strict=False)
        resolved.relative_to(root)
    except (OSError, ValueError):
        return Path("/__outside_repository__")
    return resolved


def link_findings(root: Path, paths: Sequence[str]) -> tuple[Finding, ...]:
    findings: list[Finding] = []
    for relative_path in paths:
        if not relative_path.lower().endswith(".md"):
            continue
        source = root / relative_path
        text = read_text(source)
        if text is None:
            continue
        inspected = without_fenced_code(text)
        matches = [
            *MARKDOWN_LINK.finditer(inspected),
            *MARKDOWN_REFERENCE.finditer(inspected),
            *HTML_REFERENCE.finditer(inspected),
        ]
        for match in matches:
            raw_target = match.group("target")
            target = local_link_target(source, root, raw_target)
            if target is None or target.exists():
                continue
            line = inspected.count("\n", 0, match.start()) + 1
            findings.append(
                Finding("links", relative_path, f"missing local target {normalize_link_target(raw_target)!r}", line)
            )
    return tuple(findings)


def secret_findings(root: Path, paths: Sequence[str]) -> tuple[Finding, ...]:
    findings: list[Finding] = []
    for relative_path in paths:
        text = read_text(root / relative_path)
        if text is None:
            continue
        for name, pattern in SECRET_PATTERNS:
            for match in pattern.finditer(text):
                line = text.count("\n", 0, match.start()) + 1
                findings.append(Finding("secrets", relative_path, f"matched {name} pattern", line))
    return tuple(findings)


def generated_path_findings(paths: Sequence[str]) -> tuple[Finding, ...]:
    findings: list[Finding] = []
    for relative_path in paths:
        path = Path(relative_path)
        lower_parts = tuple(part.lower() for part in path.parts)
        lower_name = path.name.lower()
        is_environment = lower_name == ".env" or (
            lower_name.startswith(".env.") and lower_name != ".env.example"
        )
        is_generated_directory = bool(lower_parts) and lower_parts[0] in GENERATED_TOP_LEVEL_DIRECTORIES
        is_tauri_target = len(lower_parts) >= 2 and lower_parts[:2] == ("src-tauri", "target")
        is_root_backup = bool(lower_parts) and lower_parts[0] == "backups"
        if (
            lower_name == ".ds_store"
            or is_environment
            or is_generated_directory
            or is_tauri_target
            or is_root_backup
            or lower_name.endswith(GENERATED_SUFFIXES)
        ):
            findings.append(Finding("generated", relative_path, "generated or local-only path is tracked"))
    return tuple(findings)


def license_findings(root: Path) -> tuple[Finding, ...]:
    for relative_path in LICENSE_EVIDENCE:
        path = root / relative_path
        if path.is_file() and path.stat().st_size > 0:
            return ()
    return (Finding("license", ".", "no license file or licensing decision record exists"),)


def command_findings(root: Path) -> tuple[Finding, ...]:
    package_path = root / "package.json"
    try:
        package = json.loads(package_path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        raise RuntimeError("package.json could not be parsed") from error
    scripts = package.get("scripts")
    if not isinstance(scripts, dict) or not all(isinstance(name, str) for name in scripts):
        raise RuntimeError("package.json scripts are invalid")

    findings = [
        Finding("commands", "package.json", f"required npm script {name!r} is missing")
        for name in sorted(REQUIRED_SCRIPTS - scripts.keys())
    ]
    for relative_path in AUTHORITATIVE_COMMAND_DOCUMENTS:
        path = root / relative_path
        if not path.is_file():
            findings.append(Finding("commands", relative_path, "authoritative command document is missing"))
            continue
        text = read_text(path)
        if text is None:
            findings.append(Finding("commands", relative_path, "authoritative command document is not UTF-8 text"))
            continue
        for match in NPM_RUN_COMMAND.finditer(text):
            script = match.group("script")
            if script not in scripts:
                line = text.count("\n", 0, match.start()) + 1
                findings.append(Finding("commands", relative_path, f"documents missing npm script {script!r}", line))
    return tuple(findings)


def workflow_findings(root: Path) -> tuple[Finding, ...]:
    workflow_directory = root / ".github" / "workflows"
    if not workflow_directory.is_dir():
        return (Finding("workflows", ".github/workflows", "workflow directory is missing"),)

    findings: list[Finding] = []
    workflows = sorted((*workflow_directory.glob("*.yml"), *workflow_directory.glob("*.yaml")))
    if not workflows:
        return (Finding("workflows", ".github/workflows", "no workflow files exist"),)
    for path in workflows:
        relative_path = path.relative_to(root).as_posix()
        text = read_text(path)
        if text is None:
            findings.append(Finding("workflows", relative_path, "workflow is not UTF-8 text"))
            continue
        for match in ACTION_USE.finditer(text):
            reference = match.group("reference")
            if reference.startswith("./"):
                continue
            if not IMMUTABLE_ACTION.fullmatch(reference) and not IMMUTABLE_CONTAINER.fullmatch(reference):
                line = text.count("\n", 0, match.start()) + 1
                findings.append(Finding("workflows", relative_path, "action reference is not pinned to an immutable digest", line))
        prohibited = (
            ("pull_request_target:", "pull_request_target is prohibited"),
            ("persist-credentials: true", "checkout credentials must not persist"),
            ("${{ secrets.", "workflow must not require repository secrets"),
            ("git commit ", "workflow must not create commits"),
            ("git push", "workflow must not push"),
            ("npm publish", "workflow must not publish packages"),
            ("cargo publish", "workflow must not publish crates"),
        )
        for needle, detail in prohibited:
            offset = text.find(needle)
            if offset >= 0:
                findings.append(Finding("workflows", relative_path, detail, text.count("\n", 0, offset) + 1))
        for match in WRITE_PERMISSION.finditer(text):
            findings.append(Finding("workflows", relative_path, "write workflow permission is prohibited", text.count("\n", 0, match.start()) + 1))
        if "self-hosted" in text:
            if SELF_HOSTED_RUNNER_SELECTOR not in text:
                findings.append(
                    Finding(
                        "workflows",
                        relative_path,
                        "self-hosted job must require the exact repository runner labels",
                    )
                )
            missing_branches = tuple(
                branch for branch in SELF_HOSTED_PUSH_BRANCHES if branch not in text
            )
            if (
                "pull_request:" in text
                or "workflow_dispatch:" not in text
                or missing_branches
            ):
                findings.append(
                    Finding(
                        "workflows",
                        relative_path,
                        "self-hosted workflow must exclude pull requests and restrict push branches",
                    )
                )
    return tuple(findings)


def checks_for(command: str) -> tuple[str, ...]:
    if command == "all":
        return ("links", "secrets", "generated", "license", "commands", "workflows")
    return (command,)


def run_checks(root: Path, command: str) -> tuple[Finding, ...]:
    paths = repository_paths(root)
    findings: list[Finding] = []
    for check in checks_for(command):
        if check == "links":
            findings.extend(link_findings(root, paths))
        elif check == "secrets":
            findings.extend(secret_findings(root, paths))
        elif check == "generated":
            findings.extend(generated_path_findings(paths))
        elif check == "license":
            findings.extend(license_findings(root))
        elif check == "commands":
            findings.extend(command_findings(root))
        elif check == "workflows":
            findings.extend(workflow_findings(root))
        else:
            raise RuntimeError(f"Unknown check: {check}")
    return tuple(findings)


def parse_arguments(arguments: Sequence[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "check",
        choices=("all", "links", "secrets", "generated", "license", "commands", "workflows"),
    )
    return parser.parse_args(arguments)


def main(arguments: Sequence[str] | None = None) -> int:
    parsed = parse_arguments(sys.argv[1:] if arguments is None else arguments)
    try:
        root = repository_root(Path.cwd())
        findings = run_checks(root, parsed.check)
    except (OSError, RuntimeError, UnicodeError) as error:
        print(f"repository-health: ERROR: {error}", file=sys.stderr)
        return 2
    if findings:
        for finding in findings:
            print(finding.render(), file=sys.stderr)
        print(f"repository-health: FAIL ({len(findings)} finding(s))", file=sys.stderr)
        return 1
    print(f"repository-health: PASS ({parsed.check})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
