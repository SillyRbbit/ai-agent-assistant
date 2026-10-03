# Bounded top-level OpenAI stream-error diagnostics

Status: Implementation and offline verification passed on 2026-10-02; quality
PASS WITH ADVISORIES. Ordinary completion and full Stop receipts are retained
externally. Owner authorization is the bounded implementation request.
No launch or live request is allowed.

## Workspace and preserved baseline

Worktree: `/Users/hdang/.codex/worktrees/local-diagnostics/ai-agent-assistant`.
Branch: `codex/local-diagnostics`. HEAD: `662fe1a57a1148a215beb04b215ad35681c4cbe6`.
The existing 31-path candidate and local-diagnostics complete/valid report remain
historical evidence. Its raw state and candidate bytes were archived unchanged in
`/private/tmp/cortexa-openai-stream-error-diagnostic-evidence/baseline` before begin.
`preflight.json` freezes predecessor, protected files, external evidence and both
debug bundles. No other visible active local task targeted this worktree.

## Objective and boundaries

Classify only the accepted top-level SSE `error.code` into a closed diagnostic
label. Preserve the original generic conversation error, request construction,
sequence/framing checks, response.failed mapping, retry policy and cleanup.
Use the existing correlated observer and diagnostic record fields; no new IPC,
storage fields, dependencies, provider capability, raw text or credential access.
No arbitrary message, code, param, nested error object, frame or response ID is
retained. This cannot recover the discarded cause of the earlier failed request.

The official [streaming reference](https://developers.openai.com/api/reference/resources/responses/streaming-events)
defines top-level code as string or null, not an exhaustive enum. Cortexa recognizes
only exact `server_error`, `rate_limit_exceeded`, and `invalid_prompt` literals.
Absent/null means missing; empty/non-string means invalid; any other nonempty
string means unknown. No trimming, case folding or nested-code fallback. These
are literal classifications, not a claim of a specific account or provider cause.

## Frozen scope

Exactly fifteen successor paths, thirty-three cumulative paths:

- `src-tauri/src/personal_assistant_direct.rs`
- `src-tauri/src/diagnostics.rs`
- `src-tauri/src/diagnostics/tests.rs`
- `src/infrastructure/tauri/diagnostics-client.ts`
- `src/infrastructure/tauri/diagnostics-client.test.ts`
- `src/features/settings/DiagnosticsPanel.test.tsx`
- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- This plan and its matching post-increment review.

Preserve all inherited document bodies. All other candidate/product/configuration,
hook, dependency and artifact bytes are protected. Reversal would require owner
authorization and use the saved baseline; no automatic rollback or cleanup.

## Checklist and validation

- [x] Verify complete/valid predecessor, receipt, baseline and frozen scope.
- [x] Archive prior raw completion and candidate bytes; ordinary begin accepted.
- [x] Implement closed classification and one bounded correlated observation.
- [x] Test configured/default decoder paths, coalesced/split frames, shape and
      sequence rejection, fixed buckets and unchanged response.failed behavior.
- [x] Test snapshot/disk/restart/export canary exclusion, late-event rejection,
      disabled/unavailable logging, and frontend closed parsing/presentation.
- [x] Run affected Rust/frontend tests; one final offline `npm run verify` is
      required because this crosses native diagnostic persistence and typed IPC.
- [x] Documentation/repository/security/whitespace, exact scope/preservation,
      independent review and session checks.
- Require report validation, ordinary finalization, complete/valid status and
  full-payload Stop; the external receipts establish their actual outcomes.

Reuse the installed process-local Python 3.12/Xcode SDK27/Cargo strip=none route;
no installs/downloads. Full verification builds no bundle and does not launch the
application. Existing native layout/Simulation evidence is inherited, not rerun.
Computer Use and live verification of new labels are not authorized and will be
reported unobserved, not passed. Existing bundles do not contain this change.

## Risks, evidence and current checkpoint

Risks are retaining untrusted material, conflating failure shapes, duplicate or
late diagnostic events, incompatible readers, or accidentally changing request
behavior. Closed enums, one event per live attempt, strict frontend parsing and
adversarial tests address these. Old records remain readable by the new code;
older binaries may reject new enum labels, so no data copying/downgrade is proposed.

Replacement-key evidence is the preserved external receipt at
`/private/tmp/cortexa-local-diagnostics-new-key-qa-cpmkkfo0/observations.json`.
The old batch remains exhausted at 5/5; the new batch is 1/10 used, 9 remaining.
No retry/fallback. Live success, actual provider cause and live cancellation remain
unverified. Retain D-127/D-128, native/provider/runtime and Codex-isolation limits,
process-local workaround, owner/760px QA and parked D-125/M1/M2.

Final verification: 7 focused Rust tests and 7 focused frontend tests passed.
Full offline verify passed with 540 frontend, 430 native unit, 74 hook and 88
repository tests, all integrations, strict lint/format/typecheck and native release
build. One pre-existing opt-in Hermes version probe remained ignored, not verified.
No product test failed; an initial documentation-write approval rejection
was safely resolved by hash-rechecking current files and applying additive patches.
The rejected script was not executed. Historical bodies remain byte-identical.
Independent review found no actionable correctness/security/architecture defect.

Next action after valid closure: separately authorize an updated isolated bundle
for native QA. Browser preview from this worktree is
`npm run dev -- --host 127.0.0.1` at http://127.0.0.1:1420; native diagnostics need
a separately authorized updated native artifact/QA. No preview is launched here.
