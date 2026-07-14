# Phase 4 Increment 4C - trusted policy-input binding

Last updated: 2026-07-14

Status: **Verified complete**

## Goal

Remove the unused raw Rust proposal bypass and make an owned `SchemaValidatedFunctionCall` the only source of policy action identity, typed arguments, tool contract version, risk, and required permission. Remove caller-supplied policy context, evaluate conservatively where call-bound evidence does not exist, and retain the exact evaluated input without creating approval or execution authority.

## Planning result

- Increment 4B binds one normalized call to exact local identity, contract version, typed arguments, risk, and required permission.
- `ToolCallProposal` still accepts arbitrary raw JSON and caller-supplied classification, while `AgentProviderResponse::ToolCalls` can carry it despite having no caller.
- `ProposedAction::new` independently accepts name, risk, permission, and public context fields, so policy can be invoked without schema validation.
- `PolicyContext` accepts unbound intent and permission booleans that cannot prove the same call, run, resource scope, provenance, or freshness.
- Current policy output drops the evaluated action and stores arbitrary reason text separately from outcome.
- Approval and audit scaffolds accept generic caller-supplied strings and are not safe to connect in this increment.
- Repository search found no production raw-proposal, policy orchestration, policy-to-approval, audit, or executor caller.
- The smallest coherent increment removes the raw proposal and caller-context paths, introduces ownership-consuming `PolicyInput`, closes policy reasons/outcomes, and proves the public gateway-to-policy path with one integration test.
- Policy fails closed without call-bound evidence: permission-bearing and read-only actions deny, while reversible and personal-data actions require approval.
- Canonical means the exact ownership-bound typed call from Increment 4B. Serialized bytes, JSON, hashes, previews, evidence, run binding, expiry, one-time approval, and execution remain later work.
- The exact execution plan is `docs/plans/04c-trusted-policy-input-binding.md`.

## Planning baseline

```text
branch: phase4/increment-4c
base: clean merged main at 9fa095e
origin/main: 9fa095e
Node.js: v26.3.0
npm: 11.16.0
Cargo: 1.90.0
Rust: 1.90.0
rustfmt: 1.8.0-stable
Clippy: 0.1.90
macOS: 26.5.2 (25F84)
npm run typecheck: passed
function-call validation baseline: 6 passed
mock-provider baseline: 3 passed
policy baseline: 5 passed
approval baseline: 3 passed
audit baseline: 4 passed
```

## Exact planned implementation files

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

No gateway protocol, schema, function-call validator, provider implementation, approval, audit, executor, storage, Tauri, frontend, manifest, lockfile, capability, CSP, packaging, or permission file may change.

## Planned boundary

- Remove `ToolCallProposal` and the unused `AgentProviderResponse::ToolCalls` path.
- Remove `PolicyContext` and `ProposedAction` rather than trusting caller-supplied state booleans.
- Construct private `PolicyInput` only by consuming `SchemaValidatedFunctionCall`.
- Derive one `PolicyOutcome` from one closed `PolicyReason`.
- Retain the exact `PolicyInput` inside private-field `PolicyDecision`.
- Keep content-bearing policy values non-cloneable, non-serializable, and debug-redacted.
- Make evaluation total; malformed tool identity cannot reach policy after the required validation boundaries.
- Allow only information-only calls that require no permission.
- Deny permission-bearing and read-only calls until exact grant and resource-scope evidence exists.
- Require approval for reversible and personal-data actions because narrow intent or approval is not bound here.
- Add no conversion to approval, audit, dispatch, or execution.

## Dependency decision

Use Rust ownership and existing types only. Add no dependency and change no manifest or lockfile. Serialization, hashing, intent evidence, permission evidence, resource scope, and run binding are deferred because they must be designed with exact approval preview, expiry, and one-time-consumption semantics.

## Risks

- A caller-supplied boolean could be mistaken for same-call intent, permission, or scope evidence.
- A policy `Allow` outcome could be mistaken for execution authority.
- Policy could drop, clone, or leak the action it evaluated.
- Closed reasons and outcomes could drift.
- Removing unused proposal APIs could expose a hidden caller.
- Conservative denial could be mistaken for a complete permission-policy implementation.
- Typed canonical input could be mistaken for a stable approval digest.
- Generic approval and audit scaffolds could be connected prematurely.

The execution plan removes caller context, uses private fields and ownership, defines closed enums and redacted debug, preserves compiler checks, adds public-boundary tests, and makes later call-bound evidence and approval binding explicit non-goals.

## Verification gate

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
```

The complete diff must also pass `$code-review` and `$security-review`. No native interaction gate is required because the implemented change remains portable, transport-free, and unreferenced by Tauri.

## Rollback

Remove the new integration test and restore the prior raw proposal, provider tool-call variant, `PolicyContext`, `ProposedAction`, public context/decision fields, arbitrary string reasons, and fallible policy signature. Preserve Increment 4A/4B, approvals, audit, storage, Tauri, frontend, manifests, capabilities, CSP, and permissions.

## Approval gate

The project owner approved the exact plan on 2026-07-14. Implementation stayed within the approved runtime, test, and closeout file lists. After verification, implementation commit `7ec2969` and the publication-state closeout were pushed and fast-forward merged into `main` at the project owner's request.

## Actual results

- Removed `ToolCallProposal`, `AgentProviderResponse::ToolCalls`, `AgentProviderResponse::tool_calls`, `PolicyContext`, `ProposedAction`, `PolicyError`, and `PolicyResult`.
- Added private ownership-consuming `PolicyInput`, closed `PolicyReason`, derived `PolicyOutcome`, and private input-retaining `PolicyDecision`.
- Made deterministic policy evaluation total over one schema-validated call and conservative without exact intent, permission, or scope evidence.
- Preserved strongest hard-deny reasons, denied all other permission-bearing and read-only calls, required approval for reversible and personal-data calls, and allowed only information-only/no-permission calls as non-authorizing data.
- Added four rule-table unit tests and two public gateway-to-schema-to-policy integration tests covering exact retained identity, version, typed arguments, classification, outcomes, and debug redaction.
- Preserved all provider, schema-validation, frontend, Rust, integration, production build, and Tauri release checks.
- Added no dependency, lockfile, approval, audit, executor, network, credential, IPC, persistence, Tauri, capability, CSP, packaging, permission, or user-visible path.

Passed:

```text
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
mock provider: 3 passed
function-call validation: 6 passed
policy rule table: 4 passed
policy-input integration: 2 passed
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

The first sandboxed dependency audit could not resolve `registry.npmjs.org`; the approved network retry passed with zero vulnerabilities. No native interaction check was required because this increment changes only portable modules unreferenced by Tauri.
