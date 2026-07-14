# Next steps

Last updated: 2026-07-13

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

## Phase 3 completion — gap analysis and planning

Status: **Ready**

Goal:

- Compare the remaining mocked-loop and conservative-limit requirements with the verified implementation through Increment 3C.
- Determine whether post-result provider continuation, final-answer presentation, or explicit loop limits require one more Phase 3 increment.
- If work remains, define one smallest bounded increment with explicit acceptance criteria, non-goals, risks, exact files, and verification commands.
- If no work remains, document the evidence required to close Phase 3.
- Preserve every verified security, cancellation, Retry, Activity, IPC, storage, lifecycle, capability, CSP, and permission boundary.

Required planning output:

- A written completion gap analysis grounded in product documents and actual code.
- One dedicated plan if another increment is required, or a documented Phase 3 closure recommendation.
- An exact file list and verification gate for any proposed work.
- Project-owner approval before implementation or phase closure.

Explicitly excluded:

- Context source selection, real data collection, attachments, or voice.
- Production model or gateway access, API keys, credentials, OAuth, or cloud accounts.
- Trusted Rust provenance, audit persistence, real tools, or privileged automation.
- Production provider continuation, arbitrary result schemas, dynamic external payloads, or additional real tools.
- Rust, IPC, Tauri commands, dependencies, capabilities, CSP, packaging, or new operating-system permissions.

Do not edit runtime code during the planning task.
