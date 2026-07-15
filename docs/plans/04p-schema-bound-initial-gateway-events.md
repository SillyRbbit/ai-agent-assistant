# Execution plan - Increment 4P schema-bound initial gateway events

Status: **Complete**

Owner: Project maintainer

Last updated: 2026-07-15

## Goal and outcome

Make the verified `InitialGatewayTurn` own exact local schema validation for every
normalized initial-turn function call. A future trusted caller must not receive a
raw `UntrustedFunctionCall` from the bound turn or choose a separate
`ToolRegistry` before policy.

The result remains transport-free. `InitialGatewayTurn::accept_frame` returns one
closed initial-turn event whose function-call variant contains only a
`SchemaValidatedFunctionCall` with locally derived identity, typed arguments,
risk, and permission. Schema failure closes the wrapper terminally and exposes
only a typed content-free error. This value is not policy allowance, approval,
audit evidence, dispatch eligibility, or execution authority.

## User-visible outcome

None. Increment 4P adds no production caller, Tauri registration, IPC route,
frontend behavior, live model traffic, persistence, credential access, native
API, or operating-system interaction.

## Existing behavior and constraints

- Increment 4O binds request bytes and one `GatewayStreamValidator` to the same
  correlation IDs and exact two local tool names/version.
- The bound turn currently returns `ValidatedGatewayEvent` directly. Its
  function-call variant still contains raw argument JSON in an
  `UntrustedFunctionCall`.
- `validate_function_call` consumes that raw call and independently validates it
  against a caller-supplied `ToolRegistry`, producing the only type accepted by
  `PolicyInput`.
- No production caller exists. Repository integration tests construct registries
  manually, but a future bound-turn caller could omit local schema validation or
  choose a differently configured registry before deciding what to do with the
  raw event.
- The exact local catalog remains only `get_current_datetime@1` and
  `create_local_task@1`. Their schemas, classifications, and permission metadata
  remain owned by `ToolSchema` and `ToolDefinition`.
- Lower-level protocol, registry, and function-validation APIs remain useful for
  their focused tests and independently verified downstream boundaries. This
  increment does not redesign or privatize them.
- O-006 and O-007 continue to block authenticated transport and live provider
  traffic.

## Exact schema-bound event contract

`InitialGatewayTurn::new` constructs one private `InMemoryToolRegistry` from the
same fixed two-element `ToolSchema` array that supplies the validator's allowed
names and common version. Any impossible registration or version disagreement
fails construction with one fixed content-free configuration error. No caller can
provide, replace, or mutate this registry.

`InitialGatewayTurn::accept_frame` returns a new closed
`InitialGatewayEvent` union with exactly the current normalized variants:

- response started with one opaque provider-response ID;
- output-text delta with the existing bounded text;
- function-call completed with one owned `SchemaValidatedFunctionCall`;
- response completed; or
- response failed with the existing closed redacted `GatewayFailure`.

The method first applies the owned protocol validator. Non-function events are
converted exhaustively without changing their data contract. A function event is
immediately consumed by `validate_function_call` using the private exact registry.
No raw `UntrustedFunctionCall` leaves `InitialGatewayTurn`.

Protocol failures return the existing typed `GatewayProtocolError` through one
closed `InitialGatewayTurnError` variant and preserve existing validator
transactionality. Local schema failures return their existing
`FunctionCallValidationError` through a separate closed variant, set a private
terminal-schema-failure flag, make public status `Failed`, reject every later
frame as already terminal, and make later cancellation a no-op. This wrapper
state does not claim a gateway or provider failure and adds no transport abort.

`InitialGatewayEvent` is non-cloneable and non-serializable. Custom `Debug`
redacts output text and function arguments while retaining only closed identity,
classification, status, and failure metadata. Existing selected-content and
request-body redaction remains unchanged.

## Exact source and test scope

Change only:

```text
src-tauri/src/agent/gateway_request.rs
src-tauri/tests/gateway_request_contract.rs
```

