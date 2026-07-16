# Meta Increment 7 - verified application icon rollout

Status: Verified complete with advisories; uncommitted and unpublished
Owner: Project maintainer
Last updated: 2026-07-16

## Goal

Generate the complete Tauri production icon family from the approved square
`assets/branding/app-icon-source.png`, replace only existing icon files, and
verify packaging and target-Mac presentation.

## User-visible outcome

Packaged and development builds use the official Cortexa identity in supported
application, Dock, window, menu, and operating-system icon contexts.

## Exact future source scope

```text
src-tauri/icons/128x128.png
src-tauri/icons/128x128@2x.png
src-tauri/icons/32x32.png
src-tauri/icons/Square107x107Logo.png
src-tauri/icons/Square142x142Logo.png
src-tauri/icons/Square150x150Logo.png
src-tauri/icons/Square284x284Logo.png
src-tauri/icons/Square30x30Logo.png
src-tauri/icons/Square310x310Logo.png
src-tauri/icons/Square44x44Logo.png
src-tauri/icons/Square71x71Logo.png
src-tauri/icons/Square89x89Logo.png
src-tauri/icons/StoreLogo.png
src-tauri/icons/icon.icns
src-tauri/icons/icon.ico
src-tauri/icons/icon.png
```

Closeout documentation is limited to `AGENTS.md`, `ARCHITECTURE.md`,
`CHANGELOG.md`, `HANDOFF.md`, `NEXT_STEPS.md`, `PLANS.md`, `PROJECT_STATUS.md`,
`ROADMAP.md`, `docs/github/MILESTONES.md`, this plan, a new Meta 7 increment
record, `docs/plans/README.md`, and one dated review. Change `DECISIONS.md` only
if implementation establishes a durable generation rule.

## Explicit non-goals

- Logo redesign, recolor, crop, transparency extraction, or alternate identity.
- Changes to the app-icon source, `tauri.conf.json`, manifests, lockfiles,
  bundle identifier, executable, database, GitHub URL, runtime, capabilities,
  entitlements, permissions, or CSP.
- UI, favicon, README, presentation, screenshot, or diagram changes.
- Increment 4V or other product capability work.

## Implementation outline

1. Confirm the baseline descends from published Meta 6 at `5281fac` and the
   dependency compatibility repair at `b298999`, plus the exact source hash and
   dimensions.
2. Begin mandatory `meta-07` gate state before icon edits.
3. Generate icons with the existing Tauri CLI in a controlled temporary path.
4. Review every format and copy only the exact 16 approved outputs.
5. Verify dimensions, PNG color/alpha, ICO/ICNS structure, references, and diff.
6. Run repository checks, target-Mac manual checks, and the mandatory gate.

## Risks

- Generation may crop, mask, upscale poorly, or leave stale cached icons.
- Development, bundle, Dock, menu, and Finder contexts can use different sizes.
- Generation could add unapproved mobile files or modify configuration.
- The protected opaque field may interact with platform masks; that is a
  verification concern, not permission to redesign the source.

## Verification

Verify exact inventory, dimensions, formats, representative rendered sizes,
decoded ICNS representation pixels per D-052, exact packaged-resource equality,
`git diff --check`, `npm run verify`, `npm audit --audit-level=low`, and a
bundled Tauri build. On the target Mac verify debug-bundled and packaged icons
through macOS application, Dock/switcher, menu, and Finder icon APIs in light
and dark appearances. D-051 records the project-owner-approved exception for
the generic icon macOS assigns to the raw unbundled `tauri dev` executable. The
`meta-07` marker must finish complete and valid.

## Rollback

Before commit, restore only the 16 icon files from repaired `b298999` and revert
declared closeout documentation. After commit, revert one bounded Meta 7 commit.
No migration, data, dependency, identifier, or remote resource requires
rollback.

## Acceptance criteria

- [x] Meta Increment 6 is verified complete and published at `5281fac`; its
      marker validity on that clean baseline is recorded before this planning
      change.
- [x] Separate project-owner approval is recorded before implementation.
- [x] Only the exact icon and declared closeout scope changes.
- [x] Every output derives from the approved app-icon source.
- [x] Formats, sizes, required app bundles, and target-Mac presentation checks
      pass with D-051's approved raw-development-icon advisory.
- [x] No source asset, config, identifier, runtime, dependency, or permission
      changes.
- [x] Complete diff and mandatory gate pass without a blocking finding.

## Renumbering record

Meta Increment 1 originally created this unimplemented plan as Meta Increment 2.
On 2026-07-15 the project owner assigned Meta Increment 2 to the engineering
operating system and directed that the icon rollout become Meta Increment 3.
Later that day, the owner assigned Meta Increment 3 to repository-local Codex
automation and directed that this unchanged icon rollout become Meta Increment 4.
On 2026-07-16 the owner selected repository health and GitHub hygiene as Meta
Increment 5 after stopping an unimplemented Meta Increment 4 executive-document
request. The unchanged icon rollout therefore becomes Meta Increment 6. D-044,
D-045, and D-048 record those renumberings. Later that day, D-049 assigned Meta
Increment 6 to the documentation-only Product Readiness Audit and deferred this
unchanged plan. After that audit was published at `5281fac`, the owner selected
the icon rollout as Meta Increment 7. D-050 records the new live number. No icon
or product behavior changed through any renumbering.

## Reconstruction record

The first verified implementation commit `a1808e2` predated the dependency
compatibility repair and produced a broken PR merge candidate through no icon
change. The local branch is preserved as
`codex/meta-verified-application-icon-rollout-pre-dependency-repair`. The same
approved icon and closeout scope was reconstructed without commit on fresh
`b298999`, then fully reverified and re-gated before remote PR #19 publication.
