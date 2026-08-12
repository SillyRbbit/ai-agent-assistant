#!/usr/bin/env python3
"""Deterministic ACP stdio fixture for the isolated Hermes ACP spike.

This is not Hermes and is not ACP conformance or containment evidence. It uses
no network, provider, model, tool, credential, memory, package manager, or shell.
"""

from __future__ import annotations

import json
import os
import sys
import time
from typing import Any


SESSION_ID = "acp-session-fixture-1"
VERSION = "0.20.0"


def emit(value: dict[str, Any]) -> None:
    sys.stdout.write(json.dumps(value, separators=(",", ":")) + "\n")
    sys.stdout.flush()


def emit_raw(value: bytes) -> None:
    sys.stdout.buffer.write(value)
    sys.stdout.buffer.flush()


def response(request_id: int, result: Any) -> None:
    emit({"jsonrpc": "2.0", "id": request_id, "result": result})


def error_response(request_id: int, code: int, message: str) -> None:
    emit(
        {
            "jsonrpc": "2.0",
            "id": request_id,
            "error": {"code": code, "message": message},
        }
    )


def update(session_id: str, kind: str, payload: dict[str, Any]) -> None:
    value = {"sessionUpdate": kind}
    value.update(payload)
    emit(
        {
            "jsonrpc": "2.0",
            "method": "session/update",
            "params": {"sessionId": session_id, "update": value},
        }
    )


def read_message() -> dict[str, Any] | None:
    line = sys.stdin.buffer.readline()
    if not line:
        return None
    return json.loads(line)


def require_request(method: str) -> dict[str, Any]:
    request = read_message()
    if request is None or request.get("method") != method:
        sys.exit(91)
    return request


def isolation_evidence() -> dict[str, Any]:
    cwd = os.getcwd()
    isolated_paths = all(
        os.path.realpath(os.environ.get(key, "")) == os.path.realpath(cwd)
        for key in (
            "HOME",
            "HERMES_HOME",
            "TMPDIR",
            "XDG_CONFIG_HOME",
            "XDG_CACHE_HOME",
            "XDG_DATA_HOME",
        )
    )
    return {
        "envKeys": sorted(os.environ.keys()),
        "cwd": cwd,
        "isolatedPaths": isolated_paths,
    }


def initialize() -> None:
    request = require_request("initialize")
    response(
        int(request["id"]),
        {
            "protocolVersion": 1,
            "agentInfo": {"name": "hermes-acp-fixture", "version": VERSION},
            "agentCapabilities": {
                "loadSession": False,
                "promptCapabilities": {"image": False},
                "sessionCapabilities": {},
            },
            "authMethods": [],
            "_meta": {"fixture": isolation_evidence()},
        },
    )


def new_session() -> None:
    request = require_request("session/new")
    response(int(request["id"]), {"sessionId": SESSION_ID})


def prompt_started() -> dict[str, Any]:
    request = require_request("session/prompt")
    update(
        SESSION_ID,
        "agent_message_chunk",
        {"content": {"type": "text", "text": "fixture response"}},
    )
    return request


def wait_for_eof() -> None:
    while read_message() is not None:
        pass


def run_interactive(scenario: str) -> None:
    if scenario == "hang_before_initialize":
        time.sleep(30)
        return
    if scenario == "early_exit":
        sys.stderr.write("fixture-private-stderr-sentinel\n")
        sys.stderr.flush()
        sys.exit(23)

    initialize()

    if scenario == "malformed":
        emit_raw(b"{not-json\n")
        return
    if scenario == "oversized":
        emit_raw(b'{"padding":"' + (b"x" * 9000) + b'"}\n')
        return
    if scenario == "unknown_method":
        request = read_message()
        if request is None:
            sys.exit(92)
        error_response(int(request["id"]), -32601, "method not found")
        wait_for_eof()
        return

    if scenario == "noisy_stderr":
        sys.stderr.write("fixture-private-stderr-sentinel:" + ("z" * 4096) + "\n")
        sys.stderr.flush()

    new_session()
    prompt = prompt_started()

    if scenario == "hang_after_update":
        time.sleep(30)
        return
    if scenario == "tool_update":
        update(
            SESSION_ID,
            "tool_call",
            {"toolCallId": "tool-fixture-1", "title": "must be rejected"},
        )
        return
    if scenario == "permission_request":
        emit(
            {
                "jsonrpc": "2.0",
                "id": 77,
                "method": "session/request_permission",
                "params": {
                    "sessionId": SESSION_ID,
                    "toolCall": {
                        "toolCallId": "permission-fixture-1",
                        "title": "must be rejected",
                    },
                    "options": [],
                },
            }
        )
        return
    if scenario == "wrong_session":
        update(
            "acp-session-other",
            "agent_message_chunk",
            {"content": {"type": "text", "text": "wrong session"}},
        )
        return
    if scenario == "wrong_id":
        response(999, {"stopReason": "end_turn"})
        return
    if scenario == "midstream_exit":
        sys.stderr.write("fixture-private-stderr-sentinel\n")
        sys.stderr.flush()
        sys.exit(24)
    if scenario == "cancel":
        cancellation = require_request("session/cancel")
        if "id" in cancellation:
            sys.exit(93)
        response(int(prompt["id"]), {"stopReason": "cancelled"})
        update(
            SESSION_ID,
            "agent_message_chunk",
            {"content": {"type": "text", "text": "late-output-must-be-rejected"}},
        )
        wait_for_eof()
        return

    response(int(prompt["id"]), {"stopReason": "end_turn"})
    wait_for_eof()


def main() -> None:
    if len(sys.argv) != 2:
        sys.exit(64)
    mode = sys.argv[1]
    if mode == "--version":
        print(VERSION, flush=True)
        return
    if mode == "--check":
        print("Hermes ACP fixture check OK", flush=True)
        return
    if mode == "--hang-probe":
        time.sleep(30)
        return
    if mode == "--flood-probe":
        emit_raw(b"x" * 9000 + b"\n")
        return
    run_interactive(mode)


if __name__ == "__main__":
    main()
