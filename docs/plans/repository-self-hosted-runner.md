# Repository Workflow Increment - trusted self-hosted runner routing

Status: Verified complete with advisories; squash-merged through PR #24 at
`eaf6c9f`
Owner: Project maintainer
Last updated: 2026-07-17

## Goal

Route the repository's existing read-only CI, Documentation, and Security jobs
to the registered Linux x64 GitHub Actions runner without executing
pull-request workflow definitions on the persistent machine or weakening
checks.

## User-visible outcome

Trusted repository changes can run the existing verification workflows when
GitHub-hosted jobs are unavailable. Product behavior is unchanged.

## Scope

- Add the repository-specific `cortexa-ci` label to the registered runner.
- Select the exact four-label runner identity in the three existing workflows.
- Remove `pull_request` triggers and limit pushes to documented
  maintainer-controlled branch families.
- Add fail-fast Linux, Rust, Python, and Tauri prerequisite checks.
- Enforce the selector, no-pull-request rule, and push allowlist in
  repository-health tests.
- Document host setup, security boundaries, maintenance, verification, and
  rollback.
- Target-gate only private approval-source imports, presentation state/parts,
  conversion methods, and evidence constructors whose sole consumer is the
  macOS decision source after Linux strict Clippy exposed their non-macOS dead
  code.

## Explicit non-goals

- No public approval contract, target-Mac behavior, dependency, manifest,
  lockfile, IPC, CSP, capability, permission, database, identifier, signing,
  notarization, deployment, or publishing change.
- No repository secret, production credential, artifact publication, automatic
  commit, push, merge, or deployment.
- No claim that Linux checks prove target-Mac native behavior.
- No ephemeral-runner orchestration or organization runner-group change.
- No modification, rebase, merge, or publication of Increment 4V / PR #23.

## Existing behavior and constraints

The registered runner is online, idle, Linux, x64, and initially exposed only
GitHub's default `self-hosted`, `Linux`, and `X64` labels. Existing workflows
select `macos-15` or `ubuntu-24.04`, so none can match it. PR #23's jobs failed
before source checkout because GitHub-hosted execution was blocked by an account
billing or spending-limit restriction.

The repository treats pull-request code and dependency proposals as untrusted.
The persistent runner is not an ephemeral security boundary and must remain
read-only, secret-free, dedicated, and unavailable to untrusted PR sources.

## Files expected to change

Workflow and policy implementation:

- `.github/workflows/ci.yml`
- `.github/workflows/documentation.yml`
- `.github/workflows/security.yml`
- `scripts/repository_health.py`
- `scripts/tests/test_repository_health.py`
- `docs/github/SELF_HOSTED_RUNNER.md`
- `SECURITY.md`
- `SECURITY_CHECKLIST.md`
- `TESTING_GUIDE.md`
- `DECISIONS.md`
- `TROUBLESHOOTING_LOG.md`
- `src-tauri/src/approvals/manager.rs`
- `src-tauri/src/approvals/types.rs`

Declared closeout scope:

