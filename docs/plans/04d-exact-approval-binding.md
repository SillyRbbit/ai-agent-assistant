# Execution plan - Increment 4D exact approval binding

Last updated: 2026-07-14

Status: **Complete**

## Gap analysis

Increments 4A-4C establish a closed path from one normalized gateway function call through exact local schema validation and conservative policy evaluation. `PolicyDecision` owns the exact typed call that was evaluated and `create_local_task@1` produces `RequireApproval`. The value deliberately has no approval, audit, dispatch, or execution conversion.

The existing approval scaffold cannot safely consume that result:

- `GatewayStreamValidator` verifies the expected run and gateway-request IDs on every frame, but `UntrustedFunctionCall` retains only call ID, name, version, and raw arguments. Run and request identity are lost before schema validation.
- `UntrustedFunctionCall` and `ValidatedGatewayEvent` derive `Clone`, and the raw argument string appears in derived debug output.
- `ApprovalRequestInput::new` accepts independent arbitrary tool-name, action-hash, and preview strings. None must match a validated call or policy result.
- `ApprovalRequest` clones those detached strings and exposes public fields. `action_hash` is caller content, not a canonical or cryptographic binding.
- `ApprovalManager::create_request` accepts the detached input, while `decide` accepts only a numeric ID and has no expiry, cancellation, run/call replay prevention, or exact subject retention after decision.
- The manager permits multiple pending records even though the verified product permits one active run and the Phase 4 protocol permits one function call per run.
- `AuditEventInput` accepts arbitrary summary and details strings. Its token redactor cannot safely represent exact arguments, approval evidence, or structured identity.
- Repository search found no production approval, policy-to-approval, approval-to-audit, approval-to-executor, or Tauri caller. Only approval unit tests construct the generic input.

| Boundary                | Verified state                                          | Remaining gap                                                     |
| ----------------------- | ------------------------------------------------------- | ----------------------------------------------------------------- |
| Gateway stream          | Run/request IDs checked against trusted expected values | IDs are dropped from the completed call                           |
| Local schema and policy | Exact typed action retained in non-authorizing decision | Decision has no run/request identity and no approval transition   |
| Approval input          | Detached caller strings                                 | No exact action, policy, preview, expiry, or replay binding       |
| Approval state          | In-memory pending/approved/rejected records             | Clonable, non-expiring, no cancellation, no terminal subject seal |
| Audit                   | Generic redacting scaffold                              | No closed approval event contract or safe content policy          |

## Goal and outcome

Build one transport-free approval boundary that consumes exactly one eligible policy decision and retains its verified run/request/call identity and typed arguments. The manager derives the preview from the same owned subject and consumes approval, rejection, cancellation, or expiry once. No caller may resupply the tool, arguments, classification, preview, hash, policy result, creation time, or deadline.

The result is deterministic locally bound approval state only. Because this increment defines no trusted user-interaction source, even an `Approved` disposition does not yet prove a user gesture or local authentication. It is not serializable, not exposed through Tauri, not accepted by an executor, and not sufficient to dispatch or execute a tool.

## Exact identity propagation

`GatewayStreamValidator` already owns locally expected run and gateway-request IDs and rejects envelope mismatches. When it accepts `function_call_completed`, copy those already-validated opaque IDs into `UntrustedFunctionCall`. Do not trust the wire values separately after the equality check.

`validate_function_call` consumes the normalized call and carries these fields into `SchemaValidatedFunctionCall`:

```text
run ID
gateway-request ID
function-call ID
local tool name
local tool-contract version
closed typed arguments
locally derived risk class
locally derived required permission
```

The existing `PolicyInput` and `PolicyDecision` then retain the entire value unchanged. Add read-only run and gateway-request accessors; add no public constructor for any identity.

