# Hermes serve WebSocket containment spike

Status: Verified complete with advisories; spike verdict FAIL / NO-GO at
Milestone 0 before any Hermes process launch
Owner: Project owner
Last updated: 2026-08-11

This is an isolated technical-spike ExecPlan under D-080. The owner supplied a
pinned candidate and authorized Milestone 0. Static provenance, import,
upstream-source, architecture, security, and readiness reviews produced a
decisive negative result before launch. No later milestone or Hermes process
was started.

## Goal

Prove or disprove whether one exact Hermes Agent package/application version
`0.20.0`, release tag `v2026.8.3`, `hermes serve` process can be supervised
behind a loopback-only authenticated TUI-gateway JSON-RPC/WebSocket projection
inside a verified whole-process containment boundary, without connecting Hermes
to Cortexa production behavior.

## User-visible outcome

None. The desktop UI and native runtime remain unchanged. The result is an
isolated evidence report with a GO, CONDITIONAL GO, or NO-GO recommendation for
a later experimental adapter.

## Originally approved scope

The following was the approved scope. Milestone 0 stopped before the test
supervisor, fake server, or real-runtime portions were implemented:

- Build a test-only Rust supervisor and closed WebSocket projection against a
  deterministic local fake server.
- Model exact discovery, version, launch, readiness, health, authentication,
  session, text, streaming, timeout, cancellation, crash, and shutdown states.
- Add an explicitly ignored, opt-in real-Hermes test for one operator-supplied
  pinned candidate inside the approved target-Mac containment boundary.
- Use an isolated empty Hermes home, temporary working directory, environment
  allowlist, per-launch token, loopback address, and OS-assigned port.
- Demonstrate one synthetic text-only session with a deterministic local fake
  model/provider inside containment, or return NO-GO if the pinned artifact
  cannot do so without forbidden capabilities or live credentials.
- Capture only bounded, redacted protocol-shape and lifecycle evidence.

## Explicit non-goals

- No production `HermesAgentRuntime`, application wiring, Tauri command/event,
  React/UI exposure, runtime selector, or behavior change.
- No Hermes installation, update, repair, bootstrap, package download, mutable
  source checkout, or production packaging.
- No live provider, model credential, external model request, personal content,
  cloud action, or non-loopback network.
- No host tool, approval, secret, MCP, memory, skill, plugin, subagent,
  scheduling, messaging, webhook, voice, media, shell, browser, clipboard,
  filesystem, Git, Keychain, or device capability.
- No production dependency, manifest, or lockfile change. A newly required test
  dependency requires a separate owner-approved plan amendment.
- No ACP evaluation or raw TUI-gateway stdio reconsideration.
- No commit, push, PR, merge, release, or publication without separate owner
  direction.

## Existing behavior and constraints

- D-079's `AgentRuntime` and `NativeAgentRuntime` are verified complete under
  gate `native-agent-runtime-boundary`; Native remains the sole/default runtime.
- The prior raw-stdio spike is NO-GO and proved fixture mechanics only; its
  internal-module launcher must not be reused.
- `hermes serve` is a supported public launcher but exposes a broad authenticated
  HTTP/WebSocket server, not a narrow chat daemon.
- `/api/health` proves lightweight liveness/version only. `gateway.ready` proves
  WebSocket connection readiness only. There is no negotiated protocol-version
  or capability handshake.
- No documented single `--no-tools` switch proves the required conversation-only
  capability set. OS containment and observed-behavior rejection are
  load-bearing.
- Target-Mac review disproved `sandbox-exec` as a sufficient sole containment
  mechanism for this plan. It cannot prove exact dynamic-listener restriction,
  package-manager execution denial, blanket Unix-socket denial, or membership
  and cleanup of detached descendants. No alternative mechanism was selected.

## Current-state evidence

