# Execution plan - Increment 4C trusted policy-input binding

Last updated: 2026-07-14

Status: **Complete**

## Gap analysis

Increment 4B establishes a closed, ownership-consuming path from one normalized gateway function call to `SchemaValidatedFunctionCall`. That value binds a bounded call ID, exact local tool name and contract version, private typed arguments, and locally derived risk and permission metadata. It is deliberately not a policy decision or execution capability.

The remaining Rust proposal and policy foundation can bypass or weaken that boundary:

- `ToolCallProposal::new` accepts arbitrary tool names, raw argument JSON, risk, and permission values, and all fields are public.
- `AgentProviderResponse::ToolCalls` accepts those raw proposals. The deterministic mock provider never constructs this variant, and the verified gateway protocol uses a separate normalized event path.
- `ProposedAction::new` independently accepts a tool name, risk, permission, and `PolicyContext`. It does not contain the validated call ID, contract version, or typed arguments.
- `PolicyContext` contains caller-supplied `explicit_user_intent` and `permission_granted` booleans. A boolean cannot prove that intent, permission, resource scope, and freshness belong to this exact call.
- `PolicyEngine::evaluate` consumes `ProposedAction` but returns only an outcome and arbitrary reason string, discarding the exact action that was evaluated.
- `PolicyDecision` has public fields and constructors that can pair arbitrary reason text with an outcome.
- Approval inputs still accept caller-supplied tool names, action hashes, and previews. Audit inputs still accept generic strings. Neither has a production caller, and neither is safe to connect during this increment.

Repository search found no production caller of `ToolCallProposal::new`, `AgentProviderResponse::tool_calls`, `ProposedAction::new`, approval creation from a model call, policy-to-approval conversion, audit recording from policy, or any executor. This permits removal of the unsafe compatibility surfaces before orchestration exists.

| Boundary                      | Verified state                                    | Remaining gap                                                |
| ----------------------------- | ------------------------------------------------- | ------------------------------------------------------------ |
| Gateway and schema validation | Exact, bounded, typed, locally classified call    | No trusted policy-input transition                           |
| Legacy provider response      | Mock text plus unused arbitrary raw proposals     | Raw proposal path can bypass schema validation later         |
| Policy input                  | Caller-supplied name/risk/permission/context      | Does not own the exact validated call or bound evidence      |
| Policy decision               | Deterministic rules over caller-supplied metadata | Drops the evaluated call and uses arbitrary reason strings   |
| Approval and audit            | Detached deterministic scaffolds                  | Generic inputs are not bound to a validated call or decision |

## Goal and outcome

Remove the unused raw Rust proposal path and make an owned `SchemaValidatedFunctionCall` the only production source of policy action identity, typed arguments, tool contract version, risk, and required permission. Remove caller-supplied policy context instead of treating unbound booleans as trusted evidence. Deterministic evaluation returns a closed `PolicyDecision` that retains the exact input it evaluated and fails closed wherever permission, resource scope, or narrow explicit intent has not been structurally established.

The result is a canonical typed policy boundary only. It does not grant approval, issue an approval request, calculate an action hash, create an audit record, dispatch a tool, or authorize execution.

## Canonical typed action definition

For Increment 4C, canonical means one ownership-bound structured value, not a JSON string or digest. The exact action identity remains the fields already held together by `SchemaValidatedFunctionCall`:

```text
call ID
local tool name
local tool contract version
closed typed arguments
locally derived risk class
locally derived required permission
```

`PolicyInput` owns that entire value without copying or serializing its arguments. The canonical `create_local_task` title is the exact already-validated title; Increment 4C performs no additional trimming, normalization, or reconstruction.

This increment deliberately does not define canonical bytes, canonical JSON, a cryptographic digest, an action hash, a preview, trusted intent evidence, permission-grant evidence, resource-scope evidence, freshness, run binding, expiry, or one-time approval consumption. Those fields must be designed together with the trusted state that produces them so policy and approval cannot apply evidence from another call or resource.

