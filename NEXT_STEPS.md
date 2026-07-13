# Next steps

Last updated: 2026-07-13

This file is the ordered implementation queue. Work only on the first item marked **Ready** unless a blocking defect requires a smaller troubleshooting increment.

## Completed increments

- Increment 2A — core interfaces and deterministic mocks: **Verified complete**.
- Increment 2B-0 — SQLite storage decision: **Complete**.
- Increment 2B-1 — SQLite dependency and migration skeleton: **Verified complete**.
- Increment 2B-1A — Rust 1.90 compatibility repair: **Verified complete**.
- Increment 2C — storage startup integration: **Verified complete**.

Increment 2C target-Mac result: 41 unit tests and 3 integration tests passed, TypeScript and Vite passed, the native app launched, and later startup reported `applied_migrations=0`, `already_applied_migrations=2`, and `previously_initialized=true`.

## Increment 2D — macOS menu-bar and window lifecycle

Status: **Implementation complete; target-Mac verification required**

Implemented:

- macOS-only Tauri tray feature,
- one menu-bar entry,
- open-main-window action,
- new-request route action,
- tasks-placeholder route action,
- quit action,
- main-window close-to-hide behavior,
- macOS Dock/reopen restoration when no window is visible,
- deterministic routing and lifecycle tests,
- no UI, IPC-command, capability, CSP, storage-schema, or permission expansion.

Required verification:

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
npm run typecheck
npm run build
npm run tauri -- dev
```

Manual gate:

- menu-bar icon appears,
- all four fixed actions behave correctly,
- closing the main window hides rather than exits,
- menu-bar and Dock reopen the hidden window,
- quit exits,
- UI, `get_app_info`, and storage startup remain correct,
- no permission prompt appears.

Record: `docs/increments/02d-menu-bar-window-lifecycle.md`.

Plan: `docs/plans/02d-menu-bar-window-lifecycle.md`.

## Increment 2E — React application shell

Status: **Blocked by Increment 2D verification**

Goal: implement basic navigation and page shells while consuming the closed menu route event.

Planned work:

- Sidebar entries for Conversations, Tasks, Memory, Activity, Integrations, Permissions, and Settings.
- Conversation pane and input composer.
- Empty, loading, and error states.
- Settings page shell.
- Permission Center shell with no OS permission requests.
- Route-event listener for New Request and Tasks placeholder.
- Component and reducer tests.

## Increment 2F — mocked agent streaming and activity

Status: **Blocked by Increment 2E**

Goal: prove the local run-event model with the deterministic mock provider.

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
