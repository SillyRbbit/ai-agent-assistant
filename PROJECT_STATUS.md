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
- Next ready increment: Increment 2B-1 — SQLite dependency and migration skeleton.

## Confirmed working behavior

- Tauri 2 application launches on the target Mac.
- React renders in the main native window.
- The WebView invokes the typed Rust `get_app_info` command.
- The Rust core returns typed app metadata.
- Strict TypeScript typechecking passes.
- Vite production build passes.
- Rust formatting, Clippy, unit tests, and integration tests passed for Increment 2A on the target Mac.

## Implemented Rust core foundations

- `AgentProvider` interface and deterministic `MockAgentProvider`.
- `ToolRegistry` interface and deterministic `InMemoryToolRegistry`.
- `PolicyEngine` interface and deterministic `DeterministicPolicyEngine`.
- `ApprovalManager` interface and deterministic `InMemoryApprovalManager`.
- `AuditLogger` interface with in-memory and no-op implementations.
- `MemoryStore` interface and deterministic `InMemoryMemoryStore`.
- `PlatformAdapter` interface and deterministic `MockPlatformAdapter`.
- Shared `RiskClass` and `PermissionKind` placeholders.

## Storage decision status

The SQLite approach has been selected but not implemented.

Accepted decision:

- Use `rusqlite` with the bundled SQLCipher feature set in Increment 2B-1.
- Use `tempfile` as a dev dependency for isolated migration tests.
- Add only a minimal migration foundation first.
- Do not persist product data in 2B-1.

## Not implemented yet

- SQLite dependency and migration code.
- Menu-bar entry and hide/show lifecycle.
- Sidebar navigation and application pages.
- Mock streaming assistant wired to the UI.
- Tool activity card.
- Approval dialog.
- Settings page.
- Permission Center shell.
- Gateway or model networking.
- Keychain integration.
- Production persistence repositories.

## Intentionally prohibited or deferred

- Production model credentials.
- API-key storage.
- OAuth.
- Accessibility.
- Screen capture.
- Apple Events.
- Unrestricted shell execution.
- Broad filesystem access.
- Calendar, contacts, reminders, notifications, or clipboard tools.
- Autonomous external or destructive actions.

## Development health

Target Mac environment confirmed by the user:

```text
Node.js: 26.3.0
npm: 11.16.0
Rust: 1.90.0-aarch64-apple-darwin
Platform: Apple Silicon macOS
```

The public GitHub repository may lag behind the local verified working tree unless local changes have been committed and pushed. Before each new implementation session, confirm:

```bash
git status --short --branch
git log -1 --oneline
```

## Known risks

- Adding SQLCipher may introduce native build complexity on macOS; Increment 2B-1 must be verified on the target Mac before it is considered complete.
- SQLite key management is not implemented yet; database keys must not be generated or stored until a dedicated secret-store boundary exists.
- Combining database implementation with UI or Tauri IPC work would make failures harder to isolate.
- Documentation can drift unless the end-session workflow is followed.

## Next milestone gate

Increment 2B-1 is complete only when the SQLite dependency, storage error type, connection configuration, migration runner, initial `schema_migrations` and `app_metadata` tables, idempotency tests, rollback tests, Rust checks, and npm checks all pass without adding product data persistence or new capabilities.

## Project memory map

- Current handoff: `HANDOFF.md`
- Priorities: `NEXT_STEPS.md`
- Decisions: `DECISIONS.md`
- Changes: `CHANGELOG.md`
- Troubleshooting history: `TROUBLESHOOTING_LOG.md`
- Product brief: `docs/product/PRODUCT_BRIEF.md`
- Architecture baseline: `docs/product/ARCHITECTURE_BASELINE.md`
- Increment 2A record: `docs/increments/02a-core-interfaces.md`
- Increment 2B-0 record: `docs/increments/02b-0-sqlite-storage-decision.md`
