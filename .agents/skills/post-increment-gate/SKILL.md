---
name: post-increment-gate
description: Run Cortexa's consolidated post-increment verification, engineering review, documentation sync, report generation, and deterministic passing or terminal-failed disposition before ending an implementation increment.
---

# Post-increment gate

Run this workflow only after the approved implementation is finished. Complete authorized local commits only after checks and review. Do not start another increment or perform unauthorized publication; defer unrelated advisories.

## A. Repository state review

1. Read `AGENTS.md`, `HANDOFF.md`, `PROJECT_STATUS.md`, `NEXT_STEPS.md`, `DECISIONS.md`, `TROUBLESHOOTING_LOG.md`, `SECURITY.md`, `CODE_REVIEW.md`, `PLANS.md`, the active increment and plan, and this skill.
2. Run `python3 .codex/hooks/post_increment_gate.py status` and confirm the expected increment is active.
3. Run `python3 .codex/hooks/session_end_gate.py` and inspect its staged, unstaged, untracked, and conflict inventory.
4. Inspect the branch, complete diff, and recent commits.
5. Refuse completion when merge conflicts exist.
6. Inspect every changed path for unrelated scope, generated files, build output, local databases, environment files, credentials, certificates, private keys, logs, or personal data.

## B. Required verification

1. Run every affected required check. Reuse valid unchanged-input evidence with command, result, input identity and environment; do not represent reuse as a fresh execution.
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

## D-133 evidence-bound acceptance maintenance

The owner authorizes only the eighteen-path `evidence-bound-acceptance-maintenance`
bootstrap in `codex/collapsed-sidebar-reachability`, preserving the inherited
48-path product candidate with a 59-path cumulative ceiling. Ordinary begin is
blocked; this explicit bootstrap is not ordinary admission, D-098 recovery or
Desktop general closure. Preserve both terminal FAIL reports, raw states and
historical inventories byte-identically. Never reopen or promote either FAIL.

After recoverable snapshots, all required governance verification, passing
architecture/security/code-health/preservation/readiness reviews and a frozen
passing maintenance report, `seal-acceptance-maintenance --request <local-request>`
may publish an immutable maintenance receipt atomically. It writes no completion
marker and grants no acceptance or publication. Original failed/FAIL/Blocked
remains; status distinguishes historical integrity from the changed workspace.
Full Stop fails closed on missing evidence, artifact drift, scope or lineage
changes. Local receipt hashes are workflow evidence, not authentication against
malicious same-user rewriting.

Only a separate owner approval permits `begin --increment collapsed-sidebar-acceptance
--acceptance-request <local-request>`. It must bind the sealed receipt, current
fingerprint, DECISIONS.md hash and exact nine-document scope. Every required
nonpassing check and completion/next-blocking finding in both FAILs requires a
criterion-specific resolved mapping to sealed evidence and owner review. No
deferred, waived, unknown or omitted criterion is admitted. Preserve application
and artifact bytes, historical reports and prior document bodies.

The successor may edit only the seven current-state root documents and add its
new dated plan/report. Ordinary report validation, documentation/repository/
security/whitespace/session checks, reviews, finalization and full Stop apply.
Schema-v4 lineage carries immutable history and the bounded scope through passing
or terminal-failed disposition. A failed successor cannot reuse the old seal.
Legacy mechanisms and D-125/M1/M2 remain unchanged. No force option, checkout
bypass, arbitrary successor, product edits or automatic recovery chain follows.

See [the maintenance plan](../../../docs/plans/2026-10-03-evidence-bound-acceptance-maintenance.md).

## D-134 milestone operation

Follow AGENTS.md and ENGINEERING_GUIDE.md: objective, exclusions and acceptance
are the scope boundary. Path counts are informational; preserve protected paths,
attribution and unrelated work. Continue authorized routine work through review,
documentation and truthful finalization. Reuse valid unchanged-input verification
with provenance; distinguish implemented, automatically verified, live verified,
deferred and blocked criteria. Historical FAIL evidence and D-133's separate
acceptance route remain immutable. Request approval only at AGENTS.md boundaries.
