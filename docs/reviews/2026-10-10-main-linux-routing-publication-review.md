# Selective main Linux routing publication review

Date: 2026-10-10
Status: Fresh local extraction checks passed; publication requires final bound review

## Scope and provenance

This is a focused thirteen-path publication export from completed local
`main-linux-routing-local-remediation`, under the
[owner-authorized plan](../plans/2026-10-10-main-linux-routing-publication.md).
Implementation-agent preparation is not independent clearance. A separate reviewer
will assess the final extracted diff and evidence applicability before commit.
The source's full mixed candidate, D-136 mechanisms and D-137 lifecycle are excluded.

## Bound evidence

Base main: `de0d6bcbb63e5377825da4edd6d31bfd98d03dfb`.
Source final report SHA-256:
`946dc68ffb50ba8026b9e7df18cc1f843c94ce764820a23f0bbce781f32484d7`.
Final-001 archive SHA-256:
`1324abe4eaf45b44ab99e6ea6ed48e819ebb2ddfdcb69645ceb161f6c846365c`.
The durable publication package is
`/Users/hdang/.codex/backups/cortexa-routing-publication-20261010-01`.
Its receipts bind frozen implementation hashes, preserved source/index/gate,
installed dependencies/interpreters, final export inputs and fresh results.
The earlier all-member archive readback is retained and reused.

Only unchanged relevant product/Rust/frontend/dependency inputs reuse the passing
source full verification: 601 frontend cases, 714 unique passing native tests
across 1,173 executions, one unique ignored case, strict Rust/frontend checks and
native release build. Repeated library executions add no unique coverage.
No fresh full verify is claimed. Local macOS and manual VPS evidence do not count
as actual GitHub Actions acceptance.

## Fresh extraction validation

All seven required fresh checks passed on the extracted candidate:

- Hook tests: 145 tests, passing.
- Repository tests: 98 tests, passing; the repository-health module contains
  61 total tests, including eight named workflow-policy tests.
- Documentation, repository, security, whitespace and session checks: exit zero.

Receipts `check-*-001.json` bind these results to `export-input-manifest-001.json`.
Only the plan and this report changed afterward to record observed results and
correct a reviewer-identified coverage wording overstatement. The executable
inputs remain unchanged. Final documentation/repository/security/whitespace/session
checks must bind those documentation changes in `check-*-002.json` and
`export-input-manifest-002.json` before publication. The first passing hook and
repository results remain applicable without repeating their unchanged inputs.
Independent final clearance is a separate input-bound sidecar in the durable
publication package; this implementation-agent report does not substitute for it.
No pending requirement is treated as passing.

## Retained history and limitations

Original routing FAIL, compiler failures, historical compatibility failures,
unknown outcomes and all D-136 preparation remain preserved in the source packages.
D-136 is unconsumed; PR #139 remains untouched. Completed source gate state is not
copied into E or reopened.
The two mmap failures and managed-tool internal cleanup attempts remain historical;
no agent cleanup command was issued. Materialization's strict comparison failure,
Finder metadata change and unproven historical `info/packs` bytes are retained.
Later checkout success does not establish the original mmap cause.

## Publication acceptance

Commit/push/PR/merge remain conditional on final checks and independent review.
Actual main CI and Documentation must meet the exact SHA/job/runner requirements
in the plan within the one-hour observation limit. No Actions success is claimed
by this preparation report. The durable publication receipts retain subsequent
Git/Actions outcomes without rewriting source history. Deployment and the deferred
Word/PDF architecture/runbook deliverables are outside this operation.
