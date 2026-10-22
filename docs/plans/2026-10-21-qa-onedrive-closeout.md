# Private OneDrive QA-build closeout

Status: Implemented and reviewed; final gate receipts control completion
Owner authorization: 2026-10-21 explicit implementation request in this chat.
Increment: qa-onedrive-closeout.

## Goal and user-visible outcome

Extend the existing conditional QA-build cleanup procedure with private OneDrive
archival and independently verified recovery before explicitly authorized local
removal. Every closeout records a disposition; no automatic executor is added.

## Scope and explicit non-goals

One central END_SESSION procedure, its engineering-guide reference, session-end
skill/prompt and additive current-state records. No executable tooling, hooks,
product/runtime/configuration/dependency changes, cloud operations, downloads or
restores, artifact deletion, process signals, permissions/credentials changes,
branch operations, commits, publication or automation activation. Existing gate
commands write only this checkout's local workflow evidence. No successor starts.

## Existing behavior and current-state evidence

- Isolated managed checkout, detached at 28948239934b6589f4b4ab55882dd13b1578490a;
  initially clean/inactive. No branch ref is created or changed. Other tasks stay
  in their own checkouts. Primary a9f2e5e remains dirty and unchanged.
- Commits 6e663ce and 3aff639 already provide conditional QA cleanup and separate
  worktree archival. Reuse them; no replay, replacement framework or executor.
- The completed October 21 archival/cleanup has immutable manifests, archive and
  recovery-record identities, independent download/restore receipts and per-path
  removal records. Reuse its design, never rerun its task-bound helpers against
  removed originals or generalize its exact owner releases.
- Original storage-resolution seal: all 158 bound local records matched during
  the preceding read-only review. This is local record integrity, not a fresh
  private-cloud availability check or a new restore.
- Historical HP/D-125/M1/M2 failures, missing acceptance and qualified runtime
  evidence remain unchanged. This documentation objective has no executable or
  acceptance dependency on them and resolves none of their criteria. Active
  Tasks/RAG/Main work and every resource hold remain protected. No historical
  state is copied into this checkout or promoted by a fresh gate.

## Files expected to change

- `.agents/skills/session-end/SKILL.md`
- `CHANGELOG.md`
- `DECISIONS.md`
- `ENGINEERING_GUIDE.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `docs/plans/2026-10-21-qa-onedrive-closeout.md`
- `docs/reviews/2026-10-21-qa-onedrive-closeout-post-increment-review.md`
- `docs/reviews/2026-10-21-qa-onedrive-closeout-readiness-post-increment-review.md`
- `docs/workflows/END_SESSION.md`
- `prompts/workflows/end-session.md`

These paths are an attribution inventory, not authority to modify protected work.
No TROUBLESHOOTING_LOG update is needed unless a new relevant failure occurs.

## Components, interfaces and invariants

This is an engineering runbook, not a Cortexa product action or device boundary.
The session-end skill and prompt delegate to END_SESSION; hooks remain completion
validators. Exact owner authorization, platform access and verified preservation
remain prerequisites to every later artifact operation. No cloud deletion or
online-only conversion is granted. Source, worktrees, Git, application data,
credentials, historical failures, required evidence and explicit local holds stay
protected. A later separately approved execution objective must bind actual paths
and archive/recovery identities; this documentation task chooses no candidates.

## Milestones and acceptance checklist

- [x] C1: single agent-guided procedure and separate implementation/cleanup status.
- [x] C2: exact inventories/holds/private identities and independent recovery proof.
- [x] C3: fresh dependency checks, narrow removal, receipts and measured reporting.
- [x] C4: entry-point/current-state integration, checks, preservation and gate.

## Security and privacy

Use owner-approved private cloud custody; screen uploads before any future
transfer and keep sensitive manifests/locators outside Git. Never log credentials,
authenticated download URLs or owner data. Record content/metadata limitations;
raw-image replacement requires its own explicit release. Retain on unknown scope,
links, dependencies, recovery evidence or denied access. No permissions bypass.

## Verification and review

Run the documentation tier once per stable relevant input set:

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
python3 -B .codex/hooks/session_end_gate.py
```

Existing Prettier 3.8.4 is supplied through a task-command PATH from the retained
Tasks dependency tree; Node 26.3.0/npm 11.16.0 and Python 3.12. No installation,
symlink, dependency or interpreter-file change. Full npm verify, application
builds/native QA and cloud recovery operations are Not run: no executable input
or artifact operation changes. Review the full diff for architecture, security,
code health, debt and readiness. Static scenarios: explicit hold, new descendant,
changed identity/hash, missing receipt, unsafe link/hardlink dependency, mounted
image, active consumer, private availability unknown, interrupted transfer and
partial removal. Static review is not execution of these scenarios.

Prepare a frozen readiness report and schema-2 admission using the existing gate.
After implementation and review, finalize only a passing report, validate complete
status and send the actual complete Stop payload to the unchanged hook. Do not
hand-edit gate state or change any historical fingerprint/report.

## Preservation, risks and rollback

Recoverable originals and complete checkout/primary file hashes are outside
temporary directories at
`/Users/hdang/.codex/visualizations/2026/10/21/01a16064-1596-7b52-bdcb-90c7155957dc/qa-onedrive-closeout-docs-20261021`.
Premature deletion, privacy loss, sync propagation and misleading metadata/space
claims are addressed in the prose contract; no executor is proven here. A later
rollback requires fresh attribution and separate applicable authority; never
reset other work or delete receipts. Preserve partial output and truthful failure
evidence on a blocker; finish independent safe documentation.

## Progress and discoveries

Readiness checks passed; frozen report admitted through normal schema 2. The
central procedure, guide, skill/prompt and additive current-state records are now
implemented. Full diff/static scenario review and five implementation checks passed; final
report-input checks and normal gate finalization follow.

## Final results

The central procedure and all entry points/current-state records are implemented.
Five implementation checks and direct static/preservation review passed. Final report-input checks and local finalization/status/Stop receipts control
completion; consult their actual outcomes in the external evidence directory.
No marker is valid without those passing results.
No cloud/archive/deletion operation is part of this increment.
