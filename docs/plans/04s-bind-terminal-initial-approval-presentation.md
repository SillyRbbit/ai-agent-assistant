# Execution plan - Increment 4S bind terminal initial approval presentation

Status: **Complete; verified with uncommitted changes**

Owner: Project maintainer

Last updated: 2026-07-15

## Goal and outcome

Make `InitialGatewayTurn` consume a terminal `RequireApproval` policy decision
through the fixed existing `InMemoryApprovalManager`, create one exact request,
and issue one owned `ApprovalPresentation` before returning to a caller.

The bound initial path will no longer expose a `RequireApproval`
`PolicyDecision` for a future caller to delay, replace, or omit approval-request
creation. `Allow` and `Deny` remain closed non-authorizing policy events. The new
presentation remains proposal data only and cannot establish a user decision,
audit receipt, dispatch eligibility, or execution authority.

## User-visible outcome

None. Increment 4S adds no production caller, native-dialog invocation, Tauri
command or event, WebView behavior, live model traffic, approval result,
persistence, credential access, dispatch, executor, or operating-system
interaction.

## Existing behavior and gap

- Increment 4R keeps the schema-valid call private through accepted terminal
  completion, runs the fixed deterministic policy engine, and returns one
  `InitialGatewayEvent::PolicyEvaluated { decision }`.
- `get_current_datetime@1` produces `Allow` / `InformationOnly`.
- `create_local_task@1` produces `RequireApproval` /
  `ReversibleRequiresApproval`, but the owned decision leaves the bound turn.
- The independently verified `InMemoryApprovalManager` accepts only
  `RequireApproval`, retains the exact decision, rejects duplicate or unsupported
  subjects, enforces one pending request and a 120-second TTL, and issues one
  exact owned `ApprovalPresentation`.
- The public approval-binding test manually reconstructs protocol, schema,
  policy, request creation, and presentation issuance. No production coordinator
  or `InitialGatewayTurn` caller exists.
- A future initial transport caller could receive `RequireApproval` and omit or
  replace the verified approval-manager transition before native presentation.

## Exact approval-presentation binding contract

`InitialGatewayTurn` gains one private `InMemoryApprovalManager` constructed by
`InitialGatewayTurn::new`. No caller can select, replace, or mutate it.

After accepted terminal `response_completed` with one pending schema-valid call,
the turn must:

1. take the exact pending call;
2. construct `PolicyInput::from_validated_call(call)`;
3. evaluate it with `DeterministicPolicyEngine::new()`;
4. branch only on the locally derived `PolicyOutcome`;
5. for `RequireApproval`, pass the exact owned decision to
   `ApprovalManager::create_request`;
6. immediately pass the returned `ApprovalId` to
   `ApprovalManager::issue_presentation`; and
7. return
   `Some(InitialGatewayEvent::ApprovalPresentationReady { presentation })`.

For `Allow` and `Deny`, the turn returns the existing
`PolicyEvaluated { decision }`. Both remain non-authorizing; no dispatch or
execution conversion is added. The current exact contracts produce no `Deny`,
but the branch remains closed and explicit.

Approval-manager failures are returned as one typed
`InitialGatewayTurnError::Approval` with the existing closed, content-free
`ApprovalError`. No fallback policy event or presentation is emitted. Because the
gateway validator has already accepted its terminal frame, the stream remains
terminal and cannot retry or produce a second request. Any private pending state
has no public resolution or execution path in this increment.

`InitialGatewayEvent` stops deriving `Eq` and `PartialEq` because the exact
`ApprovalPresentation` is intentionally owned, non-cloneable, non-serializable,
and non-comparable. Its existing redacted `Debug` implementation is used from the
new event; the affected title and exact identity must remain absent from event
debug output.

Response start, text deltas, text completion, gateway failure, local schema
failure, protocol-error transactionality, pre-terminal cancellation, status,
request bytes, limits, and redaction remain unchanged. Failure and cancellation
before terminal completion still discard the pending call without policy or
approval creation.

## Exact source and test scope

Change only:

```text
src-tauri/src/agent/gateway_request.rs
src-tauri/tests/gateway_request_contract.rs
```

