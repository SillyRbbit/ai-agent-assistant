# Project status

Last updated: 2026-07-13

## Current milestone

Phase 2 — **verified complete**. Phase 3 gap analysis and increment planning are next.

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
- Increment 2E: React application shell — **verified complete on target Mac**.
- Increment 2F: mocked assistant interaction shell — **verified complete on target Mac**.
- Increment 2G: integration hardening — **verified complete on target Mac**.

## Verified baseline through Increment 2E

- Tauri launches on the Apple Silicon target Mac.
- React renders in the native main window and invokes typed `get_app_info` IPC.
- SQLite startup is idempotent and persists only the bootstrap marker in development.
- The macOS status-item menu, close-to-hide, menu reopen, Dock reopen, and Quit work.
- The seven-route React shell, Settings diagnostics, Permissions placeholders, and closed menu routing work.
- No operating-system permission prompt appears.

## Increment 2F capability

- In-memory user and assistant messages.
- Fixed deterministic text chunks and progressive streaming.
- Stop with timer cancellation and late-event rejection.
- Mock `create_local_task` activity card.
- Exact mock preview for target, affected data, reversibility, permission, and risk.
- Deterministic approve, reject, and edit outcomes with no execution.
- Edit returns a deterministic draft to the composer.

## Verification evidence

Passed on the target Mac:

```text
npm run lint:frontend
npm run typecheck
targeted Vitest — 3 files, 37 tests
npm run verify
full Vitest — 4 files, 47 tests
Rust library tests — 50 passed
Rust integration tests — 6 passed
Vite production build
Tauri release build --no-bundle
git diff --check
native Tauri development launch
storage startup — idempotent, 2 migrations already applied
```

The project owner confirmed manual progressive streaming, Stop, approve/reject/edit outcomes, Edit draft restoration, minimum-window layout, close/reopen/Dock/quit behavior, Settings diagnostics, idempotent storage startup, and absence of permission prompts all passed.

## Security posture

- The model remains outside the authorization boundary.
- `get_app_info` remains the only custom Tauri command.
- Increment 2F changes frontend source, tests, styles, and project documentation only.
- Capabilities, CSP, Tauri configuration, Rust source, storage, dependencies, and lockfiles are unchanged.
- No model network, API key, OAuth, OS permission, shell, platform automation, or user-data persistence was added.
- WebView approval decisions are explicitly mock-only and cannot authorize or invoke an action.
- Run identifiers and valid-state checks reject stale asynchronous events.

## Increment 2G capability and evidence

- Typed mock-run driver with explicit cancellation.
- Bounded failure copy and deterministic Retry.
- Redacted in-memory Activity feed with no request, argument, result, or error-detail content.
- Stale chunk, completion, and failure events fail closed.

Passed:

```text
npm run verify
Frontend — 6 files, 63 tests passed
Rust library — 50 tests passed
Rust integration — 6 tests passed
Vite production build
Tauri release build --no-bundle
npm audit --audit-level=low — 0 vulnerabilities
git diff --check
```

Native launch passed with idempotent storage startup. The project owner confirmed streaming, Stop, approval decisions, Activity empty and populated states, newest-first lifecycle events, Activity redaction, close/reopen/Dock/quit behavior, Settings diagnostics, and absence of permission prompts all passed.

## Next action

Perform Phase 3 gap analysis and define the first bounded Phase 3 increment without changing runtime code.
