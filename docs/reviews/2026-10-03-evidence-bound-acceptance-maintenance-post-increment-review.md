# Evidence-bound acceptance maintenance post-increment review

<!-- post-increment-gate-manifest

{
  "schema_version": 1,
  "increment_id": "evidence-bound-acceptance-maintenance",
  "quality_gate": "PASS WITH ADVISORIES",
  "next_increment_readiness": "Ready with advisories",
  "files_changed": [
    ".agents/skills/post-increment-gate/SKILL.md",
    ".agents/skills/verified-increment/SKILL.md",
    ".codex/hooks/lifecycle_acceptance.py",
    ".codex/hooks/post_increment_gate.py",
    ".codex/hooks/tests/test_lifecycle_acceptance.py",
    ".codex/hooks/tests/test_post_increment_gate.py",
    "AGENTS.md",
    "CHANGELOG.md",
    "DECISIONS.md",
    "ENGINEERING_GUIDE.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "README.md",
    "TESTING_GUIDE.md",
    "TROUBLESHOOTING_LOG.md",
    "assets/branding/app-icon-source.png",
    "assets/branding/approved-september23.png",
    "assets/branding/favicon.png",
    "assets/branding/logo-dark.png",
    "assets/branding/logo-light.png",
    "assets/branding/logo-primary.png",
    "assets/branding/sidebar-symbol.png",
    "docs/branding/APP_ICON_GENERATION.md",
    "docs/branding/BRAND_GUIDELINES.md",
    "docs/plans/2026-10-03-approved-logo-integration.md",
    "docs/plans/2026-10-03-collapsed-sidebar-reachability.md",
    "docs/plans/2026-10-03-conversations-navigation-layout.md",
    "docs/plans/2026-10-03-evidence-bound-acceptance-maintenance.md",
    "docs/plans/2026-10-03-sidebar-brand-copy-alignment.md",
    "docs/reviews/2026-10-03-approved-logo-integration-post-increment-review.md",
    "docs/reviews/2026-10-03-collapsed-sidebar-reachability-post-increment-review.md",
    "docs/reviews/2026-10-03-conversations-navigation-layout-post-increment-review.md",
    "docs/reviews/2026-10-03-evidence-bound-acceptance-maintenance-post-increment-review.md",
    "docs/reviews/2026-10-03-sidebar-brand-copy-alignment-post-increment-review.md",
    "index.html",
    "scripts/branding_assets.py",
    "scripts/browser/knowledge-check.mjs",
    "scripts/tests/test_branding_assets.py",
    "src-tauri/icons/128x128.png",
    "src-tauri/icons/128x128@2x.png",
    "src-tauri/icons/32x32.png",
    "src-tauri/icons/Square107x107Logo.png",
    "src-tauri/icons/Square142x142Logo.png",
    "src-tauri/icons/Square150x150Logo.png",
    "src-tauri/icons/Square284x284Logo.png",
    "src-tauri/icons/Square30x30Logo.png",
    "src-tauri/icons/Square310x310Logo.png",
    "src-tauri/icons/Square44x44Logo.png",
    "src-tauri/icons/Square71x71Logo.png",
    "src-tauri/icons/Square89x89Logo.png",
    "src-tauri/icons/StoreLogo.png",
    "src-tauri/icons/icon.icns",
    "src-tauri/icons/icon.ico",
    "src-tauri/icons/icon.png",
    "src/App.test.tsx",
    "src/components/ApplicationSidebar.tsx",
    "src/styles.css"
  ],
  "commands_executed": [
    "git diff --check",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "npm run verify",
    "python3 -B .codex/hooks/session_end_gate.py",
    "python3 -B -m unittest discover -s .codex/hooks/tests -p test_*.py -v",
    "python3 -B /private/tmp/cortexa-evidence-acceptance-maintenance-kw8t1_eu/preserve.py"
  ],
  "verification": [
    {
      "command": "git diff --check",
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
      "command": "python3 -B .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -B -m unittest discover -s .codex/hooks/tests -p test_*.py -v",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -B /private/tmp/cortexa-evidence-acceptance-maintenance-kw8t1_eu/preserve.py",
      "required": true,
      "status": "Passed"
    }
  ],
  "manual_verification": [
    {
      "check": "Architecture review",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Code-health review",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Evidence-binding and readiness review",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Preservation and scope review",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Security review",
      "required": true,
      "status": "Passed"
    }
  ],
  "findings": [
    {
      "category": "Technical debt",
      "severity": "Advisory",
      "summary": "Separate formal acceptance and existing platform/provider advisories remain",
      "risk": "Maintenance is not product completion; local hashes are workflow evidence, not malicious same-user authentication. Native QA is inherited and no live-provider behavior is established.",
      "effort": "One separately authorized bounded documentation-only acceptance",
      "milestone": "Collapsed sidebar formal acceptance",
      "blocks_completion": false,
      "blocks_next_increment": false
    }
  ]
}

