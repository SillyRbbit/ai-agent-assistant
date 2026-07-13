# Changelog

All notable repository changes are documented here. Entries distinguish verified work from implementation awaiting target-platform checks.

## Unreleased

### Pending target-Mac verification

- Phase 2 Increment 2D macOS menu-bar and window lifecycle.
- Menu actions for Open AI Agent Assistant, New Request, Tasks (Coming Soon), and Quit AI Agent Assistant.
- Main-window close-to-hide behavior and macOS Dock/reopen restoration.
- Closed-enum `assistant-menu-route` backend event for later React-shell integration.
- Focused deterministic unit and integration tests for routing, failure behavior, close policy, and reopen policy.

### Added

- Verified Phase 2 Increment 2C managed `Storage` startup integration.
- Typed `app_initialized` metadata bootstrap with idempotent migrations.
- macOS-only Tauri `tray-icon` support using the existing bundled icon as a temporary template icon.
- Platform-neutral menu action and lifecycle policy contracts.
- Increment 2D record and execution plan.

### Changed

- Marked Increment 2C verified complete based on target-Mac results: 41 Rust unit tests, 3 Rust integration tests, TypeScript, Vite, and native launch passed.
- Updated `npm run test:integration` to execute every Rust integration-test target rather than only the original smoke test.
- Changed the Rust entrypoint to build the Tauri app explicitly so macOS `RunEvent::Reopen` can restore the hidden main window.
- Updated project-memory files to make Increment 2D the active target-Mac verification gate.

### Security

- Kept `get_app_info` as the only custom Tauri command.
- Added no WebView-invokable capability, CSP change, plugin, global shortcut, API key, OAuth flow, model networking, database schema, or product-data persistence.
- Added no Accessibility, screen capture, Apple Events, microphone, shell, or broad filesystem access.
- Menu routes use closed enums, unknown menu IDs are ignored, and no arbitrary content is executed.

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
