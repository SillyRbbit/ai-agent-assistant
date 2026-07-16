# Increment 4V - bind initial terminal approval audit

Status: Blocked; merged 4U prerequisite satisfied, separate approval required
Owner: Project maintainer
Last updated: 2026-07-15

## Goal

Require every successful approval resolution produced by
`InitialGatewayTurn`--sealed native-source resolution or the run-termination
resolution implemented by 4U--to be validated and recorded by the turn's private
typed in-memory approval-audit adapter before a caller receives it.

## User-visible outcome

None. Increment 4V adds no production caller, native dialog invocation,
frontend state, IPC command, network request, durable persistence, dispatch, or
operating-system action.

## Scope

- Add one private `InMemoryApprovalAuditAdapter` owned by
  `InitialGatewayTurn`.
- Add one closed, non-cloneable audited-resolution value that owns the exact
  `ApprovalResolution` and its `ApprovalAuditReceipt`.
- Route successful trusted native resolution through one private
  manager-then-audit helper.
- Route successful run-termination resolution from verified 4U through the same
  helper.
- Return no standalone successful resolution from either path.
- Preserve the receipt's non-authorizing sequence-only meaning and keep the
  adapter volatile and turn-local.
- Add focused unit and public-contract tests for exact resolution/receipt
  binding, record facts, duplicate prevention, error behavior, and redaction.

## Explicit non-goals

- Durable audit storage, SQLite schema or repository work, transactions,
  recovery, retention, export, or cross-run sequencing.
- Native approval-dialog invocation, modification, dismissal, or stale-window
  control.
- Proactive expiry, clocks, timers, polling, retries, background work, or
  deadlines.
- A runtime coordinator, active-run registry, caller-supplied run-liveness
  proof, source trait, or platform abstraction.
- Dispatch, execution, tool results, provider continuation, transport, gateway
  deployment, authentication, credentials, API keys, or Keychain access.
- Tauri commands, WebView changes, dependencies, manifests, lockfiles,
  capabilities, entitlements, or permissions.
- Changing lower-level approval-manager, native-source, audit-adapter, policy,
  schema, or gateway-protocol contracts.

## Existing behavior and constraints

- Increment 4T lets the turn return one sealed native source outcome to its
  private issuing manager and returns the exact non-authorizing resolution.
- Increment 4U is published and merged at `61525bf` and adds one no-argument
  run-termination path for the turn's privately retained pending approval.
- The verified `InMemoryApprovalAuditAdapter` accepts all supported terminal
  resolution facts, including recognized native outcomes, run termination, and
  expiry; it returns only a non-authorizing sequence receipt.
- One initial turn can own at most one approval subject, so a fresh private
  adapter cannot legitimately reach its 1,024-record capacity or sequence
  limit.
- Manager terminalization occurs before audit recording. A typed audit error
  cannot restore the manager subject and must not cause the turn to retain a
  stale pending ID.
