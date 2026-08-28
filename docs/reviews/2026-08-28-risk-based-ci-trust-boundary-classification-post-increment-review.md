# Risk-based CI trust-boundary classification post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "git status --porcelain=v1",
    "git branch --show-current",
    "git rev-parse HEAD",
    "git rev-parse origin/main",
    "git rev-list --left-right --count HEAD...origin/main",
    "git merge-base --is-ancestor HEAD origin/main",
    "git merge-base --is-ancestor origin/main HEAD",
    "git remote -v",
    "git fsck --no-dangling",
    "python3 .codex/hooks/post_increment_gate.py begin --increment risk-based-ci-trust-boundary-classification",
    "python3 -m unittest scripts.tests.test_ci_change_scope -v",
    "python3 classifier reproduction for the eight owner-supplied paths",
    "python3 classifier inventory for src-tauri/src and src-tauri/examples",
    "npm run test:repository",
    "npm run repository:check",
    "npm run security:scan",
    "npm run docs:check",
    "npx prettier --write TESTING_GUIDE.md",
    "npm audit --audit-level=low",
    "npm run verify",
    "npx prettier --write CHANGELOG.md HANDOFF.md NEXT_STEPS.md PLANS.md PROJECT_STATUS.md TESTING_GUIDE.md docs/increments/risk-based-ci-trust-boundary-classification.md docs/plans/2026-08-28-risk-based-ci-trust-boundary-classification.md docs/reviews/2026-08-28-risk-based-ci-trust-boundary-classification-post-increment-review.md",
    "git diff --check",
    "python3 .codex/hooks/session_end_gate.py",
    "python3 .codex/hooks/post_increment_gate.py status"
  ],
  "files_changed": [
    "CHANGELOG.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "TESTING_GUIDE.md",
    "docs/increments/risk-based-ci-trust-boundary-classification.md",
    "docs/plans/2026-08-28-risk-based-ci-trust-boundary-classification.md",
    "docs/reviews/2026-08-28-risk-based-ci-trust-boundary-classification-post-increment-review.md",
    "scripts/ci_change_scope.py",
    "scripts/tests/test_ci_change_scope.py"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Technical debt",
      "effort": "One separately authorized documentation correction",
      "milestone": "Before the next CI-classifier policy change",
      "risk": "ENGINEERING_GUIDE.md still groups examples with isolated Rust tests even though production examples now select all three jobs.",
      "severity": "Advisory",
      "summary": "One broader engineering-guide sentence is stale and outside this increment's allowed file list."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Roadmap",
      "effort": "Publication and exact-run inspection only if separately authorized",
      "milestone": "After any future publication",
      "risk": "Local evidence does not claim an unpublished GitHub Actions run.",
      "severity": "Advisory",
      "summary": "Remote Actions were not run because this increment remains uncommitted and unpublished."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Roadmap",
      "effort": "Owner selection and approval",
      "milestone": "Before any successor repository or product work",
      "risk": "Beginning credential, filesystem, document, memory, IPC, provider, persistence, or tool work would exceed this authorization.",
      "severity": "Advisory",
      "summary": "No next source or remediation increment is owner-selected or Ready."
    }
  ],
  "increment_id": "risk-based-ci-trust-boundary-classification",
  "manual_verification": [
    {
      "check": "Complete current production Rust and native-example path inventory",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Diff inspection for product, dependency, lockfile, capability, CSP, permission, workflow, runner, credential, IPC, persistence, provider, and tool changes",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Target-Mac UI or native behavior",
      "required": false,
      "status": "Not run"
    },
    {
      "check": "Exact remote GitHub Actions for the uncommitted increment",
      "required": false,
      "status": "Not run"
    }
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {
      "command": "python3 -m unittest scripts.tests.test_ci_change_scope -v",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run test:repository",
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
      "command": "npm run docs:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run verify",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm audit --audit-level=low",
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
  ]
}
-->

