# Execution plan - Increment 4Q terminally release initial function call

Status: **Verified complete; uncommitted**

Owner: Project maintainer

Last updated: 2026-07-15

## Goal and outcome

Prevent a locally schema-validated initial-turn function call from leaving
`InitialGatewayTurn` before the normalized gateway stream reaches its required
terminal `response_completed` event.

The turn privately buffers the one bounded `SchemaValidatedFunctionCall` after
accepting and validating the non-terminal function frame. `accept_frame` returns
`Ok(None)` for that frame and releases the owned call only when the existing
protocol validator accepts terminal completion. Gateway failure or local
cancellation discards the buffered call. No future caller can begin policy,
approval, audit, dispatch, or execution from a proposal whose enclosing response
later fails or never completes.

## User-visible outcome

None. Increment 4Q adds no production caller, Tauri registration, IPC route,
frontend behavior, live model traffic, persistence, credential access, native
API, or operating-system interaction.

## Existing behavior and gap

- Increment 4P makes the bound turn own exact local schema validation and prevents
  raw normalized argument JSON or caller-selected registries from leaving that
  path.
- The normalized protocol requires exactly one terminal response event. A
  `function_call_completed` event is accepted while validator status remains
  `Streaming`; a later `response_completed` or `response_failed` event is still
  required.
- The current bound turn returns `InitialGatewayEvent::FunctionCallCompleted`
  immediately from that non-terminal function frame. The only repository caller,
  `gateway_request_contract`, receives the typed call at sequence 1 without
  sending terminal sequence 2.
- A future caller could therefore start deterministic policy or approval before
  learning that the enclosing response failed, was cancelled, or never reached
  terminal completion.
- The protocol already enforces one function call, no mixed text/function output,
  contiguous sequencing, terminal state, and all relevant bounds. This increment
  does not change or duplicate that state machine.

## Exact terminal-release contract

`InitialGatewayTurn` adds one private
`Option<SchemaValidatedFunctionCall>` pending slot. It remains non-cloneable and
non-serializable and is omitted from debug output.

`InitialGatewayTurn::accept_frame` returns
`InitialGatewayTurnResult<Option<InitialGatewayEvent>>`:

- `response_started` and output-text deltas return `Some` with their existing
  closed events;
- a valid function frame is normalized, locally schema-validated, moved into the
  private pending slot, and returns `None`;
- terminal `response_completed` with a pending call takes and returns that call as
  `Some(InitialGatewayEvent::FunctionCallCompleted { call })`;
- terminal `response_completed` after text returns the existing
  `Some(InitialGatewayEvent::ResponseCompleted)`;
- terminal `response_failed` drops any pending call before returning the existing
  closed failure event; and
- successful local cancellation drops any pending call before returning `true`.

`None` has exactly one meaning: the function frame was accepted and its validated
call is withheld pending terminal completion. It is not success authority,
policy allowance, approval, audit evidence, dispatch eligibility, execution
authority, or evidence that the provider completed the response.

Protocol rejection remains transactional. A recoverable malformed, mismatched,
or out-of-sequence frame after the function frame leaves the pending call private
and permits only the protocol's existing correct next frame. Local schema failure
retains Increment 4P's terminal wrapper behavior and never populates the pending
slot.

## Exact source and test scope

Change only:

```text
src-tauri/src/agent/gateway_request.rs
src-tauri/tests/gateway_request_contract.rs
```

The request module owns pending-call retention, release, and discard behavior.
The public integration test proves the crate-boundary contract. Do not change
`gateway_protocol.rs`, `function_call_validation.rs`, `tools/`, policy,
approvals, audit, module exports, manifests, or any other source/test path. Stop
and request approval before expanding this two-file scope.

## Exact implementation closeout scope

In addition to the two source/test paths, closeout may change only:

```text
AGENTS.md
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
docs/increments/04q-terminally-release-initial-function-call.md
docs/plans/04q-terminally-release-initial-function-call.md
docs/plans/README.md
docs/reviews/2026-07-15-04q-post-increment-review.md
```

