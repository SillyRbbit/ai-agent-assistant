# Bot Identity publication preparation

Increment `bot-identity-publication-preparation`; owner-authorized 2026-09-30.
Worktree `/Users/hdang/.codex/worktrees/bot-identity-publication/ai-agent-assistant`;
branch `codex/bot-identity-publication`; baseline
`e0d0547f626ce909c3cd74d4441a5adb2cdebbef`.

## Goal and inspected state

Prepare the accepted Bot Identity/personality, saved graph names, Conductor and
measurement-readiness candidate on actual main ancestry, correcting only graph
nickname Unicode-character counting. Source `codex/provider-milestone` HEAD
`fe7e663e175eaf7c515c17397cdd81060137a5b6` has the same tree as live main but
is a sibling after PR #127's squash merge. A direct source-branch PR would show
50 paths including five already-merged provider files. This worktree starts on
main and transferred the exact 45-path accepted candidate byte/mode-identically,
without gate state or ignored outputs. Ordinary admission succeeded.

The graph helper counted UTF-16 units; frontend/Rust profile validation counts
Unicode characters. Correct only `nickname.length` to `Array.from(nickname).length`
for the same limit 48. Test 25/48 supplementary characters, ASCII/mixed boundary
and 49-character rejection, preserving canonical IDs, roles and all fixture data.
No normalize/filter relaxation, provider/permission, profile, geometry, topology,
authority, routing, cancellation, dependency, workflow or governance change.

## Exact scope and preservation

Ten successor paths, 47 cumulative paths against main:

- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-09-30-bot-identity-publication-preparation.md`
- `docs/reviews/2026-09-30-bot-identity-publication-preparation-post-increment-review.md`
- `src/features/command-center/commandCenterBotNames.ts`
- `src/features/command-center/commandCenterBotNames.test.ts`

All other candidate bytes and repository bytes remain protected. Six source root
bodies remain exact historical suffixes. Original eight worktrees, 36 prunable
entries, completion state/reports, all external evidence and original artifacts
are machine-frozen. Evidence:
`/private/tmp/cortexa-bot-identity-publication-preparation-evidence`.
New worktree is the sole added registry entry. Local node_modules reused via
verified same-APFS `cp -cR`; no installation/download. DiskManagement was unavailable;
statfs semantics were confirmed against installed SDK before the clone.

## Evidence attribution

Inherited Bot Identity full offline verification/native build passed; subsequent
frontend corrections passed their declared checks. Those results are inherited,
not newly run here. Direct Computer Use receipt:
`/private/tmp/cortexa-measurement-readiness-native-qa-5g4lhnfv/observations.json`.
It observed the exact previous debug executable, first-entry fit 64%, enabled
controls, Conductor/Application coordinator, nine names, AgentOrchestrator inspector,
route return, zoom/pan and Fit restore. It did not verify live provider personality
or this Unicode correction visually. Original report/bundle remain unchanged.

## Checklist and validation

- [x] Live refs/source completion/preservation verified; isolated main worktree.
- [x] Exact candidate transfer and installed-tooling clone verified.
- [x] Ordinary admission; reviewed external machine preservation checker.
- [x] Unicode boundary red regression; minimal one-line correction and green.
- [x] Affected name/page tests; strict lint/typecheck/format and frontend build.
- [x] Documentation/repository/security/whitespace, exact scope and preservation.
- [x] Session, architecture/security/code-health/readiness and exact report schema.
- [ ] Ordinary finalization, complete/valid status and full-payload Stop.

Frontend tier for the correction; full verify/Rust/native builds are Not run for
unchanged inherited native/dependency/configuration bytes. Reuse passing evidence
truthfully. Remote exact-head CI/review is required after separate publication
approval; it is not available for these uncommitted changes.

## Risks, rollback and stop conditions

Unicode code points match existing validation, not grapheme-cluster normalization.
No automatic rollback or cleanup. Preserve all original work; stop on unexplained
ref/worktree/evidence drift, admission rejection, security failure or scope expansion.
Repair routine in-scope code/assertion/format/report issues in this task.
No launch, credential inspection, live/provider request, commit, push, PR or merge.
OpenAI parked 4/5 used; D-127/D-128, native/provider/runtime live-success,
Codex-isolation/internal-retry and process-local Python/Xcode/SDK/Cargo workaround
advisories retained. D-125/M1/M2 parked.
Next action: complete local validation, then request separate publication authority.

## Observed results and frozen closeout

Red run: Failed 3/18; fixed name/page run: Passed 53/53 (18 names, 35 page).
Strict lint/typecheck/format, production frontend build, documentation/repository/
security/whitespace and scope/preservation/session checks Passed. No new production
change beyond one counter. No native/fixture/dependency/governance checks repeated;
prior full verification/native artifact evidence is inherited and preserved.
Final documentation checks rerun after this additive closeout. Report/schema,
ordinary finalization/status and full Stop execute after freeze and are recorded
externally; the last checkbox remains pending until those actual receipts pass.
Do not rewrite this report/plan after completion to tick it. A future publication
requires explicit owner approval, exact 47-path sole-parent commit, frozen-byte
comparison, push without force and a new PR; all applicable exact-head CI/review
must pass before any separately authorized merge.
