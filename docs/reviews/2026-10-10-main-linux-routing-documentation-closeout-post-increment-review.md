# Main Linux routing documentation closeout review

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "main-linux-routing-documentation-closeout",
  "quality_gate": "PASS WITH ADVISORIES",
  "next_increment_readiness": "Blocked",
  "commands_executed": [
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "python3 -B .codex/hooks/session_end_gate.py"
  ],
  "files_changed": [
    "CHANGELOG.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "docs/github/SELF_HOSTED_RUNNER.md",
    "docs/plans/2026-10-10-main-linux-routing-documentation-closeout.md",
    "docs/reviews/2026-10-10-main-linux-routing-documentation-closeout-post-increment-review.md",
    "docs/reviews/2026-10-10-main-linux-routing-documentation-closeout-readiness-post-increment-review.md"
  ],
  "findings": [
    {
      "category": "Technical debt",
      "severity": "Advisory",
      "summary": "Original temporary failure directory is unavailable; D-136 has stale bindings and remains unconsumed.",
      "risk": "Historical disappearance cause is unknown; executing the stale request would misbind evidence. Verified durable copies preserve recorded bytes.",
      "effort": "Retain original history and durable hashes; no restoration or retry for this documentation objective.",
      "milestone": "Historical evidence retention",
      "blocks_completion": false,
      "blocks_next_increment": false
    },
    {
      "category": "Roadmap",
      "severity": "Advisory",
      "summary": "Documentation publication and further deployment/DR/parallel work require separate owner authority.",
      "risk": "Fixed-SHA CI acceptance does not prove deployment, outage handling, added Linux capacity or future host health.",
      "effort": "Owner review and a separately scoped objective; no automatic continuation.",
      "milestone": "Documentation publication review; separate infrastructure objectives",
      "blocks_completion": false,
      "blocks_next_increment": true
    }
  ],
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
  ],
  "milestone": {
    "criteria": {
      "records": {
        "status": "live_verified",
        "evidence": "Complete documentation/evidence review: six additive current records and this report reconcile original FAIL, separate completed remediation, PR141/142 and sealed exact-SHA Actions results. evidence-reconciliation-001.json binds four unchanged routing sources and retained run/log hashes."
      },
      "preservation": {
        "status": "automatically_verified",
        "evidence": "implementation-preservation-001.json: 2868 protected bindings and 1100 source files unchanged; six prior bodies byte-identical suffixes; index/refs/worktrees unchanged; original FAIL and unconsumed D136 preserved."
      },
      "validation": {
        "status": "automatically_verified",
        "evidence": "implementation-*-result.json and implementation-docs-retry-001-result.json: all five required commands returned zero on the implemented nine-document input, with command, fingerprint and Python/Prettier environment. Final report/plan freeze receives mandatory final-current-input checks before finalization."
      }
    },
    "paths": {
      "HANDOFF.md": {
        "criterion": "records",
        "rationale": "Add current evidence attribution while preserving prior dated bodies.",
        "within_objective": true,
        "preserves_existing": true
      },
      "PROJECT_STATUS.md": {
        "criterion": "records",
        "rationale": "Add current evidence attribution while preserving prior dated bodies.",
        "within_objective": true,
        "preserves_existing": true
      },
      "NEXT_STEPS.md": {
        "criterion": "records",
        "rationale": "Add current evidence attribution while preserving prior dated bodies.",
        "within_objective": true,
        "preserves_existing": true
      },
      "PLANS.md": {
        "criterion": "records",
        "rationale": "Add current evidence attribution while preserving prior dated bodies.",
        "within_objective": true,
        "preserves_existing": true
      },
      "CHANGELOG.md": {
        "criterion": "records",
        "rationale": "Add current evidence attribution while preserving prior dated bodies.",
        "within_objective": true,
        "preserves_existing": true
      },
      "docs/github/SELF_HOSTED_RUNNER.md": {
        "criterion": "records",
        "rationale": "Add current evidence attribution while preserving prior dated bodies.",
        "within_objective": true,
        "preserves_existing": true
      },
      "docs/plans/2026-10-10-main-linux-routing-documentation-closeout.md": {
        "criterion": "validation",
        "rationale": "Maintain this task plan and consolidated observed-evidence review under the frozen documentation objective.",
        "within_objective": true,
        "preserves_existing": true
      },
      "docs/reviews/2026-10-10-main-linux-routing-documentation-closeout-post-increment-review.md": {
        "criterion": "validation",
        "rationale": "Maintain this task plan and consolidated observed-evidence review under the frozen documentation objective.",
        "within_objective": true,
        "preserves_existing": true
      }
    }
  }
}
-->

