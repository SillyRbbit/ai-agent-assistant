# Dashboard dependency security readiness post-increment review

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "dashboard-dependency-security-readiness",
  "quality_gate": "PASS WITH ADVISORIES",
  "next_increment_readiness": "Ready with advisories",
  "commands_executed": [
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "python3 -B .codex/hooks/session_end_gate.py"
  ],
  "files_changed": [
    "DECISIONS.md",
    "docs/plans/2026-10-28-dashboard-current-main-integration.md",
    "docs/reviews/2026-10-28-dashboard-dependency-security-readiness-post-increment-review.md"
  ],
  "findings": [],
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
  ]
}
-->

Date: 2026-10-28
Increment: dashboard-dependency-security-readiness
Branch: codex/dashboard-main-integration

## Executive summary

Preparation for the owner-authorized Undici/Zod remediation only. The known CI audit failure remains failed; this is not passing publication or application readiness.

## Scope and boundaries

Only compatible transitive Undici/Zod package-lock updates, fresh standard-npm graph provisioning/qualification/activation and additive current records. Root package.json, product code, validators, workflows, assertions and historical constants stay unchanged. Existing reports, original checkout/index/state and all predecessor seals remain immutable.

## Verification results

The five preparation checks are recorded in dependency-audit-fix-01/readiness-checks-01. Passing preparation qualifies only normal admission to repair; it does not resolve the known dependency audit. Earlier full/proof/CI results remain bound to their own source/graph and are not relabelled as current verification. Exact pinned Node/npm, public module closure, system Python and verified3.12 nested route, Rust1.90 and offline SDK/Cargo environment remain required.

## Architecture findings

Two development-dependency leaf updates preserve all existing UI, Rust, runtime and IPC boundaries. Current parent constraints admit patched7.x Undici and4.x Zod. No new framework, runtime, helper or production dependency is introduced.

## Security findings

The inherited audit failure at c11f1fb names Undici7.29.1 and Zod4.4.3. Official advisories identify compatible patched lines7.30.1/4.6.6. This known failure blocks merge until a fresh current audit passes; it is the remediation target, not a readiness waiver. Standard npm uses credential-free isolated HOME/config/cache, strict TLS and lockfile SRI; scripts/staging audit submissions disabled. Preserve every old graph before replacement and all no-follow accounting/cleanup safeguards.

## Code-health findings

Reject any unrelated lockfile entry, package.json or executable change. Verify complete installed graph, required platform binaries and declared package metadata before activation. Preserve exact ordinary source and prior tests; regression evidence comes from fresh full verification and audit, not new tests mirroring the lockfile.

## Technical debt

Advisory: native application/accessibility/live-provider QA remains unverified and outside scope. Step B Blocked, exec Deferred, oldest cleanup and historical volume continuity Unverified stay unchanged. No security audit bypass or live readiness claim.

## Roadmap findings

The sole objective is the already-approved PR158 dependency correction and conditional squash merge. Parked private archive/supervisor/census work remains parked; no successor milestone selected.

## Completion decision

PASS WITH ADVISORIES

## Next-increment readiness

Ready with advisories only for the owner-approved dependency-security remediation and conditional PR finish; no product/readiness waiver or later milestone.

## Exact files changed

- `DECISIONS.md`
- `docs/plans/2026-10-28-dashboard-current-main-integration.md`
- `docs/reviews/2026-10-28-dashboard-dependency-security-readiness-post-increment-review.md`

## Exact commands executed

- `npm run docs:check`: Passed.
- `npm run repository:check`: Passed.
- `npm run security:scan`: Passed.
- `git diff --check`: Passed.
- `python3 -B .codex/hooks/session_end_gate.py`: Passed.
