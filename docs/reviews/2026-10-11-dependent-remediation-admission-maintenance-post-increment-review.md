# Dependent-admission maintenance review

Date: 2026-10-11. Maintenance result is separate from original FAIL.

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "npm run verify",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "python3 -B .codex/hooks/session_end_gate.py"
  ],
  "files_changed": [
    ".codex/hooks/lifecycle_dependent_admission.py",
    ".codex/hooks/post_increment_gate.py",
    ".codex/hooks/tests/test_lifecycle_dependent_admission.py",
    ".prettierignore",
    "AGENTS.md",
    "CHANGELOG.md",
    "DECISIONS.md",
    "ENGINEERING_GUIDE.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "TROUBLESHOOTING_LOG.md",
    "docs/plans/2026-10-11-dependent-remediation-admission.md",
    "docs/reviews/2026-10-11-dependent-remediation-admission-maintenance-post-increment-review.md",
    "docs/reviews/2026-10-11-dependent-remediation-admission-post-increment-review.md",
    "docs/reviews/2026-10-11-dependent-remediation-admission-readiness-post-increment-review.md"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Roadmap",
      "effort": "Small",
      "milestone": "Separate publication review and separately approved real remediation",
      "risk": "Running the real task on an old checkout or without fresh bindings and separate owner authority would bypass the reviewed boundary. Missing historical custody remains unverified.",
      "severity": "Advisory",
      "summary": "Real dependent remediation remains unadmitted and the governance candidate is unpublished."
    }
  ],
  "increment_id": "dependent-remediation-admission-maintenance",
  "manual_verification": [
    {
      "check": "Architecture, security, code-health and readiness review",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Preservation and scope review",
      "required": true,
      "status": "Passed"
    }
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {
      "command": "npm run verify",
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
  ]
}
-->

## Executive summary

PASS WITH ADVISORIES for separately owner-authorized pre-admission maintenance. The original dependent-remediation-admission remains failed / FAIL / Blocked without a completion marker. This report is passing maintenance evidence only. Closeout additionally requires the existing sealed closure receipt and an actual non-blocking complete-payload Stop, recorded separately after this report freezes.

## Scope and boundaries

Prospective task-bound owner amendment was recorded before repair, without ordinary begin or self-admission by the unfinished validator. The prospective path correction permits a safe absent report; final report validation remains strict. State/report/index snapshot anchors, immutable history, exact input/scope bindings, carried obligations, separate approvals, stale evidence/replay and interruption/terminal-failure rejection are tested. Only necessary validator/gate/test/guidance/milestone paths changed. The later owner separately approved one .prettierignore path for the immutable original FAIL report; no other exclusion was added.

## Verification results

Fresh aggregate npm run verify: exit 0 in 678.6 seconds; no source delta during the run. It includes format/Rust fmt, repository policy, frontend lint, strict Clippy, 172 hook tests (including 27 dependent tests), 98 repository tests, frontend and Rust tests, typecheck/frontend build and Tauri no-bundle build. The fresh separate docs/repository/security/whitespace/session checks pass. verify-inputs-004.json, verify-environment-004.json, verify-result-004.json and verify-004.log bind command, source, installed environment and result. Final report/current-state edits are documentation only: compare full-run input bindings to final inputs and rerun affected formatting/documentation/policy/security/whitespace/session checks. Unchanged application/hook build stages are reused with explicit provenance; this is not a new full aggregate run at the final documentation fingerprint. No unverifiable inherited pass substitutes for the actual exit-zero aggregate.

## Architecture findings

PASS for inspected complete diff: local Python gate validation only, no model/WebView/executor/device/product/runtime/runner changes. Existing general independent schema-2 admission and D-133 retain their boundaries and passing regressions. Schema 5 only carries this exact dependent documentation lineage; D-137 is not adopted globally. Existing lifecycle_closure.py remains unchanged. No bootstrap chain, approval framework or exception registry is introduced.

## Security findings

PASS for safe paths, strict JSON/fields/hashes, immutable original raw state/report, required snapshot state/report/index anchors, root/branch/HEAD/index/workspace bindings, protected input and exact documentation scope. Separate admission/execution/acceptance-change approvals bind decision bytes. Original criteria and all unmet required commands/reviews/obligations carry forward. Duplicate keys, symlink/nonregular paths, missing copies/custody, overclaims, omitted obligations, stale input/index/evidence, tampered seals/pointer, mixed D-133 and replay reject. Orphan seals prevent changed-request replay; interruption after consumption cannot reopen the original. Failed dependent attempts remain terminal with no automatic successor. Same-user seals are integrity evidence, not malicious-same-user authentication. No credentials or external authority are added.

