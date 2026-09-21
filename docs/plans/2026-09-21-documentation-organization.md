# Bounded documentation organization

Date: 2026-09-21. Increment: `documentation-organization`.
Baseline: `39947cf9be86f2e36a57946fa1ad7d966b94ce32`.
Workspace: isolated detached worktree; no branch or publication is created.

## Goal and scope

Make existing documentation easy to find through one concise index and move the
non-authoritative assistant usage guide into the existing workflow directory.
The owner authorizes safe organization and equivalent path updates, not an
application refactor, governance change, history rewrite or publication.
Ordinary admission passed before edits; no exception or recovery was needed.

Exact paths (both sides of the move count):

- `ASSISTANT_USAGE.md`
- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `README.md`
- `docs/README.md`
- `docs/plans/2026-09-21-documentation-organization.md`
- `docs/reviews/2026-09-21-documentation-organization-post-increment-review.md`
- `docs/workflows/ASSISTANT_USAGE.md`
- `docs/workflows/README.md`
- `scripts/repository_health.py`

## Inventory decisions

Move only `ASSISTANT_USAGE.md` to `docs/workflows/ASSISTANT_USAGE.md`, preserving
its bytes. Its sole active tooling consumer is `ACTIVE_PROMPT_REFERENCE_PATHS`
in `scripts/repository_health.py`; substitute only the pathname so the same
prompt validation continues. No Markdown/image links inside the guide require
rebasing; its code/path examples deliberately run from the repository root.
Add guide links in the existing workflow index and new documentation index.

Keep README, AGENTS, CONTRIBUTING and SECURITY at root for conventional discovery
and instruction scope. Keep architecture, requirements, engineering, testing,
review, security and release checklists at root because current instructions,
skills, truth/command checks and release workflows consume them. Keep HANDOFF,
PROJECT_STATUS, NEXT_STEPS, PLANS, ROADMAP, DECISIONS, CHANGELOG and
TROUBLESHOOTING_LOG as root project-memory/authority paths used by workflows,
gate inventories and historical evidence. The licensing equivalent is already
`docs/github/LICENSING.md`; leave it there. No nested AGENTS exists in this tree.
Do not relocate or edit skills, hooks, templates or governance instructions.

Remaining old guide-path references in dated plans, review manifests/commands
and backups describe the files at that time. Preserve them verbatim and explain
the move in the index; do not rewrite finalized evidence or keep a competing copy.
No uncertain duplicate is removed or treated as obsolete.

## Preservation and risks

All existing worktrees, the original seven unpublished commits and dirty changes,
valid reconciliation/closeout markers and terminal-failed dependency record stay
untouched. Isolate work from their overlapping project-memory edits. Keep all
pre-existing bytes in the five updated state documents and change no milestone.
D-125/M1/M2 remain parked; native GUI verification remains pending advisory.
Primary risks are broken paths, silently skipped checker coverage and rewritten
history. Exact path substitution, link checks and preserved-byte checks address
those risks without changing a gate rule or instruction precedence.

## Validation and stopping point

Run documentation formatting/links, repository checks, secret scanning,
whitespace, existing repository-health regressions and a temporary behavioral
probe proving stale prompt paths in the relocated guide are still rejected.
Inspect remaining former-path references, compare guide bytes, exact scope and
all predecessor snapshots. Apply session/quality/post-increment workflows and
validate the ordinary marker and Stop. No unrelated frontend/Rust suite, package
installation or live/native test is required for this guide/path-only change.

Stop on admission failure, failed required validation, lost historical evidence,
changed checker semantics or scope beyond these paths. Preserve edits on failure;
no reset, stash, gate exception, commit, push or publication. Do not automatically
execute follow-up work. The owner may review the uncommitted result separately.

## Progress and results

- Root Markdown, directories/indexes and all text consumers inventoried.
- Required root and tool-specific scopes retained; exact 13-path scope frozen.
- Isolated from verified main; ordinary admission passed.
- Guide bytes, unchanged checker semantics, historical preservation and exact
  scope verified; documentation, repository, secret and whitespace checks passed.
- All 51 repository-health regressions passed. A temporary probe initially lacked
  a reusable prompt and correctly failed; after fixing only that fixture, valid
  references passed and stale references in the moved guide were rejected.
- Quality review: PASS WITH ADVISORIES for the unchanged native GUI advisory.
  Ordinary finalization and Stop validate the final report; leave edits uncommitted.
