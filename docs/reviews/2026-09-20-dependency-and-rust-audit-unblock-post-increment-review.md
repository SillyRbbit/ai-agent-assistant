# Dependency and Rust audit unblock post-increment review

Date: 2026-09-20. Increment: dependency-and-rust-audit-unblock.
Branch: codex/dependency-and-rust-audit-unblock.
Base: f176c36cc701b5a296162331cfcb2600b2157663.

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "dependency-and-rust-audit-unblock",
  "quality_gate": "PASS WITH ADVISORIES",
  "next_increment_readiness": "Ready with advisories",
  "verification": [
    {
      "command": "python3 -m unittest scripts.tests.test_cargo_audit_gate -v",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm ci",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm ls vitest @vitest/mocker baseline-browser-mapping js-yaml",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run test:frontend",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -m unittest discover -s scripts/tests -p 'test_*.py' -v",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -m unittest discover -s .codex/hooks/tests -p 'test_*.py' -v",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm audit --audit-level=low",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm audit --omit=dev --audit-level=low",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "zsh -c 'set +e; /private/tmp/cortexa-cargo-audit-0.22.2/bin/cargo-audit audit --json --file src-tauri/Cargo.lock > /private/tmp/cortexa-dependency-and-rust-audit-unblock-cargo-audit.json; audit_status=$?; set -e; python3 scripts/cargo_audit_gate.py /private/tmp/cortexa-dependency-and-rust-audit-unblock-cargo-audit.json --cargo-audit-exit \"$audit_status\"'",
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
      "command": "git diff --check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    }
  ],
  "manual_verification": [
    {
      "check": "Exact 16-path candidate scope, byte-identical package transfer, matching remote-main source blobs, and preserved package SHA-256 values",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Original dirty checkout, seven unpublished commits, cleanup worktree, terminal-failed dependency worktree, and PR #116 head and scope preserved",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Architecture, security, code-health, technical-debt, and readiness review of the exact candidate",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Native GUI smoke for PR #116",
      "required": false,
      "status": "Manual verification pending"
    }
  ],
  "findings": [
    {
      "category": "Technical debt",
      "severity": "Medium",
      "summary": "The accepted Rust advisory baseline still contains two vulnerabilities and eight warnings.",
      "risk": "The locked graph retains the already accepted quick-xml denial-of-service findings and informational or unsound dependency findings.",
      "effort": "Medium",
      "milestone": "Separate owner-approved Cargo dependency remediation",
      "blocks_completion": false,
      "blocks_next_increment": false
    },
    {
      "category": "Roadmap",
      "severity": "Advisory",
      "summary": "PR #116 native GUI smoke remains pending.",
      "risk": "Its browser evidence does not independently prove native WebView behavior.",
      "effort": "Small",
      "milestone": "Owner native demo walkthrough before cleanup merge",
      "blocks_completion": false,
      "blocks_next_increment": false
    }
  ],
  "files_changed": [
    "CHANGELOG.md",
    "DECISIONS.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "ROADMAP.md",
    "SECURITY.md",
    "TESTING_GUIDE.md",
    "TROUBLESHOOTING_LOG.md",
    "docs/plans/2026-09-20-dependency-and-rust-audit-unblock.md",
    "docs/reviews/2026-09-20-dependency-and-rust-audit-unblock-post-increment-review.md",
    "package-lock.json",
    "package.json",
    "scripts/cargo_audit_gate.py",
    "scripts/tests/test_cargo_audit_gate.py"
  ],
  "commands_executed": [
    "python3 -m unittest scripts.tests.test_cargo_audit_gate -v",
    "npm ci",
    "npm ls vitest @vitest/mocker baseline-browser-mapping js-yaml",
    "npm run test:frontend",
    "python3 -m unittest discover -s scripts/tests -p 'test_*.py' -v",
    "python3 -m unittest discover -s .codex/hooks/tests -p 'test_*.py' -v",
    "npm audit --audit-level=low",
    "npm audit --omit=dev --audit-level=low",
    "zsh -c 'set +e; /private/tmp/cortexa-cargo-audit-0.22.2/bin/cargo-audit audit --json --file src-tauri/Cargo.lock > /private/tmp/cortexa-dependency-and-rust-audit-unblock-cargo-audit.json; audit_status=$?; set -e; python3 scripts/cargo_audit_gate.py /private/tmp/cortexa-dependency-and-rust-audit-unblock-cargo-audit.json --cargo-audit-exit \"$audit_status\"'",
    "npm run security:scan",
    "npm run verify",
    "npm run docs:check",
    "npm run repository:check",
    "git diff --check",
    "python3 .codex/hooks/session_end_gate.py"
  ]
}
-->

## Executive summary

The combined successor transfers the already validated JavaScript dependency
candidate and reconciles the Cargo warning baseline to current RustSec data.
Vitest and its matching package family resolve at 4.1.11,
baseline-browser-mapping at 2.11.0, and js-yaml at 4.3.2. The Cargo gate keeps
the exact two quick-xml vulnerabilities and eight current warnings while
rejecting each of the ten withdrawn GTK advisories if it reappears. All required
local checks passed. Quality is PASS WITH ADVISORIES for existing accepted Rust
debt and PR #116's separate pending native GUI smoke.

## Scope and boundaries

