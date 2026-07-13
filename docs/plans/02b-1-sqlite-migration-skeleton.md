# Execution plan — Increment 2B-1 SQLite migration skeleton

Last updated: 2026-07-13

Status: **Active — implementation complete; target-Mac verification pending**

## Goal and user-visible outcome

Add the smallest trustworthy SQLite foundation to the Rust core without changing the current application UI or exposing persistence through Tauri IPC.

The only user-visible outcome should be that the existing application continues to launch and display the same `get_app_info` proof. The new behavior is internal and testable: the Rust crate can open a configured database connection, verify connection settings, and apply two versioned bootstrap migrations.

## Scope

- Add exact `rusqlite` and test-only `tempfile` dependencies selected in D-009.
- Add typed storage configuration, connection, error, and migration modules.
- Support in-memory and file-backed configurations.
- Apply a bounded busy timeout.
- Enable and verify foreign-key enforcement for every connection.
- Enable and verify WAL for file-backed databases.
- Add immutable, versioned, checksummed migrations.
- Create only `schema_migrations` and `app_metadata` as `STRICT` tables.
- Add focused unit and integration tests.
- Keep the raw SQLite connection private to the storage implementation.

## Explicit non-goals

- Product-data repositories or persistence.
- Database key generation or Keychain integration.
- A generic SQL execution interface.
- New Tauri commands or capabilities.
- React or CSS changes.
- Model networking, gateway calls, API keys, or OAuth.
- macOS permissions or native automation.
- Accessibility, ScreenCaptureKit, Apple Events, shell execution, file tools, calendar tools, contacts, reminders, notifications, or clipboard tools.

## Existing behavior and constraints

- Increment 2A core interfaces are verified on the target Mac.
- Increment 2B-0 selected `rusqlite = "=0.40.1"` with `bundled-sqlcipher-vendored-openssl` and `tempfile = "=3.23.0"` for tests.
- The current UI and typed `get_app_info` IPC command must remain unchanged.
- Production Rust paths must use typed errors and may not use `unwrap`, `expect`, `panic!`, `todo!`, `unimplemented!`, or `unreachable!`.
- No database key or sensitive product data may be stored in this increment.

## Files expected to change

```text
src-tauri/Cargo.toml
src-tauri/Cargo.lock
src-tauri/src/lib.rs
src-tauri/src/storage/mod.rs
src-tauri/src/storage/config.rs
src-tauri/src/storage/connection.rs
src-tauri/src/storage/error.rs
src-tauri/src/storage/migrations.rs
src-tauri/tests/storage_smoke.rs
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
TROUBLESHOOTING_LOG.md
docs/increments/02b-1-sqlite-migration-skeleton.md
docs/plans/02b-1-sqlite-migration-skeleton.md
```

The target Mac must regenerate or update `src-tauri/Cargo.lock` once after applying the source overlay because the artifact host cannot complete the native Tauri dependency resolution. The updated lockfile is part of the increment and must be committed.

## Ordered implementation steps

- [x] Verify the 2B-0 baseline TypeScript typecheck and Vite build.
- [x] Add exact runtime and test-only dependency declarations.
- [x] Add typed configuration and error contracts.
- [x] Add connection opening and verified PRAGMA configuration.
- [x] Add immutable migration definitions and transactional runner.
- [x] Add `schema_migrations` and `app_metadata` only.
- [x] Add unit and public-API integration tests.
- [x] Run a standalone storage harness through Rust check, Clippy with warnings denied, and tests.
- [x] Review the implementation for prohibited APIs and generic SQL exposure.
- [ ] Resolve the lockfile and run the required full-project Rust checks on the target Mac.
- [ ] Launch the Tauri application on the target Mac and confirm unchanged behavior.
- [ ] Mark the plan and increment complete only after those target-Mac checks pass.

## Security and privacy considerations

- SQLCipher support is compiled in, but no production key is created, accepted, logged, or stored.
- File-backed databases in this increment are test scaffolding only and must not contain product data.
- The `rusqlite::Connection` remains private; callers receive only narrow configuration, migration, and inspection methods.
- Migration values use prepared parameters. Migration SQL is static trusted source code, not runtime input.
- Unknown applied migration versions and changed names/checksums fail closed.
- Every pending migration runs in an immediate transaction and reports rollback failure separately.
- Foreign keys, journal mode, and busy timeout are verified after configuration.
- External SQLite extensions are not loaded.

## Tests and verification

Required target-Mac commands:

```bash
cargo check --manifest-path src-tauri/Cargo.toml

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

The initial unlocked `cargo check` is a one-time lockfile resolution step after applying the overlay. Review and commit the resulting `src-tauri/Cargo.lock`; all subsequent Rust verification uses `--locked`.

Focused tests cover:

- deterministic default configuration,
- invalid path and timeout rejection,
- in-memory connection settings,
- file-backed WAL mode,
- deterministic migration ordering,
- first migration application,
- idempotent reapplication,
- rollback after a deliberately failing migration,
- invalid migration order rejection,
- public storage API integration.

## Rollback or failure strategy

- Do not add UI or IPC integration to work around a storage failure.
- If SQLCipher fails to build on the target Mac, capture the exact native error and record it in `TROUBLESHOOTING_LOG.md`.
- Do not silently remove encryption features. A fallback requires a superseding decision in `DECISIONS.md`.
- If a migration test fails, keep product persistence disconnected and fix the migration runner before continuing.
- Revert this increment by restoring `Cargo.toml`, `Cargo.lock`, and `lib.rs`, then deleting `src-tauri/src/storage/` and `src-tauri/tests/storage_smoke.rs`.

## Exit criteria

- Exact dependencies and updated lockfile are present.
- Full Rust formatting, Clippy, and test commands pass on the target Mac.
- TypeScript typecheck and Vite build pass.
- The existing application launches with unchanged UI and IPC behavior.
- No prohibited API, key storage, product persistence, new Tauri command, capability, or permission is present.
- Handoff and status documents contain actual verification results.
