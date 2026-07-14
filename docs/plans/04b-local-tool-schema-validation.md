# Execution plan - Increment 4B exact local tool-schema validation

Last updated: 2026-07-14

Status: **Complete - verified**

## Gap analysis

Increment 4A proves a closed normalized gateway protocol and rejects malformed, oversized, duplicate-key, unknown-name, and contract-version-invalid function-call frames. Its accepted `UntrustedFunctionCall` still contains one raw JSON string, however, and is intentionally non-actionable.

The local tool foundation is not yet an adequate second validation boundary:

- `ToolSchema` is a public `{ version, document: String }` value whose only constructor stores the placeholder document `"{}"`.
- `ToolDefinition::new` accepts arbitrary names, schemas, risk classes, and permissions without binding them as one locally owned contract.
- `ToolCallProposal::new` accepts raw argument JSON plus caller-supplied risk and permission metadata.
- `AgentProviderResponse::ToolCalls` can contain those proposals, although no production path currently constructs or consumes them.
- `ToolRegistry` validates only non-empty names/descriptions and duplicate names. It does not validate arguments or contract versions.
- The policy engine trusts the `RiskClass` and `PermissionKind` already present on `ProposedAction`; it does not and should not parse model arguments.

| Boundary         | Verified state                                                 | Remaining gap                                                             |
| ---------------- | -------------------------------------------------------------- | ------------------------------------------------------------------------- |
| Gateway protocol | Closed, bounded, duplicate-aware, allowed-name/version checked | Arguments are only known to be one JSON object                            |
| Tool schema      | Arbitrary placeholder string                                   | No exact locally owned schema or typed argument result                    |
| Registry         | Deterministic name lookup                                      | Definitions do not derive identity and metadata from a closed contract    |
| Proposal         | Public raw-JSON constructor                                    | Must not receive gateway data before schema validation and later redesign |
| Policy           | Deterministic risk/permission evaluation                       | Requires trusted locally derived metadata, not gateway/model values       |

## Goal and outcome

Replace the placeholder schema representation with a closed local catalog for exactly two representative MVP input contracts and add one transport-free validator that consumes an Increment 4A `UntrustedFunctionCall`. Successful validation produces typed, schema-valid, locally classified data with no raw JSON and no authorization semantics.

This increment stops before `ToolCallProposal`, `ProposedAction`, policy evaluation, approval, audit, execution, provider continuation, or any user-visible behavior.

## Exact local contracts

The first catalog contains only:

### `get_current_datetime` contract version 1

- Risk: `InformationOnly`.
- Permission: `None`.
- Input: exactly one empty JSON object.
- Strict schema shape: object, empty `properties`, empty `required`, and `additionalProperties: false`.
- Typed arguments: a closed no-argument variant.

### `create_local_task` contract version 1

- Risk: `ReversibleLocalAction`, following the product brief's Class 2 examples.
- Permission: `None`.
- Input: exactly one required `title` string and no other property.
- Strict schema shape: object with required `title`, `additionalProperties: false`, `minLength: 1`, and `maxLength: 200`.
- Local semantic constraints: title is already trimmed, contains 1-200 Unicode scalar values, and contains no control character. Invalid input is rejected rather than normalized silently.
- Typed arguments: a private-field title value with a read-only accessor.

The fixed 200-character title ceiling is an Increment 4B contract choice that requires project-owner approval. Dates, due dates, notes, assignees, recurrence, priority, IDs, and arbitrary metadata are not part of version 1.

The frontend's `create_local_task` flow remains mock-only. Defining its Rust input contract neither registers a production executor nor changes the frontend mock risk label.

## Local ownership and validation design