While changing the content-bearing gateway type, remove `Clone` from `UntrustedFunctionCall` and from `ValidatedGatewayEvent`. Replace their derived debug output with custom implementations that redact both function arguments and output-text deltas while retaining only closed variants and opaque identity metadata. This keeps the existing ownership-consuming path meaningful and prevents accidental model-output or argument duplication and logging. Closed gateway failures may retain their existing copy/clone behavior because they contain no raw content.

## Canonical approval subject

The canonical subject is the exact owned `PolicyDecision`, not JSON, display text, or independently reconstructed fields. Approval creation consumes the decision and checks that its derived outcome is exactly `PolicyOutcome::RequireApproval`.

The subject includes, by ownership:

```text
verified run/request/call identity
local tool identity and contract version
closed typed arguments
local risk and permission classification
closed policy reason and derived outcome
```

The manager may copy only opaque run/request/call IDs into an internal replay key. It must not clone task content, serialize the decision, or expose a consuming accessor that can recover the decision for a second request.

An `Allow` or `Deny` decision is rejected and consumed with a closed `IneligiblePolicyOutcome` error. Approval cannot upgrade a denied call or turn a non-authorizing allow result into an execution capability.

## Canonical typed preview

Increment 4D supports only `create_local_task@1`, the only registered schema that can produce `RequireApproval`. The preview is a borrowed projection of the retained `ValidatedToolArguments::CreateLocalTask` value. It is not independently stored.

Use one closed preview variant with these facts:

| Product field        | Exact Increment 4D representation                           |
| -------------------- | ----------------------------------------------------------- |
| What will happen     | Closed `CreateLocalTask` variant                            |
| Target               | Closed local task-list target                               |
| Exact affected data  | Borrowed already-validated task title                       |
| Date and time        | Closed `NotScheduled` value                                 |
| Recipients           | Closed `None` value                                         |
| Reversibility        | Closed `Reversible` value                                   |
| Required permissions | Retained locally derived `PermissionKind::None`             |
| Main risk            | Retained `RiskClass::ReversibleLocalAction` plus fixed kind |

The preview and request view do not derive `Clone`, serialization, or content-bearing `Debug`. They may expose the exact title only through a borrowed typed accessor for a later trusted presentation adapter. A sentinel title must not appear in debug output or errors.

Any future approval-required schema must add and test its own closed preview variant. A policy decision with no exact preview mapping fails with `UnsupportedApprovalSubject`; generic fallback text is prohibited.

## Digest decision

Remove `action_hash` and add no replacement digest in Increment 4D.

The approval manager owns the exact policy decision and maps its internally assigned approval ID directly to that value. Decision methods accept only the ID and a closed choice; they accept no tool name, arguments, preview, classification, policy outcome, or digest that could be substituted. A hash would therefore be redundant for this same-process boundary.

Adding a digest now would also require a stable canonical byte encoding and a direct cryptographic dependency. It could duplicate personal data handling, permit offline guessing of low-entropy titles, and be mistaken for authority if echoed by a WebView. The existing transitively locked hashing crates are not an approved direct dependency.

If persistence or a cross-process protocol later requires a digest, a separate decision must define a versioned, domain-separated encoding, exact algorithm/version, retention and disclosure rules, constant-time comparison requirements where relevant, mismatch behavior, migration, and rollback. A future digest remains correlation data and must never authorize an action.

## Approval lifecycle

Replace the detached record lifecycle with these rules:

