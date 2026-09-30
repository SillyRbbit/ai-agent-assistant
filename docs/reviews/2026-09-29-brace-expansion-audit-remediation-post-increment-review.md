# Brace-expansion audit remediation post-increment review

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "brace-expansion-audit-remediation",
  "quality_gate": "PASS WITH ADVISORIES",
  "next_increment_readiness": "Ready with advisories",
  "files_changed": [
    "CHANGELOG.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "TROUBLESHOOTING_LOG.md",
    "docs/plans/2026-09-29-brace-expansion-audit-remediation.md",
    "docs/reviews/2026-09-29-brace-expansion-audit-remediation-post-increment-review.md",
    "package-lock.json"
  ],
  "commands_executed": [
    "/Library/Frameworks/Python.framework/Versions/3.12/bin/python3 -B .codex/hooks/post_increment_gate.py begin --increment brace-expansion-audit-remediation",
    "npm ci",
    "npm ls brace-expansion minimatch undici --all",
    "python3 -B /private/tmp/cortexa-brace-expansion-audit-remediation-evidence/dependency_check.py",
    "node /private/tmp/cortexa-brace-expansion-audit-remediation-evidence/package_checks.cjs",
    "npm audit --audit-level=low --json",
    "npm audit --omit=dev --audit-level=low --json",
    "npm run lint:frontend",
    "npm run format:frontend",
    "python3 -B /private/tmp/cortexa-brace-expansion-audit-remediation-evidence/cargo_check.py",
    "python3 -B /private/tmp/cortexa-brace-expansion-audit-remediation-evidence/preserve.py check",
    "npm run verify -- -- --locked --offline --config 'profile.release.build-override.strip=\"none\"'",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "python3 -B .codex/hooks/session_end_gate.py",
    "npm exec --offline -- prettier --write CHANGELOG.md HANDOFF.md NEXT_STEPS.md PLANS.md PROJECT_STATUS.md TROUBLESHOOTING_LOG.md docs/plans/2026-09-29-brace-expansion-audit-remediation.md docs/reviews/2026-09-29-brace-expansion-audit-remediation-post-increment-review.md"
  ],
  "verification": [
    {
      "command": "npm ci",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm ls brace-expansion minimatch undici --all",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -B /private/tmp/cortexa-brace-expansion-audit-remediation-evidence/dependency_check.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "node /private/tmp/cortexa-brace-expansion-audit-remediation-evidence/package_checks.cjs",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm audit --audit-level=low --json",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm audit --omit=dev --audit-level=low --json",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run lint:frontend",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run format:frontend",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -B /private/tmp/cortexa-brace-expansion-audit-remediation-evidence/cargo_check.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -B /private/tmp/cortexa-brace-expansion-audit-remediation-evidence/preserve.py check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run verify -- -- --locked --offline --config 'profile.release.build-override.strip=\"none\"'",
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
    },
    {
      "command": "npm exec --offline -- prettier --write CHANGELOG.md HANDOFF.md NEXT_STEPS.md PLANS.md PROJECT_STATUS.md TROUBLESHOOTING_LOG.md docs/plans/2026-09-29-brace-expansion-audit-remediation.md docs/reviews/2026-09-29-brace-expansion-audit-remediation-post-increment-review.md",
      "required": true,
      "status": "Passed"
    }
  ],
  "manual_verification": [
    {
      "check": "Architecture/security/code-health/technical-debt/readiness review of the exact lock/documentation delta",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Protected bytes, historical suffixes, all predecessor records and worktree registry preserved",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Native/provider/runtime live success and supported Codex isolation",
      "required": false,
      "status": "Manual verification pending"
    }
  ],
  "findings": [
    {
      "category": "Security",
      "severity": "Advisory",
      "summary": "D-127 exact quick-xml vulnerability and warning baseline remains.",
      "risk": "A passing baseline gate does not remove the accepted Rust vulnerabilities.",
      "effort": "Separate owner-approved security increment.",
      "milestone": "Existing D-127 follow-up.",
      "blocks_completion": false,
      "blocks_next_increment": false
    },
    {
      "category": "Technical debt",
      "severity": "Advisory",
      "summary": "Native release validation needs the invocation-local stripping override.",
      "risk": "Default affected stripped builds remain uncorrected.",
      "effort": "Retain verified invocation; persistent tooling repair needs separate scope.",
      "milestone": "Future build-tooling review.",
      "blocks_completion": false,
      "blocks_next_increment": false
    },
    {
      "category": "Roadmap",
      "severity": "Advisory",
      "summary": "D-128 custody/abort and native/provider/runtime live-success/Codex-isolation limits remain.",
      "risk": "Offline acceptance does not prove live success, isolation or remote CI.",
      "effort": "Separate owner-approved manual prerequisites.",
      "milestone": "Private demo follow-up; D-125/M1/M2 parked.",
      "blocks_completion": false,
      "blocks_next_increment": false
    }
  ]
}
-->

Date: 2026-09-29
Increment: brace-expansion-audit-remediation
Branch: codex/brace-expansion-audit-remediation

## Executive summary

The exact brace-expansion patch passed fresh local acceptance. Quality: PASS WITH ADVISORIES; readiness: Ready with advisories. The source completion and both failed undici predecessors remain unchanged. Local evidence is distinct from PR #126's historical failed audit; no publication or remote success is claimed.

## Scope and boundaries

Exactly nine successor Git paths and fifteen cumulative paths against main. Only six lock fields differ from the source. The original six plan/review files, product/tests/Cargo/configuration/workflows/hooks/skills/security-policy bytes remain identical. Six root document bodies are exact historical suffixes. All six predecessor worktrees, raw states/markers, frozen artifacts/external evidence and 36 prunable registry entries are preserved.

