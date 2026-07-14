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
- Increment 2D: macOS menu-bar and window lifecycle — **verified complete on target Mac**.
- Increment 2E: React application shell — **Ready**.
- Increment 2F remains blocked by Increment 2E.
- Increment 2G remains blocked by Increment 2F.

## Confirmed working baseline

The target-Mac baseline now includes:

- Tauri 2 launches on Apple Silicon macOS.
- React renders in the native main window.
- The WebView invokes the typed Rust `get_app_info` command.
- `get_app_info` remains the only custom Tauri command.
- Rust formatting passes.
- Clippy passes for all targets and features with warnings denied.
- All Rust unit and integration tests pass.
- Strict TypeScript typechecking passes.
- The Vite production build passes.
- SQLCipher-capable SQLite startup and immutable migrations work.
- Storage startup is idempotent and recognizes the persisted bootstrap marker.
- The native menu-bar and main-window lifecycle work as designed.
- No macOS permission prompt appears.

## Implemented trusted-core foundations

- Platform-neutral trusted-core interfaces with deterministic mocks.
- Shared risk and permission placeholder types.
- SQLCipher-capable SQLite connection and immutable migration skeleton.
- `schema_migrations` and `app_metadata` only.
- Managed `Storage` initialized from Tauri startup.
- Typed `app_initialized` metadata.
- Debug-only development database and release-mode in-memory storage.
- No product-data persistence yet.

## Verified macOS menu-bar and window lifecycle

- Tauri's built-in `tray-icon` feature is enabled only on macOS.
- The existing application icon is used as a temporary template menu-bar icon.
- The fixed menu contains Open, New Request, Tasks (Coming Soon), and Quit.
- Open, New Request, and Tasks show and focus the existing `main` window.
- New Request and Tasks emit a closed-enum `assistant-menu-route` event after activation.
- Unknown menu IDs have no effect.
- Closing the main window hides it and keeps the process running.
- A menu-bar action restores the hidden main window.
- A macOS reopen/Dock event restores the main window when no application window is visible.
- Quit terminates the process.
- Future non-main windows retain normal close behavior.
- The current React UI does not consume route events yet.

## Increment 2D verification

Target-Mac commands confirmed passed:

```text
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
npm run typecheck
npm run build
npm run tauri -- dev
```

Manual lifecycle checks confirmed:

- menu-bar icon appeared,
- Open showed and focused the main window,
- New Request showed and focused the main window without error,
- Tasks showed and focused the main window without error,
- close hid without terminating,
- menu-bar and Dock restored the hidden window,
- Quit exited,
- storage remained idempotent,
- `get_app_info` remained functional,
- the React UI remained unchanged,
- no permission prompt appeared.

## Test coverage through Increment 2D

- Agent-provider success and failure behavior.
- Tool registration, ordering, and unknown-tool rejection.
- Deterministic policy allow, approval, and deny decisions.
- Approval, audit, memory, and platform-adapter behavior.
- SQLite configuration, connection, migration, metadata, and startup behavior.
- Stable menu IDs and labels.
- New-request and tasks routing order.
- Quit without show or route.
- Unknown menu ID rejection without side effects.
- Failure short-circuiting.
- Main-window close-to-hide policy.
- Reopen policy for visible and hidden states.
- Public routing contract across all fixed actions.

## Next ready capability

Increment 2E will add the in-memory React application shell:

- sidebar navigation,
- conversation workspace,
- non-functional composer,
- page shells,
- empty/loading/error states,
- Settings and Permission Center shells,
- validated consumption of `assistant-menu-route`,
- focused reducer and component tests.

The shell must preserve the existing Rust, storage, menu-bar, IPC, CSP, capability, and permission boundaries.

## Explicit non-goals still in force

- Mock assistant streaming, tool activity, and approval UI until Increment 2F.
- Production OpenAI access or stored API keys.
- OAuth or gateway authentication.
- Conversation, task, memory, audit, approval, or tool-call persistence.
- New SQLite migrations during Increment 2E.
- Global shortcut.
- Accessory-only activation policy or Dock hiding.
- Dedicated production tray artwork.
- New Tauri commands, capabilities, or CSP rules.
- Accessibility, screen capture, Apple Events, microphone, shell, or broad filesystem access.
- Calendar, contacts, reminders, notifications, or clipboard tools.

## Project memory map

- Current handoff: `HANDOFF.md`
- Priorities: `NEXT_STEPS.md`
- Decisions: `DECISIONS.md`
- Changes: `CHANGELOG.md`
- Plans: `PLANS.md`
- Troubleshooting history: `TROUBLESHOOTING_LOG.md`
- Completed Increment 2D record: `docs/increments/02d-menu-bar-window-lifecycle.md`
- Completed Increment 2D plan: `docs/plans/02d-menu-bar-window-lifecycle.md`