1. `create_request` consumes one `PolicyDecision` and rejects any outcome other than `RequireApproval`.
2. It derives and validates the closed preview mapping before storing the decision.
3. It rejects a run/request/call subject already pending or previously consumed in the manager lifetime.
4. It rejects creation while another approval is pending. `MAX_PENDING_APPROVALS` is exactly `1`, matching the verified single-active-run and single-function-call product boundary.
5. It admits at most 1,024 distinct subjects during one manager lifetime, including the pending subject. `MAX_APPROVAL_SUBJECTS_PER_MANAGER` is exactly `1_024`; after that capacity is consumed, new requests fail closed rather than evicting replay tombstones.
6. It assigns the next internal `ApprovalId`; the ID field is private and read through an accessor. It is a correlation key, not a secret, authentication token, digest, or authority.
7. It records creation and deadline using a manager-owned monotonic clock. `APPROVAL_TTL` is exactly 120 seconds from request creation.
8. A borrowed request view exposes identity, closed preview facts, policy reason, classification, and remaining lifetime from the stored subject. It owns no second copy.
9. `decide(id, Approve | Reject)` looks up the exact stored subject. It accepts no caller-supplied identity or content.
10. Deadline evaluation occurs before the requested choice. If the deadline has elapsed, the terminal disposition is `Expired`, never `Approved`.
11. `cancel(id)` consumes the same pending subject with terminal `Cancelled` disposition. It is the hook a future run-cancellation orchestrator must call.
12. `expire_due()` consumes elapsed pending state with terminal `Expired` disposition so a future orchestrator can route closed facts to a reviewed audit adapter.
13. Every terminal transition removes the pending subject, records its opaque identity as consumed, and returns one sealed `ApprovalResolution` owning the original `PolicyDecision`.
14. A second decision, cancellation, request recreation, or stale event for the same subject fails closed as already consumed or replayed.
15. There is no `Edit` decision. Future Edit behavior must reject or cancel this exact subject and produce a fresh gateway call, schema validation, policy decision, preview, and approval ID. Arguments are never mutated in place.

Use `std::time::Instant` through a private manager clock boundary. The production constructor uses the system monotonic clock; unit tests inject a private deterministic clock. No public API accepts `now`, creation time, expiry time, or TTL.

The 120-second approval TTL is a relative lifetime, not proof that the run remains active. Because this increment has no orchestrator, it does not know the run's absolute deadline. Future orchestration must call `cancel` when the owning run stops, is cancelled, or reaches its own deadline; no executor may rely on TTL alone as run-liveness evidence.

Retain `ApprovalManager` as the transport-free interface, but replace clone-returning generic methods with this semantic surface:

```text
create_request(PolicyDecision) -> ApprovalId
pending() -> borrowed ApprovalRequestView, if one unexpired request exists
decide(ApprovalId, ApprovalChoice) -> ApprovalResolution
cancel(ApprovalId) -> ApprovalResolution
expire_due() -> optional ApprovalResolution
```

Exact Rust lifetime and result syntax may follow existing style, but no method may return a cloned subject or accept caller content beyond the manager-generated ID and closed choice. `InMemoryApprovalManager::new` uses the private system clock; only module tests may inject the deterministic clock.

All one-time, capacity, and replay guarantees in Increment 4D are scoped to one `ApprovalManager` instance. No production instance is wired yet. Future orchestration must own exactly one authoritative manager for the application run lifecycle; constructing parallel managers for the same subject is not a supported authorization boundary.

## Resolution and authority boundary

`ApprovalResolution` contains a closed disposition:

```text
Approved
Rejected
Cancelled
Expired
```

It owns the exact policy decision so the action cannot be swapped between request and resolution. The type must not derive `Clone`, `Serialize`, `Deserialize`, or raw-value `Debug`. Read-only accessors may expose opaque identity and closed metadata; the task title remains available only through the same borrowed preview projection.

Do not add `into_policy_decision`, `into_validated_call`, `dispatch`, `execute`, or a generic capability conversion. No existing executor, tool implementation, audit logger, Tauri command, provider continuation, or storage repository may accept `ApprovalResolution`.

`ApprovalChoice` is a closed Rust enum and is not deserializable. This increment does not define how a user gesture becomes trusted approval evidence. A future native interaction plan must establish that boundary; a WebView message cannot be mapped directly into execution authority.

Accordingly, `Approved` means only that the transport-free manager processed the closed Rust choice while the exact subject was pending and unexpired. It does not attest who made that choice. No later executor may accept this type until a separately reviewed trusted interaction source and pre-execution revalidation boundary are added.

## Typed redacted errors

