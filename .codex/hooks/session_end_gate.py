#!/usr/bin/env python3
"""Inspect repository state for Cortexa's explicit session-end workflow."""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import Any

from common import (
    ExitCode,
    GateError,
    conflicted_paths,
    repository_root,
    staged_paths,
    unstaged_paths,
    untracked_paths,
)


def inspect_repository(root: Path) -> dict[str, Any]:
    conflicts = list(conflicted_paths(root))
    return {
        "conflicted_paths": conflicts,
        "has_conflicts": bool(conflicts),
        "staged_paths": list(staged_paths(root)),
        "unstaged_paths": list(unstaged_paths(root)),
        "untracked_paths": list(untracked_paths(root)),
    }


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--cwd",
        default=".",
        help="directory inside the repository to inspect",
    )
    return parser


def main(argv: list[str] | None = None) -> int:
    arguments = build_parser().parse_args(argv)
    try:
        root = repository_root(Path(arguments.cwd))
        snapshot = inspect_repository(root)
        json.dump(snapshot, sys.stdout, sort_keys=True)
        sys.stdout.write("\n")
        if snapshot["has_conflicts"]:
            return int(ExitCode.VALIDATION_FAILED)
    except GateError as error:
        print(f"session-end-gate: {error}", file=sys.stderr)
        return int(error.exit_code)
    return int(ExitCode.OK)


if __name__ == "__main__":
    raise SystemExit(main())