-->

## Executive summary

Owner-authorized eighteen-path maintenance is verified with advisories; ordinary
admission did not occur. It adds acceptance-only admission without promoting either
historical FAIL. No formal acceptance, product completion or publication is claimed.
The frozen report is input to the subsequent local maintenance seal and full Stop;
actual seal/status/Stop receipts are external and must pass before handoff.

## Scope and boundaries

Worktree: `/Users/hdang/.codex/worktrees/collapsed-sidebar-reachability/ai-agent-assistant`.
Branch: `codex/collapsed-sidebar-reachability`; HEAD unchanged at
`dcea9df088ddb01c26c1a79ab51e0307190f551c`. Exactly 18 maintenance paths and 59
cumulative Git paths. Seven inherited current-state documents gain additive updates;
all other inherited 48 files, product bytes and original historical bodies remain.
No Desktop import, D-098 use, new checkout, app-data change, native launch, provider
request, install, commit or publication. Original raw failure states are unchanged.

## Verification results

New: full offline `npm run verify` passed using the installed process-local
Python/Xcode SDK27/Cargo route, including 105 hook tests, 94 repository tests,
580 frontend tests, 436 native unit tests, integration tests, strict lint/Clippy,
typecheck and frontend/native no-bundle release build. The existing opt-in real-Hermes
contract is ignored, not passed. A later status-label correction was verified by
all 105 hook tests again; product implementation did not change. Required docs,
repository, security, whitespace, session and exact-scope/preservation checks passed.
Final documentation edits receive a fresh relevant check before sealing.

Inherited: the original browser matrix, branding/animation/Rust records and latest
native 760x520 receipt remain intact. Direct Computer Use in that prior receipt
observed expanded/collapsed sidebar scrolling, independent composer/disclosure,
hover/focus tooltips, keyboard/Escape, exact process/window identity and quit/absence.
This maintenance performed no new native or live QA. Owner 760x640 evidence remains
owner-reported; earlier Computer Use failures remain failures.

## Architecture findings

Passed. One small local lifecycle module wraps existing report/state functions;
legacy schemas and D-098 branches retain their behavior and tests. No runtime,
provider, React, IPC, storage or device authority changes. Immutable maintenance
receipt is distinct from schema-v4 acceptance lineage and ordinary completion.

## Security findings

Passed with no unresolved blocking finding. Exact fields, bounded JSON, safe
relative paths, symlink rejection, content-addressed blobs and no-clobber atomic
publication protect evidence integrity. Both historical raw states reconstruct
original fingerprints; reports cannot be rewritten. All required blockers need
resolved owner-reviewed evidence, with no waive/defer path. Artifact files are
hashed in the exact existing bundle; full source coverage uses original artifact
and pre-build transfer receipts. Scope, HEAD, index and owner-decision bindings are
checked. Missing evidence or drift blocks status/Stop. Requests are never executed
as commands and grant no external authority. Local same-user rewriting is explicitly
outside the authentication claim.

Automatic approval rejected dropping three asset paths from source coverage.
That edit was not applied. Read-only inspection verified their hashes in the
original pre-build transfer receipt; the implementation requires that unchanged
receipt as additional evidence and retains every asset requirement.

## Code-health findings

Passed. Thirty focused regressions exercise successful sealing/admission/finalize
in temporary repositories, malformed/unknown/duplicate fields, missing history,
separate approval, incomplete mappings, exact scope, artifact/source drift,
symlink/path escape, interrupted publication, replay and new-failure blocking.
Original historical tests remain. Initial synthetic report quality mismatch was
corrected; its failing log is retained. Review corrected original-workspace status
to compare against the historical baseline even after future acceptance. All
105 hook tests pass on that final code. No unresolved regression found.

## Technical debt

Retained advisories only: separate formal acceptance; local-receipt authentication
limits; inherited D-127/D-128, unsigned/native/provider/runtime/Codex-isolation,
branding/icon/ICNS/Windows/owner-QA and process-local workaround advisories.
Live QA stays parked 3/10; D-125/M1/M2 remain parked. No live success, cancellation
or historical crash/provider-error cause is inferred.

## Roadmap findings

No roadmap reordering. Desktop general closure handles independent successors and
deferred criteria; that policy was not imported for this dependent acceptance.
D-133 permits only the reviewed maintenance seal and separately authorized bounded
acceptance. All four blocker entries remain historical; proposed mappings to the
later native receipt are documented in the plan but are not applied in this task.

## Completion decision

