# Execution plan - Increment 4R bind terminal initial function call to policy

Status: **Complete; verified and uncommitted**

Owner: Project maintainer

Last updated: 2026-07-15

## Goal and outcome

Make `InitialGatewayTurn` consume a terminally completed, locally
schema-validated function call through the fixed `DeterministicPolicyEngine`
before returning it to a caller.

The bound turn continues to retain the one typed call privately after the
non-terminal function frame. Only when the same validator accepts terminal
`response_completed` does the turn construct `PolicyInput`, run deterministic
policy, and return one owned `PolicyDecision`. A future initial-turn caller can
no longer receive a standalone `SchemaValidatedFunctionCall` or choose, replace,
or omit policy evaluation on that path.

## User-visible outcome

None. Increment 4R adds no production caller, Tauri command or event, WebView
behavior, live model traffic, approval prompt, persistence, credential access,
native API, dispatch, executor, or operating-system interaction.

## Existing behavior and gap

- Increment 4Q keeps a schema-validated function call private until terminal
  response completion and discards it on failure or cancellation.
- Terminal completion currently returns
  `InitialGatewayEvent::FunctionCallCompleted { call }` with an owned
  `SchemaValidatedFunctionCall`.
- Deterministic policy is separately verified: a caller may consume a validated
  call through `PolicyInput::from_validated_call` and
  `DeterministicPolicyEngine::evaluate`.
- The only `InitialGatewayTurn` caller is the public gateway-request contract
  test. No production coordinator exists.
- A future initial transport caller could receive the terminal call and delay,
  replace, or omit the canonical policy step before approval orchestration.
- D-023 already fixes policy construction, classification, and conservative
  outcomes. This increment binds that existing decision to the verified initial
  turn; it does not redesign policy.

## Exact policy-binding contract

`InitialGatewayTurn` retains its private
`Option<SchemaValidatedFunctionCall>` pending slot and existing terminal-release
ordering.

On accepted terminal completion with a pending call, the turn must:

1. take the exact pending call;
2. construct `PolicyInput::from_validated_call(call)`;
3. evaluate it with a locally selected `DeterministicPolicyEngine::new()`; and
4. return `Some(InitialGatewayEvent::PolicyEvaluated { decision })`.

`InitialGatewayEvent::FunctionCallCompleted` is replaced on the bound initial
path. No standalone owned call leaves `InitialGatewayTurn`; the exact typed call
remains retained inside `PolicyDecision` and is available only through its
existing borrowed accessor.

The exact current outcomes remain:

- `get_current_datetime@1` -> `Allow` / `InformationOnly`; and
- `create_local_task@1` -> `RequireApproval` /
  `ReversibleRequiresApproval`.

`Allow` is explicitly non-authorizing. The decision grants no approval, audit,
dispatch, execution, permission, run-liveness, or user-intent evidence.

Response start, text deltas, text completion, gateway failure, local schema
failure, protocol-error transactionality, cancellation, status, request bytes,
limits, and redaction remain unchanged. Failure and cancellation still discard
the pending call without policy evaluation.

## Exact source and test scope

Change only:

```text
src-tauri/src/agent/gateway_request.rs
src-tauri/tests/gateway_request_contract.rs
```

