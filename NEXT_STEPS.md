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
- Increment 4D — exact approval binding: **Verified complete**.

## Phase 4 Increment 4E - trusted approval-decision source planning

Status: **Ready - documentation only**

Goal:

- Reconcile the transport-free Increment 4D approval manager with the product approval experience, Tauri/WebView trust boundary, macOS-native interaction options, optional LocalAuthentication, security policy, and actual repository capabilities.
- Define the smallest non-executing source of trusted approval choices, including exact approval-ID and run/request/call binding, user-presence claims, expiry and cancellation, edit invalidation, replay handling, closed errors, and the boundary to future structured audit.
- Preserve the rule that neither a WebView message nor an Increment 4D `Approved` disposition is proof of a user gesture, user presence, local authentication, or execution authority.

Planning inputs:

```text
src-tauri/src/approvals/types.rs
src-tauri/src/approvals/manager.rs
src-tauri/src/lib.rs
src-tauri/capabilities/default.json
src/infrastructure/tauri/
docs/product/
docs/workflows/
SECURITY.md
CODE_REVIEW.md
docs/plans/04d-exact-approval-binding.md
```

Explicitly excluded:

- Runtime implementation, native dialogs, WebView decision handling, new Tauri commands or events, LocalAuthentication calls, operating-system permission requests, and capability or CSP changes during planning.
- Audit persistence, dispatch, executor wiring, tool implementation, provider continuation, gateway networking, credentials, Keychain, identity, deployment, and sensitive storage.
- Enabling denied policy classes, generic approval of arbitrary tools, multiple pending approvals, or treating an approval ID, preview, local disposition, local-authentication result, or future digest as execution authority.

Planning completion gate:

- Document which process and UI surface owns the trusted choice and why untrusted WebView content cannot assert it.
- Define exact typed inputs, outputs, cancellation, expiry, replay, user-presence and optional-authentication semantics, redaction, audit handoff, limits, errors, and fail-closed behavior.
- Recommend one smallest independently verified runtime increment with exact files, risks, non-goals, automated checks, any target-Mac manual gate, and rollback.
- Update planning documentation only, run documentation verification and reviews, then wait for project-owner approval.

No Increment 4E execution plan exists yet. Do not implement a trusted decision source, connect approval to execution, or begin live gateway work during this planning task.
