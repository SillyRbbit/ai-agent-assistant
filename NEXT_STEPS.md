# Next steps

Last updated: 2026-07-13

This file is the ordered implementation queue. Work only on the first item marked **Ready** unless a blocking defect requires a smaller troubleshooting increment.

## Completed increments

- Increment 2A — core interfaces and deterministic mocks: **Verified complete**.
- Increment 2B-0 — SQLite storage decision: **Complete**.
- Increment 2B-1 — SQLite dependency and migration skeleton: **Verified complete**.
- Increment 2B-1A — Rust 1.90 compatibility repair: **Verified complete**.
- Increment 2C — storage startup integration: **Verified complete**.
- Increment 2D — macOS menu-bar and window lifecycle: **Verified complete**.

Increment 2D target-Mac result:

- Rust formatting passed.
- Clippy passed for all targets and features with warnings denied.
- All Rust unit and integration tests passed.
- TypeScript typechecking and the Vite production build passed.
- The native application launched.
- The menu-bar icon and all four fixed actions worked.
- Closing hid the main window without terminating the process.
- Menu-bar and Dock reopen behavior worked.
- Quit terminated the process.
- Storage remained idempotent.
- `get_app_info` remained functional.
- The React UI remained unchanged.
- No permission prompt appeared.

Record: `docs/increments/02d-menu-bar-window-lifecycle.md`.

Plan: `docs/plans/02d-menu-bar-window-lifecycle.md`.

## Increment 2E — React application shell

Status: **Ready**

Goal: replace the proof-of-connection page with the in-memory React application shell and safely consume the existing closed menu-route event.

### Planned work

- Resolve O-004 before introducing shared frontend state. Prefer React reducer plus context unless another dependency is clearly necessary and documented.
- Add a platform-neutral application-state model.
- Add sidebar entries for:
  - Conversations
  - Tasks
  - Memory
  - Activity
  - Integrations
  - Permissions
  - Settings
- Add the conversation workspace.
- Add a non-functional input composer shell.
- Add empty, loading, and error presentation states.
- Add a Settings page shell.
- Add a Permission Center shell with placeholder permission rows and no OS permission requests.
- Listen for the existing `assistant-menu-route` event.
- Accept only `new_request` and `tasks_placeholder`.
- Route `new_request` to Conversations.
- Route `tasks_placeholder` to Tasks.
- Ignore malformed or unknown event payloads safely.
- Preserve the typed `get_app_info` wrapper and display connection diagnostics in an appropriate page.
- Keep state local and in memory.
- Add focused reducer, route-event, accessibility, and component tests.

### Constraints

- Preserve the existing menu-bar, close, Dock reopen, and quit behavior.
- Preserve SQLite startup and migrations.
- Preserve `get_app_info` as the only custom Tauri command.
- Preserve Tauri capabilities and CSP.
- Do not add mocked assistant streaming, tool activity cards, or approval behavior yet.
- Do not persist conversations, tasks, navigation state, settings, or permissions.
- Do not add or change SQLite migrations.
- Do not add a global shortcut.
- Do not add API keys, OpenAI access, gateway calls, OAuth, or model networking.
- Do not request Calendar, Reminders, Contacts, Notifications, Files, Accessibility, Screen Recording, Automation, or Microphone permissions.
- Do not add shell execution, broad filesystem access, or platform automation.
- Do not add a frontend dependency unless it is clearly necessary and recorded in `DECISIONS.md`.

### Required verification

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

### Manual gate

- The shell renders at the current minimum native-window size.
- Every sidebar item opens the correct page shell.
- The conversation workspace and composer appear.
- Empty, loading, and error states render correctly.
- Settings opens.
- Permission Center opens without an OS prompt.
- New Request from the menu bar opens Conversations.
- Tasks (Coming Soon) from the menu bar opens Tasks.
- Closing, menu-bar reopening, Dock reopening, and quitting continue to work.
- Rust connection diagnostics remain available.
- Storage startup remains idempotent.
- No permission prompt appears.

## Increment 2F — mocked agent streaming and activity

Status: **Blocked by Increment 2E**

Goal: prove the local run-event model with the deterministic mock provider.

Planned work includes mocked streaming, tool activity cards, and the approval dialog. It must not add production model access or privileged operating-system tools.

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
