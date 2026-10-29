# Dashboard dependency security acceptance post-increment review

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "dashboard-dependency-security-correction",
  "quality_gate": "PASS WITH ADVISORIES",
  "next_increment_readiness": "Ready with advisories",
  "commands_executed": [
    "npm audit --audit-level=low",
    "npm run verify",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "python3 -B .codex/hooks/session_end_gate.py"
  ],
  "files_changed": [
    "CHANGELOG.md",
    "DECISIONS.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PROJECT_STATUS.md",
    "TROUBLESHOOTING_LOG.md",
    "docs/plans/2026-10-28-dashboard-current-main-integration.md",
    "docs/reviews/2026-10-28-dashboard-dependency-security-correction-post-increment-review.md",
    "docs/reviews/2026-10-28-dashboard-dependency-security-readiness-post-increment-review.md",
    "package-lock.json"
  ],
  "findings": [],
  "verification": [
    {
      "command": "npm audit --audit-level=low",
      "required": true,
      "status": "Passed"
    },
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
      "dependency": {
        "status": "automatically_verified",
        "evidence": "LOCK-DIFF.json exact two compatible dev leaves; npm-ci-01 exit0; GRAPH-QUALIFIED complete14353-file graph,29 bins and4 platform payloads; ACTIVATION-VERIFIED preserves old graph"
      },
      "verification": {
        "status": "automatically_verified",
        "evidence": "audit-01 zero vulnerabilities, full-01 actualexit0/wait0/complete268359-byte capture and cleanup, final-checks-01 five passing checks"
      },
      "preservation": {
        "status": "automatically_verified",
        "evidence": "POST-FULL-PRESERVATION static graph hashes/modes, old complete child metadata, unchanged original source/index/state and sealed histories; fully monitored allocation"
      },
      "review": {
        "status": "automatically_verified",
        "evidence": "SCOPE-REVIEW substantive architecture/security/code-health/scope and readiness assessment,14 accounting and6 permission cases; normal finalization follows"
      }
    },
    "paths": {
      "TROUBLESHOOTING_LOG.md": {
        "criterion": "review",
        "rationale": "Additive current verification/owner/checkpoint documentation; historical bodies retained",
        "within_objective": true,
        "preserves_existing": true
      },
      "HANDOFF.md": {
        "criterion": "review",
        "rationale": "Additive current verification/owner/checkpoint documentation; historical bodies retained",
        "within_objective": true,
        "preserves_existing": true
      },
      "package-lock.json": {
        "criterion": "dependency",
        "rationale": "Only Undici/Zod transitive metadata changes",
        "within_objective": true,
        "preserves_existing": true
      },
      "CHANGELOG.md": {
        "criterion": "review",
        "rationale": "Additive current verification/owner/checkpoint documentation; historical bodies retained",
        "within_objective": true,
        "preserves_existing": true
      },
      "NEXT_STEPS.md": {
        "criterion": "review",
        "rationale": "Additive current verification/owner/checkpoint documentation; historical bodies retained",
        "within_objective": true,
        "preserves_existing": true
      },
      "docs/plans/2026-10-28-dashboard-current-main-integration.md": {
        "criterion": "review",
        "rationale": "Additive current verification/owner/checkpoint documentation; historical bodies retained",
        "within_objective": true,
        "preserves_existing": true
      },
      "docs/reviews/2026-10-28-dashboard-dependency-security-correction-post-increment-review.md": {
        "criterion": "review",
        "rationale": "Additive current verification/owner/checkpoint documentation; historical bodies retained",
        "within_objective": true,
        "preserves_existing": true
      },
      "PROJECT_STATUS.md": {
        "criterion": "review",
        "rationale": "Additive current verification/owner/checkpoint documentation; historical bodies retained",
        "within_objective": true,
        "preserves_existing": true
      }
    }
  }
}
-->

Date: 2026-10-28
Increment: dashboard-dependency-security-correction
Branch: codex/dashboard-main-integration

## Executive summary

Required current evidence passed for the bounded dependency-security correction.

## Scope and boundaries

Only compatible transitive Undici/Zod package-lock updates, fresh standard-npm graph provisioning/qualification/activation and additive current records. Root package.json, product code, validators, workflows, assertions and historical constants stay unchanged. Existing reports, original checkout/index/state and all predecessor seals remain immutable.

## Verification results

Fresh full npm verify passed in606.475 seconds with complete268359-byte capture:447 hooks,155 repository tests,711 frontend cases in52 files,559 distinct Rust library cases invoked twice plus all contract suites and release build. Fresh audit found zero vulnerabilities. The five final affected checks passed. Exact source/graph/environment, actual terminal/wait0 and complete owned-group cleanup are retained in dependency-audit-fix-01; documentation-only finalization does not relabel another full run. Earlier full/proof/CI results remain bound to their own source/graph and are not relabelled as current verification. Exact pinned Node/npm, public module closure, system Python and verified3.12 nested route, Rust1.90 and offline SDK/Cargo environment remain required.

## Architecture findings

Two development-dependency leaf updates preserve all existing UI, Rust, runtime and IPC boundaries. Current parent constraints admit patched7.x Undici and4.x Zod. No new framework, runtime, helper or production dependency is introduced.

## Security findings

The inherited audit failure at c11f1fb names Undici7.29.1 and Zod4.4.3. Official advisories identify compatible patched lines7.30.1/4.6.6. This known failure blocks merge until a fresh current audit passes; it is the remediation target, not a readiness waiver. Standard npm uses credential-free isolated HOME/config/cache, strict TLS and lockfile SRI; scripts/staging audit submissions disabled. Preserve every old graph before replacement and all no-follow accounting/cleanup safeguards.

## Code-health findings

Reject any unrelated lockfile entry, package.json or executable change. Verify complete installed graph, required platform binaries and declared package metadata before activation. Preserve exact ordinary source and prior tests; regression evidence comes from fresh full verification and audit, not new tests mirroring the lockfile.

## Technical debt

Advisory: the existing647.35kB minified Vite chunk warning, Node experimental localStorage warning and one inherited ignored Rust case remain. No threshold or test was weakened. Native application/accessibility/live-provider QA remains unverified and outside scope. Step B Blocked, exec Deferred, oldest cleanup and historical volume continuity Unverified stay unchanged. No security audit bypass or live readiness claim.

## Roadmap findings

The sole objective is the already-approved PR158 dependency correction and conditional squash merge. Parked private archive/supervisor/census work remains parked; no successor milestone selected.

## Completion decision

PASS WITH ADVISORIES

## Next-increment readiness

Ready with advisories only for the owner-approved dependency-security remediation and conditional PR finish; no product/readiness waiver or later milestone.

## Exact files changed

- `CHANGELOG.md`
- `DECISIONS.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PROJECT_STATUS.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-10-28-dashboard-current-main-integration.md`
- `docs/reviews/2026-10-28-dashboard-dependency-security-correction-post-increment-review.md`
- `docs/reviews/2026-10-28-dashboard-dependency-security-readiness-post-increment-review.md`
- `package-lock.json`

## Exact commands executed

- `npm audit --audit-level=low`: Passed.
- `npm run verify`: Passed.
- `npm run docs:check`: Passed.
- `npm run repository:check`: Passed.
- `npm run security:scan`: Passed.
- `git diff --check`: Passed.
- `python3 -B .codex/hooks/session_end_gate.py`: Passed.
