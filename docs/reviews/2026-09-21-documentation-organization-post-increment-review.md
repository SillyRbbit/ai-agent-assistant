# Documentation organization post-increment review

Date: 2026-09-21. Increment: `documentation-organization`.
Baseline: `39947cf9be86f2e36a57946fa1ad7d966b94ce32`.
Workspace: `/private/tmp/cortexa-documentation-organization`, detached HEAD.

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "documentation-organization",
  "quality_gate": "PASS WITH ADVISORIES",
  "next_increment_readiness": "Ready with advisories",
  "files_changed": [
    "ASSISTANT_USAGE.md",
    "CHANGELOG.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "README.md",
    "docs/README.md",
    "docs/plans/2026-09-21-documentation-organization.md",
    "docs/reviews/2026-09-21-documentation-organization-post-increment-review.md",
    "docs/workflows/ASSISTANT_USAGE.md",
    "docs/workflows/README.md",
    "scripts/repository_health.py"
  ],
  "commands_executed": [
    "PATH=/private/tmp/cortexa-demo-cleanup-main-reconciliation/node_modules/.bin:$PATH npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "python3 -B -m unittest discover -s scripts/tests -p test_repository_health.py -v",
    "python3 -B .codex/hooks/session_end_gate.py",
    "python3 -B -"
  ],
  "verification": [
    {
      "command": "PATH=/private/tmp/cortexa-demo-cleanup-main-reconciliation/node_modules/.bin:$PATH npm run docs:check",
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
      "command": "python3 -B -m unittest discover -s scripts/tests -p test_repository_health.py -v",
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
      "check": "Byte-identical moved guide; no internal link or image target requires rebasing; no duplicate current copy",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "One checker path substitution; temporary valid/stale prompt probe proves retained validation coverage",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "All current references resolve; remaining old locations are unchanged historical records or explicit relocation notes",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Exact 13-path scope, all existing document history and predecessor worktree snapshots preserved",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Architecture, security, code-health, debt, readiness and quality review of the final candidate",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Native GUI smoke and Research/Knowledge lifecycle controls (unchanged advisory)",
      "required": false,
      "status": "Manual verification pending"
    }
  ],
  "findings": [
    {
      "category": "Code health",
      "severity": "Advisory",
      "summary": "Native GUI smoke remains pending; this task does not supply native evidence.",
      "risk": "Prior browser evidence does not prove native WebView behavior.",
      "effort": "Small",
      "milestone": "Separately authorized owner native walkthrough",
      "blocks_completion": false,
      "blocks_next_increment": false
    }
  ]
}
-->

## Executive summary

Moved one non-authoritative assistant guide into the existing workflow directory
without changing its bytes. Added a concise documentation index and root/workflow
links. A single checker pathname follows the relocated guide under unchanged
rules. Required checks passed; result: PASS WITH ADVISORIES. Leave the exact
13-path change uncommitted, with every earlier checkout preserved.

## Scope and boundaries

The [plan](../plans/2026-09-21-documentation-organization.md) records the frozen
scope and root-retention reasons. Root authorities, AGENTS scope, licensing,
skills, hooks, GitHub templates, historical decisions and finalized evidence
stay in place. Only the named guide moves. Historical old-path inventories are
not rewritten; the index identifies their former location. No product behavior,
dependency, runtime security, permissions, gate rules or roadmap status changes.
D-125/M1/M2 remain parked. No branch, commit, push or publication was performed.

## Verification results

Passed: documentation formatting and links; repository policy; secret scan;
whitespace; session inventory; all 51 existing repository-health regressions.
Existing Prettier was reused through PATH without a dependency installation.

Passed: source comparison proves the checker differs only in one string literal.
The temporary valid/stale prompt probe confirms the moved guide is still checked.
The first probe attempt failed because its synthetic prompt directory contained
no reusable prompt. The checker correctly rejected that incomplete fixture.
One correction to the temporary fixture made both positive and rejection cases
pass; no repository code was changed in response. This initial failed probe is
not reported as passing.

