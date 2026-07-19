# Meta Increment - risk-based GitHub Actions validation

Status: Verified locally and remotely with advisories; merge pending
Date: 2026-07-18
Baseline: clean synchronized `main` at `1c03f66`
Gate: `repository-dual-self-hosted-runner-routing`

## Goal

Replace blanket verification with efficient risk-based validation across the
registered Linux and macOS runners while preserving the complete local
increment gate, security-sensitive checks, and product behavior.

## Completed implementation

- Reduced the active workflow tree to `ci.yml` and `documentation.yml`.
- Added trusted branch-push and manual triggers with explicit paths, read-only
  permissions, concurrency cancellation, immutable action SHAs, disabled
  checkout credentials, bounded timeouts, and no secrets or write steps. CI
  retains its weekly audit schedule; neither workflow accepts pull-request
  events.
- Added a standard-library fixed-SHA classifier that handles pull-request merge
  bases, push ranges, tracked deletions, scheduled audit, manual full dispatch,
  unsafe paths, and fail-closed unknown non-documentation paths.
- Split Application CI into classifier/policy, frontend, Linux Rust, target-Mac
  Rust, and dependency-audit jobs. Documentation-only changes do not start it.
- Consolidated the former weekly Security workflow into CI's audit job while
  preserving the exact accepted npm and Rust advisory policy.
- Added focused frontend scripts without changing tool behavior, dependencies,
  or lockfiles.
- Expanded repository health for prompt metadata/path checks and the exact
  dual-runner two-workflow policy.
- Preserved D-054 and D-057 as dated historical evidence and recorded D-058 for
  active Linux/macOS routing after GitHub rejected both PR #30 hosted jobs
  before allocation because the Actions minute or spending limit was exhausted.

## Change-to-workflow matrix

| Change class                                                  | Documentation | Frontend      | Linux Rust    | macOS Rust    | Audit         |
| ------------------------------------------------------------- | ------------- | ------------- | ------------- | ------------- | ------------- |
| Markdown, prompt, or project memory only                      | Yes           | No            | No            | No            | No            |
| Frontend or application brand asset                           | When mixed    | Yes           | No            | No            | No            |
| Isolated target-neutral Rust test/example                     | When mixed    | No            | Yes           | Yes           | No            |
| Tauri, IPC, policy, approval, storage, migration, or security | When mixed    | Yes           | Yes           | Yes           | Yes           |
| Dependency or Application CI workflow                         | When mixed    | Yes           | Yes           | Yes           | Yes           |
| Documentation workflow or repository validator                | Yes           | No            | No            | No            | Yes           |
| Repository hook                                               | No            | No            | No            | No            | Yes           |
| Mixed documentation and source                                | Yes           | As classified | As classified | As classified | As classified |
| Weekly schedule                                               | No            | No            | No            | No            | Yes           |
| Manual CI dispatch                                            | No            | Yes           | Yes           | Yes           | Yes           |

## Exact scope

The complete change set is the approved 29 paths:

```text
.github/workflows/ci.yml
.github/workflows/documentation.yml
.github/workflows/security.yml (deleted)
AGENTS.md
CHANGELOG.md
CODE_REVIEW.md
CONTRIBUTING.md
DECISIONS.md
ENGINEERING_GUIDE.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
README.md
ROADMAP.md
SECURITY.md
SECURITY_CHECKLIST.md
TESTING_GUIDE.md
docs/github/SELF_HOSTED_RUNNER.md
docs/increments/meta-risk-based-ci.md
docs/plans/README.md
docs/plans/meta-risk-based-ci.md
docs/reviews/2026-07-18-meta-risk-based-ci-post-increment-review.md
package.json
prompts/increments/verified-increment.md
scripts/ci_change_scope.py
scripts/repository_health.py
scripts/tests/test_ci_change_scope.py
scripts/tests/test_repository_health.py
```

No application source, test contract, dependency version, lockfile, Tauri
configuration, command, capability, CSP, permission, SQLite migration, database,
identifier, credential, or product behavior changed.

