# Changelog

All notable repository changes are documented here. This project follows a small-increment development process; entries describe verified repository changes rather than planned features.

## Unreleased

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
