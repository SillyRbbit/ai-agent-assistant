# Next steps

Last updated: 2026-07-13

This file is the ordered implementation queue. Work only on the first item marked **Ready** unless a blocking defect requires a smaller troubleshooting increment.

## Phase 2 completion strategy

Phase 2 is completed through small, independently verifiable increments. Increment labels after 2C were re-sequenced when the project owner explicitly prioritized storage startup before menu-bar behavior.

### Increment 2A — core interfaces and deterministic mocks

Status: **Verified complete**

Record: `docs/increments/02a-core-interfaces.md`.

### Increment 2B-0 — SQLite storage dependency and design decision

Status: **Complete**

Record: `docs/increments/02b-0-sqlite-storage-decision.md`.

### Increment 2B-1 — SQLite dependency and migration skeleton

Status: **Verified complete**

Verified outcome:

- `rusqlite 0.37.0` with bundled SQLCipher and vendored OpenSSL,
- `libsqlite3-sys 0.35.0`,
- test-only `tempfile 3.23.0`,
- private raw connection ownership,
- verified foreign keys, busy timeout, and WAL,
- immutable checksummed migrations,
- `schema_migrations` and `app_metadata` only,
- target-Mac Rust checks, frontend checks, and native launch passed.

Records:

- `docs/increments/02b-1-sqlite-migration-skeleton.md`
- `docs/increments/02b-1a-rust-190-compatibility-repair.md`

### Increment 2C — storage startup integration

Status: **Implementation complete; target-Mac verification pending**

Goal: initialize the SQLite foundation during application startup through one safe Rust abstraction while using only typed `app_metadata` and persisting no user data.

Implemented:

- managed `Storage` abstraction,
- typed `app_initialized` metadata,
- fail-closed metadata validation,
- startup migration and marker bootstrap,
- debug-only file-backed development database,
- release-mode in-memory database,
- Tauri setup-hook integration,
- focused unit and integration tests,
- no new Tauri command, UI, permission, or capability.

Remaining completion gate:

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
npm run typecheck
npm run build
npm run tauri -- dev
```

Launch twice and confirm the second launch applies no migrations and recognizes prior initialization.

Record: `docs/increments/02c-storage-startup.md`.

Plan: `docs/plans/02c-storage-startup.md`.

### Increment 2D — macOS menu-bar and window lifecycle

Status: **Blocked by Increment 2C verification**

Goal: add an unprivileged menu-bar entry and explicit window lifecycle.

Planned work:

- Show or focus the main window.
- Route a new-request action.
- Route a task-placeholder action.
- Quit the application.
- Define behavior when the main window closes.
- Add unit-testable routing around Tauri-specific code.

Do not add a global shortcut in the same increment.

### Increment 2E — React application shell

Status: **Blocked by Increment 2D**

Goal: implement the basic navigation and page shell.

Planned work:

- Sidebar entries for Conversations, Tasks, Memory, Activity, Integrations, Permissions, and Settings.
- Conversation pane and input composer.
- Empty, loading, and error states.
- Settings page shell.
- Permission Center shell with no OS permission requests.
- Component and state-reducer tests.

### Increment 2F — mocked agent streaming and activity

Status: **Blocked by Increment 2E**

Goal: prove the end-to-end local run-event model using a deterministic mock provider.

Planned work:

- Submit a user message.
- Stream mock assistant text.
- Stop a run.
- Render a tool activity card.
- Render a trusted mock approval dialog.
- Approve, reject, and edit mock proposals.
- Record in-memory audit events.

No network access or production API key is allowed.

### Increment 2G — Phase 2 integration and native verification

Status: **Blocked by Increments 2C through 2F**

Goal: integrate the Phase 2 shell and verify it on macOS.

Planned work:

- Connect the menu-bar entry to the window shell.
- Persist only the minimal local application state separately approved for Phase 2.
- Run the complete verification suite.
- Build the native application without bundling.
- Perform a manual macOS smoke test.
- Update Phase 2 acceptance documentation.

## Later phases

- Phase 3: complete conversation UI and mocked agent loop.
- Phase 4: authenticated gateway and OpenAI Responses function calling.
- Phase 5: policy engine, approvals, and durable audit logging.
- Phase 6: restricted macOS tools.
- Phase 7: permissions and onboarding.
- Phase 8: memory and tasks.
- Phase 9: adversarial security testing.
- Phase 10: signing, notarization, packaging, and release preparation.

## Explicitly not next

Do not add any of the following before Increment 2C verification completes:

- Product-data persistence.
- SQLCipher production key handling.
- Accessibility.
- ScreenCaptureKit.
- Apple Events.
- Shell execution.
- File write or delete tools.
- Production gateway calls.
- API-key storage.
- OAuth.
- Calendar, contacts, reminders, or notification permissions.
- Global shortcut.
