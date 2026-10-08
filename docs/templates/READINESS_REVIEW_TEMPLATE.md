# Increment readiness review

Date: YYYY-MM-DD
Candidate: increment-id and title
Reviewer: reviewer or workflow

## Decision

Use exactly `Ready`, `Ready with advisories`, or `Blocked`.

## Goal and user-visible outcome

State one bounded goal and the observable outcome.

## Prerequisite evidence

List the Git, source, test, decision, and completion-marker evidence that establishes the baseline.

## Expected paths and attribution

List every implementation, test, configuration, and closeout path expected to change.

## Risks and security impact

Identify affected trust boundaries, data, permissions, credentials, persistence, network access, execution authority, and target-platform risk.

## Explicit non-goals

List capability and cleanup work excluded from the increment.

## Verification and manual gates

List exact commands, required manual checks, environments, and completion evidence.

## Rollback

Describe the smallest pre-commit and post-commit rollback.

## Findings and advisories

Record each blocker or advisory with path or evidence, impact, owner, and smallest next action. Use `None` when no finding exists.

## Approval boundary

Record existing owner authorization, explicit remaining approval boundaries, and independence from unresolved failures. Readiness alone grants no new authority; an already-authorized milestone continues through supported admission and implementation.

## D-134 milestone operation

Follow AGENTS.md and ENGINEERING_GUIDE.md: objective, exclusions and acceptance
are the scope boundary. Path counts are informational; preserve protected paths,
attribution and unrelated work. Continue authorized routine work through review,
documentation and truthful finalization. Reuse valid unchanged-input verification
with provenance; distinguish implemented, automatically verified, live verified,
deferred and blocked criteria. Historical FAIL evidence and D-133's separate
acceptance route remain immutable. Request approval only at AGENTS.md boundaries.
