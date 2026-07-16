# Security review

Date: YYYY-MM-DD
Change: increment or component
Reviewer: reviewer or workflow

## Result

Use exactly `PASS`, `PASS WITH ADVISORIES`, or `FAIL`.

## Scope and threat model

Identify trusted and untrusted actors, assets, entry points, data, authority, and affected boundaries. Distinguish current, mocked, planned, and prohibited behavior.

## Data flow and privileges

Describe inputs, validation, outputs, storage, logs, credentials, network access, filesystem or operating-system access, capabilities, permissions, and failure handling.

## Checklist evidence

Record applicable `SECURITY_CHECKLIST.md` sections and give a reason for each section marked not applicable.

## Findings

For each finding record severity, exact path and line, impact, evidence, smallest mitigation, and whether it blocks completion or the next increment. Use `None` when no finding exists.

## Adversarial and regression tests

List malformed, unknown, traversal, symlink, injection, replay, expiry, cancellation, late-event, redaction, conflict, and permission cases that apply, with actual results.

## Verification

List exact commands and manual checks as `Passed`, `Failed`, `Not run`, or `Manual verification pending`.

## Residual risk and follow-up

State accepted residual risk, owner, milestone, and required follow-up without treating the review as authorization or execution evidence.
