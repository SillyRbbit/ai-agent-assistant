# V0-1 — sealed Personal Assistant empty-tool turn and volatile host

Status: Verified complete with advisories
Owner: Henry Dang
Last updated: 2026-08-28
Depends on: D-094 and the verified CI trust-boundary classifier
Baseline: `0b22ee79a24e11d7c67cbace111a502608b57591`

## Goal

Add one transport-free, application-owned Personal Assistant text-turn contract
whose serialized request pins an immutable instruction profile, the selected
OpenAI-through-Cloudflare model profile, an empty tool set, zero retries, and
the lower v0 limits. Route that sealed profile through the existing
`AgentRuntime::start` boundary without creating a second start API. Add the
smallest public, no-input, process-local host that issues its identities and is
a genuine production caller of the sealed request factory. The selected Native
run path must validate deterministic synthetic response frames without
performing I/O; the V0-1 host itself exposes no response-frame ingress.

## User-visible outcome

None. This is a Rust prerequisite. It does not create a live assistant,
provider request, Tauri command, or UI.

## Exact files

- `src-tauri/src/agent/gateway_request.rs`
- `src-tauri/src/agent/native_runtime.rs`
- `src-tauri/src/agent/runtime.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/personal_assistant_v0.rs` (new)
- `src-tauri/tests/agent_runtime_contract.rs`
- `src-tauri/tests/gateway_request_contract.rs`
- `src-tauri/tests/personal_assistant_v0_contract.rs` (new)
- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `ROADMAP.md`
- `SECURITY.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-08-28-personal-assistant-v0-empty-tool-turn.md`
- `docs/increments/personal-assistant-v0-empty-tool-turn.md` (new)
- `docs/reviews/2026-08-28-personal-assistant-v0-empty-tool-turn-post-increment-review.md`
  (new)

No other source, test, manifest, lockfile, capability, CSP, workflow, or
configuration file may change.

## Interfaces

Add one distinct `PersonalAssistantTextTurn` beside, not in place of,
`InitialGatewayTurn`.

- Add one private `RuntimeTurnProfile` field to `RuntimeTurnRequest`. The
  existing public constructor always selects the unchanged initial profile.
- Add one crate-private synthetic-v0 factory that accepts only host-issued
  identity strings and immediately validates them into the existing typed
  identities; raw `String` in the exact signature is not an unvalidated or
  caller-selected authority. It accepts no text, agent, instruction, provider,
  model, tool, runtime, workflow, data-class, limit, or fixture selector.
- `NativeAgentRuntime` continues to implement only `AgentRuntime::start`. It
  chooses the existing or empty-tool turn from the private request profile and
  returns the same `NativeAgentRun` surface. Do not add an inherent start method
  or a second runtime trait.
- The internal Native turn is exactly a boxed private enum—one boxed
  `InitialGatewayTurn` variant and one boxed `PersonalAssistantTextTurn`
  variant—so the existing run remains a stable-size owner without a
  `large_enum_variant` allowance. It delegates request bytes, status, typed
  `RuntimeRun::accept_event`, and cancellation while keeping existing
  initial-turn behavior exact. The existing concrete
  `NativeAgentRun::accept_frame` signature and `InitialGatewayEvent` result stay
  Initial-profile-only; invoking it on the Personal Assistant profile returns a
  new closed `WrongProfile` error before parsing or mutation.
- Preserve both other concrete Native approval methods and their signatures.
  `cancel_pending_approval_for_run_termination()` returns `Ok(None)` without
  mutation for the Personal Assistant profile because that profile can never
  own an approval. On macOS, `resolve_approval_source_outcome(...)` returns the
  new closed `InitialGatewayTurnError::WrongProfile` before inspecting the
  trusted outcome. Initial-profile behavior for both methods remains exact.
- `request_bytes()` returns the closed serialized body for a future trusted
  transport. Debug output reports only body length and closed state.
- Crate-private `PersonalAssistantTextTurn::accept_frame(&[u8])` delegates
  normalized-event validation to the existing transactional
  `GatewayStreamValidator`, then applies lower v0 byte, scalar, event,
  text-only, and terminal checks. It is exercised directly by module tests but
  is not reachable through the V0-1 host or the Initial-only concrete Native
  frame method.
- `cancel()` is local, terminal, and idempotent. Post-cancel, post-failure, and
  post-completion frames fail closed.
