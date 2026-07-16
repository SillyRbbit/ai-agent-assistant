# Execution plan - Increment 4T bind terminal initial approval resolution

Status: **Complete**

Owner: Project maintainer

Last updated: 2026-07-15

## Goal and outcome

Make `InitialGatewayTurn` accept one sealed `TrustedApprovalSourceOutcome`
derived from the exact `ApprovalPresentation` it issued and consume that outcome
through the same private `InMemoryApprovalManager` before returning one terminal,
non-authorizing `ApprovalResolution`.

The bound initial path gains one narrow way to complete the verified
presentation-to-manager transition without exposing or reconstructing manager
ownership. A caller may still omit that method, which leaves the subject pending
and non-authorizing. Exact manager-instance, approval, run, gateway-request,
call, presentation-issued, source, and deadline checks remain solely in the
existing manager.

## User-visible outcome

None. Increment 4T invokes no native dialog and adds no production caller,
Tauri command or event, WebView behavior, live model traffic, audit write,
persistence, dispatch, executor, tool result, or operating-system action.

## Existing behavior and gap

- Increment 4S keeps terminal `RequireApproval` inside `InitialGatewayTurn`,
  creates one exact request through its private manager, and returns one owned
  `ApprovalPresentation`.
- `MacOsNativeApprovalDecisionSource` already consumes that presentation and
  returns a sealed `TrustedApprovalSourceOutcome`; callers cannot construct,
  clone, serialize, inspect, or mutate its binding facts.
- `InMemoryApprovalManager::resolve_source_outcome` already consumes the sealed
  outcome, checks pointer-identical manager ownership and exact identity,
  enforces presentation issuance and monotonic expiry, and returns one closed
  `ApprovalResolution`.
- The manager is private inside `InitialGatewayTurn`, and the turn exposes no
  method that can return the sealed outcome to that exact manager. A future
  caller could obtain the presentation and source outcome but cannot complete
  the verified manager transition without bypassing the bound turn.
- No production `InitialGatewayTurn` caller exists. Its only repository caller
  is the public gateway-request contract test.

## Exact resolution-binding contract

On macOS, `InitialGatewayTurn` gains one narrowly named method that consumes a
`TrustedApprovalSourceOutcome` and delegates it directly to
`ApprovalManager::resolve_source_outcome` on the turn's existing private
manager. The method returns the exact owned `ApprovalResolution` or maps the
existing closed `ApprovalError` through
`InitialGatewayTurnError::Approval`.

The turn must not inspect, copy, transform, compare, log, serialize, or derive
authority from the outcome. It must not accept an approval ID, choice,
disposition, interaction evidence, authentication claim, preview, or arbitrary
caller metadata. It must not substitute another manager or retry a consumed
outcome.

The existing manager remains responsible for all checks and mappings:

1. pending request exists;
2. manager-instance marker is pointer-identical;
3. source kind is the fixed macOS native dialog;
4. approval/run/request/call identity is exact;
5. one presentation was issued;
6. deadline has not passed, otherwise the result is `Expired`; and
7. the sealed source decision maps to the existing closed disposition and
   evidence.

The returned resolution remains non-authorizing data. `Approved` records only
the exact closed native-source result and existing `NotEvaluated`
authentication evidence. It grants no run-liveness, audit completion,
permission, dispatch, execution, persistence, provider continuation, or tool
result authority.

Production code in `decision_source.rs` remains unchanged. Its existing
`#[cfg(test)]` synthetic result helper may widen only from approval-module
visibility to crate visibility so tests in `gateway_request.rs` can exercise the
sealed path without opening a native dialog. The helper remains absent from
shipping builds and is not a public integration-test or runtime API.

## Exact source and test scope

Change only:

```text
src-tauri/src/agent/gateway_request.rs
src-tauri/src/approvals/decision_source.rs
```

