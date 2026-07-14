# Execution plan — Increment 2D macOS menu-bar and window lifecycle

Last updated: 2026-07-13

Status: **Complete — verified on the target Mac**

## Goal and user-visible outcome

Add a macOS menu-bar entry that can show/focus the existing main window, route new-request and tasks-placeholder intents, and quit. Closing the main window hides it while the process remains active. A Dock/macOS reopen event restores it when no window is visible.

## Scope

- Enable Tauri tray support only on macOS.
- Add fixed action and route contracts.
- Add a deterministic dispatcher and lifecycle policy.
- Add a narrow Tauri adapter.
- Emit a closed route event for later React integration.
- Intercept only the main window's close request.
- Handle macOS reopen events.
- Add focused tests and documentation.

## Explicit non-goals

- Global shortcut.
- React shell or route listener.
- Accessory-only activation or Dock removal.
- Dedicated production tray artwork.
- New IPC commands or WebView capabilities.
- Database changes or user-data persistence.
- Model, credential, permission, or automation features.

## Files changed

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
HANDOFF.md
PROJECT_STATUS.md
NEXT_STEPS.md
CHANGELOG.md
DECISIONS.md
PLANS.md
TROUBLESHOOTING_LOG.md
docs/increments/02c-storage-startup.md
docs/increments/02d-menu-bar-window-lifecycle.md
docs/plans/02c-storage-startup.md
docs/plans/02d-menu-bar-window-lifecycle.md
```

## Ordered steps

- [x] Confirm the verified Increment 2C baseline from current project memory.
- [x] Read project-memory, security, review, and verified-increment guidance.
- [x] Run baseline frontend checks.
- [x] Confirm Tauri's tray and lifecycle APIs from official documentation/source.
- [x] Define pure menu action, route, dispatch, close, and reopen contracts.
- [x] Add the macOS-only Tauri adapter.
- [x] Preserve setup, storage, command list, UI, CSP, and capabilities.
- [x] Add Rust unit and integration tests.
- [x] Update project-memory and increment documentation.
- [x] Run Prettier, ESLint, TypeScript, Vitest, Vite, audit, and static security checks.
- [x] Run rustfmt, Clippy, and all Rust tests on the target Mac.
- [x] Run target-Mac TypeScript and Vite checks.
- [x] Perform the full manual menu-bar/window lifecycle smoke test.
- [x] Review the verified behavior and mark the increment complete.

## Risks and mitigations

- **Invisible app after close:** retained the Dock, added a fixed Open action, and handled macOS reopen.
- **Untrusted routing:** used a closed enum and ignored unknown IDs.
- **Tauri callback failure:** returned typed, sanitized errors and avoided panics in project code.
- **Scope creep:** deferred frontend listeners and global shortcuts.
- **Cross-platform build:** enabled tray only on macOS and provided a no-op adapter elsewhere.
- **Menu discovery confusion:** documented that custom actions appear under the right-side macOS status-item icon, not the standard application-name menu.

## Verification commands and results

The project owner confirmed these commands passed on the target Mac:

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
npm run typecheck
npm run build
npm run tauri -- dev
```

Results:

- Rust formatting passed.
- Clippy passed with warnings denied.
- All Rust unit and integration tests passed.
- TypeScript typechecking passed.
- The Vite production build passed.
- The native Tauri application launched successfully.

## Manual smoke-test results

- [x] The menu-bar icon appeared.
- [x] Open showed and focused the main window.
- [x] New Request showed and focused the main window without an error.
- [x] Tasks (Coming Soon) showed and focused the main window without an error.
- [x] Closing the main window hid it without terminating the process.
- [x] A menu-bar action reopened the hidden window.
- [x] The Dock icon reopened the hidden window.
- [x] Quit terminated the process.
- [x] The existing React UI remained unchanged.
- [x] `get_app_info` remained functional.
- [x] Storage startup remained idempotent.
- [x] No macOS permission prompt appeared.

## Rollback strategy

If a future regression requires rollback:

- Remove `menu_bar` from `src-tauri/src/lib.rs`.
- Restore the prior Tauri dependency feature declaration.
- Remove the menu-bar module and routing integration test.
- Restore normal main-window close behavior by removing the global window callback.
- Do not alter storage, migrations, CSP, capabilities, or frontend files.

## Exit criteria

All exit criteria passed:

- all locked Rust and frontend checks passed,
- the tray icon and fixed actions worked on macOS,
- close hid, menu/Dock reopened, and Quit exited,
- the existing UI, storage startup, and `get_app_info` remained functional,
- no permission prompt or authority expansion occurred,
- project-memory and increment documents record the actual verification result.

## Next step

Increment 2E — React application shell — is Ready. It may consume the existing route event but must preserve the verified native lifecycle, command surface, storage behavior, CSP, capabilities, and permission footprint.
