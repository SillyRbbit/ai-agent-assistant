# Storage inventory boundary correction post-increment review

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "storage-inventory-boundary-correction",
  "quality_gate": "PASS WITH ADVISORIES",
  "next_increment_readiness": "Ready with advisories",
  "commands_executed": [
    "python3 -B -m unittest discover -s scripts/tests -p test_storage_inventory.py -v",
    "/usr/bin/python3 -B -m unittest discover -s scripts/tests -p test_storage_inventory.py -v",
    "npm run test:repository",
    "python3 -B /Users/hdang/.codex/visualizations/2026/10/21/01a16064-1596-7b52-bdcb-90c7155957dc/storage-inventory-boundary-correction-20261021/validate_reused_evidence.py",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "python3 -B .codex/hooks/session_end_gate.py"
  ],
  "files_changed": [
    "CHANGELOG.md",
    "DECISIONS.md",
    "ENGINEERING_GUIDE.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "SECURITY.md",
    "TESTING_GUIDE.md",
    "TROUBLESHOOTING_LOG.md",
    "docs/plans/2026-10-21-storage-inventory-boundary-correction.md",
    "docs/plans/2026-10-21-storage-inventory-checker.md",
    "docs/reviews/2026-10-21-storage-inventory-boundary-correction-post-increment-review.md",
    "docs/reviews/2026-10-21-storage-inventory-boundary-correction-readiness-post-increment-review.md",
    "docs/reviews/2026-10-21-storage-inventory-checker-post-increment-review.md",
    "docs/reviews/2026-10-21-storage-inventory-checker-readiness-post-increment-review.md",
    "docs/workflows/END_SESSION.md",
    "docs/workflows/STORAGE_INVENTORY.md",
    "scripts/storage_inventory.py",
    "scripts/tests/test_storage_inventory.py"
  ],
  "findings": [
    {
      "category": "Code health",
      "severity": "Advisory",
      "summary": "Inherited whole-repository Python3.9 incompatibility remains in unchanged repository_health.py zip(strict=True).",
      "risk": "Do not claim whole-repository3.9 compatibility. Required repository checks use3.12; the corrected checker passes53 cases on both.",
      "effort": "Separate bounded interpreter-policy assessment only if selected by owner.",
      "milestone": "Future owner-selected interpreter compatibility assessment; no automatic work.",
      "blocks_completion": false,
      "blocks_next_increment": false
    }
  ],
  "verification": [
    {
      "command": "python3 -B -m unittest discover -s scripts/tests -p test_storage_inventory.py -v",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "/usr/bin/python3 -B -m unittest discover -s scripts/tests -p test_storage_inventory.py -v",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run test:repository",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -B /Users/hdang/.codex/visualizations/2026/10/21/01a16064-1596-7b52-bdcb-90c7155957dc/storage-inventory-boundary-correction-20261021/validate_reused_evidence.py",
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
      "check": "Complete scope, architecture, security, code-health, debt and readiness review",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Historical source/report/evidence preservation and attribution",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Synthetic actual-Mac alias and private-output outcomes",
      "required": true,
      "status": "Passed"
    }
  ],
  "milestone": {
    "criteria": {
      "aliases": {
        "status": "automatically_verified",
        "evidence": "53 synthetic cases include actual case/physical Mac aliases, hardlinks, Unicode/missing names and same-descriptor private output; final-boundary logs show no skips."
      },
      "holds": {
        "status": "automatically_verified",
        "evidence": "Explicit/required sealed paths, including optional flags, remain output exclusions. Malformed roles preserve valid exclusions; unreadable/ambiguous/noncanonical/uncertain scope refuses creation."
      },
      "schema": {
        "status": "automatically_verified",
        "evidence": "Non-string roles list/dict/null/bool/number yield fixed errors and deferred private reports outside exclusions; no traceback or removal authority."
      },
      "tests": {
        "status": "automatically_verified",
        "evidence": "Fresh53 cases on3.9/3.12 and153 repository tests on3.12; sealed unchanged291 hook/product stages validated through actual provenance checker. Final relevant docs/security/session checks required."
      },
      "preservation": {
        "status": "automatically_verified",
        "evidence": "Complete baseline/source/raw-state snapshot, unchanged original reports/evidence, additive notes, scoped review and normal admission. Final gate/status/Stop receipts follow immutable report freeze externally."
      }
    },
    "paths": {
      "CHANGELOG.md": {
        "criterion": "preservation",
        "rationale": "New correction evidence, updated current utility guidance or additive observed project-state note; original reports retained",
        "within_objective": true,
        "preserves_existing": true
      },
      "DECISIONS.md": {
        "criterion": "preservation",
        "rationale": "New correction evidence, updated current utility guidance or additive observed project-state note; original reports retained",
        "within_objective": true,
        "preserves_existing": true
      },
      "HANDOFF.md": {
        "criterion": "preservation",
        "rationale": "New correction evidence, updated current utility guidance or additive observed project-state note; original reports retained",
        "within_objective": true,
        "preserves_existing": true
      },
      "NEXT_STEPS.md": {
        "criterion": "preservation",
        "rationale": "New correction evidence, updated current utility guidance or additive observed project-state note; original reports retained",
        "within_objective": true,
        "preserves_existing": true
      },
      "PLANS.md": {
        "criterion": "preservation",
        "rationale": "New correction evidence, updated current utility guidance or additive observed project-state note; original reports retained",
        "within_objective": true,
        "preserves_existing": true
      },
      "PROJECT_STATUS.md": {
        "criterion": "preservation",
        "rationale": "New correction evidence, updated current utility guidance or additive observed project-state note; original reports retained",
        "within_objective": true,
        "preserves_existing": true
      },
      "SECURITY.md": {
        "criterion": "preservation",
        "rationale": "New correction evidence, updated current utility guidance or additive observed project-state note; original reports retained",
        "within_objective": true,
        "preserves_existing": true
      },
      "TESTING_GUIDE.md": {
        "criterion": "preservation",
        "rationale": "New correction evidence, updated current utility guidance or additive observed project-state note; original reports retained",
        "within_objective": true,
        "preserves_existing": true
      },
      "TROUBLESHOOTING_LOG.md": {
        "criterion": "preservation",
        "rationale": "New correction evidence, updated current utility guidance or additive observed project-state note; original reports retained",
        "within_objective": true,
        "preserves_existing": true
      },
      "docs/plans/2026-10-21-storage-inventory-boundary-correction.md": {
        "criterion": "preservation",
        "rationale": "New correction evidence, updated current utility guidance or additive observed project-state note; original reports retained",
        "within_objective": true,
        "preserves_existing": true
      },
      "docs/reviews/2026-10-21-storage-inventory-boundary-correction-post-increment-review.md": {
        "criterion": "preservation",
        "rationale": "New correction evidence, updated current utility guidance or additive observed project-state note; original reports retained",
        "within_objective": true,
        "preserves_existing": true
      },
      "docs/workflows/STORAGE_INVENTORY.md": {
        "criterion": "preservation",
        "rationale": "New correction evidence, updated current utility guidance or additive observed project-state note; original reports retained",
        "within_objective": true,
        "preserves_existing": true
      },
      "scripts/storage_inventory.py": {
        "criterion": "aliases",
        "rationale": "Attributable correction of reviewed source boundary defects",
        "within_objective": true,
        "preserves_existing": true
      },
      "scripts/tests/test_storage_inventory.py": {
        "criterion": "tests",
        "rationale": "Attributable correction of reviewed source boundary defects",
        "within_objective": true,
        "preserves_existing": true
      }
    }
  }
}
-->

