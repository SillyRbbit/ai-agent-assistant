#!/usr/bin/env python3
"""Classify Git changes for risk-based CI and validate commit-range whitespace."""

from __future__ import annotations

import argparse
import fnmatch
import re
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path, PurePosixPath
from typing import Sequence

SHA = re.compile(r"^[0-9a-f]{40}$")
ZERO_SHA = "0" * 40

FRONTEND_PATTERNS = (
    "src/**",
    "index.html",
    "public/**",
    "assets/branding/**",
    ".node-version",
    ".prettierignore",
    ".prettierrc*",
    "vite.config.*",
    "vitest.config.*",
    "tsconfig*.json",
    "eslint.config.*",
)

RUST_PATTERNS = (
    "src-tauri/**",
    "Cargo.toml",
    "Cargo.lock",
    "rust-toolchain",
    "rust-toolchain.*",
)

SECURITY_SENSITIVE_PATTERNS = (
    "src/infrastructure/tauri/**",
    "src-tauri/build.rs",
    "src-tauri/tauri.conf.json",
    "src-tauri/capabilities/**",
    "src-tauri/migrations/**",
    "src-tauri/src/agent/**",
    "src-tauri/src/approvals/**",
    "src-tauri/src/audit/**",
    "src-tauri/src/policy/**",
    "src-tauri/src/storage/**",
    "src-tauri/src/tools/**",
    "src-tauri/src/app_info.rs",
    "src-tauri/src/error.rs",
    "src-tauri/src/lib.rs",
    "src-tauri/src/main.rs",
    "src-tauri/src/startup.rs",
)

CROSS_CUTTING_PATTERNS = (
    "package.json",
    "package-lock.json",
    "npm-shrinkwrap.json",
    "Cargo.toml",
    "Cargo.lock",
    "rust-toolchain",
    "rust-toolchain.*",
    "scripts/ci_change_scope.py",
    "scripts/tests/test_ci_change_scope.py",
    "scripts/verify.sh",
    ".github/workflows/ci.yml",
    ".github/workflows/security.yml",
    "src-tauri/Cargo.toml",
    "src-tauri/Cargo.lock",
    *SECURITY_SENSITIVE_PATTERNS,
)

AUDIT_ONLY_PATTERNS = (
    ".codex/**",
    ".github/workflows/documentation.yml",
    "scripts/cargo_audit_gate.py",
    "scripts/repository_health.py",
    "scripts/tests/test_cargo_audit_gate.py",
    "scripts/tests/test_repository_health.py",
)

AUDIT_PATTERNS = (
    ".codex/**",
    "package.json",
    "package-lock.json",
    "npm-shrinkwrap.json",
    "Cargo.toml",
    "Cargo.lock",
    "src-tauri/Cargo.toml",
    "src-tauri/Cargo.lock",
    "rust-toolchain",
    "rust-toolchain.*",
    "scripts/**",
    ".github/workflows/ci.yml",
    ".github/workflows/documentation.yml",
    ".github/workflows/security.yml",
    *SECURITY_SENSITIVE_PATTERNS,
)

DOCUMENTATION_PATTERNS = (
    "docs/**",
    "prompts/**",
    ".agents/**",
    ".github/CODEOWNERS",
    ".github/PULL_REQUEST_TEMPLATE.md",
    ".github/ISSUE_TEMPLATE/**",
    ".github/dependabot.yml",
    ".github/workflows/documentation.yml",
    ".gitignore",
)


@dataclass(frozen=True)
class Scope:
    frontend: bool
    rust: bool
    audit: bool

    def render(self) -> str:
        return "\n".join(
            (
                f"frontend={str(self.frontend).lower()}",
                f"rust={str(self.rust).lower()}",
                f"audit={str(self.audit).lower()}",
            )
        )


def matches(path: str, patterns: Sequence[str]) -> bool:
    return any(fnmatch.fnmatchcase(path, pattern) for pattern in patterns)


def is_documentation_path(path: str) -> bool:
    return path.endswith((".md", ".mdx")) or matches(path, DOCUMENTATION_PATTERNS)


def validate_path(raw_path: str) -> str:
    path = PurePosixPath(raw_path)
    if not raw_path or path.is_absolute() or ".." in path.parts or "\\" in raw_path:
        raise RuntimeError("Git returned an unsafe changed path")
    return path.as_posix()


