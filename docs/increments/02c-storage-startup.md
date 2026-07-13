# Increment 2C — storage startup integration

Last updated: 2026-07-13

Status: **Verified complete on target Mac**

## Goal

Wire the existing SQLite foundation into Tauri application startup through one safe Rust abstraction. Use only the existing `schema_migrations` and `app_metadata` tables, and persist no conversations, messages, tasks, memories, audit records, tool activity, credentials, or other user data.

## Baseline

Increment 2B-1 and its Rust 1.90 compatibility repair were verified on the target Mac before this increment began:

- `rusqlite 0.37.0` resolves to `libsqlite3-sys 0.35.0`.
- Rust formatting, Clippy with warnings denied, and Rust tests passed.
- TypeScript typechecking and the Vite production build passed.
- The existing Tauri application launched with unchanged UI and IPC behavior.

## Implemented scope

### Managed storage abstraction

`Storage` owns the private `DatabaseConnection` behind a standard-library mutex. Callers can use only typed methods for:

- reading a known application metadata key,
- writing a typed value for a known application metadata key,
- reading verified connection settings,
- reading the startup migration report,
- listing applied migrations.

The raw `rusqlite::Connection` remains private to the storage module. No arbitrary SQL API is exposed.

### Typed bootstrap metadata

The only runtime metadata contract is:

```text
AppMetadataKey::AppInitialized
AppMetadataValue::Boolean(bool)
```

The database representation is validated when read. Unknown values and negative timestamps fail closed with typed `StorageError` variants. The first successful initialization writes `app_initialized=true`; later launches read the existing marker without rewriting it.

### Tauri startup integration

The Tauri builder now uses a setup hook that:

1. selects a bounded storage configuration,
2. opens the database,
3. verifies connection settings,
4. applies pending migrations,
5. reads or creates the bootstrap marker,
6. registers `Storage` as managed Rust state,
7. emits a non-sensitive initialization summary.

The existing `get_app_info` command remains the only custom Tauri command.

### Storage modes

- **Debug builds:** use a file-backed development database named `assistant-development.sqlite3` under Tauri's application-local data directory.
- **Release builds:** use an in-memory database until a later, separately reviewed Keychain-backed encryption-key increment exists.

The development database contains only migration history and the harmless bootstrap marker. No product or personal data is written.

## Files changed

```text
src-tauri/src/error.rs
src-tauri/src/lib.rs
src-tauri/src/startup.rs
src-tauri/src/storage/connection.rs
src-tauri/src/storage/error.rs
src-tauri/src/storage/metadata.rs
src-tauri/src/storage/migrations.rs
src-tauri/src/storage/mod.rs
src-tauri/src/storage/store.rs
src-tauri/src/storage/timestamp.rs
src-tauri/tests/startup_storage_smoke.rs
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
TROUBLESHOOTING_LOG.md
docs/increments/02b-0-sqlite-storage-decision.md
docs/increments/02b-1-sqlite-migration-skeleton.md
docs/increments/02b-1a-rust-190-compatibility-repair.md
docs/increments/02c-storage-startup.md
docs/plans/02b-1-sqlite-migration-skeleton.md
docs/plans/02c-storage-startup.md
```

## Tests added

Focused Rust tests cover:

- first and repeated file-backed initialization,
- compile-time `Send + Sync` compatibility for Tauri managed state,
- migration application exactly once,
- typed metadata creation, update, and read,
- preservation of the original bootstrap marker on later starts,
- rejection of malformed stored metadata,
- rejection of negative metadata timestamps on write and read,
- development database path construction,
- non-sensitive startup summaries that omit the database path and metadata key,
- public storage API behavior across repeated file-backed startup.

Tests are deterministic and use temporary or in-memory databases. They do not use network access, OS permissions, calendar data, contacts, files, or other machine state.

## Security review

Preserved invariants:

- no new Tauri commands,
- no UI or IPC changes,
- no API keys or key configuration,
- no model or gateway networking,
- no OAuth,
- no Keychain integration,
- no Accessibility, ScreenCaptureKit, Apple Events, microphone, or shell access,
- no broad filesystem API,
- no user-data tables or repositories,
- no raw SQLite connection exposed outside the storage module,
- no database path or metadata value included in startup logging,
- typed errors instead of `unwrap`, `expect`, `panic!`, `todo!`, `unimplemented!`, or `unreachable!` in new production paths.

## Verification completed in the artifact workspace

```text
Baseline npm ci: passed
Baseline npm run typecheck: passed
Baseline npm run build: passed
Post-change npx prettier --check .: passed
Post-change npm run lint:frontend: passed
Post-change npm run typecheck: passed
Post-change npx vitest run: 2 passed, 0 failed
Post-change npm run build: passed
Post-change npm audit --audit-level=low: 0 vulnerabilities
Static prohibited-API and panic-style scan: passed
Git diff whitespace check: passed
Rust formatting, Clippy, tests, and native launch: not run because Cargo is unavailable on the artifact host
```

Target-Mac Rust and native checks passed on 2026-07-13.

## Required target-Mac verification

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml

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

Expected native behavior:

- the existing main window opens,
- the existing UI is unchanged,
- `Rust core connected` and `get_app_info` still work,
- no macOS permission prompt appears,
- startup succeeds on first and later launches,
- debug startup reports only storage mode and migration counts,
- release behavior remains ephemeral until key management is implemented.

## Completion gate

Increment 2C completion evidence:

- `cargo fmt --check` passed.
- Clippy passed with warnings denied.
- 41 Rust unit tests and 3 integration tests passed.
- TypeScript and Vite passed.
- The native app launched successfully.
- A later launch reported zero newly applied migrations, two existing migrations, and prior initialization.
