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
    "gh run view 29670565671 --json databaseId,event,status,conclusion,headSha,name,url,jobs",
    "gh run view 29670565657 --json databaseId,event,status,conclusion,headSha,name,url,jobs",
    "gh api repos/SillyRbbit/ai-agent-assistant/actions/runs/29670565671/jobs --jq '.jobs[] | {id, name, conclusion, runner_name, labels}'",
    "gh api repos/SillyRbbit/ai-agent-assistant/actions/runs/29670565657/jobs --jq '.jobs[] | {id, name, conclusion, runner_name, labels}'",
    "gh run list --commit 9a2c75de9640ea9118bdc552a0d7c63155e0181f --json databaseId,event,name,status,conclusion,headSha,url",
    "gh pr view 30 --json number,title,state,headRefName,headRefOid,baseRefName,mergeable,mergeStateStatus,statusCheckRollup,body,url",
    "gh pr edit 30 --body <D-058 remote-verification closeout description>",
    "gh pr view 30 --json headRefOid,mergeable,mergeStateStatus,body,url",
    "npx prettier --write AGENTS.md CHANGELOG.md DECISIONS.md HANDOFF.md NEXT_STEPS.md PLANS.md PROJECT_STATUS.md ROADMAP.md docs/github/SELF_HOSTED_RUNNER.md docs/increments/meta-risk-based-ci.md docs/plans/meta-risk-based-ci.md docs/reviews/2026-07-18-repository-dual-self-hosted-runner-routing-post-increment-review.md",
    "git diff --exit-code -- .github src src-tauri tests scripts package.json package-lock.json Cargo.toml Cargo.lock src-tauri/Cargo.toml src-tauri/Cargo.lock .codex .agents",
    "gh pr view 30 --json state,mergedAt,mergeCommit,headRefOid,url",
    "gh run list --commit 1780d7fb668541dbb6a7fbd6fb39b82ce49fd753 --json databaseId,event,name,status,conclusion,headSha,url",
    "gh run view 29672575232 --json databaseId,event,status,conclusion,headSha,name,url,jobs",
    "gh run watch 29672575232 --exit-status --interval 10",
    "gh api repos/SillyRbbit/ai-agent-assistant/actions/runs/29672575232/jobs --jq '.jobs[] | {id, name, conclusion, runner_name, labels}'",
    "gh api repos/SillyRbbit/ai-agent-assistant/actions/runs/29672575254/jobs --jq '.jobs[] | {id, name, conclusion, runner_name, labels}'",
    "npx prettier --write AGENTS.md CHANGELOG.md DECISIONS.md HANDOFF.md NEXT_STEPS.md PLANS.md PROJECT_STATUS.md ROADMAP.md docs/increments/meta-risk-based-ci.md docs/plans/meta-risk-based-ci.md docs/reviews/2026-07-18-repository-dual-self-hosted-runner-routing-post-increment-review.md",
    "git diff --exit-code -- .github src src-tauri tests scripts package.json package-lock.json Cargo.toml Cargo.lock src-tauri/Cargo.toml src-tauri/Cargo.lock .codex .agents docs/github/SELF_HOSTED_RUNNER.md docs/reviews/2026-07-18-meta-risk-based-ci-post-increment-review.md",
    "complete architecture, security, code-health, technical-debt, roadmap-readiness, and diff review"
  ],
  "files_changed": [
    "AGENTS.md",
    "CHANGELOG.md",
    "DECISIONS.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "ROADMAP.md",
    "docs/increments/meta-risk-based-ci.md",
    "docs/plans/meta-risk-based-ci.md",
    "docs/reviews/2026-07-18-repository-dual-self-hosted-runner-routing-post-increment-review.md"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Security",
      "effort": "Small",
      "milestone": "Ongoing runner operations",
      "risk": "Successful remote assignment for one reviewed commit does not prove the continuing isolation, maintenance, or path-ownership posture of either persistent runner.",
      "severity": "Advisory",
      "summary": "Remote assignment passed; continued host isolation and path ownership remain operator-maintained advisories."
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
      "check": "Confirm successful push-triggered runs assign classification, documentation, frontend, Linux Rust, and dependency audit to runner 21; assign target-Mac Rust to runner 22; and list no pull-request run for the reviewed commit.",
      "required": true,
      "status": "Passed"
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
    },
    {
      "command": "gh run view 29670565671 --json databaseId,event,status,conclusion,headSha,name,url,jobs",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "gh run view 29670565657 --json databaseId,event,status,conclusion,headSha,name,url,jobs",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "gh api repos/SillyRbbit/ai-agent-assistant/actions/runs/29670565671/jobs --jq '.jobs[] | {id, name, conclusion, runner_name, labels}'",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "gh api repos/SillyRbbit/ai-agent-assistant/actions/runs/29670565657/jobs --jq '.jobs[] | {id, name, conclusion, runner_name, labels}'",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "gh run list --commit 9a2c75de9640ea9118bdc552a0d7c63155e0181f --json databaseId,event,name,status,conclusion,headSha,url",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "git diff --exit-code -- .github src src-tauri tests scripts package.json package-lock.json Cargo.toml Cargo.lock src-tauri/Cargo.toml src-tauri/Cargo.lock .codex .agents",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "gh pr view 30 --json state,mergedAt,mergeCommit,headRefOid,url",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "gh run list --commit 1780d7fb668541dbb6a7fbd6fb39b82ce49fd753 --json databaseId,event,name,status,conclusion,headSha,url",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "gh run watch 29672575232 --exit-status --interval 10",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "gh api repos/SillyRbbit/ai-agent-assistant/actions/runs/29672575232/jobs --jq '.jobs[] | {id, name, conclusion, runner_name, labels}'",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "gh api repos/SillyRbbit/ai-agent-assistant/actions/runs/29672575254/jobs --jq '.jobs[] | {id, name, conclusion, runner_name, labels}'",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "git diff --exit-code -- .github src src-tauri tests scripts package.json package-lock.json Cargo.toml Cargo.lock src-tauri/Cargo.toml src-tauri/Cargo.lock .codex .agents docs/github/SELF_HOSTED_RUNNER.md docs/reviews/2026-07-18-meta-risk-based-ci-post-increment-review.md",
      "required": true,
      "status": "Passed"
    }
  ]
}
-->

