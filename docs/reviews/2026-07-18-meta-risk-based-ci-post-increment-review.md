# Meta risk-based GitHub Actions validation post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "git status --short --branch",
    "git log -5 --oneline --decorate",
    "npm run test:repository",
    "npm run docs:check",
    "ruby -e 'require \"yaml\"; Dir[\".github/workflows/*.{yml,yaml}\"].sort.each { |path| YAML.load_file(path, aliases: true); puts \"parsed #{path}\" }' (failed: system Ruby does not support aliases keyword)",
    "ruby -e 'require \"yaml\"; Dir[\".github/workflows/*.{yml,yaml}\"].sort.each { |path| YAML.safe_load(File.read(path), [], [], true); puts \"parsed #{path}\" }'",
    "python3 .codex/hooks/post_increment_gate.py begin --increment meta-risk-based-ci (sandboxed attempt: state write denied)",
    "python3 .codex/hooks/post_increment_gate.py begin --increment meta-risk-based-ci (approved elevated retry: passed)",
    "python3 -m unittest scripts.tests.test_ci_change_scope -v",
    "python3 -m unittest scripts.tests.test_repository_health -v",
    "npm run repository:check",
    "npm run test:hooks",
    "npm run security:scan",
    "npm run format:frontend",
    "python3 -c 'from pathlib import Path; paths=[Path(\"scripts/ci_change_scope.py\"),Path(\"scripts/repository_health.py\"),Path(\"scripts/tests/test_ci_change_scope.py\"),Path(\"scripts/tests/test_repository_health.py\")]; [compile(p.read_text(encoding=\"utf-8\"), str(p), \"exec\") for p in paths]; print(\"python syntax: PASS\")'",
    "npx prettier --write .github/workflows/ci.yml .github/workflows/documentation.yml AGENTS.md CODE_REVIEW.md CONTRIBUTING.md DECISIONS.md ENGINEERING_GUIDE.md README.md SECURITY.md SECURITY_CHECKLIST.md TESTING_GUIDE.md docs/github/SELF_HOSTED_RUNNER.md docs/plans/README.md docs/plans/meta-risk-based-ci.md package.json prompts/increments/verified-increment.md",
    "npm ci",
    "npm run verify",
    "cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked",
    "npm audit --audit-level=low",
    "cargo install cargo-audit --version 0.22.2 --locked --root /tmp/meta-risk-based-ci-cargo-audit",
    "PATH=/tmp/meta-risk-based-ci-cargo-audit/bin:$PATH cargo audit --file src-tauri/Cargo.lock --json > /tmp/meta-risk-based-ci-cargo-audit/report.json",
    "python3 scripts/cargo_audit_gate.py /tmp/meta-risk-based-ci-cargo-audit/report.json --cargo-audit-exit 1",
    "npm ci --ignore-scripts",
    "python3 scripts/ci_change_scope.py diff-check --event pull_request --base \"$(git rev-parse HEAD^)\" --head \"$(git rev-parse HEAD)\"",
    "gh api --method GET repos/actions/checkout/commits/9c091bb21b7c1c1d1991bb908d89e4e9dddfe3e0 --jq '.sha + \" \" + .commit.verification.reason' && gh api --method GET repos/actions/setup-node/commits/820762786026740c76f36085b0efc47a31fe5020 --jq '.sha + \" \" + .commit.verification.reason'",
    "gh api --method GET repos/actions/checkout/git/ref/tags/v7.0.0 --jq '.object.sha' && gh api --method GET repos/actions/setup-node/git/ref/tags/v7.0.0 --jq '.object.sha'",
    "git diff --check",
    "git diff --exit-code -- src src-tauri tests package-lock.json .codex .agents",
    "python3 .codex/hooks/session_end_gate.py",
    "python3 .codex/hooks/post_increment_gate.py finalize --increment meta-risk-based-ci --report docs/reviews/2026-07-18-meta-risk-based-ci-post-increment-review.md (sandboxed attempt: state write denied)",
    "python3 .codex/hooks/post_increment_gate.py finalize --increment meta-risk-based-ci --report docs/reviews/2026-07-18-meta-risk-based-ci-post-increment-review.md (approved elevated retry: passed)",
    "python3 .codex/hooks/post_increment_gate.py status (complete, valid: true)",
    "complete architecture, security, code-health, technical-debt, roadmap-readiness, and diff review"
  ],
  "files_changed": [
    ".github/workflows/ci.yml",
    ".github/workflows/documentation.yml",
    ".github/workflows/security.yml",
    "AGENTS.md",
    "CHANGELOG.md",
    "CODE_REVIEW.md",
    "CONTRIBUTING.md",
    "DECISIONS.md",
    "ENGINEERING_GUIDE.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "README.md",
    "ROADMAP.md",
    "SECURITY.md",
    "SECURITY_CHECKLIST.md",
    "TESTING_GUIDE.md",
    "docs/github/SELF_HOSTED_RUNNER.md",
    "docs/increments/meta-risk-based-ci.md",
    "docs/plans/README.md",
    "docs/plans/meta-risk-based-ci.md",
    "docs/reviews/2026-07-18-meta-risk-based-ci-post-increment-review.md",
    "package.json",
    "prompts/increments/verified-increment.md",
    "scripts/ci_change_scope.py",
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
      "milestone": "Publication of the exact verified increment",
      "risk": "Local tests and static review cannot prove hosted runner availability, event-path behavior, Linux package installation, or remote required-check configuration.",
      "severity": "Advisory",
      "summary": "Actual GitHub-hosted execution and remote enforcement remain pending publication."
    }
  ],
  "increment_id": "meta-risk-based-ci",
  "manual_verification": [
    {
      "check": "Inspect applicable CI and Documentation trigger and job selection on GitHub-hosted runners after publication.",
      "required": false,
      "status": "Manual verification pending"
    }
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {
      "command": "npm ci",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm ci --ignore-scripts",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run test:repository",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "ruby -e 'require \"yaml\"; Dir[\".github/workflows/*.{yml,yaml}\"].sort.each { |path| YAML.safe_load(File.read(path), [], [], true); puts \"parsed #{path}\" }'",
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
      "command": "npm audit --audit-level=low",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 scripts/cargo_audit_gate.py /tmp/meta-risk-based-ci-cargo-audit/report.json --cargo-audit-exit 1",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "git diff --check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "git diff --exit-code -- src src-tauri tests package-lock.json .codex .agents",
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
Increment: Meta risk-based GitHub Actions validation
Branch: `main`

## Executive summary

The repository now has exactly two read-only, risk-based workflows on ephemeral
GitHub-hosted Linux runners. Application CI classifies changed paths into
frontend, Rust, and dependency-audit jobs; Documentation validates Markdown,
prompts, project memory, paths, YAML, and repository governance without running
application builds. The complete local final increment gate remains mandatory.

The result is `PASS WITH ADVISORIES`. Every required local check passed, no
product source or dependency changed, and no blocking finding exists. Actual
GitHub-hosted execution and remote required-check behavior remain pending until
publication.

## Scope and boundaries

The complete change set matches the approved 29 paths. It changes only GitHub
workflows, standard-library repository validation, focused tests, compositional
package scripts, and coordinating governance/closeout documents. The former
Security workflow is deleted only after its weekly npm/Rust audit is preserved
inside CI.

No application source, product test contract, dependency version, lockfile,
Tauri configuration, IPC command, capability, CSP, permission, SQLite migration,
database, identifier, credential, deployment, signing, or product behavior
changed. The registered D-054 runner remains available but unselected as
rollback infrastructure.

## Verification results

Passed:

- Clean regular and ignore-scripts npm installs from the committed lockfile.
- YAML parsing, Python syntax, fixed-range whitespace checking, documentation,
  repository policy, secret scan, and complete diff checks.
- Sixteen classifier cases, 37 repository tests, and 28 hook tests.
- Final `npm run verify`: frontend formatting/lint/typecheck, strict Clippy, 124
  frontend tests, 96 Rust library tests, 21 Rust integration tests, production
  frontend builds, and Tauri release no-bundle build.
- Exact all-target Rust tests, including the native-dialog example target.
- npm audit with zero vulnerabilities.
- Pinned Cargo audit with only D-025's exact two vulnerabilities and D-046's
  exact 18 warnings accepted by the existing gate.
- GitHub API verification that both full action SHAs are signed and match their
  official `v7.0.0` tags.
- Protected-path proof showing no application source, lockfile, hook, or skill
  change.

Failed required checks: none. The first Ruby parse command used an unsupported
keyword on system Ruby; the supported `safe_load` invocation passed. The first
sandboxed npm audit lacked network DNS; the approved network-enabled retry
passed with zero vulnerabilities. Cargo audit itself exited one for the known
accepted findings, and the exact baseline gate passed as designed. The first
marker finalization validated the report but could not write sandbox-protected
ignored state; the approved elevated retry completed successfully.

Checks not run: the Ubuntu `apt-get` package step and actual GitHub-hosted jobs
cannot execute on the local target Mac. No native application launch or UI check
is required because product files and behavior are unchanged.

Manual verification pending: inspect applicable hosted workflow triggers and
job selection after publication. It is non-mandatory for local completion and
is retained as an advisory rather than reported as passed.

## Architecture findings

None. Product modules, data flow, IPC, persistence, platform ownership, and
runtime portability are unchanged. Workflow responsibility is cohesive: one
classifier owns application path selection, while repository health owns static
workflow policy. Local and hosted responsibilities are explicitly separate.

## Security findings

No blocking finding. Pull-request code moves from a persistent runner to
ephemeral hosted runners with read-only permissions, no secret context,
immutable action SHAs, disabled checkout credentials, and no write or
publication operations. Git arguments are fixed; event SHAs and Git paths are
validated; classifier output is a closed Boolean set. Unknown
non-documentation paths fail closed, and security-sensitive paths run both
application jobs plus the audit.

The remaining Advisory is unverified hosted execution and remote enforcement.
Workflow success will not prove target-Mac native or release behavior.

## Code-health findings

None. The classifier is standard-library-only and covered across every declared
path/event class, including deletion and unsafe-path rejection. Package scripts
compose existing tools to avoid duplicate frontend and Rust work. Repository
health rejects unexpected workflows, self-hosted selectors, mutable actions,
write permissions, missing triggers, missing concurrency, and stale prompt
metadata/paths.

## Technical debt

One Advisory: path ownership can drift when repository structure changes. The
risk is an incorrectly skipped job; effort is Small, the maintenance milestone
is each structure-changing increment, and it blocks neither completion nor the
next task. D-057 requires event paths, classifier patterns, fixtures, and the
testing matrix to change together; unknown paths fail closed.

## Roadmap findings

The repository-governance increment does not reorder product work. Increment 4V
remains published. ARB-002 remains Blocked on O-006/O-007 and project,
security, and executive owner decisions. The exact next task is publication
review for this bounded change, not another implementation increment.

## Completion decision

`PASS WITH ADVISORIES`. Every required local verification passed, no manual
completion gate is pending, no Critical or High finding exists, and the exact
29-path scope preserves all product and security boundaries. Hosted execution
remains explicitly unverified until publication.

## Next-increment readiness

`Blocked`. No later product or remediation increment is Ready. Review and
publish only this locally verified scope after separate owner approval; do not
begin ARB-002.

## Exact files changed

The machine manifest contains all 29 changed paths, including the reviewed
Security workflow deletion, two new standard-library classifier files, the plan,
increment record, this report, and every declared governance closeout file. No
other path changed.

## Exact commands executed

The machine manifest records the relevant inspection, baseline, focused,
formatting, install, complete verification, audit, action-verification,
protected-scope, session-end, and engineering-review commands. The resolved Ruby
and sandbox-network failures, expected nonzero raw Cargo audit, and final passing
results are distinguished above.
