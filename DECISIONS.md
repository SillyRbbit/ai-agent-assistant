# Decision log

Record durable technical and workflow decisions here. Do not delete prior decisions; mark them superseded and add the replacement decision.

## D-001 — Use Tauri 2 with React, TypeScript, Vite, and Rust

Date: 2026-06-18
Status: Accepted

Decision: use Tauri 2 for the native desktop shell, React and strict TypeScript for the WebView, Vite for frontend development, and Rust for the trusted local core.

Rationale: this provides a native-feeling macOS application while preserving substantial reuse for Windows and Linux and keeping action authorization outside the WebView.

Consequences:

- IPC must remain narrow and typed.
- Platform-specific code must stay behind adapters.
- Tauri capabilities and CSP are security-critical configuration.

## D-002 — Support Node.js 26.3.0 and npm 11.16.0

Date: 2026-06-18
Status: Accepted

Decision: make Node.js 26.3.0 and npm 11.16.0 the preferred local JavaScript toolchain while also accepting supported Node.js 22 and 24 environments.

Rationale: the target workstation already uses Node.js 26 and npm 11. Strict engine checks should validate rather than block that environment.

Consequences:

- `engine-strict=true` remains enabled.
- Node.js 23 and 25 remain excluded.
- Dependency upgrades must continue to be tested against the declared engine range.

## D-003 — Pin Rust 1.90.0 through rust-toolchain.toml

Date: 2026-06-18
Status: Accepted

Decision: use Rust 1.90.0 with `clippy` and `rustfmt` components.

Rationale: a repository-pinned toolchain makes local and CI behavior reproducible.

Consequences:

- Developers using Homebrew `rustup` must ensure its `bin` directory is on `PATH`.
- Rust upgrades require a dedicated verified increment.

## D-004 — Keep the model outside the authorization boundary

Date: 2026-06-18
Status: Accepted

Decision: model output may propose typed calls but can never authorize or directly execute local actions.

Rationale: prompt instructions alone are not a security boundary.

Consequences:

- Tool validation, policy, approval, execution, and audit live in deterministic Rust code.
- Unknown tools and invalid arguments fail closed.
- No generic action or unrestricted shell tool will be registered.

## D-005 — Use repository files for durable project memory

Date: 2026-06-18
Status: Accepted

Decision: preserve project state across assistant sessions with `AGENTS.md`, `HANDOFF.md`, `PROJECT_STATUS.md`, `NEXT_STEPS.md`, `DECISIONS.md`, `CHANGELOG.md`, and `TROUBLESHOOTING_LOG.md`.

Rationale: chat history is not a reliable source of truth for long-running engineering work.

Consequences:

- Every meaningful session ends with a handoff update.
- Documentation drift is treated as an incomplete task.
- The exact resume prompt is stored in the repository.

## D-006 — Store reusable assistant skills under .agents/skills

Date: 2026-06-18
Status: Accepted

Decision: repository-scoped agent skills live under `.agents/skills/<skill-name>/SKILL.md` and include `name` and `description` metadata.

Rationale: this is the current repository-level skill discovery convention and keeps recurring workflows versioned with the codebase.

Consequences:

- Each skill remains focused on one repeatable job.
- Long-lived rules belong in `AGENTS.md`; detailed workflows belong in skills and workflow documents.
- Prompt files remain available for assistants that do not discover skills automatically.

## D-007 — Complete Phase 2 through narrow verified increments

Date: 2026-06-18
Status: Accepted

Decision: separate core interfaces, SQLite, menu-bar behavior, React shell, mocked streaming, and final integration into distinct increments.

Rationale: smaller changes make compile failures, security regressions, and platform-specific defects easier to diagnose.

Consequences:

- Work proceeds only on the first ready increment in `NEXT_STEPS.md` unless a blocker requires a smaller troubleshooting increment.
- Broader requests must be decomposed before implementation.

## D-008 — Keep Increment 2A interfaces inside the existing Tauri Rust crate

Date: 2026-07-09
Status: Accepted

Decision: implement the initial platform-neutral core interfaces as Rust modules under `src-tauri/src/` rather than splitting the repository into multiple Rust crates during Increment 2A.

Rationale: the increment is architecture-only and should prove contracts, deterministic mocks, and tests without introducing workspace movement, dependency changes, or additional build complexity.

Consequences:

- The current `get_app_info` IPC command and Tauri crate layout remain unchanged.
- Later increments may still split portable domain modules into workspace crates after the interfaces stabilize.
- Open decision O-002 remains partially answered for 2A but should be revisited before larger persistence or platform-adapter work.

## D-009 — Use rusqlite with bundled SQLCipher for the first storage implementation

Date: 2026-07-13
Status: Superseded by D-011

Decision: when SQLite is implemented in Increment 2B-1, use `rusqlite` with an exact version and the bundled SQLCipher feature set unless target-Mac verification exposes a blocker:

```toml
rusqlite = { version = "=0.40.1", features = ["bundled-sqlcipher-vendored-openssl"] }
```

Use `tempfile = "=3.23.0"` as a dev dependency for isolated temporary-database tests.

Rationale: `rusqlite` gives the trusted local Rust core explicit synchronous connection, transaction, prepared-statement, and PRAGMA control without adding an async runtime or compile-time database macro workflow. SQLCipher support should be present from the first database implementation to avoid migrating user data from plaintext storage later.

Consequences:

