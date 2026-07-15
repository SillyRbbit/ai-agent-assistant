# Execution plan - Increment 4H typed approval-audit adapter

Status: **Complete**

Owner: Project maintainer

Last updated: 2026-07-15

## Execution status

The verified Increment 4E approval manager produces an exact terminal
`ApprovalResolution`, but no typed audit path consumes that result. The existing
generic audit scaffold accepts caller-authored event, summary, and details strings,
uses token-pattern redaction, is unbounded, and has no production caller. It is not
an adequate approval-evidence boundary.

The project owner approved this exact runtime/test and closeout scope. The adapter,
focused tests, complete repository verification, dependency audit, code/security
review, documentation synchronization, and mandatory post-increment gate are
complete without scope expansion.

## Goal and outcome

Add one bounded in-memory adapter that records a closed, redacted projection of an
exact terminal `ApprovalResolution`.

The outcome is Rust-only evidence that the adapter can preserve exact
approval/run/gateway-request/call correlation, locally derived policy facts,
terminal disposition, and closed native-interaction evidence without copying the
task title or accepting arbitrary audit strings.

The adapter remains a deterministic scaffold. It is not durable audit storage, is
not wired into the shipping application, and grants no dispatch or execution
authority.

## Existing behavior and constraints

- `ApprovalResolution` owns the exact `PolicyDecision` and exposes read-only
  accessors for approval identity, normalized call identity, local tool metadata,
  policy facts, disposition, and optional interaction evidence.
- `ApprovalResolution::preview()` can expose the validated task title. The audit
  adapter must never call it.
- Source-backed terminal resolutions carry only closed
  `ApprovalInteractionEvidence`; run termination and expiry carry no interaction
  evidence.
- `Approved` remains non-authorizing. It does not prove current run liveness,
  device-owner identity, successful authentication, audit durability, dispatch
  eligibility, or execution completion.
- The generic `AuditEventInput` path accepts arbitrary strings and remains a
  disconnected mock scaffold. Increment 4H must not route approval evidence
  through it or claim that it is production-ready.
- No authoritative runtime coordinator, durable audit repository, executor, or
  production caller exists.

## Closed trust path

```text
untrusted normalized gateway call
  -> exact local schema validation
  -> deterministic policy retaining exact input
  -> exact approval manager subject
  -> Rust-owned native interaction source or manager-owned terminal condition
  -> terminal non-authorizing ApprovalResolution
  -> typed redacted in-memory approval-audit adapter
  -> non-authorizing audit receipt
```

The adapter borrows the resolution. It does not consume, clone, serialize, mutate,
or return it, and it creates no wrapper that an executor could accept.

## Typed record contract

Create a dedicated `audit::approval` module containing private-field,
non-serializable types for:

- a bounded in-memory approval-audit adapter;
- a monotonically increasing audit sequence;
- a closed approval-resolution record;
- a non-authorizing receipt containing only the assigned sequence; and
- closed typed errors.

The record contains exactly:

```text
audit sequence
approval ID
opaque run ID
opaque gateway-request ID
opaque function-call ID
local tool name
local tool-contract version
local risk class
local required permission
closed policy outcome
closed policy reason
closed terminal disposition and cancellation reason, when present
interaction source, when present
recognized native button, when present
authentication evidence, when present
closed source-failure code, when present
```

The adapter must not call `ApprovalResolution::preview()` or retain the task title,
raw arguments, prompt text, model output, native-dialog message, OS/dependency
errors, raw provider/gateway errors, credentials, arbitrary summaries, or arbitrary
details.

Record and receipt constructors remain private to the module. Records do not
derive `Clone`, `Serialize`, or `Deserialize`. Their custom `Debug` output redacts
all run/request/call identity and contains no content field. Public read-only
accessors may expose the already bounded opaque IDs and closed enum facts for
focused tests and a future reviewed repository adapter.

The record does not assign a user actor. `MacOsNativeDialog` and a recognized
button remain source evidence only, and `NotEvaluated` remains an explicit absence
of device-owner-authentication evidence.

## Resolution validation matrix

Before assigning a sequence or mutating storage, the adapter revalidates the
current closed approval subject:

```text
tool: create_local_task
tool contract: 1
risk: ReversibleLocalAction
permission: None
policy outcome: RequireApproval
policy reason: ReversibleRequiresApproval
```

It then accepts only these evidence combinations:

| Disposition                   | Required interaction evidence                                             |
| ----------------------------- | ------------------------------------------------------------------------- |
| `Approved`                    | macOS native source, Approve button, `NotEvaluated`, no source failure    |
| `Rejected`                    | macOS native source, Reject button, `NotEvaluated`, no source failure     |
| `Cancelled(EditRequested)`    | macOS native source, Edit button, `NotEvaluated`, no source failure       |
| `Cancelled(NativeNoDecision)` | macOS native source, no button, `NotEvaluated`, no source failure         |
| `Cancelled(SourceFailed)`     | macOS native source, no button, `NotEvaluated`, one closed source failure |
| `Cancelled(RunTerminated)`    | no interaction evidence                                                   |
| `Expired`                     | no interaction evidence                                                   |

