#!/usr/bin/env python3
"""Shared safe primitives for Cortexa repository-local Codex gates."""

from __future__ import annotations

import json
import subprocess
from dataclasses import dataclass
from enum import IntEnum
from pathlib import Path
from typing import Any, BinaryIO, NoReturn

MAX_HOOK_INPUT_BYTES = 64 * 1024

SUSPICIOUS_DIRECTORY_NAMES = frozenset(
    {
        ".next",
        "__pycache__",
        "build",
        "coverage",
        "dist",
        "node_modules",
        "out",
        "target",
    }
)
SUSPICIOUS_FILE_SUFFIXES = (
    ".cer",
    ".crt",
    ".db",
    ".key",
    ".log",
    ".p12",
    ".pem",
    ".pfx",
    ".sqlite",
    ".sqlite3",
)
SUSPICIOUS_FILE_NAMES = frozenset(
    {
        "credentials.json",
        "id_ed25519",
        "id_rsa",
        "service-account.json",
    }
)


class ExitCode(IntEnum):
    OK = 0
    VALIDATION_FAILED = 2
    REPOSITORY_ERROR = 3
    IO_ERROR = 4


@dataclass(frozen=True)
class GateError(Exception):
    message: str
    exit_code: ExitCode = ExitCode.VALIDATION_FAILED

    def __str__(self) -> str:
        return self.message


def fail(
    message: str, exit_code: ExitCode = ExitCode.VALIDATION_FAILED
) -> NoReturn:
    raise GateError(message, exit_code)


def run_git(root: Path, *arguments: str) -> bytes:
    try:
        result = subprocess.run(
            ["git", *arguments],
            cwd=root,
            check=False,
            capture_output=True,
        )
    except OSError as error:
        raise GateError(
            "git could not be started", ExitCode.REPOSITORY_ERROR
        ) from error
    if result.returncode != 0:
        fail("git repository inspection failed", ExitCode.REPOSITORY_ERROR)
    return result.stdout


def repository_root(cwd: Path) -> Path:
    try:
        candidate = cwd.expanduser().resolve(strict=True)
    except OSError as error:
        raise GateError("hook working directory is unavailable") from error
    if not candidate.is_dir():
        fail("hook working directory is not a directory")

    try:
        result = subprocess.run(
            ["git", "rev-parse", "--show-toplevel"],
            cwd=candidate,
            check=False,
            capture_output=True,
        )
    except OSError as error:
        raise GateError(
            "git could not be started", ExitCode.REPOSITORY_ERROR
        ) from error
    if result.returncode != 0:
        fail("hook working directory is not inside a Git repository")
    try:
        root = Path(result.stdout.decode("utf-8").strip()).resolve(strict=True)
    except (OSError, UnicodeDecodeError) as error:
        raise GateError("Git returned an invalid repository root") from error
    if not root.is_dir():
        fail("Git repository root is not a directory")
    return root


def validate_relative_path(relative_path: str) -> None:
    path = Path(relative_path)
    if (
        not relative_path
        or path.is_absolute()
        or ".." in path.parts
        or ".git" in path.parts
    ):
        fail("repository-relative path is unsafe")


def workspace_path(root: Path, relative_path: str) -> Path:
    validate_relative_path(relative_path)
    path = root / relative_path
    candidate_parent = path.parent
    while True:
        try:
            parent = candidate_parent.resolve(strict=True)
            parent.relative_to(root)
            break
        except FileNotFoundError:
            if candidate_parent == root:
                raise GateError("repository path escapes the workspace")
            candidate_parent = candidate_parent.parent
        except (OSError, ValueError) as error:
            raise GateError("repository path escapes the workspace") from error
    return path


def decode_git_paths(payload: bytes) -> tuple[str, ...]:
    paths: list[str] = []
    for raw_path in payload.split(b"\0"):
        if not raw_path:
            continue
        try:
            relative_path = raw_path.decode("utf-8")
        except UnicodeDecodeError as error:
            raise GateError("repository contains a non-UTF-8 path") from error
        validate_relative_path(relative_path)
        paths.append(relative_path)
    return tuple(paths)


def unstaged_paths(root: Path) -> tuple[str, ...]:
    return tuple(
        sorted(set(decode_git_paths(run_git(root, "diff", "--name-only", "-z", "--"))))
    )


def staged_paths(root: Path) -> tuple[str, ...]:
    return tuple(
        sorted(
            set(
                decode_git_paths(
                    run_git(root, "diff", "--cached", "--name-only", "-z", "--")
                )
            )
        )
    )


def untracked_paths(root: Path) -> tuple[str, ...]:
    return tuple(
        sorted(
            set(
                decode_git_paths(
                    run_git(root, "ls-files", "--others", "--exclude-standard", "-z")
                )
            )
        )
    )


def changed_paths(root: Path) -> tuple[str, ...]:
    return tuple(
        sorted(
            set(unstaged_paths(root))
            | set(staged_paths(root))
            | set(untracked_paths(root))
        )
    )


def conflicted_paths(root: Path) -> tuple[str, ...]:
    return tuple(
        sorted(
            set(
                decode_git_paths(
                    run_git(
                        root,
                        "diff",
                        "--name-only",
                        "--diff-filter=U",
                        "-z",
                        "--",
                    )
                )
            )
        )
    )


def has_merge_conflicts(root: Path) -> bool:
    return bool(conflicted_paths(root))


def suspicious_changed_paths(paths: tuple[str, ...]) -> tuple[str, ...]:
    suspicious: list[str] = []
    for relative_path in paths:
        path = Path(relative_path)
        lower_parts = tuple(part.lower() for part in path.parts)
        lower_name = path.name.lower()
        is_environment_file = lower_name == ".env" or (
            lower_name.startswith(".env.") and lower_name != ".env.example"
        )
        if (
            any(part in SUSPICIOUS_DIRECTORY_NAMES for part in lower_parts)
            or lower_name in SUSPICIOUS_FILE_NAMES
            or lower_name.endswith(SUSPICIOUS_FILE_SUFFIXES)
            or is_environment_file
        ):
            suspicious.append(relative_path)
    return tuple(suspicious)


def read_bounded_text(path: Path, maximum_bytes: int, label: str) -> str:
    try:
        size = path.stat().st_size
    except OSError as error:
        raise GateError(f"{label} could not be inspected", ExitCode.IO_ERROR) from error
    if size > maximum_bytes:
        fail(f"{label} exceeds its size limit")
    try:
        return path.read_text(encoding="utf-8")
    except (OSError, UnicodeError) as error:
        raise GateError(f"{label} could not be read", ExitCode.IO_ERROR) from error


def read_hook_payload(stream: BinaryIO) -> Any:
    payload = stream.read(MAX_HOOK_INPUT_BYTES + 1)
    if len(payload) > MAX_HOOK_INPUT_BYTES:
        return None
    try:
        return json.loads(payload.decode("utf-8"))
    except (UnicodeDecodeError, json.JSONDecodeError):
        return None
