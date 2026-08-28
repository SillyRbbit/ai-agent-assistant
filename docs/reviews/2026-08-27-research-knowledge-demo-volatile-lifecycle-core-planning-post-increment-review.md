# Research/Knowledge demo volatile lifecycle core planning post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "python3 .codex/hooks/session_end_gate.py"
  ],
  "files_changed": [
    "CHANGELOG.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "docs/increments/research-knowledge-demo-volatile-lifecycle-core-planning.md",
    "docs/plans/2026-08-27-research-knowledge-demo-volatile-lifecycle-core.md",
    "docs/reviews/2026-08-27-research-knowledge-demo-volatile-lifecycle-core-planning-post-increment-review.md"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Roadmap",
      "effort": "Owner decision",
      "milestone": "Before source implementation",
      "risk": "The Rust lifecycle core could begin without a renewed exact owner approval.",
      "severity": "Advisory",
      "summary": "The lifecycle-core plan is ready, but source implementation is not approved."
    }
  ],
  "increment_id": "research-knowledge-demo-volatile-lifecycle-core-planning",
  "manual_verification": [
    {
      "check": "Target-Mac and rendered UI matrix",
      "required": false,
      "status": "Not run"
    }
  ],
  "next_increment_readiness": "Ready with advisories",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {"command": "npm run docs:check", "required": true, "status": "Passed"},
    {"command": "npm run repository:check", "required": true, "status": "Passed"},
    {"command": "npm run security:scan", "required": true, "status": "Passed"},
    {"command": "git diff --check", "required": true, "status": "Passed"},
    {"command": "python3 .codex/hooks/session_end_gate.py", "required": true, "status": "Passed"}
  ]
}
-->

Date: 2026-08-27
Increment: research-knowledge-demo-volatile-lifecycle-core-planning
Branch: main

## Executive summary

PASS WITH ADVISORIES. This documentation-only increment creates one exact,
implementation-ready ExecPlan for a manually stepped, process-local Rust core
around the sealed D-086 Research -> Knowledge workflow. No product source or
runtime behavior changed.

## Scope and boundaries

The plan fixes ownership, the no-input lifecycle interface, finite visible
states, bounded journal, cleanup retention, late-event rejection, fixed
application-owned fixtures, testing, rollback, and stop conditions. It keeps
the core unwired to Tauri and React. No command, event, capability, CSP,
permission, dependency, provider, model, network, credential, tool, approval,
persistence, filesystem, timer, worker, or device-effect path changed.

## Verification results

- `npm run docs:check`: Passed.
- `npm run repository:check`: Passed.
- `npm run security:scan`: Passed.
- `git diff --check`: Passed.
- `python3 .codex/hooks/session_end_gate.py`: Passed; no conflicts or staged
  paths, and only declared planning/current-state/review files are present.
- Target-Mac and rendered UI matrix: Not run; neither runtime nor UI behavior
  changed, so it is not required for this documentation-only increment.

## Architecture findings

PASS. The plan isolates lifecycle ownership in one focused host rather than
widening `AgentOrchestrator` or coupling it to Command Center fixtures. It uses
the existing production `NativeAgentRuntime` and D-086 boundaries, while later
IPC delivery and presentation remain separate increments.

## Security findings

PASS. The plan requires no caller input, retains F-01/F-02 quarantine ownership,
uses only application-owned fixtures, closes errors, redacts identities/content,
and adds no trust-boundary surface. F-07, F-08, and F-12 remain unchanged.

## Code-health findings

PASS. Documentation follows the existing plan/increment/review system, names
the exact future ownership and boundaries, and does not overstate the current
read-only projection as a connected agent workflow.

## Technical debt

None introduced. The existing non-required DMG packaging failure remains a
distribution-scope advisory, not debt created by this planning increment.

## Roadmap findings

Ready with advisories. The exact lifecycle-core source plan is ready for a new
explicit owner approval. The later Tauri lifecycle adapter and Command Center
presentation remain separate and unapproved.

## Completion decision

PASS WITH ADVISORIES

## Next-increment readiness

Ready with advisories. The next task is owner approval of
`2026-08-27-research-knowledge-demo-volatile-lifecycle-core.md`; do not begin
a source gate or implementation until that approval is explicit.

## Exact files changed

- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `docs/increments/research-knowledge-demo-volatile-lifecycle-core-planning.md`
- `docs/plans/2026-08-27-research-knowledge-demo-volatile-lifecycle-core.md`
- `docs/reviews/2026-08-27-research-knowledge-demo-volatile-lifecycle-core-planning-post-increment-review.md`

## Exact commands executed

All commands listed in the machine manifest completed successfully. No product
build, target-Mac runtime check, commit, push, merge, release, or publication
command ran for this documentation-only increment.
