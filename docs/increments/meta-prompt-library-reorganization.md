# Meta Increment 8 - Prompt Library Reorganization

Status: Verified complete; uncommitted and unpublished
Owner: Project maintainer
Date: 2026-07-17
Gate ID: `meta-prompt-library-reorganization`

## Goal

Replace the flat copy-and-paste prompt collection with one discoverable,
version-controlled library organized by increment, review, workflow, and
authoring-template responsibility while preserving useful content and all
repository approval boundaries.

## Previous structure

The previous `prompts/` root contained one short README and 15 flat prompt
files. Implementation, review, troubleshooting, session, documentation, and
post-increment instructions shared one namespace. Files had a title and prompt
body but no standard purpose, input, output, selection, relationship, or review
metadata.

## New structure

```text
prompts/
  README.md
  increments/   # six bounded implementation modes
  reviews/      # eight non-mutating assessments
  workflows/    # six multi-step operating procedures
  templates/    # three prompt-authoring skeletons
```

`prompts/README.md` defines selection, placeholder, naming, versioning, and
review rules. Each of the 23 executable or authoring-template prompt files uses
the D-055 Markdown metadata contract.

## File migration map

| Previous path                         | New path                                   | Disposition                                     |
| ------------------------------------- | ------------------------------------------ | ----------------------------------------------- |
| `prompts/architecture-review.md`      | `prompts/reviews/architecture-review.md`   | Moved and standardized                          |
| `prompts/end-of-session-handoff.md`   | `prompts/workflows/end-session.md`         | Moved; post-increment content merged            |
| `prompts/executive-review.md`         | `prompts/reviews/executive-review.md`      | Moved and standardized                          |
| `prompts/implement-next-increment.md` | `prompts/increments/verified-increment.md` | Moved, renamed, and strengthened                |
| `prompts/post-increment-gate.md`      | `prompts/workflows/end-session.md`         | Merged and superseded                           |
| `prompts/quality-gate.md`             | `prompts/reviews/quality-gate.md`          | Moved and standardized                          |
| `prompts/readiness-review.md`         | `prompts/reviews/readiness-review.md`      | Moved and standardized                          |
| `prompts/release-review.md`           | `prompts/reviews/release-review.md`        | Moved and standardized                          |
| `prompts/resume-work.md`              | `prompts/workflows/start-session.md`       | Merged and superseded                           |
| `prompts/review-change.md`            | `prompts/reviews/code-review.md`           | Moved and renamed                               |
| `prompts/security-review.md`          | `prompts/reviews/security-review.md`       | Moved and standardized                          |
| `prompts/start-work.md`               | `prompts/workflows/start-session.md`       | Moved; resume mode merged                       |
| `prompts/technical-debt-review.md`    | `prompts/reviews/technical-debt.md`        | Moved and renamed                               |
| `prompts/troubleshooting.md`          | `prompts/increments/bug-fix.md`            | Moved and expanded into bounded correction flow |
| `prompts/update-project-memory.md`    | `prompts/workflows/documentation.md`       | Moved and renamed                               |

`prompts/README.md` remains in place and is expanded into the authoritative
selection and contribution guide.

## Files created

- `prompts/increments/feature-implementation.md`
- `prompts/increments/remediation-by-severity.md`
- `prompts/increments/remediation-single-advisory.md`
- `prompts/increments/refactor.md`
- `prompts/workflows/remediation.md`
- `prompts/workflows/release.md`
- `prompts/workflows/repository-health.md`
- `prompts/templates/increment-template.md`
- `prompts/templates/review-template.md`
- `prompts/templates/remediation-template.md`

## Files merged

- `prompts/resume-work.md` is merged into the explicit start/resume modes in
  `prompts/workflows/start-session.md`.
- `prompts/post-increment-gate.md` is merged into
  `prompts/workflows/end-session.md`, which retains the separate quality and
  post-increment skill invocations.

## Files superseded or removed

Only the two merged source paths above are removed. Their useful instructions
are retained in the destination workflows, their original commits remain in Git
history, and this map records the supersession. No prompt is removed as obsolete
without migration.

## Duplicate content eliminated

