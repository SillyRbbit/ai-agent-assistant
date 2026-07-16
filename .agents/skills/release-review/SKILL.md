---
name: release-review
description: Review Cortexa release readiness against repository, security, artifact, signing, installer, rollback, and disclosure gates. Use before tagging, distributing, or approving a production release.
---

# Release review

1. Read `RELEASE_CHECKLIST.md`, `SECURITY_CHECKLIST.md`, `TESTING_GUIDE.md`, current status, accepted decisions, and release notes.
2. Confirm a clean reviewed commit and valid increment evidence.
3. Verify versioning, dependency and secret audits, tests, builds, artifacts, hashes, signing, notarization, installer, launch, upgrade, rollback, and disclosures.
4. Record each gate as passed, failed, not run, or manual verification pending with target-platform evidence.
5. Report blocking findings first and return `PASS`, `PASS WITH ADVISORIES`, or `FAIL`.

Do not tag, sign, notarize, upload, publish, or weaken a release gate unless the project owner separately directs that action.
