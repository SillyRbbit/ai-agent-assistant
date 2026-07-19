# D-058 dual-self-hosted-runner publication correction post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "git status --short --branch",
    "git log -5 --oneline --decorate",
    "python3 .codex/hooks/post_increment_gate.py begin --increment repository-dual-self-hosted-runner-routing",
    "npm run test:repository (baseline: 37 passed)",
    "npm run repository:check (baseline: passed)",
    "gh api repos/SillyRbbit/ai-agent-assistant/actions/runners",
    "gh pr checks 30",
    "gh run view 29665772834 --json status,conclusion,event,headSha,jobs,url",
    "gh run view 29665772844 --json status,conclusion,event,headSha,jobs,url",
    "gh api repos/SillyRbbit/ai-agent-assistant/check-runs/88135818481/annotations",
    "gh api repos/SillyRbbit/ai-agent-assistant/check-runs/88135818501/annotations",
    "python3 -m unittest scripts.tests.test_repository_health -v",
    "python3 -m unittest scripts.tests.test_ci_change_scope -v",
    "npm run test:repository",
    "ruby -e 'require \"yaml\"; Dir[\".github/workflows/*.{yml,yaml}\"].sort.each { |path| YAML.safe_load(File.read(path), [], [], true); puts \"PASS #{path}\" }'",
    "npm run verify (first attempt stopped at ROADMAP.md formatting)",
    "npx prettier --write ROADMAP.md",
    "npm run verify",
    "cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "git diff --exit-code -- src src-tauri tests package.json package-lock.json Cargo.lock src-tauri/Cargo.lock .codex .agents",
    "python3 .codex/hooks/session_end_gate.py",
    "python3 .codex/hooks/post_increment_gate.py finalize --increment repository-dual-self-hosted-runner-routing --report docs/reviews/2026-07-18-meta-risk-based-ci-post-increment-review.md (failed: report filename did not match increment)",
    "git restore --source=HEAD -- docs/reviews/2026-07-18-meta-risk-based-ci-post-increment-review.md (sandboxed attempt: index lock denied)",
    "git restore --source=HEAD -- docs/reviews/2026-07-18-meta-risk-based-ci-post-increment-review.md",
    "python3 .codex/hooks/post_increment_gate.py finalize --increment repository-dual-self-hosted-runner-routing --report docs/reviews/2026-07-18-repository-dual-self-hosted-runner-routing-post-increment-review.md",
    "python3 .codex/hooks/post_increment_gate.py status",
    "complete architecture, security, code-health, technical-debt, roadmap-readiness, and diff review"
  ],
  "files_changed": [
    ".github/workflows/ci.yml",
    ".github/workflows/documentation.yml",
    "AGENTS.md",
    "CHANGELOG.md",
    "CODE_REVIEW.md",
    "DECISIONS.md",
    "ENGINEERING_GUIDE.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "ROADMAP.md",
    "SECURITY.md",
    "SECURITY_CHECKLIST.md",
    "TESTING_GUIDE.md",
    "docs/github/SELF_HOSTED_RUNNER.md",
    "docs/increments/meta-risk-based-ci.md",
    "docs/plans/meta-risk-based-ci.md",
    "docs/reviews/2026-07-18-repository-dual-self-hosted-runner-routing-post-increment-review.md",
    "scripts/repository_health.py",
    "scripts/tests/test_ci_change_scope.py",
    "scripts/tests/test_repository_health.py"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Security",
      "effort": "Small",
      "milestone": "PR #30 publication and ongoing runner operations",
      "risk": "Local review cannot prove actual remote job assignment or the continuing isolation and maintenance posture of either persistent runner after publication.",
      "severity": "Advisory",
      "summary": "Actual dual-runner execution and continued host isolation remain pending publication evidence and ongoing operator control."
    }
  ],
  "increment_id": "repository-dual-self-hosted-runner-routing",
  "manual_verification": [
    {
      "check": "Confirm both runner services use dedicated unprivileged accounts with no interactive sudo, personal files, SSH keys, production credentials, cloud metadata, or mounted sensitive data.",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "After publication, confirm Linux jobs use runner 21, target-Mac Rust uses runner 22, and no pull-request event receives either persistent runner.",
      "required": false,
      "status": "Manual verification pending"
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
      "command": "python3 -m unittest scripts.tests.test_repository_health -v",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run test:repository",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "ruby -e 'require \"yaml\"; Dir[\".github/workflows/*.{yml,yaml}\"].sort.each { |path| YAML.safe_load(File.read(path), [], [], true); puts \"PASS #{path}\" }'",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run verify",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked",
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
      "command": "git diff --exit-code -- src src-tauri tests package.json package-lock.json Cargo.lock src-tauri/Cargo.lock .codex .agents",
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

Date: 2026-07-18
Increment: D-058 dual-self-hosted-runner publication correction
Branch: `codex/ci/risk-based-github-actions-validation`
Pull request: #30
Baseline commit: `4fb7f31`

## Executive summary

PR #30 originally proposed two risk-based workflows on GitHub-hosted runners.
GitHub rejected CI run `29665772834` and Documentation run `29665772844`
before allocating a runner because the account Actions minute or spending limit
was exhausted. Both failed jobs had zero steps and explicit billing annotations;
no repository command failed.

D-058 keeps exactly two read-only workflows and the deterministic risk
classifier, but routes reviewed branch pushes across the registered persistent
runners. Linux runner 21 owns classification, documentation, frontend, Linux
Rust, and dependency audits. macOS runner 22 adds strict Clippy and all-target
Rust tests for every Rust-classified change.

The automated implementation checks pass, and the project owner confirmed on
2026-07-18 that both runner services satisfy D-058's isolated, unprivileged
host baseline. The result is `PASS WITH ADVISORIES`; actual dual-runner job
assignment remains pending publication. No product source, dependency,
lockfile, hook, skill, Tauri boundary, capability, permission, CSP, SQLite,
identifier, or behavior changed.

## Scope and boundaries

The correction changes exactly 22 paths: two workflows, repository workflow
policy and tests, one push-range classifier fixture, and the authority, runner,
plan, review, changelog, roadmap, and project-memory files required to record
D-058. The original branch remains the same 29-path product-neutral increment;
the correction adds one dedicated D-058 report while preserving the historical
Meta CI report unchanged.

The workflows no longer subscribe to `pull_request`. Eligible pushes are
limited to `main`, `codex/**`, `feature/**`, `fix/**`, `refactor/**`, `meta/**`,
and `phase*/**`; CI retains weekly audit and explicit dispatch. Fork,
external-contributor, and dependency-bot changes must be reviewed and reproduced
on a maintainer-controlled allowlisted branch before either persistent runner
executes them.

## Verification results

Passed:

- GitHub API inspection found Linux runner 21 and macOS runner 22 online, idle,
  and carrying their exact `cortexa-ci` selectors at inspection time.
- Both workflow YAML files parse; exactly two workflow files remain.
- Seventeen classifier cases, including direct push-range coverage, and all 38
  repository tests pass.
- Repository policy rejects pull-request triggers, unapproved selectors,
  mutable actions, write permissions, unexpected workflows, missing trusted
  branches, and `sudo` provisioning.
- Final `npm run verify`: formatting, repository policy, ESLint, strict Clippy,
  28 hook tests, 38 repository tests, 124 frontend tests, 96 Rust library tests,
  21 Rust integration tests, type checking, production frontend builds, and the
  Tauri release no-bundle build.
- Exact all-target Rust tests include the native approval-dialog example target.
- Documentation, secret-pattern, link, generated-output, protected-path,
  whitespace, architecture, security, code-health, technical-debt, and
  roadmap-readiness checks pass.

Failed required checks: none in the final state. The first final `npm run
verify` attempt stopped because the edited `ROADMAP.md` table needed Prettier;
the mechanical formatting correction was applied and the complete rerun passed.
The first marker finalization was rejected because the reused historical report
filename did not match the active D-058 gate ID. After explicit owner approval,
the committed Meta CI report was restored and this D-058 report was placed at
the validator-required path. The sandboxed restore attempt could not create the
Git index lock; the approved retry passed.

External failure: the original hosted jobs failed before execution because of
the account Actions limit. This is recorded evidence for D-058, not a passing
or failed repository verification command.

Checks not run: actual Linux and macOS self-hosted workflow jobs require an
approved commit and push. npm and Cargo audits were not repeated because the
dependency manifests and lockfiles are unchanged and their exact passing
`4fb7f31` evidence remains valid.

Mandatory manual verification passed: the project owner confirmed both runner
services use dedicated unprivileged accounts with no interactive `sudo`,
personal files, SSH keys, production credentials, cloud metadata, or mounted
sensitive data. After publication, confirm job-to-runner assignment and that no
pull-request event receives a persistent runner; this is a non-blocking
publication advisory, not local completion evidence. No product or native UI
manual check applies.

## Architecture findings

None. Application modules, IPC, persistence, Tauri configuration, product data
flow, and runtime portability are unchanged. Linux portability and target-Mac
conditional compilation now receive distinct jobs without changing classifier
ownership.

## Security findings

No Critical or High blocking finding remains. The project owner confirmed the
required isolated, unprivileged host baseline for both runner services.
Persistent hosts remain a larger trust surface than ephemeral runners, so
D-058 fails closed at the event boundary: no `pull_request` or
`pull_request_target`, exact trusted branch patterns, read-only permissions, no
secrets, immutable actions, disabled checkout credentials, no write or
publication commands, no `sudo`, fixed Git arguments, and validated SHAs and
paths. Runner accounts and packages remain provisioned outside workflows.

Actual runner assignment and successful dual-runner execution remain mandatory
before PR #30 merges. Continuing host isolation, patching, workspace cleanup,
monitoring, and incident response remain operator-controlled advisories.

## Code-health findings

None. Risk classification remains standard-library-only. A focused push-range
fixture covers the now-active event mode, and repository health enforces the
exact Linux/macOS selectors and persistent-runner trigger boundary.

## Technical debt

Two non-blocking maintenance obligations remain: path ownership must change
with repository structure, and persistent runner hosts require patching,
workspace cleanup, monitoring, and incident response. D-058 and the runner
guide make both obligations explicit.

## Roadmap findings

No product work is reordered. ARB-002 remains Blocked on O-006/O-007 and owner
decisions. The only next task is review and publication of this exact
dual-runner correction.

## Completion decision

`PASS WITH ADVISORIES`. Automated verification passed, the required
host-isolation manual check passed by explicit project-owner confirmation, and
no Critical or High blocking finding remains. The completion marker is complete
and valid. Actual Linux/macOS workflow execution remains pending publication
and is required before merge.

## Next-increment readiness

`Blocked`. ARB-002 remains blocked on O-006/O-007 and project-owner,
security-owner, and executive-owner decisions. The only allowed next action is
review and publication of this exact D-058 correction.

## Exact files changed

The machine manifest records the complete exact 22-path correction: two
workflows, three repository validator/test paths, and 17 authority, runner,
plan, review, changelog, roadmap, and project-memory paths. No application
source, dependency, lockfile, hook, skill, Tauri, SQLite, or product path
changed.

## Exact commands executed

The machine manifest records all inspection, baseline, focused, complete,
documentation, security, protected-path, session-end, finalization, and status
commands. It distinguishes the hosted allocation failures, the resolved
ROADMAP formatting stop, and the rejected historical report filename from the
final passing verification and valid marker.

## Rollback

Before publication, restore the 22 correction paths from `4fb7f31`. After
publication, return to D-057 hosted selectors only after Actions availability
is restored and a separate security review approves the transition. No product,
dependency, database, or native rollback is required.

## Exact next task

Review the exact 22-path dual-runner correction and wait for separate
commit/push approval. After push, require Linux runner 21 and macOS runner 22
execution evidence and confirm no pull-request event received either runner
before merging PR #30. Do not begin ARB-002 or another increment.
