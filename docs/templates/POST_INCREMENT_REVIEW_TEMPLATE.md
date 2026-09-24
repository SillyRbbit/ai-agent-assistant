# Increment N post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "executed command"
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
      "command": "executed command",
      "required": true,
      "status": "Failed"
    },
    {
      "command": "skipped command",
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

## Scope and boundaries

State the approved goal, non-goals, trust-boundary impact, and whether the exact changed-file inventory stayed within scope.

## Verification results

Run `python3 .codex/hooks/session_end_gate.py` before review. Record every required automated and manual check as exactly one of: `Passed`, `Failed`, `Not run`, or `Manual verification pending`. Do not infer success from an earlier run.
List only commands actually run in `commands_executed`. An automated check marked
`Not run` belongs in `verification` but must be absent from `commands_executed`;
required `Not run` checks block completion.

## Architecture findings

Record the `$architecture-review` result for trust boundaries, ownership, coupling, cohesion, drift, abstractions, complexity, portability, maintainability, performance, and dependency health.

## Security findings

Record the `$security-review` result for hooks, IPC, capabilities, CSP, approval and policy boundaries, unsafe Rust, secrets, logs, audit data, SQLite, filesystem and operating-system access, networking, credentials, and permissions.

## Code-health findings

Record the `$code-review` result for naming, organization, type safety, errors, tests, accessibility, dead code, documentation, complexity, and duplication.

## Technical debt

Record the `$technical-debt` result. For each item, include category, severity, risk, effort, milestone, and whether it blocks completion or the next increment. Use `None` when no item exists.

## Roadmap findings

Record the `$readiness-review` evidence. Do not reorder `NEXT_STEPS.md` without approval.

## Completion decision

Use exactly `PASS`, `PASS WITH ADVISORIES`, or `FAIL`.
Only a passing ordinary increment may receive a completion marker. A truthful
`FAIL` must be recorded with `close-failed`; it remains terminal failure
evidence and never becomes completion. The sole D-098 same-terminal-record
recovery may have a passing recovery report but must instead use the
argument-free `record-failed-disposition` command. It preserves the predecessor
`failed` / `FAIL` / `Blocked` evidence and writes no completion marker.

## Next-increment readiness

Use exactly `Ready`, `Ready with advisories`, or `Blocked` and state the exact next task.
If any finding has `blocks_next_increment: true`, this value must be `Blocked`.
Readiness never substitutes for explicit owner approval.

## Exact files changed

List every tracked and untracked changed path. The list must match the machine manifest and complete Git change set.

## Exact commands executed

List every command and its actual result. The machine manifest must contain each required verification command verbatim.
