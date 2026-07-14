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

## Increment 2E — React application shell

Status: **Implementation complete — target-Mac verification pending**

Goal: replace the proof-of-connection screen with a platform-neutral React application shell and consume the existing closed menu-route event.

Implemented:

- React reducer plus context for local deterministic shell state.
- Sidebar navigation for Conversations, Tasks, Memory, Activity, Integrations, Permissions, and Settings.
- Conversation workspace and non-functional in-memory composer.
- Empty, loading, and error presentation states.
- Settings shell containing existing `get_app_info` diagnostics.
- Permission Center shell with no operating-system request controls.
- Strict `assistant-menu-route` listener.
- Closed routing for `new_request` and `tasks_placeholder`.
- Safe rejection of malformed, unknown, or extended payloads.
- Focused reducer, event parser, subscription, page, diagnostics, Settings, and Permissions tests.

Artifact-host result:

```text
Prettier: passed
ESLint: passed
TypeScript: passed
Vitest: 30 passed, 0 failed
Vite build: passed
npm audit: 0 vulnerabilities
git diff --check: passed
```

Required target-Mac verification:

```bash
npm run format:check
npm run lint
npm run typecheck
npm run test:unit
npm run test:integration
npm run build
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
npm run tauri -- dev
```

Manual gate:

- The shell remains usable at the configured minimum window size.
- Every sidebar item opens the correct page shell.
- Conversation workspace and disabled composer render.
- Settings shows Rust core diagnostics.
- Permission Center opens without an OS prompt.
- Menu-bar New Request opens Conversations and clears the draft.
- Menu-bar Tasks opens Tasks.
- Unknown native event payloads do not change the route.
- Close-to-hide, menu-bar reopen, Dock reopen, and Quit still work.
- Storage startup remains idempotent.
- No permission prompt appears.

Record: `docs/increments/02e-react-application-shell.md`.

Plan: `docs/plans/02e-react-application-shell.md`.

Do not mark Increment 2E complete until every required automated and manual check passes.

## Increment 2F — mocked agent streaming and activity

Status: **Blocked by Increment 2E verification**

Goal: prove the local run-event model with the deterministic mock provider, then add mocked assistant text streaming, tool activity presentation, and approval presentation.

The scope must still exclude production model access, API keys, gateway calls, privileged tools, and user-data persistence.

## Increment 2G — integration hardening

Status: **Blocked by Increment 2F**

Goal: complete cancellation, error-state, audit-view, and release-verification hardening without adding production model access or privileged automation.

## Out of scope for the remaining Phase 2 increments

- Production OpenAI access or stored API keys.
- OAuth or gateway authentication.
- Accessibility, screen capture, Apple Events, microphone, or shell execution.
- Broad filesystem access.
- Calendar, contacts, reminders, notifications, or clipboard tools.
- Autonomous external or destructive actions.

## Current authoritative queue — 2026-07-13

### Increment 2E — React application shell

Status: **Complete**

All automated and manual target-Mac checks passed.

### Increment 2F — mocked assistant interaction shell

Status: **Ready**

Goal:

- Add deterministic in-memory assistant messages and mock streaming.
- Add stop behavior.
- Add tool activity presentation.
- Add a trusted mock approval dialog and deterministic approve/reject/edit behavior.
- Preserve the verified React, Rust, SQLite, menu-bar, lifecycle, IPC, CSP, capability, and permission boundaries.

Explicitly excluded:

- OpenAI, gateway, or other model networking.
- API keys or credentials.
- Real tool execution.
- New Tauri commands.
- SQLite persistence for conversations or approvals.
- Accessibility, screen capture, Apple Events, shell execution, OAuth, or OS permission requests.
