# V0 terminal-failed successor disposition recovery

Status: Tracked-evidence freeze for exceptional same-terminal-record recovery;
no normal gate
Owner: Henry Dang
Last updated: 2026-08-29
Baseline: `a417e5f1c1c602b917ca27c65af71480e3db6a45`
Decision: D-098

## Goal

Implement one source-allowlisted cumulative-evidence disposition for the
published D-097 `failed` / `FAIL` / `Blocked` record without changing its
historical report, result, readiness, or missing completion marker. The recovery
may make only the exact documentation successor
`personal-assistant-v0-signing-security-prerequisite-planning` technically
admissible after a clean workspace and separate owner-controlled start.

## User-visible outcome

No product or external-system outcome. The repository gate can truthfully show
the unchanged terminal failure plus one separately passing, redacted,
exact-target disposition lineage. It does not start the successor.

## Scope

- Add state schema v3 with exact `successor_disposition` lineage while retaining
  legacy v1/v2 compatibility.
- Add one argument-free `record-failed-disposition` command bound in source to
  the exact predecessor, baseline, recovery, report, 22 paths, and successor.
- Keep the exact source-bound 22-path recovery inventory distinct from the
  nested exact 15-path ceiling governing the later documentation successor.
- Carry validated lineage into the one later active successor as
  `predecessor_disposition`.
- Add focused fail-closed tests and synchronize repository workflow policy,
  security/current-state memory, and exact recovery evidence.
- Reconcile PR #84 publication at squash commit
  `a417e5f1c1c602b917ca27c65af71480e3db6a45` without rewriting the original
  report.

## Explicit non-goals

- No second increment and no `begin`, `finalize`, or `close-failed` call.
- No completion marker, failed-to-complete promotion, generic override,
  abandonment, waiver, arbitrary report, arbitrary path, or caller-selected
  successor.
- No edit to the original failed report, its increment record, or the D-095 and
  D-096 historical plans.
- No product source, dependency, lockfile, workflow, runner, classifier,
  capability, CSP, permission, Tauri, credential, provider, network,
  persistence, signing, Apple, Xcode, Keychain, filesystem-product, or device
  behavior.
- No commit, push, merge, publication, or automatic successor start.

## Existing behavior and constraints

D-097 records the predecessor as valid `failed` / `FAIL` / `Blocked`. Its
tracked report includes immutable Failed privacy evidence, a Pending Open
Directory boundary, and a Not-run signed build. The ignored state is
checkout-local and same-user writable; it is workflow evidence, not
authentication, authorization, or durable audit. Its Blocked readiness denies
every ordinary successor. A fresh clone must not be used to bypass it.

The recovery is an exceptional owner-authorized change against that same
terminal record. Any edit temporarily invalidates the predecessor workspace
fingerprint, so the new command must atomically validate the unchanged
predecessor evidence and the exact recovery diff before recording schema v3.

## Current-state evidence

- `HEAD`, `main`, and `origin/main` were synchronized at
  `a417e5f1c1c602b917ca27c65af71480e3db6a45` before recovery work began.
- `python3 .codex/hooks/post_increment_gate.py status` reported the exact D-097
  predecessor as valid `failed` / `FAIL` / `Blocked`.
- The owner explicitly authorized this exceptional same-terminal-record source
  recovery. That authority does not extend to publication or the successor.
- No normal gate began, and the predecessor state/report were not edited.

## Files expected to change

Exactly these 22 paths:

1. `.agents/skills/post-increment-gate/SKILL.md`
2. `.agents/skills/verified-increment/SKILL.md`
3. `.codex/hooks/post_increment_gate.py`
4. `.codex/hooks/tests/test_post_increment_gate.py`
5. `AGENTS.md`
6. `ARCHITECTURE.md`
7. `CHANGELOG.md`
8. `CODE_REVIEW.md`
9. `DECISIONS.md`
10. `ENGINEERING_GUIDE.md`
11. `HANDOFF.md`
12. `NEXT_STEPS.md`
13. `PLANS.md`
14. `PROJECT_STATUS.md`
15. `ROADMAP.md`
16. `SECURITY.md`
17. `TESTING_GUIDE.md`
18. `TROUBLESHOOTING_LOG.md`
19. `docs/templates/POST_INCREMENT_REVIEW_TEMPLATE.md`
20. `docs/plans/2026-08-29-v0-terminal-failed-successor-disposition-recovery.md`
21. `docs/increments/v0-terminal-failed-successor-disposition-recovery.md`
22. `docs/reviews/2026-08-29-v0-terminal-failed-successor-disposition-recovery-post-increment-review.md`

## Affected components

- Repository-local post-increment state parser, transition logic, status, Stop
  decision, and command parser.
- Focused Python state-machine tests.
- Repository workflow/security guidance and current project memory.
- No product component.

## Interfaces and invariants

The public recovery interface is exactly:

```bash
python3 .codex/hooks/post_increment_gate.py record-failed-disposition
```

