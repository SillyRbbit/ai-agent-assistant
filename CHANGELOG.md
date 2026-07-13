# Changelog

All notable repository changes are documented here. This project follows a small-increment development process; entries distinguish verified work from implementation awaiting target-platform checks.

## Unreleased

### Pending target-Mac verification

- Phase 2 Increment 2C managed `Storage` abstraction and Tauri startup integration.
- Typed `app_initialized` metadata read/write behavior with fail-closed value and timestamp validation.
- Debug-only application-local development database and release-mode in-memory storage pending Keychain-backed key management.
- Focused tests for first and repeated initialization, typed metadata, corruption rejection, safe startup summaries, and public storage behavior.
- Target-Mac rustfmt, Clippy, Rust tests, frontend checks, and two native launches remain required before Increment 2C is marked verified.

### Added

- Verified Phase 2 Increment 2B-1 SQLite configuration, typed storage errors, private connection ownership, and versioned migrations.
- Verified Rust 1.90-compatible SQLite dependency chain using `rusqlite 0.37.0` and `libsqlite3-sys 0.35.0`.
- Exact SQLCipher-capable `rusqlite` dependency and test-only `tempfile` dependency declarations.
- Initial `STRICT` migrations for `schema_migrations` and `app_metadata` only.
- Focused storage tests for connection settings, ordering, idempotency, rollback, and public API behavior.
- Fail-closed migration metadata validation and no generic SQL execution API.
- Increment 2C record and execution plan.
- Phase 2 Increment 2B-0 storage decision record at `docs/increments/02b-0-sqlite-storage-decision.md`.
- Phase 2 Increment 2A platform-neutral Rust core interfaces.
- Deterministic mock or no-op implementations for `AgentProvider`, `ToolRegistry`, `PolicyEngine`, `ApprovalManager`, `AuditLogger`, `MemoryStore`, and `PlatformAdapter`.
- Shared `RiskClass` and `PermissionKind` placeholders for future tool policy work.
- Focused Rust unit tests for interface success, failure, deterministic ordering, secret-like rejection, and high-risk denial behavior.
- Persistent repository instructions, handoff files, workflows, skills, prompts, product documentation, and templates.

### Changed

- Replaced the incompatible `rusqlite 0.40.1` / `libsqlite3-sys 0.38.1` chain with versions verified against Rust 1.90.0.
- Marked Increments 2A, 2B-1, and 2B-1A as verified complete based on target-Mac results.
- Re-sequenced Phase 2 so storage startup is Increment 2C and menu-bar/window lifecycle is Increment 2D.
- Centralized Unix-millisecond timestamp generation for migrations and application metadata.
- Updated project-memory files to make Increment 2C the active verification gate.
- Expanded `README.md` with project-memory and assistant-workflow navigation.

### Security

- Kept release-mode storage in memory until a reviewed Keychain-backed database-key boundary exists.
- Restricted runtime metadata to a closed key/value enum and rejected malformed stored values.
- Preserved private raw SQLite ownership and avoided arbitrary SQL APIs.
- Preserved the existing Tauri command set, capabilities, CSP, UI, and permission footprint.
- Added no API keys, OAuth, model networking, shell access, Accessibility, screen capture, Apple Events, broad filesystem access, or product-data persistence.
- Preserved the product's deterministic authorization, credential, permission, and logging boundaries.

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