## Conservative evidence boundary

Increment 4C removes `PolicyContext` and does not replace it. There is currently no trusted type that binds explicit user intent or an operating-system permission grant to the exact call ID, run, resource scope, and observation time. Naming a public boolean constructor `from_trusted_state` would not establish those properties.

Policy therefore uses only the locally derived metadata already owned by `SchemaValidatedFunctionCall`:

- `InformationOnly` with `PermissionKind::None` may produce `Allow` as non-authorizing policy data.
- Any non-`None` required permission produces `Deny` because no exact permission and scope evidence is available.
- `ReadOnlyDeviceAccess` produces `Deny` because existing permission and target scope have not been established.
- `ReversibleLocalAction` produces `RequireApproval` because narrow explicit intent has not been bound to the exact call.
- `PersonalDataModification` produces `RequireApproval`.
- `ExternalOrHighImpactAction` and `ProhibitedAutonomy` produce `Deny`.

This is intentionally stricter than the unused current boolean policy scaffold. A later approved increment may enable a narrower result only after defining typed call-bound evidence, provenance, scope, freshness, and mismatch behavior.

## Exact policy-input design

1. Remove `ToolCallProposal` from `tools/types.rs`.
2. Remove `AgentProviderResponse::ToolCalls` and `AgentProviderResponse::tool_calls`; retain the deterministic mock provider's text-only response contract unchanged.
3. Remove `PolicyContext` and replace `ProposedAction` with `PolicyInput` in `policy/types.rs`.
4. Construct `PolicyInput` only by consuming `SchemaValidatedFunctionCall`. There is no constructor that accepts a tool name, version, argument JSON, typed arguments, risk, permission, intent, permission state, or scope independently.
5. Keep `PolicyInput` fields private. Expose read-only access to the owned validated call for deterministic evaluation and tests.
6. Do not derive `Clone`, `Serialize`, or raw-value `Debug` for `PolicyInput` or `PolicyDecision`. Custom debug output may expose closed metadata but must redact typed argument content.
7. Change `PolicyEngine::evaluate` to consume `PolicyInput` and return `PolicyDecision` directly.
8. Remove `PolicyError::EmptyToolName` and `PolicyResult`. Empty or malformed tool identity is impossible after the required gateway and local-schema boundaries, and evaluation over a closed valid input is total.
9. Replace arbitrary reason strings with a closed `PolicyReason` enum. Each reason maps to exactly one `PolicyOutcome`, so outcome and reason cannot disagree.
10. Make `PolicyDecision` fields private and retain the exact `PolicyInput` it evaluated. Expose read-only outcome, reason, input, and validated-call accessors only.
11. Add no consuming conversion from `PolicyDecision` to approval, audit, dispatch, or execution. A policy `Allow` outcome remains data, not an execution capability.

Proposed closed reasons and outcomes:

| Policy reason                           | Outcome           | Meaning                                                                          |
| --------------------------------------- | ----------------- | -------------------------------------------------------------------------------- |
| `InformationOnly`                       | `Allow`           | Locally classified information-only call with no required permission             |
| `RequiredPermissionEvidenceUnavailable` | `Deny`            | The call requires a permission but no call-bound grant and scope evidence exists |
| `ReadOnlyScopeEvidenceUnavailable`      | `Deny`            | Read-only permission and target scope have not been established                  |
| `ReversibleRequiresApproval`            | `RequireApproval` | No narrow explicit-intent evidence is bound to the exact call                    |
| `PersonalDataRequiresApproval`          | `RequireApproval` | Personal-data modification always requires an exact trusted preview and approval |
| `ExternalOrHighImpactNotRegistered`     | `Deny`            | External or high-impact actions remain outside the MVP catalog                   |
| `ProhibitedAutonomy`                    | `Deny`            | Prohibited autonomy is always denied                                             |