The D-058 correction is bounded to 22 paths: the two workflows; repository
health and its workflow-policy test; the push-range classifier test; and the 17
authority, runner, plan, review, changelog, roadmap, and project-memory files
needed to replace stale active D-057 wording. The classifier implementation,
application source, dependencies, lockfiles, hooks, and skills remain unchanged.

## Verification evidence

Passed:

- Clean `npm ci` and `npm ci --ignore-scripts` installs.
- Both workflow files parse as YAML.
- Seventeen classifier cases and all 38 repository tests.
- All 28 repository hook tests.
- `npm run docs:check`, `npm run repository:check`, and
  `npm run security:scan`.
- Final `npm run verify`, including strict Clippy, 124 frontend tests, 96 Rust
  library tests, 21 Rust integration tests, production frontend builds, and the
  Tauri release no-bundle build.
- Exact all-target Rust tests.
- `npm audit --audit-level=low` with zero vulnerabilities.
- Pinned `cargo-audit 0.22.2`; the baseline gate accepted only D-025's exact two
  vulnerabilities and D-046's exact 18 warnings.
- GitHub API verification that checkout and setup-node SHAs are signed and
  match official `v7.0.0` tags.
- Complete diff, scope, generated-output, architecture, security, code-health,
  technical-debt, and roadmap-readiness review.

Failed required checks: none. One unsupported local Ruby API invocation and one
sandbox-denied npm audit attempt were resolved by a supported parser invocation
and an approved network-enabled retry.

The first GitHub-hosted jobs attempted on PR #30 failed before allocation
because the account Actions limit was exhausted; no repository step ran. D-058
implementation commit `9a2c75d` then produced successful push-triggered CI run
`29670565671` and Documentation run `29670565657`. Linux runner 21 executed
classification, documentation, frontend, Linux Rust, and dependency audit;
macOS runner 22 executed target-Mac Rust. The run listing for `9a2c75d`
contains only those two push-triggered workflows. No native product manual
check applies.

Required manual verification passed: the project owner confirmed both runner
services satisfy D-058's dedicated, unprivileged host baseline. Remote
job-to-runner assignment also passed. Continuing host isolation, patching,
cleanup, monitoring, and incident response remain explicit advisories.

## Review outcome

- Architecture: no product or runtime boundary changed. Linux, target-Mac, and
  local responsibilities are explicit.
- Security: persistent runners receive no pull-request event, secrets, writes,
  or host provisioning. Fixed commands, validated SHAs/paths, exact selectors,
  and fail-closed classification preserve the automation boundary.
- Code health: focused fixtures cover every declared classifier class; package
  script composition avoids duplicate frontend/Rust work.
- Technical debt: no new blocking debt. Path ownership must be maintained as
  repository structure changes.
- Readiness: no later product or remediation increment is Ready. ARB-002 remains
  blocked on O-006/O-007 and owner decisions.

Result: `PASS WITH ADVISORIES`. Automated local verification and the required
project-owner host-isolation confirmation pass, no Critical or High blocking
finding remains, and the `repository-dual-self-hosted-runner-routing` marker is
complete and valid after documentation-only closeout re-finalization. Actual
dual-runner execution passed for `9a2c75d`; ongoing persistent-host maintenance
and path ownership remain advisories.

## Risks and rollback

The main risks are persistent-host exposure and stale path ownership. D-058
prohibits pull-request events and requires isolated unprivileged hosts. Its path
filters, classifier rules, fixtures, and testing matrix must change together;
unknown paths fail closed. Conditional path-filtered workflows are not
represented as universal branch-protection checks.

Before PR #30 merges, revert `9a2c75d` on the feature branch if D-058 must be
withdrawn. After merge, restore D-057's hosted selectors only when Actions
availability is repaired and a separate security review approves the change,
then rerun local and remote validation. Rollback needs no product, database,
dependency, or host rebuild.

## Exact next task

Review the exact 12-path D-058 documentation-only remote-verification closeout
and wait for separate staging, commit, and push approval. Merge PR #30 only
with separate project-owner approval. Do not begin ARB-002 or another
increment.