Replace empty-string validation errors with closed variants for:

- Ineligible policy outcome.
- Unsupported approval subject.
- Pending approval already exists.
- Duplicate or replayed subject identity.
- Approval not found.
- Approval already consumed.
- Approval subject capacity exhausted.
- Approval ID space exhausted.
- Deadline calculation overflow.

Errors may include an opaque numeric approval ID or closed outcome enum. They must not include run/request/call strings, tool names, titles, raw arguments, previews, hashes, policy explanations, or debug dumps of content-bearing values.

## Cancellation and audit boundary

The approval manager provides explicit `cancel` and `expire_due` terminal paths. Future run cancellation or deadline handling must consume the pending approval before any late choice can be accepted. This increment does not wire gateway cancellation, run state, absolute run deadlines, or timers to the manager.

Do not call or modify `AuditEventInput`, `AuditLogger`, or storage. `ApprovalResolution` preserves enough closed identity and disposition for a future typed audit adapter, but exact task content must not be copied into generic audit strings. The product brief's request, validated-argument, and affected-resource requirements must later be reconciled as structured references and deliberately redacted summaries under the stricter security rule against duplicating raw personal content.

The generic audit scaffold remains non-production-ready until a separate plan defines closed event variants, timestamp ownership, persistence, redaction, retention, and failure behavior.

## Trust and authorization path

After Increment 4D, the only approval path is:

```text
normalized gateway frame
  -> run/request-bound UntrustedFunctionCall
  -> exact local schema validation
  -> run/request/call-bound SchemaValidatedFunctionCall
  -> ownership-consuming PolicyInput
  -> input-retaining RequireApproval PolicyDecision
  -> ownership-consuming ApprovalManager
  -> borrowed typed preview of that exact subject
  -> one terminal ApprovalResolution
```

There is still no path from this resolution to audit, dispatch, execution, provider continuation, IPC, or UI. The model, gateway, WebView, and caller cannot supply or override identity, typed arguments, risk, permission, policy reason/outcome, preview, creation time, expiry, or resolution subject.

## Dependency decision

Add no dependency and do not change `Cargo.toml`, `Cargo.lock`, `package.json`, or `package-lock.json`.

Rust ownership, private fields, closed enums, `std::time::Instant`, `Option`, and existing collections are sufficient. Do not promote `sha2`, `uuid`, `time`, `chrono`, or another transitive dependency. A dependency decision is required if a later approved design genuinely needs secure randomness, stable timestamps, serialization, or hashing.

## Explicit non-goals

- Approval UI, native dialog, LocalAuthentication, Touch ID, device-password confirmation, or WebView decision handling.
- Tauri commands/events, IPC serialization, frontend types, mock approval changes, or user-visible behavior.
- Treating a WebView gesture, approval ID, preview, hash, or digest as trusted user evidence.
- Canonical JSON/bytes, cryptographic hashing, signatures, MACs, random bearer tokens, stable persistence formats, or migrations.
- Audit event creation, audit storage, approval persistence, timestamps for durable records, retention, or Activity UI.
- Dispatch eligibility, executor capabilities, tool execution, local task storage, tool results, or provider continuation.
- Permission-grant, resource-scope, explicit-intent, provenance, freshness, or local-authentication evidence.
- Enabling denied read-only, permission-bearing, personal-data, external/high-impact, or prohibited tool schemas.
- New tool schemas, changes to existing argument rules, risk rules, permission rules, or policy classification.
- Gateway transport, OpenAI calls, credentials, Keychain, identity, deployment, retries, model selection, or provider cancellation.
- Database, storage, capability, CSP, plugin, packaging, or operating-system permission changes.
- Multiple concurrent approvals, background approvals, approval renewal, editable pending arguments, or cross-process replay storage.
- Multiple authoritative approval-manager instances or persistence of approval state across process restart.
- Calling the generic audit scaffold or treating an approved resolution as execution authority.

## Exact implementation files

Create:

