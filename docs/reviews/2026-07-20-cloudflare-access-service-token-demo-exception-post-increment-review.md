# Cloudflare Access service-token demo exception post-increment review

<!-- post-increment-gate-manifest
{"commands_executed":["npm run format","npm run docs:check","npm run repository:check","npm run security:scan","git diff --check","python3 .codex/hooks/session_end_gate.py"],"files_changed":["ARCHITECTURE.md","CHANGELOG.md","DECISIONS.md","HANDOFF.md","NEXT_STEPS.md","PLANS.md","PRODUCT_REQUIREMENTS.md","PROJECT_STATUS.md","SECURITY.md","docs/plans/cloudflare-access-service-token-demo-exception.md","docs/plans/cloudflare-synthetic-demo-gateway-decision.md","docs/plans/openai-synthetic-demo-gateway-readiness-plan.md","docs/plans/openai-synthetic-demo-gateway-security-implementation-plan.md","docs/plans/openai-synthetic-demo-owner-decision-record.md","docs/plans/openai-synthetic-demo-provider-decision.md","docs/plans/openai-synthetic-demo-runtime-implementation-plan.md","docs/plans/phase4-stage-b-no-traffic-provisioning.md","docs/reviews/2026-07-20-cloudflare-access-service-token-demo-exception-post-increment-review.md","docs/reviews/2026-07-20-cloudflare-synthetic-demo-gateway-decision-post-increment-review.md","docs/reviews/2026-07-20-openai-synthetic-demo-gateway-readiness-plan-post-increment-review.md","docs/reviews/2026-07-20-openai-synthetic-demo-gateway-security-implementation-plan-post-increment-review.md","docs/reviews/2026-07-20-openai-synthetic-demo-owner-decision-record-post-increment-review.md","docs/reviews/2026-07-20-openai-synthetic-demo-provider-decision-post-increment-review.md","docs/reviews/2026-07-20-openai-synthetic-demo-runtime-implementation-plan-post-increment-review.md","docs/reviews/2026-07-20-phase4-stage-b-provisioning-plan-post-increment-review.md"],"findings":[],"increment_id":"cloudflare-access-service-token-demo-exception","manual_verification":[],"next_increment_readiness":"Blocked","quality_gate":"PASS WITH ADVISORIES","schema_version":1,"verification":[{"command":"npm run format","required":true,"status":"Passed"},{"command":"npm run docs:check","required":true,"status":"Passed"},{"command":"npm run repository:check","required":true,"status":"Passed"},{"command":"npm run security:scan","required":true,"status":"Passed"},{"command":"git diff --check","required":true,"status":"Passed"},{"command":"python3 .codex/hooks/session_end_gate.py","required":true,"status":"Passed"}]}
-->

Date: 2026-07-20
Increment: cloudflare-access-service-token-demo-exception
Branch: main

## Executive summary

PASS WITH ADVISORIES. D-068 records one narrow 30-day-maximum Cloudflare
Access service-token exception for the owner-only fake-data demo. No token,
Keychain item, Access application, Worker, route, DNS change, secret,
deployment, provider request, or runtime behavior was added.

## Scope and boundaries

The decision binds one token to one application, macOS-Keychain-only secret
storage, trusted-Rust-only access, Worker JWT validation, and immediate
revocation. It explicitly preserves D-064's production 15-minute requirement.

## Verification results

All required documentation commands passed. No manual verification applies
because no credential or external artifact was created.

## Architecture findings

None. The exception is isolated to the internal demo and does not establish a
production identity or authentication architecture.

## Security findings

None. No secret or private identifier entered the repository. JWT signature,
issuer, and audience validation plus fail-closed rollback are mandatory future
controls.

## Code-health findings

None. No source or tests changed; formatting, links, repository policy, and
secret scanning pass.

## Technical debt

Advisory: no-traffic Access/Worker deployment, Keychain integration, rotation,
and manual security evidence remain intentionally unimplemented. This does not
block the decision; it blocks the next runtime increment.

## Roadmap findings

Blocked. No product increment is Ready. A separately approved no-traffic
deployment plan is required before creating any Cloudflare resource or token.

## Completion decision

PASS WITH ADVISORIES

## Next-increment readiness

Blocked. Obtain separate project-owner approval for one no-traffic Cloudflare
Access and Worker deployment plan.

## Exact files changed

See the machine manifest.

## Exact commands executed

See the machine manifest; each command exited successfully.
