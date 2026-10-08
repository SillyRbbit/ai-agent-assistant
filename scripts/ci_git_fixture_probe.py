#!/usr/bin/env python3
"""Fixed, one-push PR139 runner diagnostic. Not a product fixture repair."""
from __future__ import annotations

import hashlib
import json
import os
import re
import stat
import subprocess
import sys
from pathlib import Path

PARENT = "9c2a371b61c38504d3d8c564cf41c9e435f1d8aa"
BRANCH = "refs/heads/codex/isolated-action-publication"
FIXTURE_HASH = "2ca30bd1bb9e79d61005ad703ceba40ef48ca7e864d3f75d3995c8781afee686"
GIT = "/usr/bin/git"
GIT_OPTIONS = [
    "-c", "user.name=QA Fixture",
    "-c", "user.email=qa@example.invalid",
    "-c", "core.hooksPath=/dev/null",
    "-c", "commit.gpgsign=false",
]
BASE_ENV = {
    "PATH": "/usr/bin:/bin",
    "GIT_CONFIG_NOSYSTEM": "1",
    "GIT_CONFIG_GLOBAL": "/dev/null",
}
STAGES = frozenset({
    "preflight", "executable", "fixture_init_baseline", "fixture_init_private_home", "result",
})
OUTCOMES = frozenset({
    "context_rejected", "source_rejected", "preflight_failed", "executable_bound",
    "fixture_access_failed", "already_used", "exited", "spawn_failed", "timed_out",
    "cleanup_unresolved", "baseline_not_reproduced", "baseline_unexpected",
    "comparison_complete", "comparison_stopped",
})


def emit(stage: str, outcome: str, exit_status: int | None = None,
         executable_sha256: str | None = None) -> None:
    """Only closed labels, bounded numeric exits and a validated hash can leave."""
    if stage not in STAGES or outcome not in OUTCOMES:
        raise ValueError("invalid fixed diagnostic category")
    if exit_status is not None and (type(exit_status) is not int or not -128 <= exit_status <= 255):
        raise ValueError("invalid exit status")
    row = {"stage": stage, "outcome": outcome, "exit_status": exit_status}
    if executable_sha256 is not None:
        if re.fullmatch(r"[0-9a-f]{64}", executable_sha256) is None:
            raise ValueError("invalid executable hash")
        row["executable_sha256"] = executable_sha256
    print(json.dumps(row, sort_keys=True), flush=True)


def valid_context(values: dict[str, str]) -> bool:
    fixed = {
        "GITHUB_ACTIONS": "true", "GITHUB_EVENT_NAME": "push",
        "GITHUB_REPOSITORY": "SillyRbbit/ai-agent-assistant", "GITHUB_REF": BRANCH,
        "GITHUB_RUN_ATTEMPT": "1", "PROBE_BEFORE": PARENT,
        "RUNNER_NAME": "Henrys-MacBook-Pro", "RUNNER_OS": "macOS", "RUNNER_ARCH": "X64",
    }
    return (all(values.get(k) == v for k, v in fixed.items())
            and re.fullmatch(r"[1-9][0-9]{0,19}", values.get("GITHUB_RUN_ID", "")) is not None
            and re.fullmatch(r"[0-9a-f]{40}", values.get("GITHUB_SHA", "")) is not None
            and values.get("GITHUB_SHA") != PARENT)


def source_bound(repo: Path, sha: str) -> bool:
    fixture = repo / "src-tauri/src/isolated_action/tests.rs"
    if fixture.is_symlink() or hashlib.sha256(fixture.read_bytes()).hexdigest() != FIXTURE_HASH:
        return False
    # Fixed metadata query; output never printed or persisted. No fixture execution.
    result = subprocess.run(
        [GIT, *GIT_OPTIONS, "rev-parse", "HEAD", "HEAD^"], cwd=repo,
        env=dict(BASE_ENV), stdin=subprocess.DEVNULL, stdout=subprocess.PIPE,
        stderr=subprocess.DEVNULL, timeout=10, check=False,
    )
    return result.returncode == 0 and result.stdout == (sha + "\n" + PARENT + "\n").encode("ascii")