`DECISIONS.md` is limited to terminal ownership of the buffered initial function
call, failure/cancellation discard behavior, and the public `Option` return API
narrowing. No security, product, workflow, troubleshooting, dependency,
manifest, lockfile, Tauri, frontend, storage, capability, entitlement, or
permission file may change without separate approval.

## Implementation steps

1. After approval, run `python3 .codex/hooks/post_increment_gate.py begin --increment 04q` before source edits.
2. Add one private optional pending schema-validated call to `InitialGatewayTurn`.
3. Change frame acceptance to return optional public events and buffer a valid function call instead of releasing it.
4. Release the buffered call only when the owned validator accepts terminal response completion.
5. Drop the pending call on gateway failure and successful local cancellation while preserving Increment 4P schema-failure behavior.
6. Update only the public contract test for withholding, terminal release, protocol retry, failure/cancellation discard, text behavior, state, and redaction.
7. Run focused and complete verification, review the complete diff and trust boundaries, synchronize closeout documentation, record the durable decision, and finalize the mandatory gate.

## Implementation result

- Mandatory `04q` gate state began before source edits.
- `InitialGatewayTurn` owns one private optional pending call and returns optional
  closed events.
- Both exact local function calls return `None` from the non-terminal function
  frame and remain private while status is `Streaming`.
- Accepted terminal completion takes and releases the exact typed call once.
- Accepted failure and successful cancellation discard the pending call and
  reject every late frame through terminal validator state.
- Malformed, request-mismatched, and out-of-sequence frames retain the pending
  call transactionally for the correct terminal completion.
- Text completion, local schema failure, status, cancellation idempotence, and
  content redaction remain intact.
- Source/test changes remain exactly within the approved two-file scope. No
  production caller or user-visible behavior was added.

## Security and privacy considerations

- No schema-validated call may become caller-visible before terminal response
  completion. The model, gateway, and a truncated stream cannot create local
  authority through a non-terminal proposal.
- The pending call contains bounded typed arguments and remains ephemeral,
  non-cloneable, non-serializable, and absent from debug, errors, logs, audit,
  persistence, and IPC.
- Gateway failure and cancellation must eagerly drop the pending call. A late
  completion cannot recover or release it because the validator is terminal.
- A protocol error that does not mutate validator state must not discard the
  pending call; the correct contiguous terminal frame may still complete the same
  response.
- Terminal release establishes only completed response plus local schema validity.
  The call must still pass deterministic policy, exact approval where required,
  trusted audit, dispatch, and execution boundaries.
- No credential, gateway URL, provider parameter, permission evidence, approval
  result, audit receipt, or execution field enters the event contract.

## Risks

- Returning `Option<InitialGatewayEvent>` narrows a public Rust API. The crate is
  unpublished and the only repository caller is the approved integration test,
  but a theoretical unsupported external consumer would need to handle `None`.
- `None` could be misread as an ignored event. Documentation and public tests must
  establish its single pending-call meaning and require continued frame intake.
- Buffering extends typed-argument lifetime from function-frame acceptance to
  terminal completion. Existing argument and turn bounds cap that retention; all
  terminal non-success paths must drop it.
- Failure or cancellation could accidentally leave the pending call retained.
  Tests must prove discard and late-frame rejection without exposing a pending
  accessor.
- A protocol error after buffering must preserve transactionality. Tests must
  prove a rejected sequence does not release or discard the call and that the
  correct terminal frame still releases it exactly once.
- Scope could expand into policy or runtime orchestration. Terminal release stops
  at schema validity and adds no consuming policy transition.

## Explicit non-goals

- Deterministic policy evaluation, approval-manager integration, native prompts,
  audit writes or persistence, dispatch, executor, or real tool execution.
- HTTP, TLS, streaming transport, gateway service/deployment, endpoint
  configuration, DNS, transport abort, retries, deadlines, or runtime scheduling.
- Gateway authentication, credentials, Keychain, LocalAuthentication, OAuth,
  principal identity, retention approval, or provider/OpenAI SDK parameters.
- Continuation requests, tool-result return, second model turn, retry requests,
  history, memory, context selection, files, attachments, or external services.
- Tauri commands/events/plugins, WebView integration, UI, SQLite, dependencies,
  CSP, capabilities, entitlements, or permissions.
