# Release Workflow

- **Category:** Workflow
- **Purpose:** Coordinate release evidence and approval boundaries from candidate selection through final review.
- **Use when:** A specific reviewed commit may become a development, pilot, candidate, or production release.
- **Do not use when:** Product implementation is incomplete or the request is only a release-readiness assessment.
- **Required inputs:** `{{RELEASE_CANDIDATE}}`, `{{RELEASE_CHANNEL}}`, exact commit, artifact plan, and rollback target.
- **Expected outputs:** A bounded release plan, complete evidence matrix, release-review result, and explicit publication approval boundary.
- **Related skills:** `$release-review`, `$security-review`.
- **Related prompts:** [Release review](../reviews/release-review.md), [End session](end-session.md), [Review template](../templates/review-template.md).
- **Last reviewed:** 2026-07-17

## Prompt

```text
Coordinate the {{RELEASE_CHANNEL}} release candidate {{RELEASE_CANDIDATE}}.

Read AGENTS.md, RELEASE_CHECKLIST.md, SECURITY_CHECKLIST.md, TESTING_GUIDE.md, accepted decisions, current status, and release notes. Confirm a clean reviewed commit and define exact artifacts, target platforms, version, hashes, signing/notarization expectations, installer checks, disclosures, and rollback.

Present the release plan and wait for project-owner approval before creating tags, artifacts, or release-side changes. Run all approved automated and manual gates and record Passed, Failed, Not run, and Manual verification pending accurately.

Use prompts/reviews/release-review.md for the final assessment. A passing review does not itself authorize tagging, signing, notarization, upload, publication, or deployment. Obtain separate approval for each requested publication operation and stop on any blocking gate.
```
