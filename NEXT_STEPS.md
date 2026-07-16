# Next steps

Last updated: 2026-07-15

This file is the ordered implementation queue. Work only on the first item marked **Ready**. A verification-pending increment must close before later feature work begins.

## Completed increments

- Increment 2A — core interfaces and deterministic mocks: **Verified complete**.
- Increment 2B-0 — SQLite storage decision: **Complete**.
- Increment 2B-1 — SQLite dependency and migration skeleton: **Verified complete**.
- Increment 2B-1A — Rust 1.90 compatibility repair: **Verified complete**.
- Increment 2C — storage startup integration: **Verified complete**.
- Increment 2D — macOS menu-bar and window lifecycle: **Verified complete**.
- Increment 2E — React application shell: **Verified complete**.
- Increment 2F — mocked assistant interaction shell: **Verified complete**.
- Increment 2G — integration hardening: **Verified complete**.
- Increment 3A — in-memory conversation sessions: **Verified complete**.
- Increment 3B — mock context provenance: **Verified complete**.
- Increment 3C — simulated tool result: **Verified complete**.
- Phase 3 completion gap analysis: **Complete**.
- Increment 3D — bounded mock-loop completion: **Verified complete**.
- Phase 4 gateway and Responses security-boundary planning: **Complete**.
- Increment 4A — deterministic gateway protocol contract: **Verified complete**.
- Increment 4B — exact local tool-schema validation: **Verified complete**.
- Increment 4C — trusted policy-input binding: **Verified complete**.
- Increment 4D — exact approval binding: **Verified complete**.
- Increment 4E — trusted approval-decision source: **Verified complete**.
- Increment 4F - Cortexa product display rename: **Verified complete by project-owner direction**.
- Repository Workflow Increment 4G - automated post-increment gate: **Verified complete**.
- Increment 4H - typed approval-audit adapter: **Verified complete**.
- Increment 4I - remove generic audit scaffold: **Verified complete**.
- Repository Workflow Increment 4J - deletion-stable post-increment fingerprint: **Verified complete**.
- Increment 4K - remove legacy provider scaffold: **Verified complete**.
- Increment 4L - remove legacy memory scaffold: **Verified complete**.
- Increment 4M - remove legacy platform scaffold: **Verified complete**.
- Increment 4N - bounded initial gateway request: **Verified complete**.
- Increment 4O - bound initial gateway turn: **Verified complete**.
- Increment 4P - schema-bound initial gateway events: **Verified complete**.
- Increment 4Q - terminally release initial function call: **Verified complete**.

## Queue status

Repository Workflow Increment 4G is **Verified complete**. It adds only the repository-local post-increment skill, deterministic Stop-hook validator, focused tests, report assets, and workflow documentation required to replace D-027's one-time exception. Its consolidated result is `PASS WITH ADVISORIES`; the advisory is the documented project-hook trust/bypass boundary.

Increment 4H typed approval-audit adapter is **Verified complete**. It adds one bounded transport-free in-memory Rust adapter that derives a closed redacted record from an exact terminal `ApprovalResolution`, revalidates the complete disposition/evidence matrix, and returns only a non-authorizing sequence receipt. It adds no durable persistence, runtime coordinator, dispatch, execution, IPC, UI, or live networking. Its consolidated result is `PASS WITH ADVISORIES`; the advisory is the explicit non-durable boundary and the absence of an approved next increment.

Repository Workflow Increment 4J is **Verified complete, published, and merged**. It corrects only the deterministic completion fingerprint for reviewed tracked deletions and adds positive and negative regression coverage. Its publication prerequisite for 4I reconstruction is satisfied.

Increment 4I remove generic audit scaffold is **Verified complete after reconstruction**. It deletes only the unused `audit::logger` and `audit::types` modules and removes their exports, preserving the typed `audit::approval` adapter unchanged. The corrected 4J gate remains intact, the stale-symbol scan has no matches, focused and complete checks pass, and no replacement abstraction, persistence, coordinator, dispatch, executor, IPC, UI, networking, dependency, migration, or permission was added.

Increment 4K remove legacy provider scaffold is **Verified complete**. It deletes
only `agent::provider` and `agent::types` and removes their two exports. The
verified normalized gateway protocol and exact function-call validator remain
unchanged; focused and complete checks pass, and no replacement provider,
transport, networking, credential, coordinator, dispatch, executor, persistence,
IPC, UI, dependency, capability, or permission was added.

Increment 4K is published and merged at `5415444`; its completion marker remains
valid for that committed content.

Increment 4L remove legacy memory scaffold is **Verified complete, published,
and merged at `ecd49be`**. It deletes only the three legacy memory files and their
single crate-root export. The verified storage module is unchanged; focused and
complete checks pass, and no replacement memory, persistence, migration,
encryption, Keychain, context selection, IPC, UI, dependency, capability, or
permission was added. Its consolidated result is `PASS WITH ADVISORIES`; the
advisory is the theoretical unsupported external consumer of the removed public
module.

Increment 4M remove legacy platform scaffold is **Verified complete, published,
and merged at `1f03d1e`**. It deletes only the three legacy platform files
and their single crate-root export. Typed app-info and the fixed Permission Center
remain unchanged; focused and complete checks pass, and no replacement adapter,
OS query, native framework, permission request, Keychain, LocalAuthentication,
IPC, UI, dependency, Tauri capability, entitlement, or permission was added. Its
consolidated result is `PASS WITH ADVISORIES`; advisories are the intentionally
deferred future platform design and theoretical unsupported external consumer of
the removed public module.

