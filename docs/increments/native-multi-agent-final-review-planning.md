# Native multi-agent final-review planning

Status: Verified complete with advisories
Owner: Project owner
Last updated: 2026-08-28

## Goal

Make the blocked native multi-agent final-review draft exact and Ready with
advisories, without performing that review or changing production behavior.

## Scope

- Convert the existing final-review draft into a read-only ExecPlan.
- Select completed evidence, review matrix, severity rules, manual evidence,
  disclosure, stop conditions, verification, rollback, and approval boundary.
- Synchronize live project memory with the planning result.

## Explicit non-goals

No source review execution, remediation, runtime/UI/IPC change, dependency,
provider, model, network, credential, tool, approval, persistence, filesystem,
background work, capability, CSP, permission, device effect, release, or
publication action.

## Current-state evidence

The 2026-08-26 native-nine-agent review is historical evidence. Its F-01/F-02,
F-07, F-08, F-12, and F-15 findings have later bounded completion evidence.
The sealed Research -> Knowledge lifecycle presentation remains a separate
deterministic proof, not general agent authority.

## Files expected to change

- `docs/plans/2026-08-11-native-multi-agent-security-architecture-review.md`
- `docs/increments/native-multi-agent-final-review-planning.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `ROADMAP.md`
- `docs/roadmap/NATIVE_MULTI_AGENT_ROADMAP.md`
- `CHANGELOG.md`
- `docs/reviews/2026-08-28-native-multi-agent-final-review-planning-post-increment-review.md`

## Acceptance criteria

- [x] The final review has an exact read-only scope, matrix, stop conditions,
      evidence rules, verification set, and rollback.
- [x] Final-review execution remains separately owner-approved.
- [x] Current records distinguish the sealed lifecycle exception from generic
      agent UI or external authority.
- [x] No product or historical-evidence path changes.

## Final results

`PASS WITH ADVISORIES`. Documentation formatting/link, repository-health,
security scan, protected-path, session-end, and diff checks pass. The successor
final review is Ready with advisories but may not begin until separately
approved by the owner. The advisory is the explicit owner-approval boundary;
this planning result grants no review or remediation authority.
