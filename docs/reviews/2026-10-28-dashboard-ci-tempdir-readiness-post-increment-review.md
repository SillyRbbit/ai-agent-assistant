# Dashboard CI TMPDIR readiness post-increment review

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "dashboard-ci-tempdir-readiness",
  "quality_gate": "PASS WITH ADVISORIES",
  "next_increment_readiness": "Ready with advisories",
  "commands_executed": [
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "python3 -B .codex/hooks/session_end_gate.py"
  ],
  "files_changed": [
    "DECISIONS.md",
    "docs/plans/2026-10-28-dashboard-current-main-integration.md",
    "docs/reviews/2026-10-28-dashboard-ci-tempdir-readiness-post-increment-review.md"
  ],
  "findings": [],
  "verification": [
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
      "command": "git diff --check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -B .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    }
  ],
  "manual_verification": [
    {
      "check": "Preservation and scope review",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Architecture, security, code-health and readiness review",
      "required": true,
      "status": "Passed"
    }
  ]
}
-->

Date: 2026-10-28
Increment: dashboard-ci-tempdir-readiness
Branch: codex/dashboard-main-integration

## Executive summary

Bounded CI fixture compatibility maintenance. Required fresh evidence passed; local automated acceptance retains advisories.

## Scope and boundaries

Only optional TMPDIR selection and canonical root resolution in the disposable pinned-publication self-test, plus additive task documentation. No production, validator, workflow, dependency or assertion changes. Preserve predecessor reports/state at commit5fcd348 and private PREDECESSOR.raw.json.

## Verification results

Preparation checks passed in ci-tempdir-01/readiness-checks-02 with actual exit0/wait0 and complete cleanup. Earlier npm run verify remains bound to input tree bd43527345dae14617c5c5827b0426c7a8d0ae67 and integration commit5fcd348. Application/package/lock/tool inputs are unchanged; its application-build evidence is reused only for those inputs, never claimed as a fresh full run. Failed GitHub run40782694245 stays failed.

## Architecture findings

Only disposable fixture construction changes. Deterministic runtime, dashboard/voice routing, typed IPC and lifecycle ownership remain unchanged. No new abstraction or helper.

## Security findings

Both local environment variants select monitored build/tmp; absent TMPDIR sets TEMP. CI uses supported tempfile selection. Assertions, no-follow readers, history hashes, production validation and resource/cleanup guards remain intact. No credentials, network expansion or policy change.

## Code-health findings

Two setup expressions resolve concrete Linux KeyError and canonicalize temporary-root aliases. No production fallback, skip or assertion waiver. Existing pinned and complete hook suites supply meaningful coverage.

## Technical debt

Advisory: native application/accessibility/live-provider QA remains outside scope and unverified. Step B Blocked, exec Deferred, oldest cleanup and historical volume continuity Unverified remain unchanged; this does not block bounded CI correction or establish live readiness.

## Roadmap findings

Finish only the approved PR #158 correction and conditional squash merge. No later increment selected. Archive/supervisor/census mechanisms stay parked.

## Completion decision

PASS WITH ADVISORIES

## Next-increment readiness

Ready with advisories for the already-authorized conditional PR finish after CI/rules pass; no later milestone admitted.

## Exact files changed

- `DECISIONS.md`
- `docs/plans/2026-10-28-dashboard-current-main-integration.md`
- `docs/reviews/2026-10-28-dashboard-ci-tempdir-readiness-post-increment-review.md`

## Exact commands executed

- `npm run docs:check`: Passed.
- `npm run repository:check`: Passed.
- `npm run security:scan`: Passed.
- `git diff --check`: Passed.
- `python3 -B .codex/hooks/session_end_gate.py`: Passed.
