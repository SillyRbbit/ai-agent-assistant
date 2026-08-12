# Experimental HermesAgentRuntime adapter

Status: Draft; Blocked after WebSocket and ACP NO-GO results; no selected
Hermes transport
Owner: Project owner
Last updated: 2026-08-11

This is a future adapter ExecPlan, not an active or Ready implementation plan.
D-080's selected WebSocket evaluation and D-081's subsequent ACP evaluation
both returned NO GO for the pinned release. The plan has no eligible transport
and may not be revised into implementation work without a separate owner-
approved architecture decision, passing contained spike, and fresh readiness
review.

## Goal

Historical proposed goal: add an off-by-default experimental
`HermesAgentRuntime` as a second implementation of the application-owned
`AgentRuntime`. No current transport satisfies the prerequisite, so the
remaining WebSocket design below is non-executable historical planning evidence
that must be replaced by a separately approved decision and passing spike
before this Draft can be made Ready.

## User-visible outcome

The historical proposal planned no polished UI. Native would remain the
shipping default and continue to work independently. A future passing design
would allow a developer to explicitly select the experimental adapter through
a trusted backend-only configuration/harness and complete one bounded synthetic
text conversation. Returning to native would start a new explicit run.

## Scope

- Implement one Hermes adapter behind the exact D-079 `AgentRuntime` contract.
- Keep Hermes configuration, discovery, process supervision, WebSocket protocol,
  compatibility, parsing, and errors inside one adapter module.
- Support only discovery, version/compatibility, start/connect, readiness,
  health, one session, text request, streaming response, timeout reconciliation,
  cancellation, crash detection, and shutdown that a future contained spike
  would have to prove.
- Add a closed backend-owned runtime selection value whose default is Native;
  an explicit experimental selection is fixed before a run.
- Preserve the deterministic test-only `MockAgentRuntime` and all native/runtime
  contract tests.
- Use fake transport tests by default and one ignored opt-in contained
  integration test for the exact supported Hermes artifact.

## Explicit non-goals

- No React Hermes type/import, runtime-selection UI, raw Hermes Tauri command,
  WebView token, WebView WebSocket, or generic RPC relay.
- No automatic Hermes install, update, repair, bootstrap, model setup, provider
  fallback, runtime fallback, retry after ambiguous outcome, or global profile.
- No live provider credential or personal content in the initial adapter.
- No Hermes tool, approval, secret, MCP, memory, skill, plugin, subagent,
  schedule, messaging, webhook, voice, media, shell, filesystem, Git, browser,
  clipboard, cloud, or device action.
- No `ToolRegistry`, `PolicyEngine`, `ApprovalManager`, audit, memory, storage,
  credential, or `PlatformAdapter` responsibility inside `AgentRuntime`.
- No ACP, raw TUI-gateway stdio, direct HTTP/SSE provider surface, embedded
  Python, OpenClaw, public packaging, installer, signing, or release work.
- No production dependency until the spike identifies the exact minimal need
  and a separate owner-approved dependency review updates this plan.

## Existing behavior and constraints

- `AgentRuntime` and sole/default `NativeAgentRuntime` are verified complete;
  this Draft may not modify or dictate a broader native contract.
- D-080's managed local `hermes serve` plus TUI-gateway WebSocket evaluation is
  rejected after its Milestone 0 NO-GO. D-081 separately rejects ACP for the
  same pinned release. The adapter may not substitute another surface.
- Any future contained spike must prove the exact artifact, compatibility,
  process boundary, protocol projection, tool absence, and lifecycle.
- The pinned launcher can attempt lazy dependency installation and update-check
  traffic, and detached workers can create new sessions. The spike must prove
  read-only/offline launch and containment-wide cleanup before this Draft may
  reuse either behavior.
- Any future external text turn may use only a separately approved deterministic
  local fake provider. Live provider/credential/disclosure decisions remain
  separate.

## Current-state evidence

- D-079 accepts the native-first application-owned runtime architecture.
- The native runtime boundary is verified complete and published.
- D-080's WebSocket path and D-081's ACP path are both rejected by completed
  isolated spikes; neither may back this adapter.
- No production Hermes source, dependency, executable, runtime home, process,
  socket, credential, provider, or selector exists.

## Files expected to change

Provisional adapter scope, to be made exact after the prerequisites:

