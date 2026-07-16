# Increment 4U - bind initial approval run-termination

Status: Complete; published and merged at `61525bf`
Owner: Project maintainer
Last updated: 2026-07-15

## Goal

Let `InitialGatewayTurn` terminally deny and consume its exact pending approval
when a trusted future orchestrator reports run termination, without accepting a
caller-selected approval ID, choice, native result, or interaction evidence.

## User-visible outcome

None. Increment 4U adds no production caller, native dialog invocation, frontend
state, IPC command, network request, persistence, dispatch, or operating-system
action.

## Scope

- Retain the manager-assigned `ApprovalId` privately after the turn issues its
  one terminal approval presentation.
- Add one narrow idempotent turn operation for run-termination cancellation.
- Delegate that operation only to the turn's existing private
  `InMemoryApprovalManager::cancel_for_run_termination` path.
- Return only the existing exact non-authorizing `ApprovalResolution` when a
  pending subject is consumed, or no resolution when the turn owns no pending
  subject.
- Clear the retained ID only after a successful terminal manager resolution.
- Preserve manager-owned expiry precedence, exact identity, one-time
  consumption, tombstones, replay rejection, and late-native-outcome rejection.
- Add focused unit and public-contract tests for the bounded lifecycle.

## Explicit non-goals

- Native approval-dialog invocation, modification, dismissal, or stale-window
  control.
- Proactive expiry, timers, polling, background work, retries, or deadlines.
- A source trait, runtime coordinator, active-run registry, or caller-supplied
  run-liveness proof.
- Any cancellation choice, approval disposition, native result, interaction
  evidence, authentication evidence, or approval ID supplied by a caller.
- Approval audit writes, durable persistence, SQLite changes, or audit receipts.
- Dispatch, execution, tool results, provider continuation, transport, gateway
  deployment, authentication, credentials, API keys, or Keychain access.
- Tauri commands, WebView changes, dependencies, manifests, lockfiles,
  capabilities, entitlements, or permissions.
- Changing the existing lower-level manager cancellation contract or the bound
  response stream's existing local `cancel()` behavior.

## Existing behavior and constraints

- Increment 4S issues one exact manager-owned `ApprovalPresentation` for a
  terminal `RequireApproval` decision, but the turn does not retain the
  manager-assigned approval ID after returning that presentation.
- Increment 4T routes one sealed trusted native source outcome directly to the
  same private manager and returns only its exact non-authorizing resolution.
- `InMemoryApprovalManager` already exposes the exact typed
  `cancel_for_run_termination(id)` operation. It applies monotonic expiry before
  cancellation, consumes the subject once, stores a tombstone, and returns a
  cancellation resolution with no interaction evidence.
- At or after the approval deadline, `Expired` must win over
  `Cancelled(RunTerminated)`.
- A manager error must not cause the turn to forget a still-pending subject.
- No production `InitialGatewayTurn` caller exists; the public contract test is
  the only external caller.
- Approval resolution remains non-authorizing and unaudited. It cannot grant
  dispatch or execution authority.

## Files expected to change

Exact source/test scope:

```text
src-tauri/src/agent/gateway_request.rs
src-tauri/tests/gateway_request_contract.rs
```

No other source, test, manifest, lockfile, capability, entitlement, permission,
security-policy, product-architecture, workflow, or troubleshooting file may
change without renewed project-owner approval.

Declared planning and closeout scope:

```text
AGENTS.md
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
docs/increments/04u-bind-initial-approval-run-termination.md
docs/plans/04u-bind-initial-approval-run-termination.md
docs/plans/README.md
docs/reviews/2026-07-15-04u-post-increment-review.md
```

`DECISIONS.md` is limited to the durable private-ID ownership, cancellation
non-authority, idempotence, expiry precedence, and late-outcome treatment.

## Implementation steps

- [x] Obtain explicit project-owner approval for this exact plan.
- [x] Run `python3 .codex/hooks/post_increment_gate.py begin --increment 04u`
      before any source edit.
- [x] Add one private optional pending-approval ID to `InitialGatewayTurn`.
- [x] Store the exact manager-assigned ID during terminal presentation issuance
      without widening the public presentation or manager contract.
- [x] Add a narrow method named
      `cancel_pending_approval_for_run_termination` that accepts no approval ID,
      choice, native result, or evidence.
- [x] Return `Ok(None)` when no pending approval is owned; otherwise delegate to
      the private manager and clear the ID only after successful resolution.
- [x] Clear the retained ID after successful trusted native resolution; retain
      it after a typed resolution error such as foreign-manager rejection.
- [x] Add focused tests for exact cancellation facts, no evidence, idempotence,
      expiry precedence, typed-error retention, and rejection of late source
      outcomes after cancellation.
- [x] Run the exact verification suite and review the complete diff.
- [x] Run `$post-increment-gate`, synchronize the declared closeout documents,
      and require a valid `04u` completion marker before completion.

## Security and privacy considerations

- Run termination is a deny/close transition only. It cannot authorize a tool,
  create trusted interaction evidence, or imply authentication.
- The public operation must not accept an identifier or user choice because that
  would let a caller select or fabricate the approval subject or disposition.
