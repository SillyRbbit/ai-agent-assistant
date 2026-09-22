# Direct provider stream stage live-result evidence closeout

Date: 2026-09-21. Increment:
`direct-provider-stream-stage-live-result-evidence-closeout`. Status: Complete.

## Goal

Reconcile the final owner-supplied native screenshot with the completed
stream-stage diagnostics evidence while preserving every predecessor record and
all product, test, security-policy and governance behavior.

## User-visible outcome

Repository current-state documents accurately record that the final authorized
fixed-sample request was consumed and ended in the closed
`provider_stream_error_event` state, without overstating what Computer Use or the
screenshot proved.

## Scope

The successor changes exactly ten repository paths: CHANGELOG.md, HANDOFF.md,
NEXT_STEPS.md, PLANS.md, PROJECT_STATUS.md, ROADMAP.md, TESTING_GUIDE.md,
TROUBLESHOOTING_LOG.md, this plan and its review. The cumulative Git inventory is
exactly 19 paths: the completed 17-path candidate plus these two new files.

## Explicit non-goals

Do not change runtime, tests, SECURITY.md, dependencies, configuration,
workflows, hooks, skills, harnesses or predecessor plans/reports; rerun application
tests or builds; launch Cortexa; inspect credentials, Terminal contents, process
arguments/environments or raw provider data; make a provider request; infer
upstream cause; claim absent transient output, ownership release or native live
success; reopen predecessor records; commit; push; publish; or resume D-125/M1/M2.

## Existing behavior and constraints

The source closeout at `ebaae34ea32e4930e60b53c6064a8bf25036d52a` is valid
`complete / PASS WITH ADVISORIES` evidence over 17 changed paths. Its terminal
source remains valid `failed / FAIL / Blocked` evidence at
`3f99165b4dcb0ef18c52f9242b346eb1d711afaa`, with no completion marker. Both
share tree `ebf8f96ca6160d738d651344f06d0c6636cffc54` with the baseline.

The existing closed client mapping associates the displayed screenshot message
with `provider_stream_error_event`. Regression tests establish intended no-retry
and control-release behavior, but tests do not prove the live request's ownership
release. The screenshot contains no completed answer and cannot establish whether
transient text appeared before capture.

## Current-state evidence

- Completed closeout report SHA-256:
  `69f2eade0e377bcc9611e513eadd56dc69abf02bb7a82623162ae667255ac770`.
- Completed closeout raw state SHA-256:
  `1982f29f301b0de50feff2595583f87c2ee9b0f32c75aad00329d3a5872ad45d`.
- Terminal predecessor report SHA-256:
  `60893ebff4208c87eb489669ad41b813374bc1b828b05f6cff8809fc8638be7c`.
- Terminal predecessor raw state SHA-256:
  `073b8e13d2624b52ead94cf65a1fe93376bba5e43730a0946251748cb9fde068`.
- Owner-supplied screenshot SHA-256:
  `0e281c31319035dc8422372fc0c31df24f196f6d75bccf02653e4fa24cef2d21`.
- Bundle executable SHA-256:
  `f2cf3f5c998b072ffb47dce81c3047c0473307484b0e68006e21c27a3bb58e7d`.

Direct Computer Use observed the idle, enabled, disclosed pre-request state and
the checked acknowledgement. Its Start action then lost the active binding before
a post-click state returned. The owner-supplied screenshot separately shows
terminal `error`, the closed top-level stream-event message, no completed answer
and the static no-automatic-retry statement. The final authorized request was
consumed, leaving zero attempts.

## Files expected to change

- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `ROADMAP.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-09-21-direct-provider-stream-stage-live-result-evidence-closeout.md`
- `docs/reviews/2026-09-21-direct-provider-stream-stage-live-result-evidence-closeout-post-increment-review.md`

## Affected components

Only repository current-state documentation, plan/review evidence and external
preservation/report-schema validation. Runtime, Tauri, frontend, Rust, transport,
provider, dependency and permission behavior are unaffected.

## Interfaces and invariants

The 17 inherited candidate paths remain exact except for the eight approved
current-state documents. Those eight documents preserve their prior bodies as
exact historical suffixes. SECURITY.md, four executable/test files and four
predecessor plan/review files remain byte-identical. Every pre-existing worktree,
raw gate state, completion marker, external evidence directory, bundle identity
and both prunable registry entries remain unchanged. The manifest lists exactly
19 cumulative paths.

