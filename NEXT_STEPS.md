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

## Phase 4 planning — gateway and Responses security boundary

Status: **Ready**

Goal:

- Perform a documentation-only reconciliation of Phase 4 provider and gateway requirements against the verified Phase 3 architecture.
- Define credential ownership, authenticated gateway responsibilities, strict Responses event and function-call validation, cancellation, timeout, limit, error-redaction, and audit boundaries before implementation.
- Determine the smallest independently verified Phase 4 increment and its prerequisite decisions.
- Preserve the rule that the gateway cannot execute local tools and the WebView/model cannot authorize or execute operating-system actions.
- Produce one exact plan, file list, risk analysis, verification gate, and rollback strategy for project-owner approval.

Required planning output:

- A gap analysis grounded in product, architecture, security, decision, and actual repository state.
- Explicit trust-boundary diagrams or data-flow descriptions for app, gateway, provider, model output, tools, approval, audit, and credentials.
- A recommendation for one smallest Phase 4 increment or a prerequisite decision-only increment.
- Exact acceptance criteria, non-goals, files, risks, verification commands, and approval gate.
- No runtime implementation before project-owner approval.

Explicitly excluded:

- Production provider calls, gateway deployment, network access, API keys, credentials, OAuth, or cloud-account setup.
- Real tool execution, new approvals, privileged automation, or operating-system integrations.
- New Rust or WebView IPC commands, dependencies, capabilities, CSP changes, packaging, persistence, or permissions.
- Context collection, attachments, voice, or unrelated product work.

Do not edit runtime code during this planning task.
