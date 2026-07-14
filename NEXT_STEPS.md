# Next steps

Last updated: 2026-07-14

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

## Phase 4 Increment 4D planning - exact trusted approval binding

Status: **Ready - documentation only**

Goal:

- Reconcile the verified input-retaining `PolicyDecision` with the generic approval types and manager, local audit boundary, run/call identity, typed tool arguments, product approval requirements, and actual repository callers.
- Define one smallest transport-free increment that can create and consume an approval only for the exact eligible policy input without granting dispatch or execution authority.
- Specify canonical preview and digest representation, run/call binding, expiry, one-time decision semantics, rejection behavior, content redaction, exact files, tests, risks, non-goals, verification, and rollback.

Planning inputs:

```text
src-tauri/src/agent/function_call_validation.rs
src-tauri/src/policy/types.rs
src-tauri/src/policy/engine.rs
src-tauri/src/approvals/types.rs
src-tauri/src/approvals/manager.rs
src-tauri/src/audit/
docs/plans/04c-trusted-policy-input-binding.md
```

Explicitly excluded:

- Runtime or dependency edits during planning.
- Approval UI, WebView decisions, IPC, audit persistence, executor wiring, dispatch, tool implementation, and provider continuation.
- Permission-grant or resource-scope evidence, device access, operating-system permissions, and enabling currently denied policy classes.
- Gateway networking, OpenAI calls, credentials, Keychain, identity, deployment, persistence, capabilities, CSP, packaging, or operating-system permissions.
- Treating a policy decision, approval preview, digest, or approval record as execution authority.

Planning completion gate:

- Inspect clean merged `main`, repository memory, the verified 4A-4C contracts, current approval/audit types, actual callers, and exact dependency state.
- Recommend only one smallest independently verified increment and update planning documentation only.
- Run the smallest relevant documentation and baseline checks, review the complete diff, and wait for project-owner approval.

Do not implement Increment 4D, connect approval to execution, or begin live gateway work during this planning task.