Date: 2026-08-28
Increment: risk-based-ci-trust-boundary-classification
Branch: main
Baseline: 3fc14e4f17bb171957dc09241a868f3c6deb7cb1

## Executive summary

`PASS WITH ADVISORIES`. The approved repository-workflow increment closes the
risk-based CI trust-boundary classification gap without changing product
source or GitHub workflow behavior. All current and future production Rust
source and native-example paths select frontend, Rust, and audit jobs by
default. Independent review found no blocking security, architecture,
correctness, privacy, or prohibited-change defect.

## Baseline and reproduction

The work began on clean synchronized `main`
`3fc14e4f17bb171957dc09241a868f3c6deb7cb1`; `HEAD` and `origin/main` were
equal, both ancestor checks passed, and `git fsck --no-dangling` passed.

All eight owner-supplied paths reproduced as
`frontend=false rust=true audit=false`:

- `src-tauri/src/credentials/cloudflare_access.rs`
- `src-tauri/src/documents.rs`
- `src-tauri/src/memory.rs`
- `src-tauri/src/research_knowledge_demo_lifecycle.rs`
- `src-tauri/src/research_knowledge_demo_lifecycle_tauri.rs`
- `src-tauri/src/research_knowledge_demo_projection.rs`
- `src-tauri/src/menu_bar/tauri_adapter.rs`
- `src-tauri/examples/cloudflare_access_keychain_probe.rs`

The complete inventory contained 65 Rust files under `src-tauri/src` and two
under `src-tauri/examples`. Fifty-four already selected all three jobs. The 13
under-classified paths were:

- `src-tauri/src/credentials/cloudflare_access.rs`
- `src-tauri/src/credentials/mod.rs`
- `src-tauri/src/documents.rs`
- `src-tauri/src/memory.rs`
- `src-tauri/src/menu_bar/action.rs`
- `src-tauri/src/menu_bar/controller.rs`
- `src-tauri/src/menu_bar/mod.rs`
- `src-tauri/src/menu_bar/tauri_adapter.rs`
- `src-tauri/src/research_knowledge_demo_lifecycle.rs`
- `src-tauri/src/research_knowledge_demo_lifecycle_tauri.rs`
- `src-tauri/src/research_knowledge_demo_projection.rs`
- `src-tauri/examples/cloudflare_access_keychain_probe.rs`
- `src-tauri/examples/native_approval_dialog.rs`

## Implemented classifier boundary

`src-tauri/src/**` and `src-tauri/examples/**` now fail closed to
`frontend=true`, `rust=true`, and `audit=true`. The production exception
allowlist is intentionally empty. Its validator permits only a normalized,
exact `.rs` file under `src-tauri/src`; it rejects globs, directories,
examples, tests, and non-Rust paths. Exact exceptions are checked only after
path validation and before the production fail-closed rule.

`src-tauri/tests/**` remains Rust-only. Documentation-only, frontend-only,
audit-only, schedule, manual dispatch, deletion, unknown non-documentation,
and unsafe-path behavior remains exact.

## Verification results

- Representative post-fix reproduction: Passed — 8/8 select all three jobs.
- Complete current inventory: Passed — 67/67 select all three jobs.
- `python3 -m unittest scripts.tests.test_ci_change_scope -v`: Passed — 21/21.
- `npm run test:repository`: Passed — 80/80.
- `npm run repository:check`: Passed.
- `npm run security:scan`: Passed.
- `npm run docs:check`: Passed after correcting one Prettier-only discrepancy
  in the permitted testing guide.
- `npm run verify`: Passed — formatting, strict lint, 28 hook tests, 80
  repository tests, 313 frontend tests, 269 Rust library tests, 244 Rust
  integration tests, production frontend build, and Tauri release no-bundle
  build. The opt-in real Hermes executable probe remains intentionally ignored.
- `npm audit --audit-level=low`: Passed — zero vulnerabilities after the
  sandboxed attempt could not resolve the registry and the authorized network
  retry completed.
