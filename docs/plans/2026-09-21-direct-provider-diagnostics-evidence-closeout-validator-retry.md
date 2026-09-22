# Direct provider diagnostics evidence closeout validator retry

Date: 2026-09-21. Increment: direct-provider-diagnostics-evidence-closeout-validator-retry.
Status: Complete.

## Goal

Produce valid documentation-only closeout evidence for the inherited direct-provider
candidate by correcting one external preservation-validator type mismatch.

## User-visible outcome

Repository memory truthfully records the diagnostics candidate and its terminal
predecessors, with a valid completion marker for this bounded evidence closeout.

## Scope

The repository delta is exactly CHANGELOG.md, HANDOFF.md, NEXT_STEPS.md, PLANS.md,
PROJECT_STATUS.md, ROADMAP.md, TESTING_GUIDE.md, TROUBLESHOOTING_LOG.md, this plan,
and its review. The cumulative Git inventory is exactly 44 paths.

## Explicit non-goals

Do not alter executable, test, dependency, configuration, workflow, hook, skill or
harness bytes; install or resolve packages; run application tests or builds; launch
Cortexa; inspect credentials; make provider requests; commit; publish; or advance
D-125/M1/M2.

## Existing behavior and constraints

The inherited retry is terminal failed and immutable. Its source candidate contains
42 changed paths. Its external validator returned raw Git bytes but compared them
to a string. The original closeout validator’s absent-path correction returns None.
All three approved live attempts are exhausted.

## Current-state evidence

The source retry is detached at 0ed15810e90b6a4bd312a8096c61b0abb1ab7eff with a
valid failed status. Its state, report and status fingerprints were frozen before
creation. The candidate transferred byte-identically without gate state. Existing
formatter tooling was clone-copied from the verified source only after executable,
APFS-volume, target-absence, ignored-path and source-unchanged checks passed.

## Files expected to change

The ten repository paths listed in Scope. The new preservation and report-schema
scripts live outside the repository under
/private/tmp/cortexa-direct-diagnostics-evidence-closeout-validator-retry-evidence.

## Affected components

Repository current-state documentation, plan/review evidence, and external
preservation validation only.

## Interfaces and invariants

The external Git helper remains byte-returning. `entry()` returns None for absent
paths. The only corrective membership comparison uses `b"node_modules"`. Every
predecessor worktree, report, raw state, completion marker, external script and
the two prunable worktree registry entries remain unchanged.

## Implementation milestones

- [x] Verify predecessor records, candidate bytes, baseline, registry and COW tooling.
- [x] Admit the successor and create the bounded documentation plan.
- [x] Run offline documentation and preservation workflows.
- [x] Complete independent review, schema validation, finalization and Stop.

## Security and privacy considerations

No credential, process environment, provider payload, provider request or native
app interaction is accessed. The local formatter clone is ignored by Git and is
not an installation or dependency-resolution action.

## Test plan

Run documentation/repository/security checks, diff hygiene, external preservation,
session/quality review, report-schema validation, ordinary finalization, status and
Stop. Treat historical application results as inherited evidence only.

## Verification commands

```bash
env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run docs:check
env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run repository:check
env -u OPENAI_API_KEY CORTEXA_OPENAI_DEMO=0 CARGO_NET_OFFLINE=true npm_config_offline=true npm run security:scan
git diff --check
python3 -B /private/tmp/cortexa-direct-diagnostics-evidence-closeout-validator-retry-evidence/preserve.py
python3 -B .codex/hooks/session_end_gate.py
python3 -B /private/tmp/cortexa-direct-diagnostics-evidence-closeout-validator-retry-evidence/validate_report.py
```

## Risks

Any source drift, clone failure, dependency download, changed protected byte,
validator scope expansion or failed validation invalidates completion.

## Rollback or failure strategy

Stop immediately, retain all changes and record a truthful terminal failure with
no automatic repair, reset, clean, reclose, commit or publication.

## Decisions made

Use a bytes literal at the one membership comparison rather than decoding the
shared helper, preserving its NUL-safe and hash-compatible behavior.

## Discoveries

The previous retry successfully resolved formatter absence and absent-path
handling. It stopped solely because Python rejects membership comparison between
str and bytes.

## Progress

2026-09-21: Candidate and formatter tooling were verified and transferred. The
new increment was admitted. The initial formatter-only documentation failure was
repaired locally; all required documentation-tier, preservation, session, quality,
report-schema, finalization and Stop evidence passed.

## Acceptance criteria

- [x] The repository delta equals the ten-path allowlist and cumulative inventory equals 44.
- [x] Protected predecessor and candidate bytes, all terminal records and registry entries match frozen evidence.
- [x] Every required documentation-tier and completion workflow passes.
- [x] The report schema validates, status is complete and Stop passes.

## Final results

Complete. The external validator proved the corrected byte comparison, exact scope, protected bytes, historical suffixes, formatter clone and registry. The first documentation check found formatting only; the local formatter repaired it and the rerun passed. No application test or build was rerun.

## Documentation updates

- [x] `HANDOFF.md`
- [x] `PROJECT_STATUS.md`
- [x] `NEXT_STEPS.md`
- [x] `CHANGELOG.md`
- [x] `TROUBLESHOOTING_LOG.md`