- `PersonalAssistantV0Host` owns one optional process-local run. Its public
  `start_synthetic()` method accepts no arguments, issues the run/request
  identities, constructs the sealed request, captures its expected
  `RuntimeRunIdentity`, and calls only `AgentRuntime::start`. Before projecting
  status or request bytes, it compares the returned run's full identity to the
  captured identity exactly. A mismatch is rejected, cancelled, and either
  proved terminal or retained in a private quarantine with the process lease;
  it is never exposed or replaced. A process-wide atomic lease rejects a second
  start across multiple host instances while a run is nonterminal or
  quarantined. V0-2, not this increment, adds presentation handles, journals,
  polling, and deadlines.
- V0-1 adds no host response-frame ingress. Public callers cannot inject raw
  frame bytes or embedded provider identities. Focused tests exercise raw
  Personal Assistant frames directly on the crate-private turn and exercise the
  Native branch through unchanged `RuntimeRun::accept_event`; both reject tools
  and enforce the lower text limits. V0-7 may add a crate-private host/raw-frame
  ingress only in the same increment that gives it a real trusted-transport
  caller. The host exposes only closed status/cancellation values and retains no
  transcript after terminal cleanup or drop.

The wire request is version 1 and contains exact application-owned values:

- agent: `personal-assistant`;
- instruction profile: `personal-assistant-text-v0@1` plus the fixed exact
  instruction text;
- provider profile: `openai-cloudflare-personal-assistant-v0@1`;
- provider/model: `openai` / `gpt-5.6-luna`, streaming, low reasoning,
  low-verbosity plain text, disabled truncation, and no parallel tool calls;
- data class: `application-owned-synthetic@1`;
- `store: false`, `background: false`;
- tool set: `empty@1`, an empty schema list, and zero function calls;
- one model turn, one gateway request, zero retries, and no fallback; and
- the exact v0 limits recorded in the program plan.

The exact immutable instruction text is:

```text
Act as Cortexa's Personal Assistant for one bounded synthetic text request. Answer only from the supplied application-owned synthetic text and return concise plain text. Do not call or propose tools, access files, memory, retrieval, networks, or devices, delegate, schedule, persist, approve, execute, retry, or claim any action or context not supplied.
```

The exact application-owned synthetic fixture is:

```text
Prepare a concise three-bullet board update from this synthetic status: planning is approved; implementation has not started; no external systems have changed.
```

The phrase `implementation has not started` is deliberately frozen synthetic
fixture content. It describes neither this V0-1 repository increment nor
current project status.

The serialized JSON object has exactly this key/nesting grammar; angle-bracket
values are the two validated identities issued by the V0-1 host or
deterministic unit tests and are not literal wire text:

```json
{
  "contract_version": 1,
  "protocol_version": 1,
  "run_id": "<rust-issued-run-id>",
  "gateway_request_id": "<rust-issued-gateway-request-id>",
  "request_kind": "personal_assistant_text_v0",
  "model_turn": 1,
  "retry_attempt": 0,
  "agent": {
    "id": "personal-assistant",
    "instruction_profile_id": "personal-assistant-text-v0",
    "instruction_profile_version": 1,
    "instructions": "Act as Cortexa's Personal Assistant for one bounded synthetic text request. Answer only from the supplied application-owned synthetic text and return concise plain text. Do not call or propose tools, access files, memory, retrieval, networks, or devices, delegate, schedule, persist, approve, execute, retry, or claim any action or context not supplied."
  },
  "provider": {
    "profile_id": "openai-cloudflare-personal-assistant-v0",
    "profile_version": 1,
    "provider": "openai",
    "model": "gpt-5.6-luna",
    "stream": true,
    "store": false,
    "background": false,
    "reasoning_effort": "low",
    "text_format": "text",
    "text_verbosity": "low",
    "truncation": "disabled",
    "parallel_tool_calls": false,
    "fallback": "none"
  },
  "data_class": {
    "id": "application-owned-synthetic",
    "version": 1
  },
  "input": {
    "type": "application_owned_synthetic_text",
    "text": "Prepare a concise three-bullet board update from this synthetic status: planning is approved; implementation has not started; no external systems have changed."
  },
  "tool_set": {
    "id": "empty",
    "version": 1,
    "tools": []
  },
  "limits": {
    "max_user_input_characters": 4096,
    "max_user_input_bytes": 16384,
    "max_model_turns": 1,
    "max_gateway_requests": 1,
    "max_retry_attempts": 0,
    "max_function_calls": 0,
    "max_gateway_request_bytes": 65536,
    "max_gateway_event_bytes": 16384,
    "max_gateway_events": 128,
    "max_text_delta_characters": 1024,
    "max_text_delta_bytes": 4096,
    "max_assistant_output_characters": 8192,
    "max_assistant_output_bytes": 32768,
    "connect_timeout_ms": 10000,
    "stream_idle_timeout_ms": 20000,
    "provider_deadline_ms": 60000,
    "run_deadline_ms": 120000
  }
}
```

