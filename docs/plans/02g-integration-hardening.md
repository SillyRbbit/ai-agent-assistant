# Execution plan — Increment 2G integration hardening

Last updated: 2026-07-13

Status: **Complete**

## Goal and user-visible outcome

Harden the deterministic frontend run lifecycle with explicit cancellation, bounded failure and Retry, a redacted volatile Activity feed, and complete Phase 2 release-verification evidence.

## Scope

- Put mock scheduling behind an injectable typed driver.
- Return an idempotent cancellation handle from every started driver.
- Accept only closed chunk, completion, and bounded failure events.
- Add a generic failed assistant state and deterministic Retry.
- Reuse the failed request without duplicating its user message.
- Record accepted lifecycle transitions as fixed-copy in-memory Activity events.
- Render Activity empty and populated states.
- Add focused success, failure, cancellation, stale-event, privacy, and interaction tests.

## Explicit non-goals

- Production model or gateway networking.
- API keys, credentials, OAuth, or cloud accounts.
- Real tools or trusted WebView authorization.
- Trusted audit persistence or sensitive SQLite data.
- Rust, IPC, Tauri command, capability, CSP, dependency, packaging, or OS permission changes.

## Existing behavior preserved

- Typed `get_app_info` diagnostics.
- In-memory streaming, Stop, tool activity, and mock approval decisions.
- Closed native menu routing.
- SQLite startup, menu-bar lifecycle, restrictive capabilities, and CSP.

## Risks and mitigations

- Stale callbacks: bind every event to the exact active streaming run ID.
- Duplicate retries: append a new assistant attempt without another user message.
- Sensitive Activity content: construct events only from fixed copy and validated mock run IDs.
- Raw error disclosure: map all driver failures and startup exceptions to one bounded reason.
- Cancellation leaks: make driver cancellation idempotent and invoke it through effect cleanup.

## Verification

- [x] Strict TypeScript and frontend lint.
- [x] Focused Activity, driver, reducer, and App tests.
- [x] Full `npm run verify`.
- [x] `npm audit --audit-level=low`.
- [x] `git diff --check`.
- [x] Native Tauri launch and idempotent startup.
- [x] Manual regression and Activity presentation checks.
- [x] Confirm no operating-system permission prompt appears.

## Rollback strategy

Remove the new driver, Activity model, and Activity page; restore the Increment 2F hook, reducer, App composition, conversation presentation, tests, and styles; preserve all Rust, Tauri, storage, dependency, capability, CSP, and permission files.

## Exit criteria

Increment 2G and Phase 2 complete only after automated verification, native launch, manual acceptance, security review, and project-memory synchronization all pass.

## Completion record

Completed on 2026-07-13. Automated release verification, dependency audit, and native launch passed on the target Mac. The project owner confirmed the complete interaction, Activity redaction, lifecycle, diagnostics, storage, and no-permission-prompt checklist passed.
