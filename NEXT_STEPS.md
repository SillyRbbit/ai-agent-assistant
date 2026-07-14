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

## Increment 2G — integration hardening

Status: **Ready**

Goal:

- Complete bounded cancellation and error-state hardening.
- Add a deterministic local audit-view scaffold without sensitive persistence.
- Complete release-verification hardening appropriate to Phase 2.
- Preserve the verified React, Rust, SQLite, menu-bar, lifecycle, IPC, CSP, capability, and permission boundaries.

Required planning gate before implementation:

- Define exact acceptance criteria for cancellation, errors, audit presentation, and release evidence.
- Keep the increment small enough to verify independently; split it if the plan spans unrelated trust boundaries.
- State the exact file list and wait for approval before editing.

Explicitly excluded:

- Production OpenAI or gateway access.
- API keys, credentials, OAuth, or cloud accounts.
- Real tool execution or trusted WebView authorization.
- New operating-system permissions or privileged platform APIs.
- Sensitive SQLite persistence.

Do not broaden the increment beyond the approved plan.

## Out of scope for the remaining Phase 2 increments

- Production OpenAI access or stored API keys.
- OAuth or gateway authentication.
- Accessibility, screen capture, Apple Events, microphone, or shell execution.
- Broad filesystem access.
- Calendar, contacts, reminders, notifications, or clipboard tools.
- Real tool execution or trusted WebView authorization.
- Autonomous external or destructive actions.
