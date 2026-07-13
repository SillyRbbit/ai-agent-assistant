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
Status: Accepted

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

## Open decisions

| ID    | Topic                                                            | Required before                     |
| ----- | ---------------------------------------------------------------- | ----------------------------------- |
| O-002 | Workspace split between one Tauri crate and multiple Rust crates | Revisit before later modularization |
| O-003 | macOS minimum deployment target confirmation on target Mac       | Native release preparation          |
| O-004 | State-management library versus React reducer/context            | Increment 2D                        |
| O-005 | Menu-bar icon assets and close behavior                          | Increment 2C                        |
