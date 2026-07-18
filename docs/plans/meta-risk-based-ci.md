# Meta Increment - risk-based GitHub Actions validation

Status: Complete locally; publication and GitHub-hosted execution pending

## Goal

Replace blanket persistent-runner verification with two read-only,
GitHub-hosted workflows that select documentation, frontend, Rust, and
dependency-audit checks from the affected repository paths while preserving the
complete local increment gate.

## Current baseline

- Clean synchronized `main` began at `1c03f66`.
- D-054 routes three push-only workflows to one persistent Linux runner.
- D-056 defines local risk-based validation but the hosted CI workflow still
  runs `npm run verify` for every eligible source or documentation push.
- The repository pins Node.js `26.3.0`, npm `11.16.0`, and Rust `1.90.0` with
  Clippy and rustfmt.
- `npm run test:repository`, `npm run docs:check`, and YAML parsing passed
  before implementation.

## Exact implementation scope

```text
.github/workflows/ci.yml
.github/workflows/documentation.yml
.github/workflows/security.yml
scripts/ci_change_scope.py
scripts/repository_health.py
scripts/tests/test_ci_change_scope.py
scripts/tests/test_repository_health.py
package.json
README.md
AGENTS.md
ENGINEERING_GUIDE.md
TESTING_GUIDE.md
SECURITY.md
SECURITY_CHECKLIST.md
CODE_REVIEW.md
CONTRIBUTING.md
prompts/increments/verified-increment.md
docs/github/SELF_HOSTED_RUNNER.md
DECISIONS.md
docs/plans/meta-risk-based-ci.md
docs/plans/README.md
docs/increments/meta-risk-based-ci.md
docs/reviews/2026-07-18-meta-risk-based-ci-post-increment-review.md
HANDOFF.md
PROJECT_STATUS.md
NEXT_STEPS.md
CHANGELOG.md
PLANS.md
ROADMAP.md
```

`.github/workflows/security.yml` is deleted only after its weekly npm and Rust
advisory audit is preserved as the conditional `dependency-audit` job in
`ci.yml`.

## Workflow behavior

Application CI uses explicit pull-request and push path filters. One
standard-library classifier compares fixed Git SHAs and emits closed frontend,
Rust, and audit booleans. Documentation-only paths produce no application job;
unknown non-documentation paths fail closed to both application jobs.

| Change class                                                  | GitHub-hosted jobs                               |
| ------------------------------------------------------------- | ------------------------------------------------ |
| Documentation only                                            | Documentation                                    |
| Frontend or application brand asset                           | Classifier and frontend                          |
| Rust tests or examples                                        | Classifier and Rust                              |
| IPC, Tauri, policy, approval, storage, migration, or security | Classifier, frontend, Rust, and dependency audit |
| Repository governance validator                               | Classifier, dependency audit, and Documentation  |
| Documentation workflow                                        | Classifier, dependency audit, and Documentation  |
| Repository hooks                                              | Classifier and dependency audit                  |
| Dependency or CI workflow                                     | Classifier, frontend, Rust, and dependency audit |
| Mixed source and documentation                                | Applicable application jobs and Documentation    |
| Scheduled CI                                                  | Classifier and dependency audit                  |
| Manual CI dispatch                                            | All application jobs                             |

Both workflows use `ubuntu-latest`, top-level `contents: read`, immutable
official action SHAs, non-persistent checkout credentials, bounded timeouts,
and concurrency cancellation. No workflow receives secrets or writes,
publishes, deploys, signs, notarizes, or caches build output.

## Linux requirements

The Rust job installs only the current Tauri Debian/Ubuntu build requirements:
build tools, WebKitGTK 4.1, Ayatana AppIndicator, librsvg, OpenSSL, libxdo, and
pkg-config. It runs formatting, strict Clippy, and all Rust targets. Frontend
formatting, lint, type checking, tests, and production build remain separate.

## Non-goals

- No application source or product behavior.
- No dependency version or lockfile change.
- No Tauri configuration, command, capability, CSP, permission, or SQLite
  change.
- No deployment, publishing, signing, notarization, or artifact upload.
- No macOS hosted job or claim of target-Mac native verification.
- No remote branch-protection, repository-secret, billing, or runner
  administration change.
- No self-hosted-runner deregistration.

## Risks and controls

- Path filters can become stale when repository structure changes. Exact
  workflow-policy checks, classifier fixtures, fail-closed unknown paths, and a
  documented update rule mitigate this risk.
- GitHub evaluates path filters with platform limits. Oversized or ambiguous
  changes require manual dispatch and the complete local increment gate.
- Path-filtered checks cannot be unconditional branch-protection requirements
  because skipped checks may remain pending. Applicable checks remain a merge
  review requirement; no unsupported remote enforcement claim is made.
- GitHub-hosted billing or availability can prevent remote execution. Local
  completion evidence remains required, and D-054's configured runner is kept
  available as rollback infrastructure.
- Pull-request code is untrusted. It runs only on ephemeral hosted runners with
  read-only permissions, no secrets, immutable actions, and disabled checkout
  credentials.

## Verification

```bash
python3 -m unittest scripts.tests.test_ci_change_scope -v
python3 -m unittest scripts.tests.test_repository_health -v
npm run test:repository
ruby -e 'require "yaml"; Dir[".github/workflows/*.{yml,yaml}"].sort.each { |path| YAML.parse_file(path) }'
npm run docs:check
npm run repository:check
npm run security:scan
npm run verify
npm audit --audit-level=low
git diff --check
python3 .codex/hooks/session_end_gate.py
python3 .codex/hooks/post_increment_gate.py status
```

Run the pinned Rust advisory command locally where practical. GitHub-hosted
execution remains pending until a separately approved publication step.

## Manual verification

No product or target-Mac manual check applies. After publication, confirm the
two workflow trigger matrix and job selection on actual GitHub runs before
treating hosted behavior as verified.

## Rollback

Before commit, restore the declared paths from `1c03f66`. After publication,
revert the bounded increment, restore the three D-054 workflows and repository
policy, and rerun local and hosted checks. No product, dependency, database, or
native rollback is required.

## Exit criteria

- Exactly two active workflows implement the approved matrix.
- Security audit coverage is preserved without a third workflow.
- Focused classifier and repository-policy tests pass.
- Complete local verification passes after the final executable edit.
- Documentation and project memory match observed evidence.
- The mandatory report is PASS or PASS WITH ADVISORIES and the
  `meta-risk-based-ci` marker is valid.
- No commit, push, merge, or later increment starts automatically.