The later gateway must compare this complete profile exactly; it may not accept
caller instructions or silently substitute a model.

## Exact Rust signatures

No new public request constructor or runtime-start method is added. The new
interfaces are exactly equivalent to:

```rust
pub(crate) fn RuntimeTurnRequest::personal_assistant_v0_synthetic(
    run_id: String,
    request_id: String,
) -> RuntimeResult<Self>;

pub struct PersonalAssistantV0Host {
    // Opaque private fields own the optional NativeAgentRun, lease ownership,
    // process generation, closed status, and terminal cleanup state. One
    // process-owned private quarantine slot can retain a rejected Native run.
}

impl Default for PersonalAssistantV0Host {
    fn default() -> Self;
}

impl PersonalAssistantV0Host {
    pub fn new() -> Self;
    pub fn start_synthetic(&mut self) -> Result<(), PersonalAssistantV0Error>;
    pub fn status(&self) -> PersonalAssistantV0Status;
    pub fn request_byte_len(&self) -> Result<usize, PersonalAssistantV0Error>;
    pub(crate) fn request_bytes(&self) -> Result<&[u8], PersonalAssistantV0Error>;
    pub fn cancel(&mut self) -> Result<PersonalAssistantV0Cancellation, PersonalAssistantV0Error>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PersonalAssistantV0Cancellation {
    Cancelled,
    AlreadyTerminal(PersonalAssistantV0Status),
}
```

`lib.rs` privately declares the application-owned host module and reexports
only its closed host/status/error/cancellation types. No new event type or raw
frame ingress is exported. Public callers see only request length, never
serialized request or response bytes. The host itself uses the
crate-private byte accessor for its length projection, so the future transport
seam has a genuine non-test production caller without an allow. `Default` is
exactly equivalent to `new`; both create an `Idle` host with no run or lease.
The private host states are exactly Idle, Active, TerminalSummary, and
Quarantined; the latter is never projected as a usable session.
An accepted start projects `Starting` while the returned runtime is
`AwaitingStart`, and only an accepted `Started` event can project `Streaming`.
After proved terminal cleanup, the run and all content are discarded while one
content-free terminal status is retained until the next accepted start or
drop. Repeated cancel then returns `AlreadyTerminal(status)`; `Idle` cancel and
other operations that require a run return `NoActiveRun`. A new start from a
proved terminal summary clears that summary. Run, request, provider-response,
and support-correlation identities remain private. The host error codes are
exactly `Busy`, `NoActiveRun`, `IdentityExhausted`,
`ReturnedIdentityMismatch`, `RequestRejected`, `ProtocolViolation`,
`LimitExceeded`, or `Internal`, with fixed redacted display strings.

The closed public status is exactly `Idle`, `Starting`, `Streaming`,
`Completed`, `Failed`, or `Cancelled`. Accepted internal events are exactly
`Started`, `TextDelta { delta }`, `Completed { final_answer }`, or
`Failed { code }`; the provider response identity is validated and retained
privately. Failure codes are exactly `Unauthenticated`, `Forbidden`,
`RateLimited`, `RequestRejected`, `ProviderUnavailable`, `ProviderTimeout`,
`ProtocolViolation`, `LimitExceeded`, or `Internal`. A gateway failure that
claims retryability or supplies `retry_after_ms` is a protocol violation.

Turn errors are exactly `InvalidTrustedIdentity`, `SerializationFailed`,
`WrongProfile`, `ProtocolViolation`, or `LimitExceeded`, with fixed redacted
display text. `TextDelta` and `Completed` use non-debuggable bounded newtypes.
The host cancellation enum is only a closed projection of the existing
`RuntimeRun::cancel` and `RuntimeCancellationOutcome`; it is not a second
runtime cancellation API.