Classification precedence is exact: `ProhibitedAutonomy`, then `ExternalOrHighImpactAction`, then any non-`None` permission, then the remaining risk-class mapping. This preserves the strongest hard-deny reason while ensuring ordinary permission-bearing classes cannot allow without evidence.

The current two registered schemas exercise `InformationOnly` and `ReversibleLocalAction`, both with `PermissionKind::None`. Keep the complete risk and permission rule table tested through a private pure classification helper. That helper is not a production constructor and cannot create a `PolicyInput`; public boundary tests must use real normalized and schema-validated calls.

## Trust and authorization boundary

The only production path to policy input after this increment is:

```text
normalized gateway frame
  -> UntrustedFunctionCall
  -> local registry and exact schema validation
  -> SchemaValidatedFunctionCall
  -> PolicyInput
  -> conservative deterministic PolicyDecision retaining the exact input
```

No model, gateway, WebView, or caller value can provide risk, permission state, intent, resource scope, policy outcome, policy reason, approval state, or execution authority. The direct `policy -> agent::function_call_validation` type dependency is narrow, portable, and acyclic. Moving shared domain values into a separate crate or module is deferred until O-002 is revisited; a module relocation would not improve this increment's security property.

## Approval and audit boundary

Increment 4C must not call or modify:

- `ApprovalRequestInput::new`.
- `ApprovalManager::create_request` or `decide`.
- `AuditEventInput::new` or `AuditLogger::record`.
- Any executor, tool implementation, Tauri command, WebView event, or storage repository.

The current approval and audit modules remain detached Phase 2 scaffolds. Their generic string fields are not evidence that an exact action is bound or redacted safely. A later approved increment must consume an eligible `PolicyDecision`, define exact preview and canonical digest rules, bind run, evidence, expiry, and one-time use, and define closed redacted audit fields before either scaffold can enter production orchestration.

## Dependency decision

Add no dependency and do not change `Cargo.toml`, `Cargo.lock`, `package.json`, or `package-lock.json`.

The implementation requires only Rust ownership, enums, private fields, and existing types. Adding serialization or hashing now would prematurely define the later approval contract and increase the personal-data handling surface.

## Explicit non-goals

- Caller-supplied policy context or booleans for explicit intent, permission state, scope, or freshness.
- Typed intent, permission, resource-scope, or freshness evidence and their trusted local producers.
- Canonical JSON or byte serialization, action hashing, cryptographic dependencies, signatures, or stable persistence formats.
- Run or conversation identity binding, approval previews, expiry, one-time consumption, rejection state, or local authentication.
- Approval request creation, approval decisions, audit events, audit persistence, execution eligibility, dispatch, or tool results.
- Tool implementations, runtime registry bootstrap, local task storage, device access, or operating-system permissions.
- Gateway transport, OpenAI calls, provider continuation, credentials, Keychain, identity, deployment, retries, or cancellation changes.
- Tauri commands/events, WebView IPC, UI changes, database migrations, capabilities, CSP, plugins, packaging, or permissions.
- New tool schemas, schema changes, provider protocol changes, policy configuration, user-configurable policy, or risk escalation by tool target.
- Treating `PolicyOutcome::Allow` as approval or dispatch authority.

## Exact implementation files

Create:

```text
src-tauri/tests/policy_input_binding.rs
```

Change:

```text
src-tauri/src/tools/types.rs
src-tauri/src/agent/types.rs
src-tauri/src/policy/types.rs
src-tauri/src/policy/engine.rs
```

Do not change `gateway_protocol.rs`, `function_call_validation.rs`, `schema.rs`, `provider.rs`, approvals, audit, storage, Tauri, frontend, manifests, or lockfiles.

Implementation closeout may update only:

```text
AGENTS.md
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
docs/increments/04c-trusted-policy-input-binding.md
docs/plans/04c-trusted-policy-input-binding.md
```

No file outside these implementation and closeout lists may change without stopping for project-owner approval.

## Implementation steps

