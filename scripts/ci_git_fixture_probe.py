#!/usr/bin/env python3
"""Fixed, one-push PR139 runner diagnostic. Not a product fixture repair."""
from __future__ import annotations

import hashlib
import json
import os
import re
import stat
import selectors
import time
import subprocess
import sys
from pathlib import Path

PARENT = "24b65bb676775d3de12fe38c0ef26f401ef6dcc0"
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
    "metadata_baseline", "metadata_private_home",
})
OUTCOMES = frozenset({
    "context_rejected", "fixture_binding_rejected", "metadata_command_failed",
    "metadata_spawn_failed", "metadata_capture_failed", "metadata_wait_failed",
    "metadata_nonzero", "metadata_read", "metadata_stream_close_failed",
    "metadata_cleanup_reaped", "metadata_baseline_verified", "metadata_comparison_verified",
    "executable_drift",
    "metadata_malformed", "head_parent_mismatch", "metadata_timed_out",
    "metadata_cleanup_unresolved", "preflight_failed", "executable_bound",
    "metadata_stderr_limit", "stderr_empty", "stderr_unknown", "stderr_truncated",
    "stderr_capture_failed", "stderr_git_repository_signature", "stderr_git_ownership_signature",
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


def stop_metadata_child(child: subprocess.Popen) -> tuple[str, int | None]:
    """Cleanup is independent of the primary failure; only the retained child."""
    try:
        if child.poll() is None:
            child.terminate()
    except (OSError, ValueError):
        return "metadata_cleanup_unresolved", None
    try:
        return "metadata_cleanup_reaped", child.wait(timeout=5)
    except (OSError, ValueError, subprocess.TimeoutExpired):
        return "metadata_cleanup_unresolved", None


# Reviewed Git v2.47.0 setup.c/usage.c signatures; observations, not causes.
# https://github.com/git/git/blob/v2.47.0/setup.c
# https://github.com/git/git/blob/v2.47.0/usage.c
STDERR_LIMIT = 2048
REPOSITORY_SIGNATURE = b"fatal: not a git repository (or any of the parent directories): .git\n"
OWNERSHIP_PREFIX = b"fatal: detected dubious ownership in repository at '"


def classify_stderr(captured: bytearray) -> str:
    """Only reviewed anchored signatures; no dynamic values leave this function."""
    if not captured:
        return "stderr_empty"
    if captured == REPOSITORY_SIGNATURE:
        return "stderr_git_repository_signature"
    newline = captured.find(b"\n")
    if (captured.startswith(OWNERSHIP_PREFIX) and newline > len(OWNERSHIP_PREFIX)
            and captured[newline - 1] == ord("'")
            and all(32 <= captured[i] < 127 for i in range(newline))):
        return "stderr_git_ownership_signature"
    return "stderr_unknown"


def metadata(repo: Path) -> tuple:
    """One unchanged baseline query; raw stderr never escapes transient capture."""
    try:
        child = subprocess.Popen(
            [GIT, *GIT_OPTIONS, "rev-parse", "HEAD", "HEAD^"], cwd=repo,
            env=dict(BASE_ENV), stdin=subprocess.DEVNULL, stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
    except OSError:
        return "metadata_spawn_failed", None, b"", None, False, "stderr_capture_failed"
    deadline = time.monotonic() + 10
    buffers = {"stdout": bytearray(), "stderr": bytearray()}
    streams = {"stdout": child.stdout, "stderr": child.stderr}
    limits = {"stdout": 83, "stderr": STDERR_LIMIT + 1}
    outcome, status = "metadata_read", None
    cleanup, close_failed = None, False
    try:
        try:
            with selectors.DefaultSelector() as selector:
                for name, stream in streams.items():
                    selector.register(stream, selectors.EVENT_READ, data=name)
                pending = set(streams)
                while pending and outcome == "metadata_read":
                    remaining = deadline - time.monotonic()
                    if remaining <= 0:
                        raise subprocess.TimeoutExpired("fixed metadata", 10)
                    events = selector.select(remaining)
                    if not events:
                        raise subprocess.TimeoutExpired("fixed metadata", 10)
                    for key, _ in events:
                        name = key.data
                        chunk = os.read(streams[name].fileno(), limits[name] - len(buffers[name]))
                        if not chunk:
                            selector.unregister(streams[name])
                            pending.remove(name)
                            continue
                        buffers[name].extend(chunk)
                        if len(buffers[name]) == limits[name]:
                            outcome = "metadata_malformed" if name == "stdout" else "metadata_stderr_limit"
                            break
        except subprocess.TimeoutExpired:
            outcome = "metadata_timed_out"
        except (OSError, ValueError):
            outcome = "metadata_capture_failed"
        if outcome == "metadata_read":
            try:
                status = child.wait(timeout=max(0, deadline - time.monotonic()))
                if status != 0:
                    outcome = "metadata_nonzero"
            except subprocess.TimeoutExpired:
                outcome = "metadata_timed_out"
            except (OSError, ValueError):
                outcome = "metadata_wait_failed"
        if outcome not in ("metadata_read", "metadata_nonzero"):
            cleanup = stop_metadata_child(child)
        for stream in streams.values():
            try:
                stream.close()
            except (OSError, ValueError):
                close_failed = True
        if close_failed:
            category = "stderr_capture_failed"
        elif outcome == "metadata_stderr_limit":
            category = "stderr_truncated"
        elif outcome in ("metadata_read", "metadata_nonzero"):
            category = classify_stderr(buffers["stderr"])
        else:
            category = "stderr_capture_failed"
        output = bytes(buffers["stdout"]) if outcome == "metadata_read" and not close_failed else b""
        return outcome, status, output, cleanup, close_failed, category
    finally:
        # Best-effort clearing, not a guarantee against Python/OS memory retention.
        for captured in buffers.values():
            for i in range(len(captured)):
                captured[i] = 0
            captured.clear()


def source_bound(output: bytes, sha: str) -> str:
    if re.fullmatch(rb"[0-9a-f]{40}\n[0-9a-f]{40}\n", output) is None:
        return "metadata_malformed"
    if output != (sha + "\n" + PARENT + "\n").encode("ascii"):
        return "head_parent_mismatch"
    return "source_bound"


def fixture_bound(repo: Path) -> bool:
    fixture = repo / "src-tauri/src/isolated_action/tests.rs"
    try:
        return not fixture.is_symlink() and hashlib.sha256(fixture.read_bytes()).hexdigest() == FIXTURE_HASH
    except OSError:
        return False


def private_metadata_home(temp: Path, run_id: str) -> Path:
    """An exclusive diagnostic claim, not a Git fixture or a personal HOME."""
    if not temp.is_absolute() or temp.is_symlink() or not temp.is_dir():
        raise ValueError("invalid metadata parent")
    if re.fullmatch(r"[1-9][0-9]{0,19}", run_id) is None:
        raise ValueError("invalid run identity")
    root = temp.resolve() / ("cortexa-pr139-metadata-" + run_id)
    root.mkdir(mode=0o700)
    if stat.S_IMODE(root.stat().st_mode) != 0o700 or root.stat().st_uid != os.getuid():
        raise ValueError("invalid metadata ownership")
    home = root / "private-home"
    home.mkdir(mode=0o700)
    if home.is_symlink() or stat.S_IMODE(home.stat().st_mode) != 0o700 or home.stat().st_uid != os.getuid():
        raise ValueError("invalid private home")
    return home


def record_metadata(stage: str, result: tuple) -> None:
    outcome, status, _, cleanup, close_failed, category = result
    emit(stage, outcome, status)  # Primary result recorded before branching.
    emit(stage, category)
    if cleanup is not None:
        emit(stage, *cleanup)
    if close_failed:
        emit(stage, "metadata_stream_close_failed")


def executable_hash() -> str:
    return hashlib.sha256(Path(GIT).read_bytes()).hexdigest()


def metadata_only(repo: Path, sha: str, git_hash: str) -> int:
    result = metadata(repo)
    record_metadata("metadata_baseline", result)
    outcome, status, output, cleanup, close_failed, _ = result
    if executable_hash() != git_hash:
        emit("result", "executable_drift")
        return 1
    if outcome != "metadata_read" or status != 0 or cleanup is not None or close_failed:
        emit("result", "comparison_stopped")
        return 1
    binding = source_bound(output, sha)
    emit("result", "metadata_baseline_verified" if binding == "source_bound" else binding)
    return 0 if binding == "source_bound" else 1


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
    if not fixture_bound(repo):
        emit("preflight", "fixture_binding_rejected")
        return 1
    try:
        git_hash = executable_hash()
        emit("executable", "executable_bound", executable_sha256=git_hash)
        private_metadata_home(Path(os.environ.get("RUNNER_TEMP", "")), values["GITHUB_RUN_ID"])
    except FileExistsError:
        emit("preflight", "already_used")
        return 1
    except (OSError, ValueError):
        emit("preflight", "preflight_failed")
        return 1
    try:
        # This authorized one-push continuation ends here, before any Git-init case.
        return metadata_only(repo, values["GITHUB_SHA"], git_hash)
    except (OSError, ValueError, subprocess.SubprocessError):
        emit("preflight", "preflight_failed")
        return 1



if __name__ == "__main__":
    sys.exit(main())
