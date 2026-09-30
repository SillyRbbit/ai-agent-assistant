# Undici audit remediation post-increment review

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "undici-audit-remediation",
  "quality_gate": "FAIL",
  "next_increment_readiness": "Blocked",
  "files_changed": [
    "CHANGELOG.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "TROUBLESHOOTING_LOG.md",
    "docs/plans/2026-09-29-undici-audit-remediation.md",
    "docs/reviews/2026-09-29-undici-audit-remediation-post-increment-review.md",
    "package-lock.json"
  ],
  "commands_executed": [
    "python3 -B .codex/hooks/post_increment_gate.py begin --increment undici-audit-remediation",
    "npm ci",
    "npm audit --audit-level=low",
    "npm ls jsdom undici --all",
    "npm audit --omit=dev --audit-level=low",
    "node node_modules/prettier/bin/prettier.cjs --write package-lock.json CHANGELOG.md HANDOFF.md NEXT_STEPS.md PLANS.md PROJECT_STATUS.md TROUBLESHOOTING_LOG.md docs/plans/2026-09-29-undici-audit-remediation.md docs/reviews/2026-09-29-undici-audit-remediation-post-increment-review.md",
    "npm run test:frontend -- src/features/agents/AgentsPage.test.tsx",
    "python3 -B /private/tmp/cortexa-undici-audit-remediation-evidence/cargo_check.py",
    "npm run verify",
    "node node_modules/prettier/bin/prettier.cjs --write CHANGELOG.md HANDOFF.md NEXT_STEPS.md PLANS.md PROJECT_STATUS.md TROUBLESHOOTING_LOG.md docs/plans/2026-09-29-undici-audit-remediation.md docs/reviews/2026-09-29-undici-audit-remediation-post-increment-review.md",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "python3 -B /private/tmp/cortexa-undici-audit-remediation-evidence/validate.py",
    "python3 -B .codex/hooks/session_end_gate.py",
    "python3 -B /private/tmp/cortexa-undici-audit-remediation-evidence/report_schema.py --failed"
  ],
  "verification": [
    {
      "command": "npm ci",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm audit --audit-level=low",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm ls jsdom undici --all",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm audit --omit=dev --audit-level=low",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "node node_modules/prettier/bin/prettier.cjs --write package-lock.json CHANGELOG.md HANDOFF.md NEXT_STEPS.md PLANS.md PROJECT_STATUS.md TROUBLESHOOTING_LOG.md docs/plans/2026-09-29-undici-audit-remediation.md docs/reviews/2026-09-29-undici-audit-remediation-post-increment-review.md",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run test:frontend -- src/features/agents/AgentsPage.test.tsx",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -B /private/tmp/cortexa-undici-audit-remediation-evidence/cargo_check.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run verify",
      "required": true,
      "status": "Failed"
    },
    {
      "command": "node node_modules/prettier/bin/prettier.cjs --write CHANGELOG.md HANDOFF.md NEXT_STEPS.md PLANS.md PROJECT_STATUS.md TROUBLESHOOTING_LOG.md docs/plans/2026-09-29-undici-audit-remediation.md docs/reviews/2026-09-29-undici-audit-remediation-post-increment-review.md",
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
      "command": "python3 -B /private/tmp/cortexa-undici-audit-remediation-evidence/validate.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -B .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -B /private/tmp/cortexa-undici-audit-remediation-evidence/report_schema.py --failed",
      "required": true,
      "status": "Passed"
    }
  ],
  "manual_verification": [
    {
      "check": "Inspect official metadata, exact three-field lock diff and preserved trust boundaries",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Native/provider/runtime live-success and Codex isolation",
      "required": false,
      "status": "Manual verification pending"
    }
  ],
  "findings": [
    {
      "category": "Code health",
      "severity": "High",
      "summary": "Required native no-bundle build failed with missing zerofrom_derive crate; cause unresolved.",
      "risk": "Local full acceptance and publication readiness are not established.",
      "effort": "Requires separately authorized bounded read-only assessment first.",
      "milestone": "Before any proposed acceptance or publication.",
      "blocks_completion": true,
      "blocks_next_increment": true
    },
    {
      "category": "Security",
      "severity": "Advisory",
      "summary": "D-127 accepted quick-xml vulnerability and warning debt remains.",
      "risk": "Audit gate passing does not remove the two accepted Rust vulnerabilities.",
      "effort": "Separate owner-selected security increment.",
      "milestone": "Existing D-127 follow-up.",
      "blocks_completion": false,
      "blocks_next_increment": false
    },
    {
      "category": "Roadmap",
      "severity": "Advisory",
      "summary": "D-128 custody/abort, native/provider/runtime live-success and Codex isolation remain advisory.",
      "risk": "Offline test evidence does not prove provider/runtime live success or supported Codex isolation.",
      "effort": "Separate owner-approved prerequisite/manual verification.",
      "milestone": "Existing demo follow-up; D-125/M1/M2 parked.",
      "blocks_completion": false,
      "blocks_next_increment": false
    }
  ]
}
-->

Date: 2026-09-29
Increment: undici-audit-remediation
Branch: codex/undici-audit-remediation

## Executive summary

The authorized undici 7.29.1 patch eliminates the observed npm audit finding locally. Acceptance failed at the required native build. Quality: FAIL; readiness: Blocked. No passing completion marker or publication is justified.

## Scope and boundaries