The request module owns the fixed terminal policy transition. The public contract
test proves both exact tool outcomes, retained typed identity and arguments,
terminal ordering, unchanged non-function behavior, and redaction. Do not change
policy source, the lower-level gateway protocol, function-call validation,
registry, tools, approvals, audit, module exports, manifests, or any other
source/test path. Stop and request approval before expanding this two-file scope.

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
docs/increments/04r-bind-terminal-initial-policy.md
docs/plans/04r-bind-terminal-initial-policy.md
docs/plans/README.md
docs/reviews/2026-07-15-04r-post-increment-review.md
```

`DECISIONS.md` is limited to terminal initial-turn policy ownership, fixed
deterministic-engine selection, the non-authorizing decision boundary, and public
event API narrowing. No security, product, workflow, troubleshooting, dependency,
manifest, lockfile, Tauri, frontend, storage, capability, entitlement, or
permission file may change without separate approval.

## Implementation steps

1. After approval, run `python3 .codex/hooks/post_increment_gate.py begin --increment 04r` before source edits.
2. Import only the existing deterministic engine trait/type and policy input/decision types into `gateway_request.rs`.
3. Replace the terminal standalone-call event with one closed policy-decision event.
4. Evaluate only after accepted terminal completion; retain all 4Q failure, cancellation, protocol-retry, text, schema-failure, status, and redaction behavior.
5. Update only the public gateway-request contract for exact `Allow` and `RequireApproval` decisions, retained call facts, terminal ordering, and non-authority fields.
6. Run focused and complete verification, review the complete diff and trust boundaries, synchronize closeout documentation, record the durable decision, and finalize the mandatory gate.

## Security and privacy considerations

- A model or gateway proposal cannot choose the policy engine, policy input,
  risk, permission, outcome, or reason.
- Policy runs only after terminal response completion and exact local schema
  validation. Failure, cancellation, truncation, and rejected protocol frames
  cannot create a decision.
- The policy decision retains the exact owned call; callers cannot substitute
  identity, arguments, classification, or permission between validation and
  policy.
- Typed arguments remain bounded, non-cloneable, non-serializable, and redacted
  from debug and errors.
- `Allow` remains data only. No consuming conversion to dispatch or execution is
  added.
- No credential, gateway URL, provider parameter, permission evidence, approval
  result, audit receipt, tool result, or execution field enters the contract.

## Risks

- `agent::gateway_request` will depend on existing policy types while policy
  types already retain an agent-owned validated-call type. The ownership graph is
  non-recursive, but the cross-module dependency must remain limited to this
  trusted assembly boundary; any need for a new coordinator module stops the
  increment for approval.
- Replacing the public event variant narrows the Rust API. The crate is
  unpublished and its only repository caller is the approved integration test,
  but a theoretical unsupported external consumer would need to migrate.
- `PolicyOutcome::Allow` could be mistaken for execution authority. Types,
  documentation, tests, and debug output must preserve its non-authorizing
  meaning.
- Policy could accidentally run before terminal completion or after a failure.
  Tests must prove there is no decision at the function frame and none on failure
  or cancellation.
- Lower-level validators and policy constructors remain public for focused tests.
  Future initial transport code must own `InitialGatewayTurn` rather than
  reconstructing the path.
- Scope could expand into approval or runtime orchestration. The increment ends
  at one retained policy decision.

## Explicit non-goals

- Changing policy rules, outcomes, reasons, risk classes, permissions, schemas,
  tools, argument validation, or conservative defaults.
- Approval-manager integration, native approval presentation, trusted interaction
  results, LocalAuthentication, audit writes or persistence.
- Dispatch, executor, real tool execution, tool-result construction, provider
  continuation, second model turn, or retry orchestration.
- HTTP, TLS, gateway service/deployment, endpoint configuration, authentication,
  credentials, Keychain, provider/OpenAI SDK parameters, live traffic, transport
  abort, deadlines, or runtime scheduling.
- Tauri commands/events/plugins, WebView integration, UI, SQLite, dependencies,
  CSP, capabilities, entitlements, or permissions.
- Redesigning or privatizing lower-level gateway, schema, registry, policy, or
  approval APIs; adding a coordinator, trait, module, or generic policy injection
  point.

## Test plan

Preserve the six request, 18 protocol, six function-validation, four policy,
nine tool, two policy-input binding, and two approval-binding tests. Update the
nine public gateway-request contract tests to prove:

- the function frame still returns `None` while status remains `Streaming`;
- terminal completion for `get_current_datetime@1` returns one
  `PolicyEvaluated` decision with `Allow` / `InformationOnly` and the exact
  retained typed call;
- terminal completion for `create_local_task@1` returns one decision with
  `RequireApproval` / `ReversibleRequiresApproval`, exact identity, locally
  derived classification, and redacted typed arguments;
- failure and cancellation after buffering expose no decision and reject late
  completion;
- malformed, identity-mismatched, and out-of-sequence frames retain the pending
  call without policy evaluation, and the correct terminal frame produces one
  decision;
- text, schema-failure, status, cancellation, request, and redaction contracts
  remain unchanged; and
- no event exposes an owned standalone schema-validated call, approval,
  dispatch, execution, credential, or permission-evidence field.

No manual verification is required because the increment has no production
caller or user-visible, network, credential, native, persistence, or
operating-system behavior.

## Verification commands

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_request::
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_protocol::
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::function_call_validation::
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked policy::engine::
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked tools::
cargo test --manifest-path src-tauri/Cargo.toml --test gateway_request_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test policy_input_binding --locked
cargo test --manifest-path src-tauri/Cargo.toml --test approval_binding --locked
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
npm run verify
npm audit --audit-level=low
git diff --check
python3 .codex/hooks/post_increment_gate.py status
```