`NativeAgentRunError::WrongProfile` is the exact closed error for attempting
the existing Initial-only concrete `accept_frame` method on a Personal
Assistant run. It carries no profile, request, frame, or provider content and
does not change the method's public input or result type.
`InitialGatewayTurnError::WrongProfile` is the corresponding closed error for
the macOS approval-source method; the concrete pending-approval cancellation
method is instead the exact safe `Ok(None)` no-op described above.

## Invariants

- Existing public `RuntimeTurnRequest::new`, `InitialGatewayTurn`,
  `NativeAgentRuntime::start`, agent workflows, policy, approval, audit, and
  tool behavior remain byte-for-byte compatible for the existing profile.
- Only the no-input application host can construct the new profile; public
  callers may request the fixed synthetic start but cannot select any trusted
  identity or configuration, inject raw transport frames, or observe content.
  Every WebView/Tauri caller remains absent.
- The host calls the existing `AgentRuntime::start` exactly once per accepted
  session. It does not call `AgentOrchestrator`, create a task/delegation, or
  add an inherent Native start path; D-094 deliberately bounds this single-run
  Personal Assistant exception to the existing runtime ownership boundary.
- Before the start call consumes the request, the host captures its complete
  expected `RuntimeRunIdentity`. It accepts the returned run only when
  `RuntimeRun::identity()` equals that value exactly and the initial status is
  exactly `AwaitingStart`; any other identity or initial status is a rejected
  runtime construction, never a usable session.
- A rejected returned run is cancelled exactly once before release. Only
  `Cancelled` or `AlreadyTerminal` with a terminal status proves cleanup. A
  cancel error or nonterminal `AlreadyTerminal` retains the run in an opaque
  quarantine, keeps the process lease, exposes only closed `Failed`/`Busy`
  projections, and permits no request-byte/underlying-status/content access or
  replacement.
  A later explicit host cancel may retry bounded local cleanup; there is no
  background loop. Drop attempts once and, on ambiguity, moves the rejected
  `NativeAgentRun` into one standard-library, process-owned private quarantine
  slot. No public API or background worker can read or replace that slot; the
  run and lease remain owned until process exit. Mutex poisoning or occupied-
  slot ambiguity leaks/rejects replacement fail-closed and never releases the
  lease. V0-7 must extend this owner with transport quarantine before any
  external request can exist.
- A process-wide atomic lease prevents concurrent starts across host instances.
  Pre-run construction/start failure releases it; terminal completion,
  terminal failure, or successful local cancellation releases it exactly once
  only after cleanup is proved. Drop attempts local cancellation, and an
  unexpected cleanup error retains the lease fail-closed until process exit.
- One process-wide `AtomicU64` generation begins at 1. A successful checked
  reservation formats exactly `pa-v0-run-{generation:016x}` and
  `pa-v0-request-{generation:016x}`; zero/wrap or formatting/validation failure
  is `IdentityExhausted` and starts nothing. The atomic lease uses one
  compare/exchange from idle to held before generation reservation. These IDs
  are predictable process-local correlation, not authentication secrets, and
  never cross the public host surface.
- The empty-tool turn owns no registry, policy engine, approval manager, audit
  adapter, executor, transport, credential, or provider client.
- Every function-call event is rejected. It cannot become proposal, policy,
  approval, audit, or execution data.
- Accepted text deltas concatenate in sequence. Completion requires non-empty
  text and yields one final answer exactly equal to the accepted concatenation.
- A rejected frame commits no partial event or output state. The turn then
  enters one closed terminal protocol failure so a later frame cannot resume
  the rejected stream.
- Request and response content never appears in `Debug`, `Display`, or closed
  errors.
- Identity exhaustion, serialization ambiguity, configuration mismatch, or
  limit overflow fails closed without a fallback factory.

## Threats and controls

