# Phase 4 Increment 4D - exact approval binding

Last updated: 2026-07-14

Status: **Verified complete**

## Goal

Replace the detached caller-authored approval scaffold with one transport-free Rust boundary that consumes an eligible `PolicyDecision`, retains the exact run/request/call identity and typed arguments, derives the approval preview from that same owned value, and consumes approve, reject, cancel, or expiry exactly once without creating dispatch or execution authority.

## Planning result

- At planning start, `PolicyDecision` retained the exact schema-validated action, but `SchemaValidatedFunctionCall` lost the already-verified run and gateway-request IDs.
- The previous `ApprovalRequestInput` independently accepted arbitrary tool names, action hashes, and preview strings. None was bound to the policy decision, and repository search found no production caller.
- The previous `ApprovalRequest` and manager cloned detached content, had no expiry or cancellation, permitted multiple decisions only through mutable state checks, and exposed no run/call replay boundary.
- The generic audit scaffold accepts arbitrary strings and cannot safely receive exact approval content. It remains disconnected.
- The smallest coherent increment carries validator-owned run and gateway-request identity through local schema validation and policy, then lets the approval manager consume only `PolicyOutcome::RequireApproval`.
- The manager owns one exact non-cloneable policy decision, assigns a correlation-only approval ID, derives a borrowed closed preview from the retained typed arguments, and accepts a decision by approval ID only. A caller cannot resupply or replace identity, arguments, classification, preview, or policy outcome.
- Approval lifetime is fixed at 120 seconds from request creation on a manager-owned monotonic clock. Only one approval may be pending, and one manager admits at most 1,024 subjects without evicting replay history.
- Approve, reject, cancellation, and expiry are terminal. Consumed run/request/call identities are rejected on replay for the manager lifetime, and an edit requires a fresh validated call rather than mutating an approved subject.
- An `Approved` disposition records only the transport-free manager's closed Rust choice and exact subject binding. It does not prove a user gesture, user presence, or local authentication; that trusted source remains a later prerequisite for execution.
- One-time, capacity, and replay guarantees are scoped to one manager instance. A future orchestrator must own one authoritative instance before any execution path is added.
- The plan adds no action digest and removes the existing untrusted `action_hash`. Exact same-process ownership is the canonical binding; a digest would duplicate sensitive canonicalization, add a dependency, and still provide no authorization if returned by an untrusted caller.
- The exact execution plan is `docs/plans/04d-exact-approval-binding.md`.

## Planning baseline

```text
branch: phase4/increment-4d
base: clean merged main at 55626b6
origin/main: 55626b6
Node.js: v26.3.0
npm: 11.16.0
Cargo: 1.90.0
Rust: 1.90.0
rustfmt: 1.8.0-stable
Clippy: 0.1.90
macOS: 26.5.2 (25F84)
npm run typecheck: passed
gateway protocol baseline: 17 passed
function-call validation baseline: 6 passed
policy baseline: 4 passed
approval baseline: 3 passed
audit baseline: 4 passed
policy-input integration baseline: 2 passed
```

## Implementation result

- The project owner approved the exact plan and file list before runtime edits.
- `UntrustedFunctionCall` and `SchemaValidatedFunctionCall` now retain validator-owned run and gateway-request IDs with the existing call identity. Policy and approval retain that exact owned value.
- Content-bearing gateway calls and events no longer derive `Clone` or raw `Debug`; custom debug output omits raw arguments and output-text deltas.
- The detached approval input and record types were replaced with an `ApprovalManager` that consumes only an owned `RequireApproval` decision and derives one borrowed typed `create_local_task@1` preview.
- Request views and resolutions retain exact identity, closed policy/classification metadata, and borrowed preview access without cloning, serialization, or a consuming dispatch/executor conversion.
- The manager enforces one pending request, a 1,024-subject lifetime cap, manager-owned 120-second monotonic expiry, explicit cancellation, terminal approve/reject/cancel/expiry, and non-evicting replay tombstones.
- `action_hash` was removed. No digest, dependency, manifest, or lockfile change was introduced.
- `Approved` remains non-authorizing local state and does not prove a user gesture, user presence, local authentication, active run, or execution eligibility.
- D-024 records the durable exact-binding and authority boundary.

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

No policy rule, tool schema, registry, audit, executor, provider, storage, Tauri, frontend, manifest, lockfile, capability, CSP, packaging, or permission file may change.

## Planned boundary