## Verification results

All required fresh commands listed below Passed. Clean npm ci, exact installed targets 1.1.21/5.0.12, unchanged parent/dependency requirements and undici 7.29.1; full/production npm audits zero vulnerabilities. Both package copies/consumers passed 42 assertions, including five-second bounded child checks for the three advisory patterns. Focused frontend lint/format Passed. Full verify Passed with verified Python 3.12.1, Rust 1.90.0, Xcode/SDK 27.0 and the process-local Cargo stripping override, including strict lint, tests, typechecking and frontend/native release builds. Pinned cargo-audit 0.22.2 raw exit 1 for the exact accepted two quick-xml vulnerabilities/eight warnings; repository gate exit 0. First documentation/repository/secret/whitespace/session and preservation checks Passed. Final documentation/schema/finalization/status/Stop receipts are external and recorded after report freeze. Live requests/GUI, Linux execution and new remote CI: Not run in this increment.

## Architecture findings

Reviewed module ownership, trust boundaries, coupling, portability and dependency health. No architecture or native/provider authority change. Two patch upgrades remain within the existing minimatch ranges; no parent upgrade, new dependency, API/module-export or engine change. Framework and orchestration bytes are protected.

## Security findings

Reviewed exact official registry metadata/integrities and all three GitHub advisories. Full and production npm audits now report zero findings. Bounded offline checks cover the reported CPU/recursion patterns without raw provider/credential input. No ignore flags, audit exceptions, overrides, network/provider permission changes, secrets or sensitive output introduced. Exact Rust baseline remains disclosed. Review found no new blocking issue in this delta.

## Code-health findings

Reviewed full lock/documentation diff, ordinary and pathological pattern checks, strict lint and unchanged tests. No broad resolution, dead-code suppression, fixture repair or product-byte change. Fresh full verification passed on the local Mac; it does not substitute for exact-commit remote Linux/Mac CI. Invocation selection reuses proven environment fixes without repeating their diagnosis.

## Technical debt

Three existing advisory findings are recorded in the manifest with risks, effort/milestones and nonblocking dispositions: D-127 debt, invocation-local native stripping workaround and D-128/live/isolation limits. No new debt or gate weakening was introduced. Project owner selects separate follow-up scope.

## Roadmap findings

The increment completes bounded local dependency acceptance. Proposed next work is read-only publication readiness, then separately authorized fast-forward publication/review for PR #126 if refs and evidence remain valid. D-125/M1/M2 remain parked; no graph, provider expansion, real-runtime or execution work selected.

## Completion decision

PASS WITH ADVISORIES. Finalization is permitted only after passing final documentation, preservation, report schema and session checks. Require actual complete/valid status and full-payload Stop; their receipts remain outside the repository and are not inferred from this prose.

## Next-increment readiness

Ready with advisories for read-only publication-readiness review. Git publication and PR #126 updates need separate owner authorization. No merge readiness or live authority follows from local acceptance.

## Exact files changed

- CHANGELOG.md
- HANDOFF.md
- NEXT_STEPS.md
- PLANS.md
- PROJECT_STATUS.md
- TROUBLESHOOTING_LOG.md
- docs/plans/2026-09-29-brace-expansion-audit-remediation.md
- docs/reviews/2026-09-29-brace-expansion-audit-remediation-post-increment-review.md
- package-lock.json

These are the nine successor paths recorded by the manifest. The fifteen-path cumulative inventory also includes the six immutable inherited undici plan/review files.

## Exact commands executed

- /Library/Frameworks/Python.framework/Versions/3.12/bin/python3 -B .codex/hooks/post_increment_gate.py begin --increment brace-expansion-audit-remediation — Passed (exit 0).
- npm ci — Passed (exit 0).
- npm ls brace-expansion minimatch undici --all — Passed (exit 0).
- python3 -B /private/tmp/cortexa-brace-expansion-audit-remediation-evidence/dependency_check.py — Passed (exit 0).
- node /private/tmp/cortexa-brace-expansion-audit-remediation-evidence/package_checks.cjs — Passed (exit 0).
- npm audit --audit-level=low --json — Passed (exit 0).
- npm audit --omit=dev --audit-level=low --json — Passed (exit 0).
- npm run lint:frontend — Passed (exit 0).
- npm run format:frontend — Passed (exit 0).
- python3 -B /private/tmp/cortexa-brace-expansion-audit-remediation-evidence/cargo_check.py — Passed (exit 0).
- python3 -B /private/tmp/cortexa-brace-expansion-audit-remediation-evidence/preserve.py check — Passed (exit 0).
- npm run verify -- -- --locked --offline --config 'profile.release.build-override.strip="none"' — Passed (exit 0).
- npm run docs:check — Passed (exit 0).
- npm run repository:check — Passed (exit 0).
- npm run security:scan — Passed (exit 0).
- git diff --check — Passed (exit 0).
- python3 -B .codex/hooks/session_end_gate.py — Passed (exit 0).

- npm exec --offline -- prettier --write CHANGELOG.md HANDOFF.md NEXT_STEPS.md PLANS.md PROJECT_STATUS.md TROUBLESHOOTING_LOG.md docs/plans/2026-09-29-brace-expansion-audit-remediation.md docs/reviews/2026-09-29-brace-expansion-audit-remediation-post-increment-review.md — Passed (initial and final document formatting).

Commands use the explicit credential-free process environment in new external context.py. Official metadata/advisory and toolchain preflight evidence is preserved externally. Prior native diagnosis and predecessor validation are historical evidence only. Final schema, finalize/status and full Stop receipts are retained after this report freezes.
