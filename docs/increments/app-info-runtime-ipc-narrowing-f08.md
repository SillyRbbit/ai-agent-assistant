# F-08 app-info runtime IPC narrowing

Status: Verified complete with advisories
Owner: Project owner
Last updated: 2026-08-27

## Goal and outcome

Fail closed when the existing `get_app_info` command returns malformed,
insecure, or unexpectedly detailed data. The WebView now accepts only one exact
bounded DTO and displays fixed safe failure copy.

## Implemented scope

- Invoke `get_app_info` as `unknown` before runtime validation.
- Require exactly the six application-owned fields, non-empty strings bounded
  to 128 Unicode code points, the development/production environment enum, and
  `secureCore: true`.
- Reject missing, extra, empty, oversized, wrong-type, invalid-environment, and
  insecure values without promoting them to ready state.
- Prevent native or IPC error text from entering UI diagnostics.
- Add focused parser and UI redaction regression tests.

## Boundaries preserved

No new command, event, Rust behavior, capability, CSP, permission, dependency,
agent IPC, provider, model, network, credential, tool, approval, execution,
persistence, filesystem, background autonomy, audit, or device effect.

## Evidence

- Focused app-info and App tests: Passed, 37/37.
- Full frontend tests: Passed, 220/220.
- `npm run verify`: Passed, including strict lint/type/Rust checks, tests,
  frontend build, and Tauri release no-bundle build.
- `npm run security:scan`, repository checks, and `git diff --check`: Passed.
- Target-Mac native smoke: Passed; `Local core ready` and the closed Settings
  app-info fields rendered. Controlled rejection/redaction is proven by the
  focused UI test.

## Advisory

Computer Use directly inspected the available release app while the same
source-current `npm run tauri -- dev` run separately launched the debug native
process. No direct debug-window accessibility binding was available. This does
not block the unchanged command contract or focused rejection evidence.