| Threat                                        | Required control                                                                         |
| --------------------------------------------- | ---------------------------------------------------------------------------------------- |
| Caller selects trusted configuration          | Crate-private sealed profile factory; fixed fixture/constants; no public or IPC selector |
| Existing two-tool request reused accidentally | Distinct request kind and exact `empty@1` wire assertion                                 |
| Gateway emits a tool call                     | Empty validator allowlist plus a zero-function-call wrapper invariant                    |
| Prompt/model drift                            | Exact complete request fixture and constant-value regression tests                       |
| Oversize or multibyte bypass                  | Scalar and UTF-8 byte tests at, below, and above every lower v0 boundary                 |
| Partial mutation on rejection                 | Clone/apply/commit before terminal closed failure; no later recovery on the same turn    |
| Runtime returns a different identity/status   | Exact post-start compare plus terminal cleanup or retained quarantine                    |
| Rejected run escapes or is replaced           | Process lease retained until proved terminal; no public quarantine projection            |
| Content leakage                               | Redacted debug/error tests with canary prompt and output strings                         |

## Test plan

Focused tests must cover:

- the exact serialized keys and every fixed profile/configuration value;
- crate-private fixed synthetic fixture construction;
- host-issued unique process-local run/request identities, exact preservation
  and 128-byte validation, wrap/exhaustion denial, process-wide busy denial,
  failed-construction lease release, fail-closed drop behavior, and no caller
  identity input;
- exactly one invocation of `AgentRuntime::start` per accepted host start and no
  second start surface;
- a production-called checked-start helper exercised with fake runtimes that
  return the exact identity/`AwaitingStart`, a wrong run ID, a wrong request ID,
  and every wrong initial status; mismatches never expose a session;
- rejected-run cancellation returning `Cancelled`, every terminal
  `AlreadyTerminal`, a nonterminal `AlreadyTerminal`, and an error; only proved
  terminal cleanup releases the lease, while ambiguous cleanup quarantines the
  run, rejects restart, permits only bounded explicit cleanup, and rejects
  access/late events;
- Drop-time transfer to the single process-owned quarantine slot, plus poisoned
  or occupied-slot fail-closed behavior with the lease retained until process
  exit and no public/background access;
- exact public `Idle -> Starting -> Streaming -> terminal` projection,
  content-free terminal summary retention, new-start clearing, and
  `Cancelled`/`AlreadyTerminal` cancellation projection;
- request byte boundary and multibyte handling;
- started, ordered deltas, and completion with exact final answer;
- per-delta, aggregate scalar, aggregate byte, and 128-event boundaries;
- failure and cancellation as terminal states;
- unknown field, malformed JSON, wrong protocol, wrong identity, wrong
  sequence, duplicate start/terminal, event-before-start, empty output, mixed
  output, and late event rejection;
- every function-call shape rejected with no registry/policy/approval/audit
  transition;
- both boxed Native variants under strict Clippy with no lint suppression, plus
  unchanged Initial-profile and closed Personal Assistant behavior for
  `accept_frame`, `cancel_pending_approval_for_run_termination`, and the macOS
  `resolve_approval_source_outcome` method;
- rejected-frame partial-state immutability followed by terminal failure; and
- canary absence from debug/errors.

Existing initial-turn unit and integration tests must remain passing.

## Verification

```bash
cargo test --manifest-path src-tauri/Cargo.toml agent::gateway_request
cargo test --manifest-path src-tauri/Cargo.toml agent::native_runtime
cargo test --manifest-path src-tauri/Cargo.toml agent::runtime
cargo test --manifest-path src-tauri/Cargo.toml --test agent_runtime_contract
cargo test --manifest-path src-tauri/Cargo.toml --test gateway_request_contract
cargo test --manifest-path src-tauri/Cargo.toml --test personal_assistant_v0_contract
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings
npm run verify
npm audit --audit-level=low
npm run security:scan
npm run repository:check
npm run docs:check
git diff --check
python3 .codex/hooks/session_end_gate.py
```

Manual diff inspection must confirm no production I/O, dependency, Tauri,
capability, CSP, provider, credential, persistence, tool, file, background, or
device-effect expansion. Target-Mac UI, networking, signing, Keychain, gateway,
and provider checks are `Not run` because the turn is transport-free.

## Dependencies and non-goals

This plan depends only on accepted D-094 and current transport-free validators.
It does not depend on credentials or provider evidence because it cannot send a
request.

No production clock/entropy guarantee, network, HTTP, SSE, Worker, OpenAI call,
Keychain, signing, Tauri IPC, WebView, typed personal input, external
disclosure, persistence, memory, tools, delegation, approval, audit, or UI is
in scope. Host identifiers are process-local correlation only and are not
security credentials.

## Rollback

Before publication, revert only the exact source/test and closeout files from
this increment. The existing initial gateway path remains the unchanged
fallback for existing tests; no external state exists to revoke.