The exact change set is 16 paths: two package files, the Cargo audit gate and
focused test, ten additive current-state documentation files, and the plan and
review pair. No product UI, native or Cargo dependency, workflow, hook, skill,
harness, capability, IPC, credential, persistence, permission, or runtime path
changed. The terminal-failed predecessor remains failed and untouched. PR #116,
D-125, M1, and M2 remain separate and unchanged.

## Verification results

- Passed: baseline package blobs in the predecessor worktree matched current
  remote main before transfer. The transferred files retain SHA-256
  6652d6fd99dc0a34773f127c0b781a4e2c057c8ad50b08d960cf20dce7768423
  and b90e638439baf8183faa7c15576152c7161d6a6001bf4508b8a0dd54b3547adc.
- Passed: clean npm installation added 288 packages and reported zero
  vulnerabilities. Exact tree inspection resolved Vitest and @vitest/mocker
  4.1.11, baseline-browser-mapping 2.11.0, and js-yaml 4.3.2 with no peer error.
- Passed: 22 frontend files and 370 tests, 82 repository tests, and 74 hook
  tests.
- Passed: full and production npm audits each reported zero vulnerabilities.
- Passed: pinned cargo-audit 0.22.2 used advisory database commit
  d5c17953a895cf19e8d3ce66eaa42b6fc1fb16 and reported exactly
  RUSTSEC-2026-0194 and RUSTSEC-2026-0195 plus the exact eight accepted
  warnings. The repository gate passed.
- Passed: secret scanning and complete npm run verify, including strict
  formatting and lint, repository policy, all application tests, frontend
  production build, and Tauri no-bundle release build.
- Recoverable first attempt: npm run verify stopped at formatting for the new
  plan. The exact in-scope Prettier output was applied; the second attempt
  passed in full.
- Passed: documentation, repository, whitespace, exact-scope, preservation,
  independent review, and session checks.
- Manual verification pending, non-blocking: PR #116 native GUI smoke. This
  successor has no visual or native behavior change.

## Architecture findings

No architecture finding. The change affects development/test package versions
and repository validation only. It introduces no product dependency, new
abstraction, authority owner, runtime coupling, platform path, or trust-boundary
change. Exact static sets preserve fail-closed behavior and keep advisory
interpretation in the repository-owned gate.

## Security findings

The full and production npm audits report zero vulnerabilities. The Cargo gate
continues to expose the accepted two vulnerabilities and eight warnings and
rejects any new, missing, malformed, version-drifted, or exit-inconsistent
finding. No ignore or automatic synchronization was added. RustSec withdrawal
is recorded as maintenance-status evidence only and is not treated as proof
that GTK 0.18.2 is vulnerability-free.

## Code-health findings

The focused suite asserts the exact eight-warning set and uses subtests to prove
that all ten withdrawn IDs fail if reported again. Existing clean-report,
reviewed-baseline, malformed-report, exit-status, new-finding, missing-finding,
and version-drift behavior remains covered. Complete review found no blocking
defect or unrelated lockfile/direct-dependency change.

## Technical debt

Medium, existing and accepted: two quick-xml vulnerabilities and eight warning
findings remain in the Rust dependency graph. Risk: denial-of-service,
informational maintenance, and unsoundness findings remain until a separate
compatible Cargo remediation is approved. Effort: Medium. Milestone: separate
owner-approved Cargo dependency remediation. Blocks completion: no. Blocks next
increment: no.

Advisory: PR #116 native GUI smoke remains pending. Risk: its browser evidence
does not prove native WebView behavior. Effort: Small. Milestone: owner native
demo walkthrough. Blocks completion and this successor PR: no.

## Roadmap findings

The combined audit successor is Ready with advisories for PR review after a
valid completion marker. It does not make D-125, M1, M2, or any product
successor Ready. PR #116 remains a separate cleanup PR and keeps its native GUI
advisory. No merge is authorized.

## Completion decision

PASS WITH ADVISORIES. Every required automated and manual check passed. The
accepted Rust advisory baseline and unrelated PR #116 native GUI smoke remain
non-blocking advisories. Finalization and Stop must succeed before commit, push,
and PR creation.

## Next-increment readiness

Ready with advisories for exact-head PR review after publication. Merge requires
separate owner approval and applicable passing remote checks. No other increment
starts automatically.

## Exact files changed

- CHANGELOG.md
- DECISIONS.md
- HANDOFF.md
- NEXT_STEPS.md
- PLANS.md
- PROJECT_STATUS.md
- ROADMAP.md
- SECURITY.md
- TESTING_GUIDE.md
- TROUBLESHOOTING_LOG.md
- docs/plans/2026-09-20-dependency-and-rust-audit-unblock.md
- docs/reviews/2026-09-20-dependency-and-rust-audit-unblock-post-increment-review.md
- package-lock.json
- package.json
- scripts/cargo_audit_gate.py
- scripts/tests/test_cargo_audit_gate.py

## Exact commands executed

The machine manifest lists every required completion command. All listed
commands returned exit 0 in their final required execution. The standalone
cargo-audit process returned its expected finding status 1 because the accepted
quick-xml vulnerabilities remain; the listed combined command passed that exact
status and JSON to the repository gate and returned 0. The first npm run verify
attempt stopped on the new plan's formatting; after the exact in-scope repair,
the second attempt returned 0. No product file changed after complete
verification.