## Implementation milestones

- [x] Verify heads, trees, gate statuses, report/state hashes, screenshot and external evidence.
- [x] Create an isolated detached worktree and transfer the exact 17-path candidate.
- [x] Freeze preservation evidence and clone ignored formatter tooling with APFS copy-on-write.
- [x] Receive ordinary admission before tracked edits.
- [x] Add the exact ten-document evidence reconciliation.
- [x] Pass documentation-tier, preservation, review, schema, finalization and Stop gates.

## Security and privacy considerations

The owner-supplied screenshot contains only the fixed synthetic disclosure and
closed sanitized result. No credential, process environment, provider payload,
raw error, request ID, header or body is read or recorded. No network request or
native process is created. External evidence stores hashes and repository-local
metadata only.

## Test plan

Run targeted formatting; offline documentation, repository and security checks;
whitespace; exact scope, historical-suffix, screenshot/report/state/external-
evidence hash, registry and protected-byte checks; session and independent
architecture/security/code-health/debt/readiness review; exact 12-section report
schema; ordinary finalization; complete/valid status; and full-payload Stop.
Application tests and builds remain inherited historical evidence and are not rerun.

## Verification commands

```bash
./node_modules/.bin/prettier --write CHANGELOG.md HANDOFF.md NEXT_STEPS.md PLANS.md PROJECT_STATUS.md ROADMAP.md TESTING_GUIDE.md TROUBLESHOOTING_LOG.md docs/plans/2026-09-21-direct-provider-stream-stage-live-result-evidence-closeout.md docs/reviews/2026-09-21-direct-provider-stream-stage-live-result-evidence-closeout-post-increment-review.md
env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run docs:check
env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run repository:check
env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run security:scan
git diff --check
python3 -B /private/tmp/cortexa-direct-provider-stream-stage-live-result-evidence/preserve.py check
python3 -B .codex/hooks/session_end_gate.py
python3 -B /private/tmp/cortexa-direct-provider-stream-stage-live-result-evidence/validate_report.py
```

## Risks

Any drift in screenshot, source records, worktree registry, external evidence,
historical suffixes or protected bytes invalidates the closeout. Documentation
can reconcile observed state but cannot diagnose the upstream provider event or
prove native lifecycle details not captured by Computer Use.

## Rollback or failure strategy

Stop and preserve the isolated worktree on any failure. Do not repair outside the
exact scope, alter predecessor evidence, make a provider request or begin an
automatic successor. Freeze a truthful terminal failed report if ordinary
completion cannot pass.

## Decisions made

Record the screenshot only as owner-supplied evidence and retain direct Computer
Use observations as a separate evidence class. No durable architecture, security,
provider or product decision changes.

## Discoveries

The existing current-state headers incorrectly say the additional request remains
unused. The screenshot resolves request accounting but not the provider cause,
transient streaming history or ownership release.

## Progress

2026-09-21: All predecessor and external evidence passed drift checks. The exact
17-path candidate transferred to a new detached worktree, local formatter tooling
was clone-copied without installation, ordinary admission passed, and the bounded
documentation delta was drafted.

## Acceptance criteria

- [x] Evidence classes and their limits are stated separately and accurately.
- [x] Zero remaining authorized requests is recorded without a live-success claim.
- [x] Scope is exactly ten documents and 19 cumulative changed paths.
- [x] Protected bytes and historical bodies remain unchanged.
- [x] Documentation-tier and completion gates pass.

## Final results

Complete. The exact ten-document successor delta and 19-path cumulative inventory
passed formatting, offline documentation/repository/security, whitespace,
preservation, session, independent review and report-schema validation. Ordinary
finalization produced valid `complete / PASS WITH ADVISORIES` status and the
full-payload Stop hook passed. Application tests and builds were not rerun; no
native launch, credential inspection or provider request occurred.

## Documentation updates

- [x] `HANDOFF.md`
- [x] `PROJECT_STATUS.md`
- [x] `NEXT_STEPS.md`
- [x] `CHANGELOG.md`
- [x] `PLANS.md`
- [x] `ROADMAP.md`
- [x] `TESTING_GUIDE.md`
- [x] `TROUBLESHOOTING_LOG.md`
