# Repository self-hosted runner post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "git status --short --branch",
    "git log --oneline --decorate -8",
    "python3 .codex/hooks/post_increment_gate.py begin --increment repository-self-hosted-runner",
    "gh api repos/SillyRbbit/ai-agent-assistant/actions/runners",
    "gh api --method POST repos/SillyRbbit/ai-agent-assistant/actions/runners/21/labels -f 'labels[]=cortexa-ci'",
    "gh pr view 23 --repo SillyRbbit/ai-agent-assistant --json number,title,state,isDraft,headRefName,headRefOid,baseRefName,mergeStateStatus,statusCheckRollup,url",
    "npm run test:repository",
    "npm run docs:check",
    "npm run repository:check",
    "ruby -e 'require \"yaml\"; ARGV.each { |path| YAML.parse_file(path) or abort(\"empty YAML: #{path}\") }; puts \"workflow-yaml: PASS (#{ARGV.length} files)\"' .github/workflows/ci.yml .github/workflows/documentation.yml .github/workflows/security.yml",
    "npm run security:scan",
    "npm run verify",
    "npx prettier --write docs/plans/repository-self-hosted-runner.md",
    "git diff --check",
    "python3 .codex/hooks/session_end_gate.py",
    "python3 .codex/hooks/post_increment_gate.py status",
    "gh api repos/SillyRbbit/ai-agent-assistant/actions/runners/21 --jq '{id, name, os, status, busy, version, labels: [.labels[].name]}'",
    "complete architecture, security, code-health, technical-debt, readiness, and diff review"
  ],
  "files_changed": [
    ".github/workflows/ci.yml",
    ".github/workflows/documentation.yml",
    ".github/workflows/security.yml",
    "AGENTS.md",
    "CHANGELOG.md",
    "DECISIONS.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "SECURITY.md",
    "SECURITY_CHECKLIST.md",
    "TESTING_GUIDE.md",
    "TROUBLESHOOTING_LOG.md",
    "docs/github/SELF_HOSTED_RUNNER.md",
    "docs/increments/repository-self-hosted-runner.md",
    "docs/plans/README.md",
    "docs/plans/repository-self-hosted-runner.md",
    "docs/reviews/2026-07-17-repository-self-hosted-runner-post-increment-review.md",
    "scripts/repository_health.py",
    "scripts/tests/test_repository_health.py"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Security",
      "effort": "Large: provision disposable hosts or an ephemeral runner lifecycle with preserved external diagnostics",
      "milestone": "Before accepting untrusted pull requests or storing any sensitive material on runner infrastructure",
      "risk": "A trusted writer can change and push workflow code, and a persistent runner does not guarantee a clean isolated machine between jobs",
      "severity": "Advisory",
      "summary": "Persistent self-hosted runner isolation remains operator-controlled"
    }
  ],
  "increment_id": "repository-self-hosted-runner",
  "manual_verification": [
    {
      "check": "Commit and push the reviewed branch, then confirm CI, Documentation, and Security execute on runner 21 and pass on the final commit",
      "required": true,
      "status": "Manual verification pending"
    },
    {
      "check": "Launch and inspect the native Cortexa application on a target Mac",
      "required": false,
      "status": "Not run"
    }
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "FAIL",
  "schema_version": 1,
  "verification": [
    {
      "command": "npm run test:repository",
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
      "command": "ruby -e 'require \"yaml\"; ARGV.each { |path| YAML.parse_file(path) or abort(\"empty YAML: #{path}\") }; puts \"workflow-yaml: PASS (#{ARGV.length} files)\"' .github/workflows/ci.yml .github/workflows/documentation.yml .github/workflows/security.yml",
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
      "command": "git diff --check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "gh api repos/SillyRbbit/ai-agent-assistant/actions/runners/21 --jq '{id, name, os, status, busy, version, labels: [.labels[].name]}'",
      "required": true,
      "status": "Passed"
    }
  ]
}
-->

Date: 2026-07-17
Increment: Repository Workflow Increment - trusted self-hosted runner routing
Branch: `codex/repository/use-self-hosted-runner`

## Executive summary

The bounded repository workflow change routes CI, Documentation, and Security
to runner 21 through the exact `cortexa-ci` selector, removes all
`pull_request` triggers from persistent-runner workflows, and limits push
execution to documented maintainer-controlled branch families. Local checks
pass and no product source changed.

Completion is blocked because the branch is uncommitted and unpushed by policy,
so the three workflows have not executed on the runner. The quality-gate result
is `FAIL`; no completion marker may be written.

## Scope and boundaries

