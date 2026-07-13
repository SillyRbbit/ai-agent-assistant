# Project status

Last updated: 2026-07-13

## Current milestone

Phase 2 — local desktop shell and trusted local-core foundation.

## Current increment status

- Increment 1: smallest runnable Tauri application — **complete**.
- Increment 1.1: Node.js 26/npm 11 compatibility — **complete**.
- Increment 1.2: repository workflow and handoff system — **complete**.
- Increment 2A: platform-neutral Rust interfaces and deterministic mocks — **verified complete on target Mac**.
- Increment 2B-0: SQLite storage dependency and design decision — **complete**.
- Increment 2B-1: SQLite dependency and migration skeleton — **implementation complete; target-Mac verification pending**.
- Increment 2C remains blocked until 2B-1 is fully verified.

## Confirmed working behavior before 2B-1

- Tauri 2 application launches on the target Mac.
- React renders in the main native window.
- The WebView invokes the typed Rust `get_app_info` command.
- The Rust core returns typed app metadata.
- Strict TypeScript typechecking passes.
- Vite production build passes.
- Increment 2A Rust formatting, Clippy, unit tests, and integration tests passed on the target Mac.

## Implemented Rust core foundations

- `AgentProvider` interface and deterministic `MockAgentProvider`.
- `ToolRegistry` interface and deterministic `InMemoryToolRegistry`.
- `PolicyEngine` interface and deterministic `DeterministicPolicyEngine`.
- `ApprovalManager` interface and deterministic `InMemoryApprovalManager`.
- `AuditLogger` interface with in-memory and no-op implementations.
- `MemoryStore` interface and deterministic `InMemoryMemoryStore`.
- `PlatformAdapter` interface and deterministic `MockPlatformAdapter`.
- Shared `RiskClass` and `PermissionKind` placeholders.

## Implemented storage foundation

- Exact `rusqlite` SQLCipher dependency declaration.
- Test-only `tempfile` dependency declaration.
- Typed storage configuration and errors.
- In-memory and file-backed connection modes.
- Bounded busy timeout with verification.
- Foreign-key enforcement with verification.
- File-backed WAL with verification.
- Private raw connection ownership.
- Immutable, checksummed, transactional migration catalog.
- `schema_migrations` and `app_metadata` only.
- Unit and public-API integration tests.

## Verification state

Artifact workspace checks:

```text
Baseline npm ci: passed
Baseline npm run typecheck: passed
Baseline npm run build: passed
Storage-only cargo check: passed
Storage-only cargo clippy -D warnings: passed
Storage-only cargo test: 8 passed, 0 failed
```

Remaining gate:

- resolve and review `src-tauri/Cargo.lock` on the target Mac,
- run full Tauri-crate formatting, Clippy, and tests with the pinned Rust toolchain,
- rerun TypeScript and Vite checks,
- launch the app and confirm unchanged behavior.

## Not implemented yet

- Product-data SQLite repositories.
- SQLCipher production key retrieval and Keychain integration.
- Menu-bar entry and hide/show lifecycle.
- Sidebar navigation and application pages.
- Mock streaming assistant wired to the UI.
- Tool activity card.
- Approval dialog.
- Settings page.
- Permission Center shell.
- Gateway or model networking.

## Intentionally prohibited or deferred

- Production model credentials or API-key storage.
- OAuth.
- Accessibility.
- Screen capture.
- Apple Events.
- Unrestricted shell execution.
- Broad filesystem access.
- Calendar, contacts, reminders, notifications, or clipboard tools.
- Autonomous external or destructive actions.

## Known risks

- The bundled SQLCipher/OpenSSL build must pass on the Apple Silicon target Mac before 2B-1 is considered complete.
- File-backed databases are not encrypted until a later secret-store increment supplies a key; no product data may use this foundation yet.
- Migration definitions are immutable after release; changing a version's name, checksum, or SQL is rejected.
- Documentation and the public repository can lag the user's local checkout unless the end-session and push workflow is followed.

## Next milestone gate

Increment 2B-1 is complete only when the target Mac has an updated lockfile, all required locked Rust checks pass, npm checks pass, and the existing Tauri application launches without UI or IPC changes.

## Project memory map

- Current handoff: `HANDOFF.md`
- Priorities: `NEXT_STEPS.md`
- Decisions: `DECISIONS.md`
- Changes: `CHANGELOG.md`
- Plans: `PLANS.md`
- Troubleshooting history: `TROUBLESHOOTING_LOG.md`
- Product brief: `docs/product/PRODUCT_BRIEF.md`
- Architecture baseline: `docs/product/ARCHITECTURE_BASELINE.md`
- Increment 2A record: `docs/increments/02a-core-interfaces.md`
- Increment 2B-0 record: `docs/increments/02b-0-sqlite-storage-decision.md`
- Increment 2B-1 record: `docs/increments/02b-1-sqlite-migration-skeleton.md`
- Active plan: `docs/plans/02b-1-sqlite-migration-skeleton.md`
