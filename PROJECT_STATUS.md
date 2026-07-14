# Project status

Last updated: 2026-07-13

## Current milestone

Phase 2 — local desktop shell, trusted local-core foundation, and frontend application shell.

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
- Increment 2E: React application shell — **implementation complete; target-Mac verification pending**.
- Increment 2F: mocked agent streaming and activity — **blocked by Increment 2E**.

## Confirmed verified baseline through Increment 2D

- Tauri 2 launches on the Apple Silicon target Mac.
- React renders in the native main window.
- The WebView invokes the typed Rust `get_app_info` command.
- Rust formatting, Clippy, and all Rust tests pass.
- TypeScript and the Vite production build pass.
- Storage startup is idempotent and recognizes the persisted bootstrap marker.
- The macOS menu-bar status icon and all four fixed actions work.
- Closing the main window hides it without terminating the process.
- Menu-bar and Dock actions restore the hidden main window.
- No macOS permission prompt appears.

## Increment 2E implemented scope

- Platform-neutral reducer-and-context shell state.
- Seven deterministic sidebar routes.
- Conversation workspace and disabled in-memory composer.
- Tasks, Memory, Activity, and Integrations page shells.
- Settings shell with typed Rust diagnostics.
- Permission Center shell with nine placeholder permission rows and no request controls.
- Empty, loading, and error presentations.
- Strict listener for `assistant-menu-route`.
- Closed route mapping:
  - `new_request` → Conversations and clear the draft.
  - `tasks_placeholder` → Tasks.
- Unknown, malformed, or extended event payloads are ignored.
- Focused frontend test coverage.

## Increment 2E artifact-host verification

Passed:

```text
npm ci
npx prettier --check .
npm run lint:frontend
npm run typecheck
npx vitest run — 3 files, 30 tests
npm run build
npm audit --audit-level=low — 0 vulnerabilities
git diff --check
```

Not available on the artifact host:

```text
cargo fmt
cargo clippy
cargo test
native Tauri launch
macOS menu-route and lifecycle smoke test
```

## Current completion gate

Increment 2E remains open until the target Mac passes:

```bash
npm run format:check
npm run lint
npm run typecheck
npm run test:unit
npm run test:integration
npm run build
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
npm run tauri -- dev
```

The manual gate must confirm all sidebar destinations, Settings, Permissions without an OS prompt, New Request routing, Tasks routing, close/reopen/Dock/quit regressions, Rust diagnostics, and idempotent storage startup.

## Security posture

- The model remains outside the authorization boundary.
- `get_app_info` remains the only custom Tauri command.
- Increment 2E changes frontend source and project documentation only.
- Capabilities, CSP, Tauri configuration, Rust source, storage migrations, dependency manifests, and lockfiles are unchanged.
- No API key, model network, OAuth, OS permission, shell, platform automation, or user-data persistence was added.
- Native event payloads are validated as `unknown` against a closed contract.

## Next scope

Increment 2F may begin only after Increment 2E is verified and checkpointed. It will introduce deterministic mocked streaming and activity presentation without production model access or privileged tools.

## Phase 2 Increment 2E verification

Increment 2E is verified complete on the target Mac as of 2026-07-13.

Completed capabilities:

- React reducer-and-context application state.
- Sidebar navigation for Conversations, Tasks, Memory, Activity, Integrations, Permissions, and Settings.
- Conversation workspace and non-functional composer shell.
- Empty, loading, and error presentation states.
- Settings diagnostics backed by the existing `get_app_info` command.
- Permission Center placeholders with no OS permission requests.
- Closed `assistant-menu-route` handling for `new_request` and `tasks_placeholder`.
- Safe rejection of unknown or malformed route payloads.

All required frontend, Rust, native-launch, lifecycle, storage-idempotence, and no-permission-prompt checks passed.

The next ready increment is Phase 2 Increment 2F.