It accepts no arguments. Exact source constants bind the failed increment,
predecessor report, baseline commit, recovery ID, recovery report, successor
ID, exact 22-path recovery inventory, and exact 15-path future-successor
ceiling. No caller supplies trusted identity or readiness. The recovery command
matches the complete current diff against the 22-path inventory; the nested
`successor_disposition.allowed_paths` field stores only the distinct 15-path
ceiling for the later documentation successor.

That future-successor ceiling is exactly:

1. `ARCHITECTURE.md`
2. `CHANGELOG.md`
3. `DECISIONS.md`
4. `HANDOFF.md`
5. `NEXT_STEPS.md`
6. `PLANS.md`
7. `PROJECT_STATUS.md`
8. `ROADMAP.md`
9. `SECURITY.md`
10. `SECURITY_CHECKLIST.md`
11. `TESTING_GUIDE.md`
12. `TROUBLESHOOTING_LOG.md`
13. `docs/increments/personal-assistant-v0-signing-security-prerequisite-planning.md`
14. `docs/plans/2026-08-29-personal-assistant-v0-signing-security-prerequisite-planning.md`
15. `docs/reviews/2026-08-29-personal-assistant-v0-signing-security-prerequisite-planning-post-increment-review.md`

Schema v3 changes only the top-level `schema_version`, preserves every other v2
failed-state evidence field, and adds exactly one `successor_disposition`
object with:

- `disposition_id`;
- `predecessor_state_sha256`;
- `report_path`;
- `report_sha256`;
- `baseline_commit`;
- `workspace_fingerprint`;
- `successor_increment_id`;
- `quality_gate`;
- `next_increment_readiness`; and
- `allowed_paths`.

The predecessor remains `failed`, its quality remains `FAIL`, its original
readiness remains `Blocked`, and it has no completion marker. The recovery
report must compute `PASS` or `PASS WITH ADVISORIES`, have non-Blocked
readiness, contain no next-blocking finding, bind the complete exact diff, and
record every required automated/manual result as Passed. Exact replay may be
idempotent; altered replay fails closed.

Only the exact successor may later call `begin`, from a clean workspace and
valid disposition. Its active schema-v3 state carries the validated lineage as
`predecessor_disposition`. No transition starts automatically, and the lineage
never becomes product or security authority.

## Implementation milestones

- [x] Owner authorized one exceptional recovery against the existing terminal
      record; no normal gate began.
- [x] Implement exact state-schema-v3 parsing, validation, status, Stop, and
      one-target successor admission.
- [x] Implement the argument-free, source-allowlisted, atomic
      `record-failed-disposition` transition.
- [x] Add focused success, compatibility, drift, replay, transition, and
      adversarial tests.
- [x] Synchronize the exact 22 documentation/evidence paths without altering
      historical predecessor evidence.
- [x] Run focused and complete verification, independent review, and freeze the
      exact recovery report.
- [ ] Run the transition once, confirm valid redacted status, and stop without
      beginning, committing, publishing, or operating externally. This is
      intentionally post-freeze evidence and cannot be checked in tracked text.

## Security and privacy considerations

The command must reject caller-selected identities, alternate paths, alternate
reports, alternate successors, malformed or unknown state, predecessor/report
drift, a non-passing recovery report, Blocked readiness, next-blocking findings,
failed or pending required evidence, conflicts, suspicious paths, dirty
successor admission, and non-identical replay. State writes remain atomic and
mode-restricted. No raw report content, file digest, workspace digest, local
path, or private project evidence is added to redacted status.

The ignored state remains forgeable by a process with same-user repository and
state access. D-098 does not claim authentication, authorization, durable audit,
or resistance to a malicious local owner. Repository policy and explicit owner
control remain necessary.

## Test plan

Focused tests cover exact v3 keys; legacy v1/v2 compatibility; immutable
predecessor fields; exact source constants and 22 paths; passing-report and
manual-evidence enforcement; Blocked and next-blocking rejection; argument-free
CLI parsing; exact idempotence and altered replay denial; state, report, and
workspace drift; conflict and suspicious-path denial; no completion marker;
redacted status; Stop behavior; exact clean successor admission; wrong, dirty,
same, and unrelated successor rejection; and `predecessor_disposition`
lineage.

Complete verification covers all repository tests, strict checking, production
frontend build, and target-Mac Tauri release no-bundle build. Apple, Xcode,
Keychain, signing, credential, and product runtime checks are Not run because
they are prohibited and irrelevant to this repository-governance recovery.

## Verification commands

```bash
python3 -m unittest discover -s .codex/hooks/tests -p 'test_post_increment_gate.py' -v
python3 -c "from pathlib import Path; paths = ('.codex/hooks/post_increment_gate.py', '.codex/hooks/tests/test_post_increment_gate.py'); [compile(Path(path).read_text(encoding='utf-8'), path, 'exec') for path in paths]"
npm run test:hooks
npm run verify
npm audit --audit-level=low
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
git diff --exit-code -- src src-tauri package.json package-lock.json src-tauri/Cargo.toml src-tauri/Cargo.lock .github scripts
python3 .codex/hooks/session_end_gate.py
```

