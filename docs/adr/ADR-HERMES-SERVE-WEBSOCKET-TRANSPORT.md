# ADR: Hermes serve WebSocket transport for contained evaluation

Status: Rejected after isolated spike; historical D-080 evaluation record
Date: 2026-08-11
Decision owners: Project owner
Related accepted decisions: D-078, D-079, D-080, D-081
Related evidence:
[`HERMES_INTEGRATION_ASSESSMENT.md`](../architecture/HERMES_INTEGRATION_ASSESSMENT.md)
and [`HERMES_TRANSPORT_SPIKE.md`](../spikes/HERMES_TRANSPORT_SPIKE.md)

This ADR selected the transport for the now-completed isolated containment
spike. The spike returned FAIL / NO-GO at Milestone 0. This historical record
does not approve a production adapter, execute or install Hermes, add a
dependency, open a socket, use a provider, handle a credential, or change
application behavior.

Except for the outcome section and explicit subsequent-result notes, the
remaining decision language preserves the original D-080 evaluation contract
in historical present tense; it is not current approval or readiness.

## Outcome after spike

The owner-selected spike preserved in
[`HERMES_SERVE_WEBSOCKET_SPIKE.md`](../spikes/HERMES_SERVE_WEBSOCKET_SPIKE.md)
rejected this mechanism under the approved D-080 conditions because:

1. the supplied provenance did not content-manifest the complete virtual
   environment or the externally located Python runtime used by the launcher;
2. pinned source had no supported complete switch for update-prefetch,
   dotenv/managed-secret loading, credential keepalive, plugin discovery, skill
   synchronization, and a true zero-tool startup; and
3. the reviewed target-Mac `sandbox-exec` mechanism could not prove the exact
   dynamic-listener, package-manager execution, blanket Unix-socket, or
   detached-descendant membership and cleanup guarantees.

Those were explicit NO-GO conditions. No Hermes server, listener, WebSocket,
session, prompt, provider, credential, tool, or adapter was started. The result
rejects this evaluated transport/containment combination without claiming that
every possible future WebSocket implementation or Hermes release is unsafe.
It prevents `HermesAgentRuntime` implementation against this mechanism unless
an additive owner-approved decision supplies materially different evidence and
controls.

## Context

The completed raw TUI-gateway stdio spike established a NO-GO for that
production mechanism at the evaluated release. The gateway wire exists, but the
release exposes no supported public raw-gateway launcher, initial
version/capability negotiation, or gateway-shutdown RPC. The fixture proves
host-side framing and lifecycle mechanics only.

As of 2026-08-11, the latest official Nous Research source release inspected is:

