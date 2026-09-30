# Python-corrected undici acceptance post-increment review

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "undici-audit-remediation-python-acceptance",
  "quality_gate": "PASS WITH ADVISORIES",
  "next_increment_readiness": "Ready with advisories",
  "files_changed": [
    "CHANGELOG.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "TROUBLESHOOTING_LOG.md",
    "docs/plans/2026-09-29-undici-audit-remediation-acceptance.md",
    "docs/plans/2026-09-29-undici-audit-remediation-python-acceptance.md",
    "docs/plans/2026-09-29-undici-audit-remediation.md",
    "docs/reviews/2026-09-29-undici-audit-remediation-acceptance-post-increment-review.md",
    "docs/reviews/2026-09-29-undici-audit-remediation-post-increment-review.md",
    "docs/reviews/2026-09-29-undici-audit-remediation-python-acceptance-post-increment-review.md",
    "package-lock.json"
  ],
  "commands_executed": [
    "/Library/Frameworks/Python.framework/Versions/3.12/bin/python3 -B .codex/hooks/post_increment_gate.py begin --increment undici-audit-remediation-python-acceptance",
    "npm ci",
    "npm view undici@7.29.1 version engines dist --json",
    "npm ls jsdom undici --all --json",
    "npm audit --audit-level=low --json",
    "npm audit --omit=dev --audit-level=low --json",
    "python3 -B /private/tmp/cortexa-undici-python-acceptance-evidence/dependency_check.py",
    "python3 -B /private/tmp/cortexa-undici-python-acceptance-evidence/cargo_check.py",
    "npm run verify -- -- --locked --offline --config 'profile.release.build-override.strip=\"none\"'",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "python3 -B /private/tmp/cortexa-undici-python-acceptance-evidence/preserve.py check",
    "python3 -B .codex/hooks/session_end_gate.py"
  ],
  "verification": [
    {
      "command": "npm ci",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm view undici@7.29.1 version engines dist --json",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm ls jsdom undici --all --json",
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
      "command": "python3 -B /private/tmp/cortexa-undici-python-acceptance-evidence/dependency_check.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -B /private/tmp/cortexa-undici-python-acceptance-evidence/cargo_check.py",
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
      "command": "python3 -B /private/tmp/cortexa-undici-python-acceptance-evidence/preserve.py check",
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
      "check": "Architecture/security/code-health/technical-debt/readiness review of the full diff and exact lock patch",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Final documentation, repository, secret, whitespace, preservation and session checks",
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
      "summary": "D-127 accepted quick-xml vulnerability and warning debt remains unchanged.",
      "risk": "Passing the exact audit baseline does not remove the accepted Rust vulnerabilities.",
      "effort": "Separate owner-selected security increment.",
      "milestone": "Existing D-127 follow-up.",
      "blocks_completion": false,
      "blocks_next_increment": false
    },
    {
      "category": "Technical debt",
      "severity": "Advisory",
      "summary": "macOS 27 native release acceptance requires the process-local Cargo host-build stripping override.",
      "risk": "Default affected release builds without the override remain uncorrected.",
      "effort": "Retain the verified invocation; any persistent or upstream toolchain correction needs separate scope.",
      "milestone": "Future build-tooling review before changing the invocation.",
      "blocks_completion": false,
      "blocks_next_increment": false
    },
    {
      "category": "Roadmap",
      "severity": "Advisory",
      "summary": "D-128 custody/abort, native/provider/runtime live success and Codex isolation remain advisory; D-125/M1/M2 parked.",
      "risk": "Offline tests cannot prove live/provider/runtime success or supported isolation.",
      "effort": "Separate owner-approved prerequisites and manual validation.",
      "milestone": "Existing private demo follow-up.",
      "blocks_completion": false,
      "blocks_next_increment": false
    }
  ]
}
-->

Date: 2026-09-29
Increment: undici-audit-remediation-python-acceptance
Branch: codex/undici-audit-remediation-python-acceptance

## Executive summary

Local acceptance validation passed. The existing dependency patch is accepted by fresh application/native and audit evidence. Quality decision: PASS WITH ADVISORIES; readiness: Ready with advisories. Both historical failures remain unchanged. The checkout-local marker and Stop are verified after report freeze; this report does not grant publication.

## Scope and boundaries

Exactly thirteen cumulative paths, with only eight successor-document edits. The source lockfile and both predecessor plan/review pairs are byte-identical. Six root-document bodies remain exact historical suffixes. Every unrelated product/test/Cargo/dependency/configuration/workflow/hook/skill byte is unchanged. No new permissions, endpoint, runtime or provider behavior. All five previous valid worktrees and 36 prunable registry entries remain preserved.

## Verification results

