# Next steps

Last updated: 2026-07-13

This file is the ordered implementation queue. Work only on the first item marked **Ready** unless a blocking defect requires a smaller troubleshooting increment.

## Phase 2 completion strategy

Phase 2 is completed through small, independently verifiable increments rather than one broad implementation.

### Increment 2A — core interfaces and deterministic mocks

Status: **Verified complete**

Record: `docs/increments/02a-core-interfaces.md`.

### Increment 2B-0 — SQLite storage dependency and design decision

Status: **Complete**

Record: `docs/increments/02b-0-sqlite-storage-decision.md`.

### Increment 2B-1 — SQLite dependency and migration skeleton

Status: **Implementation complete; target-Mac verification pending**

Implemented:

- exact `rusqlite = "=0.40.1"` declaration with `bundled-sqlcipher-vendored-openssl`,
- `tempfile = "=3.23.0"` as a dev dependency,
- typed storage configuration and errors,
- in-memory and file-backed connections,
- verified foreign keys, busy timeout, and file-backed WAL,
- versioned checksummed migrations,
- `schema_migrations` and `app_metadata` only,
- migration idempotency, rollback, ordering, connection, and integration tests,
- private raw SQLite connection with no generic SQL API.

Remaining completion gate:

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

The first unlocked `cargo check` updates the lockfile once. Review that diff before running the locked checks.

Record: `docs/increments/02b-1-sqlite-migration-skeleton.md`.

Plan: `docs/plans/02b-1-sqlite-migration-skeleton.md`.

### Increment 2C — macOS menu-bar and window lifecycle

Status: **Blocked by 2B-1 target-Mac verification**

Goal: add a menu-bar entry without privileged APIs.

Planned work:

- Show or focus the main window.
- Create a new request action.
- Open the task placeholder.
- Quit the application.
- Define behavior when the main window closes.
- Add unit-testable command routing around Tauri-specific code.

Do not add a global shortcut in the same increment.

### Increment 2D — React application shell

Status: **Blocked by 2C or explicit reprioritization**

Goal: implement the basic navigation and page shell.

Planned work:

- Sidebar entries for Conversations, Tasks, Memory, Activity, Integrations, Permissions, and Settings.
- Conversation pane and input composer.
- Empty/loading/error states.
- Settings page shell.
- Permission Center shell with no OS permission requests.
- Component and state-reducer tests.

### Increment 2E — mocked agent streaming and activity

Status: **Blocked by 2D**

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

### Increment 2F — Phase 2 integration and native verification

Status: **Blocked by 2B through 2E**

Goal: integrate the Phase 2 shell and verify it on macOS.

Planned work:

- Connect the menu-bar entry to the window shell.
- Persist only the minimal local application state approved for Phase 2.
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

Do not add any of the following while 2B-1 verification is pending:

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
- Calendar, contacts, reminders, or notifications permissions.