- `src-tauri/src/agent/hermes_runtime/mod.rs` (new)
- `src-tauri/src/agent/hermes_runtime/config.rs` (new)
- `src-tauri/src/agent/hermes_runtime/protocol.rs` (new)
- `src-tauri/src/agent/hermes_runtime/supervisor.rs` (new)
- `src-tauri/src/agent/runtime_selection.rs` (new, application-owned closed
  selection with Native default)
- `src-tauri/src/agent/mod.rs`
- `src-tauri/tests/hermes_agent_runtime_contract.rs` (new)
- the passing spike's deterministic fake server fixture, reused rather than
  copied if its contract remains suitable

Provisional architecture/closeout scope:

- `ARCHITECTURE.md`
- `PLANS.md`
- `HANDOFF.md`
- `PROJECT_STATUS.md`
- `NEXT_STEPS.md`
- `CHANGELOG.md`
- this plan
- `docs/increments/hermes-agent-runtime-adapter.md` (new)
- `docs/reviews/YYYY-MM-DD-hermes-agent-runtime-adapter-post-increment-review.md`
  (new)

No frontend, Tauri command/capability/configuration, storage, policy, approval,
tool, audit, credential, workflow, hook, skill, installer, or packaging path is
currently in scope. Before this plan can become Ready, replace the provisional
list with the exact implemented-contract paths and name every dependency,
manifest, lockfile, fixture, containment, and target-Mac evidence path. Any
additional path requires owner approval.

## Affected components

- Rust `agent` adapter boundary and application-owned runtime selection.
- Test-only fake transport and contract suites.
- Local contained child/process and loopback WebSocket lifecycle.

The general domain, frontend, Tauri IPC, tools, policy, approvals, audit,
storage, memory, credentials, and platform adapters remain independent.

## Interfaces and invariants

> **Historical non-executable design:** Every WebSocket-specific interface,
> lifecycle, protocol, containment, test, risk, rollback, and acceptance detail
> below preserves the rejected D-080 proposal for traceability. None is proved,
> selected, approved, or Ready. Milestone 0 requires this Draft to be rewritten
> from a new owner decision and passing spike before any implementation.

### Contract implementation

`HermesAgentRuntime` implements the exact already-verified `AgentRuntime`. It
may not add framework-specific methods to that contract. Any Hermes-only
operation remains private and is translated into closed application-owned
descriptor, capability, run, event, cancellation, terminal, and error types.

Only basic text lifecycle capability is initially declared. A capability
declaration is informational and never permission. A Hermes event that
contradicts the descriptor or requests an unmodeled capability terminates the
run.

### Adapter-local configuration

The WebSocket-specific controls in this section are the rejected D-080 design
record, not spike-proven inputs. They cannot be implemented or reused unless a
later owner-approved decision and passing spike establish replacement evidence.

`HermesRuntimeConfig` must validate:

- an explicit absolute candidate-distribution path plus a manifest/digest that
  covers its complete interpreter environment and Hermes distribution;
- exact supported package version, tag, commit, and importable `[web]`/POSIX
  `[pty]` extras;
- isolated runtime-home/cwd/cache/config/state constructors;
- separately proved offline/no-update controls, read-only artifact, package-manager
  child denial, updater-egress denial, mutation detection, and exact local fake-
  provider endpoint;
- a proved absence/denial policy for distribution-root `.env`, isolated-
  home `.env`/`.op.env`, the pinned machine-managed dotenv path, and configured
  external secret sources;
- bounded readiness, health, connect, turn, cancellation, shutdown, output,
  event, and diagnostic limits; and
- a separately approved whole-process containment command/profile.

The numeric loopback host and OS-assigned port are security invariants, not
user-selectable endpoints. The per-launch token is generated by trusted Rust,
held in a secret-bearing private type, passed only to the child and backend
WebSocket client, and never serialized, logged, cloned into errors, or exposed
over IPC. The private connector uses only
`ws://127.0.0.1:<port>/api/ws?token=<per-launch-token>` and never logs or returns
that complete URL.

No user-specific path, port, credential, provider/model name, development
directory, global `~/.hermes`, or mutable release channel is hard-coded.

### Lifecycle

The rejected WebSocket design proposed this closed lifecycle:

```text
Unavailable
  -> CandidateValidated
  -> Starting
  -> Ready
  -> SessionActive
  -> TurnActive
  -> Completed | Cancelled | Failed
  -> Stopping
  -> Stopped
```

