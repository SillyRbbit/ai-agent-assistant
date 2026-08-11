#!/usr/bin/env python3
"""Deterministic fake for the isolated Hermes transport spike.

This fixture is not Hermes and must never be used as conformance evidence.
It performs no network, tool, shell, model, memory, or filesystem operation.
"""

from __future__ import annotations

import json
import os
import sys
import time
from typing import Any


SESSION_ID = "session-spike-1"
EXPECTED_VERSION = "Hermes Agent v0.20.0 (2026.8.3)"


def emit(value: dict[str, Any]) -> None:
    sys.stdout.write(json.dumps(value, separators=(",", ":")) + "\n")
    sys.stdout.flush()


def emit_raw(value: bytes) -> None:
    sys.stdout.buffer.write(value)
    sys.stdout.buffer.flush()


def result(request_id: int, value: dict[str, Any]) -> None:
    emit({"jsonrpc": "2.0", "id": request_id, "result": value})


def event(event_type: str, payload: dict[str, Any], session_id: str = SESSION_ID) -> None:
    emit(
        {
            "jsonrpc": "2.0",
            "method": "event",
            "params": {
                "type": event_type,
                "session_id": session_id,
                "payload": payload,
            },
        }
    )


def ready() -> None:
    cwd = os.getcwd()
    isolated_paths = all(
        os.path.realpath(os.environ.get(key, "")) == os.path.realpath(cwd)
        for key in (
            "HOME",
            "TMPDIR",
            "XDG_CONFIG_HOME",
            "XDG_CACHE_HOME",
            "XDG_DATA_HOME",
        )
    )
    emit(
        {
            "jsonrpc": "2.0",
            "method": "event",
            "params": {
                "type": "gateway.ready",
                "payload": {
                    "skin": "fixture",
                    "change_events": True,
                    "env_keys": sorted(os.environ.keys()),
                    "isolated_paths": isolated_paths,
                    "cwd": cwd,
                },
            },
        }
    )


def read_request() -> dict[str, Any] | None:
    line = sys.stdin.buffer.readline()
    if not line:
        return None
    return json.loads(line)


def expect_request(method: str) -> dict[str, Any]:
    request = read_request()
    if request is None or request.get("method") != method:
        sys.exit(91)
    return request


def begin_session() -> None:
    create = expect_request("session.create")
    result(
        int(create["id"]),
        {
            "session_id": SESSION_ID,
            "info": {"desktop_contract": 5},
        },
    )


def begin_prompt() -> None:
    prompt = expect_request("prompt.submit")
    result(int(prompt["id"]), {"status": "streaming"})
    event("message.start", {})


def finish_success() -> None:
    event("message.delta", {"text": "fixture response"})
    event("message.complete", {"text": "fixture response", "status": "completed"})
    close = expect_request("session.close")
    result(int(close["id"]), {"status": "closed"})
    # The raw gateway has no shutdown RPC. The host closes stdin after this
    # response, and the fixture exits when it observes EOF.
    while read_request() is not None:
        pass


def run_interactive(scenario: str) -> None:
    if scenario == "hang_before_ready":
        time.sleep(30)
        return
    if scenario == "early_exit":
        sys.stderr.write("fixture-private-stderr-sentinel\n")
        sys.stderr.flush()
        sys.exit(23)

    ready()

    if scenario == "malformed":
        emit_raw(b"{not-json\n")
        return
    if scenario == "invalid_utf8":
        emit_raw(b"\xff\n")
        return
    if scenario == "oversized":
        emit_raw(b'{"padding":"' + (b"x" * 9000) + b'"}\n')
        return
    if scenario == "forbidden":
        event("cli.exec", {"command": "never-run"})
        return
    if scenario == "unknown":
        event("future.unknown", {})
        return
    if scenario == "wrong_identity":
        event("message.delta", {"text": "wrong"}, session_id="session-other")
        return

    if scenario == "noisy_stderr":
        sys.stderr.write("fixture-private-stderr-sentinel:" + ("z" * 4096) + "\n")
        sys.stderr.flush()

    begin_session()
    begin_prompt()

    if scenario == "hang_after_start":
        time.sleep(30)
        return
    if scenario == "midstream_exit":
        event("message.delta", {"text": "partial-only"})
        sys.stderr.write("fixture-private-stderr-sentinel\n")
        sys.stderr.flush()
        sys.exit(24)
    if scenario == "cancel":
        interrupt = expect_request("session.interrupt")
        result(int(interrupt["id"]), {"status": "interrupted"})
        event("message.complete", {"text": "", "status": "interrupted"})
        event("message.delta", {"text": "late-output-must-be-rejected"})
        while read_request() is not None:
            pass
        return

    finish_success()


def main() -> None:
    if len(sys.argv) != 2:
        sys.exit(64)
    mode = sys.argv[1]
    if mode == "--version":
        print(EXPECTED_VERSION, flush=True)
        return
    if mode == "--wrong-version":
        print("Hermes Agent v9.9.9 (2099.9.9)", flush=True)
        return
    run_interactive(mode)


if __name__ == "__main__":
    main()
