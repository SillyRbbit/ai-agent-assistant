# Cloudflare demo macOS identity and secret-memory boundary plan post-increment review

<!-- post-increment-gate-manifest
{"commands_executed":["npm run format","npm run docs:check","npm run repository:check","npm run security:scan","git diff --check","python3 .codex/hooks/session_end_gate.py"],"files_changed":["ARCHITECTURE.md","CHANGELOG.md","DECISIONS.md","HANDOFF.md","NEXT_STEPS.md","PLANS.md","PROJECT_STATUS.md","SECURITY.md","docs/plans/cloudflare-demo-macos-identity-secret-boundary-plan.md","docs/reviews/2026-07-28-cloudflare-demo-macos-identity-secret-boundary-plan-post-increment-review.md"],"findings":[{"blocks_completion":false,"blocks_next_increment":true,"category":"Technical debt","effort":"separate approved decision and implementation increment","milestone":"before real Cloudflare Access demo-token creation or ingestion","risk":"the unsigned fake proof does not establish stable app-specific access or production secret-memory handling","severity":"Advisory","summary":"No macOS credential control is selected and real credential ingestion remains blocked."},{"blocks_completion":false,"blocks_next_increment":false,"category":"Technical debt","effort":"reassess maintenance, advisories, and alternatives before real credential handling","milestone":"before any real credential implementation","risk":"security-framework 3.7.0 declares looking-for-maintainer","severity":"Advisory","summary":"The fake-proof wrapper requires renewed review before real credential use."}],"increment_id":"cloudflare-demo-macos-identity-secret-boundary-plan","manual_verification":[],"next_increment_readiness":"Blocked","quality_gate":"PASS WITH ADVISORIES","schema_version":1,"verification":[{"command":"npm run format","required":true,"status":"Passed"},{"command":"npm run docs:check","required":true,"status":"Passed"},{"command":"npm run repository:check","required":true,"status":"Passed"},{"command":"npm run security:scan","required":true,"status":"Passed"},{"command":"git diff --check","required":true,"status":"Passed"},{"command":"python3 .codex/hooks/session_end_gate.py","required":true,"status":"Passed"}]}
-->

Date: 2026-07-28
Increment: cloudflare-demo-macos-identity-secret-boundary-plan
Branch: main

## Executive summary

PASS WITH ADVISORIES. This documentation-only increment defines the future
selection criteria for a stable signed macOS identity or narrow Keychain ACL,
and the secret-memory, lifecycle, and private-evidence controls required before
real credential work can be proposed. It selects neither control and creates no
runtime capability or external state.

## Scope and boundaries

The exact approved ten-file inventory is preserved. No code, dependency,
signing asset, entitlement, profile, Keychain action, credential, Cloudflare
resource, provider request, traffic, deployment, or runtime behavior changed.
D-064's 15-minute production requirement and D-068's 30-day demo exception are
unchanged.

## Verification results

All required documentation checks passed. No manual target-Mac, Keychain,
signing, Cloudflare, provider, or network action applies to this plan.

## Architecture findings

None. Current fake-only proof behavior remains separated from the future
credential boundary, and no current module ownership or runtime path changed.

## Security findings

No completion-blocking finding. D-071 rejects treating unsigned prompts as an
approved control and requires a later owner-approved selection, bounded secret
handling, lifecycle design, and private target-Mac evidence.

## Code-health findings

None. This is a documentation-only change with no source, dependency, test, or
configuration change.

## Technical debt

1. Category: credential-boundary readiness. Severity: Advisory. Risk: no stable
   macOS credential control is selected and the fake proof does not establish
   production secret-memory handling. Effort: separate approved decision and
   implementation increment. Milestone: before real token creation or
   ingestion. Blocks completion: no. Blocks next credential increment: yes.
2. Category: dependency health. Severity: Advisory. Risk:
   `security-framework 3.7.0` declares `looking-for-maintainer`. Effort:
   reassess maintenance, advisories, and alternatives. Milestone: before real
   credential handling. Blocks completion: no. Blocks next credential
   increment: no.

## Roadmap findings

Blocked. No product or operational increment is Ready. The smallest next action
is a separately approved decision or implementation increment that selects one
macOS control model and defines its exact test and private-evidence scope.

## Completion decision

PASS WITH ADVISORIES

## Next-increment readiness

Blocked. Do not begin signing, real credential ingestion, Worker implementation,
or Cloudflare provisioning without a separately approved exact increment.

## Exact files changed

- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `DECISIONS.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `SECURITY.md`
- `docs/plans/cloudflare-demo-macos-identity-secret-boundary-plan.md`
- `docs/reviews/2026-07-28-cloudflare-demo-macos-identity-secret-boundary-plan-post-increment-review.md`

## Exact commands executed

- `npm run format` — Passed
- `npm run docs:check` — Passed
- `npm run repository:check` — Passed
- `npm run security:scan` — Passed
- `git diff --check` — Passed
- `python3 .codex/hooks/session_end_gate.py` — Passed