- package/application version: `0.20.0`;
- calendar release tag: `v2026.8.3`;
- source commit: `3c27eb6234bf91b8ceee9e9071591b31e9b148cb`;
- release date: 2026-08-03; and
- official release:
  [Hermes Agent v0.20.0](https://github.com/NousResearch/hermes-agent/releases/tag/v2026.8.3).

`0.20.0` and `v2026.8.3` are distinct version identifiers for the same tagged
source artifact, not separate Hermes releases. The exact commit is the immutable
source reference used by this decision; mutable `main` or `latest` is not a
compatibility target.

At that tag, official upstream documentation describes
[`hermes serve`](https://github.com/NousResearch/hermes-agent/blob/v2026.8.3/website/docs/reference/cli-commands.md)
as the supported public headless backend used by Hermes Desktop. The
[programmatic-integration guide](https://github.com/NousResearch/hermes-agent/blob/v2026.8.3/website/docs/developer-guide/programmatic-integration.md)
documents the TUI gateway as JSON-RPC over stdio or WebSocket. The pinned
[`hermes_cli/web_server.py`](https://github.com/NousResearch/hermes-agent/blob/v2026.8.3/hermes_cli/web_server.py)
mounts and authenticates `/api/ws`, then delegates the connection to
[`tui_gateway.ws.handle_ws`](https://github.com/NousResearch/hermes-agent/blob/v2026.8.3/tui_gateway/ws.py),
which reuses the broad gateway dispatcher.

Headless does not mean conversation-only. The same authenticated surface can
reach configuration, CLI, approval, secret, subprocess, clipboard, attachment,
reload, and subagent methods. Hermes's configuration and allowlists are useful
defense in depth but are not the OS containment required for an untrusted model
runtime.

## Decision

Conditionally select a Rust-supervised managed local `hermes serve` child plus a
closed application-owned projection of the TUI-gateway JSON-RPC/WebSocket
protocol for the next external-runtime spike.

The spike may begin only after the accepted native `AgentRuntime` and
`NativeAgentRuntime` phase is verified complete with a valid post-increment
marker. A passing spike may inform a later Hermes-adapter plan; it does not make
that adapter Ready automatically.

### Evaluated launch shape

The planned contained launch shape is conceptually:

```text
validated pinned Hermes executable
  hermes serve --host 127.0.0.1 --port 0
```

The spike must not use a shell, search an untrusted `PATH`, interpolate user
input into a command, auto-install or auto-update Hermes, or hard-code a
user-specific path. The operator supplies an explicit candidate distribution
path. The spike validates its identity, version, tag/commit evidence, and a
manifest/digest covering the complete interpreter environment and Hermes
distribution before launch and refuses any mismatch. Hashing only the `hermes`
console script is not sufficient provenance.

The `[web]` and POSIX `[pty]` runtime support required by the public launcher and
embedded Chat socket must already be importable. Missing support is an
unavailable result, not permission to install packages. The pinned launcher can
otherwise invoke `tools.lazy_deps.ensure(..., prompt=False)`, and its gateway
import starts an update-check path. Before launch, the spike must prove the
exact extras are present, keep the candidate distribution read-only, apply
reviewed offline/no-update controls, deny package-manager child execution and
updater egress, and detect any artifact mutation. If those paths cannot be
suppressed and verified, the result is NO-GO. No Hermes distribution channel or
production packaging mechanism is selected by this ADR.

### Isolation and authentication

The spike must:

- reject any bind other than the numeric loopback address `127.0.0.1`;
- request operating-system-assigned port `0` and accept only a bounded numeric
  port reported by the exact child;
- generate a high-entropy per-launch backend token in trusted Rust, pass it only
  through the child environment as `HERMES_DASHBOARD_SESSION_TOKEN`, and use it
  only in the backend-owned WebSocket connection;
- keep the token, complete WebSocket URL, process environment, and raw headers
  out of the WebView, IPC, logs, errors, tests, reports, and audit text;
- use a newly created owner-only `HERMES_HOME`, temporary working directory,
  isolated cache/config/state paths, and an explicit environment allowlist;
- inherit no provider keys, proxy variables, shell startup state, global Hermes
  profile, `.env`, sessions, memory, skills, plugins, MCP configuration, cron,
  messaging configuration, repository path, or application database;
- enumerate every dotenv/managed-secret source computed by the pinned
  `hermes_cli.env_loader`; reject a candidate containing a distribution-root
  `.env`, keep the new isolated home free of `.env` and `.op.env`, make the
  machine-managed dotenv location unreadable, disable all configured external
  secret sources, and treat any attempted dotenv/secret-manager access as
  NO-GO without reading or logging secret contents;
- never set `HERMES_DESKTOP=1` or enable a scheduler;
- place the entire process tree inside an OS-level boundary that denies host
  filesystem writes, repository and Git writes, Keychain/credential access,
  arbitrary process inspection, external networking, every loopback destination
  except the exact deterministic fake-provider endpoint, all Unix-domain socket
  access, and descendant escape; the separately authenticated inbound
  `127.0.0.1` serve listener is the only other socket allowance;
- track and terminate containment-wide membership, including descendants that
  create a new session or process group; a single parent PGID is not sufficient
  ownership or cleanup evidence; and
- treat configuration-based tool disablement as defense in depth only. Failure
  to prove the required OS restrictions is a NO-GO.

The token authenticates the local transport; it grants no Cortexa policy,
approval, execution, audit, or user identity authority.

### Readiness and protocol projection

The supervisor must bound and correlate each lifecycle stage:

1. validate the pinned candidate and spawn one owned containment domain with a
   membership ledger that survives descendant `setsid`/process-group changes;
2. parse only the exact bounded `HERMES_BACKEND_READY port=<N>` readiness line;
3. call `/api/health` on numeric loopback to verify liveness and exact Hermes
   software version, while treating it as insufficient provider/session health;
4. connect to
   `ws://127.0.0.1:<port>/api/ws?token=<per-launch-token>` and wait for the
   closed `gateway.ready` event, accepting only bounded `skin` and
   `change_events: true` fields from the pinned shape;
5. issue only application-owned JSON-RPC IDs and a minimal allowlist of
   `session.create`, `prompt.submit`, `session.status`, `session.history`,
   `session.interrupt`, and `session.close` as required by the spike;
6. accept only the narrowly modeled session results, asynchronous `session.info`
   after deferred session creation and turn finalization, `message.start` before
   text streaming, `message.delta`, `message.complete`, and closed failure/
   termination events required for one text-only turn; and
7. reject malformed, oversized, duplicate, unknown, out-of-sequence,
   wrong-session, wrong-ID, late, or privileged frames before they leave the
   spike boundary.

`/api/health` identifies liveness and software version. `gateway.ready`
identifies WebSocket protocol readiness. Neither is a protocol-version
negotiation, capability attestation, provider-readiness proof, or tool-disablement
proof. Compatibility therefore requires an exact release allowlist, pinned
fixtures, closed translation, and fail-closed handling of all drift.

The pinned `session.status` response is not a structured activity object. It
returns an `output` string containing the human-oriented line
`Agent Running: Yes|No`. The spike must isolate a bounded redacted parser that
accepts exactly one anchored occurrence, translates only that flag into a
closed application-owned state, immediately discards all other bounded output,
and fails closed on missing, duplicate, or changed text. Raw status output may
never leave the adapter boundary. This parsing fragility is an explicit
compatibility risk, not a production-ready protocol claim.

### Text, timeout, cancellation, and shutdown

The contained spike must demonstrate one basic session and text request/response
without live credentials or cloud actions. It may use only a deterministic
local fake model/provider endpoint inside the same containment boundary. If the
pinned Hermes artifact cannot complete a text turn under those restrictions,
the result is NO-GO rather than permission to add a live provider.

Text is bounded by bytes, event count, delta count, elapsed time, and one
terminal outcome. Tool calls or privileged event families are failures, not
requests for approval.

An ambiguous `prompt.submit` timeout must not be blindly retried. The host first
uses the isolated pinned-text `session.status` parser and bounded
`session.history` projection to reconcile whether Hermes accepted or completed
the turn. Unresolved ambiguity terminates the spike as a closed failure.

Cancellation sends `session.interrupt` once and treats its immediate
`{"status":"interrupted"}` result only as a cooperative acknowledgement. It
then performs bounded `session.status` reconciliation until the exact pinned
status parser yields `NotRunning`;
`message.complete` is useful when observed but is not guaranteed after an
interrupt. The host owns the local terminal decision, makes repeated cancel
requests idempotent, and rejects late output. Because agent-started background
processes can outlive the turn, shutdown closes the session and socket, then
terminates and reaps the entire validated containment membership with a
deadline and containment-wide force-cleanup primitive. A parent process-group
kill alone cannot prove cleanup of descendants that create new sessions. The
CLI's separate `--stop` command is not treated as an in-protocol shutdown
acknowledgement.

### Forbidden capabilities

The spike must neither send nor accept tool, approval, clarify, sudo, secret,
configuration, command, CLI, reload, MCP, memory, skill, plugin, subagent,
delegation, scheduling, messaging, webhook, voice, media, clipboard, attachment,
browser, shell, filesystem, Git, cloud, or device methods/events.

Any observed provider fallback, external network attempt, host file access,
state outside the isolated home, child escape, forbidden method/event, secret
request/source access, dotenv load, or inability to demonstrate tool absence
makes the spike NO-GO.

Even without `HERMES_DESKTOP=1`, the pinned FastAPI lifespan starts maintenance
tasks and the WebSocket path starts a change-watcher thread. The isolated home,
read-only artifact, network policy, bounded shutdown, and evidence collection
must account for those background activities rather than treating the launch as
an inert chat-only process.

## Alternatives

### Raw TUI-gateway stdio

Rejected for production at package/application version `0.20.0`, release tag
`v2026.8.3`, by the completed spike. The internal Python-module launcher is not
promoted to a public contract.

### Hermes ACP

At D-080's decision time, ACP was deferred as a documented fallback. It was
later evaluated separately and rejected by D-081 for the pinned release because
its normal sessions hardcode privileged internal tools without a supported
conversation-only mode or application-owned pre-execution gate.

### Direct WebView or application-service connection

Rejected. The WebView must never receive the token or raw Hermes payloads, and
general application services must not learn Hermes methods or configuration.

### OpenAI-compatible HTTP/SSE provider surface

Rejected for this runtime spike because it resembles provider transport, would
blur D-032/D-060 boundaries, and does not remove the broad Hermes process risk.

### Embedded Python

Rejected because it weakens crash, process, plugin, skill, and OS containment
and creates interpreter/ABI coupling inside the desktop process.

## Consequences

- The native runtime boundary remains the only Ready implementation plan.
- The WebSocket spike plan is owner-approved but Blocked until native completion.
- `HermesAgentRuntime` remains Draft/Blocked until the native boundary and
  contained spike pass, exact supported-version assumptions are recorded, the
  process tree is contained, and tool execution remains disabled.
- The selected evaluation adds listener, authentication, WebSocket, process,
  packaging, and local-attack-surface work that raw stdio would not have added.
- A failed spike leaves native fully functional and may lead to an ADR revision,
  a later ACP decision, or permanent native-only operation.
- No phase begins automatically and no document here is evidence that Hermes is
  currently installed, executable, compatible, contained, healthy, or safe.

## Spike acceptance conditions

The contained spike may report GO or CONDITIONAL GO only when it records:

- exact distribution identity, package version `0.20.0`, tag `v2026.8.3`, source
  commit, complete-runtime manifest/digest, and importable `[web]`/POSIX `[pty]`
  extras; inability to establish installed-artifact tag/commit provenance is
  NO-GO;
- proof that dependency-install and update-check paths are suppressed, package
  managers cannot execute, the candidate remains read-only, and no updater
  egress or artifact mutation occurs;
- proof that every pinned dotenv/managed-secret source is absent, disabled, or
  unreadable and that no dotenv, secret-manager, credential-store, subprocess,
  socket, or network access is attempted;
- loopback-only ephemeral binding and token-authenticated WebSocket access;
- bounded readiness, `/api/health`, and `gateway.ready` evidence;
- deterministic fake-server tests for lifecycle, protocol conversion,
  mandatory `session.info`/`message.start` events, pinned status-text parsing,
  authentication failure, incompatible version, timeout, cancellation,
  malformed frames, crash, stderr separation, redaction, and cleanup;
- an explicitly opt-in contained real-Hermes test that skips cleanly when the
  exact candidate is absent and never installs or updates it;
- one synthetic text-only session using no live credential or cloud provider;
- whole-process and descendant containment, including intentionally detached
  `setsid` descendants, with no prohibited host access;
- no tool or privileged method/event and no state outside the isolated home;
- clean shutdown, forced-cleanup evidence, and no orphan process or socket; and
- complete security, architecture, portability, packaging, and rollback review.

Any missing containment, tool-restriction, lifecycle, compatibility, or
credential boundary yields NO-GO and blocks the adapter plan.

## Rollback

This ADR changes documentation only. Reverting D-080 and this file returns the
transport to unselected while preserving D-079, the native runtime plan, and the
raw-stdio NO-GO evidence. A later failed spike removes only its disposable
harness, isolated runtime state, and synthetic fixtures; native behavior remains
unchanged.
