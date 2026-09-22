# Configurable agent current-model correction

Date: 2026-09-22. Increment: `configurable-agent-current-models`.

## Goal and authority

The owner reported that the configurable demo's GPT-5.4 choices are no longer
available and directed Cortexa to use only 5.6 or 6 models. This bounded
successor replaces the selectable catalog with the documented GPT-5.6 Luna,
Terra and Sol models, using Luna as the default. GPT-6 Astra is not added because
the existing shared control offers `none`, which Astra does not document; adding
model-specific effort controls would exceed this corrective scope.

## Baseline and preservation

Work continues in the completed 36-path configurable-agent candidate at HEAD
`d9b09255f4c95a80e275a15714fbb6c509673f2c`. Its valid completion state was
archived externally before ordinary admission. The predecessor plan and report
remain byte-identical. The optional OpenAI verification consumed its single
request and ended with the sanitized `model_unavailable` category, no completed
answer and no retry. Zero live requests remain authorized.

## Exact scope and non-goals

The successor changes exactly 21 paths: six current-state documents, this plan
and its review, the typed client and UI with focused tests, native preferences,
chat and command-boundary tests, one inert configured-request fixture, migration
and startup/storage tests. The resulting cumulative candidate has 38 paths.

No provider request, credential inspection, dependency, endpoint, transport,
stream parser, retry/fallback, tool authority, permission, workflow, hook,
skill, harness, native launch, build publication, commit or push is in scope.
D-125/M1/M2 remain parked.

## Design and invariants

Both TypeScript and Rust accept only `gpt-5.6-luna`, `gpt-5.6-terra` and
`gpt-5.6-sol`; Luna is the first/default choice. Existing reasoning efforts and
request semantics stay unchanged. Append-only migration 4 converts only stored
OpenAI profiles using `gpt-5.4-mini` or `gpt-5.4-nano` to Luna and increments
their revision. It preserves effort, owner instructions, memory mode and note;
the revision change invalidates any stale in-memory conversation. Simulation
and Codex rows are not changed. A revision already at SQLite's signed maximum
stays saturated while its removed model is replaced; the closed validator still
prevents its captured 5.4 conversation from sending.

## Validation and stop conditions

Run focused frontend, native catalog, request, migration and storage checks,
then one complete offline `npm run verify`. Run documentation, repository,
security, whitespace, exact-scope and predecessor-preservation checks, followed
by session, quality and post-increment workflows. Stop on an unsupported model
contract, migration data loss, scope expansion, download, sensitive output or
failed required check. No live verification is available because the authorized
allowance is exhausted.

## Progress and results

Ordinary admission passed. The closed 5.6 catalog and append-only migration are
implemented. Focused frontend, migration, complete Rust library and storage
integration checks passed. Independent review found and corrected a signed-
revision overflow edge by saturating the maximum revision; its removed model is
still rejected by stale captured-profile validation. The final complete offline
`npm run verify` passed 74 hook tests, 85 repository tests, 412 frontend tests,
337 Rust library tests, the full integration suite, strict lint/type checks,
frontend builds and the release Tauri build. Documentation and completion gates
passed. Completion result: `PASS WITH ADVISORIES`.
