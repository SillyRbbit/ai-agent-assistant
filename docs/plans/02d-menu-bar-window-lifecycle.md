# Execution plan — Increment 2D macOS menu-bar and window lifecycle

Last updated: 2026-07-13

Status: **Active — implementation complete; target-Mac verification pending**

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

## Files expected to change

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
- [ ] Run rustfmt, Clippy, and all Rust tests on the target Mac.
- [ ] Run target-Mac TypeScript and Vite checks.
- [ ] Perform the full manual menu-bar/window lifecycle smoke test.
- [ ] Review the complete diff and mark the increment verified.

## Risks and mitigations

- **Invisible app after close:** retain the Dock, add a fixed Open action, and handle macOS reopen.
- **Untrusted routing:** use a closed enum and ignore unknown IDs.
- **Tauri callback failure:** report typed, sanitized errors; never panic in project code.
- **Scope creep:** defer frontend listeners and global shortcuts.
- **Cross-platform build:** enable tray only on macOS and provide a no-op adapter elsewhere.

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

## Rollback strategy

- Remove `menu_bar` from `src-tauri/src/lib.rs`.
- Restore the Tauri dependency feature declaration.
- Remove the menu-bar module and routing integration test.
- Restore normal main-window close behavior by removing the global window callback.
- Do not alter storage, migrations, CSP, capabilities, or frontend files.

## Exit criteria

- All locked Rust and frontend checks pass.
- Tray icon and fixed actions work on macOS.
- Close hides; menu/Dock reopen; quit exits.
- Existing UI, storage startup, and `get_app_info` remain functional.
- No permission prompt or authority expansion occurs.
- Documentation contains actual verification results.
