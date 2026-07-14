# Phase 4 Increment 4B - exact local tool-schema validation

Last updated: 2026-07-14

Status: **Verified complete**

## Goal

Replace the arbitrary placeholder `ToolSchema` with a closed local catalog and independently validate Increment 4A normalized function calls into typed, non-authorizing Rust values before any proposal, policy, approval, audit, or executor conversion.

## Planning result

- `UntrustedFunctionCall` currently guarantees a bounded duplicate-free JSON object, allowed normalized name, and expected gateway contract version, but it intentionally does not know the exact local per-tool schema.
- `ToolSchema::placeholder` stores `"{}"`; `ToolDefinition::new` accepts independently supplied name/risk/permission/schema values; and `ToolCallProposal::new` accepts raw JSON plus caller-supplied classification.
- No production path currently converts normalized calls, invokes policy, or executes tools, so the missing schema boundary can be added transport-free without preserving unsafe compatibility behavior.
- The smallest representative catalog contains `get_current_datetime@1` with an exact empty object and `create_local_task@1` with one required canonical title capped at 200 Unicode scalar values.
- Schema-backed definitions derive identity, version, risk, and permission locally. Gateway/model content supplies none of those trusted metadata fields.
- Successful validation returns private typed arguments and discards raw JSON. Its type is explicitly schema-valid only, not authorized or executable.
- Existing `serde` and `serde_json` are sufficient. No new dependency, manifest edit, or lockfile edit is proposed.
- `ToolCallProposal`, `AgentProviderResponse`, policy, approvals, audit, executor, gateway transport, IPC, persistence, UI, and permissions remain unchanged.
- The exact execution plan is `docs/plans/04b-local-tool-schema-validation.md`.

## Planning baseline

```text
branch: phase4/increment-4b
base: clean merged main at e1db18b
origin/main: e1db18b
Node.js: v26.3.0
npm: 11.16.0
Cargo: 1.90.0
Rust: 1.90.0
rustfmt: 1.8.0-stable
Clippy: 0.1.90
macOS: 26.5.2 (25F84)
npm run typecheck: passed
tools registry baseline: 3 passed
gateway protocol baseline: 17 passed
```

## Exact planned implementation files

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

No Cargo, lockfile, provider, proposal, policy, approval, audit, storage, Tauri, frontend, capability, CSP, packaging, or permission file may change.

## Planned security properties

- Closed locally owned schema variants; no arbitrary schema string or remote reference.
- Exact local identity/version lookup after normalized gateway validation.
- Strict object shape, all properties required, and additional properties rejected.
- Typed successful arguments with raw JSON discarded.
- Risk and permission derived only from the local definition.
- Closed redacted failures and custom debug output with no task title or raw arguments.
- No authorization, approval, policy, audit, IPC, or execution semantics.
- No runtime tool registration or platform behavior.

## Dependency decision

Use only existing direct `serde` and `serde_json`. Do not promote transitive `schemars` or add a general JSON Schema validator. The two tiny closed schemas can be represented exactly and validated with typed deserialization plus focused fixtures; a dependency can be reconsidered when schema diversity creates a measured need.

## Risks

- Drift between strict schema documents and typed validation.
- Misreading schema-valid data as approved or executable.
- Leaking task titles through errors or derived debug output.
- Silently normalizing arguments and changing later approval meaning.
- Prematurely treating the mock `create_local_task` contract as a real registered tool.
- Leaving legacy raw proposal constructors available for a later caller to misuse.

The execution plan defines exact mitigations and tests for each risk.

## Verification gate

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked tools::schema
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked tools::registry
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::function_call_validation
npm run verify
npm audit --audit-level=low
git diff --check
```

The complete diff must also pass `$code-review` and `$security-review`. No native interaction gate is planned because the proposed modules remain transport-free and unreferenced by Tauri.

## Rollback

Remove the new schema and function-call-validation modules and exports, then restore the current placeholder schema/types/registry tests. Preserve Increment 4A and every provider, policy, approval, audit, storage, Tauri, capability, CSP, and permission boundary.

## Approval gate

The project owner approved the exact goal, two-contract catalog, 200-character title limit, file plan, no-dependency decision, validation semantics, verification gate, and rollback on 2026-07-14. No commit or push was requested or performed.

## Actual results

- Replaced the arbitrary placeholder schema with closed `GetCurrentDatetimeV1` and `CreateLocalTaskV1` variants that own exact metadata, strict schema values, and typed parsers.
- Made tool definitions private and schema-derived while preserving deterministic registry lookup, ordering, duplicate rejection, and unknown-tool rejection.
- Added an ownership-consuming local validator that independently checks normalized call identity, version, exact arguments, and semantic constraints.
- Returned private `SchemaValidatedFunctionCall` data containing typed arguments and locally derived risk/permission with explicit non-authorizing documentation and redacted debug output.
- Added five schema tests, four registry tests, and six gateway-to-local boundary tests.
- Recorded the durable schema-ownership and no-dependency boundary as D-022.
- Left `ToolCallProposal`, provider response, policy, approval, audit, executor, runtime registration, transport, IPC, persistence, and UI unchanged.

Passed:

```text
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked tools::schema
  5 passed; 0 failed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked tools::registry
  4 passed; 0 failed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::function_call_validation
  6 passed; 0 failed
npm run verify
  frontend: 10 files, 124 tests passed
  Rust library: 79 passed
  Rust integration: 6 passed
  TypeScript, ESLint, Prettier, rustfmt, Clippy, Vite builds, and Tauri release build --no-bundle passed
npm audit --audit-level=low
  0 vulnerabilities
git diff --check
code review
security review
```

Failed during development, then corrected:

- The first focused schema run had four passes and one failure because Serde empty-struct decoding did not enforce an object-only no-argument contract. The implementation now parses a JSON value and explicitly requires an empty object; all final focused and full tests pass.
- The first sandboxed npm audit could not resolve `registry.npmjs.org`. The approved network retry completed with zero vulnerabilities.

Checks not run:

- Native interaction testing, because the new portable modules remain unreferenced by Tauri and change no UI or platform behavior.

No manual verification remains pending. No Cargo manifest, lockfile, dependency, gateway protocol, provider, policy, approval, audit, storage, Tauri, frontend, capability, CSP, packaging, or permission file changed.