Date: 2026-10-10 (America/New_York). Documentation-only closeout on branch `codex/docs/main-linux-routing-closeout`, baseline `026586fdbaa415e16714ed37c109fe3ffc090bfe`.

## Executive summary

Additive records reconcile completed local remediation, PR #141/#142 publication and actual exact-SHA Actions acceptance. Original routing **FAIL / Blocked** remains historical and immutable. This task uses main's D-134 schema-2 admission and ordinary completion; neither D-136 execution nor broader D-137 adoption occurs. This review records observed results; final completion is established by the checkout-local finalize/status/full Stop receipts, not a prospective assertion.

## Scope and boundaries

Six existing current-state documents receive additive entries; their original bodies remain byte-identical. The task-owned plan and final review may evolve; the corrected readiness review is frozen at admission. All executable paths, original dated reviews, dependencies, caches, source worktree, index, branches and other worktrees are preserved. No compilation, full verification, Actions operation, commit, push, PR, merge, runner or host change. The prepared ID is `main-linux-routing-documentation-closeout`; the owner's shorter doc-closeout label refers to this same existing task.

Known failed criteria are explicitly assessed in the plan: original required verification, pending quality review, compiler finding, blocked validation, composite preservation/publication and implemented-only documentation remain in their historical records. This task independently records later sealed results, rather than rerunning or passing those original criteria. Empty checkout-local dependencies reflect no local closure state; no live state is copied, converted or evaded.

## Verification results

The corrected readiness category changed only `Documentation` to supported `Technical debt`. The failed `admission-result.json`, old request and before-images remain in `pre-repair-002`; the corrected request `admission-request-002.json` used a refreshed fingerprint and unchanged DECISIONS.md hash. Ordinary begin returned zero. One corrective admission retry and one documentation-format retry were consumed; the latter added separator blank lines before six unchanged original bodies. All failure logs and task-owned before-images remain retained. at most two per failing stage are authorized. A temporary preservation helper initially reversed snapshot ref columns; its corrected comparison passed and the inspection failure is recorded. Neither issue changed repository validators or historical evidence.

Repository/security/whitespace/session implementation checks passed; the documentation check then passed after the recorded separator-only repair. These input-qualified results are followed by all five frozen-current-input completion checks. Fresh checks are retained in `.codex/state/main-linux-routing-doc-closeout-20261010-01`: corrected-readiness receipts passed; implementation/frozen-final receipts distinguish later inputs. Commands, timestamps, result and workspace fingerprint accompany raw logs. Environment: Python 3.12.1 with bytecode disabled, installed Prettier 3.8.4 supplied through PATH from the preserved source installation. No dependency acquisition or cache modification. Full verification, frontend/Rust tests, builds and Actions are **Not run in this task** under the documentation risk tier. Reused evidence is not a new full-verification run.

### Separate completed local remediation

Source `/Users/hdang/.codex/worktrees/vps-main-linux-routing/ai-agent-assistant`, original branch `codex/ci/main-linux-routing`, HEAD `de0d6bcbb63e5377825da4edd6d31bfd98d03dfb`. The distinct `main-linux-routing-local-remediation` successor completed with PASS WITH ADVISORIES. Its retained `receipts/final-status-001.json` says complete/valid, acceptance satisfied; `receipts/final-stop-001.json` records exit zero and empty output within the 30-second bound. They are under `.codex/state/d137-routing-local-remediation-20261009-01`; no such state or executable is adopted here.