```text
src-tauri/tests/approval_binding.rs
```

Change:

```text
src-tauri/src/agent/gateway_protocol.rs
src-tauri/src/agent/function_call_validation.rs
src-tauri/src/approvals/types.rs
src-tauri/src/approvals/manager.rs
```

Do not change policy rules/types, tool schemas/registry, provider, audit, executor, storage, Tauri, frontend, manifests, or lockfiles.

Implementation closeout may update only:

```text
AGENTS.md
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
docs/increments/04d-exact-approval-binding.md
docs/plans/04d-exact-approval-binding.md
```

No file outside these implementation and closeout lists may change without stopping for project-owner approval.

## Implementation steps

1. Retain the already-verified run and gateway-request IDs on normalized function calls and carry them into the schema-validated call.
2. Remove content-bearing gateway call/event cloning and replace raw call debug output with an argument-redacted implementation.
3. Replace detached approval input/records with private ownership-retaining request, borrowed view/preview, closed choice/disposition, and sealed resolution types.
4. Implement the one-pending, 120-second monotonic lifecycle with ineligible-input rejection, replay prevention, exact-ID lookup, cancellation, expiry, and terminal one-time consumption.
5. Derive the `create_local_task@1` preview only from the retained typed arguments and local metadata; fail closed for every unsupported subject.
6. Replace approval unit tests with exact lifecycle, fake-clock, replay, limit, error-redaction, and preview-redaction coverage.
7. Add a public-boundary integration test that traverses gateway validation, local schema validation, policy, request creation, exact preview, and terminal resolution.
8. Confirm no caller can submit a tool name, argument, hash, preview, classification, policy result, time, deadline, or resolution subject at the approval boundary.
9. Confirm no audit, executor, IPC, persistence, network, credential, capability, CSP, packaging, or permission path was added.
10. Run focused checks, the complete repository gate, dependency audit, diff checks, code review, and security review.
11. Synchronize only the approved closeout documents and record D-024 only after implementation is approved and verified.

## Implementation evidence

- The project owner approved this exact plan and file list before runtime edits.
- Only the five approved runtime/test files and nine approved closeout documents changed.
- Validator-owned run and gateway-request IDs now remain attached to the consumed call through schema validation, policy, approval view, and terminal resolution.
- Content-bearing gateway events and approval values are non-cloneable and use redacted custom debug output.
- Approval creation consumes one exact `RequireApproval` decision and derives the `create_local_task@1` preview from its retained typed arguments and local metadata.
- The manager enforces one pending request, the 1,024-subject lifetime cap, relative 120-second monotonic expiry, explicit cancellation, terminal approve/reject/cancel/expiry, and non-evicting replay prevention.
- `action_hash` was removed; no canonical bytes, digest, dependency, manifest, or lockfile change was added.
- No audit, dispatch, executor, IPC, persistence, provider, network, credential, Tauri, frontend, capability, CSP, packaging, permission, or user-visible path was added.
- Focused checks passed with 18 gateway, six function-call validation, four policy, six approval, two policy-input integration, and two approval-binding integration tests.
- `npm run verify` passed with 124 frontend tests, 82 Rust library tests, ten Rust integration tests, TypeScript, production frontend builds, and the Tauri release no-bundle build.
- `npm audit --audit-level=low` reported zero vulnerabilities after a sandboxed DNS failure was retried with network access. rustfmt, Clippy with warnings denied, diff checks, code review, and security review passed.
- No native interaction gate was required because the changed modules remain transport-free and unreferenced by Tauri or the UI.
- D-024 records the verified boundary and the non-authorizing meaning of `Approved`.

## Risks and mitigations