`gateway_request.rs` owns the macOS-gated same-manager delegation and focused
unit tests. `decision_source.rs` changes only the visibility of its existing
test-only synthetic mapping helper. Do not change the approval manager,
approval types, native decision-source production behavior, public integration
tests, audit adapter, policy, gateway protocol, schema validator, registry,
tools, module exports, manifests, lockfiles, Tauri, frontend, storage,
capabilities, entitlements, or permissions. Stop and request approval before
expanding this two-file source/test scope.

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
docs/increments/04t-bind-terminal-initial-approval-resolution.md
docs/plans/04t-bind-terminal-initial-approval-resolution.md
docs/plans/README.md
docs/reviews/2026-07-15-04t-post-increment-review.md
```

`DECISIONS.md` is limited to same-manager sealed-outcome ownership,
non-authorizing resolution, macOS API gating, and test-only helper visibility.
No security, product, workflow, troubleshooting, dependency, manifest,
lockfile, Tauri, frontend, storage, capability, entitlement, or permission file
may change without separate approval.

## Implementation steps

1. After approval, run
   `python3 .codex/hooks/post_increment_gate.py begin --increment 04t` before
   source edits.
2. Add only the existing macOS-gated trusted-outcome and resolution imports
   needed by `gateway_request.rs`.
3. Add one consuming same-manager delegation method that returns the existing
   typed resolution/error without fallback or transformation.
4. Widen only the existing `#[cfg(test)]` synthetic native-result helper to
   crate visibility.
5. Add focused gateway-request unit tests for exact same-manager resolution,
   closed result/evidence mappings, cross-manager rejection without recipient
   mutation, ownership, and redaction.
6. Run focused and complete verification, review the complete diff and trust
   boundaries, synchronize closeout documentation, record any durable decision,
   and finalize the mandatory gate.

## Security and privacy considerations

- The model, gateway, WebView, and caller cannot construct or modify a trusted
  source outcome or choose its manager marker, identity, source, decision, or
  authentication evidence.
- Pointer-identical manager ownership and exact identity remain checked in the
  existing manager before mutation.
- The turn consumes the outcome by ownership. It cannot clone or replay it.
- The resolution derives from the manager-retained exact policy decision and
  retains bounded typed arguments only through the existing owned value.
- Outcome, resolution, turn, and errors remain content-redacted in debug output.
- The test-only helper is compiled only for crate unit tests. No production,
  integration-test, feature-flag, environment, IPC, or serialization bypass is
  introduced.
- `Approved` remains interaction evidence, not authorization or execution
  authority.

## Risks

- `agent::gateway_request` gains a macOS-gated dependency on the sealed native
  source outcome and approval resolution, deepening the existing trusted
  assembly coupling. A platform-neutral coordinator remains future work.
- Returning `ApprovalResolution::Approved` from the turn could be mistaken for
  dispatch authority. The API, tests, decisions, and documentation must retain
  its explicitly non-authorizing meaning.
- Widening the synthetic mapper to `pub(crate)` under `cfg(test)` increases
  test-build reachability. It must remain absent from production builds and
  must not become a public helper, Cargo feature, or integration-test bypass.
- The private manager still exposes no turn-owned run-termination cancellation
  or proactive expiry method. A no-response/stale-dialog path remains a
  production blocker and is deliberately not hidden by this increment.
- The synchronous native source can still leave a stale visible prompt after
  manager cancellation. Increment 4T does not invoke or change that source.
- Audit remains separate. A future caller could omit audit unless a later
  bounded transition keeps the exact resolution inside trusted orchestration.
- O-006 and O-007 continue to block authenticated gateway transport and live
  provider traffic.

## Explicit non-goals

- Invoking `MacOsNativeApprovalDecisionSource::request_decision`, changing the
  native dialog, button mapping, title validation, message limits, dependency,
  target gating, or target-Mac manual harness.
- Adding platform-neutral source traits, a coordinator, dependency injection,
  background tasks, threads, async runtime, dialog cancellation handle, or
  production caller.
- Exposing manager access, approval IDs, raw choices, arbitrary evidence,
  authentication claims, or constructors for trusted outcomes or resolutions.
- Run-termination cancellation, proactive expiry polling, timer scheduling,
  active-run validation, retry, replay, edit resubmission, or second approval.
- Audit recording or persistence, durable approval storage, dispatch,
  executor, real tool execution, tool-result construction, provider
  continuation, second model turn, or retry orchestration.
