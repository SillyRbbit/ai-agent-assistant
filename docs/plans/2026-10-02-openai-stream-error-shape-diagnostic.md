# Bounded OpenAI stream-error shape diagnostic

Status: Implementation and automated/synthetic browser acceptance verified; see external
completion receipts for final gate status.

Worktree: `/Users/hdang/.codex/worktrees/local-diagnostics/ai-agent-assistant`; branch `codex/local-diagnostics`.
HEAD `662fe1a57a1148a215beb04b215ad35681c4cbe6`. Exact scope was frozen before ordinary begin.

## Objective, invariants and risks

The official [streaming schema](https://developers.openai.com/api/reference/resources/responses/streaming-events)
defines top-level code and param as string or null. Parameter labels here are a
local allowlist, not an upstream taxonomy or proof of a cause. Distinguish absent
from null code while retaining legacy missing_code records without reinterpretation.
Classify param by exact model, reasoning, reasoning.effort, max_output_tokens and
service_tier literals, or absent/null/invalid/unknown. Empty/non-string is invalid;
other strings are unknown. No normalization, nested fallback, arbitrary string,
message, frame, credential or hash of excluded content is retained.

One guarded observer emits at most one code and one parameter observation for an
accepted top-level error. Retain the 14-field record, correlation, bounded logger,
request bytes, retry policy, response.failed handling and ownership cleanup.
No new IPC, storage fields, dependencies, permissions or execution authority.
Older binaries may reject new labels. Bounded logging can lose observations;
absence of a diagnostic is not proof of an absent provider field. No downgrade
compatibility or tamper-proof audit claim. Rollback needs explicit owner approval.

## Frozen scope

Exactly sixteen successor paths and thirty-six cumulative paths:

- `src-tauri/src/personal_assistant_direct.rs`
- `src-tauri/src/diagnostics.rs`
- `src-tauri/src/diagnostics/tests.rs`
- `src/infrastructure/tauri/diagnostics-client.ts`
- `src/infrastructure/tauri/diagnostics-client.test.ts`
- `src/features/settings/DiagnosticsPanel.test.tsx`
- `scripts/browser/diagnostics-fixture.html`
- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-10-02-openai-stream-error-shape-diagnostic.md`
- `docs/reviews/2026-10-02-openai-stream-error-shape-diagnostic-post-increment-review.md`

## Completion checklist and validation

- [x] Verify complete/valid predecessor, freeze scope and archive raw state before begin.
- [x] Implement bounded native categories and strict frontend vocabulary.
- [x] Focused Rust top_level tests and both affected frontend test files.
- [x] Full offline npm run verify using installed process-local tooling.
- [x] Synthetic Computer Use of actual DiagnosticsPanel: labels, correlation, severity and copy feedback.
- [x] Documentation/repository/security/whitespace and scope/preservation checks.
- [x] Architecture/security/code-health/debt/readiness and session/report review.
- [x] Freeze completion handoff; rely on external finalization/status/full-Stop receipts for actual gate results.

Regression matrix: every literal and absent/null/empty/non-string/unknown values;
case/whitespace variants, nested conflicts, arbitrary canaries, split/coalesced
frames, malformed framing and separate response.failed behavior. Verify no canary
or its hash reaches snapshots, disk, restart or export. Preserve old records.
Exercise duplicate/late/other-provider suppression and unavailable logging.

The synthetic browser fixture injects a fixed parsed snapshot through the existing
DiagnosticsPanel client. No native IPC, owner data, network request to a provider or
real file export. Computer Use observes current UI rather than substituting tests.
Stop the task-owned preview after acceptance. Preserve old bundles; native/live
verification remains outside scope and requires later separate authorization.

## Stop conditions and handoff

Continue recoverable in-scope repair. Stop for conflicting drift, rejected admission,
unsupported access, scope expansion or unresolved security failure. Preserve work
on failure. Exact next action is ordinary finalization/status/Stop and freezing the completed
candidate; only later owner approval may authorize a new bundle or live QA. Do not infer historical cause or live cancellation.
