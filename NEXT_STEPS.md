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

## Phase 4 Increment 4C planning - trusted proposal and policy-input binding

Status: **Ready - documentation only**

Goal:

- Reconcile `SchemaValidatedFunctionCall`, the legacy raw `ToolCallProposal`, `AgentProviderResponse`, `ProposedAction`, and `PolicyEngine` with the product, security, and actual Rust boundaries.
- Define the smallest transport-free increment that makes one locally validated call the only source of canonical policy input without granting approval or execution authority.
- Produce one exact plan covering canonical argument representation, identity/version/risk/permission binding, typed errors, tests, files, non-goals, risks, verification, and rollback.

Planning inputs:

```text
src-tauri/src/agent/function_call_validation.rs
src-tauri/src/agent/types.rs
src-tauri/src/tools/types.rs
src-tauri/src/policy/types.rs
src-tauri/src/policy/engine.rs
src-tauri/src/approvals/
src-tauri/src/audit/
docs/plans/04b-local-tool-schema-validation.md
```

Explicitly excluded:

- Runtime or dependency edits during planning.
- Approval issuance or consumption, audit persistence, executor wiring, tool implementations, and provider continuation.
- Gateway networking, OpenAI calls, credentials, Keychain, identity, deployment, IPC, UI, persistence, capabilities, CSP, packaging, or operating-system permissions.
- Treating schema validity or policy allowance as user approval or execution authorization.

Planning completion gate:

- Inspect clean merged `main`, repository memory, the verified 4A/4B contracts, current policy/proposal/approval/audit types, and exact dependency state.
- Recommend only one smallest independently verified increment and update planning documentation only.
- Run the smallest relevant documentation and baseline checks, review the complete diff, and wait for project-owner approval.

Do not implement Increment 4C or begin live gateway work during this planning task.
