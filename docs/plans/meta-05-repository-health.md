# Meta Increment 5 - repository health and GitHub hygiene

Status: Verified complete; uncommitted and unpublished
Owner: Project maintainer
Last updated: 2026-07-16

## Goal

Establish a production-oriented repository operating surface with honest README
status, contribution and licensing boundaries, owner review paths, structured
issues and pull requests, review-only dependency proposals, least-privilege
quality workflows, reusable health checks, and release-note guidance without
changing Cortexa application behavior.

## User and maintainer outcome

Contributors and maintainers can determine what the product currently does,
submit bounded work, run the same local checks as CI, review dependency and
security findings, and prepare release notes without granting automation write
authority or implying production readiness.

## Exact implementation scope

```text
.gitignore
package.json
scripts/repository_health.py
scripts/cargo_audit_gate.py
scripts/tests/test_repository_health.py
scripts/tests/test_cargo_audit_gate.py
.github/CODEOWNERS
.github/PULL_REQUEST_TEMPLATE.md
.github/ISSUE_TEMPLATE/bug_report.yml
.github/ISSUE_TEMPLATE/feature_request.yml
.github/ISSUE_TEMPLATE/security_review.yml
.github/ISSUE_TEMPLATE/config.yml
.github/dependabot.yml
.github/workflows/ci.yml
.github/workflows/documentation.yml
.github/workflows/security.yml
README.md
CONTRIBUTING.md
ENGINEERING_GUIDE.md
SECURITY.md
SECURITY_CHECKLIST.md
TESTING_GUIDE.md
RELEASE_CHECKLIST.md
CODE_REVIEW.md
docs/github/LICENSING.md
docs/github/LABELS.md
docs/github/MILESTONES.md
docs/templates/RELEASE_NOTES_TEMPLATE.md
```

Declared planning and closeout scope is limited to `AGENTS.md`,
`ARCHITECTURE.md`, `CHANGELOG.md`, `DECISIONS.md`, `HANDOFF.md`,
`NEXT_STEPS.md`, `PLANS.md`, `PROJECT_STATUS.md`, `ROADMAP.md`, this plan,
`docs/plans/README.md`, the Meta 6 icon-plan rename, one Meta 5 increment record,
and one dated Meta 5 review.

## Implementation rules

- GitHub workflows use only SHA-pinned official checkout and Node setup actions,
  top-level `contents: read`, disabled persisted credentials, and no secret
  context.
- No workflow uses `pull_request_target`, commits, pushes, merges, publishes,
  deploys, signs, notarizes, or begins another increment.
- Dependabot proposes npm, Cargo, and GitHub Action updates on review branches;
  it does not rebase automatically or auto-merge.
- Local health scripts use only the Python standard library, fixed Git
  inspection arguments, bounded text reads, redacted finding output, and
  explicit exit codes.
- Cargo audit accepts only D-025's exact two-vulnerability baseline and D-046's
  exact 18-warning baseline for the unchanged lockfile. Any new, changed,
  missing, malformed, or tool-failure state fails; accepted findings remain
  visible and unresolved.
- The licensing record states that no license is selected. It does not invent
  legal ownership or grant rights.

## Risks

- Untrusted pull-request code executes tests. Read-only permissions, no secrets,
  and disabled checkout credentials limit impact but do not make code trusted.
- macOS CI consumes more runner time and can fail because of runner-image drift.
- Pattern scanners can miss unknown secret formats or produce false positives;
  they remain defense in depth.
- The accepted RustSec baseline can drift. The closed parser fails on any
  identity or version change and requires a new decision rather than silent
  acceptance.
- CODEOWNERS, labels, milestones, and workflow files do not prove remote branch
  protection or repository settings.
- GitHub-hosted execution cannot be observed before publication; local command
  and YAML validation must be recorded accurately.

## Explicit non-goals

- Application source, test behavior, runtime, Tauri commands, capabilities,
  CSP, permissions, entitlements, or window behavior.
- SQLite schema, migrations, data, credentials, provider networking, gateway,
  policy, approval, audit, dispatch, or execution.
- Dependency, manifest, or lockfile version changes.
- Deployment, publishing, release artifacts, signing, notarization, installers,
  automatic merge, or default-branch writes.
- Remote branch protection, labels, milestones, repository settings, GitHub
  Advanced Security, or secret configuration.
- Selecting an open-source or commercial license.
- Executive documentation, application icons, Increment 4V, or another product
  increment.

## Verification

```text
npm run test:repository
npm run docs:check
npm run repository:check
npm run security:scan
ruby YAML syntax validation for every .github YAML file
npm run format:check
npm run lint
npm run typecheck
npm run test
npm run build
npm run verify
npm audit --audit-level=low
cargo-audit 0.22.2 JSON plus scripts/cargo_audit_gate.py
git diff --check
python3 .codex/hooks/session_end_gate.py
complete architecture, security, code-health, debt, readiness, and diff review
mandatory meta-05 post-increment gate
```

No product or native manual check applies because no application or icon path
changes. GitHub-hosted runs are Not run until publication and are not a
completion gate for this uncommitted increment; every workflow command must pass
locally where practical.

## Rollback

Before commit, restore modified tracked files to `ad9042c`, delete only the new
Meta 5 files, and restore the Meta 6 icon plan to its Meta 4 path. After commit,
revert one bounded Meta 5 commit. If published, cancel any active workflow and
revert the commit; no migration, data, credential, package, signing identity, or
remote repository setting requires rollback.

## Acceptance criteria

- [x] Exact approved scope only.
- [x] README is branded, honest, complete, and explicitly pre-production.
- [x] GitHub templates and guidance preserve private security reporting.
- [x] Workflows use read-only permissions, immutable actions, no secrets, and no
      write or publication behavior.
- [x] npm, Cargo, and action dependency proposals remain review-only.
- [x] Repository health checks cover every requested category with positive and
      negative tests.
- [x] Licensing status is explicit without inventing terms.
- [x] Complete local verification, audits, links, YAML, diff, and mandatory gate
      pass.
- [x] Meta Increment 6 is Ready but not implemented.