The implementation stays within the declared three-workflow, repository-policy,
focused-test, operating-guide, plan, and project-memory scope. It adds no
application source, dependency, lockfile, Tauri command, IPC, CSP, capability,
permission, SQLite schema, identifier, secret, signing, deployment, or product
behavior.

The original draft used a job-level pull-request condition. Security review
found that an untrusted PR can modify its own workflow definition, so that
condition was not a reliable boundary. The final diff removes `pull_request`
entirely and adds a regression that rejects reintroduction while the persistent
runner is selected.

## Verification results

Passed:

- `npm run test:repository`: 19 tests passed, including exact selector,
  allowlisted push, generic-selector rejection, and pull-request rejection.
- All three workflow files parsed as YAML.
- `npm run docs:check`, `npm run repository:check`, and
  `npm run security:scan` passed.
- `npm run verify` passed formatting, policy, ESLint, strict Clippy, 28 hook
  tests, 19 repository tests, 124 frontend tests, 95 Rust library tests, 21 Rust
  integration tests, typecheck, both Vite builds, and the Tauri release
  no-bundle build.
- `git diff --check` passed.
- Runner 21 remained online, idle, version `2.335.1`, with `self-hosted`,
  `Linux`, `X64`, and `cortexa-ci` labels.
- Session-end inventory found no staged paths or conflicts and exactly the
  declared workflow, policy, test, plan, closeout, and report paths.

Failed: no required local command remains failed. Two intermediate checks
stopped only on Prettier wrapping in the new plan and passed after formatting.

Not run / manual pending:

- Required CI, Documentation, and Security execution on runner 21 is pending
  explicit commit and push approval.
- Native application inspection is not required for this repository-only
  change and was not run.

## Architecture findings

No product architecture finding. The change affects only repository automation.
Product trust boundaries, module ownership, IPC, Rust core, WebView, storage,
gateway, approval, audit, and platform architecture remain unchanged. Linux CI
is explicitly not represented as target-Mac native evidence.

## Security findings

No Critical or High finding remains. The blocking draft flaw was corrected by
removing `pull_request` from every self-hosted workflow. Workflow permissions
remain `contents: read`; actions remain immutable; checkout credentials do not
persist; no secrets, write operation, `sudo`, publication, deployment, signing,
or `pull_request_target` was added.

One Advisory remains: a persistent runner can be compromised by code pushed by
a trusted writer and is not an ephemeral clean machine. The operating guide
therefore requires a dedicated unprivileged credential-free host and recommends
ephemeral infrastructure before accepting untrusted PR execution.

## Code-health findings

No blocking correctness or maintainability finding. The repository-health rule
matches the exact workflow contract and has one positive and two negative
regressions. The checks are standard-library only, deterministic, read-only,
and emit no sensitive content. No application accessibility or UI path changed.

## Technical debt

- Category: Security.
- Severity: Advisory.
- Summary: persistent runner isolation remains operator-controlled.
- Risk: trusted workflow code can persist changes on the host between jobs.
- Effort: Large for an ephemeral runner lifecycle with clean host provisioning
  and externally retained diagnostics.
- Milestone: before untrusted pull-request execution or any sensitive runner
  infrastructure.
- Blocks completion: No.
- Blocks next increment: No after required remote checks pass.

## Roadmap findings

The runner workflow increment is not complete until the final branch content
executes successfully in all three workflows. Increment 4V / PR #23 must remain
untouched and unmerged until this workflow increment is verified and merged.
No later product or remediation increment is Ready to start.

## Completion decision

`FAIL`

Required remote execution remains pending. The report must not be finalized and
the active gate must not be marked complete.

## Next-increment readiness

`Blocked`

The exact next task is project-owner approval to commit and push the bounded
runner branch. After all three workflows pass on runner 21, update this report,
rerun final verification, finalize a valid marker, and seek separate merge
approval. Do not modify PR #23.

## Exact files changed

```text
.github/workflows/ci.yml
.github/workflows/documentation.yml
.github/workflows/security.yml
AGENTS.md
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
SECURITY.md
SECURITY_CHECKLIST.md
TESTING_GUIDE.md
TROUBLESHOOTING_LOG.md
docs/github/SELF_HOSTED_RUNNER.md
docs/increments/repository-self-hosted-runner.md
docs/plans/README.md
docs/plans/repository-self-hosted-runner.md
docs/reviews/2026-07-17-repository-self-hosted-runner-post-increment-review.md
scripts/repository_health.py
scripts/tests/test_repository_health.py
```

## Exact commands executed

The machine manifest records the exact bounded setup, verification, runner API,
Git inspection, review, and gate commands. All required local commands passed.
The remote runner execution command is intentionally absent because no branch
has been pushed and no setup PR exists.
