# F-08 app-info runtime IPC narrowing post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "npx vitest run src/infrastructure/tauri/app-info-client.test.ts src/App.test.tsx",
    "npm run test:repository",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "npm run verify",
    "git diff --check",
    "python3 .codex/hooks/session_end_gate.py",
    "npm run tauri -- dev"
  ],
  "files_changed": [
    "ARCHITECTURE.md",
    "CHANGELOG.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "docs/increments/app-info-runtime-ipc-narrowing-f08.md",
    "docs/plans/2026-08-27-app-info-runtime-ipc-narrowing-f08.md",
    "docs/reviews/2026-08-27-app-info-runtime-ipc-narrowing-f08-post-increment-review.md",
    "scripts/repository_health.py",
    "scripts/tests/test_repository_health.py",
    "src/App.test.tsx",
    "src/application/useCoreConnection.ts",
    "src/infrastructure/tauri/app-info-client.test.ts",
    "src/infrastructure/tauri/app-info-client.ts"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Roadmap",
      "effort": "Small documentation and readiness review",
      "milestone": "Before any agent IPC or interactive-demo implementation",
      "risk": "Starting later work without an exact approved plan would bypass dependency ordering and trust-boundary review.",
      "severity": "Advisory",
      "summary": "No successor implementation plan is owner-selected or Ready."
    }
  ],
  "increment_id": "app-info-runtime-ipc-narrowing-f08",
  "manual_verification": [
    {
      "check": "Target-Mac npm run tauri -- dev launches the source-current debug process",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Native Settings renders Local core ready and the closed get_app_info fields",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Controlled rejection seam renders fixed safe copy without the upstream sentinel",
      "required": true,
      "status": "Passed"
    }
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {
      "command": "npx vitest run src/infrastructure/tauri/app-info-client.test.ts src/App.test.tsx",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run test:repository",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run docs:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run repository:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run security:scan",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run verify",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "git diff --check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    }
  ]
}
-->

Date: 2026-08-27
Increment: app-info-runtime-ipc-narrowing-f08
Branch: codex/f08-runtime-ipc-narrowing

## Executive summary

The existing app-info WebView client now treats the sole `get_app_info` reply
as untrusted `unknown`, accepts only one exact bounded six-field DTO, requires
the closed environment and `secureCore: true`, and exposes only fixed failure
copy. Acceptance criteria are met. The quality result is `PASS WITH
ADVISORIES` solely because no successor implementation plan is Ready.

## Scope and boundaries

The runtime change is confined to the existing TypeScript IPC client and
connection hook with focused tests. The F-15 static documentation-truth guard
was updated atomically so it protects the new current-state claim. No command,
event, Rust behavior, capability, CSP, permission, dependency, agent IPC,
provider, model, tool, execution, network, credential, persistence, filesystem,
background autonomy, audit, or device effect changed.

## Verification results

- `npx vitest run src/infrastructure/tauri/app-info-client.test.ts src/App.test.tsx`:
  Passed, 37/37.
- `npm run test:repository`: Passed, 47/47.
- `npm run docs:check`: Passed.
- `npm run repository:check`: Passed.
- `npm run security:scan`: Passed.
- `npm run verify`: Passed, including formatting, repository health, strict
  frontend/Rust lint, 28 hook tests, 47 repository tests, 220 frontend tests,
  Rust library/integration tests with one intentional Hermes probe ignored,
  frontend production build, and Tauri release no-bundle build.
- `git diff --check`: Passed.
- `python3 .codex/hooks/session_end_gate.py`: Passed; no conflicts or unrelated
  generated, credential, personal-data, log, or build-output paths were found.
- Target-Mac `npm run tauri -- dev`: Passed; Vite started and the debug native
  process launched.
- Native app-info smoke: Passed. The available native Settings window rendered
  `Local core ready`, `Rust core connected`, command `get_app_info`, Cortexa
  0.1.0, `macos · aarch64`, and the closed build environment.
- Controlled rejection/redaction: Passed in the focused App test; the fixed
  message rendered and the upstream `IPC unavailable` sentinel was absent.

Two intermediate `npm run verify` attempts failed before the final passing run:
the first found Prettier formatting in the changed client, and the second found
an unused catch binding after redaction. One focused App run then identified
the intentionally stale raw-error assertion. Each was corrected within scope,
and every affected command was rerun successfully after the last edit.

## Architecture findings

No finding. Runtime narrowing remains owned by the Tauri infrastructure client;
the React hook consumes only validated `AppInfo`. The Rust command, command
registration, capability file, CSP, and module boundaries are unchanged. No
new abstraction, dependency, or cross-layer authority was introduced.

## Security findings

No finding. The untrusted WebView boundary now fails closed on non-object,
array, missing, extra, empty, oversized, wrong-type, invalid-environment, and
`secureCore: false` values. Strings are bounded to 128 Unicode code points.
Rejected data and upstream error details do not enter UI state. There is no new
credential, personal data, logging, storage, filesystem, network, permission,
approval, policy, audit, or execution path.

## Code-health findings

No finding. Strict TypeScript passes. The validator returns a newly constructed
closed DTO rather than forwarding the unknown object. Focused table-driven
tests cover success, the exact 128-code-point boundary, malformed shapes,
limits, enum rejection, secure-core rejection, and UI error redaction.

## Technical debt

None introduced or exposed by this increment.

## Roadmap findings

Advisory: no successor implementation plan is owner-selected or Ready. F-08
does not authorize agent IPC or the interactive demo. The smallest next action
is a separate readiness review of the next dependency-ordered prerequisite.

## Completion decision

`PASS WITH ADVISORIES`.

## Next-increment readiness

`Blocked`. No exact successor plan is owner-selected or Ready.

## Exact files changed

- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `docs/increments/app-info-runtime-ipc-narrowing-f08.md`
- `docs/plans/2026-08-27-app-info-runtime-ipc-narrowing-f08.md`
- `docs/reviews/2026-08-27-app-info-runtime-ipc-narrowing-f08-post-increment-review.md`
- `scripts/repository_health.py`
- `scripts/tests/test_repository_health.py`
- `src/App.test.tsx`
- `src/application/useCoreConnection.ts`
- `src/infrastructure/tauri/app-info-client.test.ts`
- `src/infrastructure/tauri/app-info-client.ts`

## Exact commands executed

- `npx vitest run src/infrastructure/tauri/app-info-client.test.ts src/App.test.tsx`
- `npm run test:repository`
- `npm run docs:check`
- `npm run repository:check`
- `npm run security:scan`
- `npm run verify`
- `git diff --check`
- `python3 .codex/hooks/session_end_gate.py`
- `npm run tauri -- dev`
