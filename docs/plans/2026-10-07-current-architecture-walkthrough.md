# Current architecture and execution walkthrough

Date: 2026-10-07
Increment: `current-architecture-walkthrough`
Status: Artifact set delivered; ordinary final review/completion receipts govern acceptance.

## Goal and evidence boundary

Document the actual implemented product at `bc12776412c717613de1fdc42c38bf3312493726`
on `codex/live-provider-qa`; its tree equals merged main
`bbe7546281fec3c8b4b608d68a52bc9732d9ecd9`. The checkout began clean.
The documentation will separate implemented/verified, implemented without native
verification, partial, deferred and uncertain behavior. Read source rather than
assuming older plans reflect current wiring. No product, dependency, permission,
credential, profile or QA changes, live calls, commits or publication.

## Components and deliverables

Nine deterministic SVG diagrams with stable component identifiers; editable JSON
source and Python standard-library renderer; standalone offline HTML navigation
and zoom; PDF overview using installed Chromium if supported. Index, bot/runtime
capability inventory, concrete walkthrough, security evidence and external-project
adoption tables. Every diagram includes source-symbol and retained-test evidence.
Use existing Cortexa palette and canonical logo without modifying the image.

## Scope

The exact documentation-only allowlist is below. No other tracked path may change.
Prior completion was archived byte-identically before ordinary begin in external
evidence `/private/tmp/cortexa-architecture-walkthrough-8sim6hio`.

- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/architecture/current/01-system.svg`
- `docs/architecture/current/02-sequence.svg`
- `docs/architecture/current/03-collaboration.svg`
- `docs/architecture/current/04-context.svg`
- `docs/architecture/current/05-lifecycle.svg`
- `docs/architecture/current/06-observability.svg`
- `docs/architecture/current/07-security.svg`
- `docs/architecture/current/08-delivery.svg`
- `docs/architecture/current/09-future.svg`
- `docs/architecture/current/README.md`
- `docs/architecture/current/capabilities.md`
- `docs/architecture/current/diagrams.json`
- `docs/architecture/current/external-projects.md`
- `docs/architecture/current/overview.pdf`
- `docs/architecture/current/render_diagrams.py`
- `docs/architecture/current/security.md`
- `docs/architecture/current/sources.json`
- `docs/architecture/current/viewer.html`
- `docs/architecture/current/walkthrough.md`
- `docs/plans/2026-10-07-current-architecture-walkthrough.md`
- `docs/reviews/2026-10-07-current-architecture-walkthrough-post-increment-review.md`

## Risks and validation

Primary risk is presenting a fixture or advisory role as an enforced production
capability. Verify runtime entry points, IPC list, data ownership and actual
provider request builders. Retained native evidence is not new QA. Do not inspect
credentials, personal databases or raw provider logs. No security-audit claim.

Read-only baseline; ordinary gate admission; product-byte and historical-document
preservation; exact-scope check; source/symbol references; deterministic renderer;
offline HTML/PDF generation; screenshot inspection for clipping/overlap/labels;
viewer navigation/zoom checks; `npm run docs:check`, `npm run repository:check`,
`npm run security:scan`, `git diff --check`; session/report schema; reviews;
ordinary finalization, complete/valid status and complete-payload Stop.
No broad product builds/tests. Missing optional PDF tooling must be disclosed,
not replaced with an unapproved install. Keep ledger33, all advisories, disabled
ECC hooks/MCP, native workaround and D-125/M1/M2 parked.

## Progress

- Read current instructions and relevant handoffs; checked clean branch/state.
- Preserved predecessor completion and all tracked-file hashes externally.
- Ordinary begin accepted for this documentation increment.
- Tracing provider, context, collaboration, Knowledge, security and graph sources.

## Recovery

No deletion or automatic rollback. Retain the pre-edit baseline hashes and original
root documents. Any future reversal is limited to this new documentation and its
additive current-state entries, under owner direction. Historical gate evidence
is immutable; the ordinary live gate transition is recorded separately.

## Completed artifact and review checkpoint

Source tracing covers all nine roles, six connections, context and state ownership,
Knowledge, collaboration, graphs, deterministic security and development paths.
Nine SVGs, standalone HTML, editable JSON/renderer, 57-source hash register and
10-page PDF are rendered. Visual review found no clipped node text or misleading
crossing arrows. Refined labels distinguish comparisons from data flow and restart
interruption from already terminal runs. Browser navigation/zoom and offline-only
loading passed. Retained product verification and native/owner evidence were reused.

Architecture/security/code-health/debt/readiness review is recorded in the scoped
post-increment report. Required documentation, preservation, session/report and
ordinary completion/Stop checks are recorded externally with actual exit status.
No product repair or broader runtime acceptance follows from this document.
