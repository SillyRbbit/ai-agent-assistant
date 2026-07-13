# Working-session handoff

Last updated: 2026-07-13

## Current phase

Phase 2 — repository setup, working Tauri shell, and trusted local-core foundation.

Current increment: **Increment 2D — macOS menu-bar and window lifecycle**.

Status: **Implementation complete; target-Mac verification pending**.

## Verified baseline

The project owner verified Increment 2C on the Apple Silicon target Mac before Increment 2D began:

- `cargo fmt --check` passed.
- Clippy passed for all targets and all features with warnings denied.
- 41 Rust unit tests passed.
- 3 Rust integration tests passed.
- `npm run typecheck` passed.
- `npm run build` passed.
- The Tauri application launched successfully.
- A later launch reported `applied_migrations=0`, `already_applied_migrations=2`, and `previously_initialized=true`.
- The UI, `get_app_info`, storage startup, CSP, capabilities, and permission footprint remained intact.

Increment 2C is verified complete.

## Increment 2D implementation

Increment 2D now:

- enables Tauri's built-in `tray-icon` feature only for macOS,
- creates one menu-bar entry using the existing bundled application icon as a temporary template icon,
- adds fixed actions for opening the main window, starting a new request, opening a tasks placeholder, and quitting,
- shows the application, then unminimizes, shows, and focuses the existing `main` window,
- emits the closed-enum `assistant-menu-route` event for new-request and tasks-placeholder intents,
- ignores unknown menu identifiers without side effects,
- hides the main window instead of destroying it when its close control is used,
- restores the main window on a macOS reopen/Dock event when no application window is visible,
- leaves future non-main windows on the normal close path,
- keeps Tauri-specific operations behind deterministic routing contracts,
- adds focused unit and integration tests.

The current React UI does not consume the route event yet. Selecting New Request or Tasks still shows and focuses the current main window. React routing is deferred to Increment 2E.

## Files changed in Increment 2D

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

## Verification completed in the artifact workspace

Before implementation:

```text
npm ci: passed
npm run typecheck: passed
npm run build: passed
Vitest: 2 passed, 0 failed
Cargo baseline: not run because Cargo is unavailable on the artifact host
```

After implementation:

```text
Prettier: passed
ESLint: passed with zero warnings
Strict TypeScript: passed
Vitest: 2 passed, 0 failed
Vite production build: passed
npm audit: 0 vulnerabilities
Tauri invoke-handler scan: only get_app_info remains registered
Capability and CSP diff: no changes
Cargo.lock and package-lock diff: no changes
Prohibited panic-style production scan: passed
Git whitespace check: passed
```

Rust formatting, Clippy, Rust tests, and native macOS lifecycle behavior were not run in the artifact workspace because Cargo and a macOS runtime are unavailable there. They remain the target-Mac completion gate.

## Required target-Mac verification

From `/Users/hdang/Desktop/Projects/ai-agent-assistant`, run:

```bash
export PATH="$(brew --prefix rustup)/bin:$PATH"
rustup default 1.90.0

cargo fmt --manifest-path src-tauri/Cargo.toml
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
npm run typecheck
npm run build
npm run tauri -- dev
```

Manual macOS checks:

1. Confirm the existing main window opens and `Rust core connected` remains visible.
2. Confirm a menu-bar icon appears.
3. Choose **Open AI Agent Assistant** and verify the main window is shown and focused.
4. Choose **New Request** and verify the main window is shown and focused without an error.
5. Choose **Tasks (Coming Soon)** and verify the main window is shown and focused without an error.
6. Close the main window with its red close control and verify the process and menu-bar entry remain active.
7. Reopen the main window from the menu bar.
8. Hide the main window again, click the Dock icon, and verify the window is restored.
9. Choose **Quit AI Agent Assistant** and verify the process exits.
10. Confirm no macOS permission prompt appears.
11. Confirm storage startup remains idempotent and `get_app_info` still works.

Then review:

```bash
git status --short --branch
git diff --check
git diff --stat
git diff
```

## Security boundaries preserved

- No new Tauri command or WebView-invokable capability.
- No capability or CSP changes.
- No global shortcut.
- No API keys, OAuth, model networking, or gateway calls.
- No database schema or product-data persistence changes.
- No Accessibility, ScreenCaptureKit, Apple Events, microphone, shell, or broad filesystem access.
- Unknown menu identifiers are ignored.
- Route events carry only a closed enum, not arbitrary content.

## Next ready work after verification

After Increment 2D is verified and committed, the next ready increment is **Increment 2E — React application shell**.

Do not begin Increment 2E until the target-Mac menu-bar, close, Dock/menu reopen, quit, Rust, frontend, and native checks pass.

## Exact resume prompt

```text
Use $session-end.

Mark Phase 2 Increment 2D verified complete using the actual target-Mac command and manual smoke-test results. Update HANDOFF.md, PROJECT_STATUS.md, NEXT_STEPS.md, CHANGELOG.md, PLANS.md, TROUBLESHOOTING_LOG.md if needed, docs/increments/02d-menu-bar-window-lifecycle.md, and docs/plans/02d-menu-bar-window-lifecycle.md. Identify Increment 2E — React application shell — as the next ready increment, but do not implement it.
```
