# Knowledge & Documents v1

Increment: knowledge-documents
Status: Active — implementation and focused verification

## Authority, baseline and preservation

Owner authorizes one end-to-end local document milestone, native selection/read/export, persistent library and source integration, isolated Simulation/browser/native QA. Worktree: `/Users/hdang/.codex/worktrees/knowledge-documents/ai-agent-assistant`; branch `codex/knowledge-documents`; baseline `263f879c05155c960f0121dbd8c67df3066a0b4c`. Started clean; ordinary admission passed. Other worktrees, owner profiles/rooms and historical records remain untouched. Evidence: `/private/tmp/cortexa-knowledge-documents-evidence`.

## Complete acceptance checklist

- [ ] Bounded native file picker ingestion, unsupported/encoding/link/race/duplicate errors.
- [ ] Compatible migration, immutable versions/hashes, restart persistence and owner note editing.
- [ ] Knowledge navigation, list/details, safe literal Markdown preview, clear retention and limits.
- [ ] Deterministic line/chunk passages and bounded keyword search with honest empty results.
- [ ] Explicit selections, Rust-owned source resolution and preview invalidation; all four routes unchanged.
- [ ] Historical references inspect exact frozen passages after library edits/removal; graph links narrow.
- [ ] Reviewed editable generated drafts, readable attribution, explicit non-overwriting Markdown export.
- [ ] Focused security/storage/migration/reference/UI regressions and complete required offline checks.
- [ ] Browser visual/interactive QA, then isolated native import → search → operations Simulation → references → draft → restart → export.
- [ ] Final reviews, documentation/preservation/schema, ordinary finalization and full Stop.

## Boundaries and design

Library is separate from workflow-local documents.rs/memory.rs and private bot notes. Reuse safe selected-file validation without exposing arbitrary WebView paths. Native dialogs grant only one explicit operation. No vault scanning, linked-file resolution, watches, embeddings, providers, tools or graph redesign. Imported content remains literal UTF-8 snapshots; safe preview is text, not an HTML renderer. Export uses native destination selection and exclusive creation; existing files are never overwritten.

Personal-demo bounds: 16 KiB/version, 200 library items, eight immutable versions/item, 4 MiB retained library text. Deterministic passages at line boundaries with bounded long-line chunks, stable IDs/locations and content hashes. No source-path persistence or provider disclosure. Search scans bounded retained current versions locally. Run evidence retains only selected content under existing six-source, 4096-character/source and 16-KiB aggregate limits. Removing an item prevents future selection; existing room snapshots survive until explicit room deletion. No encryption-at-rest claim.

Native resolves IDs into exact version/passage/title/hash/text, rejecting stale content at prepare/start. Existing source labels and validator bind model references. Provenance is not factual support. Owner-authored and generated-draft kinds remain distinct; reviewed edits do not automatically grant trust.

## Expected files and risks

Native: new knowledge domain/storage/Tauri modules and tests, documents.rs reusable selected reader, appended storage migration and associated count tests, lib.rs command registration, collaboration source metadata/prepare validation. SHA-256 reuses the already locked sha2 0.10.9 library and rustix 1.1.4 descriptor-relative no-follow opens as two direct pinned dependencies; Cargo manifests/lock change only those dependency edges. No capability widening or new generic filesystem command.

Frontend: typed knowledge client, Knowledge page/style/tests, SourcePicker and evidence/draft components, collaboration client/page/tests, App/navigation and narrow operational inspector integration/tests. Browser: synthetic knowledge fixture/check added to existing harness. Documentation: this plan/review, HANDOFF, PROJECT_STATUS, NEXT_STEPS, PLANS, CHANGELOG, TROUBLESHOOTING_LOG, TESTING_GUIDE, ARCHITECTURE, SECURITY, DECISIONS. Freeze actual scope before completion; no unrelated paths.

Key risks: stale preview, file replacement/link races, Markdown injection, quotas, migration compatibility, historical-reference loss. Focused regressions cover each. File dialogs run on the supported native boundary; unavailable platform access remains explicit.

## Validation and checkpoints

Focused Rust/frontend tests, formatting, strict lint/Clippy/typecheck, complete offline npm run verify with process-local Python 3.12/Xcode SDK27/Cargo stripping override, audits/security/docs/repository/scope checks. Native app uses separate identifier and synthetic documents/vault; never owner data or credentials. Preserve old artifacts. Computer Use required when supported; distinguish automated fixtures from native observations and live-provider behavior.

Current checkpoint: admitted from a clean baseline; native versioned library, dialog ingestion/export, source binding, search, draft and workspace integration implemented. Initial offline native cargo check and frontend typecheck passed. Focused security/storage/UI regressions and visual/native QA are next; completion is not yet claimed. Continue in this increment through recoverable failures. No commits, pushes, publication or provider calls. OpenAI parked4/5; D-127/D-128 and native/runtime/Codex advisories retained; D-125/M1/M2 parked.