Date: 2026-07-18
Increment: D-058 dual-self-hosted-runner publication correction
Implementation branch: `codex/ci/risk-based-github-actions-validation`
Pull request: #30
Baseline commit: `4fb7f31`
D-058 implementation commit: `9a2c75d`
Documentation closeout commit: `da08573`
Published main commit: `1780d7f`

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
host baseline. Commit `9a2c75d` produced successful push-triggered CI run
`29670565671` and Documentation run `29670565657`. The result remains `PASS
WITH ADVISORIES` because persistent-host maintenance and path ownership remain
operator responsibilities. No product source, dependency, lockfile, hook,
skill, Tauri boundary, capability, permission, CSP, SQLite, identifier, or
behavior changed.

Documentation closeout commit `da08573` passed Documentation run
`29671289962`. PR #30 was then squash-merged at `1780d7f`. Post-merge CI run
`29672575232` and Documentation run `29672575254` passed with the exact Linux
runner 21 and macOS runner 22 assignments.

## Scope and boundaries

Implementation commit `9a2c75d` contains the exact reviewed 22-path correction:
two workflows, repository workflow policy and tests, one push-range classifier
fixture, and the authority, runner, plan, review, changelog, roadmap, and
project-memory files required to record D-058. The original branch remains the
same 29-path product-neutral increment, and the historical Meta CI report is
unchanged.

The pre-merge remote-verification closeout changed exactly 12 documentation
paths and is preserved in source commit `da08573`. This post-publication memory
reconciliation changes exactly 11 documentation paths: eight root authority
and project-memory files, the Meta CI increment and plan, and this D-058 report.
The runner guide and historical Meta CI report remain unchanged. No workflow,
classifier, repository validator, application source, dependency, lockfile,
hook, or skill file changes.

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
- CI run `29670565671` completed successfully as a `push` event for
  `9a2c75d`. Linux runner 21 `henry-dang-HP-Elite-Slice` executed
  classification job `88148646821`, frontend job `88148677830`, Linux Rust job
  `88148677826`, and dependency-audit job `88148677832` with exact labels
  `[self-hosted, Linux, X64, cortexa-ci]`.
- Documentation run `29670565657` completed successfully as a `push` event for
  `9a2c75d`; job `88148646740` executed on Linux runner 21 with the same exact
  selector.