PASS WITH ADVISORIES for maintenance verification. Freeze this report and seal
only after the final checks pass. Verify the actual maintenance/status/full Stop
receipts externally. The source record must still read failed/FAIL/Blocked, valid
with maintenance evidence, no completion marker, and acceptance_started:false.
Do not call ordinary finalize/close-failed or rewrite either raw FAIL.

## Next-increment readiness

Ready with advisories only for separate owner-authorized `collapsed-sidebar-acceptance`.
Scope: CHANGELOG.md, HANDOFF.md, NEXT_STEPS.md, PLANS.md, PROJECT_STATUS.md,
TESTING_GUIDE.md, TROUBLESHOOTING_LOG.md and the new
`docs/plans/2026-10-03-collapsed-sidebar-acceptance.md` plus
`docs/reviews/2026-10-03-collapsed-sidebar-acceptance-post-increment-review.md`.
Nine successor paths / 61 cumulative. Require complete criterion-to-evidence mapping,
separate approval bound to current fingerprint/decision hash, normal documentation
checks/reviews/schema/finalize/status/full Stop. Preserve product and artifact bytes,
all historical bodies and both FAILs. Do not repeat passed product tests/native QA,
begin without approval, change governance or publish.

## Exact files changed

- `.agents/skills/post-increment-gate/SKILL.md`
- `.agents/skills/verified-increment/SKILL.md`
- `.codex/hooks/lifecycle_acceptance.py`
- `.codex/hooks/post_increment_gate.py`
- `.codex/hooks/tests/test_lifecycle_acceptance.py`
- `.codex/hooks/tests/test_post_increment_gate.py`
- `AGENTS.md`
- `CHANGELOG.md`
- `DECISIONS.md`
- `ENGINEERING_GUIDE.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `README.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `assets/branding/app-icon-source.png`
- `assets/branding/approved-september23.png`
- `assets/branding/favicon.png`
- `assets/branding/logo-dark.png`
- `assets/branding/logo-light.png`
- `assets/branding/logo-primary.png`
- `assets/branding/sidebar-symbol.png`
- `docs/branding/APP_ICON_GENERATION.md`
- `docs/branding/BRAND_GUIDELINES.md`
- `docs/plans/2026-10-03-approved-logo-integration.md`
- `docs/plans/2026-10-03-collapsed-sidebar-reachability.md`
- `docs/plans/2026-10-03-conversations-navigation-layout.md`
- `docs/plans/2026-10-03-evidence-bound-acceptance-maintenance.md`
- `docs/plans/2026-10-03-sidebar-brand-copy-alignment.md`
- `docs/reviews/2026-10-03-approved-logo-integration-post-increment-review.md`
- `docs/reviews/2026-10-03-collapsed-sidebar-reachability-post-increment-review.md`
- `docs/reviews/2026-10-03-conversations-navigation-layout-post-increment-review.md`
- `docs/reviews/2026-10-03-evidence-bound-acceptance-maintenance-post-increment-review.md`
- `docs/reviews/2026-10-03-sidebar-brand-copy-alignment-post-increment-review.md`
- `index.html`
- `scripts/branding_assets.py`
- `scripts/browser/knowledge-check.mjs`
- `scripts/tests/test_branding_assets.py`
- `src-tauri/icons/128x128.png`
- `src-tauri/icons/128x128@2x.png`
- `src-tauri/icons/32x32.png`
- `src-tauri/icons/Square107x107Logo.png`
- `src-tauri/icons/Square142x142Logo.png`
- `src-tauri/icons/Square150x150Logo.png`
- `src-tauri/icons/Square284x284Logo.png`
- `src-tauri/icons/Square30x30Logo.png`
- `src-tauri/icons/Square310x310Logo.png`
- `src-tauri/icons/Square44x44Logo.png`
- `src-tauri/icons/Square71x71Logo.png`
- `src-tauri/icons/Square89x89Logo.png`
- `src-tauri/icons/StoreLogo.png`
- `src-tauri/icons/icon.icns`
- `src-tauri/icons/icon.ico`
- `src-tauri/icons/icon.png`
- `src/App.test.tsx`
- `src/components/ApplicationSidebar.tsx`
- `src/styles.css`

## Exact commands executed

- `git diff --check`
- `npm run docs:check`
- `npm run repository:check`
- `npm run security:scan`
- `npm run verify`
- `python3 -B .codex/hooks/session_end_gate.py`
- `python3 -B -m unittest discover -s .codex/hooks/tests -p test_*.py -v`
- `python3 -B /private/tmp/cortexa-evidence-acceptance-maintenance-kw8t1_eu/preserve.py`

Receipts: `/private/tmp/cortexa-evidence-acceptance-maintenance-kw8t1_eu`.
The full verify run used the unchanged offline wrapper. Focused failures and their
successful corrections are retained. Seal/status/Stop are recorded separately
after report freeze; they are not misrepresented as already executed here.