- Session orientation and resumption share one mode-aware prompt instead of two
  copies of the reading, Git, baseline, and scope procedure.
- Session closeout owns report and marker coordination rather than duplicating a
  second standalone post-increment prompt.
- Project-wide security and engineering rules are referenced from authoritative
  files instead of copied into every prompt.
- Remediation workflow selection is distinct from the single-advisory and
  severity-based implementation authorization prompts.
- Release workflow coordination is distinct from the non-mutating release
  review.

## Broken links repaired

Active references in repository governance, assistant usage, code review, and
session/troubleshooting runbooks point to the categorized library. Searches may
still find old paths in dated plans, reports, backups, and historical handoff
file inventories; those references are intentionally preserved as evidence and
are not active instructions.

## Scope and non-goals

This increment changes prompt Markdown, active governance and workflow
references, current project memory, D-055, this migration record, and the
mandatory review report. It adds no parser or dependency.

No application source, Tauri configuration, IPC, capability, permission, CSP,
SQLite schema, manifest, lockfile, skill, hook, product behavior, credential,
network, release, commit, push, merge, or later increment is in scope.

## Risks and controls

- Active links could retain an old flat path; full-repository searches and the
  link checker distinguish active guidance from historical evidence.
- Similar prompts could create ambiguous authorization; selection guidance and
  root-cause-specific responsibilities keep each prompt distinct.
- A prompt could imply approval or publication authority; every mutating prompt
  pauses before editing and treats Git publication as separately approved.
- Prompt and document templates could be confused; README and D-055 define
  their separate ownership.

## Verification plan

- Confirm the exact 24-file target tree and no flat executable prompt.
- Confirm all 23 prompt/template files contain the complete metadata contract.
- Confirm the required severity and single-advisory placeholders.
- Search the entire repository for every old path and classify remaining
  references as historical.
- Verify all Markdown links and referenced paths.
- Review prompt purposes and shared text for substantive duplication.
- Run `npm run docs:check`, `npm run repository:check`, and `npm run verify`.
- Run `python3 .codex/hooks/session_end_gate.py`, complete review, and finalize
  the mandatory post-increment marker.
- Confirm no protected application, Tauri, manifest, lockfile, permission, CSP,
  SQLite, skill, or hook path changed.

## Rollback

Before commit, restore the prompt and documentation paths from `HEAD`. After a
future commit, revert only the bounded prompt-library commit. No runtime,
database, dependency, capability, or migration rollback is required.

## Actual results

- The final library contains exactly 24 files: one authoritative README and 23
  executable or authoring-template prompts under the four approved categories.
- Thirteen useful flat prompts were moved and standardized. Two overlapping
  prompts were merged into the start-session and end-session workflows, with
  all useful content retained and supersession recorded above.
- Every prompt asset has the complete D-055 metadata contract. The severity and
  single-advisory prompts contain every required placeholder, and
  `prompts/README.md` documents replacement rules.
- Active prompt links now use categorized paths. Remaining old-path references
  are dated historical evidence in completed plans, reviews, backups, increment
  records, or older handoff entries.
- Exact-tree, metadata, placeholder, old-reference, and substantive-duplication
  checks passed. The highest pairwise similarity was `0.408`, limited to the
  intentionally related authoring templates.
- `npm run docs:check`, `npm run repository:check`, and `npm run verify` passed.
  Complete verification included 28 hook tests, 19 repository tests, 124
  frontend tests, 95 Rust library tests, 21 Rust integration tests, strict
  Clippy, type checking, production frontend builds, and the Tauri release
  no-bundle build.
- The complete diff contains no application source, Tauri configuration,
  manifest, lockfile, SQLite, permission, CSP, dependency, skill, or hook
  change. No manual verification was required for this documentation-only
  increment.
- The consolidated review result is `PASS`. The
  `meta-prompt-library-reorganization` completion marker is complete and valid.
  Commit, push, merge, and the next increment were not started.

## Remaining recommendations

Do not create a new prompt unless an existing prompt, placeholder, or
cross-reference cannot express the responsibility. Re-review metadata and
active paths whenever an authoritative workflow or skill changes.