1. Replace the arbitrary `ToolSchema` document with a closed `ToolSchema` enum containing only `GetCurrentDatetimeV1` and `CreateLocalTaskV1`.
2. Each schema variant owns its exact name, description, version, risk, permission, strict JSON Schema value, and typed argument parser.
3. Make `ToolDefinition` fields private and construct definitions only from a `ToolSchema`, so name, version, risk, and permission cannot be supplied independently.
4. Preserve deterministic registry storage and lookup. Registry tests use the two actual built-in contracts instead of arbitrary placeholder definitions.
5. Add a separate agent-layer function-call validator that consumes `UntrustedFunctionCall` by value plus `&impl ToolRegistry`; the consumed instance cannot be reused after this boundary.
6. Look up the local definition by the normalized call name, independently compare its contract version, and parse the argument string through that exact schema.
7. Return `SchemaValidatedFunctionCall`, with private fields for call ID, tool name, contract version, typed arguments, locally derived risk, and locally derived permission.
8. Clone only bounded opaque identity needed by the typed result, parse through borrowed accessors, and let the consumed call and its raw JSON string drop before returning. Content-bearing validated argument types do not derive `Clone`, `Serialize`, or raw-value `Debug`; custom `Debug` output for typed arguments and the validated call must not reveal a task title or raw arguments.
9. Map registry, version, and parsing failures to closed redacted errors that contain no tool arguments, title, or underlying `serde_json` message.

`SchemaValidatedFunctionCall` means only that local identity, version, schema, and metadata checks passed. It is not approved, permitted, executable, audited, or safe to dispatch. Its API and documentation must say so explicitly.

## Proposal and policy boundary

Increment 4B deliberately does not call or modify:

- `ToolCallProposal::new`.
- `AgentProviderResponse::tool_calls`.
- `ProposedAction::new`.
- `PolicyEngine::evaluate`.
- Any approval, audit, executor, Tauri, or frontend API.

The existing proposal types remain legacy architecture placeholders with no production caller. A later approved orchestration increment must remove or restrict their raw constructors and define the only conversion from `SchemaValidatedFunctionCall` into a policy input. That conversion must bind exact typed/canonical arguments and locally derived metadata; Increment 4B does not pre-authorize its design.

## Dependency decision

Add no dependency and do not change either Cargo manifest or lockfile.

Existing direct `serde` and `serde_json` dependencies can express these two small closed contracts with `deny_unknown_fields`, exact typed deserialization, explicit semantic checks, and fixed `serde_json::Value` schema documents. A general JSON Schema validator is unnecessary for two product-owned schemas and would add a larger parsing surface. `schemars` is already present transitively through unrelated dependencies, but promoting it to a direct dependency would generate schemas rather than validate local arguments and would not remove the need for explicit OpenAI-compatible strictness checks.

The schema document and typed validator could drift. Mitigate that risk with one module per contract catalog, exact schema-shape assertions, and a shared accepted/rejected fixture corpus. Reconsider a reviewed schema library only when several materially different schemas make this approach demonstrably repetitive or incomplete.

## Error and privacy boundary

- The consumed normalized call's raw argument JSON is dropped on every return path. A successful private typed value retains only the validated task title or the closed no-argument variant; the validator does not clone or return the raw JSON.
- Production errors contain only closed variants such as unknown local tool, contract mismatch, malformed/schema-invalid arguments, empty title, non-canonical title, title too long, or control character.
- Do not retain `serde_json::Error`, the raw JSON string, or the rejected title in an error.
- `Display` and `Debug` tests use sentinel content to prove it is absent.
- The validated-call debug representation may expose closed metadata and bounded opaque IDs, but not argument values.
- No logging or audit event is added.

## Explicit non-goals

- A generic JSON Schema engine, arbitrary/caller-defined schema documents, schema loading, schema persistence, or remote schema resolution.
- More than the two exact version-1 input contracts above.
- Tool output schemas, tool result validation, canonical approval serialization, hashes, signatures, or persistence.
- Conversion to `ToolCallProposal`, `AgentProviderResponse`, `ProposedAction`, policy, approval, audit, or execution.
- Tool implementations, runtime registration, dispatch, local task storage, calendar/device access, or operating-system permissions.
- Gateway transport, OpenAI calls, provider tool definitions, credentials, Keychain, identity, deployment, retries, or continuation.
- Tauri commands/events, WebView IPC, UI changes, capabilities, CSP, plugins, packaging, or database changes.
- Changing the existing frontend mock flow.