- Redesigning or privatizing `GatewayStreamValidator`, `ValidatedGatewayEvent`,
  `UntrustedFunctionCall`, `ToolRegistry`, `validate_function_call`, or policy
  types outside the bound initial-turn path.

## Test plan

Preserve the six exact request unit tests, 18 protocol tests, six independent
function-validation tests, nine tool tests, and two public policy-binding tests.
Update the public gateway-request contract to prove:

- exact request bytes, response identity, text streaming, and initial status
  remain unchanged;
- both allowed function frames return `None`, retain `Streaming` status, and
  expose neither typed arguments nor function debug content;
- terminal completion releases exactly one schema-validated call with exact
  identity, typed arguments, locally derived risk, and permission, then reports
  `Completed` and rejects late frames;
- malformed or out-of-sequence frames after buffering return typed protocol
  errors without release or discard, and the correct terminal frame still
  releases the same call once;
- gateway failure after buffering returns only the closed failure, reports
  `Failed`, and never releases the call;
- cancellation after buffering is local, idempotent, terminal, discards the call,
  and rejects late completion;
- local schema rejection remains typed, redacted, and terminal with no pending
  call; and
- debug/error output omits selected content, output deltas, raw JSON, buffered
  arguments, and task-title sentinels.

No manual verification is required because the increment has no production
caller or user-visible, network, credential, native, persistence, or
operating-system behavior.

## Verification commands

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_request::
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_protocol::
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::function_call_validation::
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked tools::
cargo test --manifest-path src-tauri/Cargo.toml --test gateway_request_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test policy_input_binding --locked
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
npm run verify
npm audit --audit-level=low
git diff --check
python3 .codex/hooks/post_increment_gate.py status
```

Then run `$post-increment-gate`, finalize the exact 4Q review report, and require
a complete valid marker with `PASS` or `PASS WITH ADVISORIES`.

## Rollback or failure strategy

Before commit, restore the two source/test files to `8c1a2e0` and revert only the
declared 4Q planning/closeout documentation. After commit, revert the single 4Q
commit. No migration, data, dependency, credential, compatibility identifier, or
remote resource requires rollback.

Any need for a third source/test path, dependency, protocol-validator change,
policy/approval/audit integration, transport, credential, Tauri route,
product/security document, or permission change stops the increment for
project-owner approval.

## Acceptance criteria

- [x] Project owner approves the exact two-file source/test plan and closeout scope.
- [x] Mandatory 04q gate state begins before source edits.
- [x] A valid function call remains private until terminal response completion.
- [x] Terminal completion releases the exact buffered schema-validated call once.
- [x] Failure and cancellation discard the pending call and reject late release.
- [x] Transactional protocol errors preserve the pending call for the correct next terminal frame.
- [x] Text, schema-failure, status, cancellation, and redaction contracts remain intact.
- [x] No policy, approval, audit, transport, coordinator, Tauri, persistence, execution, dependency, capability, or permission path is added.
- [x] Focused and complete checks and reviews pass.
- [x] Closeout documentation and the durable decision match actual evidence.
- [x] The post-increment report passes and the 04q marker is complete and valid.

## Planning baseline

Passed on clean synchronized `main` at `8c1a2e0` before planning edits:

```text
python3 .codex/hooks/post_increment_gate.py status
  Increment 04p complete, valid: true, PASS WITH ADVISORIES
npm run typecheck
  passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_request::
  6 passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_protocol::
  18 passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::function_call_validation::
  6 passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked tools::
  9 passed
cargo test --manifest-path src-tauri/Cargo.toml --test gateway_request_contract --locked
  8 passed
cargo test --manifest-path src-tauri/Cargo.toml --test policy_input_binding --locked
  2 passed
```

Git `HEAD`, local `main`, and `origin/main` all resolved to
`8c1a2e0c082e5ef996c3f0e29f56785a884e3b94`. Toolchains are Node.js `v26.3.0`,
npm `11.16.0`, Cargo and rustc `1.90.0`, rustfmt `1.8.0-stable`, and Clippy
`0.1.90` on arm64 macOS `26.5.2` with Xcode Command Line Tools at
`/Library/Developer/CommandLineTools`.