- **Approval applies to the wrong run or request:** carry IDs only from the gateway validator's trusted expected values and retain them through policy and approval.
- **Preview/action substitution:** derive a borrowed preview from the exact stored policy decision; accept no caller preview or action fields.
- **Hash creates false authority:** remove `action_hash`, add no digest, and document the correlation-only role of approval IDs.
- **Stale approval is accepted:** use a manager-owned monotonic 120-second deadline, check it before choice, and provide cancellation/expiry consumption; document that future orchestration must also cancel on run termination or absolute deadline.
- **Decision is replayed:** consume the pending value once and retain an opaque run/request/call tombstone; cap the manager at 1,024 subjects and fail closed rather than evicting replay history.
- **Parallel manager instances each resolve the same subject:** scope the guarantee explicitly to one instance and require future orchestration to own one authoritative manager before any execution path exists.
- **Edited arguments reuse approval:** provide no Edit or mutation path; require a fresh call and approval identity.
- **Multiple pending requests confuse the user or caller:** enforce a hard one-pending limit matching the verified single-active-run boundary.
- **Personal content leaks:** do not clone or serialize content-bearing values; custom-debug redaction and sentinel tests cover gateway, request, preview, resolution, and errors.
- **Unsupported tool receives incomplete preview:** use one exact `create_local_task` variant and fail closed for any other approval-required subject.
- **Approved evidence becomes executable by implication:** add no consuming extractor or executor/dispatch conversion and leave every runtime integration disconnected.
- **WebView choice is treated as trusted:** add no IPC or deserialization and require a later reviewed native approval-source plan.
- **Approved state is mistaken for proof of user presence:** state explicitly that Increment 4D binds lifecycle and subject only; trusted interaction and local authentication remain absent.
- **Expiry loses future audit evidence:** return closed terminal resolutions from explicit decide/cancel/expire paths; defer the audit adapter rather than writing generic strings.
- **No digest prevents later persistence:** persistence is out of scope; a future versioned digest/encoding decision can be added without weakening the stronger current ownership boundary.

## Test plan

- Preserve all 17 gateway-protocol tests while asserting accepted calls retain exact run and gateway-request IDs and redact both raw arguments and output-text deltas from call/event debug output.
- Preserve all six function-call validation tests while asserting both registered schemas retain exact run/request/call identity.
- Preserve the four policy rule-table tests and two public policy-input binding tests.
- Replace generic approval tests with tests for one eligible request, exact borrowed preview, one-pending limit, 1,024-subject capacity, approve, reject, cancel, expiry, unknown ID, duplicate decision, duplicate subject, and ID exhaustion.
- Use a private deterministic clock to test the boundary immediately before, exactly at, and after the 120-second deadline without sleeping.
- Assert deadline evaluation wins over a simultaneous Approve choice.
- Through the current public policy path, assert an `Allow` decision cannot become an approval request; keep the manager's closed non-`RequireApproval` check exhaustive so future `Deny` inputs also fail.
- Unit-test the private preview mapper directly so `GetCurrentDatetime` has no fallback preview and returns the closed unsupported-subject error without creating a request.
- Assert the request view exposes the exact retained run/request/call identity, tool/version, policy reason, risk, permission, and task title without accepting replacement values.
- Assert same run/request/call identity cannot be recreated with equal or different arguments after approval, rejection, cancellation, or expiry.
- Assert an Edit-equivalent path has no mutation API and must use a fresh call identity.
- Assert request, preview, resolution, gateway event, errors, and manager debug output omit a sentinel task title and raw argument JSON.
- Through public APIs, parse a normalized `create_local_task` frame, validate it locally, evaluate policy, create approval, inspect the exact preview, approve it once, and confirm a second decision fails.
- Through public APIs, prove `get_current_datetime` cannot enter approval because its policy outcome is `Allow`.
- Preserve all existing frontend, Rust library, and Rust integration tests.

