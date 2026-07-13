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

- The next increment is interfaces and mocks only.
- A broader request must be decomposed before implementation.

## Open decisions

| ID    | Topic                                                            | Required before            |
| ----- | ---------------------------------------------------------------- | -------------------------- |
| O-001 | SQLite crate and encryption approach                             | Increment 2B               |
| O-002 | Workspace split between one Tauri crate and multiple Rust crates | Increment 2A or 2B         |
| O-003 | macOS minimum deployment target confirmation on target Mac       | Native release preparation |
| O-004 | State-management library versus React reducer/context            | Increment 2D               |
| O-005 | Menu-bar icon assets and close behavior                          | Increment 2C               |
