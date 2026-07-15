# Increment N post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "exact command"
  ],
  "files_changed": [
    "path/to/file"
  ],
  "findings": [],
  "increment_id": "replace-me",
  "manual_verification": [
    {
      "check": "exact manual check",
      "required": true,
      "status": "Manual verification pending"
    }
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "FAIL",
  "schema_version": 1,
  "verification": [
    {
      "command": "exact command",
      "required": true,
      "status": "Not run"
    }
  ]
}
-->

Date: YYYY-MM-DD
Increment: N
Branch: branch-name

## Executive summary

State what changed, whether acceptance criteria are met, and the exact quality-gate result.

## Verification results

Record every required command as exactly one of: `Passed`, `Failed`, `Not run`, or `Manual verification pending`. Do not infer success from an earlier run.

## Architecture findings

Review module boundaries, coupling, cohesion, drift, abstractions, complexity, maintainability, scalability, performance, and dependency health.

## Security findings

Review IPC, capabilities, CSP, approval and policy boundaries, unsafe Rust, secrets, logs, audit data, SQLite, filesystem and operating-system access, networking, and permissions.

## Code-health findings

Review naming, organization, type safety, errors, tests, accessibility, dead code, documentation, complexity, and duplication.

## Technical debt

For each item, record severity, risk, effort, milestone, and whether it blocks completion or the next increment. Use `None` when no item exists.

## Roadmap findings

State whether the next increment is `Ready`, `Ready with advisories`, or `Blocked`. Do not reorder `NEXT_STEPS.md` without approval.

## Completion decision

Use exactly `PASS`, `PASS WITH ADVISORIES`, or `FAIL`.

## Next-increment readiness

Use exactly `Ready`, `Ready with advisories`, or `Blocked` and state the exact next task.

## Exact files changed

List every tracked and untracked changed path. The list must match the machine manifest and complete Git change set.

## Exact commands executed

List every command and its actual result. The machine manifest must contain each required verification command verbatim.