The request module owns the fixed terminal approval-request and presentation
transition. The public contract test proves the exact `Allow` event, exact local
task presentation, retained identity and preview facts, terminal ordering,
failure/cancellation behavior, and redaction. Do not change the approval manager,
approval types, native decision source, audit adapter, policy source, lower-level
gateway protocol, function-call validation, registry, tools, module exports,
manifests, or any other source/test path. Stop and request approval before
expanding this two-file scope.

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
docs/increments/04s-bind-terminal-initial-approval-presentation.md
docs/plans/04s-bind-terminal-initial-approval-presentation.md
docs/plans/README.md
docs/reviews/2026-07-15-04s-post-increment-review.md
```

`DECISIONS.md` is limited to terminal approval-manager ownership, exact
presentation issuance, non-authority, and public event API narrowing. No
security, product, workflow, troubleshooting, dependency, manifest, lockfile,
Tauri, frontend, storage, capability, entitlement, or permission file may change
without separate approval.

## Implementation steps

1. After approval, run `python3 .codex/hooks/post_increment_gate.py begin --increment 04s` before source edits.
2. Add only the existing approval-manager trait/type, error, presentation, and policy-outcome imports required by `gateway_request.rs`.
3. Construct one private in-memory approval manager with the bound turn.
4. Route only terminal `RequireApproval` through exact request creation and one presentation issuance.
5. Keep `Allow` and `Deny` as non-authorizing policy events and preserve every pre-terminal failure and cancellation path.
6. Update only the public gateway-request contract for exact outcome routing, presentation facts, ownership, ordering, and redaction.
7. Run focused and complete verification, review the complete diff and trust boundaries, synchronize closeout documentation, record any durable decision, and finalize the mandatory gate.

## Security and privacy considerations

- A model, gateway, WebView, or future transport caller cannot choose the
  approval manager, request ID, preview, risk, permission, outcome, or reason.
- Approval creation occurs only after terminal response completion, exact local
  schema validation, and deterministic `RequireApproval` classification.
- The manager retains the exact owned policy decision. The presentation derives
  its identity, classification, and affected title from that retained value.
- `ApprovalPresentation` remains non-cloneable and non-serializable. It carries
  no approval disposition, trusted source outcome, authentication result, audit
  receipt, run-liveness evidence, dispatch token, or execution authority.
- A dropped presentation cannot authorize anything. No native source or manager
  resolution method is exposed through the turn in this increment.
- Typed arguments remain bounded and ephemeral. Presentation and event debug
  output remain redacted.

## Risks

- `agent::gateway_request` will depend on approval types whose manager already
  depends on policy types retaining an agent-owned call. The ownership graph is
  non-recursive, but module coupling deepens at this trusted assembly boundary.
  Any need for a new coordinator stops the increment for approval.
- Removing `Eq` and `PartialEq` from the public event narrows the Rust API. The
  crate is unpublished and its only repository caller uses pattern matching, but
  a theoretical unsupported external consumer would need to migrate.
- The manager remains private after the presentation leaves the event. Trusted
  source resolution, expiry, and run cancellation cannot yet re-enter it; this
  is a deliberate non-executable intermediate boundary.
- The 120-second TTL starts at request creation. A future runtime must coordinate
  native presentation and expiry without extending or bypassing that deadline.
- Approval creation or presentation issuance occurs after terminal stream
  acceptance. A typed manager failure cannot retry the terminal frame and must
  never fall back to an exposed `RequireApproval` decision.
- `PolicyOutcome::Allow` could still be mistaken for execution authority. Tests
  and documentation must retain its non-authorizing meaning.

## Explicit non-goals

- Changing approval eligibility, previews, IDs, TTL, capacity, replay,
  presentation issuance, native-source evidence, policy rules, outcomes, reasons,
  risk classes, permissions, schemas, tools, or argument validation.
- Invoking the macOS native dialog, accepting `TrustedApprovalSourceOutcome`,
  resolving Approve/Reject/Edit/no-decision/source-failure, LocalAuthentication,
  or adding another platform source.
- Audit recording or persistence, durable approval storage, active-run
  orchestration, dispatch, executor, real tool execution, tool-result
  construction, provider continuation, second model turn, or retry orchestration.
- HTTP, TLS, gateway service/deployment, endpoint configuration, authentication,
  credentials, Keychain, provider/OpenAI SDK parameters, live traffic, transport
  abort, deadlines, or runtime scheduling.
- Tauri commands/events/plugins, WebView integration, UI, SQLite, dependencies,
  CSP, capabilities, entitlements, or permissions.
- Redesigning or privatizing lower-level gateway, schema, registry, policy,
  approval, native-source, or audit APIs; adding a coordinator, trait, module, or
  generic approval injection point.

## Test plan

Preserve the six request, 18 protocol, six function-validation, four policy, nine
tool, 17 approval, two approval-binding, and one approval-audit-binding tests.
Update the nine public gateway-request contract tests to prove:

- the function frame still returns `None` while status remains `Streaming`;
- terminal completion for `get_current_datetime@1` returns
  `PolicyEvaluated` with `Allow` / `InformationOnly`, the exact retained typed
  call, and no approval presentation;
- terminal completion for `create_local_task@1` returns exactly one
  `ApprovalPresentationReady` with manager-assigned ID, exact run/request/call
  identity, tool/version, policy outcome/reason, locally derived risk/permission,
  and the exact typed preview;
- the reversible decision itself is not returned to the caller and no event
  carries an approval disposition, audit receipt, dispatch, execution,
  credential, or permission-evidence field;
- presentation and event debug output omit the affected title and exact opaque
  identities while retaining fixed classification metadata;
- failure and cancellation after buffering expose no policy decision,
  approval request, or presentation and reject late completion;
- malformed, identity-mismatched, and out-of-sequence frames retain the pending
  call without policy or approval creation, and only the correct terminal frame
  produces one presentation; and
- text, schema-failure, status, cancellation, request, limits, and redaction
  contracts remain unchanged.

No manual verification is required because the increment has no production
caller, native-dialog invocation, user-visible behavior, network, credential,
persistence, capability, permission, dispatch, or operating-system action.

## Verification commands

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_request::
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_protocol::
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::function_call_validation::
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked policy::engine::
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked tools::
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked approvals::
cargo test --manifest-path src-tauri/Cargo.toml --test gateway_request_contract --locked
cargo test --manifest-path src-tauri/Cargo.toml --test approval_binding --locked
cargo test --manifest-path src-tauri/Cargo.toml --test approval_audit_binding --locked
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
npm run verify
npm audit --audit-level=low
git diff --check
python3 .codex/hooks/post_increment_gate.py status
```

