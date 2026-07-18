# Contributing to Cortexa

## Contribution boundary

Cortexa is pre-production and currently maintainer-controlled. Bug reports,
feature proposals, and sanitized design-review requests are welcome through the
repository issue forms. External code contributions require prior maintainer
coordination while the repository has no selected open-source license. Read
[the licensing decision record](docs/github/LICENSING.md) before submitting
code or redistributing repository content.

Never place credentials, personal content, private files, raw database content,
private paths, or exploitable vulnerability details in an issue or pull request.
Follow [SECURITY.md](SECURITY.md) for private reporting.

## Development workflow

1. Read `AGENTS.md`, `ENGINEERING_GUIDE.md`, and the current project-memory
   files in their required order.
2. Start from clean synchronized `main` and confirm the supported toolchain.
3. Select only the first Ready increment from `NEXT_STEPS.md` unless the project
   owner explicitly selects another bounded task.
4. Declare exact files, acceptance criteria, risks, non-goals, verification,
   and rollback, then wait for approval.
5. Create one descriptive capability branch and begin the mandatory repository
   gate before editing.
6. Implement only the approved scope and run focused checks while working.
7. Run complete verification, review the complete diff, and synchronize project
   memory with actual evidence.
8. Require a valid post-increment marker, then stop. Commit, push, pull-request,
   merge, tag, or release only with explicit project-owner direction.

## Setup

```bash
npm ci
npm run tauri -- dev
```

Preferred toolchain:

```text
Node.js 26.3.0
npm 11.16.0
Rust 1.90.0 with Clippy and rustfmt
```

## Required checks

Use focused commands during development and the complete gate before claiming
completion:

```bash
npm run format:check
npm run lint
npm run typecheck
npm run test
npm run build
npm run docs:check
npm run repository:check
npm run verify
```

Testing details and the change-to-test matrix are in
[TESTING_GUIDE.md](TESTING_GUIDE.md).

## GitHub validation

GitHub Actions supplements the local final increment gate with two read-only,
risk-based workflows:

- `ci.yml` classifies application, dependency, Tauri, IPC, storage, migration,
  permission, security, and workflow changes and runs the affected frontend,
  Rust, and dependency-audit jobs.
- `documentation.yml` validates Markdown, prompts, project memory, and
  repository governance without compiling or testing the application.

Both workflows run on pull requests targeting `main`, pushes to `main`, and
manual dispatch. CI also runs its dependency audit on the weekly schedule.
Because GitHub can leave a path-filtered skipped workflow pending when it is
configured as a required check, branch protection should require only checks
applicable to the changed paths. Reviewers must verify the expected jobs from
`TESTING_GUIDE.md`; use manual dispatch and the complete local gate when scope is
ambiguous.

When repository structure changes, update the workflow trigger paths,
`scripts/ci_change_scope.py`, its fixtures, and the change-to-test matrix in the
same reviewed increment. Unknown non-documentation paths deliberately run both
application jobs.

## Branches and commits

Use one descriptive capability branch. Codex-created branches use the `codex/`
prefix. Examples:

```text
codex/feature/add-agent-memory-store
codex/fix/srm-health-check-timeout
codex/meta-repository-health-github-hygiene
```

Use Conventional Commits that explain the bounded capability:

```text
feat(memory): implement a typed memory repository
fix(gateway): reject a late terminal event
docs(architecture): reconcile the current trust boundary
chore(repository): add read-only quality workflows
```

Generic names such as `update`, `changes`, `misc`, `temp`, and `final` are
prohibited. Dependency upgrades, refactors, and feature behavior remain separate
unless they are inseparable from the approved goal.

## Pull requests

Pull-request titles summarize the capability. The description must use the
repository template and include:

- Purpose
- Files changed
- Testing performed, separated into passed, failed, not run, and manual pending
- Breaking changes, including `None`
- Security and scope review
- Next increment

Pull requests do not bypass the increment gate. CODEOWNERS identifies required
reviewers but does not prove branch protection or approval. Dependabot updates
are review-only proposals; they are never auto-merged by repository workflows.

## Generated and local files

Do not commit `node_modules/`, `dist/`, coverage, Rust targets, local databases,
environment files, logs, backup files, secrets, certificates, or personal test
data. Use synthetic bounded fixtures only. Run:

```bash
npm run repository:check
npm run security:scan
```

## Review references

- [Engineering guide](ENGINEERING_GUIDE.md)
- [Current architecture](ARCHITECTURE.md)
- [Code-review guide](CODE_REVIEW.md)
- [Security checklist](SECURITY_CHECKLIST.md)
- [Release checklist](RELEASE_CHECKLIST.md)
- [GitHub label policy](docs/github/LABELS.md)
- [GitHub milestone policy](docs/github/MILESTONES.md)