After every pre-disposition check and manual gate passes and the recovery
report is frozen, run the separately evidenced transition and redacted-status
check:

```bash
python3 .codex/hooks/post_increment_gate.py record-failed-disposition
python3 .codex/hooks/post_increment_gate.py status
```

The frozen report's machine manifest contains only commands truthfully executed
before the transition. The transition and subsequent status cannot appear as
pre-passed report verification because the disposition binds that report's
digest; their actual output and resulting ignored state are separate
post-report evidence. Run them only after every preceding required check,
manual gate, independent review, and report freeze passes, then stop.

## Manual gates

- Owner authorization of this exact exceptional source recovery: Passed.
- Exact baseline and valid predecessor state before edits: Passed.
- Original predecessor report/state content remains unchanged: Passed.
- Exact 22-path inventory and no prohibited path: Passed.
- Independent architecture, security, code-health, technical-debt, and
  readiness reviews: Passed with accepted non-blocking advisories.
- No prohibited Apple, Xcode, Keychain, signing, credential, product, or
  external-system mutation/operation: Passed. The required
  `npm audit` registry read is recorded separately and grants no other network
  or external authority.
- Commit, push, merge, publication, and successor start: Not authorized and Not
  run.

## Risks

- A broad or caller-selectable transition could become a governance bypass.
- Mutating predecessor quality/readiness or emitting a marker would launder
  failure.
- Incorrect fingerprint or replay rules could admit drifted evidence.
- Failing to carry lineage into the successor could discard the reason its
  admission was exceptional.
- Documentation that calls this completion or operational readiness could
  overstate authority.

## Rollback or failure strategy

The state write must be atomic; a failed write leaves the prior v2 state file
intact. Before the command succeeds, any failed check leaves the recovery
unrecorded and the successor Blocked. Do not reset, clean, stash, or silently
discard work. Abandonment requires explicit owner authorization to reverse only
the 22 reviewed paths, after which the original state can validate again.

After schema-v3 disposition is written, rollback requires a separately approved
recovery because tracked evidence and ignored lineage are coupled. After
publication, use a non-destructive Git revert and do not rewrite the historical
failed report.

## Stop conditions

Stop on any baseline/state mismatch, predecessor report or state mutation,
unexpected path, merge conflict, suspicious file, generic/caller-selectable
interface need, non-passing required check or manual gate, Blocked recovery
readiness, next-blocking finding, state-write atomicity defect, inability to
preserve v1/v2 compatibility, product/dependency/workflow/external scope need,
or request to begin/publish/operate without separate authority.

## Decisions made

D-098 selects only this one additive exact-target lineage. It does not select a
general recovery mechanism or authorize the documentation successor to start.

## Discoveries

The valid D-097 state survives publication of identical reviewed contents but
cannot change its original Blocked readiness. The cumulative evidence therefore
must be a second immutable lineage rather than a revised predecessor report.

## Progress

- 2026-08-29: Owner authorized the exceptional recovery; clean synchronized
  baseline and valid predecessor status were confirmed. Documentation and
  implementation began without a normal gate transition.
- 2026-08-29: Schema v3, the exact argument-free transition, immutable
  predecessor reconstruction, 22-path recovery scope, 15-path successor
  ceiling, redacted status, lineage propagation, and fail-closed tests were
  implemented. All 65 focused tests passed.
- 2026-08-29: Independent architecture, security, code-health, debt, and
  readiness review defects were corrected. The tracked report was frozen as
  pre-transition evidence; its later state transition remains intentionally
  outside tracked closeout.

## Acceptance criteria

- [x] Complete diff is exactly the 22 allowed paths.
- [x] Original report/state evidence remains unchanged and no completion marker
      exists.
- [x] Every focused and complete required check passes.
- [x] Independent reviews contain no completion- or next-blocking finding.
- [x] Recovery report computes PASS or PASS WITH ADVISORIES and non-Blocked
      readiness.
- [ ] Argument-free transition writes valid schema-v3 lineage atomically.
- [ ] Status remains predecessor `failed` / `FAIL` / `Blocked` and identifies
      only the exact documentation successor.
- [x] No successor, publication, product, or prohibited external-system
      mutation/operation begins.

## Final results

At tracked-evidence freeze, the argument-free state transition and subsequent
redacted status are necessarily Not run. The frozen report records every
pre-transition verification and independent review. Only the ignored
schema-v3 state plus observed `post_increment_gate.py status` output can
establish the later transition result; no tracked closeout edit follows it and
no completion marker exists.

## Documentation updates

- [x] `HANDOFF.md`
- [x] `PROJECT_STATUS.md`
- [x] `NEXT_STEPS.md`
- [x] `DECISIONS.md` with D-098
- [x] `CHANGELOG.md`
- [x] `TROUBLESHOOTING_LOG.md`
