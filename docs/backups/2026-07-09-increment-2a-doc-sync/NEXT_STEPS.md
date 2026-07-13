# Next steps

Last updated: 2026-06-18

This file is the ordered implementation queue. Work only on the first item marked **Ready** unless a blocking defect requires a smaller troubleshooting increment.

## Phase 2 completion strategy

Phase 2 will be completed through small, independently verifiable increments rather than one broad implementation.

### Increment 2A — core interfaces and deterministic mocks

Status: **Ready**

Goal: establish platform-neutral contracts before adding persistence or UI behavior.

Create Rust interfaces for:

- `AgentProvider`
- `ToolRegistry`
- `PolicyEngine`
- `ApprovalManager`
- `AuditLogger`
- `MemoryStore`
- `PlatformAdapter`

Requirements:

- Keep interfaces in platform-neutral modules.
- Add small domain types needed to express requests and outcomes.
- Add deterministic in-memory or no-op mock implementations.
- Use typed error enums and `Result`; no `unwrap`, `expect`, or `panic!` in production paths.
- Do not add model networking, SQLite, macOS permissions, or new Tauri commands.
- Add focused unit tests for success and failure behavior.
- Preserve the current running UI.

Verification:

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
npm run typecheck
npm run build
```

Exit criteria:

- All seven interfaces exist and are documented.
- Every interface has at least one deterministic test implementation.
- Tests prove representative success and error paths.
- Existing `get_app_info` behavior still works.
- Handoff documents are updated.

### Increment 2B — SQLite foundation

Status: **Blocked by 2A and dependency decision**

Goal: add the local database foundation without yet persisting conversations or agent runs.

Planned work:

- Select and document the SQLite crate and encryption approach.
- Add a connection manager and typed storage error.
- Enable foreign keys and WAL where supported.
- Add versioned migration infrastructure.
- Create `schema_migrations` and a minimal application metadata table.
- Add temporary-database migration and rollback tests.

Do not add OAuth tokens, API keys, or raw sensitive content to the database.

### Increment 2C — macOS menu-bar and window lifecycle

Status: **Blocked by 2A**

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

Status: **Blocked by 2A**

Goal: implement the basic navigation and page shell.

Planned work:

- Sidebar entries for Conversations, Tasks, Memory, Activity, Integrations, Permissions, and Settings.
- Conversation pane and input composer.
- Empty/loading/error states.
- Settings page shell.
- Permission Center shell with no OS permission requests.
- Component and state-reducer tests.

### Increment 2E — mocked agent streaming and activity

Status: **Blocked by 2A and 2D**

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
- Phase 8: memory and task management.
- Phase 9: security and prompt-injection testing.
- Phase 10: signing, notarization, packaging, and release preparation.

## Explicitly not next

Do not add any of the following during the next increment:

- Accessibility
- ScreenCaptureKit
- Apple Events
- Shell execution
- File write or delete tools
- Production gateway calls
- API-key storage
- OAuth
- Calendar, contacts, reminders, or notifications permissions
