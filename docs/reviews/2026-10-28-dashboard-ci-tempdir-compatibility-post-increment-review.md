# Dashboard CI TMPDIR acceptance post-increment review

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "dashboard-ci-tempdir-compatibility",
  "quality_gate": "PASS WITH ADVISORIES",
  "next_increment_readiness": "Ready with advisories",
  "commands_executed": [
    "/usr/bin/python3 -B -m unittest discover -s .codex/hooks/tests -p test_lifecycle_dashboard_pinned_publication.py",
    "/usr/bin/python3 -B -m unittest discover -s .codex/hooks/tests -p test_*.py",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "python3 -B .codex/hooks/session_end_gate.py"
  ],
  "files_changed": [
    ".codex/hooks/tests/test_lifecycle_dashboard_pinned_publication.py",
    "CHANGELOG.md",
    "DECISIONS.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PROJECT_STATUS.md",
    "TROUBLESHOOTING_LOG.md",
    "docs/plans/2026-10-28-dashboard-current-main-integration.md",
    "docs/reviews/2026-10-28-dashboard-ci-tempdir-compatibility-post-increment-review.md",
    "docs/reviews/2026-10-28-dashboard-ci-tempdir-readiness-post-increment-review.md"
  ],
  "findings": [],
  "verification": [
    {
      "command": "/usr/bin/python3 -B -m unittest discover -s .codex/hooks/tests -p test_lifecycle_dashboard_pinned_publication.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "/usr/bin/python3 -B -m unittest discover -s .codex/hooks/tests -p test_*.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run docs:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run repository:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run security:scan",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "git diff --check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -B .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    }
  ],
  "manual_verification": [
    {
      "check": "Preservation and scope review",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Architecture, security, code-health and readiness review",
      "required": true,
      "status": "Passed"
    }
  ],
  "milestone": {
    "criteria": {
      "fixture": {
        "status": "automatically_verified",
        "evidence": "SCOPE-REVIEW.json proves reversal of only two setup expressions recovers original bytes;38 provided-TMPDIR cases passed"
      },
      "verification": {
        "status": "automatically_verified",
        "evidence": "Fresh38 pinned and447 missing-TMPDIR hooks plus final-checks-01 five checks; actual primary0/wait0 and complete cleanup"
      },
      "preservation": {
        "status": "automatically_verified",
        "evidence": "Complete baseline delta, old immutable reports, original source/index/state, owned custody and full monitored allocation checked; historical continuity remains Unverified"
      },
      "review": {
        "status": "automatically_verified",
        "evidence": "Substantive architecture/security/code-health/preservation review recorded in SCOPE-REVIEW.json; final normal gate validation follows"
      }
    },
    "paths": {
      "TROUBLESHOOTING_LOG.md": {
        "criterion": "review",
        "rationale": "Additive current CI acceptance/checkpoint evidence; historical body retained",
        "within_objective": true,
        "preserves_existing": true
      },
      "docs/plans/2026-10-28-dashboard-current-main-integration.md": {
        "criterion": "review",
        "rationale": "Additive current CI acceptance/checkpoint evidence; historical body retained",
        "within_objective": true,
        "preserves_existing": true
      },
      "docs/reviews/2026-10-28-dashboard-ci-tempdir-compatibility-post-increment-review.md": {
        "criterion": "review",
        "rationale": "Additive current CI acceptance/checkpoint evidence; historical body retained",
        "within_objective": true,
        "preserves_existing": true
      },
      "NEXT_STEPS.md": {
        "criterion": "review",
        "rationale": "Additive current CI acceptance/checkpoint evidence; historical body retained",
        "within_objective": true,
        "preserves_existing": true
      },
      ".codex/hooks/tests/test_lifecycle_dashboard_pinned_publication.py": {
        "criterion": "fixture",
        "rationale": "Exact two disposable fixture expressions",
        "within_objective": true,
        "preserves_existing": true
      },
      "CHANGELOG.md": {
        "criterion": "review",
        "rationale": "Additive current CI acceptance/checkpoint evidence; historical body retained",
        "within_objective": true,
        "preserves_existing": true
      },
      "PROJECT_STATUS.md": {
        "criterion": "review",
        "rationale": "Additive current CI acceptance/checkpoint evidence; historical body retained",
        "within_objective": true,
        "preserves_existing": true
      },
      "HANDOFF.md": {
        "criterion": "review",
        "rationale": "Additive current CI acceptance/checkpoint evidence; historical body retained",
        "within_objective": true,
        "preserves_existing": true
      }
    }
  }
}
-->