Start, cancel, and shutdown bind to one application run ID, one adapter instance,
one Hermes session ID, one exact containment identity/membership ledger, and one
WebSocket. Membership and termination must include descendants that call
`setsid` or create another process group; a parent PGID is not sufficient. No
automatic restart or failover occurs. Crash, EOF, protocol error, health loss,
timeout ambiguity, containment loss, or forbidden activity becomes a typed
failure and forces bounded containment-wide cleanup.

The historical design sent `session.interrupt` once, treated its immediate
`{"status":"interrupted"}` result only as a cooperative acknowledgement, and
performed bounded `session.status` reconciliation until the private pinned-text
status parser yielded `NotRunning`. `message.complete` was not assumed after an
interrupt. The host owned the terminal result, made cancellation locally
idempotent, rejected late output, and cleaned up all containment members,
including background processes that could outlive the turn. A Hermes failure
would never crash the desktop. The caller could later make a new explicit
Native selection; the adapter would never replay or silently migrate the failed
run.

### Protocol isolation

Private adapter protocol models cover only:

- readiness and exact `/api/health` identity;
- `gateway.ready` with bounded `skin` and `change_events: true`;
- `session.create`, `prompt.submit`, `session.status`, `session.history`,
  `session.interrupt`, and `session.close`;
- correlated response envelopes; and
- the historically proposed asynchronous `session.info`, `message.start`, text delta,
  completion, cancellation, and closed failure events, with exact bounded fields
  and permitted interleavings.

The historical private `session.status` projection proposed a bounded
redacted parser for exactly one anchored `Agent Running: Yes|No` line inside the
human-oriented `output` string. It returns only a closed `Running | NotRunning`
state, immediately discards all other raw status text, and fails closed on
missing, duplicate, or changed text. No fixture may model a fictional structured
`running` field, and the parser may not become an application-owned domain type.

There is no generic `send_rpc`, arbitrary JSON, open method string, raw payload
return, or WebView-facing protocol type. Unknown fields, IDs, sessions,
methods/events, order, limits, versions, terminal states, or capabilities fail
closed. Adapter errors retain only closed categories and bounded non-sensitive
technical facts.

### Runtime selection

An application-owned closed `RuntimeSelection` contains only `Native` and
`ExperimentalHermes`. `Default` is `Native`. The experimental selection is
accepted only from trusted backend test/development configuration before a run;
the model and WebView cannot construct or change it. Selection is fixed for the
run and does not authorize automatic fallback.

The shipping application remains usable with Hermes absent, invalid,
incompatible, unhealthy, or crashed. Native construction cannot depend on any
Hermes module, artifact, state, token, process, or dependency initialization.

## Implementation milestones

- [ ] Milestone 0 - unblock and rewrite this Draft from evidence
  - require the valid native-runtime marker plus a separately selected transport
    with a passing spike; the completed WebSocket and ACP NO-GO markers cannot
    satisfy this condition;
  - require a GO or owner-accepted CONDITIONAL GO with no blocking containment,
    tool, credential, lifecycle, or compatibility finding;
  - freeze a newly proved complete-runtime manifest/digest, installed-artifact
    provenance, `[web]`/POSIX `[pty]` imports, offline/no-update controls,
    dotenv/managed-secret denial, endpoint policy, detached-descendant cleanup,
    and protocol fixtures;
  - select and review exact minimal Rust dependencies and complete manifest,
    lockfile, license, advisory, native-build, and packaging impact;
  - replace provisional paths with exact files and run fresh readiness review;
  - obtain owner implementation approval and begin one adapter gate.
- [ ] Milestone 1 - tests and adapter-local closed types
  - add fake transport, configuration, protocol conversion, unavailable,
    incompatible, timeout, cancellation, malformed, crash, and redaction tests;
  - prove Hermes types do not escape the adapter.
- [ ] Milestone 2 - supervisor and lifecycle
  - implement only newly proved spawn/readiness/health/session/text/cancel/
    shutdown projection and containment;
  - preserve exact cleanup and no-tool evidence.
- [ ] Milestone 3 - runtime implementation and selection
  - implement `AgentRuntime` without changing the shared contract;
  - add Native-default closed selection and explicit experimental test harness;
  - prove native works when Hermes is absent or fails.
- [ ] Milestone 4 - opt-in target-Mac evidence and stop
  - run ordinary fake tests without Hermes;
  - run separately approved contained synthetic integration against the exact
    candidate;
  - complete reviews and documentation;
  - stop without enabling tools or a polished UI.

