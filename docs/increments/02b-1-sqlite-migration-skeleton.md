# Increment 2B-1 — SQLite dependency and migration skeleton

Last updated: 2026-07-13

Status: **Implementation complete; target-Mac verification pending**

## Goal

Add a minimal, platform-neutral SQLite foundation to the trusted Rust core without persisting product data or changing the UI, IPC surface, permissions, or application capabilities.

## Implemented scope

### Dependencies

```toml
rusqlite = { version = "=0.40.1", features = ["bundled-sqlcipher-vendored-openssl"] }

[dev-dependencies]
tempfile = "=3.23.0"
```

The runtime dependency compiles SQLCipher support into the SQLite library. This increment does not create, accept, log, or store a production database key.

### Storage contracts

Added `src-tauri/src/storage/` with:

- `config.rs` — in-memory and file-backed database configuration with bounded busy timeout validation.
- `connection.rs` — connection ownership, PRAGMA application and verification, and narrow migration methods.
- `error.rs` — typed `StorageError` and `StorageResult`.
- `migrations.rs` — immutable migration definitions, metadata validation, transactional application, listing, and rollback handling.
- `mod.rs` — intentionally narrow public exports.

The raw `rusqlite::Connection` remains private to the storage module. No generic execute/query API is exposed.

### Connection behavior

Every opened connection:

1. validates the supplied configuration,
2. opens an in-memory or file-backed SQLite database,
3. applies a five-second default busy timeout bounded to thirty seconds,
4. enables `PRAGMA foreign_keys = ON`,
5. verifies foreign-key enforcement,
6. verifies the configured busy timeout,
7. enables and verifies WAL for file-backed databases,
8. reports typed errors instead of panicking.

In-memory databases retain SQLite's memory journal mode rather than pretending to support WAL.

### Initial migrations

Migration 1 creates and records the migration ledger:

```sql
CREATE TABLE schema_migrations (
  version INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  checksum TEXT NOT NULL,
  applied_at_ms INTEGER NOT NULL
) STRICT;
```

Migration 2 creates the only non-ledger table in this increment:

```sql
CREATE TABLE app_metadata (
  key TEXT PRIMARY KEY,
  value TEXT NOT NULL,
  updated_at_ms INTEGER NOT NULL
) STRICT;
```

Each migration has a positive monotonically increasing version, fixed name, fixed SHA-256 SQL checksum, and static SQL. Each pending migration runs in an immediate transaction. Unknown applied versions or changed metadata fail closed.

## Tests added

Unit tests cover:

- deterministic configuration defaults,
- invalid empty file paths,
- zero and excessive busy timeouts,
- verified foreign keys and memory journal mode,
- WAL for a temporary file database,
- deterministic migration listing,
- first application and idempotent reapplication,
- rollback of a deliberately failing migration,
- invalid migration order rejection.

An integration test exercises the public storage API by opening an in-memory connection and applying/listing the initial migrations.

## Security review

Preserved boundaries:

- No API key or database key storage.
- No Keychain access.
- No product-data persistence.
- No new Tauri command, capability, plugin, or CSP change.
- No React or UI change.
- No model or gateway networking.
- No OAuth.
- No macOS permission request.
- No Accessibility, screen capture, Apple Events, shell execution, file tool, calendar, contacts, reminders, notifications, or clipboard integration.
- No unrestricted SQL interface.
- No production-path `unwrap`, `expect`, `panic!`, `todo!`, `unimplemented!`, or `unreachable!`.

## Files created

```text
src-tauri/src/storage/mod.rs
src-tauri/src/storage/config.rs
src-tauri/src/storage/connection.rs
src-tauri/src/storage/error.rs
src-tauri/src/storage/migrations.rs
src-tauri/tests/storage_smoke.rs
docs/increments/02b-1-sqlite-migration-skeleton.md
docs/plans/02b-1-sqlite-migration-skeleton.md
```

## Files changed

```text
src-tauri/Cargo.toml
src-tauri/Cargo.lock
src-tauri/src/lib.rs
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
TROUBLESHOOTING_LOG.md
```

The target Mac must perform the one-time dependency resolution that updates `src-tauri/Cargo.lock` after applying the source overlay. The lockfile is not complete until that step succeeds and the resulting diff is reviewed.

## Verification actually performed in the artifact workspace

Passed before source editing:

```text
npm ci
npm run typecheck
npm run build
```

Passed against an isolated crate containing the exact storage modules and dependency feature set:

```text
cargo check --all-targets --all-features --locked
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-targets --locked
```

Storage harness result:

```text
8 passed; 0 failed
```

The harness caught one Clippy issue before packaging: `StorageError::MigrationRollback` initially held two large SQLite errors inline. Both are now boxed, and Clippy passes with warnings denied.

Not claimed as passed in the artifact workspace:

```text
Full Tauri-crate Clippy and tests
Native Apple Silicon SQLCipher build
Tauri application launch after the dependency change
```

The artifact host does not match the target Mac and could not complete the full Tauri native dependency build. Those checks are the remaining completion gate.

## Target-Mac verification

From the repository root:

```bash
export PATH="$(brew --prefix rustup)/bin:$PATH"
rustup default 1.90.0

cargo check --manifest-path src-tauri/Cargo.toml

git diff -- src-tauri/Cargo.lock

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
npm run tauri -- dev
```

Expected Rust test count after full integration is the existing Increment 2A suite plus the new storage tests. Do not update this record with a precise total until the target-Mac command reports it.

## Completion gate

Do not mark Increment 2B-1 complete and do not start Increment 2C until:

- the target-Mac lockfile is resolved and committed,
- all required locked Rust checks pass,
- TypeScript and Vite checks pass,
- the Tauri application launches with unchanged behavior.