def classify(paths: Sequence[str]) -> Scope:
    frontend = False
    rust = False
    audit = False

    for raw_path in paths:
        path = validate_path(raw_path)
        if matches(path, AUDIT_PATTERNS):
            audit = True
        if matches(path, CROSS_CUTTING_PATTERNS):
            frontend = True
            rust = True
        elif matches(path, FRONTEND_PATTERNS):
            frontend = True
        elif matches(path, RUST_PATTERNS):
            rust = True
        elif matches(path, AUDIT_ONLY_PATTERNS):
            pass
        elif not is_documentation_path(path):
            # The workflow-level include list should make this rare. Running both
            # application jobs is safer than silently accepting a new path class.
            frontend = True
            rust = True

    return Scope(frontend=frontend, rust=rust, audit=audit)


def run_git(root: Path, *arguments: str) -> subprocess.CompletedProcess[bytes]:
    return subprocess.run(
        ["git", *arguments],
        cwd=root,
        check=False,
        capture_output=True,
    )


def validated_sha(value: str | None, name: str) -> str:
    if value is None or SHA.fullmatch(value) is None:
        raise RuntimeError(f"{name} must be a full lowercase Git SHA")
    return value


def comparison_range(
    root: Path, event: str, base: str | None, head: str | None
) -> tuple[str, str] | None:
    if event == "schedule":
        return None
    head_sha = validated_sha(head, "head")
    if event == "workflow_dispatch":
        parent = run_git(root, "rev-parse", f"{head_sha}^")
        if parent.returncode != 0:
            return None
        return parent.stdout.decode("ascii").strip(), head_sha

    base_sha = validated_sha(base, "base")
    if event == "push":
        if base_sha == ZERO_SHA:
            return None
        return base_sha, head_sha
    if event == "pull_request":
        merge_base = run_git(root, "merge-base", base_sha, head_sha)
        if merge_base.returncode != 0:
            raise RuntimeError("Git could not determine the pull-request merge base")
        return merge_base.stdout.decode("ascii").strip(), head_sha
    raise RuntimeError(f"unsupported GitHub event: {event}")


def changed_paths(
    root: Path, event: str, base: str | None, head: str | None
) -> tuple[str, ...] | None:
    comparison = comparison_range(root, event, base, head)
    if comparison is None:
        return None
    start, end = comparison
    result = run_git(root, "diff", "--name-only", "-z", start, end, "--")
    if result.returncode != 0:
        raise RuntimeError("Git could not inspect changed paths")
    return tuple(
        validate_path(value.decode("utf-8"))
        for value in result.stdout.split(b"\0")
        if value
    )


def check_diff(
    root: Path, event: str, base: str | None, head: str | None
) -> int:
    comparison = comparison_range(root, event, base, head)
    if comparison is None:
        print("ci-change-scope: no bounded commit range; diff check not applicable")
        return 0
    start, end = comparison
    result = run_git(root, "diff", "--check", "--no-ext-diff", start, end, "--")
    if result.stdout:
        sys.stdout.buffer.write(result.stdout)
    if result.stderr:
        sys.stderr.buffer.write(result.stderr)
    return result.returncode


def parse_arguments(arguments: Sequence[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    subparsers = parser.add_subparsers(dest="command", required=True)
    for command in ("classify", "diff-check"):
        subparser = subparsers.add_parser(command)
        subparser.add_argument(
            "--event",
            required=True,
            choices=("pull_request", "push", "schedule", "workflow_dispatch"),
        )
        subparser.add_argument("--base")
        subparser.add_argument("--head")
    return parser.parse_args(arguments)


def main(arguments: Sequence[str] | None = None) -> int:
    parsed = parse_arguments(sys.argv[1:] if arguments is None else arguments)
    root = Path.cwd().resolve(strict=True)
    try:
        if parsed.command == "diff-check":
            return check_diff(root, parsed.event, parsed.base, parsed.head)
        if parsed.event == "workflow_dispatch":
            scope = Scope(frontend=True, rust=True, audit=True)
        elif parsed.event == "schedule":
            scope = Scope(frontend=False, rust=False, audit=True)
        else:
            paths = changed_paths(root, parsed.event, parsed.base, parsed.head)
            if paths is None or not paths:
                scope = Scope(frontend=True, rust=True, audit=True)
            else:
                scope = classify(paths)
    except (OSError, RuntimeError, UnicodeError) as error:
        print(f"ci-change-scope: ERROR: {error}", file=sys.stderr)
        return 2
    print(scope.render())
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
