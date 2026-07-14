# Execution plan — Increment 2F mocked assistant interaction shell

Last updated: 2026-07-13

Status: **Complete**

## Goal and user-visible outcome

Add a deterministic local interaction flow to the verified React shell: in-memory messages, progressive mock assistant text, Stop, tool activity, and a clearly non-executing mock approval preview with approve, reject, and edit outcomes.

## Scope

- Extend the existing reducer with closed mock run transitions.
- Generate one fixed local run script from a normalized request.
- Bind scheduled chunks and completion to an active run identifier.
- Render messages and one mock tool activity type.
- Present exact mock action details before a decision.
- Keep all state in WebView memory.
- Add focused success and failure-state tests.

## Explicit non-goals

- Model or gateway networking, API keys, credentials, or OAuth.
- Real tool execution or trusted approval authorization.
- New Tauri commands, events, capabilities, plugins, or CSP changes.
- Rust, SQLite, persistence, dependency, lockfile, or OS permission changes.
- Audit persistence, retry, production error recovery, or background runs.

## Files

The implementation is limited to the frontend application, conversation components, focused tests, styles, and project-memory documents listed in the Increment 2F record.

## Risks and mitigations

- Late timer events: require the exact active run identifier and streaming state.
- Stop cleanup: cancel all scheduled timers when run state changes or the component unmounts.
- False authority: label approval as mock-only and execute no IPC or local action.
- Accidental persistence: keep state only in the existing reducer context.
- Layout regression: use bounded transcript, dialog, and responsive activity layout.

## Verification

- [x] Focused mock-script, reducer, and App interaction tests.
- [x] Strict TypeScript and frontend lint.
- [x] `npm run verify` on the target Mac.
- [x] Native Tauri launch and idempotent storage startup.
- [x] Manual progressive-streaming and Stop behavior.
- [x] Manual approve, reject, and edit outcomes.
- [x] Minimum-window layout and lifecycle regression checks.
- [x] Confirm no operating-system permission prompt appears.

## Rollback strategy

Remove the new mock-run and conversation components, restore the prior reducer, App composition, conversation workspace, tests, and styles, and preserve all Rust, Tauri, storage, dependency, capability, and CSP files.

## Exit criteria

Increment 2F completes only after every automated and manual verification item passes, the security boundaries remain unchanged, and project memory records the observed results.

## Completion record

Completed on 2026-07-13. Automated verification and native launch passed on the target Mac. The project owner subsequently confirmed every manual interaction, layout, lifecycle, storage, diagnostics, and no-permission-prompt check passed.