Then run `$post-increment-gate`, finalize the exact 4R review report, and require
a complete valid marker with `PASS` or `PASS WITH ADVISORIES`.

## Implementation result

The exact two-file source/test implementation is complete. Accepted terminal
completion consumes the pending call through `PolicyInput` and the fixed
`DeterministicPolicyEngine`, then returns one closed `PolicyEvaluated` event.
Both local tool outcomes and exact retained typed facts pass. Function-frame
withholding, protocol-error retention, failure/cancellation discard, text,
schema rejection, status, limits, and redaction remain unchanged.

All focused commands, Clippy with warnings denied, complete `npm run verify`, npm
audit, conflict, secret, preserved-boundary, exact-scope, complete-diff, code,
security, documentation, and mandatory gate reviews pass. No manual verification
is required. D-039 records terminal policy ownership, fixed engine selection,
non-authority, and API narrowing. The consolidated result is
`PASS WITH ADVISORIES`, and the `04r` marker is complete and valid for the
current uncommitted workspace.

## Rollback or failure strategy

Before commit, restore the two source/test files to `8598612` and revert only the
declared 4R planning/closeout documentation. After commit, revert the single 4R
commit. No migration, data, dependency, credential, compatibility identifier, or
remote resource requires rollback.

Any need for a third source/test path, policy-rule change, new module or trait,
approval/audit integration, transport, credential, Tauri route, product/security
document, dependency, or permission change stops the increment for project-owner
approval.

## Acceptance criteria

- [x] Project owner approves the exact two-file source/test plan and closeout scope.
- [x] Mandatory 04r gate state begins before source edits.
- [x] No standalone owned schema-validated call leaves the bound initial turn.
- [x] Policy evaluation occurs only after accepted terminal response completion.
- [x] Both exact local tools produce the existing deterministic outcome and reason.
- [x] Failure, cancellation, schema rejection, and protocol errors cannot create an early decision.
- [x] The decision retains exact call identity, typed arguments, risk, and permission with redacted debug output.
- [x] `Allow` remains non-authorizing and no approval, audit, dispatch, or execution path is added.
- [x] Focused and complete checks and reviews pass.
- [x] Closeout documentation and the durable decision match actual evidence.
- [x] The post-increment report passes and the 04r marker is complete and valid.

## Planning baseline

Passed on clean synchronized `main` at `8598612` before planning edits:

```text
python3 .codex/hooks/post_increment_gate.py status
  Increment 04q complete, valid: true, PASS WITH ADVISORIES
npm run typecheck
  passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_request::
  6 passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_protocol::
  18 passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::function_call_validation::
  6 passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked policy::engine::
  4 passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked tools::
  9 passed
cargo test --manifest-path src-tauri/Cargo.toml --test gateway_request_contract --locked
  9 passed
cargo test --manifest-path src-tauri/Cargo.toml --test policy_input_binding --locked
  2 passed
cargo test --manifest-path src-tauri/Cargo.toml --test approval_binding --locked
  2 passed
```

Git `HEAD`, local `main`, and `origin/main` all resolved to
`8598612187c99aacad3a2d2a3dbefb6b5c3b87dd`. Toolchains are Node.js `v26.3.0`,
npm `11.16.0`, Cargo and rustc `1.90.0`, rustfmt `1.8.0-stable`, and Clippy
`0.1.90` on arm64 macOS `26.5.2` with Xcode Command Line Tools at
`/Library/Developer/CommandLineTools`.
