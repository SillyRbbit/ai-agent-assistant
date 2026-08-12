# Hermes serve WebSocket containment spike

Date: 2026-08-11
Verdict: **FAIL / NO-GO under the approved D-080 conditions**
Plan: `docs/plans/2026-08-11-hermes-serve-websocket-spike.md`

## Executive result

Milestone 0 disproved readiness before any Hermes process was started. The
operator-supplied candidate consistently identifies Hermes Agent `0.20.0`, tag
`v2026.8.3`, commit
`3c27eb6234bf91b8ceee9e9071591b31e9b148cb`, and contains the required web and
PTY modules. However:

1. the supplied provenance does not cover the complete virtual environment or
   the externally located Python runtime used by its launcher;
2. the pinned release has no supported startup mode that suppresses all update,
   dotenv/managed-secret, credential-keepalive, plugin, skill, and privileged
   default-tool initialization; and
3. target-Mac review found that deprecated `sandbox-exec` alone cannot meet the
   plan's exact dynamic-listener, package-manager execution, Unix-socket, or
   detached-descendant containment guarantees.

These are explicit stop conditions. No `hermes serve` command, HTTP listener,
WebSocket, session, prompt, model/provider request, tool, credential, or child
process was exercised. No fixture implementation was created merely to force a
positive result.

## Evaluated candidate and provenance

The exact operator-supplied absolute path is deliberately omitted from this
repository evidence because it contains the local user name. Relative to the
supplied distribution root, the validated executable was:

```text
hermes-agent/.venv/bin/hermes
```

It is a regular non-symlink file, mode `-r-x--x--x`, size 386 bytes. The source
checkout is detached and clean, its origin is the official NousResearch
repository, and read-only Git inspection established:

- package/application version: `0.20.0`;
- exact release tag: `v2026.8.3`;
- exact commit: `3c27eb6234bf91b8ceee9e9071591b31e9b148cb`;
- source archive SHA-256:
  `1e9319c58a7f5e95808546af1091d58472be7437adc63fae0cbb53316e2711aa`;
- `pyproject.toml` SHA-256:
  `64d1085ee1c23caf0ae0d9e65c73e280f466362ed43fdda1531f18f3af1d9869`;
- `uv.lock` SHA-256:
  `aab3c83f71b683507a590b6315b23bdc0abd6b63b76b2349eae15bf00dfbaf2b`;
- console launcher SHA-256:
  `6ba08e9b84ce23d42175b873a18bad7ebeeb1bfb782fdbbf57400b0fb2abaa0c`.

The 61 recorded installed-package name/version pairs exactly match the 61
installed distribution metadata records, including the editable Hermes source
mapping. The annotated upstream tag contains an SSH signature, but the local
machine has no configured allowed-signers trust file, so this review does not
claim independent signature trust.

### Provenance blocker

The virtual environment contains 4,077 regular files and three symlinks. The
supplied manifest hashes only the console launcher inside that environment.
The launcher resolves through `.venv/bin/python` to a Python `3.13.15` runtime
outside the supplied distribution; that interpreter was owner-writable when
observed and had SHA-256
`4e1dfb03f82c5f7f253bbc3c04a79bb9f09a5cd0528829c32d6984ef309ebb2f`.
Neither it, its standard library, nor the remaining environment contents are in
the supplied content manifest. The distribution's enclosing directory is also
owner-writable, so a complete immutable runtime identity and replacement-race
proof were not established.

No repair, installation, update, permission change, or candidate mutation was
attempted.

## Dependency and import preflight

A single candidate-Python `-I` metadata/module-discovery preflight ran under an
empty environment with a fresh disposable home and did not import the Hermes
server. It found:

| Module/package              | Result       |
| --------------------------- | ------------ |
| `hermes-agent`              | `0.20.0`     |
| `fastapi`                   | `0.133.1`    |
| `uvicorn`                   | `0.41.0`     |
| `starlette`                 | `1.3.1`      |
| `python-multipart`          | `0.0.32`     |
| `websockets`                | `15.0.1`     |
| `pty`                       | discoverable |
| `ptyprocess`                | `0.7.0`      |
| `hermes_cli`, `tui_gateway` | discoverable |

The environment was cleared first and allowlisted only fresh `HOME`,
`HERMES_HOME`, `TMPDIR`, XDG cache/config/data roots,
`PYTHONDONTWRITEBYTECODE=1`, `PYTHONNOUSERSITE=1`, `PYTHONSAFEPATH=1`, and a
system-only `PATH`. It contained no inherited provider credential, proxy, or
normal Hermes-profile variable. The disposable directory remained empty and
was removed.

Critical hashes and clean Git state passed again after this preflight. No file
newer than the supplied provenance record was observed inside the candidate.

## Pinned startup and capability findings

Static source inspection confirmed that a credential-free deterministic
loopback OpenAI-compatible fake provider is technically configurable with a
custom provider, explicit loopback `base_url`, chat-completions mode, fixed
model name, and explicit context length. The pinned runtime supplies a
`no-key-required` sentinel for that local custom-provider shape.

That useful transport fact is outweighed by these startup blockers:

- `tui_gateway/server.py` unconditionally begins update prefetch. The pinned
  implementation can execute `git fetch origin main` and write an update-check
  record; there is no supported disable flag.
- `hermes_cli/main.py` and `tui_gateway/server.py` call the Hermes dotenv loader.
  It probes the isolated home `.env` and `.op.env`, source-root `.env`,
  configured external secret sources, and the managed directory (by default
  `/etc/hermes`). Existing dotenv files may be sanitized and rewritten.
- `hermes_cli/web_server.py` starts Nous credential keepalive unconditionally;
  its delayed work probes provider/credential state.