Any unsupported subject facts or inconsistent evidence fail before insertion. The
error contains only a closed variant and, where relevant, a fixed capacity limit;
it contains no identity or content.

## Bounds, duplicate handling, and failure semantics

Freeze these limits for the in-memory adapter:

```text
records per adapter instance: 1,024
records per exact approval/run/request/call subject: 1
eviction: none
persistence writes: 0
IPC events: 0
execution attempts: 0
```

The adapter rejects a duplicate exact approval/run/request/call key. Capacity and
sequence checks occur before mutation, and a failed insertion does not consume a
sequence number or alter existing records. Sequence arithmetic uses checked
addition.

The duplicate guarantee is process-local to one adapter instance. It is not a
durable idempotency claim and does not replace the approval manager's one-time
subject lifecycle. A future authoritative coordinator and durable repository must
define transaction, restart, and write-failure behavior separately.

A successful receipt proves only that this in-memory adapter accepted and retained
one closed record. It has no conversion to a policy input, approval resolution,
run-state proof, dispatch capability, executor capability, or tool result. Future
orchestration must treat audit failure as fail-closed before execution, but no such
orchestration is added here.

## Exact runtime and test files

Create:

```text
src-tauri/src/audit/approval.rs
src-tauri/tests/approval_audit_binding.rs
```

Change:

```text
src-tauri/src/audit/mod.rs
src-tauri/src/approvals/decision_source.rs
```

The `decision_source.rs` change is test-only coverage that sends real sealed native
source outcomes through the adapter. It must not change production source mapping,
dialog behavior, visibility, or manager authority.

Implementation closeout may change only:

```text
AGENTS.md
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
docs/increments/04h-typed-approval-audit-adapter.md
docs/plans/04h-typed-approval-audit-adapter.md
docs/reviews/2026-07-14-04h-post-increment-review.md
```

No other runtime, test, documentation, manifest, lockfile, Tauri, capability, CSP,
frontend, storage, gateway, provider, policy, approval-manager, native-dialog,
executor, packaging, or permission file may change. Stop and request approval
before expanding either list.

## Implementation steps

1. Add the closed approval-audit record, receipt, errors, fixed limits, and bounded
   in-memory adapter in the new module.
2. Project only the approved resolution accessors. Do not call `preview()` and do
   not accept a caller-created string or record.
3. Validate exact subject metadata and the complete disposition/evidence matrix
   before sequence assignment or storage mutation.
4. Enforce one exact record per approval/run/request/call key, 1,024 records per
   adapter, no eviction, checked sequence arithmetic, and redacted debug/errors.
5. Export only the dedicated module from `audit::mod`; leave the generic audit
   scaffold unchanged and disconnected.
6. Add portable public-boundary integration coverage plus macOS source-backed
   test coverage without opening a native dialog.
7. Run focused and complete verification, review the complete diff against
   `CODE_REVIEW.md` and `SECURITY.md`, synchronize closeout documentation, and run
   the mandatory post-increment gate.

## Test plan

Focused tests must prove:

- the public gateway -> schema -> policy -> approval -> run-termination -> typed
  audit path preserves exact opaque identity and local metadata;
- every accepted terminal disposition maps to the exact closed record facts;
- Approve, Reject, Edit, native no-decision, and each source-failure result are
  exercised through the existing sealed native-source test path;
- run termination and expiry require no interaction evidence;
- missing, extra, or contradictory interaction evidence fails before mutation;
- the adapter never calls or stores the preview title, and title content is absent
  from records, errors, and debug output;
- duplicate subjects, capacity exhaustion, and sequence exhaustion fail without
  eviction or partial mutation;
- sequence assignment is deterministic and contiguous for successful records;
- records and receipts expose no serialization, executor, dispatch, policy, or
  approval conversion; and
- existing generic audit and approval tests retain their behavior.

Tests may use crate-private constructors only inside the new module to exercise
impossible-state rejection. They must not widen production visibility or add a
test feature, public raw constructor, synthetic source-outcome constructor, or
native dialog invocation.

## Verification commands