## Exact implementation files

Create:

```text
src-tauri/src/tools/schema.rs
src-tauri/src/agent/function_call_validation.rs
```

Change:

```text
src-tauri/src/tools/mod.rs
src-tauri/src/tools/types.rs
src-tauri/src/tools/registry.rs
src-tauri/src/agent/mod.rs
```

Focused tests remain inline in `schema.rs`, `registry.rs`, and `function_call_validation.rs`. Do not change `gateway_protocol.rs`; validator tests must obtain `UntrustedFunctionCall` through its public normalized-frame API.

Implementation closeout may update only:

```text
AGENTS.md
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
docs/increments/04b-local-tool-schema-validation.md
docs/plans/04b-local-tool-schema-validation.md
```

No file outside these implementation and closeout lists may change without stopping for project-owner approval.

## Implementation steps

1. Add the closed schema catalog, exact strict schema values, private typed arguments, semantic validation, redacted errors/debug output, and focused fixture tests.
2. Replace placeholder schema construction with schema-derived private `ToolDefinition` values and preserve deterministic registry behavior.
3. Add the agent-layer validator and non-authorizing `SchemaValidatedFunctionCall` value.
4. Test the real boundary by parsing normalized fixtures through `GatewayStreamValidator` before local registry/schema validation.
5. Prove local rejection of unknown tools, version mismatch, missing/extra/wrong-type fields, malformed values that survive the protocol-object check, title constraints, and redaction failures.
6. Confirm no proposal, policy, approval, audit, executor, IPC, persistence, network, capability, CSP, or permission path was added.
7. Run focused checks, the complete repository gate, dependency audit, diff checks, code review, and security review.
8. Synchronize only the approved closeout documents with actual evidence and record the approved closed-catalog/no-dependency trust-boundary decision in `DECISIONS.md`.

## Risks and mitigations

- **Schema and typed validator drift:** colocate the closed catalog and typed parsing; assert the exact JSON Schema shape and run one shared fixture corpus.
- **`validated` is mistaken for `authorized`:** use the name `SchemaValidatedFunctionCall`, private fields, explicit documentation, and no conversion or dispatcher API.
- **Risk or permission comes from the model:** derive both only from the local schema-backed registry definition; normalized gateway events cannot supply either field.
- **Raw personal content leaks through errors/debug:** map parsing failures to closed variants, implement redacted `Debug`, and test sentinel title/JSON exclusion.
- **Silent title normalization changes approval meaning:** reject non-canonical surrounding whitespace and control characters instead of trimming or rewriting.
- **An argument schema prematurely becomes a production tool:** add no runtime registry bootstrap or executor and retain the existing mock-only UI.
- **A generic schema abstraction grows too early:** freeze only two variants and require a later reviewed increment for each added contract family.
- **Legacy proposal constructors bypass the new validator later:** document them as untrusted placeholders and require a separate approved conversion/hardening increment before any orchestration wiring.

## Test plan

- Assert both exact names, descriptions, versions, risk classes, permissions, and strict JSON Schema values.
- Assert every object property is required and `additionalProperties` is false.
- Accept `{}` for `get_current_datetime`; reject any property, array, scalar, null, or trailing JSON.
- Accept one canonical `create_local_task` title at the 1- and 200-character boundaries.
- Reject missing title, additional properties, wrong types, empty/whitespace-only values, surrounding whitespace, 201 characters, control characters, malformed JSON, arrays, scalars, null, and trailing JSON.
- Preserve deterministic registry lookup/list ordering and duplicate/unknown rejection using the two built-in definitions.
- Parse accepted gateway fixtures into owned `UntrustedFunctionCall` values, then accept matching local definitions and independently reject unknown local tools or contract-version mismatch.
- Make the validator's ownership-consuming signature part of the reviewed API so a normalized raw call cannot be reused after schema validation.
- Prove successful values contain typed arguments and locally derived risk/permission while retaining no raw JSON string.
- Prove errors and debug output exclude sentinel title, raw arguments, and parser detail.
- Confirm no `ToolCallProposal`, `ProposedAction`, policy, approval, audit, or executor call occurs.
- Preserve all existing frontend, Rust library, and Rust integration tests.

