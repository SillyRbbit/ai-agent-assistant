# Connected Knowledge Workspace

Status: PASS WITH ADVISORIES — implementation and required local validation complete;
ordinary completion validity is established by status/full Stop receipts.
Owner: Henry

## Goal and boundaries

Complete the owner-authorized Connected Knowledge milestone: safe Markdown reading,
explicit editing, stable internal links/backlinks, frontmatter properties, five
templates, and an isolated local graph. Preserve explicit workflow sources and
immutable historical evidence. No providers, vault crawling, watches, tools,
private-note sharing, operational graph redesign, commits or publication.

## Workspace and baseline

Worktree: `/Users/hdang/.codex/worktrees/connected-knowledge-workspace/ai-agent-assistant`.
Branch: `codex/connected-knowledge-workspace`.
Baseline: `cc8b1b4e21b765b3af5bd5bdf5cf7fc91e021324` (verified live main).
Fresh checkout, no inherited dirty files or gate state. All older checkouts remain.
Desktop branding/general closure is intentionally excluded. No other active
session was found targeting this worktree. Ordinary begin succeeded.

Browser preview: `npm run dev -- --host 127.0.0.1` from this worktree,
then http://127.0.0.1:1420. Native QA uses a separate identifier/test data directory.

## Completion checklist

- [x] Baseline focused checks and installed tooling
- [x] Markdown React rendering without HTML injection or remote resources
- [x] Explicit edit/read/save, unsaved navigation and close protection
- [x] Stable saved link binding; code exclusion; ambiguity, rename and removal
- [x] Backlinks/context and keyboard link autocomplete
- [x] Canonical frontmatter properties with lossless unsupported-content handling
- [x] Five templates including exact ordered Change request headings
- [x] Bounded Knowledge-only graph and accessible linked-note list
- [x] Additive migration, restart, immutable version/source preservation
- [x] Focused regression tests and existing layout coverage
- [x] Full verify, security/repository/docs/whitespace and native packaging
- [x] Browser Computer Use and isolated native end-to-end scenario
- [x] Evidence review and frozen report; ordinary finalization/status/full Stop receipts required

## Expected scope

Knowledge domain/storage/migration and typed client; Knowledge components/styles;
shared unsaved-navigation guard at application dispatch; App/provider integration;
focused native/frontend/integration and browser tests; package metadata only for
one Markdown parser; existing current-state/security/architecture/testing records;
this plan and its post-increment review. No capabilities/CSP/governance changes.

## Design and invariants

Use existing immutable versions and optimistic concurrency. Persist stable link
bindings separately from original Markdown, without rewriting prior versions.
Bind unambiguous exact titles; preserve established targets across renames; missing
or ambiguous links are explicit. Graph/backlinks consume the same bound results.
Metadata remains in original Markdown frontmatter, never a competing property store.
Unsupported/malformed frontmatter remains editable as raw source; do not normalize it.
Rendering uses a maintained lexer and React elements, no innerHTML. Images and
external links remain inert text. No remote loads or filesystem link resolution.
Limits (16 KiB/version, 200 items, 8 versions, 4 MiB) remain unchanged.

## Dependency assessment

Proposed sole runtime dependency: marked 18.0.14, MIT, Node >=20 (official npm
metadata inspected). Use its lexer only; HTML tokens become escaped text and no
raw HTML renderer is used. Exact pin and lock integrity required. Audit before
acceptance. YAML subset is deliberately non-executable, line-based and bounded.

## Validation and risks

Focused native storage/link/migration and frontend rendering/editor/graph tests;
strict format/lint/typecheck, existing browser layout harness, full npm run verify,
npm audits, repository/security/docs/whitespace and ordinary completion gates.
Native scenarios must use synthetic data and preserve previous artifacts; Computer
Use unavailability is reported separately. Main risks: link rebinding, stale saves,
unsaved route loss, malformed metadata corruption and graph/layout regressions.
No destructive rollback: preserve failed output and repair within this milestone.

## Progress and results

Ordinary admission succeeded on the isolated baseline. All implementation is local
and uncommitted. Migration 9 adds a binding sidecar and leaves migration 1–8/raw
version/history bytes unchanged. Only marked 18.0.14 was added; other lock entries
are preserved. No new IPC, capabilities, CSP, provider or graph authority.

Focused frontend 22 tests and native 46 tests passed; final full frontend 531 tests.
Full offline verify passed once before final UI corrections and was rerun for the
final implementation. Exact final exit/results are in the consolidated review.
Existing actual-shell browser matrix and new connected graph/editor matrix passed.
Both npm audits reported zero findings. Native unsigned isolated bundles built offline.

Direct Computer Use completed the synthetic incident/runbook/research scenario,
rename/restart, properties, keyboard links, backlinks/graph, explicit source preview,
one all-Simulation Research run, reviewed linked synthesis draft, historical source
preservation after edit and export/no-overwrite. Final receipt:
`/private/tmp/cortexa-connected-knowledge-evidence/native-observations-final.json`.
Artifact: `artifact-accepted.json` in the same directory. Prior bundles retained.
QA processes quit. No provider requests, owner data or profile changes.

Recovered defects: missing removal transaction commit; migration-count assertions;
case-insensitive module naming; Marked task marker rendering; React Flow measured
node readiness; native confirmation dialog; native SVG button padding. The test
assertion for icon size remains unchanged. A rejected proposal to change that
assertion was not executed; product SVG dimensions were fixed instead. An optional
native-window listener was removed after its boundary allowlist amendment was
rejected; gates/capabilities remain unchanged. Native application Quit protection is
not claimed; editor/note/route guards are implemented and directly observed.

Native layout: 1440/960px observed; 760px verified in browser only because native
resize did not reach it. Streaming/cancellation/live behavior were not inferred from
this completed Simulation. Owner manual QA and exact-head remote CI remain separate.

## Exact next action

Required checks passed. Validate ordinary finalization/status/full Stop receipts,
then owner QA and read-only publication review only. No commit,
push, PR or merge under this authorization. Retain D-127/D-128, runtime/provider,
Codex-isolation and native-workaround advisories; OpenAI parked 4/5; D-125/M1/M2 parked.