- Canonical upstream package/application version: `0.20.0`.
- Canonical release tag: `v2026.8.3`.
- Canonical source commit:
  `3c27eb6234bf91b8ceee9e9071591b31e9b148cb`.
- Official public launcher documentation:
  [`hermes serve`](https://github.com/NousResearch/hermes-agent/blob/v2026.8.3/website/docs/reference/cli-commands.md).
- Protocol documentation:
  [Programmatic Integration](https://github.com/NousResearch/hermes-agent/blob/v2026.8.3/website/docs/developer-guide/programmatic-integration.md).
- Pinned route/authentication owner:
  [`hermes_cli/web_server.py`](https://github.com/NousResearch/hermes-agent/blob/v2026.8.3/hermes_cli/web_server.py).
- Pinned bridge handler:
  [`tui_gateway/ws.py`](https://github.com/NousResearch/hermes-agent/blob/v2026.8.3/tui_gateway/ws.py).
- Transport decision:
  `docs/adr/ADR-HERMES-SERVE-WEBSOCKET-TRANSPORT.md` and D-080.

## Files expected to change

Experimental scope:

- `docs/spikes/HERMES_SERVE_WEBSOCKET_SPIKE.md` (new negative-result report)
- this plan

The planned Rust harness and Python fixture were not created because Milestone
0 reached an explicit stop condition before implementation.

Closeout scope:

- `ARCHITECTURE.md`
- `PLANS.md`
- `HANDOFF.md`
- `PROJECT_STATUS.md`
- `NEXT_STEPS.md`
- `CHANGELOG.md`
- `docs/increments/hermes-serve-websocket-spike.md` (new)
- `docs/reviews/YYYY-MM-DD-hermes-serve-websocket-spike-post-increment-review.md`
  (new)

No `src-tauri/src/**`, `src/**`, manifest, lockfile, Tauri configuration,
capability, workflow, hook, skill, installer, or generated file is in scope. If
the exact containment proof requires a helper file or entitlement, update this
plan and obtain owner approval before editing or executing Hermes.

## Planned affected components; not created

- Planned Rust integration-test process supervisor and protocol harness only.
- Planned deterministic local fake server fixture.
- Reviewed target-Mac containment prerequisites and negative-result
  documentation only; no containment process was started.

No production runtime, provider, policy, approval, tool, audit, memory,
platform, storage, Tauri, or React component changes.

## Interfaces and invariants

### Lifecycle states

The test-only supervisor uses a closed monotonic state machine:

```text
Undiscovered
  -> CandidateValidated
  -> Starting
  -> ReadyLineObserved
  -> HealthVerified
  -> WebSocketReady
  -> SessionActive
  -> TurnActive
  -> TurnTerminal
  -> Stopping
  -> Stopped
```

Any nonterminal state may enter `Failing -> Stopping -> Stopped`. No restart is
automatic. Cancellation is locally idempotent; its cooperative acknowledgement
is not terminal until the bounded pinned-text status parser yields `NotRunning`
or the host closes the run as failed and starts containment-wide cleanup. Output
after the host terminal decision is rejected. A transition cannot skip version,
health, authentication, or session binding. Partial text is never completion.

### Test-only protocol types

Keep these private to `hermes_serve_websocket_spike.rs`:

- `PinnedHermesIdentity`: exact distribution form, package version, tag, commit,
  complete interpreter/runtime manifest and digest, and supported extras;
- `SpikeConfig`: validated executable path, numeric loopback host, port zero,
  bounded deadlines/limits, isolated paths, and containment command vector;
- `LaunchSecret`: private per-launch token with redacted Debug, no
  string-returning accessor outside request construction, and an explicitly
  reviewed memory-cleanup strategy before real execution;
- `ReadyAnnouncement` and `HealthStatus`: closed parsed readiness, port,
  liveness, software version, and authentication facts;
- `PinnedSessionActivity`: a closed `Running | NotRunning` result produced by an
  isolated bounded parser for exactly one anchored `Agent Running: Yes|No` line
  in `session.status.output`; all other raw status text is discarded and never
  logged or returned;
- `RpcId`, `SessionId`, `JsonRpcRequest`, `JsonRpcResponse`, and `GatewayEvent`:
  exact bounded identifiers and allowlisted shapes;
- `SpikeState`, `TurnTerminal`, `CancellationOutcome`, and `SpikeError`: closed
  lifecycle results with content-free Display/Debug; and
- `ContainedProcessSet`: exact validated containment identity, membership ledger,
  and bounded termination/reap behavior that includes descendants which call
  `setsid` or create a new process group.

Raw JSON, HTTP headers, token-bearing URLs, stderr content, environment values,
provider responses, or Hermes-native payload structs may not leave the test
module. Unknown fields are rejected unless the pinned protocol fixture marks a
specific harmless field optional.

### Launch and readiness

- Candidate discovery is explicit; no untrusted `PATH` search.
- Version output, tagged-source evidence, the chosen installed-distribution
  form, and a manifest/digest covering the complete interpreter environment and
  Hermes distribution must match before launch and be rechecked against
  replacement races. A console-script hash alone is insufficient.
- Before starting Hermes, preflight the exact `[web]` and POSIX `[pty]` imports.
  Keep the candidate distribution read-only, apply reviewed offline/no-update
  controls, deny package-manager child execution and updater egress, and detect
  mutation. The pinned launcher's `tools.lazy_deps.ensure(..., prompt=False)` and
  gateway update-check path must be suppressed and tested; inability to do so is
  NO-GO.
- Use direct argv: `hermes serve --host 127.0.0.1 --port 0`; never a shell.
- Use an environment allowlist, isolated home/cwd/cache/config/state, no
  `HERMES_DESKTOP=1`, and no inherited credential/proxy/provider variables.
- Enumerate the exact dotenv and managed-secret locations/sources used by the
  pinned `hermes_cli.env_loader`. Reject a candidate with a distribution-root
  `.env` without reading its contents; keep the fresh isolated home free of
  `.env`/`.op.env`; exclude the machine-managed dotenv path from the containment
  view; disable configured external secret sources; and fail NO-GO on any
  dotenv, secret-manager, credential-store, child, socket, or network access
  attempt.
- Parse a bounded `HERMES_BACKEND_READY port=<N>` line from the owned child,
  then require exact-version `/api/health`, connect only to
  `ws://127.0.0.1:<port>/api/ws?token=<per-launch-token>`, and accept one
  `gateway.ready` event with bounded `skin` and `change_events: true` before
  session creation. Never log or return the complete token-bearing URL.
- Treat stdout/stderr as separate bounded channels. Retain closed counts and
  categories only; never persist raw diagnostic or prompt content.

### Protocol allowlist

The spike may send only `session.create`, `prompt.submit`, `session.status`,
`session.history`, `session.interrupt`, and `session.close`. It may accept only
their correlated results plus required `gateway.ready`, asynchronous
`session.info` after deferred creation and turn finalization, `message.start`
before streaming, `message.delta`, `message.complete`, and closed failure/
termination shapes. Pinned fixtures must define the exact bounded fields and
permitted interleavings for both mandatory asynchronous event families.

Every ID, session, method, event, byte count, event count, delta count, text
length, sequence, and deadline is bounded. Unknown, duplicate, late,
out-of-order, wrong-session, wrong-ID, malformed, oversized, binary, compressed,
or fragmented frames outside the deliberately supported test profile fail
closed.

`session.interrupt` returns only an immediate cooperative
`{"status":"interrupted"}` acknowledgement. The supervisor must then perform a
bounded `session.status` reconciliation until `PinnedSessionActivity` yields
`NotRunning`.
`message.complete` may be accepted when correlated but is not required after
interruption. The host owns the terminal result and rejects late output; cleanup
must still terminate all containment members because Hermes-started background
processes can outlive the interrupted turn.

The pinned status result is human-oriented JSON-RPC payload text, not a
structured `running` field. Its private parser accepts exactly one anchored
`Agent Running: Yes|No` line in bounded `output`, maps only that flag, discards
the remaining raw text, and fails closed on a missing, duplicate, or changed
shape. Fake fixtures may not invent `{"running":false}`. This fragile parser
must remain spike-local and is part of the compatibility verdict.

### Capability and containment invariant

Configuration requests no tools or mutable agent capabilities, but the OS
boundary must independently deny host access, all external networking, every
loopback destination except the exact deterministic fake-provider endpoint,
and every Unix-domain socket. It separately permits only the authenticated
inbound `127.0.0.1` serve listener. Any observed
tool/approval/secret/config/CLI/reload/MCP/memory/skill/plugin/subagent/schedule/
messaging/shell/file/Git/browser/clipboard/cloud/device method, event, process,
file, or connection is a spike failure.

The fake fixture proves host logic only. Real-Hermes evidence is valid only when
the later approved target-Mac containment mechanism proves process-tree
ownership, filesystem/network/credential denial, descendant cleanup, and no
state outside the isolated home. Its membership and termination primitive must
capture deliberately detached descendants; tracking or killing only the parent
process group cannot satisfy this invariant. The evidence must also account for
the pinned FastAPI lifespan's maintenance tasks and the WebSocket change-watcher
thread even when `HERMES_DESKTOP=1` is absent.

## Implementation milestones

- [x] Milestone 0 - prerequisite and exact-containment review completed with a
      negative result; required controls remain unmet
  - require clean synchronized baseline after valid
    `native-agent-runtime-boundary` completion;
  - confirm D-079/D-080 and this plan are still current;
  - select the exact installed-distribution form and record a complete-runtime
    manifest/digest plus version/tag/commit provenance; unresolved provenance is
    a blocker, not a console-script-hash substitute;
  - prove `[web]` and POSIX `[pty]` imports before launch; select explicit
    offline/no-update controls, read-only artifact enforcement, package-manager
    child denial, updater-egress denial, and mutation detection without
    installing or updating anything;
  - enumerate the pinned dotenv/managed-secret resolution paths and prove
    distribution-root, isolated-home, machine-managed, and configured external
    sources are absent, disabled, or unreadable without inspecting contents;
  - select and review one target-Mac whole-process containment mechanism with
    explicit filesystem, endpoint-level network, Unix-socket, Keychain, process,
    and detached-descendant membership/termination guarantees;
  - record the exact deterministic local fake-provider endpoint for a text turn;
  - fresh architecture, security, and readiness reviews returned Blocked;
  - begin exactly one `hermes-serve-websocket-spike` gate only to preserve and
    validate this negative-result documentation after the stop decision.
- [ ] Milestone 1 - deterministic fake server and closed host state machine
  - add success, timeout, cancel, crash, malformed, auth, version, forbidden,
    stderr, redaction, and cleanup scenarios;
  - use no third-party or production dependency.
- [ ] Milestone 2 - opt-in contained real-Hermes proof
  - keep the test ignored by default and require explicit operator candidate;
  - skip cleanly only when the opt-in is absent; once opted in, missing or
    mismatched prerequisites fail;
  - prove preflight, no install/update attempt, start, readiness, health,
    authentication, one session, one synthetic text turn, streaming,
    cancellation/status reconciliation, shutdown, and no forbidden activity.
- [ ] Milestone 3 - negative containment and portability evidence
  - prove non-loopback bind, host/repository write, Keychain/secret access,
    external network, an unapproved loopback endpoint, Unix-domain socket,
    dotenv/managed-secret source, package-manager/update egress, child escape,
    and state leakage are denied;
  - prove cleanup of an intentionally detached `setsid` descendant and record
    target-Mac containment-membership and socket cleanup.
- [x] Milestone 4 - negative report, documentation sync, and safe stop complete
  - publish GO, CONDITIONAL GO, or NO-GO with exact evidence and limitations;
  - synchronize documentation and finalize the gate;
  - do not begin the adapter.

## Security and privacy considerations

No real content or credential may enter the spike. Prompt, response, token,
headers, environment values, paths containing a user name, and raw stderr are
not repository evidence. The plan assumes Hermes is adversarial and treats the
OS boundary, not its approval/redaction/tool settings, as authoritative.

The real test may use only a fixed synthetic prompt and deterministic local
fake provider. If Hermes requires live credentials, an external model, a global
home, host files, an uncontained subprocess, or tool initialization for the
turn, the test stops NO-GO.

## Test plan

Ordinary fake-server tests must cover:

- discovery unavailable and invalid executable/configuration;
- exact compatible and incompatible version/distribution identity, missing
  `[web]`/POSIX `[pty]`, attempted lazy installation, update-check egress,
  package-manager child execution, and artifact-mutation detection;
- distribution-root `.env`, isolated-home `.env`/`.op.env`, machine-managed
  dotenv, configured external secret-source, and attempted credential-source
  access, all without reading or emitting secret contents;
- readiness-line timeout, malformed port, health failure, and version mismatch;
- WebSocket authentication failure and missing/duplicate `gateway.ready`;
- one basic session with deferred/terminal `session.info`, `message.start`, text
  request, streaming deltas, exact completion, and valid pinned interleavings;
- malformed JSON-RPC, unknown/forbidden method/event, duplicate IDs, wrong
  session, out-of-order and late frames, oversize, invalid UTF-8, and EOF;
- prompt timeout reconciliation through the exact bounded
  `Agent Running: Yes|No` status parser and history without blind retry,
  including missing/duplicate/changed status text and raw-output redaction;
- cancellation before and during streaming, immediate cooperative interrupt
  acknowledgement, bounded `NotRunning` status reconciliation with and without
  `message.complete`, idempotence, and late-output rejection;
- child crash before readiness and mid-turn, bounded separated stderr, and
  redacted errors;
- graceful shutdown, forced containment-wide termination, intentionally detached
  descendant reap, socket closure, and no leaked state; and
- secret/env/path sentinel absence from every public error and report value.

The ignored real-Hermes test must cover the same supported happy-path lifecycle
and selected containment negatives. It must never auto-install, auto-update, or
silently pass after explicit opt-in when a prerequisite is missing.

## Verification commands

Focused and final commands, subject to the later approved containment mechanism:

```bash
python3 -m py_compile src-tauri/tests/fixtures/hermes_serve_websocket_stub.py
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo test --manifest-path src-tauri/Cargo.toml --test hermes_serve_websocket_spike --locked
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
npm run verify
npm run security:scan
git diff --check
python3 .codex/hooks/session_end_gate.py
```

The opt-in target-Mac command and containment evidence must be added verbatim
before this plan becomes Ready. Missing required manual containment evidence
makes the quality result FAIL.

Milestone 0 stopped before the new fixture/real-runtime targets existed, so
their commands were not run. The applicable negative-result closeout commands
were:

```bash
cargo test --manifest-path src-tauri/Cargo.toml --test hermes_transport_spike --locked
cargo test --manifest-path src-tauri/Cargo.toml --test agent_runtime_contract --locked
npm run verify
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
git diff --exit-code -- src src-tauri/src src-tauri/tests src-tauri/Cargo.toml src-tauri/Cargo.lock package.json package-lock.json src-tauri/tauri.conf.json src-tauri/capabilities .github .codex .agents
python3 .codex/hooks/session_end_gate.py
python3 .codex/hooks/post_increment_gate.py status
```

Exact outcomes and the negative/not-run transport evidence are in the
post-increment review. The spike verdict is FAIL / NO-GO; the separately scored
engineering closeout is PASS WITH ADVISORIES because the plan's
prove-or-disprove objective completed safely.

## Risks

| Risk                                  | Severity | Mitigation / blocker                                                                   |
| ------------------------------------- | -------- | -------------------------------------------------------------------------------------- |
| Whole-process escape                  | Critical | Exact reviewed OS boundary plus adversarial deny evidence; otherwise NO-GO             |
| Broad privileged gateway surface      | High     | Closed method/event allowlist; any forbidden traffic fails the spike                   |
| Tool initialization despite config    | Critical | OS deny boundary plus observed process/file/network proof; otherwise NO-GO             |
| Token or diagnostic leakage           | High     | Backend-only secret type, no raw URLs/logs, sentinel/redaction tests                   |
| Protocol/status-text drift            | High     | Exact release/digest, pinned events/parser, reject drift, separate upgrade decision    |
| Ambiguous timeout or cancellation     | High     | Status/history reconciliation, no blind retry, local terminal authority                |
| Orphan process or socket              | High     | Containment ledger/termination captures detached descendants; socket proof required    |
| Lazy install or updater side effect   | Critical | Import preflight, read-only artifact, offline controls, deny child/egress; else NO-GO  |
| Dotenv or managed-secret ingestion    | Critical | Reject/deny every pinned source and observe attempts without reading secret contents   |
| Fake-only false confidence            | High     | Label fixtures non-conformance; require opt-in contained real artifact for GO          |
| Provider/credential creep             | High     | Deterministic local fake provider only; otherwise NO-GO                                |
| Test wire reused as production client | Medium   | Keep all code in integration test and require a later adapter design/dependency review |

Any Critical or High unresolved finding blocks completion and the adapter.

## Rollback or failure strategy

No experimental test/fixture file or runtime directory was created. Rollback is
to revert only this spike's ten documentation paths and remove any disposable
preflight state. The observed disposable preflight directory was removed; no
Hermes child, socket, token, profile, cache, or fake-provider process was
created. Native runtime source and behavior remain unchanged.

## Decisions made

- D-080 selects this transport only for contained evaluation.
- Raw stdio remains rejected and ACP remains deferred.
- A real text turn uses a deterministic local fake provider or the spike returns
  NO-GO; live credentials and external provider traffic are not authorized.
- No test dependency or containment mechanism is selected by this documentation
  plan. Those exact choices require Milestone 0 review before readiness.

## Discoveries

- `hermes serve` is public and used by Hermes Desktop, but retains the full
  broad gateway/API core in headless mode.
- Loopback WebSocket access still needs a session token; health success does not
  remove that authentication requirement.
- The pinned ready event has no protocol version/capability attestation, so
  compatibility is host-enforced rather than negotiated.
- The supplied source archive, three critical-file digests, clean Git state,
  package metadata, and 61-package inventory are internally consistent, and a
  sanitized isolated `find_spec`/metadata preflight found the required web and
  PTY modules. They do not constitute the plan-required complete-runtime
  manifest: the virtual environment contains 4,077 regular files and delegates
  to an externally located, owner-writable Python runtime that is absent from
  the supplied manifest.
- The pinned server starts update-prefetch, dotenv/managed-secret loading,
  credential keepalive, skill synchronization, and plugin discovery without a
  supported complete disable mode. Its default toolset is privileged, and an
  empty toolset does not mean zero tools. Containment could deny effects but
  could not truthfully establish the required absence of those attempts.
- A credential-free deterministic loopback OpenAI-compatible fake provider is
  source-supported, but that does not cure the startup and containment
  blockers.
- On the target Mac, deprecated `sandbox-exec` restrictions inherit across
  descendants but provide no containment membership/kill primitive. Static
  policy also cannot restrict a port-zero child to only its dynamically chosen
  inbound port, distinguish allowed Python from `python -m pip`, or deny
  anonymous Unix socket pairs.

## Progress

- 2026-08-11: Plan created from D-080. No experimental file was created, no
  Hermes candidate was discovered or run, and all milestones remain Blocked.
- 2026-08-11: The owner supplied a pinned installed candidate and authorized
  Milestone 0. Exact source/tag/commit, critical hashes, installed metadata, and
  sanitized module discovery passed. Complete-runtime provenance and the
  required target-Mac containment/startup-suppression guarantees failed fresh
  architecture, security, and readiness review. The work stopped before any
  `hermes serve` process, socket, WebSocket frame, session, provider, tool, or
  fixture implementation.
- 2026-08-11: Added the bounded negative-result report for closeout review. The
  adapter remains Draft/Blocked; Native remains sole/default; Prompt 4D was not
  started.
- 2026-08-11: Post-increment review classified the bounded negative-result
  closeout PASS WITH ADVISORIES. Critical provenance, startup, and containment
  findings block every later Hermes increment rather than preservation of this
  safely stopped evidence.

## Acceptance criteria

- [x] Native `AgentRuntime` and `NativeAgentRuntime` are verified complete.
- [ ] Exact distribution manifest/digest and provenance, `[web]`/POSIX `[pty]`,
      install/update suppression, containment mechanism, fake-provider endpoint,
      commands, and target-Mac evidence are approved and recorded.
- [ ] Every pinned dotenv/managed-secret path and external source is enumerated,
      absent/disabled/unreadable, and covered by negative access evidence.
- [ ] Ordinary fake tests pass without Hermes, network, model, or dependency.
- [ ] Opt-in real-Hermes evidence proves the allowed lifecycle and every required
      containment boundary on the exact candidate.
- [ ] One synthetic text-only turn completes without live credentials or cloud.
- [ ] No tool, privileged event, host access, external or unapproved-loopback
      network, Unix socket, escaped/detached descendant, leaked secret, install
      or update attempt, dotenv/managed-secret access, artifact mutation, or
      state outside the isolated home is observed.
- [ ] Cancellation, timeout reconciliation, crash handling, shutdown, forced
      cleanup, and redaction pass.
- [ ] Architecture, security, portability, packaging, and rollback reviews pass.
- [x] The report declares GO, CONDITIONAL GO, or NO-GO without beginning an
      adapter.

## Final results

**FAIL / NO-GO under the approved D-080 conditions.** Milestone 0 disproved
readiness before real execution. The candidate matches Hermes Agent `0.20.0`,
tag `v2026.8.3`, and commit
`3c27eb6234bf91b8ceee9e9071591b31e9b148cb`; its source and critical hashes are
consistent and required modules are discoverable. The supplied evidence does
not hash the complete virtual environment or its external Python runtime.

More decisively, the pinned release has no supported startup mode that disables
update-prefetch, dotenv/managed-secret initialization, credential keepalive,
plugin discovery, skill synchronization, and all tools. The reviewed
`sandbox-exec` mechanism cannot independently satisfy the exact dynamic
listener, package-manager, Unix-socket, or detached-descendant controls. No
Hermes executable or server process was run, no fake/real WebSocket test was
created, and no credential was required or accessed. See
`docs/spikes/HERMES_SERVE_WEBSOCKET_SPIKE.md`.

This result does not select ACP or another transport and does not amend D-080 by
inference. A later proposal requires an owner-approved ADR/plan amendment and
either an upstream/pinned Hermes build with explicit disable controls plus a
complete immutable runtime manifest, or a separately reviewed containment
mechanism that closes every failed guarantee.

## Documentation updates

Negative-result closeout:

- [x] `HANDOFF.md`
- [x] `PROJECT_STATUS.md`
- [x] `NEXT_STEPS.md`
- [ ] `DECISIONS.md`, only if evidence requires an additive decision
- [x] `CHANGELOG.md`
- [ ] `TROUBLESHOOTING_LOG.md`, only for a durable failure/resolution
