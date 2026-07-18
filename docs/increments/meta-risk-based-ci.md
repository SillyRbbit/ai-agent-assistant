# Meta Increment - risk-based GitHub Actions validation

Status: Verified complete locally with advisories; publication pending
Date: 2026-07-18
Baseline: clean synchronized `main` at `1c03f66`
Gate: `meta-risk-based-ci`

## Goal

Replace blanket persistent-runner verification with efficient risk-based
GitHub-hosted validation while preserving the complete local increment gate,
security-sensitive checks, and product behavior.

## Completed implementation

- Reduced the active workflow tree to `ci.yml` and `documentation.yml`.
- Added pull-request, `main` push, and manual triggers with explicit paths,
  read-only permissions, concurrency cancellation, immutable action SHAs,
  disabled checkout credentials, bounded timeouts, and no secrets or write
  steps.
- Added a standard-library fixed-SHA classifier that handles pull-request merge
  bases, push ranges, tracked deletions, scheduled audit, manual full dispatch,
  unsafe paths, and fail-closed unknown non-documentation paths.
- Split Application CI into classifier/policy, frontend, Rust, and
  dependency-audit jobs. Documentation-only changes do not start it.
- Consolidated the former weekly Security workflow into CI's audit job while
  preserving the exact accepted npm and Rust advisory policy.
- Added focused frontend scripts without changing tool behavior, dependencies,
  or lockfiles.
- Expanded repository health for prompt metadata/path checks and the exact
  hosted two-workflow policy.
- Recorded D-057 and synchronized active engineering, testing, security,
  review, contribution, README, runner, prompt, plan, roadmap, and project-memory
  guidance. D-054 remains dated historical and rollback evidence.

## Change-to-workflow matrix

| Change class                                                  | Documentation | Frontend      | Rust          | Audit         |
| ------------------------------------------------------------- | ------------- | ------------- | ------------- | ------------- |
| Markdown, prompt, or project memory only                      | Yes           | No            | No            | No            |
| Frontend or application brand asset                           | When mixed    | Yes           | No            | No            |
| Isolated target-neutral Rust test/example                     | When mixed    | No            | Yes           | No            |
| Tauri, IPC, policy, approval, storage, migration, or security | When mixed    | Yes           | Yes           | Yes           |
| Dependency or Application CI workflow                         | When mixed    | Yes           | Yes           | Yes           |
| Documentation workflow or repository validator                | Yes           | No            | No            | Yes           |
| Repository hook                                               | No            | No            | No            | Yes           |
| Mixed documentation and source                                | Yes           | As classified | As classified | As classified |
| Weekly schedule                                               | No            | No            | No            | Yes           |
| Manual CI dispatch                                            | No            | Yes           | Yes           | Yes           |

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

## Verification evidence

Passed:

- Clean `npm ci` and `npm ci --ignore-scripts` installs.
- Both workflow files parse as YAML.
- Sixteen classifier cases and all 37 repository tests.
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

Not run: Ubuntu package installation and actual GitHub-hosted jobs. The former
is target-specific to the workflow image; the latter requires publication. No
native product manual check applies.

Manual verification pending: inspect applicable CI and Documentation jobs after
publication. This is non-blocking for local completion and remains an explicit
advisory.

## Review outcome

- Architecture: no product or runtime boundary changed. Hosted/local
  responsibilities and target-Mac limits are explicit.
- Security: untrusted pull-request code moves from a persistent host to
  ephemeral read-only runners. Fixed commands, validated SHAs/paths, no secrets,
  and fail-closed classification preserve the automation boundary.
- Code health: focused fixtures cover every declared classifier class; package
  script composition avoids duplicate frontend/Rust work.
- Technical debt: no new blocking debt. Path ownership must be maintained as
  repository structure changes.
- Readiness: no later product or remediation increment is Ready. ARB-002 remains
  blocked on O-006/O-007 and owner decisions.

Result: `PASS WITH ADVISORIES` because hosted execution and remote enforcement
are pending publication. The `meta-risk-based-ci` marker is complete and valid.

## Risks and rollback

The main risk is stale path ownership. D-057 requires path filters, classifier
rules, fixtures, and the testing matrix to change together; unknown paths fail
closed. Conditional path-filtered workflows are not represented as universal
branch-protection checks.

Before publication, restore the 29 paths from `1c03f66`. After publication,
revert the bounded change, restore the three D-054 workflows and their policy,
then rerun local and hosted validation. The registered self-hosted runner is not
removed, so rollback needs no product, database, dependency, or host rebuild.

## Exact next task

Review and publish only this exact 29-path increment after separate owner
approval. After push, require applicable GitHub-hosted jobs and reconcile any
new report evidence before merge. Do not begin ARB-002 or another increment.