- Retain the validator-owned run ID and gateway-request ID on `UntrustedFunctionCall`, then carry them through `SchemaValidatedFunctionCall` and the existing input-retaining policy decision.
- Remove content-bearing gateway call/event cloning and redact both raw arguments and output-text deltas from debug output while changing that identity boundary.
- Remove `ApprovalRequestInput`, caller-supplied tool names, `action_hash`, arbitrary preview text, clonable approval records, and public construction of approval IDs.
- Create a request only by consuming a `PolicyDecision` whose derived outcome is exactly `RequireApproval`.
- Support only the current `create_local_task@1` approval preview. Any later approval-required tool needs an explicit typed preview variant before it can enter this manager.
- Represent the preview as a borrowed closed Rust variant over the retained task title plus fixed target, schedule, recipient, reversibility, risk, and permission facts. Do not store a second content copy or arbitrary presentation string.
- Use a fixed 120-second monotonic deadline, one pending request maximum, and a 1,024-subject manager-lifetime cap. No caller supplies the creation time, deadline, or lifetime, and replay tombstones are never evicted to admit more work.
- Resolve by manager-generated approval ID and closed choice only. Deadline checks precede approval, and stale input resolves as expired rather than approved.
- Provide explicit cancellation and expiry-consumption hooks so future orchestration and audit work can observe terminal outcomes without making them executable.
- Return a sealed, non-cloneable, non-serializable, debug-redacted resolution that still owns the exact policy decision. It exposes read-only facts but no consuming conversion back into policy input, approval creation, dispatch, or execution.
- Reject unknown IDs, duplicate decisions, duplicate subject identities, ineligible policy outcomes, unsupported preview variants, pending-limit conflicts, deadline overflow, and ID exhaustion through closed redacted errors.

## Canonical preview and digest decision

The canonical approval subject is the single owned value already established by Increments 4A-4C:

```text
verified run ID
verified gateway-request ID
validated function-call ID
local tool name and contract version
closed typed arguments
locally derived risk and required permission
closed policy reason and derived RequireApproval outcome
```

For `create_local_task@1`, the preview is a borrowed typed projection of that subject. The exact affected data is the already-validated title. Target, no schedule, no recipients, reversibility, permission, and main risk are closed local facts; they are not caller or model strings.

Increment 4D defines no canonical bytes and no digest. The manager binds an approval ID directly to the owned subject, and decision APIs accept no arguments or digest to compare. If a future persistence or cross-process requirement genuinely needs a digest, it must define a versioned domain-separated encoding, a reviewed cryptographic dependency, content-retention implications, and mismatch behavior in a separate decision. Such a digest would remain correlation data, never approval or execution authority.

## Dependency decision

Use Rust ownership, private fields, closed enums, `std::time::Instant`, and existing collections only. Add no dependency and change no manifest or lockfile. The transitively present hashing crates are not promoted because hashing does not improve the in-process ownership binding and could expose low-entropy personal arguments to offline guessing.

## Risks

- Retaining only the call ID could bind a decision to the wrong run or gateway request.
- Caller-authored previews or hashes could describe one action while the retained policy decision contains another.
- A stale approval could be accepted after its run ended or after cancellation.
- A relative approval TTL could be mistaken for proof that the owning run is still active.
- Approve, reject, or edit could be replayed against an already consumed subject.
- Parallel manager instances could each process the same subject if a future orchestrator failed to own one authoritative instance.
- A cloneable request or resolution could duplicate content or be mistaken for a reusable authority token.
- Approval IDs or future digests could be mistaken for authentication secrets.
- Exact preview content could leak through `Debug`, errors, or the generic audit string fields.
- A new approval-required tool could accidentally reuse a generic preview.
- An approved resolution could be wired directly to an executor before dispatch and audit gates exist.

The plan mitigates these risks with verified run/request/call identity retention, one owned subject, closed typed previews, a manager-owned monotonic deadline, explicit cancellation, one-time terminal consumption, bounded non-evicting replay tombstones, redacted non-cloneable values, no digest, no audit call, and no executor conversion. The plan explicitly does not treat the relative TTL as run-liveness evidence; future orchestration must cancel approval when the run terminates or reaches its absolute deadline.

## Verification gate

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
```

The complete diff must also pass `$code-review` and `$security-review`. No native interaction gate is required because the proposed implementation remains portable, transport-free, and unreferenced by Tauri.

## Verification result

Passed:

```text
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_protocol
  18 passed; 0 failed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::function_call_validation
  6 passed; 0 failed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked policy::
  4 passed; 0 failed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked approvals::
  6 passed; 0 failed
cargo test --manifest-path src-tauri/Cargo.toml --test policy_input_binding --locked
  2 passed; 0 failed
cargo test --manifest-path src-tauri/Cargo.toml --test approval_binding --locked
  2 passed; 0 failed
npm run verify
  124 frontend tests, 82 Rust library tests, and 10 Rust integration tests passed
  TypeScript, Vite production builds, and Tauri release build --no-bundle passed
npm audit --audit-level=low
  0 vulnerabilities
git diff --check
code review
security review
  passed with no findings
```

The first sandboxed dependency-audit attempt failed with `ENOTFOUND registry.npmjs.org`; the required retry with network access completed and reported zero vulnerabilities. No native interaction check was run or required because the modules remain transport-free and unreferenced by Tauri or the UI.

## Rollback

Remove the new integration test, restore the prior gateway and validated-call identity shapes, and restore the generic detached approval types and manager. Preserve the verified 4A-4C protocol, schema, policy rules, audit, provider, storage, Tauri, frontend, manifests, capabilities, CSP, and permissions.

## Approval gate

The project owner approved the exact goal, identity propagation, canonical-preview/no-digest decision, relative 120-second expiry, one-pending and 1,024-subject limits, run-cancellation boundary, terminal and replay semantics, exact files, no-dependency decision, tests, non-goals, verification gate, and rollback before implementation. No scope expansion occurred. After verification, implementation commit `1bf1f10` and the publication-state closeout were pushed and fast-forward merged into `main` at the project owner's request.