Fresh clean npm ci, official undici metadata/requirements, npm tree, full and production npm audits Passed; zero npm vulnerabilities. Pinned cargo-audit 0.22.2 produced raw exit 1 for the accepted two quick-xml vulnerabilities and eight warnings; unchanged repository gate Passed (exit 0), RustSec commit f23b768236fe2880e4cfa167da662cad8ca79240. Full npm run verify with locked/offline Cargo and the stripping override Passed: format, repository, frontend/Rust strict lint, 74 hook and 85 repository tests, 431 frontend tests, 368 Rust library tests, integration suites including ACP 7/7 and Hermes 13 passed/one intentionally ignored real-runtime probe, typecheck, frontend builds and native no-bundle release build. The full command covers those suites; no redundant standalone application tests were added. Final documentation-tier results below reflect actual execution. Live/native GUI/provider requests and remote CI: Not run, outside this authorization. Report-schema, finalize, complete-status and full Stop are executed after freeze and retained in external evidence.

## Architecture findings

Reviewed: model/WebView remain untrusted, deterministic native ownership and orchestration are unchanged. No IPC, module, permission or provider boundary changes. The Node development dependency remains under jsdom 29.1.1 with its existing allowed range; package.json is unchanged. Process-local verification selection introduces no repository architecture change.

## Security findings

Reviewed registry integrity, exact three-field lock patch, sanitized subprocess configuration and preserved security-sensitive files. No secrets, provider requests, ignore flags, audit exceptions, dependency overrides or permission expansion introduced. Both npm audits and the exact Rust gate passed; D-127 risk remains disclosed. Secret-scan status is listed only after actual execution.

## Code-health findings

The known Python prerequisite failure was corrected solely by explicit process-local selection of installed Python 3.12.1 before npm executes checkers. The npm subprocess preflight proved the executable and strict-zip behavior. Product, fixtures and checkers were not modified. Required full verification now succeeds, including the affected native build under the already-validated stripping override. No new code-health defect found in the lock/documentation delta.

## Technical debt

All findings are Advisory and do not block this bounded local acceptance: existing D-127 security debt, process-local stripping workaround dependence, and D-128/live/runtime/Codex-isolation limitations. Owners remain the project owner and separately approved follow-up increments; effort/milestones are recorded in the manifest. No advisory was silently repaired or waived.

## Roadmap findings

The authorized outcome is local acceptance of the undici patch. The next proposed task is read-only publication-readiness review, followed only by explicit owner authorization for Git publication and required CI/review. D-125/M1/M2 remain parked. This increment grants no live request allowance or production authority.

## Completion decision

PASS WITH ADVISORIES. All required local validation recorded here passed. Ordinary finalize is permitted only after final report-schema and preservation checks; require complete/valid status and full Stop afterward.

## Next-increment readiness

Ready with advisories. Ready for separately requested publication-readiness review with the documented advisories. Actual publication/merge requires owner authority and CI/review.

## Exact files changed

- CHANGELOG.md
- HANDOFF.md
- NEXT_STEPS.md
- PLANS.md
- PROJECT_STATUS.md
- TROUBLESHOOTING_LOG.md
- docs/plans/2026-09-29-undici-audit-remediation-acceptance.md
- docs/plans/2026-09-29-undici-audit-remediation-python-acceptance.md
- docs/plans/2026-09-29-undici-audit-remediation.md
- docs/reviews/2026-09-29-undici-audit-remediation-acceptance-post-increment-review.md
- docs/reviews/2026-09-29-undici-audit-remediation-post-increment-review.md
- docs/reviews/2026-09-29-undici-audit-remediation-python-acceptance-post-increment-review.md
- package-lock.json

## Exact commands executed

- /Library/Frameworks/Python.framework/Versions/3.12/bin/python3 -B .codex/hooks/post_increment_gate.py begin --increment undici-audit-remediation-python-acceptance — Passed.
- npm ci — Passed (exit 0).
- npm view undici@7.29.1 version engines dist --json — Passed (exit 0).
- npm ls jsdom undici --all --json — Passed (exit 0).
- npm audit --audit-level=low --json — Passed (exit 0).
- npm audit --omit=dev --audit-level=low --json — Passed (exit 0).
- python3 -B /private/tmp/cortexa-undici-python-acceptance-evidence/dependency_check.py — Passed (exit 0).
- python3 -B /private/tmp/cortexa-undici-python-acceptance-evidence/cargo_check.py — Passed (exit 0).
- npm run verify -- -- --locked --offline --config 'profile.release.build-override.strip="none"' — Passed (exit 0).
- npm run docs:check — Passed (exit 0).
- npm run repository:check — Passed (exit 0).
- npm run security:scan — Passed (exit 0).
- git diff --check — Passed (exit 0).
- python3 -B /private/tmp/cortexa-undici-python-acceptance-evidence/preserve.py check — Passed (exit 0).
- python3 -B .codex/hooks/session_end_gate.py — Passed (exit 0).

Commands execute under the explicit sanitized environment in the new external evidence runner. Tool identity/npm-Python preflight is recorded in preflight.json. Original diagnoses and predecessor application results are historical evidence only. Final schema/finalization/status/Stop receipts are preserved after report freeze under /private/tmp/cortexa-undici-python-acceptance-evidence.