1. Remove the unused raw `ToolCallProposal` type and mock-provider tool-call response variant.
2. Remove caller-supplied `PolicyContext`; add ownership-consuming `PolicyInput`, closed `PolicyReason`, and input-retaining `PolicyDecision` types.
3. Make deterministic policy evaluation total and conservative over only locally validated call metadata.
4. Retain full rule-table unit coverage through a private classification helper that cannot construct production input.
5. Add a public-boundary integration test that obtains calls through `GatewayStreamValidator` and `validate_function_call` before policy evaluation.
6. Prove exact identity, version, typed arguments, risk, and permission survive evaluation without caller-supplied metadata or context.
7. Prove policy input and decision debug output omit sentinel task content and raw argument JSON.
8. Confirm all legacy raw proposal/context symbols are removed and no approval, audit, executor, IPC, persistence, or network path was added.
9. Run focused checks, the complete repository gate, dependency audit, diff checks, code review, and security review.
10. Synchronize only the approved closeout documents and record the accepted boundary as D-023 if implementation is approved and verified.

## Risks and mitigations

- **Policy accepts unbound intent or permission facts:** remove `PolicyContext` entirely; permission-bearing and read-only calls deny, while reversible calls require approval.
- **Policy drops or swaps the evaluated action:** `PolicyDecision` owns the exact consumed `PolicyInput`; integration tests assert call identity, version, typed arguments, risk, and permission after evaluation.
- **Outcome and explanation disagree:** derive outcome from one closed `PolicyReason` instead of storing arbitrary strings and outcome independently.
- **An `Allow` decision becomes executable by implication:** add no consuming dispatch API and document policy allowance as non-authorizing data.
- **Personal content leaks through debug or errors:** do not derive content-bearing debug, clone, or serialization; use redacted custom debug and test sentinel exclusion.
- **Private rule tests create a hidden production bypass:** keep the classification helper private and pure; only public integration tests construct `PolicyInput`, through the verified call validator.
- **Removing legacy APIs breaks a hidden caller:** repository-wide search found no caller; compiler and full tests fail if one exists.
- **Typed canonical is mistaken for a stable approval digest:** explicitly defer serialization, hashes, previews, evidence, run binding, expiry, and one-time consumption to one later approved design.
- **Conservative denial is treated as complete permission policy:** document that no trusted grant/scope evidence exists and prohibit new permission-bearing tools in this increment.
- **Approval or audit scaffolds are treated as production-ready:** change neither module and add no conversion; record their remaining generic-input risk in handoff and the next plan.

## Test plan

- Preserve the three existing text-only mock-provider tests after removing the unused tool-call response variant.
- Unit-test every `PolicyReason -> PolicyOutcome` mapping.
- Unit-test hard-deny precedence and permission-evidence denial before ordinary risk classification for every non-`None` `PermissionKind` through the private helper.
- Unit-test all six risk classes with `PermissionKind::None` through the private helper.
- Through public APIs, parse normalized gateway frames, validate both registered schemas, construct `PolicyInput`, and evaluate them.
- Assert the decision retains exact call ID, tool name, contract version, typed arguments, local risk, and local required permission.
- Assert `get_current_datetime` allows with `InformationOnly`.
- Assert `create_local_task` requires approval with `ReversibleRequiresApproval`; no caller can mark intent true to bypass that result.
- Assert no constructor can supply or override tool name, version, arguments, risk, permission, intent, grant state, or scope at the policy boundary.
- Assert `PolicyInput` and `PolicyDecision` debug output omit a sentinel task title and raw JSON.
- Assert no public action-hash, preview, approval, audit, dispatch, or executor conversion exists.
- Preserve all existing frontend, Rust library, and Rust integration tests.