- HTTP, TLS, gateway service/deployment, endpoint configuration,
  authentication, credentials, Keychain, provider/OpenAI SDK parameters, live
  traffic, transport abort, or runtime deadlines.
- Tauri commands/events/plugins, WebView integration, UI, SQLite,
  dependencies, CSP, capabilities, entitlements, or permissions.

## Test plan

Preserve the six request, 17 approval, nine public gateway-request contract,
two approval-binding, and one approval-audit-binding tests. Add macOS-only unit
coverage in `gateway_request.rs` proving:

- a presentation emitted by one turn can produce a sealed synthetic native
  outcome without opening a dialog;
- the exact outcome returns to the same turn-owned manager and produces the
  existing exact disposition, identity, preview, source, button,
  authentication, and source-failure evidence for every closed native result;
- an outcome from another turn fails with typed
  `ApprovalError::ManagerInstanceMismatch` before public-ID comparison and does
  not mutate the recipient turn's pending approval;
- the recipient turn can still consume its own exact outcome after rejecting a
  foreign outcome;
- no caller-supplied approval ID, disposition, evidence, preview, or permission
  enters the method;
- outcome and resolution debug output omit title and opaque identity content;
  and
- existing request, stream, schema, policy, presentation, cancellation,
  manager lifecycle, audit-adapter, status, limit, and redaction contracts
  remain unchanged.

Do not add a public constructor, feature flag, environment bypass, mock source
trait, integration-test production hook, or actual native-dialog invocation.

No manual verification is required because the increment invokes no native UI
and changes no production caller, user-visible behavior, network, credential,
persistence, capability, permission, dispatch, or operating-system action.

## Verification commands

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_request::
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

Then run `$post-increment-gate`, finalize the exact 4T review report, and require
a complete valid marker with `PASS` or `PASS WITH ADVISORIES`.

## Rollback or failure strategy

Before commit, restore the two source/test files to `6d0bed4` and revert only the
declared 4T planning/closeout documentation. After commit, revert the single 4T
commit. No migration, data, dependency, credential, compatibility identifier,
or remote resource requires rollback.

Any need for a third source/test path, production decision-source change,
approval-manager/type change, new trait or coordinator, cancellation/expiry,
audit/dispatch integration, transport, credential, Tauri route,
product/security document, dependency, or permission change stops the increment
for project-owner approval.

## Acceptance criteria

- [x] Project owner approves the exact two-file source/test plan and closeout scope.
- [x] Mandatory 04t gate state begins before source edits.
- [x] Only a sealed trusted source outcome can enter the new turn method.
- [x] The exact outcome returns only to the same private manager and retains every existing identity, issuance, source, and deadline check.
- [x] The exact owned non-authorizing resolution or existing typed approval error is returned without fallback or transformation.
- [x] Cross-manager substitution fails without mutating the recipient pending subject.
- [x] The synthetic mapper remains test-only and absent from shipping APIs.
- [x] No native invocation, cancellation/expiry orchestration, audit, dispatch, or execution path is added.
- [x] Focused and complete checks and reviews pass.
- [x] Closeout documentation and D-041 match actual evidence.
- [x] The post-increment report passes and the 04t marker is complete and valid.

## Planning baseline

Passed on clean synchronized `main` at `6d0bed4`:

```text
python3 .codex/hooks/post_increment_gate.py status
  Increment 04s complete, valid: true, PASS WITH ADVISORIES
npm run typecheck
  passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_request::
  6 passed
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
`6d0bed41b06f7f3f79bfd8ea44c3c47b3743ebd7`. Toolchains are Node.js `v26.3.0`,
npm `11.16.0`, Cargo and rustc `1.90.0`, rustfmt `1.8.0-stable`, and Clippy
`0.1.90` on arm64 macOS `26.5.2` with Xcode Command Line Tools at
`/Library/Developer/CommandLineTools`.

At planning time, repository search confirmed the public gateway-request
contract was the only `InitialGatewayTurn` caller. The manager and native source
were independently verified, but no production source returned
`TrustedApprovalSourceOutcome` to the turn-owned manager, and no `04t` gate
state or source implementation had begun.