## Security and privacy considerations

Hermes is an untrusted external planner/process. Whole-process containment,
backend-only token handling, isolated state, a scrubbed environment, closed
protocol translation, and bounded cleanup remain mandatory production-path
controls. Hermes configuration, approvals, allowlists, redaction, and logs do
not replace Cortexa policy or the OS boundary.

The boundary permits the authenticated inbound numeric-loopback serve listener
and Hermes egress only to the exact approved local fake-provider endpoint. It
denies external network, every other loopback destination, and all Unix-domain
sockets. It also keeps the candidate distribution read-only, blocks dependency-
installer children and updater egress, detects artifact mutation, and terminates
all containment members rather than relying on a single process group. The
pinned FastAPI maintenance tasks and WebSocket change-watcher remain inside
those controls.

Any future adapter must inherit a passing spike's explicit denial of every pinned
dotenv/managed-secret source: no distribution-root `.env`, no isolated-home
`.env`/`.op.env`, no readable machine-managed dotenv, and no configured external
secret source. Any attempted dotenv, secret-manager, credential-store,
subprocess, socket, or network access is a fatal unavailable/security error;
secret contents are never read, logged, or returned.

The adapter receives no live credential or personal content initially. Any
provider/network activation requires its own accepted provider, disclosure,
retention, secret, and operational-evidence decision. A tool or privileged
event is a fatal adapter error, never a call into Cortexa approval or execution.

## Test plan

Ordinary Hermes-free tests must cover:

- Native default selection and native behavior regression;
- explicit experimental selection without Hermes installed;
- executable missing, invalid config/path, wrong distribution/manifest,
  incompatible version, missing `[web]`/POSIX `[pty]`, attempted lazy install or
  update check, artifact mutation, unavailable gateway, failed health, and
  unhealthy runtime;
- distribution-root `.env`, isolated-home `.env`/`.op.env`, machine-managed
  dotenv, configured external secret source, and any attempted credential-source
  access without reading or emitting secret contents;
- exact protocol conversion and permitted interleavings for readiness, deferred/
  terminal `session.info`, `message.start`, text deltas, completion,
  cancellation, and closed failures;
- malformed/oversized/unknown/duplicate/out-of-order/wrong-ID/wrong-session/
  late frames and capability contradiction;
- start, connect, session, prompt, read, reconciliation, cancel, and shutdown
  timeouts, plus valid and missing/duplicate/changed
  `Agent Running: Yes|No` status text with raw-output redaction;
- cooperative interrupt acknowledgement, status reconciliation with and without
  `message.complete`, cancellation idempotence, crash/EOF before readiness and
  mid-turn, containment-wide cleanup of an intentionally detached descendant,
  socket closure, stderr separation, and secret redaction;
- no automatic retry/failover and explicit return to a new Native run;
- no tool, policy, approval, audit, memory, platform, credential, Tauri, React,
  or raw JSON dependency direction; and
- `MockAgentRuntime` and `NativeAgentRuntime` contract parity.

The opt-in integration test must be ignored by default, require the exact
operator-supplied contained candidate, fail after opt-in if prerequisites are
missing, use only fixed synthetic text and the approved local fake provider,
and prove no forbidden activity or leaked state.

## Verification commands

