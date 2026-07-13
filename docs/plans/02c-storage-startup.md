# Execution plan — Increment 2C storage startup integration

Last updated: 2026-07-13

Status: **Complete**

## Goal and user-visible outcome

Initialize the existing SQLite foundation during Tauri startup through a safe managed Rust abstraction, using only the bootstrap `app_metadata` record and no user-data persistence.

The expected user-visible result is unchanged: the current native window opens and the existing `get_app_info` proof continues to work. No new UI, command, permission prompt, or setting is introduced.

## Scope

- Add a `Storage` abstraction that owns the private database connection.
- Add typed read/write behavior for `app_initialized` only.
- Initialize storage from the Tauri setup hook.
- Apply migrations before metadata access.
- Register initialized storage as managed Rust state.
- Use a file-backed database only in debug builds.
- Use in-memory storage in release builds until Keychain-backed key management exists.
- Add focused unit and integration tests.
- Update project-memory and increment documentation.

## Explicit non-goals

- Conversation, message, task, memory, approval, tool-call, or audit persistence.
- Keychain or production database-key handling.
- API keys, OAuth, model access, or gateway calls.
- New Tauri commands, capabilities, plugins, or IPC data.
- React, CSS, menu-bar, or window-lifecycle changes.
- Accessibility, ScreenCaptureKit, Apple Events, microphone, shell execution, or broad filesystem access.
- Calendar, contacts, reminders, notifications, or clipboard tools.

## Files expected to change

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
HANDOFF.md
PROJECT_STATUS.md
NEXT_STEPS.md
CHANGELOG.md
DECISIONS.md
PLANS.md
TROUBLESHOOTING_LOG.md
docs/increments/02b-0-sqlite-storage-decision.md
docs/increments/02b-1-sqlite-migration-skeleton.md
docs/increments/02b-1a-rust-190-compatibility-repair.md
docs/increments/02c-storage-startup.md
docs/plans/02b-1-sqlite-migration-skeleton.md
docs/plans/02c-storage-startup.md
```

## Ordered steps

- [x] Confirm the 2B-1A source baseline and dependency versions.
- [x] Run baseline TypeScript and Vite checks.
- [x] Add typed metadata contracts and validation.
- [x] Add the managed `Storage` abstraction.
- [x] Add Tauri startup configuration and state registration.
- [x] Preserve the existing Tauri command and UI.
- [x] Add focused storage-startup tests, including managed-state trait bounds and log-redaction assertions.
- [x] Scan new production code for prohibited panic-style calls and privileged APIs.
- [x] Update project documentation for the re-sequenced Increment 2C.
- [x] Run Rust formatting, Clippy, and all Rust tests on the target Mac.
- [x] Run post-change TypeScript and Vite checks on the target Mac.
- [x] Launch the application twice and confirm idempotent startup.
- [x] Review the complete diff and mark the increment verified.

## Security and privacy considerations

- The debug database may persist only migration metadata and `app_initialized=true`.
- Release builds remain in-memory because no reviewed database-key provider exists yet.
- Metadata keys and values are closed enums; arbitrary strings are not accepted through the public API.
- Corrupt or unexpected stored values fail closed.
- Startup logs omit database paths and metadata values.
- The raw SQLite connection remains storage-private.
- No new WebView authority is introduced.

## Verification commands

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
npm run typecheck
npm run build
npm run tauri -- dev
```

Launch the app twice. The first launch should apply migrations and create the marker; the second should recognize prior initialization and apply no migrations.

## Rollback strategy

- Remove the setup hook from `src-tauri/src/lib.rs`.
- Remove `src-tauri/src/startup.rs` and the metadata/store/timestamp modules.
- Restore storage module exports and error variants.
- Remove the startup integration test.
- Do not delete or alter released migration definitions.
- Delete the development database manually only when intentionally resetting local development state.

## Exit criteria

- All target-Mac Rust and frontend checks pass.
- The application launches twice with unchanged UI and IPC behavior.
- No permission prompt appears.
- Only the two bootstrap tables and one typed metadata marker are used.
- No prohibited dependency, API, command, capability, or data category is introduced.
- Documentation reflects actual verification results.
