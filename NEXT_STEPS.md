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

## Phase 4 Increment 4B planning - exact local tool-schema validation

Status: **Ready - documentation only**

Goal:

- Reconcile the placeholder `ToolSchema`, current tool registry, `ToolCallProposal`, policy boundary, and verified `UntrustedFunctionCall` with the product and security requirements.
- Define the smallest transport-free increment that gives trusted Rust an exact locally owned per-tool schema and validates normalized arguments before any proposal or policy conversion.
- Produce one exact implementation plan with files, dependency decision, typed errors, adversarial tests, risks, non-goals, verification, rollback, and an explicit project-owner approval gate.

Planning inputs:

```text
src-tauri/src/agent/gateway_protocol.rs
src-tauri/src/agent/types.rs
src-tauri/src/tools/registry.rs
src-tauri/src/policy/engine.rs
docs/plans/04a-gateway-protocol-contract.md
```

Explicitly excluded:

- Runtime or dependency edits during planning.
- Gateway networking, OpenAI calls, credentials, Keychain, identity, deployment, provider continuation, and tool-result submission.
- Policy authorization, approval UI, executor wiring, Tauri/WebView IPC, persistence, capabilities, CSP, packaging, or operating-system permissions.
- Generic or caller-defined schemas, provider-selected authority, broad tool-registry refactors, and any real tool execution.

Planning completion gate:

- Inspect clean merged `main`, repository memory, current Rust contracts/tests, and exact dependency state.
- Recommend only one smallest independently verified increment and update planning documentation only.
- Run the smallest relevant documentation/baseline checks, review the complete diff, and wait for project-owner approval.

Do not start Increment 4B implementation or any live gateway work during this planning task.