- Increment 2B-1 must verify the native SQLCipher build on the target Mac.
- Database keys must remain outside SQLite and outside repository configuration.
- Production Keychain integration remains a later increment behind a secret-store boundary.
- If SQLCipher blocks local builds, record a superseding decision before falling back to non-encrypted SQLite.
- Open decision O-001 is resolved by this decision.

## D-010 — Keep the SQLite connection private and migrations immutable

Date: 2026-07-13
Status: Accepted

Decision: `DatabaseConnection` owns the raw `rusqlite::Connection` privately and exposes only narrow connection-settings, migration, and applied-migration methods. Migration definitions are static source code with positive increasing versions, fixed names, fixed checksums, and one immediate transaction per pending migration. Migration 1 bootstraps `schema_migrations`; migration 2 creates `app_metadata`.

Rationale: a generic SQL execution surface would bypass future repository, policy, audit, and redaction boundaries. Immutable migration metadata detects accidental or malicious history drift, while per-migration transactions provide deterministic rollback behavior.

Consequences:

- Callers cannot submit arbitrary SQL through the public storage API.
- Unknown applied migration versions fail closed.
- A changed migration name or checksum fails closed.
- Released migrations must never be edited; corrections require a new version.
- SQLCipher support is compiled in, but key application remains deferred until a dedicated secret-store boundary exists.
- No product data may use a file-backed database before key management is implemented and reviewed.

## D-011 — Pin rusqlite 0.37.0 for Rust 1.90 compatibility

Date: 2026-07-13
Status: Accepted

Decision: retain bundled SQLCipher and vendored OpenSSL support while pinning the first storage implementation to:

```toml
rusqlite = { version = "=0.37.0", features = ["bundled-sqlcipher-vendored-openssl"] }
```

This resolves to `libsqlite3-sys 0.35.0` on the verified target-Mac lockfile.

Rationale: the originally selected `rusqlite 0.40.1` resolved to `libsqlite3-sys 0.38.1`, whose build script used a standard-library feature unavailable on the repository's pinned Rust 1.90.0 toolchain. The 0.37.0 dependency line preserves the required encryption build features and passed target-Mac formatting, Clippy, tests, and native launch.

Consequences:

- D-009's exact 0.40.1 version is superseded, but its choice of `rusqlite`, bundled SQLCipher, vendored OpenSSL, and external key management remains in force.
- Rust 1.90.0 remains pinned.
- `Cargo.lock` must retain `rusqlite 0.37.0` and `libsqlite3-sys 0.35.0` until a dedicated dependency/toolchain increment changes them.
- Dependency upgrades require target-Mac native verification.

## D-012 — Bootstrap storage at Tauri startup without persisting user data

Date: 2026-07-13
Status: Accepted

Decision: initialize the storage foundation from Tauri's setup hook and register one managed `Storage` value. Debug builds use an application-local file-backed database containing only migration history and `app_initialized=true`; release builds use an in-memory database until reviewed Keychain-backed key management exists.

Application metadata is a closed typed contract rather than arbitrary key/value strings. The only current key is `AppInitialized`, and the only current value type is Boolean.

Rationale: startup integration proves the database lifecycle, migration ordering, state ownership, and typed read/write boundary before product repositories are introduced. Keeping release storage ephemeral prevents accidental plaintext user-data persistence before an encryption-key boundary exists.

Consequences:

- The raw `rusqlite::Connection` remains private to the storage module.
- Tauri startup fails with a typed error if storage cannot initialize or managed state is already registered.
- Corrupt metadata values and negative timestamps fail closed.
- Startup logs contain no database path or metadata value.
- No conversation, task, memory, audit, approval, tool-call, credential, or personal data may be stored yet.
- The previously planned menu-bar increment is re-labeled Increment 2D; React shell, mocked streaming, and integration increments move to 2E, 2F, and 2G.

## D-013 — Use a fixed macOS menu-bar contract and hide only the main window

Date: 2026-07-13
Status: Accepted

Decision: enable Tauri's built-in `tray-icon` feature only for the macOS target and create one menu-bar entry with four fixed actions: open the main window, route a new request, route a tasks placeholder, and quit. New-request and tasks actions emit the closed-enum `assistant-menu-route` event only after the existing `main` window is shown and focused. Unknown menu identifiers are ignored.

Closing the `main` window prevents destruction and hides that window. Future non-main windows retain normal close behavior. A macOS `RunEvent::Reopen` restores the main window only when no application window is visible. The app keeps its regular activation policy and Dock icon in this increment.

Rationale: the menu bar needs a deterministic, least-privilege lifecycle contract before the React shell and global shortcut are introduced. Fixed IDs and closed route values prevent arbitrary model or content-driven actions. Keeping the Dock avoids an invisible-app failure mode while close-to-hide is first validated.

Consequences:

- `get_app_info` remains the only custom Tauri command.
- The backend may emit only `NewRequest` or `TasksPlaceholder` route values.
- React does not consume those route events until Increment 2E.
- Dedicated production tray artwork and accessory-only activation remain deferred.
- Increment 2D requires native macOS verification of tray, close, menu/Dock reopen, and quit behavior.
- Open decision O-005 is resolved for the MVP scaffold; artwork may be revisited later.

## Open decisions

| ID    | Topic                                                            | Required before                     |
| ----- | ---------------------------------------------------------------- | ----------------------------------- |
| O-002 | Workspace split between one Tauri crate and multiple Rust crates | Revisit before later modularization |
| O-003 | macOS minimum deployment target confirmation on target Mac       | Native release preparation          |
| O-004 | State-management library versus React reducer/context            | Increment 2E                        |