## Verification commands

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::provider
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::function_call_validation
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked policy::
cargo test --manifest-path src-tauri/Cargo.toml --test policy_input_binding --locked
npm run verify
npm audit --audit-level=low
git diff --check
git diff -- src-tauri/src/tools/types.rs src-tauri/src/agent/types.rs src-tauri/src/policy/types.rs src-tauri/src/policy/engine.rs src-tauri/tests/policy_input_binding.rs
```

Review gates:

- Run `$code-review` against the complete diff.
- Run `$security-review` because this increment changes the model-to-policy trust boundary.
- Confirm `ToolCallProposal`, `AgentProviderResponse::ToolCalls`, `AgentProviderResponse::tool_calls`, `PolicyContext`, `ProposedAction`, and `ProposedAction::new` no longer exist.
- Confirm `PolicyInput` has no caller-supplied tool metadata, raw argument, intent, permission-state, or scope constructor.
- Confirm permission-bearing and read-only classifications cannot allow without a later approved evidence type, and reversible actions cannot bypass approval through a caller boolean.
- Confirm approval, audit, executor, gateway protocol, schema, validator, provider implementation, Tauri, frontend, manifests, and lockfiles are unchanged.
- Confirm no generated file, database, build output, credential, URL, capability, CSP, or permission change is present.
- No native interaction check is required because the implemented modules remain transport-free and change no UI or platform behavior.

## Rollback

Remove the integration test and restore the prior `ToolCallProposal`, `AgentProviderResponse::ToolCalls`, `PolicyContext`, `ProposedAction`, public policy fields, string reasons, and fallible policy signature. Do not alter Increment 4A/4B validation, approvals, audit, storage, Tauri, frontend, manifests, capabilities, CSP, or permissions during rollback.

## Implementation evidence

The project owner approved the exact plan on 2026-07-14. Implementation changed only the five approved runtime/test files before synchronizing the approved closeout documents.

Passed:

```text
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::provider
  3 passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::function_call_validation
  6 passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked policy::
  4 passed
cargo test --manifest-path src-tauri/Cargo.toml --test policy_input_binding --locked
  2 passed
npm run verify
  frontend: 10 files, 124 tests passed
  Rust library: 78 passed
  Rust integration: 8 passed
  TypeScript and both Vite production builds passed
  Tauri release build --no-bundle passed
npm audit --audit-level=low
  0 vulnerabilities
git diff --check
code review
security review
```

The first sandboxed dependency audit could not resolve `registry.npmjs.org`; the approved network retry passed. No native interaction gate was required because the changed modules remain transport-free and unreferenced by Tauri.

## Acceptance criteria

- [x] The unused raw Rust proposal type and mock-provider tool-call variant are removed.
- [x] Only an owned `SchemaValidatedFunctionCall` can supply policy action identity, version, typed arguments, risk, and required permission.
- [x] `PolicyContext` and all caller-supplied intent, permission-state, and scope booleans are removed from the policy boundary.
- [x] Permission-bearing and read-only actions deny until exact trusted evidence exists; reversible actions require approval because narrow intent is not bound.
- [x] Policy evaluation is total over valid input and returns one closed reason with one derived outcome.
- [x] The policy decision retains the exact input it evaluated without cloning or serializing argument content.
- [x] Policy input and decision remain explicitly non-authorizing and expose no approval, audit, dispatch, or executor conversion.
- [x] Debug and error output retain no raw arguments, task title, parser detail, credential, or personal content.
- [x] Approval, audit, gateway protocol, schema validation, provider implementation, IPC, persistence, UI, capability, CSP, packaging, and permission boundaries remain unchanged.
- [x] No dependency or lockfile change is introduced.
- [x] D-023 records the approved trusted policy-input boundary after implementation.
- [x] Only the exact approved files change.
- [x] Focused tests, `npm run verify`, dependency audit, diff checks, code review, and security review pass.
- [x] Closeout documentation records actual evidence and the next exact task.

## Approval gate

The project owner approved the exact goal, raw-proposal and caller-context removal, canonical typed-input definition, conservative evidence boundary, closed reason/outcome model, input-retaining decision, exact files, no-dependency decision, tests, non-goals, verification gate, and rollback. Increment 4C is implemented and verified; do not start Increment 4D from this plan.