`receipts/verification-analysis-001.json` records actual `/opt/homebrew/bin/npm run verify` from its isolated candidate, 721.109 seconds, input SHA-256 `042b26e215404b624b62c52d90ff715a10ff6fd6a50fa8238a9cdd5fe7cd0ef3`, log SHA-256 `254a43849029213fbc2a86bf59e16a4b7c4c53113d9b04431ed7c23ebec1a29c`. Pinned CommandLineTools SDK 27, offline Rust 1.90, task-specific Python routes, process-local strip overrides and a scratch Cargo target bound that environment. Recorded counts: 601 frontend; 159 hook; 98 repository tests including 61 total policy-module tests (not 61 routing-only tests); 714 unique passing native cases across 1,173 executions, including 459 repeats; one unique ignored case. This historical macOS result applies only to its unchanged relevant inputs. Four routing files match both preserved source and this main baseline; PR #142's later test-fixture input uses its separate same-SHA evidence.

### Publication and actual GitHub Actions

[PR #141](https://github.com/SillyRbbit/ai-agent-assistant/pull/141) merged at `4c8c935dacbe0d673fdc40dc6bde1d2911bec572`; [PR #142](https://github.com/SillyRbbit/ai-agent-assistant/pull/142) merged at the current exact baseline. The former's retained CI run `38042081928` recorded 600 passing frontend tests and one timeout failure. Later unobserved outcomes in its handoff remain unknown. The fixture correction and later acceptance do not rewrite that run.

Retained [CI run 38099448267](https://github.com/SillyRbbit/ai-agent-assistant/actions/runs/38099448267), workflow `314196304`, event `workflow_dispatch`, main attempt 1, actor SillyRbbit: success at exact SHA `026586fdbaa415e16714ed37c109fe3ffc090bfe`. Its separately authorized single dispatch is historical; this closeout performs no dispatch or remote observation.

| Job                                  | Job ID       | Runner ID/name          | Conclusion |
| ------------------------------------ | ------------ | ----------------------- | ---------- |
| Classification/policy                | 114352155183 | 24 / cortexa-vps        | success    |
| Frontend                             | 114352228945 | 24 / cortexa-vps        | success    |
| Dependency/secret audit              | 114352228953 | 24 / cortexa-vps        | success    |
| Linux Rust                           | 114352228981 | 24 / cortexa-vps        | success    |
| Target-Mac Rust                      | 114352229005 | 22 / Henrys-MacBook-Pro | success    |
| Reused Documentation run 38098283934 | 114348688217 | 24 / cortexa-vps        | success    |

All six required jobs ran at the same exact SHA. Five complete CI logs match their retained hashes. Frontend: 601 passing tests/40 files; Linux Rust: 693 passing executions/one ignored; Mac Rust: 714 passing executions/one ignored. Together 1,407 native passing executions cover 715 distinct target/case pairs, with 692 shared; they are not 1,407 unique tests. Published-main repository/hook checks recorded 98/145 tests. Audit passed with the configured accepted Rust advisory baseline, not an absence-of-advisories claim. Same-SHA correction CI `38098283938` and Documentation logs remain separate retained evidence. UTC October 11 events occurred October 10 in America/New_York.

## Architecture findings

Complete diff and four unchanged routing sources reviewed. Exact main ref chooses cortexa-linux; development refs keep cortexa-ci. Mac selector, triggers, permissions, action SHAs, job steps and behavior are unchanged. No runtime/provider/device authority, orchestration or product capability is added. Local macOS verification, manual VPS checks and actual Actions are distinguished. PASS for this documentation architecture boundary; no independent agent review is claimed.

## Security findings

Complete content and protected paths reviewed. Only public IDs, sanitized provenance and hashes are added; no credentials, raw job logs or private content enter tracked documents. Existing metadata/account/firewall validation is inherited, not re-executed. Runner labels are not isolation or priority; PR #139's jobs and failed macOS diagnostic are untouched. No validator, permission, service, settings, dependency or host change. Local receipt hashes provide workflow integrity, not authentication against malicious same-user rewriting. PASS for inspected documentation scope.

## Code-health findings

The current six records consistently supersede old pending assertions without rewriting dated bodies. Links point to existing plans/reviews and public run/PR pages; all nine paths serve the frozen contract. Task plan includes bounded recovery; readiness remains protected after begin. No unsupported historical-cause, fresh full-verify or deployment claim. PASS for scoped engineering review; full application testing is outside this change class.

## Technical debt

The original `/private/tmp/cortexa-main-linux-routing-2_pp7r9y` directory is unavailable. Cause remains unknown. Durable `/Users/hdang/.codex/backups/cortexa-routing-retry-admission-4gttb41d` retains 1,091 workspace and 463 state/evidence entries, including all recorded original temporary bytes. Its snapshot.json hash is `11937e0a3dc90854d4a7b20c652746bdad52d61ce1d097eea716877d10285086`. Original failed raw-state hash `a3c4fb93f99078c5aa3177ea58338c548ab2635c4aed5978146c539c57754a53`; original report hash `d10ee53b8bf74f73032671629d741930651918e90637d9e29178f6a9b72603b0`. They remain unchanged and no original completion marker is issued.

D-136 preparation ID `bdd504a802673a7bce133eec33e15f046a579cd38df2951d592281a0b3627d54`, request hash `d5d1e3c7e8bda124bab95f3507f22649a02c2cfff6e9fbee1b041b917ec7b2c7`: sealed, unconsumed; admission/execution approvals remain false. Current candidate/decision bindings differ and its original temporary bound path is unavailable. Preserve archival integrity; never execute this stale request. No restoration or new exception is needed for this documentation acceptance.

Task snapshot `/Users/hdang/.codex/backups/cortexa-routing-doc-closeout-20261010-01/preservation.json`, SHA-256 `1394fb5fc33c2b59da9f5a053db42b541d55006eddc6e419454dd0a6ea99ea10`, binds 1,092 base files, 1,100 source files and 2,868 protected files plus index/refs/worktrees. Comparisons passed; prior document bodies are preserved verbatim. Historical publication strict-comparison failure, Finder metadata change and missing info/packs historical-byte binding stay qualified, without a blanket Git-metadata or mmap-cause claim.

Publication package: `/Users/hdang/.codex/backups/cortexa-routing-publication-20261010-01`. Operational package: `/Users/hdang/.codex/backups/cortexa-main-routing-actions-acceptance-20261011T004243Z-01`; its verified 85-member `evidence-final-001.zip` SHA-256 is `d64b808c091544b7cb34840dafc4eeab7ba9ae790bcaf7050522c1acb86deafa`. All recorded manifest members remain protected; archive and five CI logs were rehashed. Advisory debt is inherited and does not block this bounded documentation objective.

## Roadmap findings

Publication of these uncommitted documents needs separate owner authority. Deployment, DR outage handling/drills, additional parallel Linux capacity, Word/PDF deployment runbooks and broader D-137 adoption remain separate and unverified. The retained weekly-disk/idle-only disposable-cleanup/trusted-cache/no-admin-credentials policy does not authorize maintenance execution. No product roadmap is reordered or next milestone automatically admitted.

## Completion decision

PASS WITH ADVISORIES based on observed implementation checks, preservation and scoped review. This report's result is limited to the new documentation objective. Finalize only after the frozen current-input checks, preservation and both scoped engineering reviews pass. Ordinary D-134 finalize/status/full Stop receipts under the task evidence directory establish terminal COMPLETE/valid; no original FAIL is reopened, relabeled or supplied a completion marker. Rollback is not automatic: use verified task-owned before-images only under separately attributed restoration authority, preserving evidence and unrelated work.

## Next-increment readiness

Blocked for publication or further execution until separately authorized. A read-only owner review is the next proposal; fixed-SHA operational acceptance requires no repeated Actions. Current documentation completion does not grant Git or infrastructure authority.

## Exact files changed

Six additive current-state paths: HANDOFF.md, PROJECT_STATUS.md, NEXT_STEPS.md, PLANS.md, CHANGELOG.md and docs/github/SELF_HOSTED_RUNNER.md. Three task records: dated closeout plan, frozen readiness review and this consolidated report, exactly as the machine inventory. Post-admission attribution excludes the unchanged readiness report; its admission freeze is separately enforced. No other tracked/untracked path belongs to this diff.

## Exact commands executed

The five commands in the machine manifest ran during corrected readiness. Implementation results and final frozen-input receipts distinguish their later execution. Ordinary schema-2 begin used admission-request-002.json. Preservation comparison and evidence reconciliation were read-only with ignored receipts. Full verification/build/tests/Actions were not executed here. Finalization, status and full JSON-payload Stop are captured separately without modifying the frozen report or plan.
