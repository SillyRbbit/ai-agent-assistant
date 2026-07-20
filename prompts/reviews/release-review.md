# Release Review

- **Category:** Review
- **Purpose:** Assess a release candidate against repository, security, artifact, and target-platform gates.
- **Use when:** A specific reviewed commit and artifact set are proposed for release.
- **Do not use when:** The request is to implement a feature, create an unsigned development build, or publish automatically.
- **Required inputs:** `{{RELEASE_CANDIDATE}}`, exact commit, artifacts, release notes, and verification evidence.
- **Expected outputs:** Release blockers first and exactly `PASS`, `PASS WITH ADVISORIES`, or `FAIL`.
- **Related skills:** `$release-review`, `$security-review`.
- **Related prompts:** [Release workflow](../workflows/release.md), [Security review](security-review.md), [Executive review](executive-review.md).
- **Last reviewed:** 2026-07-17

## Prompt

```text
Follow the root AGENTS.md and docs/governance/MASTER_PROMPT.md. Apply the current task-specific instructions below without violating the approved increment.

Use $release-review for {{RELEASE_CANDIDATE}}.

Review RELEASE_CHECKLIST.md, SECURITY_CHECKLIST.md, TESTING_GUIDE.md, accepted decisions, current status, release notes, exact commit, and exact artifacts. Verify clean state, versioning, dependency and secret audits, tests, builds, hashes, signing, notarization, installer behavior, launch, upgrade, rollback, and disclosures.

Record each gate as Passed, Failed, Not run, or Manual verification pending, report blockers first, and return PASS, PASS WITH ADVISORIES, or FAIL. Do not tag, sign, upload, publish, or weaken a gate.
```