## Verification commands

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked tools::schema
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked tools::registry
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::function_call_validation
npm run verify
npm audit --audit-level=low
git diff --check
git diff -- src-tauri/src/tools/mod.rs src-tauri/src/tools/types.rs src-tauri/src/tools/schema.rs src-tauri/src/tools/registry.rs src-tauri/src/agent/mod.rs src-tauri/src/agent/function_call_validation.rs
```

Review gates:

- Run `$code-review` against the complete diff.
- Run `$security-review` because this increment strengthens the model-to-tool trust boundary.
- Confirm `Cargo.toml` and `Cargo.lock` are unchanged.
- Confirm no Tauri command, invoke handler, provider/network client, URL, credential, database, capability, CSP, permission, generated file, or build output was added.
- No native manual interaction check is required because the modules remain unreferenced by Tauri and change no UI or platform behavior.

## Rollback

Remove the two new modules and their exports, restore the placeholder `ToolSchema` and current `ToolDefinition` API, and restore the existing registry tests. Do not alter Increment 4A, provider mocks, policy, approvals, audit, storage, Tauri configuration, capabilities, CSP, or permissions during rollback.

## Acceptance criteria

- [x] Exactly two versioned local input contracts replace the arbitrary placeholder schema.
- [x] Definitions derive tool identity, risk, and permission from the closed local catalog.
- [x] Normalized function calls are independently checked against local registry identity, contract version, exact schema, and semantic constraints.
- [x] Successful validation returns private typed arguments and locally derived metadata, with raw JSON discarded.
- [x] Schema-valid data remains explicitly non-authorizing and has no proposal, policy, approval, audit, IPC, or executor conversion.
- [x] Unknown tools, contract mismatches, missing/extra/wrong-type fields, bounds, non-canonical titles, controls, and malformed inputs fail closed.
- [x] Errors and debug output retain no raw arguments, task title, parser details, credentials, or personal content.
- [x] No dependency, lockfile, network, credential, gateway, persistence, Tauri, capability, CSP, packaging, permission, or user-visible change is introduced.
- [x] The approved local schema ownership and no-dependency decision is recorded durably in `DECISIONS.md`.
- [x] Only the exact approved implementation and closeout files change; the complete branch also retains the project-owner-approved planning correction to the Increment 4A merge record.
- [x] Focused tests, `npm run verify`, dependency audit, diff checks, code review, and security review pass.
- [x] Closeout documentation records actual evidence and the next exact task.

## Implementation result

The exact two-contract catalog, schema-derived private definitions, deterministic registry updates, ownership-consuming validator, typed non-authorizing output, closed redacted errors, and focused tests are implemented in the approved six runtime files. Raw JSON is dropped after successful parsing, and successful content-bearing values do not derive `Clone`, `Serialize`, or raw-value `Debug`.

The first focused schema test exposed that Serde empty-struct decoding did not enforce the planned object-only no-argument contract. The implementation was corrected to parse structured JSON and explicitly require an empty object. This stayed inside the approved schema module and all final checks pass.

Final evidence:

```text
schema tests: 5 passed
registry tests: 4 passed
gateway-to-local validation tests: 6 passed
npm run verify: passed
frontend: 10 files, 124 tests passed
Rust library: 79 passed
Rust integration: 6 passed
Tauri release build --no-bundle: passed
npm audit --audit-level=low: 0 vulnerabilities
git diff --check: passed
code review: passed with no findings
security review: passed with no findings
```

No native interaction test was required because no Tauri, UI, IPC, platform, persistence, capability, CSP, or permission behavior changed. Cargo manifests and lockfiles remain unchanged.

## Approval gate

The project owner approved this exact goal, two-contract catalog, 200-character title limit, file list, no-dependency decision, validation semantics, verification gate, and rollback on 2026-07-14. Implementation and closeout stayed within that scope. After verification, implementation commit `ce9fd40` and the publication-state closeout were pushed and fast-forward merged into `main` at the project owner's request.
