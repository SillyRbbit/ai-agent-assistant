# Next steps

Last updated: 2026-07-13

This file is the ordered implementation queue. Work only on the first item marked **Ready**. A verification-pending increment must be closed before later feature work begins.

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

## Phase 3 — gap analysis and increment planning

Status: **Ready**

Goal:

- Compare the Phase 3 product requirements with the verified Phase 2 implementation.
- Identify the smallest missing conversation-UI or mocked-loop capability.
- Define one bounded first Phase 3 increment with explicit acceptance criteria, non-goals, risks, exact files, and verification commands.
- Preserve every verified security, IPC, storage, capability, CSP, lifecycle, and permission boundary.

Required planning output:

- A written gap analysis grounded in `docs/product/PRODUCT_BRIEF.md` and actual code.
- One dedicated Phase 3 plan document.
- An exact proposed file list and verification gate.
- Project-owner approval before implementation.

Explicitly excluded from planning implementation:

- Production model or gateway access.
- API keys, credentials, OAuth, or cloud accounts.
- Real tools, privileged automation, or trusted WebView authorization.
- Sensitive persistence or new operating-system permissions.

Do not edit runtime code during the planning task.

## Out of scope for the remaining Phase 2 increments

- Production OpenAI access or stored API keys.
- OAuth or gateway authentication.
- Accessibility, screen capture, Apple Events, microphone, or shell execution.
- Broad filesystem access.
- Calendar, contacts, reminders, notifications, or clipboard tools.
- Real tool execution or trusted WebView authorization.
- Autonomous external or destructive actions.