- `AGENTS.md`
- `HANDOFF.md`
- `PROJECT_STATUS.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `CHANGELOG.md`
- `docs/plans/README.md`
- `docs/plans/repository-self-hosted-runner.md`
- `docs/increments/repository-self-hosted-runner.md`
- `docs/reviews/2026-07-17-repository-self-hosted-runner-post-increment-review.md`

## Implementation steps

- [x] Confirm clean synchronized `main`, create an isolated workflow branch,
      and inspect the registered runner and existing workflow selectors.
- [x] Begin mandatory `repository-self-hosted-runner` gate state.
- [x] Assign the custom label and add exact routing, trusted trigger limits, and
      prerequisite checks.
- [x] Add focused positive and negative repository-policy tests.
- [x] Document the persistent-runner boundary and operating procedure.
- [x] Run complete local verification and review the exact diff.
- [x] With explicit publication approval, commit and push the branch and open
      PR #24.
- [x] Repair the runner service, remove its duplicate interactive listener, and
      confirm Documentation and Security pass.
- [x] Obtain separate approval for the exact two-file Linux strict-Clippy
      portability correction and implement it without weakening checks.
- [x] Pass focused approval-manager tests, strict Clippy, and complete local
      target-Mac verification after the correction.
- [x] Commit and push the approved correction, then pass CI, Documentation, and
      Security on runner 21.
- [x] Create the consolidated report with the evidence-backed interim `FAIL`
      result.
- [x] Update the report and finalize the completion marker only after remote
      evidence passes.

## Security and privacy considerations

The runner host can be persistently compromised by code it executes. It must use
a dedicated unprivileged account, contain no production credentials or personal
data, and have no access to unrelated trusted services. Removing
`pull_request` prevents fork-controlled workflow definitions from running, but
the policy cannot protect the host from a trusted writer who can modify and push
workflow code. No secrets enter any job.

## Test plan

- Positive fixture accepts the exact custom-label selector, no-pull-request
  policy, and push allowlist.
- Negative fixtures reject a generic self-hosted selector and any
  pull-request-triggered exact selector.
- Existing immutable-action, read-only permission, no-secret, no-write, link,
  generated-output, and command checks remain green.
- Complete repository verification passes locally.
- Existing focused approval-manager tests preserve the target-Mac presentation
  and resolution behavior.
- Published CI, Documentation, and Security jobs execute on the registered
  runner and pass.

## Verification commands

```bash
npm run test:repository
npm run docs:check
npm run repository:check
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --lib approvals::manager --locked
npm run verify
git diff --check
gh api repos/SillyRbbit/ai-agent-assistant/actions/runners
gh pr checks <runner-setup-pr-number> --watch
python3 .codex/hooks/session_end_gate.py
python3 .codex/hooks/post_increment_gate.py status
```

## Rollback or failure strategy

Restore the prior hosted `runs-on` values and remove the trust/preflight blocks.
Remove `cortexa-ci` from runner 21 through the GitHub runner-label API. Keep the
existing read-only/no-secret workflow policy. Rerun all local checks and hosted
workflow checks before declaring rollback complete.

## Acceptance criteria

- [x] Runner 21 is online and has all four expected labels.
- [x] All three workflows preserve read-only permissions, immutable actions,
      non-persistent checkout credentials, and no secret context.
- [x] Repository tests reject missing custom labels, pull-request triggers, and
      incomplete push allowlists.
- [x] Full local verification and complete diff review pass.
- [x] The approved two-file correction target-gates only private macOS-source
      support and passes focused plus complete local verification.
- [x] CI, Documentation, and Security execute successfully on the registered
      runner after explicit publication approval.
- [x] The final gate report and marker are complete and valid.

## Actual results

Baseline repository-health tests, documentation checks, and complete repository
health passed before edits. Focused post-edit repository-health tests pass 19 of 19. Workflow YAML parsing, documentation, repository health, secret scanning,
diff checks, and complete `npm run verify` pass locally. Two intermediate
formatting-only failures in this plan were corrected with Prettier before the
final complete pass. Commit `80bced4` is pushed on PR #24. Documentation passed
on runner 21; CI run `29624042629` and Security run `29624042656` both failed
at `command -v rustup`. The consolidated report remains `FAIL`, and the gate
remains active. After host repair, attempt 3 proves Documentation and Security
pass. CI reaches strict Clippy and fails on five private approval-code items
whose only consumer is the macOS-gated decision source. The original file plan
was expanded by separate project-owner approval. The exact two-file correction
is implemented locally; rustfmt, strict Clippy, six focused manager tests, and
complete `npm run verify` pass. At that checkpoint PR #24 still pointed to
`80bced4`, so remote CI confirmation remained pending separate commit and push
approval. The first post-correction documentation check reported only
formatting in three closeout files; formatting those exact files made the rerun
pass.

The correction and reviewed interim closeout were committed as `1621a55` and
pushed to PR #24. Security run `29629669283` passed in 3 minutes 22 seconds,
Documentation run `29629669305` passed in 16 seconds, and CI run `29629669300`
passed complete Linux verification in 9 minutes 57 seconds. The final result is
`PASS WITH ADVISORIES`; the remaining persistent-runner isolation advisory is
non-blocking, and the completion marker is valid.

The final twelve-path documentation closeout was committed as `cfa976f`.
Documentation run `29630372279`, CI run `29630372265`, and Security run
`29630372253` passed on that commit. PR #24 was squash-merged as `eaf6c9f`, its
remote branch was deleted, synchronized `main` was clean, and the marker
remained valid before the next gate began.

## Documentation updates

- [x] `HANDOFF.md`
- [x] `PROJECT_STATUS.md`
- [x] `NEXT_STEPS.md`
- [x] `DECISIONS.md`
- [x] `CHANGELOG.md`
- [x] `TROUBLESHOOTING_LOG.md`