Exactly nine paths: one lockfile and eight additive documentation paths. Only undici version, resolved URL and integrity change. package.json, all unrelated lock entries, product/fixture/Cargo/workflow/hook/skill/configuration bytes and every predecessor record are preserved. No authority, endpoint, permission, provider behavior or audit baseline changes.

## Verification results

Actual executed commands and results are listed below. Clean install, dependency inspection, both npm audits (zero vulnerabilities), focused Agents tests (18), and pinned cargo-audit/repository gate passed. The Rust audit reports two accepted quick-xml advisories and eight warning records, not zero findings. npm run verify FAILED at the native Tauri no-bundle build, with E0463 missing zerofrom_derive and E0432 in zerofrom 0.1.8. Earlier full-suite phases passed: 74 hook tests, 85 repository tests, 431 frontend tests, 368 Rust library tests, integration suites including ACP 7/7 and Hermes 13 passed/one intentionally ignored real-runtime probe, linting, type checking and frontend builds. No automatic repair or rerun occurred. Failure-documentation formatting, docs:check, repository:check, security:scan, whitespace, exact scope/preservation/registry, session and failed-report schema checks passed. They validate truthful failure evidence and do not convert the failed build to acceptance. Passing finalize, complete/valid status, exact-head CI and live/manual product checks: Not run; the required full acceptance failed and no publication/live authority is used.

## Architecture findings

Reviewed: no module, ownership, IPC, orchestration, framework or production dependency changes. The transitive development dependency remains under jsdom and satisfies its existing range. No architecture drift finding introduced.

## Security findings

Reviewed official package metadata and closed lock delta. Full/production npm audits passed with zero findings. Cargo audit retained the unchanged D-127 baseline. No ignore flags, overrides, new secrets, provider traffic, permission or trust-boundary changes. Security acceptance cannot make the failed native build pass.

## Code-health findings

The three-field lock patch and installed tree are correct. The required native build is a blocking observed failure; its underlying generated macro-library/toolchain cause is not diagnosed here. Application source was not changed. Preserve this result without relabeling historical failures.

## Technical debt

High blocking build-acceptance finding: native build failed; effort requires separate bounded assessment, milestone before acceptance/publication, blocks completion and next implementation. Existing D-127 and D-128/provider/runtime/Codex advisories remain nonblocking historical debt for this dependency change; owners and follow-up milestones remain unchanged.

## Roadmap findings

The increment stopped on failed required validation. D-125/M1/M2 remain parked; no new product work or successor is admitted. Proposed next action is read-only assessment of the native build failure using existing evidence, with separate owner direction.

## Completion decision

FAIL. Do not call finalize. Freeze truthful failure evidence, validate it using the unchanged gate and use ordinary close-failed only if schema/scope/preservation pass. Do not manufacture complete status or a completion marker.

## Next-increment readiness

Blocked. A read-only review may propose an authorized next action; this record grants no successor, repair, publication or live-request authority.

## Exact files changed

- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-09-29-undici-audit-remediation.md`
- `docs/reviews/2026-09-29-undici-audit-remediation-post-increment-review.md`
- `package-lock.json`

## Exact commands executed

- `npm ci` — Passed (exit 0).
- `npm audit --audit-level=low` — Passed (exit 0).
- `npm ls jsdom undici --all` — Passed (exit 0).
- `npm audit --omit=dev --audit-level=low` — Passed (exit 0).
- `node node_modules/prettier/bin/prettier.cjs --write package-lock.json CHANGELOG.md HANDOFF.md NEXT_STEPS.md PLANS.md PROJECT_STATUS.md TROUBLESHOOTING_LOG.md docs/plans/2026-09-29-undici-audit-remediation.md docs/reviews/2026-09-29-undici-audit-remediation-post-increment-review.md` — Passed (exit 0).
- `npm run test:frontend -- src/features/agents/AgentsPage.test.tsx` — Passed (exit 0).
- `python3 -B /private/tmp/cortexa-undici-audit-remediation-evidence/cargo_check.py` — Passed (exit 0).
- `npm run verify` — Failed (exit 1).
- `node node_modules/prettier/bin/prettier.cjs --write CHANGELOG.md HANDOFF.md NEXT_STEPS.md PLANS.md PROJECT_STATUS.md TROUBLESHOOTING_LOG.md docs/plans/2026-09-29-undici-audit-remediation.md docs/reviews/2026-09-29-undici-audit-remediation-post-increment-review.md` — Passed (exit 0).
- `npm run docs:check` — Passed (exit 0).
- `npm run repository:check` — Passed (exit 0).
- `npm run security:scan` — Passed (exit 0).
- `git diff --check` — Passed (exit 0).
- `python3 -B /private/tmp/cortexa-undici-audit-remediation-evidence/validate.py` — Passed (exit 0).
- `python3 -B .codex/hooks/session_end_gate.py` — Passed (exit 0).
- `python3 -B /private/tmp/cortexa-undici-audit-remediation-evidence/report_schema.py --failed` — Passed (exit 0).

Ordinary begin passed before tracked edits. Owner-authorized cargo-audit 0.22.2 installation and official registry metadata retrieval passed before admission. Native build errors are observed evidence; no cause or repair is asserted. The full-verify log SHA-256 is `5688318342dd99cd74d23be5bc3736cfdb370b69bf9fe6948655196e0e0c69b0`.

Ordinary close-failed, status and full-payload Stop are post-freeze actions. Their actual results are retained in external disposition evidence so this report is not mutated after terminal closure. Passing finalize, complete/valid status, exact-head CI, app launch and provider requests were Not run. No failed predecessor was reopened or rewritten.
