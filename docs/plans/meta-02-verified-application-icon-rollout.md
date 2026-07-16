# Meta Increment 2 - verified application icon rollout

Status: Ready; requires separate project-owner approval
Owner: Project maintainer
Last updated: 2026-07-15

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

Closeout documentation is limited to `AGENTS.md`, `CHANGELOG.md`, `HANDOFF.md`,
`NEXT_STEPS.md`, `PLANS.md`, `PROJECT_STATUS.md`, this plan, a new Meta 2
increment record, `docs/plans/README.md`, and one dated review. Change
`DECISIONS.md` only if implementation establishes a durable generation rule.

## Explicit non-goals

- Logo redesign, recolor, crop, transparency extraction, or alternate identity.
- Changes to the app-icon source, `tauri.conf.json`, manifests, lockfiles,
  bundle identifier, executable, database, GitHub URL, runtime, capabilities,
  entitlements, permissions, or CSP.
- UI, favicon, README, presentation, screenshot, or diagram changes.
- Increment 4V or other product capability work.

## Implementation outline

1. Confirm clean synchronized `main`, a valid Meta 1 marker, and exact source
   hash and dimensions.
2. Begin mandatory `meta-02` gate state before icon edits.
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
`git diff --check`, `npm run verify`, `npm audit --audit-level=low`, and a
bundled Tauri build. On the target Mac verify development and packaged icons in
the Dock, app/window switcher, menu/application context, and Finder in light and
dark appearances. The `meta-02` marker must finish complete and valid.

## Rollback

Before commit, restore only the 16 icon files from the verified Meta 1 baseline
and revert declared closeout documentation. After commit, revert one bounded
Meta 2 commit. No migration, data, dependency, identifier, or remote resource
requires rollback.

## Acceptance criteria

- [ ] Separate project-owner approval is recorded before implementation.
- [ ] Only the exact icon and declared closeout scope changes.
- [ ] Every output derives from the approved app-icon source.
- [ ] Formats, sizes, builds, and target-Mac presentation checks pass.
- [ ] No source asset, config, identifier, runtime, dependency, or permission
      changes.
- [ ] Complete diff and mandatory gate pass without a blocking finding.
