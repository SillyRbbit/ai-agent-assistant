# Working-session handoff

Last updated: 2026-07-13

## Current phase

Phase 2 — repository setup, working Tauri shell, and trusted local-core foundation.

Last completed increment: **Increment 2D — macOS menu-bar and window lifecycle**.

Status: **Verified complete on the target Mac**.

Next ready increment: **Increment 2E — React application shell**.

## Increment 2D verification result

The project owner completed the required target-Mac verification and manual smoke test.

### Commands confirmed passed

```text
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
npm run typecheck
npm run build
npm run tauri -- dev
```

Results:

- Rust formatting passed.
- Clippy passed for all targets and features with warnings denied.
- All Rust unit and integration tests passed.
- Strict TypeScript typechecking passed.
- The Vite production build passed.
- The native Tauri application launched successfully.

### Manual macOS smoke test confirmed passed

- The macOS menu-bar icon appeared.
- **Open AI Agent Assistant** showed and focused the main window.
- **New Request** showed and focused the main window without an error.
- **Tasks (Coming Soon)** showed and focused the main window without an error.
- Closing the main window hid it without terminating the application.
- A menu-bar action reopened the hidden window.
- Clicking the Dock icon reopened the hidden window.
- **Quit AI Agent Assistant** terminated the process.
- Storage startup remained idempotent.
- The existing `get_app_info` IPC behavior remained functional.
- The existing React proof-of-connection UI remained unchanged.
- No macOS permission prompt appeared.

Increment 2D's completion gate is satisfied.

## What works now

- Tauri 2 launches on the Apple Silicon target Mac.
- React renders inside the existing native main window.
- `get_app_info` remains the only custom Tauri command.
- SQLCipher-capable SQLite startup and immutable migrations remain operational.
- The development database recognizes the persisted bootstrap marker and does not rerun applied migrations.
- The macOS menu-bar entry exposes a fixed menu contract:

  ```text
  Open AI Agent Assistant
  New Request
  Tasks (Coming Soon)
  Quit AI Agent Assistant
  ```

- Closing only the `main` window hides it while keeping the process alive.
- Menu-bar and Dock reopen behavior restore the hidden main window.
- New Request and Tasks emit the closed `assistant-menu-route` event only after the main window is activated.
- The current React UI does not consume those route events yet; visible page routing belongs to Increment 2E.

## Security boundaries preserved

- No new Tauri IPC command or WebView-invokable capability.
- No CSP or capability-file change.
- No unrestricted shell or generic command execution.
- No Accessibility, ScreenCaptureKit, Apple Events, microphone, or broad filesystem access.
- No API key, OAuth, model networking, or backend gateway integration.
- No database schema expansion or product-data persistence.
- Route payloads remain a closed enum, and unknown menu identifiers are ignored.
- Production Rust paths continue to use typed errors without `unwrap`, `expect`, `panic!`, `todo!`, `unimplemented!`, or `unreachable!`.

## Git and repository state

Expected branch: `main`.

This documentation-closing session did not have direct access to the project owner's local Git working tree and did not create a commit or push. After applying the documentation sync, run:

```bash
git status --short --branch
git diff --check
git diff --stat
```

The Increment 2D source and documentation should be reviewed and committed as one verified checkpoint before Increment 2E begins. Do not include local databases, build output, credentials, certificates, environment files, logs, or Downloads backups.

## Files belonging to the Increment 2D checkpoint

```text
package.json
src-tauri/Cargo.toml
src-tauri/src/lib.rs
src-tauri/src/error.rs
src-tauri/src/menu_bar/mod.rs
src-tauri/src/menu_bar/action.rs
src-tauri/src/menu_bar/controller.rs
src-tauri/src/menu_bar/tauri_adapter.rs
src-tauri/tests/menu_bar_routing.rs
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
TROUBLESHOOTING_LOG.md
docs/increments/02c-storage-startup.md
docs/increments/02d-menu-bar-window-lifecycle.md
docs/plans/02c-storage-startup.md
docs/plans/02d-menu-bar-window-lifecycle.md
```

## Partial, broken, or unknown