Increment 4N bounded initial gateway request is **Verified complete, published,
and merged at `d7c4b69`**. It adds one non-cloneable transport-free request
value with closed private serialization, fixed initial-turn and tool-set fields,
all existing conservative limits, shared opaque-ID validation, redacted debug and
errors, and final 64 KiB enforcement after escaping. Six focused tests, one
public-boundary integration test, Clippy, complete repository verification, npm
audit, exact-scope review, code review, security review, documentation sync, and
the mandatory gate pass. No transport, credential, provider, continuation,
runtime, IPC, persistence, policy, approval, audit, dispatch, execution,
dependency, capability, or permission path was added.

Increment 4O bound initial gateway turn is **Verified complete, published, and
merged at `87be00e`**. It adds one non-cloneable transport-free turn that owns
the closed request bytes and response validator, derives both correlation IDs and
the exact two local tool names/version from trusted construction, and exposes only
borrowed request bytes, status, frame acceptance, and local cancellation. Raw
initial-request construction is private; the lower-level validator remains public
for protocol fixtures. Six preserved request tests, six public contract tests,
Clippy, complete repository verification, npm audit, code/security review,
documentation sync, and the mandatory gate pass. No transport, credential,
continuation, coordinator, IPC, persistence, dispatch, execution, dependency,
capability, or permission path was added.

Increment 4P schema-bound initial gateway events is **Verified complete,
published, and merged at `8c1a2e0`**. `InitialGatewayTurn` owns one private exact
registry built from the same fixed schema catalog as response validation and
returns only closed `InitialGatewayEvent` values. Function events contain a
`SchemaValidatedFunctionCall`; local schema rejection returns a typed redacted
error, reports `Failed`, rejects late frames, and makes cancellation a no-op.
Six request, 18 protocol, six function-validation, nine tool, eight public
contract, and two policy-binding tests pass with Clippy, complete repository
verification, npm audit, exact-scope review, code/security review, documentation
sync, and the mandatory gate. No transport, credential, continuation, policy,
approval, audit persistence, runtime, IPC, execution, dependency, capability, or
permission path was added.

Increment 4Q terminally release initial function call is **Verified complete,
published, and merged at `8598612`**. `InitialGatewayTurn` privately retains a
schema-validated call after its non-terminal function frame returns `None`.
Accepted terminal completion releases the exact call once; gateway failure and
successful local cancellation discard it; transactional protocol errors retain
it for the correct terminal frame. Six request, 18 protocol, six
function-validation, nine tool, nine public contract, and two policy-binding
tests pass with Clippy, complete repository verification, npm audit, exact-scope
review, code/security review, documentation sync, and the mandatory gate. No
transport, credential, continuation, policy, approval, audit persistence,
runtime, IPC, execution, dependency, capability, or permission path was added.

## Ready

No later implementation increment is Ready.

Increment 4R bind terminal initial function call to policy is **Verified
complete, published, and merged at `5e58edb`**. Accepted terminal completion
consumes the exact pending schema-valid call through the fixed deterministic
engine and returns one retained `PolicyDecision`; failure and cancellation still
discard the pending call without evaluation. Both exact tool outcomes, retained
typed facts, redaction, lower-level contracts, Clippy, complete repository
verification, npm audit, scope review, documentation sync, and the mandatory gate
pass. No approval, audit, transport, runtime, dispatch, execution, dependency,
capability, or permission path was added.

### Increment 4S - bind terminal initial approval presentation

**Status:** Verified complete, published, and merged at `6d0bed4`.

The bound initial turn now consumes terminal `RequireApproval` through its
private fixed approval manager and returns one owned non-authorizing
`ApprovalPresentation`. `Allow` and `Deny` remain closed non-authorizing policy
events.

**Exact source/test scope:**

```text
src-tauri/src/agent/gateway_request.rs
src-tauri/tests/gateway_request_contract.rs
```

Accepted terminal completion creates one manager-owned approval request and
issues one exact presentation for `create_local_task@1`. The information-only
tool remains a non-authorizing `PolicyEvaluated` event. No trusted interaction,
approval disposition, audit, run-liveness, dispatch, or execution authority was
added.

No approval-manager, native-source, audit, policy-rule, transport, credential,
runtime, Tauri, frontend, SQLite, dependency, capability, entitlement, or
permission work is included.

Focused and complete verification, dependency audit, exact-scope, code,
security, documentation, and mandatory gate reviews pass. D-040 records the
durable boundary. No manual check is required.

### Increment 4T - bind terminal initial approval resolution

**Status:** Verified complete with uncommitted changes.

The bound turn now returns one sealed trusted approval source outcome to the
exact private manager that issued its presentation and exposes only the exact
non-authorizing terminal resolution.

**Exact source/test scope:**

```text
src-tauri/src/agent/gateway_request.rs
src-tauri/src/approvals/decision_source.rs
```

On macOS, the turn consumes one existing sealed
`TrustedApprovalSourceOutcome`, delegates directly to its private manager, and
returns the existing exact `ApprovalResolution` or typed approval error. The
native source's production behavior remains unchanged; only its existing
synthetic result mapper gains crate-wide test visibility under `cfg(test)` so
focused unit tests do not open a dialog.

No native invocation, run cancellation, proactive expiry, audit, persistence,
active-run validation, dispatch, execution, transport, credential, Tauri,
frontend, SQLite, dependency, capability, entitlement, or permission work is
included.

Eight request, 17 approval, nine public gateway-request, two approval-binding,
and one approval-audit test pass. Strict Clippy, complete `npm run verify`, npm
audit, exact-scope, code, security, documentation, and mandatory gate reviews
pass with no manual gate. D-041 records same-manager ownership and the
non-authorizing resolution boundary.

Exact next task: wait for explicit project-owner direction to commit, push, and
merge Increment 4T. Do not start later planning or implementation automatically.