def private_fixtures(temp: Path, run_id: str) -> tuple[Path, Path, Path]:
    if not temp.is_absolute() or temp.is_symlink() or not temp.is_dir():
        raise ValueError("invalid fixture parent")
    if re.fullmatch(r"[1-9][0-9]{0,19}", run_id) is None:
        raise ValueError("invalid run identity")
    root = temp.resolve() / ("cortexa-pr139-git-probe-" + run_id)
    # Exclusive persistent claim for this job. Never remove or reuse a prior root.
    root.mkdir(mode=0o700)
    if stat.S_IMODE(root.stat().st_mode) != 0o700 or root.stat().st_uid != os.getuid():
        raise ValueError("invalid fixture ownership")
    result = tuple(root / name for name in ("baseline", "variant", "private-home"))
    for path in result:
        path.mkdir(mode=0o700)
    return result


def git_init(root: Path, home: Path | None = None) -> tuple[str, int | None]:
    env = dict(BASE_ENV)
    if home is not None:
        env["HOME"] = str(home)
    try:
        child = subprocess.Popen(
            [GIT, *GIT_OPTIONS, "init", "-q"], cwd=root, env=env,
            stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
        )
    except OSError:
        return "spawn_failed", None
    try:
        return "exited", child.wait(timeout=10)
    except subprocess.TimeoutExpired:
        # Only the retained owned child; never a name/ambient PID search or force kill.
        if child.poll() is None:
            try:
                child.terminate()
            except OSError:
                return "cleanup_unresolved", None
        try:
            status = child.wait(timeout=5)
        except subprocess.TimeoutExpired:
            return "cleanup_unresolved", None
        return "timed_out", status


def compare(baseline: Path, variant: Path, home: Path) -> int:
    outcome, status = git_init(baseline)
    emit("fixture_init_baseline", outcome, status)  # Record before evaluation.
    if outcome != "exited":
        emit("result", "comparison_stopped")
        return 1
    if status == 0:
        emit("result", "baseline_not_reproduced")
        return 0
    if status != 1:
        emit("result", "baseline_unexpected")
        return 1
    outcome, status = git_init(variant, home)
    emit("fixture_init_private_home", outcome, status)
    if outcome != "exited" or status not in (0, 1):
        emit("result", "comparison_stopped")
        return 1
    # A completed comparison is diagnostic evidence, not passing product CI.
    emit("result", "comparison_complete")
    return 0


def main() -> int:
    # Read only named, nonsecret workflow context fields; never enumerate environments.
    keys = ("GITHUB_ACTIONS", "GITHUB_EVENT_NAME", "GITHUB_REPOSITORY", "GITHUB_REF",
            "GITHUB_RUN_ATTEMPT", "PROBE_BEFORE", "RUNNER_NAME", "RUNNER_OS",
            "RUNNER_ARCH", "GITHUB_RUN_ID", "GITHUB_SHA")
    values = {k: os.environ.get(k, "") for k in keys}
    if len(sys.argv) != 1 or not valid_context(values):
        emit("preflight", "context_rejected")
        return 1
    repo = Path(__file__).resolve().parents[1]
    try:
        if not source_bound(repo, values["GITHUB_SHA"]):
            emit("preflight", "source_rejected")
            return 1
        git_hash = hashlib.sha256(Path(GIT).read_bytes()).hexdigest()
        emit("executable", "executable_bound", executable_sha256=git_hash)
    except (OSError, ValueError, subprocess.SubprocessError):
        emit("preflight", "preflight_failed")
        return 1
    try:
        fixtures = private_fixtures(Path(os.environ.get("RUNNER_TEMP", "")), values["GITHUB_RUN_ID"])
    except FileExistsError:
        emit("preflight", "already_used")
        return 1
    except (OSError, ValueError):
        emit("preflight", "fixture_access_failed")
        return 1
    return compare(*fixtures)


if __name__ == "__main__":
    sys.exit(main())