- Target-Mac Rust job `88148677829` passed on macOS runner 22
  `Henrys-MacBook-Pro` with exact labels
  `[self-hosted, macOS, X64, cortexa-ci]`.
- The GitHub run listing for `9a2c75d` contains only CI run `29670565671` and
  Documentation run `29670565657`, both with event `push`.
- Before closeout and merge, PR #30 was open, mergeable, and clean at
  implementation head `9a2c75d`; its description recorded the successful
  remote evidence instead of pending execution.
- Documentation closeout commit `da08573` passed push-triggered Documentation
  run `29671289962` on Linux runner 21 before merge.
- PR #30 squash-merged at `1780d7f`. Post-merge CI run `29672575232` passed
  classification, frontend, Linux Rust, dependency audit, and target-Mac Rust;
  post-merge Documentation run `29672575254` also passed.
- Post-merge classification, documentation, frontend, Linux Rust, and dependency
  audit ran on Linux runner 21 `henry-dang-HP-Elite-Slice`; target-Mac Rust ran
  on macOS runner 22 `Henrys-MacBook-Pro`.

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

Checks not run during this documentation-only post-publication reconciliation:
frontend, Rust, Tauri, and product builds were not rerun locally because no
applicable source, dependency, lockfile, workflow, classifier, or
repository-validator path changed. Passing local implementation evidence,
branch runs, and post-merge runs remain applicable.

Mandatory manual verification passed: the project owner confirmed both runner
services use dedicated unprivileged accounts with no interactive `sudo`,
personal files, SSH keys, production credentials, cloud metadata, or mounted
sensitive data. GitHub evidence confirms the exact job-to-runner assignment and
shows only push-triggered runs for `9a2c75d`. No product or native UI manual
check applies.

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

Actual runner assignment and successful dual-runner execution passed for
`9a2c75d` and published `1780d7f`. Continuing host isolation, patching,
workspace cleanup, monitoring, and incident response remain
operator-controlled advisories.

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
decisions. The only next task is review and separate publication of this exact
documentation-only post-publication memory reconciliation.

## Completion decision

`PASS WITH ADVISORIES`. Automated verification passed, the required
host-isolation manual check passed by explicit project-owner confirmation, and
the exact Linux/macOS remote assignment passed for `9a2c75d`. No Critical or
High blocking finding remains. The completion marker is complete and valid
after documentation-only post-publication re-finalization. PR #30 is published
at `1780d7f`, and its post-merge CI and Documentation runs passed. Ongoing
persistent-host maintenance and path ownership remain advisories.

## Next-increment readiness

`Blocked`. ARB-002 remains blocked on O-006/O-007 and project-owner,
security-owner, and executive-owner decisions. The only allowed next actions
are review and separate publication of this exact D-058 post-publication memory
reconciliation.

## Exact files changed

The machine manifest records the exact 11-path documentation-only
post-publication reconciliation:
`AGENTS.md`, `CHANGELOG.md`, `DECISIONS.md`, `HANDOFF.md`, `NEXT_STEPS.md`,
`PLANS.md`, `PROJECT_STATUS.md`, `ROADMAP.md`, the Meta CI increment and plan,
and this report. The runner guide is not changed in this reconciliation. No
workflow, classifier, repository validator, application source, dependency,
lockfile, hook, skill, Tauri, SQLite, or product path changed.

## Exact commands executed

The machine manifest records all implementation inspection, baseline, focused,
complete, documentation, security, protected-path, session-end, finalization,
and status commands plus the remote run, job-assignment, event, and PR evidence.
It distinguishes the historical hosted allocation failures, the resolved
ROADMAP formatting stop, and the rejected historical report filename from the
final passing verification and valid marker.

## Rollback

After publication, revert squash commit `1780d7f` only through a separately
reviewed repository change. Return to D-057 hosted selectors only after Actions
availability is restored and a separate security review approves the
transition. No product, dependency, database, or native rollback is required.

## Exact next task

Review the exact 11-path documentation-only D-058 post-publication memory
reconciliation, confirm the valid marker and protected-path exclusion, and
propose a descriptive branch name, Conventional Commit, PR title, and PR
description. Wait for project-owner approval before staging, committing, or
pushing. Do not begin ARB-002 or another increment.
