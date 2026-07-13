# Increment 2D — macOS menu-bar and window lifecycle

Last updated: 2026-07-13

Status: **Implementation complete; target-Mac verification pending**

## Goal

Add an unprivileged macOS menu-bar entry and safe main-window lifecycle behavior while preserving the existing React UI, `get_app_info` command, SQLite startup, Tauri capabilities, CSP, and permission footprint.

## Verified baseline

Increment 2C was verified on the target Mac before this work began:

- rustfmt and Clippy passed,
- 41 Rust unit tests passed,
- 3 Rust integration tests passed,
- TypeScript and Vite passed,
- the native app launched,
- later startup applied no migrations and recognized prior initialization.

## Implemented scope

### Fixed menu contract

The macOS menu-bar menu contains:

```text
Open AI Agent Assistant
—
New Request
Tasks (Coming Soon)
—
Quit AI Agent Assistant
```

Each item has a private fixed identifier. Unknown identifiers are ignored.

### Window activation

Opening or routing first resolves the existing window labeled `main`, then:

1. shows the macOS application,
2. unminimizes the window,
3. shows the window,
4. focuses the window.

A missing main window or Tauri failure returns a typed `MenuBarError`.

### Route events

New Request and Tasks emit:

```text
assistant-menu-route
```

with one closed route value:

```text
new_request
tasks_placeholder
```

The React UI does not consume the event in this increment. The event exists so Increment 2E can connect the shell without adding another native command.

### Close and reopen behavior

- Closing only the `main` window prevents destruction and hides it.
- Future non-main windows retain normal close behavior.
- A macOS reopen/Dock event shows the main window only when no application window is visible.
- The regular Dock icon and activation policy remain unchanged.
- Quit uses the fixed menu action and exits the application.

### Portability

The pure action, dispatch, close, and reopen contracts compile on every target. The Tauri tray adapter is macOS-only; other targets use a no-op adapter and do not enable the tray feature.

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

No lockfile, frontend source, Tauri capability, or Tauri configuration file changes.

## Tests added

Rust unit and integration tests cover:

- stable action order, IDs, and labels,
- new-request routing after window activation,
- tasks routing after window activation,
- quit without show or route,
- unknown menu IDs with no side effects,
- activation failure before route emission,
- route failure after successful activation,
- close-to-hide only for the main window,
- reopen only when no window is visible,
- public routing behavior across every fixed action.

No test requires a real tray, operating-system permission, network, calendar, contacts, local files, or product data.

## Security review

Preserved invariants:

- `get_app_info` remains the only custom Tauri command.
- No WebView-invokable capability was added.
- CSP and capability files are unchanged.
- Route payloads are closed enums and carry no arbitrary content.
- Unknown menu IDs fail closed by doing nothing.
- Menu callbacks do not use model output as authorization.
- No API key, OAuth, model network, database schema, Keychain, or privileged macOS API was added.
- New production paths use typed errors and no `unwrap`, `expect`, `panic!`, `todo!`, `unimplemented!`, or `unreachable!`.

## Artifact-workspace verification

```text
Prettier: passed
ESLint: passed with zero warnings
Strict TypeScript: passed
Vitest: 2 passed, 0 failed
Vite production build: passed
npm audit: 0 vulnerabilities
Tauri invoke-handler scan: only get_app_info remains registered
Cargo.lock and package-lock: unchanged
Tauri capability and CSP files: unchanged
Prohibited production-code scan: passed
Git whitespace check: passed
```

Cargo and a native macOS runtime are unavailable in the artifact workspace, so Rust compilation and lifecycle behavior remain pending.

## Required target-Mac verification

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
npm run typecheck
npm run build
npm run tauri -- dev
```

Manual checks:

- menu-bar icon appears,
- Open shows and focuses the main window,
- New Request shows/focuses without error,
- Tasks shows/focuses without error,
- closing the main window hides it without exiting,
- menu action reopens the hidden window,
- Dock click reopens it when hidden,
- Quit exits,
- UI and `get_app_info` remain correct,
- storage remains idempotent,
- no permission prompt appears.

## Completion gate

Do not mark Increment 2D complete or begin Increment 2E until all target-Mac checks, manual lifecycle checks, and complete diff review pass.