- `git diff --check`: Passed.
- `python3 .codex/hooks/session_end_gate.py`: Passed.
- Target-Mac UI/native checks: Not run — no application or native behavior
  changed.
- Remote GitHub Actions: Not run — no commit, push, PR, merge, or publication
  was authorized or performed.

## Architecture findings

None. Matcher order is fail-closed, exact path validation precedes allowlist
membership, current production and example trees select the full scope before
legacy generic Rust matching, and the change introduces no new dependency or
runtime coupling.

## Security findings

None. The current production exception allowlist is empty, its validator
rejects every broad or non-source entry, and the safe default is additional CI
rather than an omitted job. The diff contains no credential, permission,
workflow, runner, product, IPC, persistence, provider, or tool change.

## Code-health findings

None. The classifier preserves the existing `Scope` contract and uses named,
ordered path families. Table-driven family, dynamic inventory, synthetic
future-path, allowlist, preserved-event, deletion, and unsafe-path tests pass.

## Technical debt

One low documentation advisory remains: `ENGINEERING_GUIDE.md` says Rust
test/example paths remain isolated. The classifier and the more specific
testing guide now correctly route every production example to all three jobs.
The broader guide is outside this increment's explicit allowed-file list and
was not edited.

## Roadmap findings

Remote GitHub Actions are `Not run` because this increment is uncommitted and
unpublished. No next source or remediation increment is owner-selected or
Ready; all credential, filesystem, document, memory, Tauri IPC, provider,
persistence, and tool work remains blocked pending separate authorization.

## Exact commands executed

The baseline Git inspection used:

```bash
git status --porcelain=v1
git branch --show-current
git rev-parse HEAD
git rev-parse origin/main
git rev-list --left-right --count HEAD...origin/main
git merge-base --is-ancestor HEAD origin/main
git merge-base --is-ancestor origin/main HEAD
git remote -v
git fsck --no-dangling
```

The required verification used:

```bash
python3 -m unittest scripts.tests.test_ci_change_scope -v
npm run test:repository
npm run repository:check
npm run security:scan
npm run docs:check
npm run verify
npm audit --audit-level=low
git diff --check
python3 .codex/hooks/session_end_gate.py
python3 .codex/hooks/post_increment_gate.py status
```

Diagnostic discrepancies, all resolved or non-repository:

- The first ad hoc import harness omitted `sys.modules` registration and
  failed before classification; the corrected reproduction produced the
  baseline and final evidence.
- `rg` was unavailable, so equivalent `find`/`grep` inspection was used.
- The first compact final-inventory one-liner had a shell quoting syntax error;
  the corrected command reported `inventory=67` and `non_full_scope=0`.
- The sandboxed gate-begin attempt could not write gate state; the authorized
  retry activated the exact increment.
- The sandboxed npm audit could not resolve the registry; the authorized retry
  passed with zero vulnerabilities.
- The first documentation check found a Prettier-only discrepancy in
  `TESTING_GUIDE.md`; the permitted file was formatted and the final check
  passed.
- One independent-review file-existence loop overwrote zsh's special `path`
  array; its corrected read-only rerun passed.

## Completion decision

PASS WITH ADVISORIES

## Next-increment readiness

Blocked. No successor is owner-selected or Ready. Credential, filesystem,
document, memory, Tauri IPC, live-provider, persistence, tool, or publication
work requires a separately selected and approved bounded increment.

## Exact files changed

- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `TESTING_GUIDE.md`
- `docs/increments/risk-based-ci-trust-boundary-classification.md`
- `docs/plans/2026-08-28-risk-based-ci-trust-boundary-classification.md`
- `docs/reviews/2026-08-28-risk-based-ci-trust-boundary-classification-post-increment-review.md`
- `scripts/ci_change_scope.py`
- `scripts/tests/test_ci_change_scope.py`
