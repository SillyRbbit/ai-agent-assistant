---
name: verified-increment
description: Implement one small, bounded Cortexa increment with explicit non-goals, typed errors, focused tests, security review, and complete handoff updates.
---

# Verified increment

1. Select one ready item from `NEXT_STEPS.md` or one explicitly requested bounded task.
2. Read the relevant product, security, decision, and increment documents.
3. State goal, non-goals, expected files, risks, and verification commands.
4. Use existing owner authorization, prepare the checklist/readiness evidence where needed, then begin through the supported gate before implementation. New general milestones use schema-2 admission. Preserve the separately recorded D-098/D-133 historical procedures; never replay them for arbitrary new work.
5. Preserve current behavior outside scope.
6. Implement the smallest coherent change.
7. Add or update focused tests for success and failure behavior.
8. Use strict TypeScript and typed Rust errors; no panic-style production shortcuts.
9. Run targeted checks, then the full relevant verification.
10. Review the diff using `CODE_REVIEW.md` and `SECURITY.md`.
11. Update handoff, status, next steps, changelog, decisions, troubleshooting, and increment documentation as applicable.
12. Run `$post-increment-gate`; do not mark an ordinary increment complete without a valid passing report and completion marker. D-098 instead requires its separate passing recovery report and valid schema-v3 disposition while retaining no completion marker.

Resolve recoverable in-scope failures and assess inherited failures by impact. Continue independent authorized work; stop when a required decision, attribution, access or verification remains blocked.

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
