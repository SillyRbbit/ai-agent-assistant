# Changelog

All notable repository changes are documented here.

This project follows a small-increment development process. Entries describe verified repository changes rather than planned features.

## Phase 2 Increment 2A — 2026-07-09

### Added

- Platform-neutral Rust `AgentProvider` interface and deterministic mock implementation.
- Platform-neutral Rust `ToolRegistry` interface and deterministic in-memory implementation.
- Platform-neutral Rust `PolicyEngine` interface and deterministic policy implementation.
- Platform-neutral Rust `ApprovalManager` interface and deterministic in-memory implementation.
- Platform-neutral Rust `AuditLogger` interface, in-memory logger, no-op logger, and simple secret-like token redaction.
- Platform-neutral Rust `MemoryStore` interface and deterministic in-memory implementation.
- Platform-neutral Rust `PlatformAdapter` interface and deterministic mock implementation.
- Shared `RiskClass` enum compatible with the product policy.
- Shared `PermissionKind` placeholder for future permission modeling.
- Focused Rust tests covering representative success, failure, deterministic ordering, unknown tools, denied high-risk/prohibited actions, approval state, audit ordering, memory rejection, and platform capability placeholders.
- Increment documentation at `docs/increments/02a-core-interfaces.md`.

### Verified

- Rust formatting check passed after applying `cargo fmt`.
- Clippy passed with warnings denied.
- Rust tests passed with 25 unit tests and 1 integration test.
- TypeScript check passed.
- Vite production build passed.
- Native Tauri development app launched successfully.

### Security

- No API keys, model networking, backend gateway calls, SQLite, macOS permissions, new Tauri commands, Accessibility, ScreenCaptureKit, Apple Events, shell execution, OAuth, file tools, calendar tools, contact tools, reminders, notifications, or clipboard tools were added.
- Existing UI behavior and `get_app_info` IPC behavior were preserved.

## Phase 2 Increment 1.2 — 2026-06-18

### Added

- Persistent repository instructions in `AGENTS.md`.
- Working-session handoff and status files.
- Prioritized Phase 2 next-step plan.
- Decision, changelog, and troubleshooting logs.
- Human and coding-assistant usage guide.
- Start, resume, end-session, troubleshooting, review, and security workflows.
- Repository-scoped skills under `.agents/skills/`.
- Reusable prompt library under `prompts/`.
- Product brief and architecture baseline in repository documentation.
- Templates for handoffs, increments, decisions, and troubleshooting entries.

### Changed

- Expanded `README.md` with project-memory and assistant-workflow navigation.

### Security

- Documented the product's non-negotiable authorization, credential, permission, and logging boundaries.

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