- `serve` synchronizes bundled skills and performs plugin discovery. Marker and
  configuration workarounds can reduce effects but do not provide a supported
  complete disable mode.
- the default `hermes-cli` toolset includes terminal/process, filesystem,
  memory, skill, code, delegation/subagent, cron, and computer-use capability;
  an empty configured toolset does not mean a closed zero-tool set.
- missing web imports trigger an in-process lazy dependency installer. Imports
  are present in this candidate, but the code path remains part of the pinned
  compatibility/security surface.

An OS deny could limit side effects, but it would not support the approved
claim that these update, credential, plugin, and privileged-capability
initializations do not occur. Therefore the real process was not started.

## Containment review

The target Mac had Apple-signed `/usr/bin/sandbox-exec`; its own manual marks it
deprecated. A deny-default policy can usefully restrict ordinary filesystem
writes, named socket destinations, non-loopback networking, and known Keychain
Mach services, and policy inheritance continues after fork, exec, process-group
changes, and `setsid`.

It was not sufficient as the plan's sole containment boundary:

- it exposes no containment identity, complete membership enumeration,
  containment-wide termination, or descendant-reap primitive;
- process-group cleanup misses a descendant that calls `setsid`;
- a static profile cannot learn a port-zero child's selected port and then
  permit only that inbound listener;
- process execution rules filter executable paths, not arguments, so allowing
  Python cannot separately deny `python -m pip` or in-process updater code;
- anonymous `AF_UNIX` socket pairs remained available under the reviewed
  deny-default behavior, so blanket Unix-domain socket denial was not proved;
- Keychain denial remained only a promising design until a dedicated
  `SecItemCopyMatching` negative probe could run inside an otherwise eligible
  boundary.

No containment profile was applied to Hermes. No listener, socket, descendant,
process-group shutdown, or detached-child cleanup result is claimed.

## Protocol evidence

Pinned source confirms the selected public shape:

```text
hermes serve --host 127.0.0.1 --port 0 --isolated
HERMES_BACKEND_READY port=<actual>
GET /api/health
ws://127.0.0.1:<port>/api/ws?token=<per-launch-token>
```

It also confirms:

- `HERMES_DASHBOARD_SESSION_TOKEN` controls the per-launch WebSocket token;
- the first accepted gateway event is `gateway.ready`;
- `session.create` establishes the session;
- `prompt.submit` returns an immediate streaming acknowledgement, followed by
  `message.start`, `message.delta`, and `message.complete` events;
- `session.status` exposes the run flag only inside human-formatted output;
- `session.interrupt` acknowledges interruption but does not terminate
  agent-started background processes; and
- `session.close` may invoke agent/plugin cleanup.

These facts were source-inspected, not dynamically demonstrated. No handshake,
JSON-RPC method, event, cancellation, timeout, crash, or shutdown test ran.

## Validation results

- source/tag/commit and clean read-only Git checks: passed;
- source archive and three critical-file digest checks: passed before and after
  import discovery;
- installed package inventory reconciliation: passed, 61/61;
- sanitized required-module discovery: passed;
- complete interpreter/runtime manifest: failed;
- no-update/no-credential/no-plugin/zero-tool startup controls: failed;
- exact whole-process containment selection: failed;
- deterministic fake WebSocket fixture tests: not created or run because the
  Milestone 0 stop condition fired;
- opt-in real-Hermes test: not created or run;
- existing raw-stdio fixture regression: 7 passed, 0 failed, 1 ignored;
- existing native runtime contract regression: 20 passed, 0 failed;
- repository documentation, repository-health, security, diff, session-end,
  quality, and post-increment closeout checks are recorded in the associated
  post-increment review.

## Failure modes and portability/packaging concerns

- A version-only package inventory cannot replace a complete immutable runtime
  artifact and interpreter manifest.
- The candidate's external Python runtime and editable source mapping make
  replacement-race and packaging boundaries broader than the distribution
  root.
- Reliance on deprecated private-profile `sandbox-exec` is not a portable or
  sufficient production containment strategy.
- Port-zero readiness and a static socket policy conflict when the policy must
  allow exactly one dynamically selected inbound port.
- The gateway's broad method/event surface, default toolset, status-text parser,
  delayed credential work, maintenance tasks, and plugin cleanup remain pinned
  compatibility and security liabilities.
- Linux/container evidence would not by itself prove the target-Mac desktop
  packaging and containment model.

## Recommendation and decision gate

The managed `hermes serve` plus TUI-gateway WebSocket transport is **NO-GO for
the approved contained spike as currently specified**. Do not approve or begin
`HermesAgentRuntime`; its ExecPlan remains Draft/Blocked. Do not switch to ACP,
raw stdio, OpenAI-compatible HTTP as the application transport, or in-process
Python by inference.

The smallest responsible follow-up is an owner-approved documentation/ADR plan
amendment. It should require both:

1. an exact immutable, fully content-manifested Hermes distribution including
   its interpreter, with supported explicit controls for no update, no
   credential keepalive, no dotenv/managed secrets, no plugins/skills, and a
   true zero-tool mode; and
2. a separately reviewed target-Mac containment mechanism that can enforce the
   exact dynamic network policy and enumerate, terminate, and verify all
   descendants, including detached processes.

If those controls require a patched Hermes build or a new containment helper,
their provenance, source diff, packaging, privilege, rollback, and adversarial
tests need a new owner-approved plan. ACP may be reconsidered only through a
separate owner decision.

## Production and cleanup confirmation

No production file, dependency, manifest, lockfile, Tauri capability, UI,
provider, model, credential, process, socket, or runtime selector changed.
`NativeAgentRuntime` remains the sole/default implementation. The temporary
preflight home was removed. Prompt 4D and `HermesAgentRuntime` implementation
were not started.