Exact commands remain Blocked on the implemented Phase A contract and selected
dependency set. Before Ready, this plan must name focused tests plus at minimum:

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
npm run verify
npm run security:scan
git diff --check
python3 .codex/hooks/session_end_gate.py
```

Required manual target-Mac containment and synthetic conversation commands must
be recorded verbatim before execution. No failed, skipped-required, or pending
manual gate can produce PASS.

## Risks

| Risk                                  | Severity | Mitigation / blocker                                                            |
| ------------------------------------- | -------- | ------------------------------------------------------------------------------- |
| External process gains host authority | Critical | Reuse only passing whole-process containment; any gap blocks adapter            |
| Hermes-specific concepts leak outward | High     | One private module, closed translations, dependency-direction tests/review      |
| Broad gateway method becomes executor | Critical | No generic RPC; allowlisted text lifecycle only; forbidden event terminates     |
| Secret/token/provider leakage         | Critical | Backend-only private token, no live credentials, scrubbed env, redaction tests  |
| Native regresses or depends on Hermes | High     | Native default, independent construction, absence/crash regression tests        |
| Automatic fallback replays ambiguity  | High     | Fixed selection, typed failure, explicit new Native run only                    |
| Protocol/version/status-text drift    | High     | Exact artifact/digest/events/parser and separate upgrade increments             |
| Process or descendant leak            | High     | Require proved containment ledger/termination for detached descendants          |
| Lazy install or updater side effect   | Critical | Read-only/offline spike controls and denial evidence; otherwise adapter blocked |
| Dotenv or managed-secret ingestion    | Critical | Require proved source denial; any access attempt fails the adapter              |
| Premature dependency choice           | High     | Block Ready status until exact dependency/license/build review is approved      |
| Fake-provider result overstated       | Medium   | Label synthetic only; no live-provider or user-value claim                      |

Any unresolved Critical or High finding blocks implementation or completion.

## Rollback or failure strategy

Disable the experimental selection, terminate and reap the exact child tree,
remove adapter-specific modules/tests/dependencies and isolated runtime state,
restore `agent/mod.rs` and selection construction, and retain the verified
`AgentRuntime`, `NativeAgentRuntime`, deterministic mock, native tests, D-079,
and spike evidence. No user data migration exists in the initial adapter.

If the adapter cannot preserve the shared contract or tool prohibition, record
NO-GO, remove it, and continue native-only. Do not weaken the contract or OS
boundary to preserve sunk work.

## Historical decisions and current disposition

- D-080 formerly fixed this Draft to contained managed local `hermes serve`
  TUI-gateway JSON-RPC/WebSocket. That path and ACP subsequently returned NO GO;
  this Draft now has no selected transport.
- Native remains default and independent; experimental selection is explicit
  and backend-only.
- Initial capability is synthetic basic text lifecycle only.
- Exact dependencies, files, artifact digest, provider stub, and containment
  profile remain unresolved blockers to Ready status.

## Discoveries

- The final adapter API must be derived from the implemented native contract,
  not copied from Hermes's broad method catalog.
- The public launcher needs `[web]` and POSIX `[pty]` support plus a loopback
  query token; none should become an application-wide type or user-facing
  credential.
- The pinned launcher/update paths and detached worker behavior make read-only
  offline launch and containment-wide membership prerequisites, not optional
  adapter hardening.

## Progress

- 2026-08-11: Draft created from D-079/D-080. No adapter source, dependency,
  runtime selection, Hermes process, or application behavior exists.
- 2026-08-11: WebSocket and ACP spikes returned NO GO. Historical WebSocket
  details were retained as non-executable evidence; the plan remains
  Draft/Blocked with no selected transport.

## Acceptance criteria

- [ ] Native runtime implementation is valid and passes all contract/regression
      tests.
- [ ] Contained WebSocket spike passes with no blocking finding.
- [ ] Exact complete-runtime manifest/digest and provenance, `[web]`/POSIX
      `[pty]`, offline/no-update controls, endpoint policy, protocol fixtures,
      containment profile, dependencies, paths, commands, and target-Mac
      evidence are approved.
- [ ] Hermes-specific configuration, process, protocol, version, and error types
      remain adapter-local.
- [ ] Basic synthetic text lifecycle works behind the unchanged `AgentRuntime`.
- [ ] Native remains default, independent, and usable for every Hermes failure.
- [ ] Fake tests cover unavailability, incompatibility, timeouts, cancellation,
      malformed frames, termination, switching, native regression, and redaction.
- [ ] Opt-in contained integration passes against the exact supported candidate.
- [ ] No tool, memory, skill, subagent, schedule, messaging, shell, filesystem,
      Git, cloud, credential, or device action is enabled.
- [ ] No dependency install/update path, artifact mutation, unapproved endpoint,
      Unix socket, or escaped/detached descendant is observed.
- [ ] No dotenv, managed-secret, credential-store, or external secret-source
      access is possible or observed.
- [ ] Complete verification and post-increment review pass.

## Final results

Not started. Status remains Draft/Blocked.

## Documentation updates

When separately approved and completed:

- [ ] `HANDOFF.md`
- [ ] `PROJECT_STATUS.md`
- [ ] `NEXT_STEPS.md`
- [ ] `DECISIONS.md`, only if implementation evidence changes a durable decision
- [ ] `CHANGELOG.md`
- [ ] `TROUBLESHOOTING_LOG.md`, only for durable failure/resolution evidence