- The private ID is lifecycle state, not authority. Manager state remains the
  source of truth for issuance, expiry, consumption, and replay.
- Clearing the ID before successful manager resolution could orphan a pending
  subject; clearing it after success prevents repeated cancellation attempts.
- A native result that arrives after run termination must remain rejected as
  already consumed. Increment 4U does not close a still-visible native dialog.
- Debug and error output must not expose task titles, arguments, raw content,
  native results, evidence, or secrets.

## Risks

- Turn and manager lifecycle state can drift if the retained ID is set or
  cleared at the wrong transition.
- Incorrect deadline handling could convert an expired subject into a
  run-terminated subject. The manager's existing expiry-first behavior must be
  preserved.
- A future caller could misread the returned cancellation resolution as audit or
  execution authority; documentation and tests must retain its non-authorizing
  meaning.
- A stale native prompt can remain visible after cancellation because native
  window control is excluded.
- O-006 and O-007 remain open: active-run orchestration and durable audit binding
  are not solved here.

## Test plan

- Public contract: cancellation before a pending approval returns no resolution.
- Public contract: cancellation after presentation returns the exact approval,
  request, run, conversation, classification, preview, and
  `Cancelled(RunTerminated)` facts with no interaction evidence.
- Public contract: repeated cancellation returns no resolution and does not
  recreate or replay the subject.
- Unit contract: a sealed native outcome created before cancellation is rejected
  as already consumed when delivered afterward, without mutating terminal state.
- Unit contract: a typed native-resolution error does not clear the retained ID,
  and a later valid same-manager terminal resolution can still consume it.
- Manager regression: the existing deadline tests continue to prove that expiry
  at the deadline remains `Expired`, not run-terminated.
- Regression: existing request, approval, approval-binding, approval-audit, and
  complete repository checks remain green.

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

No manual verification is required because the increment adds no native UI
invocation, production caller, user-visible behavior, network, credential,
persistence, dispatch, or operating-system action.

## Rollback or failure strategy

Before commit, restore the two source/test files to `244a1d8` and revert only the
declared 4U planning and closeout documents. After commit, revert the single 4U
commit. No migration, data, dependency, credential, compatibility identifier,
or remote resource requires rollback.

Stop and request renewed approval if implementation requires any file outside
the exact source/test and declared closeout scopes, any lower-level manager or
native-source behavior change, a dependency, or a user-visible/manual gate.

## Acceptance criteria

- [x] The exact `04u` gate begins before source edits.
- [x] Only the exact two source/test files change outside the declared closeout
      documentation.
- [x] The turn retains only its own exact manager-assigned pending approval ID.
- [x] The cancellation operation accepts no ID, choice, native result, or
      evidence.
- [x] Successful run termination returns the exact existing non-authorizing
      cancellation resolution with no interaction evidence.
- [x] No-pending and repeated calls are idempotent and return no resolution.
- [x] Expiry precedence and late-native-outcome rejection are preserved.
- [x] Typed resolution errors do not orphan a still-pending subject.
- [x] All focused and complete automated checks pass.
- [x] The complete diff has no scope expansion, secret, generated output, or
      blocking code/security finding.
- [x] The declared project-memory documents and review report match actual
      evidence.
- [x] The final `04u` completion marker is complete and valid.

## Actual results

Increment 4U is implemented within the exact two-file source/test scope. The
turn retains the exact manager-assigned ID after successful presentation,
exposes one no-argument idempotent run-termination operation, clears ownership
only after successful manager resolution, and clears it after successful native
resolution while retaining it after typed errors.

Focused verification passed with 9 gateway-request unit tests, 17 approval
tests, 10 public gateway-request contract tests, 2 approval-binding tests, and 1
approval-audit-binding test. Strict Clippy, complete `npm run verify`, npm audit,
formatting, diff, code, security, and mandatory gate reviews pass. No manual
verification applies. The consolidated result is `PASS WITH ADVISORIES` and the
`04u` completion marker is complete and valid.

The proposed follow-on is Increment 4V, which would bind native and
run-termination resolutions to the turn's private typed in-memory audit
adapter. 4V is not part of this scope and must not start automatically; it
requires explicit queue selection and separate project-owner approval.

Commit `61525bf208553cb17f3dfe665d3f7b7d32d306c1` with message
`Bind initial approval run termination` is pushed on
`codex/phase4-increment-4u`, fast-forward merged into `main`, and synchronized
with `origin/main`. The `04u` marker remained complete and valid after
publication.

## Documentation updates

- [x] Planning state reconciled in `AGENTS.md`, `HANDOFF.md`,
      `PROJECT_STATUS.md`, `NEXT_STEPS.md`, `CHANGELOG.md`, `PLANS.md`, and
      `docs/plans/README.md`.
- [x] Increment record and detailed plan created.
- [x] `DECISIONS.md` updated with D-042 for the confirmed private-ID ownership,
      expiry, idempotence, late-outcome, and non-authority boundary.
- [x] Post-increment review report created at
      `docs/reviews/2026-07-15-04u-post-increment-review.md`.
- [x] No troubleshooting update was required because no product or build issue
      was diagnosed. The sandboxed npm audit DNS failure passed on the required
      network-enabled retry.