Then run `$post-increment-gate`, finalize the exact 4S review report, and require
a complete valid marker with `PASS` or `PASS WITH ADVISORIES`.

## Rollback or failure strategy

Before commit, restore the two source/test files to `5e58edb` and revert only the
declared 4S planning/closeout documentation. After commit, revert the single 4S
commit. No migration, data, dependency, credential, compatibility identifier, or
remote resource requires rollback.

Any need for a third source/test path, approval-manager or native-source change,
new coordinator, resolution/audit/dispatch integration, transport, credential,
Tauri route, product/security document, dependency, or permission change stops
the increment for project-owner approval.

## Acceptance criteria

- [x] Project owner approves the exact two-file source/test plan and closeout scope.
- [x] Mandatory 04s gate state begins before source edits.
- [x] A terminal `RequireApproval` decision cannot leave the bound initial turn.
- [x] The exact retained decision creates one manager-owned request and one owned presentation.
- [x] `Allow` and `Deny` remain non-authorizing policy events and cannot enter approval.
- [x] Failure, cancellation, schema rejection, and protocol errors cannot create an early request or presentation.
- [x] The presentation retains exact identity, preview, risk, permission, outcome, and reason with redacted debug output.
- [x] No approval result, native interaction, audit, dispatch, or execution path is added.
- [x] Focused and complete checks and reviews pass.
- [x] Closeout documentation and D-040 match actual evidence.
- [x] The post-increment report passes and the 04s marker is complete and valid.

## Implementation verification

The mandatory `04s` gate began before either approved source/test path changed.
The implementation remained within the exact two-file source/test scope and
declared closeout scope. Six request, 18 protocol, six function-validation, four
policy, nine tool, 17 approval, nine public gateway-request contract, two
approval-binding, and one approval-audit-binding tests pass. Strict Clippy,
complete `npm run verify`, the network-enabled npm audit, formatting, whitespace,
exact-scope, conflict, secret, preserved-boundary, architecture, code, security,
documentation, and mandatory gate reviews pass.

The first post-edit formatting check found three rustfmt line-wrap differences
in the approved test file; `cargo fmt` corrected them and the required rerun
passed. The first sandboxed npm audit could not resolve the registry or write npm
logs; the approved network-enabled retry found zero vulnerabilities. No manual
verification is required because no production caller, native interaction,
user-visible behavior, network, credential, persistence, capability, permission,
dispatch, or operating-system action changed.

D-040 records the durable terminal approval-manager ownership, exact
presentation issuance, typed failure, non-authority, and public event API
narrowing. The consolidated result is `PASS WITH ADVISORIES`, and the `04s`
completion marker is complete and valid for the current uncommitted workspace.
No later increment is Ready.

## Planning baseline

Passed on clean synchronized `main` at `5e58edb`:

```text
python3 .codex/hooks/post_increment_gate.py status
  Increment 04r complete, valid: true, PASS WITH ADVISORIES
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
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked approvals::
  17 passed
cargo test --manifest-path src-tauri/Cargo.toml --test gateway_request_contract --locked
  9 passed
cargo test --manifest-path src-tauri/Cargo.toml --test approval_binding --locked
  2 passed
cargo test --manifest-path src-tauri/Cargo.toml --test approval_audit_binding --locked
  1 passed
```

Git `HEAD`, local `main`, and `origin/main` all resolve to
`5e58edbb5e4774a6b91aaf97779143bda15336b6`. Toolchains are Node.js `v26.3.0`,
npm `11.16.0`, Cargo and rustc `1.90.0`, rustfmt `1.8.0-stable`, and Clippy
`0.1.90` on arm64 macOS `26.5.2` with Xcode Command Line Tools at
`/Library/Developer/CommandLineTools`.

At the planning baseline, the nine planning-document changes correctly made the
prior `04r` workspace fingerprint stale. Its merged completion report remains
historical evidence. The mandatory `04s` state began later, before either source
or test edit, as recorded in the implementation verification above.
