# Project status

Last updated: 2026-07-13

## Current milestone

Phase 2 — local desktop shell and trusted local-core foundation.

## Increment status

- Increment 1: smallest runnable Tauri application — **complete**.
- Increment 1.1: Node.js 26/npm 11 compatibility — **complete**.
- Increment 1.2: repository workflow and handoff system — **complete**.
- Increment 2A: platform-neutral Rust interfaces and deterministic mocks — **verified complete on target Mac**.
- Increment 2B-0: SQLite storage dependency and design decision — **complete**.
- Increment 2B-1: SQLite dependency and migration skeleton — **verified complete on target Mac**.
- Increment 2B-1A: Rust 1.90 SQLite compatibility repair — **verified complete on target Mac**.
- Increment 2C: storage startup integration — **verified complete on target Mac**.
- Increment 2D: macOS menu-bar and window lifecycle — **implementation complete; target-Mac verification pending**.
- Increment 2E remains blocked until Increment 2D is verified.

## Confirmed working baseline

Before Increment 2D:

- Tauri 2 launches on the target Mac.
- React renders in the native main window.
- The WebView invokes the typed Rust `get_app_info` command.
- Rust formatting and Clippy with warnings denied pass.
- 41 Rust unit tests and 3 Rust integration tests pass.
- TypeScript and the Vite production build pass.
- Storage startup is idempotent and recognizes the persisted bootstrap marker.
- No permission prompt appears.

## Implemented foundations

- Platform-neutral trusted-core interfaces with deterministic mocks.
- Shared risk and permission placeholder types.
- SQLCipher-capable SQLite connection and immutable migration skeleton.
- `schema_migrations` and `app_metadata` only.
- Managed `Storage` initialized from Tauri startup.
- Typed `app_initialized` metadata.
- Debug-only development database and release-mode in-memory storage.

## Increment 2D implementation state

- Tauri's built-in `tray-icon` feature is enabled only on macOS.
- The existing application icon is used as a temporary template menu-bar icon.
- Fixed actions: open main window, new request, tasks placeholder, and quit.
- Main-window activation shows the application, unminimizes, shows, and focuses the existing window.
- New-request and tasks actions emit a closed-enum `assistant-menu-route` event.
- Unknown menu IDs have no effect.
- Closing the main window hides it and keeps the process running.
- A macOS reopen/Dock event restores the window when no application window is visible.
- Future non-main windows retain normal close behavior.
- React does not consume route events yet.

## Test coverage added

- Stable menu IDs and labels.
- New-request and tasks routing order.
- Quit without show or route.
- Unknown menu ID rejection without side effects.
- Failure short-circuiting.
- Main-window close-to-hide policy.
- Reopen policy for visible and hidden states.
- Public routing contract across all actions.

## Explicit non-goals

- React navigation or UI changes.
- Global shortcut.
- Accessory-only activation policy or Dock hiding.
- Dedicated production tray artwork.
- New Tauri commands, plugins, capabilities, or CSP rules.
- API keys, OAuth, model or gateway networking.
- Database schema or product-data persistence.
- Accessibility, screen capture, Apple Events, microphone, shell, or broad filesystem access.

## Artifact-workspace verification

```text
Prettier: passed
ESLint: passed with zero warnings
Strict TypeScript: passed
Vitest: 2 passed, 0 failed
Vite build: passed
npm audit: 0 vulnerabilities
Tauri invoke-handler scan: only get_app_info remains registered
Capability and CSP diff: no changes
Cargo.lock and package-lock diff: no changes
Prohibited production-code scan: passed
Cargo/rustfmt/Clippy/native checks: unavailable on artifact host
```

## Completion gate

Do not mark Increment 2D complete or begin Increment 2E until all locked Rust checks, frontend checks, native menu-bar/window behavior checks, and the complete diff review pass on the target Mac.

## Project memory map

- Current handoff: `HANDOFF.md`
- Priorities: `NEXT_STEPS.md`
- Decisions: `DECISIONS.md`
- Changes: `CHANGELOG.md`
- Plans: `PLANS.md`
- Troubleshooting history: `TROUBLESHOOTING_LOG.md`
- Increment 2D: `docs/increments/02d-menu-bar-window-lifecycle.md`
- Active plan: `docs/plans/02d-menu-bar-window-lifecycle.md`