## Executive summary

Corrects all three reviewed defects in the owner-approved read-only inventory utility. All bounded acceptance and required preflight checks pass. The original Python3.9 repository incompatibility remains an advisory. Final report-input checks precede freeze and supported completion.

## Scope and boundaries

Cumulative candidate contains20 paths; correction delta serves only the approved criteria. Source/test and utility guide are extended with attributable changes; original first-milestone plan, readiness/final reports and78-record sealed evidence remain byte-identical. Nine current-state documents are append-only relative to the correction snapshot. Product, configuration, dependencies, hooks, CI, skills and prompts remain unchanged. Same detached4e3e94d, no Git writes or artifact/cloud/signal/permission/credential/background operation.

## Verification results

FINAL-BOUNDARY-CHECKS.json binds current source/test SHA256 to actual passing command logs:53 focused tests on Python3.12 and3.9, no skips, plus153 repository tests on3.12. Native case/physical Data-volume aliases are tested only on synthetic fixtures. The original37-case suite is preserved and extended by16 regressions; the mount test now supplies complete stat observations.

The actual validate_reused_evidence.py command verifies all78 original evidence records,293 unchanged product inputs and unchanged hook inputs/retained291-test pass. Original full npm run verify final direct exit0, exact log/receipt identities and environment provenance remain bound through VERIFIED-REUSE.json and the original sealed lineage. Product/frontend/Rust/native build stages are inherited; affected tool/tests/docs/checks run freshly. This composite is not a new full verify, build, hook-suite run or native acceptance. Formatting of changed Markdown is checked freshly; unchanged non-Markdown formatter inputs retain their original evidence.

The inherited optional repository3.9 diagnostic exit1/170 errors remains unchanged and is not replayed; required repository validation uses3.12. Checker3.9 compatibility passes independently. Initial correction failure (one duplicate-count assertion and one incomplete mount stub error) is preserved in ITERATION-RESULTS.json; corrected source passes. No failure is promoted or hidden.