## Stop conditions

Stop on any need to change `gateway_protocol.rs`, `definition.rs`,
`orchestrator.rs`, a manifest/lockfile, current tool schemas, existing-profile
request bytes, Tauri commands/state, frontend, capabilities, CSP,
permissions, or an external system. Also stop if implementation needs another
runtime start API, permits a profile/configuration selector, issues identity
inside the turn/runtime adapter, requires a dead-code allowance or contrived
startup call, cannot reject all function calls, cannot enforce both scalar and
byte bounds, cannot preserve transactional rejection, changes a concrete Native
approval-method signature, or cannot quarantine a returned identity/status
mismatch without releasing the process lease.

## Acceptance criteria

- [x] Exact empty-tool v1 request and text-only stream contracts exist.
- [x] All trusted identity/configuration values are application-owned.
- [x] Returned runtime identity/status is exact and every rejected run is
      terminal-cleaned or retained in quarantine with cleanup ownership.
- [x] Fixed synthetic success, failure, cancellation, limits, and late-event
      rejection pass without I/O.
- [x] Existing behavior and the full completion gate pass.
- [x] Independent architecture and security review find no blocking issue.

## Progress

- 2026-08-28: confirmed clean synchronized `main` at the exact baseline,
  recorded pinned toolchains, and began the fresh V0-1 gate.
- 2026-08-28: implemented only the sealed request/validator, private runtime
  profile split, boxed Native branch, no-input host, focused tests, and exact
  public contract test authorized by this plan.
- 2026-08-28: addressed every independent architecture and security/code
  review finding without widening scope.
- 2026-08-28: completed focused, acceptance, repository-wide, security,
  documentation, audit, build, session-end, and deterministic marker checks.

## Discoveries

- A typed optional JSON field cannot prove wire-key absence: explicit
  `retry_after_ms: null` collapsed to `None`. The Personal Assistant turn now
  performs a bounded key-presence preflight without changing the shared
  gateway protocol or Initial profile.
- Rejected-run cleanup status is cleanup proof only. It must never promote an
  unaccepted run to a public Completed or Cancelled result.
- Failed restart attempts must preserve the prior content-free terminal
  summary until a replacement run is accepted.
- The public host can truthfully prove fixed request/start/cancel ownership in
  V0-1, but stream success/failure remains fixture-only until a later approved
  host transport ingress exists.

## Documentation checklist

- [x] Architecture and security ownership match the implemented Rust boundary.
- [x] Project status, handoff, queue, plan index, roadmap, changelog, testing,
      troubleshooting, increment, and review records are reconciled.
- [x] Every current capability claim distinguishes public-host evidence from
      fixture-only evidence and lists the absent live/product boundaries.
- [x] Later plans remain Blocked and no successor authority is inferred.

## Implementation result

V0-1 is locally complete from the exact clean synchronized baseline above. The
implementation adds the sealed Personal Assistant request/validator, routes it
through the sole Native runtime start boundary, and adds the public no-input
volatile host with process lease, exact returned identity/status validation,
closed status/cancellation, and fail-closed rejected-run quarantine. The
existing Initial profile remains separately routed and passing.

The public host deliberately has no response-frame ingress. Deterministic
started/delta/completed/failed outcomes remain crate-private and Native runtime
fixtures; they do not establish a working assistant, provider call, or
user-visible conversation. No dependency, manifest, lockfile, Tauri/WebView,
provider, network, credential, persistence, memory, tool, approval, audit,
filesystem, background, or device-effect surface changed.

During review, the implementation was tightened so a failed restart preserves
the prior terminal summary until an accepted replacement, a rejected or
identity-mismatched run can never be projected as Completed or Cancelled,
explicit `retry_after_ms: null` is rejected by presence rather than collapsed
into absence, and all wrong returned identities/statuses exercise the exact
production-called checked-start helper. Independent architecture and
security/code review report no remaining findings.

The completion review records the exact focused/full validation results,
manual diff inspection, `Not run` platform/external checks, and valid marker.
The increment is uncommitted and unpublished pending separate owner authority.

## Readiness

**Complete.** V0-1 meets its local acceptance criteria and completion gate.
V0-2 remains Blocked until this exact V0-1 source baseline is published or the
owner otherwise accepts it as the authoritative baseline. No successor
implementation may begin from this status.