The request module owns the exact private registry, wrapper terminal state,
schema-bound event/error types, and unit-test-preserving implementation. The
public integration test proves the crate-boundary contract. Do not change
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
docs/increments/04p-schema-bound-initial-gateway-events.md
docs/plans/04p-schema-bound-initial-gateway-events.md
docs/plans/README.md
docs/reviews/2026-07-15-04p-post-increment-review.md
```

`DECISIONS.md` is limited to a durable record of initial-turn schema ownership,
terminal local-schema failure, and the public event/error API narrowing. No
security, product, workflow, troubleshooting, dependency, manifest, lockfile,
Tauri, frontend, storage, capability, entitlement, or permission file may change
without separate approval.

## Implementation steps

1. After approval, run `python3 .codex/hooks/post_increment_gate.py begin --increment 04p` before source edits.
2. Construct and retain one exact private local registry from the same two schemas used for request/validator configuration.
3. Add the closed non-cloneable `InitialGatewayEvent` and typed `InitialGatewayTurnError` with redacted debug/error behavior.
4. Convert every normalized non-function event exhaustively and consume every function event through `validate_function_call` before returning it.
5. Add private terminal failure state for local schema rejection, including failed status, late-frame rejection, and no-op cancellation.
6. Update only the public contract test for exact typed arguments/classification, invalid schemas, terminal failure, protocol errors, cancellation, and redaction.
7. Run focused and complete verification, review the complete diff and trust boundaries, synchronize closeout documentation, record the durable decision, and finalize the mandatory gate.

## Security and privacy considerations

- Raw function argument JSON must not leave the bound turn. Only the existing
  non-cloneable typed arguments may cross to future policy code.
- The exact registry, names, versions, risk, permission, and schemas must come
  only from trusted Rust catalog values, never the model, WebView, gateway event,
  or future transport caller.
- Local schema failure must close the wrapper terminally. It must not permit a
  corrected same-sequence replay, later completion, policy conversion, or
  execution path.
- Output text and typed arguments remain content-bearing values. Their custom
  debug output must stay redacted, and no logging, persistence, IPC, or audit path
  is added.
- A schema-valid function call remains non-authorizing and must still pass
  deterministic policy, exact approval where required, trusted audit, dispatch,
  and execution boundaries.
- No credential, gateway URL, provider parameter, permission evidence, approval
  result, audit receipt, or execution field enters the event contract.

## Risks

- Mirroring normalized event variants could drift. The conversion must use one
  exhaustive Rust match so any future protocol variant causes a compile failure.
- The private registry could drift from the request tool set. Build both from the
  same fixed schema array and test exact names, common version, typed arguments,
  risk, and permission.
- Schema validation occurs after protocol state accepts the frame. A private
  wrapper terminal-failure flag must override status and reject every later
  operation so partial protocol state cannot be reused.
- Replacing the bound turn's public event and error types narrows a public Rust
  API. The crate is not published and the only repository caller is the approved
  integration test, but an unsupported external consumer would need to migrate.
- Lower-level public protocol and registry APIs remain independently
  constructible for focused and downstream tests. Future initial transport code
  must use the schema-bound turn.
- Scope could expand into policy or runtime orchestration. The new event stops at
  schema validity and local classification only.

## Explicit non-goals

- HTTP, TLS, streaming transport, gateway service/deployment, endpoint
  configuration, DNS, transport abort, retries, deadlines, or runtime scheduling.
- Gateway authentication, credentials, Keychain, LocalAuthentication, OAuth,
  principal identity, retention approval, or provider/OpenAI SDK parameters.
- Continuation requests, tool-result return, second model turn, retry requests,
  history, memory, context selection, files, attachments, or external services.
- Policy evaluation, approval-manager integration, native prompts, audit writes,
  durable persistence, dispatch, executor, or real tool execution.
- Tauri commands/events/plugins, WebView integration, UI, SQLite, dependencies,
  CSP, capabilities, entitlements, or permissions.
- Redesigning or privatizing `GatewayStreamValidator`, `ValidatedGatewayEvent`,
  `UntrustedFunctionCall`, `ToolRegistry`, or `validate_function_call` outside the
  bound initial-turn path.

## Test plan

Preserve the six exact request unit tests, 18 protocol tests, six independent
function-validation tests, nine tool tests, and two public policy-binding tests.
Update the public gateway-request contract to prove:

- exact request bytes and initial status remain unchanged;
- identity, unknown-name, and wrong-version errors remain typed protocol errors;
- both allowed tools return `SchemaValidatedFunctionCall` with exact typed
  arguments, locally derived risk, and permission;
- malformed or semantically invalid arguments for both schemas return typed local
  validation errors, set status to `Failed`, reject late frames, and make
  cancellation a no-op;
- valid text and terminal failure events preserve the existing data contract;
- cancellation remains local, idempotent, and terminal; and
- debug/error output omits selected content, output deltas, raw JSON, and task
  title sentinels.

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

Then run `$post-increment-gate`, finalize the exact 4P review report, and require
a complete valid marker with `PASS` or `PASS WITH ADVISORIES`.

## Rollback or failure strategy

Before commit, restore the two source/test files to `87be00e` and revert only the
declared 4P planning/closeout documentation. After commit, revert the single 4P
commit. No migration, data, dependency, credential, compatibility identifier, or
remote resource requires rollback.

Any need for a third source/test path, dependency, protocol-validator change,
policy/approval/audit integration, transport, credential, Tauri route,
product/security document, or permission change stops the increment for
project-owner approval.

## Acceptance criteria

- [x] Project owner approves the exact two-file source/test plan and closeout scope.
- [x] Mandatory 04p gate state begins before source edits.
- [x] The bound turn constructs one private exact registry from its fixed schemas.
- [x] No raw function call leaves the bound turn; valid calls carry typed local arguments/classification.
- [x] Local schema failure is typed, content-free, terminal, and rejects every later operation.
- [x] Protocol errors and non-function events preserve their verified contracts.
- [x] Debug and errors expose no selected content, output delta, raw arguments, or typed argument content.
- [x] No transport, credential, continuation, policy, approval, audit, coordinator, Tauri, persistence, execution, dependency, capability, or permission path is added.
- [x] Focused and complete checks and reviews pass.
- [x] Closeout documentation and the durable decision match actual evidence.
- [x] The post-increment report passes and the 04p marker is complete and valid.

## Completion evidence

`InitialGatewayTurn` now owns an exact private registry built from the same fixed
schema array used for response validation. It returns only closed
`InitialGatewayEvent` values; function events contain one
`SchemaValidatedFunctionCall`. Local schema rejection is typed, content-free,
terminal at the wrapper, and makes late frames fail plus cancellation no-op.

Six request, 18 protocol, six function-validation, nine tool, eight public
contract, and two policy-binding tests pass. Clippy passes with warnings denied;
complete `npm run verify` passes with 17 hook, 124 frontend, 92 Rust library, and
19 Rust integration tests plus builds and Tauri release no-bundle; npm audit
reports zero vulnerabilities. Exact-scope, conflict, secret, generated-output,
complete-diff, architecture, security, code-health, and documentation reviews
have no blocking finding. No manual check is required. D-037 records the durable
decision, and the consolidated result is `PASS WITH ADVISORIES`.

## Planning baseline

Passed on clean synchronized `main` at `87be00e` before planning edits:

```text
python3 .codex/hooks/post_increment_gate.py status
  Increment 04o complete, valid: true, PASS WITH ADVISORIES
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
  6 passed
cargo test --manifest-path src-tauri/Cargo.toml --test policy_input_binding --locked
  2 passed
rg -n 'InitialGatewayEvent|InitialGatewayTurnError|SchemaBoundGateway|schema-bound initial' src-tauri/src src-tauri/tests docs --glob '!docs/reviews/**'
  no existing schema-bound initial event abstraction; required exit status 1
```

Git `HEAD`, local `main`, and `origin/main` all resolved to
`87be00e6b06767cc4a276a683546e097958cf6fa`. Toolchains are Node.js `v26.3.0`,
npm `11.16.0`, Cargo and rustc `1.90.0`, rustfmt `1.8.0-stable`, and Clippy
`0.1.90` on arm64 macOS `26.5.2` with Xcode Command Line Tools at
`/Library/Developer/CommandLineTools`.