Date: 2026-10-28
Increment: dashboard-ci-tempdir-compatibility
Branch: codex/dashboard-main-integration

## Executive summary

Bounded CI fixture compatibility maintenance. Required fresh evidence passed; local automated acceptance retains advisories.

## Scope and boundaries

Only optional TMPDIR selection and canonical root resolution in the disposable pinned-publication self-test, plus additive task documentation. No production, validator, workflow, dependency or assertion changes. Preserve predecessor reports/state at commit5fcd348 and private PREDECESSOR.raw.json.

## Verification results

Results bind exact source/environment/argv and separate primary, actual wait and cleanup in private ci-tempdir-01 evidence. Pending results are not promoted. Earlier npm run verify remains bound to input tree bd43527345dae14617c5c5827b0426c7a8d0ae67 and integration commit5fcd348. Application/package/lock/tool inputs are unchanged; its application-build evidence is reused only for those inputs, never claimed as a fresh full run. Failed GitHub run40782694245 stays failed.

## Architecture findings

Only disposable fixture construction changes. Deterministic runtime, dashboard/voice routing, typed IPC and lifecycle ownership remain unchanged. No new abstraction or helper.

## Security findings

Both local environment variants select monitored build/tmp; absent TMPDIR sets TEMP. CI uses supported tempfile selection. Assertions, no-follow readers, history hashes, production validation and resource/cleanup guards remain intact. No credentials, network expansion or policy change.

## Code-health findings

Two setup expressions resolve concrete Linux KeyError and canonicalize temporary-root aliases. No production fallback, skip or assertion waiver. Existing pinned and complete hook suites supply meaningful coverage.

## Technical debt

Advisory: native application/accessibility/live-provider QA remains outside scope and unverified. Step B Blocked, exec Deferred, oldest cleanup and historical volume continuity Unverified remain unchanged; this does not block bounded CI correction or establish live readiness.

## Roadmap findings

Finish only the approved PR #158 correction and conditional squash merge. No later increment selected. Archive/supervisor/census mechanisms stay parked.

## Completion decision

PASS WITH ADVISORIES

## Next-increment readiness

Ready with advisories for the already-authorized conditional PR finish after CI/rules pass; no later milestone admitted.

## Exact files changed

- `.codex/hooks/tests/test_lifecycle_dashboard_pinned_publication.py`
- `CHANGELOG.md`
- `DECISIONS.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PROJECT_STATUS.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-10-28-dashboard-current-main-integration.md`
- `docs/reviews/2026-10-28-dashboard-ci-tempdir-compatibility-post-increment-review.md`
- `docs/reviews/2026-10-28-dashboard-ci-tempdir-readiness-post-increment-review.md`

## Exact commands executed

- `/usr/bin/python3 -B -m unittest discover -s .codex/hooks/tests -p test_lifecycle_dashboard_pinned_publication.py`: Passed.
- `/usr/bin/python3 -B -m unittest discover -s .codex/hooks/tests -p test_*.py`: Passed.
- `npm run docs:check`: Passed.
- `npm run repository:check`: Passed.
- `npm run security:scan`: Passed.
- `git diff --check`: Passed.
- `python3 -B .codex/hooks/session_end_gate.py`: Passed.
