# Optional API status repair

Date: 2026-09-22
Increment: optional-api-status-repair

## Goal and evidence

The latest configured-agent native attempt returned `model_unavailable`. Existing Rust mapping collapsed HTTP 400, 403 and 404 into that category before reading a body, so this evidence does not identify a model, account or request cause. The owner authorizes one time-boxed optional API repair and at most one new acknowledged fixed-sample native request. Diagnosis selected this bounded status distinction; API success is not a prerequisite for independently verified Agents, private notes or Codex behavior.

## Scope and invariants

- `src-tauri/src/personal_assistant_direct.rs`
- `src/infrastructure/tauri/personal-assistant-direct-client.ts`
- `src/infrastructure/tauri/personal-assistant-direct-client.test.ts`
- `src/infrastructure/tauri/agent-chat-client.ts`
- `src/infrastructure/tauri/agent-chat-client.test.ts`
- `src/features/conversations/PersonalAssistantDirectDemo.test.tsx`
- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-09-22-optional-api-status-repair.md`
- `docs/reviews/2026-09-22-optional-api-status-repair-post-increment-review.md`

Preserve all inherited dirty work and historical records. Split only the three HTTP statuses into payload-free `http_bad_request`, `http_forbidden`, and `http_not_found` codes with static messages. Preserve legacy frontend error readability, all other mappings, model selection, request bytes, endpoint, validation, transport, custody, cancellation and no-retry behavior. No provider bodies, headers or raw errors are read for classification. No credentials are inspected. No dependencies, permissions, governance, graph work, new diagnostic framework, commit or publication.

## Validation and progress

Ordinary admission passed before edits. Required checks are focused frontend/Rust regressions, full offline `npm run verify`, an offline locked debug app-only no-sign build, documentation/repository/security/whitespace checks, independent quality review, preservation/scope checks, session/report/finalization/Stop gates. The focused checks, full offline verification and offline debug bundle build passed.
Focused counts were 38 frontend and 17 native; full counts were 421 frontend,
337 Rust library, 74 hook and 85 repository tests plus integration suites.
Documentation/repository/security/whitespace checks passed after correcting only
new-prefix formatting within this task. Independent review found no concrete
issue. Final report/schema/preservation and ordinary finalization checks remain
necessary before the completion marker; optional live verification remains
pending owner setup and acknowledgment at 0/1 used. UI regressions must retain streaming, terminal error, cleanup, partial-output and single-start assertions. The installed bundle was older than the current source; rebuild is necessary before any meaningful native retest.

One new live request is optional and requires the owner's private launch plus explicit acknowledgment of the fixed synthetic board-update sample and existing limits/disclosures. Use Conversations' fixed sample (gpt-5.6-luna,512 output-token limit), not arbitrary Agents chat. Count any attempted Start, stop on failure without retry, verify final state/output/cleanup if accessible, then stop the test process. Initial new-batch count: 0/1. Do not infer cause or success from credential activity.

## Stop, rollback and handoff

Stop on unresolved security/shared-build failure or scope expansion. Routine local assertions/formatting/report repairs stay in this task. Preserve work rather than reset. If the live result fails or remains unclear, leave OpenAI API unavailable/unverified and park the API lane. Retain D-127 audit debt, D-128 custody/abort limits, prior live failures and native live-success advisory; keep D-125/M1/M2 parked. No successor closeout chain. The final report will record actual tests and request count.