## Verification commands

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_protocol
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::function_call_validation
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked policy::
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked approvals::
cargo test --manifest-path src-tauri/Cargo.toml --test policy_input_binding --locked
cargo test --manifest-path src-tauri/Cargo.toml --test approval_binding --locked
npm run verify
npm audit --audit-level=low
git diff --check
git diff -- src-tauri/src/agent/gateway_protocol.rs src-tauri/src/agent/function_call_validation.rs src-tauri/src/approvals/types.rs src-tauri/src/approvals/manager.rs src-tauri/tests/approval_binding.rs
```

Review gates:

- Run `$code-review` against the complete diff.
- Run `$security-review` because this increment changes gateway identity retention and approval state.
- Confirm `ApprovalRequestInput`, caller-supplied tool names, `action_hash`, arbitrary preview strings, clonable requests, and raw gateway-call debug output are removed.
- Confirm approval creation accepts only an owned `PolicyDecision` and accepts no caller identity, arguments, classification, preview, time, deadline, or digest.
- Confirm only `RequireApproval` can enter and only `create_local_task@1` has a preview mapping.
- Confirm approve, reject, cancel, and expiry consume once; stale and duplicate subjects fail closed.
- Confirm no type provides a consuming path from approval resolution to policy input, validated call, dispatch, audit, or execution.
- Confirm audit, provider, policy rules, schemas, registry, executor, Tauri, frontend, manifests, and lockfiles are unchanged.
- Confirm no generated file, database, build output, credential, URL, capability, CSP, package, or permission change is present.
- No native interaction check is required because the planned modules remain transport-free and change no UI or target-platform behavior.

## Rollback

Remove `src-tauri/tests/approval_binding.rs`, restore the previous gateway/event and schema-validated call identity fields/derives, and restore the detached approval types and manager tests. Do not alter the verified protocol state machine, local schemas, policy rules, audit, provider, storage, Tauri, frontend, manifests, capabilities, CSP, or permissions during rollback.

## Acceptance criteria

- [x] Accepted function calls retain validator-owned run, gateway-request, and call identity through schema validation, policy, approval request, preview, and resolution.
- [x] Raw content-bearing gateway calls/events cannot be cloned, and custom debug output redacts both function arguments and output-text deltas.
- [x] Approval creation consumes only an exact `RequireApproval` policy decision.
- [x] Caller-supplied tool names, hashes, previews, arguments, classification, policy outcomes, creation times, and deadlines are removed from the approval boundary.
- [x] `create_local_task@1` has one closed borrowed preview containing exact affected data and fixed product facts; unsupported subjects fail closed.
- [x] No canonical bytes or digest are introduced, and `action_hash` is removed.
- [x] The manager enforces one pending request, a 1,024-subject lifetime cap, a fixed 120-second monotonic expiry, explicit cancellation, and one-time terminal consumption.
- [x] One-time, capacity, and replay guarantees are explicitly per manager instance; future orchestration is required to own one authoritative instance.
- [x] Approve, reject, cancel, expiry, unknown ID, duplicate decision, and replay behavior are typed and fail closed.
- [x] Edit cannot mutate or reuse a pending subject; a fresh validated call is required.
- [x] Approval request/view/resolution values are private, non-cloneable, non-serializable, and content-redacted in debug and errors.
- [x] An approved resolution exposes no dispatch, executor, audit, IPC, or policy-input conversion.
- [x] An approved disposition is documented and tested as locally bound state, not proof of a user gesture, user presence, or local authentication.
- [x] Audit, provider, policy rules, schemas, registry, executor, IPC, persistence, UI, capabilities, CSP, packaging, and permissions remain disconnected.
- [x] No dependency or lockfile change is introduced.
- [x] Only the exact approved files change.
- [x] Focused tests, `npm run verify`, dependency audit, diff checks, code review, and security review pass.
- [x] D-024 and closeout documentation record only actual approved and verified behavior.

## Approval gate

The project owner approved this exact goal, run/request/call propagation, non-cloneable redacted gateway values, canonical typed preview, no-digest decision, relative 120-second deadline, one-pending and 1,024-subject limits, run-cancellation boundary, terminal/replay semantics, exact files, no-dependency decision, tests, non-goals, verification gate, and rollback before runtime edits. The implementation stayed within the approved file list and boundary.
