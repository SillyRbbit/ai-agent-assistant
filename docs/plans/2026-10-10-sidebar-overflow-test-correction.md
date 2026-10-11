# Sidebar overflow test correction

Date: 2026-10-10
Task: `sidebar-overflow-test-correction`
Authority: explicit owner-authorized, single-task bootstrap without ordinary D-134 admission.
Status: Implementation and local checks passed; final evidence sealing follows.

## Objective and dependency

Replace the twenty full App workflows used to prepare one structural sidebar test
with twenty typed conversation fixtures rendered by the existing
`ApplicationSidebar`. Preserve the twenty-button assertion, conversation-list
scroll attribute, containment of both lists within the sidebar, existing App
integration tests and the five-second default. This task depends on the retained
GitHub frontend failure in job `114184244341`, run `38042081928`: 600 passed,
one failed. Local correction evidence will not rewrite that result or establish
GitHub Actions acceptance. JSDOM checks structure, not rendered overflow geometry.

Ordinary D-134 rejects dependent successors. The owner explicitly authorizes this
bootstrap and excludes begin, finalize, closure/disposition and gate-based Stop.
No validator, existing gate or completion marker changes are permitted.

## Exact scope and preservation

Repository edits are limited to:

- `src/App.test.tsx`;
- this new plan;
- `docs/reviews/2026-10-10-sidebar-overflow-test-correction-review.md`.

Evidence package:
`/Users/hdang/.codex/backups/cortexa-sidebar-overflow-test-correction-20261010-01`.
It retains owner authority, budget history, original test bytes, baseline and final
input hashes, command logs/receipts, review, handoff and final seal.

Baseline HEAD is `e6ac8866e331dab003d760c3877e53b480185385` on
`codex/ci/main-linux-routing-publication`. The clean candidate matches all 1,090
retained publication inputs; the 112-member publication archive and seal verify.
The existing publication package, its budget/deadlines, source gate and D-136
preparation/request remain byte-preserved. This checkout has no live gate.

No production, workflow, validator, infrastructure, dependency installation,
timeout-policy, CI, Git publication, cleanup or broader adoption change is allowed.
D-136 stays sealed and unconsumed; PR #139 is untouched. Architecture overview and
runbook Word/PDF remain deferred until deployment and applicable validation finish.

## Implementation and acceptance

- [x] Reconcile preserved candidate, archive and authority.
- [x] Replace only the selected test setup, using `createConversationSession` and
      `ApplicationSidebar`; remove its unnecessary fake timers and workflow loop.
- [x] Pass the focused test, entire App file and complete frontend suite.
- [x] Pass formatting, lint, strict type checking and applicable documentation,
      repository, security, whitespace and nonmutating session checks.
- [x] Review the exact diff and prove every other test and product input unchanged.
- [ ] Seal truthful bootstrap implementation evidence with retained failures.
      The final package seal and handoff record completion after document freeze.

Existing App shell-region and conversation create/restore/reject/approve integration
coverage remains unchanged. Risks are accidental narrowing of structural assertions
or accidental edits outside the selected test; exact diff review and full frontend
validation address them. There is no automatic rollback: preserve failures and
original bytes, and stop on a genuine boundary rather than reset or clean work.

## Execution and budgets

Use the existing attended local executor, installed npm/Node and the preserved
Python shim for repository checks; use `/usr/bin/python3 -B` for the session check.
Bind actual tool identities and inputs in receipts. This is not the task-bound
D-137 runner and makes no unattended or arbitrary-command execution claim.

Each command is limited to two minutes plus five seconds for cancellation. The
attending agent owns outcome recording and may signal only positively identified
task-owned processes. Uncertain ownership or unconfirmed termination stops further
execution and leaves a retained unknown outcome, without replay.

Limits are two corrective retries per stable stage, six overall, one hour of active
diagnosis/repair, four elapsed hours from recorded authorization and ten further
minutes solely for preservation/necessary owned-process termination. Usage and
pause boundaries persist; no resets or inferred idle intervals. Initial inspection
is charged conservatively. The original publication ledger is untouched.

## Required validation and closeout

Run the named test, `src/App.test.tsx`, then full `npm run test:frontend` with
unchanged execution settings. Run `npm run format:frontend`,
`npm run lint:frontend`, `npm run typecheck`, `npm run docs:check`,
`npm run repository:check`, `npm run security:scan`, `git diff --check` and
`/usr/bin/python3 -B .codex/hooks/session_end_gate.py` after final relevant edits.
Reuse unchanged native/hook/repository test evidence with provenance; no full
verify or build is required for this test-only change. Record unique tests
separately from repeated executions. Review only this correction, not cleared
routing/lifecycle/cancellation/adoption work.

The final review and sealed package establish only bootstrap implementation
verification. Publication and actual GitHub Actions acceptance require separate
authority and evidence. Stop on access loss, drift, exhausted limits, uncertain
containment or required scope expansion.

## Observed results

The focused case passed (82 ms reported test duration), all 43 App tests passed,
and all 601 frontend tests across 40 files passed. These total 645 executions of
601 unique cases, including 44 repeats; 42 filtered cases in the focused run are
not additional executed tests. Formatting, lint, typecheck, documentation,
repository, security, whitespace and nonmutating session checks all passed.
Source SHA-256: `ed0ba0c624739d049de17a3f9142d70b4eb241ae9b44591999552719a00b13bc`.
No executable changes follow these results. The final documentation checks bind
the report update separately; successful unchanged source checks are reused.

The independent reviewer found no concrete defects. The retained Node localStorage
warning and the distinction between local structural evidence and actual GitHub
Actions acceptance remain. See the
[final review](../reviews/2026-10-10-sidebar-overflow-test-correction-review.md).