- No known Increment 2D defect remains.
- New Request and Tasks do not change React pages yet by design.
- The exact local Git status must be confirmed on the target Mac.
- Open decision O-004, React reducer/context versus a state-management dependency, must be resolved during Increment 2E. The current recommendation is React reducer plus context unless the implementation demonstrates a clear need for another dependency.

## Next ready work

**Increment 2E — React application shell** is Ready.

Its scope is limited to:

- platform-neutral in-memory React application state,
- sidebar navigation,
- conversation workspace and non-functional composer shell,
- page shells for Tasks, Memory, Activity, Integrations, Permissions, and Settings,
- empty, loading, and error presentation states,
- safe handling of the existing `assistant-menu-route` values,
- preservation of `get_app_info` diagnostics,
- focused reducer, route-event, and component tests.

It must not add mocked streaming, tool activity cards, approvals, persistence, SQLite migrations, new Tauri commands, permissions, global shortcuts, API keys, model networking, OAuth, shell access, or privileged macOS integration.

## Exact resume prompt

```text
Use $verified-increment.

Implement only Phase 2 Increment 2E from NEXT_STEPS.md for AI Agent Assistant.

Goal:
Replace the current proof-of-connection screen with the React application shell and consume the existing closed menu-route event, while preserving all verified Rust, SQLite, menu-bar, window-lifecycle, IPC, CSP, capability, and permission behavior.

Before changing files:
1. Read AGENTS.md, HANDOFF.md, PROJECT_STATUS.md, NEXT_STEPS.md, DECISIONS.md, TROUBLESHOOTING_LOG.md, SECURITY.md, CODE_REVIEW.md, docs/increments/02d-menu-bar-window-lifecycle.md, and docs/plans/02d-menu-bar-window-lifecycle.md.
2. Inspect the current branch and working tree.
3. Confirm Increment 2D is committed and the working tree is clean.
4. Confirm Node.js, npm, Rust, Cargo, rustfmt, and Clippy versions.
5. Run the smallest baseline checks needed to prove the verified app still builds.
6. State the exact files to create or change before editing.

Implementation scope:
- Resolve O-004; prefer React reducer plus context unless a new dependency is clearly necessary and documented.
- Add sidebar routes for Conversations, Tasks, Memory, Activity, Integrations, Permissions, and Settings.
- Add the conversation workspace and a non-functional input composer shell.
- Add empty, loading, and error presentation states.
- Add a Settings page shell.
- Add a Permission Center shell that requests no OS permission.
- Listen for assistant-menu-route and accept only new_request and tasks_placeholder.
- Route new_request to the conversation workspace.
- Route tasks_placeholder to the Tasks page.
- Safely ignore invalid or unknown event payloads.
- Preserve the typed get_app_info wrapper and expose the Rust connection state in an appropriate diagnostics or Settings area.
- Keep all application state local and in memory.
- Add focused reducer, event-routing, accessibility, and component tests.

Constraints:
- Preserve the existing Rust menu-bar, window lifecycle, storage startup, command surface, CSP, and Tauri capabilities.
- Do not add mocked streaming, tool activity cards, or approval behavior yet.
- Do not persist conversations, tasks, navigation state, settings, or permissions.
- Do not add or alter SQLite migrations.
- Do not add Tauri commands, global shortcuts, API keys, OpenAI access, gateway calls, OAuth, shell execution, broad filesystem access, Accessibility, ScreenCaptureKit, Apple Events, or OS permission requests.
- Keep strict TypeScript enabled and validate external event payloads from unknown.
- Do not commit unless explicitly asked.

Verification:
npm run format:check
npm run lint
npm run typecheck
npm run test:unit
npm run test:integration
npm run build
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
npm run tauri -- dev

Manual verification:
- The application shell renders at the current minimum window size.
- Every sidebar item opens the correct page shell.
- The conversation workspace and composer appear.
- Settings and Permission Center open without an OS prompt.
- New Request from the menu bar opens Conversations.
- Tasks (Coming Soon) from the menu bar opens Tasks.
- Closing, menu-bar reopening, Dock reopening, and quitting still work.
- Rust diagnostics remain available.
- SQLite startup remains idempotent.

At completion, review the complete diff, update all affected project-memory and increment documents with actual results, and stop after Increment 2E.
```
