# Meta Increment - risk-based GitHub Actions validation

Status: Dual-runner publication correction verified locally; publication pending

## Goal

Preserve two read-only risk-based workflows while routing trusted validation
across the registered Linux and macOS `cortexa-ci` runners. Keep untrusted
pull-request events off persistent hosts and preserve the complete local
increment gate.

## Current baseline

- Clean synchronized `main` began at `1c03f66`.
- D-054 routes three push-only workflows to one persistent Linux runner.
- D-056 defines local risk-based validation but the hosted CI workflow still
  runs `npm run verify` for every eligible source or documentation push.
- The repository pins Node.js `26.3.0`, npm `11.16.0`, and Rust `1.90.0` with
  Clippy and rustfmt.
- `npm run test:repository`, `npm run docs:check`, and YAML parsing passed
  before implementation.
- The first PR #30 hosted runs `29665772834` and `29665772844` failed before
  runner allocation because the account Actions minute or spending limit was
  exhausted. No workflow step executed.
- GitHub runner inspection found Linux runner 21 and macOS runner 22 online,
  idle, and carrying the exact `cortexa-ci` selectors.

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

The D-058 publication correction changes exactly 22 paths: 21 existing paths
are modified and one dedicated D-058 report is added:

```text
.github/workflows/ci.yml
.github/workflows/documentation.yml
AGENTS.md
CHANGELOG.md
CODE_REVIEW.md
DECISIONS.md
ENGINEERING_GUIDE.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
ROADMAP.md
SECURITY.md
SECURITY_CHECKLIST.md
TESTING_GUIDE.md
docs/github/SELF_HOSTED_RUNNER.md
docs/increments/meta-risk-based-ci.md
docs/plans/meta-risk-based-ci.md
docs/reviews/2026-07-18-repository-dual-self-hosted-runner-routing-post-increment-review.md
scripts/repository_health.py
scripts/tests/test_ci_change_scope.py
scripts/tests/test_repository_health.py
```

## Workflow behavior

Application CI uses explicit trusted-push path filters. One
standard-library classifier compares fixed Git SHAs and emits closed frontend,
Rust, and audit booleans. Documentation-only paths produce no application job;
unknown non-documentation paths fail closed to both application jobs.

| Change class                                                  | Jobs and runner                                   |
| ------------------------------------------------------------- | ------------------------------------------------- |
| Documentation only                                            | Documentation on Linux                            |
| Frontend or application brand asset                           | Classifier and frontend on Linux                  |
| Rust tests or examples                                        | Classifier and Rust on Linux and macOS            |
| IPC, Tauri, policy, approval, storage, migration, or security | Frontend, both Rust jobs, and audit as classified |
| Repository governance validator                               | Classifier, audit, and Documentation on Linux     |
| Documentation workflow                                        | Classifier, audit, and Documentation on Linux     |
| Repository hooks                                              | Classifier and audit on Linux                     |
| Dependency or CI workflow                                     | Frontend, both Rust jobs, and audit               |
| Mixed source and documentation                                | Applicable application jobs and Documentation     |
| Scheduled CI                                                  | Classifier and dependency audit on Linux          |
| Manual CI dispatch                                            | All application jobs across both runners          |

Linux jobs use `[self-hosted, Linux, X64, cortexa-ci]`; target-Mac Rust uses
`[self-hosted, macOS, X64, cortexa-ci]`. Both workflows keep top-level
`contents: read`, immutable official action SHAs, non-persistent checkout
credentials, bounded timeouts, and concurrency cancellation. No workflow
receives secrets or writes, publishes, deploys, signs, notarizes, uses `sudo`,
or caches build output.

## Linux requirements

The Linux host is preprovisioned outside workflow execution with build tools,
WebKitGTK 4.1, Ayatana AppIndicator, librsvg, OpenSSL, libxdo, and pkg-config.
The workflow fails fast if required packages are absent and never invokes
`sudo`. Linux runs formatting, strict Clippy, and all Rust targets; macOS runs
strict Clippy and all Rust targets for target-gated native coverage.

## Non-goals

- No application source or product behavior.
- No dependency version or lockfile change.
- No Tauri configuration, command, capability, CSP, permission, or SQLite
  change.
- No deployment, publishing, signing, notarization, or artifact upload.
- No native UI, signing, notarization, installer, or release claim from the
  target-Mac Rust job.
- No remote branch-protection, repository-secret, billing, or runner
  administration change.
- No runner registration, label, service, host package, or account change.
- No modification to the classifier implementation; only one push-range
  regression fixture is added for the active trigger mode.

## Risks and controls

- Path filters can become stale when repository structure changes. Exact
  workflow-policy checks, classifier fixtures, fail-closed unknown paths, and a
  documented update rule mitigate this risk.
- GitHub evaluates path filters with platform limits. Oversized or ambiguous
  changes require manual dispatch and the complete local increment gate.
- Path-filtered checks cannot be unconditional branch-protection requirements
  because skipped checks may remain pending. Applicable checks remain a merge
  review requirement; no unsupported remote enforcement claim is made.
- Persistent hosts retain state and are not security boundaries. Exact runner
  selectors, dedicated unprivileged accounts, no secrets, no `sudo`, and host
  maintenance reduce but do not remove that risk.
- Pull-request code is untrusted and never receives either runner directly.
  Reviewed changes must be reproduced on an allowlisted repository branch;
  missing checks are not approval to merge.

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

Run the pinned Rust advisory command locally where practical. Actual Linux and
macOS runner execution remains pending until a separately approved publication
step.

## Manual verification

The project owner confirmed both runner services satisfy D-058's dedicated,
unprivileged host baseline. No product manual check applies. After publication,
confirm trusted push selection, Linux job assignment to runner 21, target-Mac
Rust assignment to runner 22, and absence of direct pull-request execution.

## Rollback

Before the correction is committed, restore its declared paths from `4fb7f31`.
After publication, revert D-058 to the D-057 hosted selectors only after Actions
minutes or billing are available, then rerun local and remote checks. No
product, dependency, database, or native rollback is required.

## Exit criteria

- Exactly two active workflows implement the approved matrix.
- Security audit coverage is preserved without a third workflow.
- Persistent runners receive no pull-request trigger, secrets, writes, or
  in-workflow host provisioning.
- Focused classifier and repository-policy tests pass.
- Complete local verification passes after the final executable edit.
- Documentation and project memory match observed evidence.
- The mandatory report is PASS or PASS WITH ADVISORIES and the
  `repository-dual-self-hosted-runner-routing` marker is valid.
- No commit, push, merge, or later increment starts automatically.