- Approval resolution and audit receipt remain insufficient for dispatch,
  execution, provider continuation, or durable audit claims.

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
docs/increments/04v-bind-initial-terminal-approval-audit.md
docs/plans/04v-bind-initial-terminal-approval-audit.md
docs/plans/README.md
docs/reviews/2026-07-15-04v-post-increment-review.md
```

`DECISIONS.md` is limited to the durable turn-owned audit ordering,
manager-terminalization failure boundary, volatility, and receipt
non-authority.

## Implementation steps

- [x] Verify Increment 4U is merged with a valid completion marker.
- [x] Reconcile this plan against the exact merged 4U API.
- [ ] Obtain separate project-owner approval for this exact 4V scope.
- [ ] Run `python3 .codex/hooks/post_increment_gate.py begin --increment 04v`
      before source edits.
- [ ] Add one private turn-owned typed in-memory audit adapter.
- [ ] Add one closed audited-resolution value with read-only resolution and
      receipt accessors and redacted debug output.
- [ ] Centralize manager-success-to-audit-success conversion in one private
      helper.
- [ ] Route native and run-termination success paths through that helper.
- [ ] Clear pending turn ownership after manager terminalization even if audit
      recording returns a typed error.
- [ ] Add focused tests for both paths, exact stored facts, receipt sequence,
      impossible-to-bypass return types, audit failure behavior, and redaction.
- [ ] Run the exact verification suite and review the complete diff.
- [ ] Run `$post-increment-gate`, synchronize declared closeout documents, and
      require a valid `04v` completion marker before completion.

## Security and privacy considerations

- Audit validates and records terminal evidence; it does not authorize an
  action. `Approved` remains non-executable without later run-liveness, durable
  audit, and dispatch boundaries.
- A caller must not be able to request a receipt for a caller-authored
  resolution or pair one resolution with another receipt.
- The adapter must receive the exact manager-owned resolution by reference;
  no identity, disposition, evidence, or preview field may be reconstructed.
- Debug and errors must not expose task titles, arguments, preview content,
  native results, interaction evidence details, or secrets.
- The adapter is volatile. Documentation must not imply crash durability,
  persistence, compliance retention, or global ordering.

## Risks

- Manager success followed by audit failure is not transactional. The method
  must fail closed, drop the resolution, and clear stale turn ownership while
  reporting a typed audit error.
- Incorrect helper reuse could allow one path to return an unaudited resolution
  or record one resolution twice.
- A future caller could treat a receipt as execution authority or durable audit
  evidence.
- Adding the adapter to the request module increases assembly-boundary coupling;
  a later coordinator should assume this ownership explicitly rather than
  duplicate it.
- O-006 and O-007 remain open and continue to block live provider traffic.

## Test plan

- Public contract: run termination returns one audited resolution whose exact
  identity and disposition match the private manager subject and whose receipt
  sequence is one.
- Public contract: no-pending and repeated run-termination calls produce no new
  result or receipt.
- Unit contract: every sealed native outcome returns one matching audited
  resolution and record.
- Unit contract: native and run-termination paths cannot return a successful
  standalone resolution.
- Unit contract: a typed audit failure after manager terminalization returns no
  resolution and does not retain a stale pending subject.
- Unit contract: record and result debug output remain redacted.
- Regression: existing request, approval, approval-binding, approval-audit, and
  complete repository checks remain green.

## Verification commands

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_request::
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked audit::approval::
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
durable persistence, dispatch, or operating-system action.

## Rollback or failure strategy

Before commit, restore the two source/test files to the verified merged 4U
commit and revert only the declared 4V planning and closeout documents. After
commit, revert the single 4V commit. No migration, data, dependency, credential,
compatibility identifier, or remote resource requires rollback.

Stop and request renewed approval if implementation requires any file outside
the exact source/test and declared closeout scopes, any lower-level manager,
native-source, or audit-adapter change, a dependency, durable persistence, or a
user-visible/manual gate.

## Acceptance criteria

- [x] Verified 4U is merged before the `04v` gate begins.
- [ ] The exact `04v` gate begins before source edits.
- [ ] Only the exact two source/test files change outside declared closeout
      documentation.
- [ ] The turn owns one private typed in-memory audit adapter.
- [ ] Native and run-termination success paths return only one closed audited
      resolution value.
- [ ] The exact manager-owned resolution is recorded without reconstruction.
- [ ] The receipt remains sequence-only, volatile, and non-authorizing.
- [ ] Audit failure returns no successful resolution and leaves no stale pending
      turn ownership.
- [ ] Existing manager expiry, replay, identity, and late-outcome behavior is
      unchanged.
- [ ] All focused and complete automated checks pass.
- [ ] The complete diff has no scope expansion, secret, generated output, or
      blocking code/security finding.
- [ ] Project-memory documents and the review report match actual evidence.
- [ ] The final `04v` completion marker is complete and valid.

## Actual results

Planning only. The merged 4U prerequisite is satisfied and this plan is
reconciled against commit `61525bf`. Increment 4V remains blocked on explicit
queue selection and separate project-owner approval; it has no gate or source
evidence.

## Documentation updates

- [x] Proposed follow-on scope recorded during post-4T planning.
- [x] Reconcile against verified merged 4U before approval.
- [ ] `DECISIONS.md` updated during closeout if implementation confirms the
      planned durable boundary.
- [ ] Post-increment review report created during closeout.
- [ ] No troubleshooting update is planned unless a real issue is diagnosed.