## Checkpoint — native import-to-reuse observed

Initial offline native compile passed; 42 focused Rust tests and 22 focused frontend tests passed. Browser Playwright passed empty/import/search/selection/draft/export-conflict/loading/long-Unicode at 1280 and 760 widths. Strict frontend typing/lint and native Clippy passed after routine test/helper corrections; final acceptance follows the last edits.

Direct supported Computer Use in the separately identified `com.cortexa.qa.knowledge20261001` debug app imported synthetic Runbook.md and Incident.txt, found both availability passages, explicitly selected them, acknowledged five Simulation stages for Operations, observed completed synthesis and released controls, opened room and graph historical references, reviewed/saved a draft, quit/reopened, searched the retained draft, and exported it. Read-only synthetic verification confirms exact exported/saved bytes. No provider or owner data was used. Native QA receipt: `/private/tmp/cortexa-knowledge-documents-evidence/native-observations.json`. Original bundle receipt retained at artifact.json. Fixing observed clipped navigation label and adding focused draft-editor/save confirmation; no execution changes.

Current branch and HEAD unchanged. Newly introduced changes are only this milestone's declared native/UI/tests/browser/config/docs paths; there were no inherited edits in this worktree. TypeScript config adds only the new browser fixture to strict checking. Next: affected regressions, required full offline verification, revised isolated bundle/presentation recheck, security/architecture/code reviews, documentation/preservation/schema and completion gates. Live-provider and remote CI remain unverified; owner QA follows acceptance.

## Boundary-check checkpoint

Full verification caught the repository's exact IPC allowlist omissions before tests/build. The necessary two-file repository checker/test scope is explicitly frozen: `scripts/repository_health.py` and `scripts/tests/test_repository_health.py`. Add only the nine fixed Knowledge commands/import/payloads; preserve all existing allowlists and rejection logic. Replace the new client's generic command parameter with nine literal calls and add arbitrary-command/path-payload rejection regressions. This supports the approved native library boundary; it does not weaken the gate or grant generic file access. Focused v7-to-v8 migration preservation passed.

## Integration checkpoint

Full acceptance passed 516 frontend tests, 74 hook tests, 87 repository tests and 408 native library tests, then found two existing storage smoke modules still expecting seven migrations. Correct only their version-list expectations for the appended eighth migration (`src-tauri/tests/startup_storage_smoke.rs` and `src-tauri/tests/storage_smoke.rs`); preserve restart/bootstrap assertions. This is the planned associated migration-test scope, now 45 total paths. Re-run the two smoke modules and complete verification; no unrelated repairs.

## Final error-path review

The new library stale-selection path originally reused generic conversation guidance and left an outdated preview visible. The existing collaboration page now clears preview/acknowledgement on native `stale_context`, directs explicit current-source reselection, and never retries or substitutes content. Bounded-limit guidance now names room/run/source limits. A focused regression proves acknowledgement resets and exactly one failed Start is attempted. Twenty-three affected frontend tests pass. No new path or native authority change; complete final acceptance still follows.

Platform review scoped file-dialog-only closed error variants to macOS (or export-test builds where needed), preserving strict Linux linting without a blanket allowance. Native unavailable behavior on other platforms remains explicit. The final source freeze supersedes the earlier pre-portability freeze; remote Linux execution still requires future CI.

## 2026-10-01 — Knowledge acceptance checkpoint; native access blocked

This additive checkpoint supersedes earlier in-progress verification statements
without removing any prior text. Worktree: `/Users/hdang/.codex/worktrees/knowledge-documents/ai-agent-assistant`;
branch `codex/knowledge-documents`; HEAD `263f879c05155c960f0121dbd8c67df3066a0b4c`.
Exact current scope: 45 uncommitted milestone paths; no inherited edits.

Implementation and automated acceptance passed: 517 frontend tests, 74 hook tests,
87 repository tests, 408 Rust library tests (also run by the integration command),
255 other native tests, strict formatting/lint/typecheck/Clippy, release build and
isolated unsigned debug bundle. One opt-in real-Hermes test is ignored, not passed.
Both npm audits found zero vulnerabilities; pinned cargo-audit 0.22.2 retains its
known raw exit 1 baseline, and the unchanged repository Rust audit gate passed.
All package versions remain unchanged; only two already-locked direct Rust edges
were added. Browser matrix and direct Computer Use inspected affected surfaces,
including loading/error/long Unicode/narrow layout. All previous in-task failures
were repaired without a successor or historical-record rewrite.