Passed: the guide is byte-identical to the baseline, has no Markdown/image/HTML
link requiring rebasing and still instructs commands to run from the checkout
root. There is no remaining root copy. Active index and tooling references use
the new location; older bare filenames remain only in historical plans,
review manifests/commands and backups, or explicit relocation notes.

Passed: exact 13-path inventory, retained old bodies in all five current-state
documents, unchanged protected paths and every predecessor worktree snapshot
(HEAD, dirty inventory, tracked/untracked file hashes and gate-state JSON).
Original seven unpublished commits, dirty changes, prior completion markers and
the terminal-failed dependency record remain intact. Existing prunable worktree
registrations were left alone.

Not run: unrelated frontend/Rust suites, builds, dependency installation and
native GUI smoke. The sole executable diff is a document lookup string; the
existing checker tests plus path probe and repository checks cover its effect.
No trust/language/packaging boundary changed. Native GUI evidence remains a
non-blocking inherited advisory, not a requirement of this organization task.

## Architecture findings

None. Document ownership and current/planned distinctions remain intact.
No runtime, coupling, interface, abstraction, performance or dependency change.

## Security findings

None. Prompt validation still examines the same guide at its new location;
no validation rule, hook, instruction precedence, permission, credential,
networking, IPC, logging or device boundary changed. Secret scan passed.

## Code-health findings

None blocking. The guide is preserved exactly and active navigation is concise.
The checker is identical except for its pathname and all 51 regressions passed.
Historical evidence remains immutable rather than being treated as live links.
No uncertain duplicate was deleted and no competing current copy remains.

## Technical debt

Existing Advisory: pending native GUI smoke. Owner: project owner. Risk: browser
checks do not prove native behavior. Effort: Small. Milestone: separately approved
native walkthrough. Blocks this completion or next review: no. No new debt found.

## Roadmap findings

No milestone or task priority changed. D-125/M1/M2 stay parked. Ready with
advisories only for owner review of these uncommitted changes, not publication,
new implementation or a governance successor.

## Completion decision

PASS WITH ADVISORIES. All applicable checks and preservation reviews passed.
Finalize the ordinary marker and validate status/Stop against this frozen report.
Do not commit, push, publish or automatically execute another task.

## Next-increment readiness

Ready with advisories for read-only review of the uncommitted organization.
The work is complete within its bounded scope; any later publication or further
relocation needs separate owner direction. Native GUI remains pending.

## Exact files changed

- `ASSISTANT_USAGE.md`
- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `README.md`
- `docs/README.md`
- `docs/plans/2026-09-21-documentation-organization.md`
- `docs/reviews/2026-09-21-documentation-organization-post-increment-review.md`
- `docs/workflows/ASSISTANT_USAGE.md`
- `docs/workflows/README.md`
- `scripts/repository_health.py`

The old/new guide paths count separately in the gate's unstaged/untracked
inventory. Their bytes are identical; this is one move, not lost content.

## Exact commands executed

- Passed: `PATH=/private/tmp/cortexa-demo-cleanup-main-reconciliation/node_modules/.bin:$PATH npm run docs:check`.
- Passed: `npm run repository:check`.
- Passed: `npm run security:scan`.
- Passed: `git diff --check`.
- Passed: `python3 -B -m unittest discover -s scripts/tests -p test_repository_health.py -v`.
- Passed: `python3 -B .codex/hooks/session_end_gate.py`.

`python3 -B -` ran the inline temporary probe and source/preservation assertions
summarized above; the first incomplete-fixture invocation failed and its corrected
invocation passed. The exact scope/preservation assertions also passed.
The first finalization rejected an unsupported finding category in this report.
The metadata was corrected to the existing `Code health` category; no gate code
or validation rule changed. Required affected checks run again before retry.
Ordinary admission passed with
`python3 -B .codex/hooks/post_increment_gate.py begin --increment documentation-organization`.
The final documentation edits are checked again before finalization; marker and
Stop outcomes are inspected afterward without reopening historical reports.