## Code-health findings

PASS: 18 existing dependent tests and nine added boundary/interruption tests reach intended assertions; full hooks include all 27. Independent/general closure and D-133 regressions pass. gate.\_safe_report_path remains unchanged. The precise formatter conflict was resolved only by the owner-authorized immutable historical path exclusion; the original report bytes/hash still match. Previously failed environment preparations are corrected in scope without application source changes or lint relaxation.

## Technical debt

No new blocking finding. Historical raw FAIL/report, preparation-format failure, original index and D-136 remain preserved. Existing missing sealed adoption/correction/validation packages, complete retired ignored-state custody and original retired indices remain unavailable evidence; do not infer complete preservation, permanent loss or absence of loss. Prior blocked maintenance report versions and all rejected receipts remain in verified snapshots/task custody. Existing original unresolved criteria remain in the closure receipt rather than being erased by this maintenance pass.

## Roadmap findings

Real vps-dual-label-operating-mode-remediation admission/execution approvals remain false; it is not admitted or executed. Candidate remains local/unpublished. Publication and a real request with fresh exact bindings require separate owner authority; this report does not select another milestone. Deployment, DR, extra Linux capacity, PR139 and Actions remain separate.

## Completion decision

The separately passing maintenance report meets verification/review prerequisites. Under the exact owner amendment, freeze it and call existing close-without-completion with immutable original failure bindings, recoverable historical snapshot, exact maintenance delta and all historical unresolved IDs. After a valid receipt, run status and actual full Stop. Success is established only by passing closeout receipts under .codex/state/dependent-remediation-admission-maintenance-20261011-01; do not call ordinary begin/finalize/close-failed, change original FAIL or issue its completion marker.

## Next-increment readiness

Blocked for real dependent remediation until separately approved publication/prerequisites and real admission/execution with current bindings. This advisory does not block the bounded maintenance closeout. The next proposed work is a read-only governance publication-readiness review, with no automatic commit or real attempt.

## Exact files changed

- .codex/hooks/lifecycle_dependent_admission.py
- .codex/hooks/post_increment_gate.py
- .codex/hooks/tests/test_lifecycle_dependent_admission.py
- .prettierignore
- AGENTS.md
- CHANGELOG.md
- DECISIONS.md
- ENGINEERING_GUIDE.md
- HANDOFF.md
- NEXT_STEPS.md
- PLANS.md
- PROJECT_STATUS.md
- TROUBLESHOOTING_LOG.md
- docs/plans/2026-10-11-dependent-remediation-admission.md
- docs/reviews/2026-10-11-dependent-remediation-admission-maintenance-post-increment-review.md
- docs/reviews/2026-10-11-dependent-remediation-admission-post-increment-review.md
- docs/reviews/2026-10-11-dependent-remediation-admission-readiness-post-increment-review.md

The pre-existing original FAIL/readiness reports remain unchanged by maintenance. The current Git inventory includes retained pre-maintenance implementation edits. Exact maintenance delta and file hashes are recorded separately against the verified 1100-record snapshot; the single newly authorized formatting-policy path is included.

## Exact commands executed

- `npm run verify`
- `npm run docs:check`
- `npm run repository:check`
- `npm run security:scan`
- `git diff --check`
- `python3 -B .codex/hooks/session_end_gate.py`

Prior focused runs and all full-run attempts remain retained. verify-001 stopped on the immutable report formatting conflict; verify-002 exposed missing marked and older Vitest in the initial copied environment; verify-003 scanned the task-created incomplete dependency copy placed inside the checkout. That copy is retained outside lint traversal with a custody relocation receipt. verify-004 uses an existing byte-identical project lock, all matching declared dependency versions and installed integrity entries, without downloads or installs. Source dependency trees, locks and toolchains remain unchanged; existing caches are reused without deletion or cleanup. Fresh Cargo targets preserve original artifacts; process-local DEV/TEST/RELEASE build-dependency strip=none overrides mitigate the established macOS procedural macro problem. Environment/failed receipt hashes and custody paths are task-local evidence, not credential contents.
