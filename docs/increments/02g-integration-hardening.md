# Phase 2 Increment 2G — integration hardening

Last updated: 2026-07-13

Status: **Verified complete**

## Goal

Complete bounded cancellation, error-state, Activity-view, and release-verification hardening without adding production model access, real tools, privileged automation, or persistence.

## Implemented behavior

- `MockRunDriver` emits only chunk, completed, or bounded failed events.
- Every driver start returns an idempotent cancellation handle.
- Stop, failure, completion, Retry, and unmount clean up the prior driver.
- Driver startup exceptions become the same bounded failure reason.
- Retry uses a new run ID and does not duplicate the user message.
- Late events fail closed unless they match the active streaming run.
- Activity records accepted starts, Stops, failures, approval requests, and decisions.
- Activity records contain fixed copy and validated mock run IDs only.
- Activity remains in memory and clears on reload.

## Files created

```text
src/application/activity.ts
src/application/activity.test.ts
src/application/mockRunDriver.ts
src/application/mockRunDriver.test.ts
src/features/activity/ActivityPage.tsx
docs/plans/02g-integration-hardening.md
docs/increments/02g-integration-hardening.md
```

## Files changed

```text
src/application/mockAssistantRun.ts
src/application/state.ts
src/application/state.test.ts
src/application/useMockAssistantRun.ts
src/App.tsx
src/App.test.tsx
src/features/conversations/ConversationWorkspace.tsx
src/styles.css
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
```

No other source, dependency, lockfile, Rust, Tauri, capability, CSP, storage, packaging, or permission file changed.

## Focused coverage

- Fixed Activity copy for every closed event kind.
- Invalid activity ordinals and run identifiers.
- Ordered driver events and idempotent cancellation.
- Stop and unmount cancellation.
- Stale chunk, completion, and failure rejection.
- Bounded emitted and thrown driver failures.
- Retry with a fresh run ID and one user message.
- Activity empty state, populated order, and request-content exclusion.
- Existing streaming, approvals, navigation, diagnostics, permissions, and native menu routing.

## Automated verification evidence

```text
npm run typecheck: passed
npm run lint:frontend: passed
targeted Vitest: 4 files, 48 tests passed
npm run verify: passed
full Vitest: 6 files, 63 tests passed
Rust library tests: 50 passed
Rust integration tests: 6 passed
Vite production build: passed
Tauri release build --no-bundle: passed
npm audit --audit-level=low: 0 vulnerabilities
git diff --check: passed
```

## Security review

- The WebView gains no executor or authorization path.
- Activity is presentation-only and is not described as the trusted audit log.
- Activity cannot contain request text, tool arguments, tool results, or underlying errors.
- Failure copy is fixed and contains no provider detail.
- No network, credential, OAuth, storage, dependency, IPC, Tauri, Rust, capability, CSP, packaging, or OS permission change was added.

## Native and manual verification result

Native Tauri launch passed with idempotent storage startup. The project owner confirmed:

- Activity initially showed the empty state.
- Streaming, Stop, Approve mock, Reject, and Edit remained functional.
- Completed and stopped runs produced newest-first fixed Activity events.
- Activity exposed no request text, tool arguments, tool results, or error details.
- Close-to-hide, status-item reopen, Dock reopen, and Quit remained functional.
- Settings diagnostics remained functional.
- No operating-system permission prompt appeared.

Bounded failure and Retry remain verified through injected-driver automated tests because the production deterministic driver does not intentionally fail.

Increment 2G and Phase 2 are verified complete.