Final documentation/repository/security/whitespace/session report-input checks must pass before freeze/finalization. No real QA artifact scan, transfer, restore, deletion, app launch or reclaim occurred.

## Architecture findings

PASS. Reuses the standalone standard-library CLI and canonical END_SESSION authority. Alias helpers observe directory/file identities through no-follow descriptors; no realpath traversal, network, shell, subprocess, product action, generic executor, integration or dependency is introduced. Context validation and conservative output exclusions stay separate from observations/operation authority.

## Security findings

PASS for approved correction. Physical identity anchors plus conservative case/NFC suffix comparison close deterministic aliases, including unresolved held names. Holds, required sealed paths and context candidates/receipts cannot be report destinations. Parsed malformed roles preserve restrictions; ambiguous/unreadable/noncanonical scopes stop creation with fixed errors. Parent checks and exclusive0600 creation share one descriptor, with a final identity comparison. Dataless, symlink, special-file, hardlink and protected-store guards remain. No owner assertion/receipt body/content is emitted; no permission change.

Not an atomic filesystem snapshot or same-user sandbox. Scope metadata may change after point-in-time checks; no continuous quiescence is asserted. Current process/task/runner consumers, worktrees/mount registries, cloud availability, complete recovery coverage and authorization remain unverified. Every report still has removal_eligible=false and removal_authority=false.

## Code-health findings

PASS WITH ADVISORIES. Strict string role checking precedes set membership; invalid JSON/container scope cannot silently drop a known restriction. Bounded scope paths/depth and fixed error categories. Synthetic regressions exercise actual filesystem outcomes, alternate identities, malformed fields, safe partial reports and refusal before creation, rather than only copying implementation predicates. No new dependency or unrelated refactor. The existing Python3.9 repository incompatibility stays qualified.

## Technical debt

Advisory only: existing repository3.9 incompatibility in unchanged zip(strict=True). Risk: misleading whole-repository compatibility claim. Effort: small separate policy assessment if selected. Blocks neither correction completion nor read-only review. Conservative over-retention and point-in-time checks are documented scope limits, not implicit permission for a later executor.

## Roadmap findings

Only these related boundary corrections are authorized. Prior review publication blockers are addressed locally; no publication action or successor implementation is selected. Historical HP/product/M1/M2 failures and all original gate/results remain in their own snapshots; this task neither clears nor adopts them.

## Completion decision

PASS WITH ADVISORIES. The three defects and related unsafe-scope cases are corrected and verified. Actual supported finalization/status and complete-payload Stop receipts are appended externally after report freeze. This report does not validate itself or rewrite historical evidence.

## Next-increment readiness

Ready with advisories for a read-only corrected-candidate review only. No artifact operation, automatic cleanup, commit or publication authority follows. All worktrees and evidence stay retained.

## Exact files changed

- `CHANGELOG.md`
- `DECISIONS.md`
- `ENGINEERING_GUIDE.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `SECURITY.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-10-21-storage-inventory-boundary-correction.md`
- `docs/plans/2026-10-21-storage-inventory-checker.md`
- `docs/reviews/2026-10-21-storage-inventory-boundary-correction-post-increment-review.md`
- `docs/reviews/2026-10-21-storage-inventory-boundary-correction-readiness-post-increment-review.md`
- `docs/reviews/2026-10-21-storage-inventory-checker-post-increment-review.md`
- `docs/reviews/2026-10-21-storage-inventory-checker-readiness-post-increment-review.md`
- `docs/workflows/END_SESSION.md`
- `docs/workflows/STORAGE_INVENTORY.md`
- `scripts/storage_inventory.py`
- `scripts/tests/test_storage_inventory.py`

## Exact commands executed

- `python3 -B -m unittest discover -s scripts/tests -p test_storage_inventory.py -v`
- `/usr/bin/python3 -B -m unittest discover -s scripts/tests -p test_storage_inventory.py -v`
- `npm run test:repository`
- `python3 -B /Users/hdang/.codex/visualizations/2026/10/21/01a16064-1596-7b52-bdcb-90c7155957dc/storage-inventory-boundary-correction-20261021/validate_reused_evidence.py`
- `npm run docs:check`
- `npm run repository:check`
- `npm run security:scan`
- `git diff --check`
- `python3 -B .codex/hooks/session_end_gate.py`

Exact fresh exits and bound inputs are external. Inherited npm run verify/npm run test:hooks/npm run format:frontend commands were not replayed; their qualified lineage is explicit. Normal schema2 begin used .codex/state/storage-inventory-boundary-correction-admission.json. Final actual finalize/status/Stop receipts follow this frozen report externally.
