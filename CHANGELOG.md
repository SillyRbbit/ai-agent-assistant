# Changelog

All notable repository changes are documented here. This project follows a small-increment development process; entries describe verified repository changes rather than planned features.

## Unreleased

### Added

- Phase 2 Increment 2B-0 storage decision record at `docs/increments/02b-0-sqlite-storage-decision.md`.
- SQLite dependency and migration-skeleton acceptance criteria for Increment 2B-1.
- Phase 2 Increment 2A source implementation for platform-neutral Rust core interfaces.
- Deterministic mock or no-op implementations for `AgentProvider`, `ToolRegistry`, `PolicyEngine`, `ApprovalManager`, `AuditLogger`, `MemoryStore`, and `PlatformAdapter`.
- Shared `RiskClass` and `PermissionKind` placeholders for future tool policy work.
- Focused Rust unit tests for interface success, failure, deterministic ordering, secret-like rejection, and high-risk denial behavior.
- Increment record at `docs/increments/02a-core-interfaces.md`.
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

- Marked Increment 2A as verified complete based on target-Mac verification.
- Resolved the SQLite crate and encryption open decision for the first storage implementation.
- Updated `NEXT_STEPS.md` so Increment 2B-1 is the next ready implementation increment.
- Expanded `README.md` with project-memory and assistant-workflow navigation.

### Security

- Selected SQLCipher-capable SQLite support for the first storage implementation while keeping database keys out of SQLite and repository configuration.
- Preserved Phase 2 prohibited boundaries during Increment 2B-0: no runtime behavior, dependencies, Tauri commands, UI behavior, model networking, macOS permissions, shell execution, OAuth, or API-key storage were added.
- Documented the product's non-negotiable authorization, credential, permission, and logging boundaries.
- Preserved Phase 2 prohibited boundaries while adding Increment 2A: no SQLite, model networking, new Tauri commands, macOS permissions, shell execution, OAuth, or API-key storage were added.

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
