# Changelog

All notable repository changes are documented here. Entries distinguish verified work from implementation awaiting target-platform checks.

## Unreleased

### Added

- Phase 2 Increment 2E React application shell, pending target-Mac verification.
- Reducer-and-context navigation for Conversations, Tasks, Memory, Activity, Integrations, Permissions, and Settings.
- Conversation workspace with a controlled, non-functional in-memory composer.
- Settings diagnostics, Permission Center placeholders, and reusable empty, loading, and error presentations.
- Strict frontend listener for the closed `assistant-menu-route` native event.
- Thirty focused frontend tests across reducer, event parsing, navigation, diagnostics, Settings, and Permissions.
- Verified Phase 2 Increment 2C managed `Storage` startup integration.
- Typed `app_initialized` metadata bootstrap with idempotent migrations.
- Verified Phase 2 Increment 2D macOS menu-bar and window lifecycle.
- macOS-only Tauri `tray-icon` support using the existing bundled icon as a temporary template icon.
- Fixed menu actions for Open AI Agent Assistant, New Request, Tasks (Coming Soon), and Quit AI Agent Assistant.
- Main-window close-to-hide behavior and macOS Dock/reopen restoration.
- Closed-enum `assistant-menu-route` backend event for later React-shell integration.
- Platform-neutral menu action and lifecycle policy contracts.
- Focused deterministic unit and integration tests for routing, failure behavior, close policy, and reopen policy.
- Increment 2D record and execution plan.

### Changed

- Replaced the proof-of-connection page with the platform-neutral React shell while retaining `get_app_info` diagnostics.
- Resolved O-004 with React reducer plus context and no new state-management dependency.
- Routed menu-bar New Request to Conversations with a cleared draft and Tasks to the Tasks page.
- Marked Increment 2E implementation complete with target-Mac verification still pending.
- Marked Increment 2C verified complete based on target-Mac results: 41 Rust unit tests, 3 Rust integration tests, TypeScript, Vite, and native launch passed.
- Marked Increment 2D verified complete based on target-Mac Rust, frontend, native launch, menu-action, close, reopen, Dock, quit, storage, and permission checks.
- Updated `npm run test:integration` to execute every Rust integration-test target rather than only the original smoke test.
- Changed the Rust entrypoint to build the Tauri app explicitly so macOS `RunEvent::Reopen` can restore the hidden main window.
- Updated project memory to make Increment 2E — React application shell — the next ready increment.
- Documented that the custom right-side menu-bar status item is distinct from the standard left-side macOS application menu.

### Security

- Added no Rust, SQLite, Tauri command, capability, CSP, dependency, persistence, credential, networking, or operating-system permission change in Increment 2E.
- Native event payloads enter as `unknown` and are ignored unless they exactly match one closed route object.
- Permission Center is status-only and contains no control that can request operating-system access.
- Kept `get_app_info` as the only custom Tauri command.
- Added no WebView-invokable capability, CSP change, plugin, global shortcut, API key, OAuth flow, model networking, database schema, or product-data persistence.
- Added no Accessibility, screen capture, Apple Events, microphone, shell, or broad filesystem access.
- Menu routes use closed enums, unknown menu IDs are ignored, and no arbitrary content is executed.
- Target-Mac verification produced no macOS permission prompt.

## 0.1.0 — 2026-06-18

### Added

- Smallest runnable Tauri 2 desktop application.
- React 19, TypeScript 5.9, and Vite 7 frontend.
- Typed `get_app_info` command across the Tauri IPC boundary.
- Typed Rust startup error propagation.
- Restrictive Content Security Policy and minimal Tauri capability set.
- ESLint, Prettier, rustfmt, Clippy, Vitest, Rust unit tests, and Rust integration smoke test.

### Changed

- Added compatibility for Node.js 26.3.0 and npm 11.16.0 while preserving strict engine enforcement.

### Security

- No API keys, shell plugin, opener plugin, broad filesystem plugin, Accessibility, screen capture, or Apple Events access.

## 2026-07-13 — Phase 2 Increment 2E verified

- Marked the React application shell as verified complete on the target Mac.
- Recorded successful frontend, Rust, native-launch, menu-routing, lifecycle, diagnostics, storage-idempotence, and no-permission-prompt checks.
- Marked Phase 2 Increment 2F as Ready.