Focused during implementation:

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked audit::approval::
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked approvals::decision_source::
cargo test --manifest-path src-tauri/Cargo.toml --test approval_audit_binding --locked
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
```

Complete gate:

```bash
npm run verify
npm audit --audit-level=low
git diff --check
python3 .codex/hooks/post_increment_gate.py status
```

Then run `$post-increment-gate`, create and finalize the exact 4H report, and
confirm status is complete and valid. No native manual interaction gate is
required because production dialog code and shipping application wiring do not
change. Manual verification is limited to confirming that no user-visible behavior
or permission prompt was introduced if a native launch is otherwise required by
the consolidated gate.

## Risks and mitigations

- **Personal-content leakage:** the adapter never calls `preview()` and has no
  title, argument, prompt, result, or arbitrary details field.
- **User-identity overclaim:** records retain `NotEvaluated` and do not assign a
  user actor or device-owner-authenticated status.
- **Audit-as-authority confusion:** records and receipts have no conversion to
  dispatch or execution and remain explicitly non-authorizing.
- **Fabricated evidence:** callers cannot construct a record; the adapter derives
  it from one manager-produced resolution and revalidates the closed matrix.
- **Duplicate or unbounded storage:** one exact subject is accepted once, capacity
  is fixed, arithmetic is checked, and no record is evicted.
- **Generic-string bypass:** the existing generic logger remains disconnected and
  is not used by the adapter. Replacing the entire audit architecture is deferred.
- **Durability confusion:** type and documentation names include `InMemory`; the
  increment makes no persistence, restart, transaction, or durable-completion
  claim.
- **Native regression:** production decision-source code must remain byte-for-byte
  unchanged except for test-module imports and assertions.

## Explicit non-goals

- Shipping application, Tauri command/event, WebView, or runtime-coordinator
  wiring.
- SQLite, migrations, files, durable audit storage, restart recovery, retention,
  deletion, export, or Activity-history UI.
- Executor, dispatch, local-task creation, provider continuation, or tool-result
  handling.
- Run-liveness checks, absolute run deadlines, permission grants, device-owner
  authentication, LocalAuthentication, or actor identity.
- Gateway transport, deployment, credentials, access tokens, live OpenAI traffic,
  or provider retention work.
- Redesign or removal of the existing generic audit scaffold.
- Generic audit events for run, proposal, policy, execution, cancellation, or final
  outcome beyond the terminal approval-resolution record.
- New dependencies, manifests, lockfiles, Tauri capabilities, CSP, packaging,
  entitlements, or operating-system permissions.

## Rollback

Delete the two new files, remove the `audit::approval` module export, and remove the
test-only decision-source assertions. Restore closeout documentation to make the
4H plan approval-blocked again. Do not alter the verified gateway, schema, policy,
approval manager, native dialog, generic audit scaffold, frontend, storage, or
workflow gate.

## Acceptance criteria

- The adapter accepts only one exact terminal `ApprovalResolution` reference and
  caller code cannot create or submit an arbitrary approval-audit record.
- Exact opaque identity, local tool/version/risk/permission, policy facts,
  disposition, and closed source evidence are retained.
- The task title and all other raw content are structurally absent.
- Every valid disposition/evidence combination is accepted and every inconsistent
  combination fails before mutation.
- One adapter retains at most 1,024 exact, non-evicted, non-duplicate records with
  checked deterministic sequence assignment.
- Debug and errors reveal no title, run/request/call value, raw error, or personal
  content.
- A receipt remains non-authorizing and no executor, dispatch, persistence, IPC,
  provider, or shipping-app path is added.
- The exact four-file runtime/test scope and declared closeout scope are preserved.
- Focused tests, full repository verification, dependency audit, complete diff,
  code review, security review, documentation synchronization, and the mandatory
  post-increment gate pass.
- D-029 is accepted and the increment is marked complete only after project-owner
  approval and all implementation gates pass.

## Planning baseline evidence

Passed on clean `codex/phase4-increment-4h` at `74692c1`:

```text
npm run typecheck
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked audit::
  4 passed
cargo test --manifest-path src-tauri/Cargo.toml --test approval_binding --locked
  2 passed
```

Planning changed documentation only. No runtime behavior, dependency, lockfile,
Tauri, frontend, storage, provider, gateway, approval authority, audit persistence,
dispatch, execution, capability, CSP, packaging, or permission path changed.

## Completion result

Implementation creates only `src-tauri/src/audit/approval.rs` and
`src-tauri/tests/approval_audit_binding.rs`, exports the new module, and adds
test-only assertions to the existing native decision-source module. Production
native-source behavior is unchanged.

The adapter uses closed, private-field, non-serializable records and a closed tool
variant; revalidates exact local tool, policy, disposition, and interaction facts;
never calls the title-bearing preview accessor; rejects unsupported, missing,
extra, contradictory, duplicate, capacity, and sequence-overflow inputs before
mutation; and returns only a sequence receipt with no authority conversion.

Focused checks pass with six adapter tests, eleven native-source tests, and one new
public-boundary integration test. `npm run verify` passes with 15 hook tests, 124
frontend tests, 99 Rust library tests, 11 Rust integration tests, lint, typecheck,
production frontend builds, and the Tauri release no-bundle build. The
network-enabled npm audit retry reports zero vulnerabilities. Complete scope,
secret, generated-output, diff, architecture, code-health, and security reviews
have no blocking finding.

No manual interaction gate applies because the implementation is transport-free
and production native-dialog and shipping application behavior are unchanged.
D-029 records the durable typed-adapter and non-authorizing in-memory boundary.
The post-increment report result is `PASS WITH ADVISORIES`; its advisory is that no
later increment is Ready and that this adapter is intentionally non-durable. The
completion marker is complete and valid.
