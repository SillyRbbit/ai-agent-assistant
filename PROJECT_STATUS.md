# Project status

Last updated: 2026-07-13

## Current milestone

Phase 2 — local desktop shell and trusted local-core foundation.

## Increment status

- Increment 1: smallest runnable Tauri application — **complete**.
- Increment 1.1: Node.js 26/npm 11 compatibility — **complete**.
- Increment 1.2: repository workflow and handoff system — **complete**.
- Increment 2A: platform-neutral Rust interfaces and deterministic mocks — **verified complete on target Mac**.
- Increment 2B-0: SQLite storage dependency and design decision — **complete**.
- Increment 2B-1: SQLite dependency and migration skeleton — **verified complete on target Mac**.
- Increment 2B-1A: Rust 1.90 SQLite compatibility repair — **verified complete on target Mac**.
- Increment 2C: storage startup integration — **implementation complete; target-Mac verification pending**.
- Increment 2D remains blocked until Increment 2C is verified.

## Confirmed working baseline

Before Increment 2C:

- Tauri 2 launches on the target Mac.
- React renders in the native main window.
- The WebView invokes the typed Rust `get_app_info` command.
- Rust returns typed application metadata.
- Strict TypeScript typechecking passes.
- The Vite production build passes.
- Rust formatting, Clippy with warnings denied, and all Rust tests pass.
- `rusqlite 0.37.0` and `libsqlite3-sys 0.35.0` compile with Rust 1.90.0.
- The native application launches without a permission prompt.

## Implemented trusted-core foundations

- `AgentProvider` and deterministic `MockAgentProvider`.
- `ToolRegistry` and deterministic `InMemoryToolRegistry`.
- `PolicyEngine` and deterministic `DeterministicPolicyEngine`.
- `ApprovalManager` and deterministic `InMemoryApprovalManager`.
- `AuditLogger` with in-memory and no-op implementations.
- `MemoryStore` and deterministic `InMemoryMemoryStore`.
- `PlatformAdapter` and deterministic `MockPlatformAdapter`.
- Shared `RiskClass` and `PermissionKind` placeholders.

## Implemented storage foundation

- SQLCipher-capable `rusqlite 0.37.0` dependency compatible with Rust 1.90.
- Test-only `tempfile 3.23.0` dependency.
- Typed storage configuration and errors.
- In-memory and file-backed connections.
- Bounded and verified busy timeout.
- Verified foreign-key enforcement.
- Verified file-backed WAL mode.
- Private raw SQLite connection ownership.
- Immutable, checksummed, transactional migrations.
- `schema_migrations` and `app_metadata` only.
- Migration, connection, rollback, idempotency, and integration tests.

## Increment 2C implementation state

- `Storage` safely owns the database connection behind a mutex.
- Application metadata is restricted to a closed typed key/value contract.
- Corrupt values and negative timestamps fail closed.
- Migrations run before metadata access.
- `app_initialized=true` is created once and not rewritten on later launches.
- Tauri startup initializes storage through the setup hook.
- Initialized storage is registered as managed Rust state.
- Debug builds use an application-local development database.
- Release builds use in-memory storage pending Keychain-backed key management.
- Startup logging omits paths and metadata values.
- Existing UI and `get_app_info` IPC behavior are unchanged in source.

## Increment 2C verification state

Artifact workspace:

```text
Baseline npm ci: passed
Baseline npm run typecheck: passed
Baseline npm run build: passed
Post-change Prettier: passed
Post-change frontend lint: passed
Post-change strict TypeScript: passed
Post-change Vitest: 2 passed, 0 failed
Post-change Vite build: passed
Post-change npm audit: 0 vulnerabilities
Rust and native checks: not run because Cargo is unavailable on the artifact host
```

Remaining target-Mac gate:

- run rustfmt,
- run Clippy for all targets/features with warnings denied,
- run all Rust tests with the lockfile,
- run TypeScript and Vite checks,
- launch the application twice,
- confirm idempotent storage startup and unchanged UI/IPC,
- review the complete diff.

## Not implemented yet

- Product-data repositories or persistence.
- SQLCipher production key retrieval and Keychain integration.
- Menu-bar entry and hide/show lifecycle.
- Global shortcut.
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

- The debug database is intentionally unkeyed and must contain only migration metadata and the harmless initialization marker.
- Release storage remains ephemeral until key management is implemented.
- Migration definitions are immutable; corrections require new versions.
- A poisoned connection mutex fails closed and prevents storage use.
- Public GitHub state may lag the local checkout until changes are committed and pushed.
- Increment 2C Rust compilation and native startup remain unverified until the target-Mac commands run.

## Next milestone gate

Increment 2C is complete only when all required target-Mac checks pass and two consecutive debug launches prove migrations are idempotent while the existing UI and IPC behavior remain unchanged.

## Project memory map

- Current handoff: `HANDOFF.md`
- Priorities: `NEXT_STEPS.md`
- Decisions: `DECISIONS.md`
- Changes: `CHANGELOG.md`
- Plans: `PLANS.md`
- Troubleshooting history: `TROUBLESHOOTING_LOG.md`
- Product brief: `docs/product/PRODUCT_BRIEF.md`
- Architecture baseline: `docs/product/ARCHITECTURE_BASELINE.md`
- Increment 2A: `docs/increments/02a-core-interfaces.md`
- Increment 2B-0: `docs/increments/02b-0-sqlite-storage-decision.md`
- Increment 2B-1: `docs/increments/02b-1-sqlite-migration-skeleton.md`
- Increment 2B-1A: `docs/increments/02b-1a-rust-190-compatibility-repair.md`
- Increment 2C: `docs/increments/02c-storage-startup.md`
- Active plan: `docs/plans/02c-storage-startup.md`
