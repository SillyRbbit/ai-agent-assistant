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

## Phase 3C — gap analysis and increment planning

Status: **Ready**

Goal:

- Compare the remaining Phase 3 tool-result and mocked-loop requirements with the verified implementation through Increment 3B.
- Identify the smallest missing user-visible capability after conversation identity and mock context provenance.
- Define one bounded Phase 3C increment with explicit acceptance criteria, non-goals, risks, exact files, and verification commands.
- Preserve every verified security, cancellation, Retry, Activity, IPC, storage, lifecycle, capability, CSP, and permission boundary.

Required planning output:

- A written gap analysis grounded in product documents and actual code.
- One dedicated Phase 3C plan document.
- An exact proposed file list and verification gate.
- Project-owner approval before implementation.

Explicitly excluded:

- Context source selection, real data collection, attachments, or voice.
- Production model or gateway access, API keys, credentials, OAuth, or cloud accounts.
- Trusted Rust provenance, audit persistence, real tools, or privileged automation.
- Rust, IPC, Tauri commands, dependencies, capabilities, CSP, packaging, or new operating-system permissions.

Do not edit runtime code during the planning task.
