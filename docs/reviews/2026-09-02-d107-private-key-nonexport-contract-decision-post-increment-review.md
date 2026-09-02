# D-107 private-key non-export contract decision post-increment review

<!-- post-increment-gate-manifest
{"commands_executed":["npm run docs:check","npm run repository:check","npm run security:scan","git diff --check","python3 .codex/hooks/session_end_gate.py","python3 .codex/hooks/post_increment_gate.py status"],"files_changed":["docs/increments/d107-private-key-nonexport-contract-decision.md","docs/plans/2026-09-02-d107-private-key-nonexport-contract-decision.md","docs/reviews/2026-09-02-d107-private-key-nonexport-contract-decision-post-increment-review.md"],"findings":[],"increment_id":"d107-private-key-nonexport-contract-decision","manual_verification":[],"next_increment_readiness":"Blocked","quality_gate":"PASS WITH ADVISORIES","schema_version":1,"verification":[{"command":"npm run docs:check","required":true,"status":"Passed"},{"command":"npm run repository:check","required":true,"status":"Passed"},{"command":"npm run security:scan","required":true,"status":"Passed"},{"command":"git diff --check","required":true,"status":"Passed"},{"command":"python3 .codex/hooks/session_end_gate.py","required":true,"status":"Passed"},{"command":"python3 .codex/hooks/post_increment_gate.py status","required":true,"status":"Passed"}]}
-->

## Executive summary

Negative repository-only result: non-export is unproved. **PASS WITH ADVISORIES**.

## Scope and boundaries

Documentation only; no operational work.

## Verification results

Required checks passed.

## Architecture findings

No runtime edge.

## Security findings

Non-export remains unproved and blocks successors.

## Code-health findings

No finding.

## Technical debt

None introduced.

## Roadmap findings

**Blocked.**

## Completion decision

**PASS WITH ADVISORIES.**

## Next-increment readiness

**Blocked.**

## Exact files changed

Manifest is complete.

## Exact commands executed

Manifest records actual statuses.
