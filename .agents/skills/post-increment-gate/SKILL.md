---
name: post-increment-gate
description: Run Cortexa's consolidated post-increment verification, engineering review, documentation sync, report generation, and deterministic passing or terminal-failed disposition before ending an implementation increment.
---

# Post-increment gate

Run this workflow only after the approved implementation is finished. Do not commit, push, begin another increment, or automatically fix advisory findings.

## A. Repository state review

1. Read `AGENTS.md`, `HANDOFF.md`, `PROJECT_STATUS.md`, `NEXT_STEPS.md`, `DECISIONS.md`, `TROUBLESHOOTING_LOG.md`, `SECURITY.md`, `CODE_REVIEW.md`, `PLANS.md`, the active increment and plan, and this skill.
2. Run `python3 .codex/hooks/post_increment_gate.py status` and confirm the expected increment is active.
3. Run `python3 .codex/hooks/session_end_gate.py` and inspect its staged, unstaged, untracked, and conflict inventory.
4. Inspect the branch, complete diff, and recent commits.
5. Refuse completion when merge conflicts exist.
6. Inspect every changed path for unrelated scope, generated files, build output, local databases, environment files, credentials, certificates, private keys, logs, or personal data.

## B. Required verification

1. Run every automated command required by the active increment. Do not substitute an earlier run.
2. Record each command as exactly `Passed`, `Failed`, `Not run`, or `Manual verification pending`.
3. Record each required manual check using the same statuses.
4. Never claim success without the command's actual zero exit status or the project owner's explicit manual confirmation.
5. Any failed or not-run required command, or any pending required manual check, makes the quality result `FAIL`.

## C. Architecture review

Apply `$architecture-review` to module boundaries, ownership, trust boundaries, coupling, cohesion, drift, abstractions, maintainability, portability, performance, and dependency health.

## D. Security review

Apply `$security-review` to Tauri IPC, hooks, capabilities, CSP, approval and policy boundaries, unsafe Rust, secrets, logging and audit exposure, SQLite safety, filesystem and operating-system access, network additions, and permission changes.

## E. Code-health review

Apply `$code-review` to naming, folder organization, type safety, error handling, test quality, accessibility, dead code, documentation accuracy, complexity, and duplication.

## F. Technical-debt review

Apply `$technical-debt`. For every finding record category, severity, summary, concrete risk, estimated effort, recommended milestone, whether it blocks completion, and whether it blocks the next increment. Use only `Critical`, `High`, `Medium`, `Low`, or `Advisory`.

Any Critical or High finding that blocks completion makes the quality result `FAIL`.

## G. Roadmap and readiness review

Apply `$readiness-review` and classify the next increment as exactly `Ready`, `Ready with advisories`, or `Blocked`. Do not reorder `NEXT_STEPS.md` unless the recommendation is recorded and approved.

## H. Documentation sync

Update the applicable `HANDOFF.md`, `PROJECT_STATUS.md`, `NEXT_STEPS.md`, `CHANGELOG.md`, `DECISIONS.md`, `PLANS.md`, `TROUBLESHOOTING_LOG.md`, active increment record, and active plan. Preserve historical evidence and include an exact resume prompt.

## I. Consolidated report and marker

1. Create `docs/reviews/YYYY-MM-DD-<increment>-post-increment-review.md` from `docs/templates/POST_INCREMENT_REVIEW_TEMPLATE.md`.
2. Include the exact machine manifest, every required section, complete changed-file inventory, and exact commands executed.
3. Set the result to exactly `PASS`, `PASS WITH ADVISORIES`, or `FAIL` based on the recorded evidence.
4. Review the complete report and diff before finalization.
5. Only for an ordinary active increment with `PASS` or
   `PASS WITH ADVISORIES`, run:

   ```bash
   python3 .codex/hooks/post_increment_gate.py finalize \
     --increment <increment> \
     --report docs/reviews/YYYY-MM-DD-<increment>-post-increment-review.md
   ```

6. For `FAIL`, never call `finalize`. Freeze the truthful report, require
   `next_increment_readiness: Blocked` whenever any finding blocks the next
   increment, and run:

   ```bash
   python3 .codex/hooks/post_increment_gate.py close-failed \
     --increment <increment> \
     --report docs/reviews/YYYY-MM-DD-<increment>-post-increment-review.md
   ```

7. Run `python3 .codex/hooks/post_increment_gate.py status`. A passing result
   requires `status: complete` with `valid: true`. A truthful failed result
   requires `status: failed`, `quality_gate: FAIL`, and `valid: true`; it has no
   completion marker and grants no completion or successor authority.

## J. Exact D-098 same-terminal-record recovery

This exceptional path applies only to
`v0-terminal-failed-successor-disposition-recovery` under D-098. It is not a
new increment. Do not call `begin`, `finalize`, or `close-failed`, and do not
modify the predecessor report or state by hand.

1. Confirm `status` reports the exact valid D-097 predecessor as
   `failed` / `FAIL` / `Blocked`.
2. Confirm the complete diff is exactly the D-098 22-path allowlist and the
   recovery report records every required command and manual gate.
3. Require `PASS` or `PASS WITH ADVISORIES`, non-Blocked recovery readiness,
   no next-blocking finding, and passing independent architecture, security,
   code-health, and readiness review.
4. Freeze the report, then run exactly once:

   ```bash
   python3 .codex/hooks/post_increment_gate.py record-failed-disposition
   ```

5. Run `status`. The predecessor must remain `failed` / `FAIL` / `Blocked`
   without a completion marker, while its schema-v3 cumulative evidence names
   only `personal-assistant-v0-signing-security-prerequisite-planning` as the
   validated successor. Stop; do not begin it, commit, or publish
   automatically.

The ignored local state is checkout-local workflow evidence, not authentication
or authorization. Its hashes detect unreclosed report or workspace drift but do
not protect against a malicious same-user rewrite. A completion marker exists
only for a passing result. A terminal-failed record preserves failure and lets
the Stop hook end; it grants no security, approval, audit, execution,
publication, or verification authority beyond the validated report and current
workspace.
