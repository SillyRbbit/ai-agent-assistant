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
    "git add -A && git diff --cached --check && git diff --cached --stat && git status --short",
    "git commit -m \"ci(actions): route trusted checks to self-hosted runner\"",
    "git push -u origin codex/repository/use-self-hosted-runner",
    "gh pr create --repo SillyRbbit/ai-agent-assistant --base main --head codex/repository/use-self-hosted-runner --title \"ci: Route trusted repository checks to self-hosted runner\" --body <approved descriptive body>",
    "gh pr checks 24 --repo SillyRbbit/ai-agent-assistant --watch --interval 10",
    "gh run view 29624042656 --repo SillyRbbit/ai-agent-assistant --log-failed",
    "gh run view 29624042629 --repo SillyRbbit/ai-agent-assistant --log-failed",
    "gh run rerun 29624042629 --repo SillyRbbit/ai-agent-assistant --failed",
    "gh run rerun 29624042656 --repo SillyRbbit/ai-agent-assistant --failed",
    "gh run view 29624042629 --repo SillyRbbit/ai-agent-assistant --attempt 2 --log-failed",
    "gh run view 29624042656 --repo SillyRbbit/ai-agent-assistant --attempt 2 --log-failed",
    "gh run view 29624042629 --repo SillyRbbit/ai-agent-assistant --attempt 3 --log-failed",
    "gh run view 29624042656 --repo SillyRbbit/ai-agent-assistant --json status,conclusion,attempt,jobs",
    "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check",
    "cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings",
    "cargo test --manifest-path src-tauri/Cargo.toml --lib approvals::manager --locked",
    "rg -n \"ApprovalManagerInstanceMarker|into_source_parts|ApprovalPresentationParts|recognized_button|no_decision|source_failed|ApprovalInteractionSource\" src-tauri/src/approvals",
    "npx prettier --write HANDOFF.md docs/increments/repository-self-hosted-runner.md docs/reviews/2026-07-17-repository-self-hosted-runner-post-increment-review.md",
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
    "scripts/tests/test_repository_health.py",
    "src-tauri/src/approvals/manager.rs",
    "src-tauri/src/approvals/types.rs"
  ],
  "findings": [
    {
      "blocks_completion": true,
      "blocks_next_increment": true,
      "category": "Testing",
      "effort": "Small: publish the approved correction and rerun all required workflows",
      "milestone": "Current self-hosted runner verification before PR #24 completion",
      "risk": "PR #24 still points to the failing commit, so local target gating is not yet confirmed by Linux strict Clippy",
      "severity": "Medium",
      "summary": "Approved portability correction awaits remote Linux confirmation"
    },
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
      "check": "Documentation executes on runner 21 for commit 80bced4 and passes",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "CI executes on runner 21 for commit 80bced4 and passes",
      "required": true,
      "status": "Failed"
    },
    {
      "check": "Security executes on runner 21 for commit 80bced4 and passes",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Runner 21 is restarted with Rustup and Cargo visible to its managed service environment",
      "required": true,
      "status": "Passed"
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
    },
    {
      "command": "gh pr checks 24 --repo SillyRbbit/ai-agent-assistant --watch --interval 10",
      "required": true,
      "status": "Failed"
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
pass. The separately approved two-file extension target-gates only private
approval-source support whose sole consumer is macOS-only; no public approval
contract or target-Mac behavior changed.

Commit `80bced4` is pushed and PR #24 is open. Documentation and Security pass
on runner 21, proving exact-label routing and the repaired managed service. CI
passes the same host preflight and runs the complete repository command, but
strict Linux Clippy exposes five target-conditional warnings in approval code.
The exact private-only correction is implemented and fully verified locally,
but it is uncommitted and PR #24 still points to `80bced4`. The quality-gate
result remains `FAIL`; no completion marker may be written before remote CI
passes on the corrected commit.

## Scope and boundaries

The implementation stays within the declared three-workflow, repository-policy,
focused-test, operating-guide, plan, project-memory, and separately approved
two-file Rust portability scope. The Rust extension changes only conditional
compilation of private macOS-decision-source support. It adds no public API,
dependency, lockfile, Tauri command, IPC, CSP, capability, permission, SQLite
schema, identifier, secret, signing, deployment, or product behavior.

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
- Rust formatting passed after the approved portability correction.
- Strict all-target Clippy with all features and warnings denied passed after
  the correction without suppressing or weakening any lint.
- All six existing focused approval-manager tests passed.
- The sole-consumer trace confirmed that the target-gated manager marker,
  presentation parts/conversion, and evidence constructors feed only the
  existing macOS-gated source-resolution path.
- `npm run verify` passed formatting, policy, ESLint, strict Clippy, 28 hook
  tests, 19 repository tests, 124 frontend tests, 95 Rust library tests, 21 Rust
  integration tests, typecheck, both Vite builds, and the Tauri release
  no-bundle build.
- `git diff --check` passed.
- Runner 21 remained online, idle, version `2.335.1`, with `self-hosted`,
  `Linux`, `X64`, and `cortexa-ci` labels.
- Documentation and repository policy passed on runner 21 in 26 seconds for
  commit `80bced4`.
- Session-end inventory found no staged, untracked, or conflicted paths and
  exactly ten unstaged paths: the two approved Rust files and eight existing
  closeout documents.

Failed:

- CI run `29624042629` failed in four seconds at
  `Verify self-hosted runner prerequisites`; `git` and `python3` were found,
  then `command -v rustup` returned exit code 1.
- Security run `29624042656` failed in five seconds at the same prerequisite;
  `git` and `python3` were found, then `command -v rustup` returned exit code 1.
- After the reported service installation, CI attempt 2 job `88038644524` and
  Security attempt 2 job `88038655825` again reached runner 21 and failed in
  five seconds at the same `command -v rustup` check. The repair therefore has
  not reached the listener receiving jobs.
- Host repair installed Rustup `1.29.0`, set default toolchain
  `1.90.0-x86_64-unknown-linux-gnu`, and exposed `rustup` and `cargo` under
  `/home/henry-dang/.cargo/bin` in the interactive shell. `svc.sh stop` and
  `svc.sh start` then reported that the runner service unit was not loaded or
  installed. Runner 21 remained online through its earlier interactive listener,
  which still owns the pre-install PATH.
- Follow-up host diagnostics confirmed `.path` begins with
  `/home/henry-dang/.cargo/bin` and the managed service is active. Two listeners
  share the same registration: stale interactive PID `7699` and service PID
  `36245`. The service journal repeatedly reports that a session for the runner
  already exists. The stale listener received both reruns, explaining why the
  corrected service path was not observed.
- The stale listener was stopped and the managed service restarted as the sole
  listener. Attempt 3 passed runner preflight in both Rust-dependent workflows.
- Security attempt 3 passed secret scanning, npm audit, pinned Rustup setup,
  pinned `cargo-audit` installation, and the accepted Rust advisory baseline in
  3 minutes 39 seconds.
- CI attempt 3 passed preflight, checkout, Node setup, Rust setup, dependency
  installation, formatting, repository policy, and frontend lint before strict
  Linux Clippy found five existing target-conditional warnings.
- CI attempt 3 job `88039053953` failed after 3 minutes 9 seconds because Linux
  strict Clippy reported one unused `ApprovalInteractionSource` import, the
  unread private `ApprovalPresentation::manager_instance` field, unused
  `into_source_parts`, unconstructed `ApprovalPresentationParts`, and three
  unused private `ApprovalInteractionEvidence` constructors. All are consumed
  only through the macOS-gated decision source on the target Mac.
- The exact correction now exists locally in the two approved Rust files, but
  no later remote failure exists because the project owner has not yet approved
  committing or pushing it. PR #24 therefore still reports the attempt 3
  failure for `80bced4`.

No required local command remains failed. Two intermediate local checks stopped
only on Prettier wrapping in the new plan and passed after formatting. The first
post-correction documentation check similarly reported formatting in three
edited closeout files; formatting those exact files made the rerun pass.

Not run / manual pending:

- Required CI remains failed for the published branch pending separate approval
  to commit and push the implemented two-file correction. Documentation and
  Security pass on `80bced4` and must rerun for the corrected commit.
- Native application inspection is not required for this repository-only
  change and was not run.

## Architecture findings

No product architecture finding. The change affects only repository automation.
Product trust boundaries, module ownership, IPC, WebView, storage, gateway,
approval semantics, audit, and platform architecture remain unchanged. The
approved correction aligns private compilation scope with the existing
macOS-only decision-source ownership: its import, manager marker, presentation
parts and conversion, and evidence constructors now compile only where their
sole consumer exists. Target-Mac Clippy, focused tests, and the complete local
gate pass. Linux CI is explicitly not represented as target-Mac native evidence
and still must confirm the corrected branch.

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

The workflow implementation has no blocking correctness or maintainability
finding. The repository-health rule matches the exact workflow contract and
has one positive and two negative regressions. The checks are standard-library
only, deterministic, read-only, and emit no sensitive content. The two-file
correction is private, typed, and covered by the existing focused manager tests;
strict Clippy and complete target-Mac verification pass. CI completion remains
blocked only until that correction is published and passes remotely. No
application accessibility or UI path changed.

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

The runner workflow increment is not complete until the corrected final branch
content executes successfully in all three workflows. Increment 4V / PR #23
must remain untouched and unmerged until this workflow increment is verified
and merged. No later product or remediation increment is Ready to start.

## Completion decision

`FAIL`

Required CI failed on the currently published commit after runner-host
prerequisites passed. The locally corrected files cannot satisfy remote
verification until separately approved for commit and push. The report must not
be finalized and the active gate must not be marked complete.

## Next-increment readiness

`Blocked`

The exact next task is project-owner review of the ten-path uncommitted diff:
the approved Rust files plus the eight existing closeout documents. After
separate commit and push approval, rerun CI, Documentation, and Security on
runner 21. Only if all three pass may this report change to `PASS` or `PASS WITH
ADVISORIES` and the gate be finalized. Do not modify PR #23.

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
src-tauri/src/approvals/manager.rs
src-tauri/src/approvals/types.rs
```

## Exact commands executed

The machine manifest records the bounded setup, verification, runner API, Git
publication, remote inspection, focused Rust verification, complete local
verification, review, and gate commands. All required local commands passed.
The approved PR body is represented by its descriptive-body placeholder to keep
the machine manifest bounded; PR #24 preserves the exact submitted text.