The full isolated native import-to-reuse scenario passed: two selected synthetic
files, search/passages, acknowledged five-stage Operations Simulation, room/graph
historical references, reviewed draft save, restart/search and byte-exact export.
The final bundle also directly showed the unclipped Knowledge navigation, retained
three-item library/completed room and visible focused draft review. The Mac then
locked during draft editing, before any Save click. Final save-feedback observation
and graceful test-app cleanup are pending, not passed. No provider request occurred.
The QA Vite server was stopped. Do not infer a product defect from unavailable access.

Evidence: `/private/tmp/cortexa-knowledge-documents-evidence`; `artifact-final.json`
identifies the final bundle and frozen source. The original bundle is preserved at
`initial-native-bundle.app`. Native prior observations, browser screenshots, failed
and passing logs remain. Formal acceptance stays active until required cleanup and
ordinary gates pass; do not close/promote/reopen a historical terminal record.

Exact next action: after the owner unlocks the Mac, verify the already-running
isolated Cortexa Knowledge QA executable against artifact-final.json. Resume only
reviewed synthetic draft save-feedback, then quit that test-owned app. Do not
rebuild, repeat workflows/passing tests, touch owner data or make provider calls.
Update this same active plan/review truthfully; run documentation, repository,
security, whitespace, preservation and report-schema checks, then ordinary
finalization/status/full Stop only when all required evidence passes. Preserve
all 45 paths, historical bodies, prior artifacts, other worktrees and prunable
entries. No commit, push or publication. OpenAI remains parked 4/5; retain
D-127/D-128, native/provider/runtime and Codex-isolation advisories, process-local
Python 3.12.1/Xcode SDK27/Cargo stripping workaround, and parked D-125/M1/M2.

## 2026-10-01 — Knowledge native acceptance resumed and verified

This additive result supersedes the earlier locked-Mac checkpoint, which remains
unchanged. Worktree `/Users/hdang/.codex/worktrees/knowledge-documents/ai-agent-assistant`,
branch `codex/knowledge-documents`, HEAD `263f879c05155c960f0121dbd8c67df3066a0b4c`:
exactly 45 uncommitted paths. No product/test/build bytes changed in this resumption.

After owner unlock and confirmation that the QA window was idle, supported
Computer Use opened the existing completed synthetic Operations room, reviewed a
generated draft, changed only its synthetic title/heading while retaining source
attribution, and clicked Save once. The visible result was: “Draft saved to
Knowledge. It is not automatically shared.” Command-Q gracefully quit the isolated
QA app; the recorded process was confirmed absent. No workflow was rerun, no
provider request was made and no owner profiles, notes or rooms were touched.
Receipt: `/private/tmp/cortexa-knowledge-documents-evidence/native-resume-observations.json`.
Both prior native receipts, the interrupted draft report and both artifacts remain.

All implementation checklist items and required local native observations now
have passing evidence. Reuse the source-bound full offline verification, strict
Clippy, native builds, focused regressions, browser matrix and audits explicitly;
these passing checks were not rerun in this resumption. Documentation, repository,
security, whitespace, preservation, session inventory and report schema are the
remaining closeout routes. The consolidated review records PASS WITH ADVISORIES;
formal completion requires ordinary finalization, complete/valid status and full
Stop. Gate receipts in the evidence directory are authoritative for actual status.

Observed visual advisory: the generic workspace inspector overlaid right-side
room content in the final native window. Save feedback was visible and the scoped
save/cleanup check passed; no unrelated layout repair is claimed. Owner QA should
check inspector-open/closed readability at the intended window size. Retain
D-127/D-128, native/provider/runtime and Codex-isolation advisories, Python 3.12.1,
Xcode SDK27/Cargo stripping workaround, OpenAI parked 4/5 and D-125/M1/M2 parked.
Remote CI/Linux execution, live providers and owner manual QA are not verified.

### Exact next prompt

```text
Assist owner manual QA of Knowledge & Documents in
/Users/hdang/.codex/worktrees/knowledge-documents/ai-agent-assistant.
Inspect instructions, the knowledge-documents plan/review, complete/valid status,
artifact-final.json and native-resume-observations.json in
/private/tmp/cortexa-knowledge-documents-evidence first. Preserve all 45 paths,
historical records and both artifacts. Do not repeat passed builds/tests or the
completed native Simulation scenario. Use the existing isolated synthetic bundle
and supported Computer Use; confirm the intended owner-QA actions before changing
local test data. Check library editing/versioning, source selection and historical
references, Markdown portability/export conflicts, and inspector-open/closed
readability. No owner vault/profile/room changes, live requests or publication.
Stop on drift, unsupported access or necessary scope expansion. Report only
observed results, retain all advisories, OpenAI parked 4/5 and D-125/M1/M2 parked.
```
