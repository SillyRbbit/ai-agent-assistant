# Next steps

Last updated: 2026-07-09

## Completed

### Phase 2 Increment 2A — platform-neutral core interfaces

Status: Verified complete locally.

Implemented deterministic Rust contracts and mocks for:

- AgentProvider
- ToolRegistry
- PolicyEngine
- ApprovalManager
- AuditLogger
- MemoryStore
- PlatformAdapter

Verification reported passing:

- `cargo fmt --manifest-path src-tauri/Cargo.toml -- --check`
- `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings`
- `cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked`
- `npm run typecheck`
- `npm run build`
- `npm run tauri -- dev`

## Next ready increment

### Phase 2 Increment 2B-0 — SQLite dependency and storage-design decision

Goal: Decide and document the storage stack before implementing persistence.

This should answer:

- Which Rust SQLite crate will be used.
- Whether encryption is included in the first implementation or staged behind a trait.
- How migrations will be represented and tested.
- How repository interfaces will map to storage boundaries.
- How the project will avoid storing secrets or raw sensitive tool outputs.
- Which checks are required before storage code is accepted.

Allowed changes:

- Documentation updates.
- Decision record updates.
- Optional dependency research notes.
- Optional minimal crate-selection proof if explicitly approved before coding.

Not allowed in this increment unless explicitly re-scoped:

- SQLite schema implementation.
- Database connection code.
- New Tauri commands.
- Model networking.
- API keys.
- OAuth.
- macOS permissions.
- File, calendar, contact, reminder, notification, or clipboard tools.
- Shell execution.

Expected files:

```text
DECISIONS.md
docs/increments/02b-0-sqlite-storage-decision.md
PROJECT_STATUS.md
HANDOFF.md
NEXT_STEPS.md
CHANGELOG.md
```

## Later increments

### Phase 2 Increment 2B-1 — SQLite foundation

Add the first persistence crate, migration runner, and database test harness after Increment 2B-0 is accepted.

### Phase 2 Increment 2C — mock conversation runtime

Wire the deterministic mocked AgentProvider into Rust application state without real model networking.

### Phase 2 Increment 2D — shell UI expansion

Add sidebar, conversation view, composer, mock streaming response, tool activity card, approval dialog, Settings page, and Permission Center shell.

### Phase 2 Increment 2E — macOS menu-bar entry

Add menu-bar behavior without adding privileged permissions.
