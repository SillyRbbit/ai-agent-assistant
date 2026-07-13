# Increment 2B-0 — SQLite storage dependency and design decision

## Goal

Decide the SQLite storage approach before adding database code, dependencies, migrations, or persistence behavior.

This is a documentation and architecture-decision increment. It prepares Increment 2B-1 but does not implement SQLite yet.

## Status

Status: **Complete**

Compatibility note: the architectural choice of `rusqlite` with bundled SQLCipher remains accepted, but the provisional 0.40.1 version shown below was superseded by D-011 after target-Mac verification. The verified implementation uses `rusqlite 0.37.0` with `libsqlite3-sys 0.35.0` on Rust 1.90.0.

## Scope

This increment documents:

- the selected Rust SQLite crate,
- the planned encryption approach,
- the first storage module boundary,
- initial migration rules,
- initial schema scope,
- security constraints for future persistence work,
- verification expectations for the first SQLite implementation increment.

## Decision summary

Use `rusqlite` as the first SQLite access layer for the local Rust core.

When the dependency is introduced in Increment 2B-1, use an exact version and the SQLCipher bundled feature set unless target-Mac verification exposes a blocker:

```toml
rusqlite = { version = "=0.40.1", features = ["bundled-sqlcipher-vendored-openssl"] }
```

Use `tempfile` as a development dependency for isolated migration tests:

```toml
[dev-dependencies]
tempfile = "=3.23.0"
```

The dependency is intentionally not added in 2B-0 because this increment is a decision gate only.

## Rationale

`rusqlite` is a synchronous, small-surface SQLite wrapper that fits the trusted local-core model. It provides explicit connection and transaction control, works without an async runtime, and avoids introducing compile-time query macros or a network-capable database abstraction.

`sqlx` is not selected for the first storage layer because the current application needs deterministic local SQLite access, not an async database framework. The added runtime and offline-query workflow would increase build complexity before persistence requirements are concrete.

The SQLCipher bundled feature is selected so database encryption is available from the first database implementation rather than requiring a plaintext-to-encrypted migration later. The first implementation still must keep database keys outside SQLite; production key retrieval belongs behind a later secret-store adapter.

## Planned storage module boundary

Increment 2B-1 should add storage modules under:

```text
src-tauri/src/storage/mod.rs
src-tauri/src/storage/error.rs
src-tauri/src/storage/config.rs
src-tauri/src/storage/connection.rs
src-tauri/src/storage/migrations.rs
```

Initial responsibilities:

- Open a configured SQLite connection.
- Apply connection PRAGMAs.
- Apply versioned migrations in a transaction.
- Expose typed `StorageError` and `StorageResult`.
- Support in-memory and temporary-file database tests.
- Avoid connecting storage to Tauri IPC or the React UI.

## Required connection rules for 2B-1

Every opened connection must:

- enable foreign keys with `PRAGMA foreign_keys = ON`,
- verify that foreign keys are actually enabled,
- use WAL mode for file-backed databases where supported,
- set a bounded busy timeout,
- avoid loading external SQLite extensions,
- use prepared statements for values,
- return typed errors instead of panicking.

The implementation should use SQLCipher keying only through an explicit configuration value passed from tests or a future secret-store boundary. It must not create a production key, prompt Keychain, or store a key in SQLite during 2B-1.

## Initial migration scope for 2B-1

Add only the minimal schema needed to prove the migration system:

```sql
CREATE TABLE schema_migrations (
  version INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  checksum TEXT NOT NULL,
  applied_at_ms INTEGER NOT NULL
) STRICT;

CREATE TABLE app_metadata (
  key TEXT PRIMARY KEY,
  value TEXT NOT NULL,
  updated_at_ms INTEGER NOT NULL
) STRICT;
```

Do not persist conversations, messages, memories, tasks, audit events, tool calls, approvals, permissions, file scopes, OAuth state, API keys, or raw sensitive content in 2B-1.

## Security constraints for 2B-1

- No API keys, OAuth tokens, database keys, passwords, authentication codes, or private keys in SQLite.
- No production Keychain integration yet.
- No Tauri commands.
- No UI behavior changes.
- No network access.
- No macOS permissions.
- No filesystem tools beyond temporary database files inside tests.
- No raw tool results, clipboard content, or file content.
- No unrestricted SQL execution API exposed outside the storage module.

## 2B-1 acceptance criteria

Increment 2B-1 is complete only when:

- `rusqlite` is added with the documented exact version and features or a new decision records the deviation.
- `tempfile` is added only as a dev dependency.
- Storage modules compile and are wired through `src-tauri/src/lib.rs`.
- Opening an in-memory database applies and verifies foreign keys.
- Opening a temporary file database applies WAL where supported.
- Migrations create `schema_migrations` and `app_metadata`.
- Re-running migrations is idempotent.
- A deliberately failing migration rolls back cleanly.
- Typed errors are returned for invalid configuration and migration failures.
- Production paths contain no `unwrap`, `expect`, `panic!`, `todo!`, `unimplemented!`, or `unreachable!`.
- Existing UI and `get_app_info` IPC behavior remain unchanged.

## Verification commands for 2B-1

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check

cargo clippy --manifest-path src-tauri/Cargo.toml \
  --all-targets \
  --all-features \
  --locked \
  -- -D warnings

cargo test --manifest-path src-tauri/Cargo.toml \
  --all-targets \
  --locked

npm run typecheck
npm run build
```

Because 2B-1 introduces native database dependencies, it must be verified on the target Mac before the increment is marked complete.

## Files changed in 2B-0

```text
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
TROUBLESHOOTING_LOG.md
docs/increments/02a-core-interfaces.md
docs/increments/02b-0-sqlite-storage-decision.md
```

## Verification for 2B-0

This increment changes documentation only. It does not change TypeScript, Rust, Tauri configuration, dependencies, or runtime behavior.

Required local verification for this documentation-only increment:

```bash
npx prettier --check .
npm run typecheck
npm run build
```

Recommended target-Mac formatting check before commit:

```bash
npm run format:check
```

Optional native smoke test:

```bash
npm run tauri -- dev
```
