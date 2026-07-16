# Meta Increment 1 - branding and identity foundation

Status: Complete
Owner: Project maintainer
Last updated: 2026-07-15

## Goal

Create one authoritative Cortexa brand system from the owner-supplied logo,
apply it to existing placeholder UI and repository entry points, and add a
repository-local branding skill without changing product behavior or
compatibility identifiers.

## User-visible outcome

The README, browser favicon, and application sidebar use the official Cortexa
logo. Contributors have one documented palette, typography, iconography,
logo-usage, presentation, and architecture-diagram standard.

## Scope

- Preserve the owner source as the primary, light, and dark logo assets.
- Derive a padded 64 x 64 favicon and 512 x 512 application-icon source.
- Replace only the sidebar letter placeholder with a light/dark picture.
- Add favicon and README logo references, six brand guides, and one skill.
- Reconcile stale post-publication 4U/4V memory.
- Defer production Tauri icon replacement to Meta Increment 2.

## Explicit non-goals

- Product behavior, runtime, trust-boundary, IPC, Rust, storage, networking,
  provider, approval, audit, dispatch, or execution changes.
- Repository, package, crate, executable, bundle, database, GitHub, command,
  event, or storage identifier renames.
- Tauri production icon or `tauri.conf.json` changes.
- Logo redraw, vector reconstruction, transparency extraction, crop, rotation,
  recolor, animation, or AI-generated alternative.
- Global product-theme redesign beyond the placeholder brand mark.
- Editing owner files outside the repository.
- Dependency, manifest, lockfile, capability, entitlement, permission, or CSP
  changes.
- Increment 4V or Meta Increment 2 implementation.

## Existing behavior and constraints

- D-026 defines `Cortexa` as the display name and preserves compatibility IDs.
- The sidebar uses a gradient letter `C` placeholder.
- Existing Tauri icons use an older placeholder identity and are out of scope.
- The 360 x 434 source has a fully opaque near-white field. Removing that field
  would invent a new asset.
- Increment 4U is published and merged at `61525bf`; its marker was valid before
  this gate began. Increment 4V remains unstarted and separately controlled.

## Exact files

Created:

```text
assets/branding/logo-primary.png
assets/branding/logo-light.png
assets/branding/logo-dark.png
assets/branding/favicon.png
assets/branding/app-icon-source.png
docs/branding/BRAND_GUIDELINES.md
docs/branding/BRAND_USAGE.md
docs/branding/COLORS.md
docs/branding/TYPOGRAPHY.md
docs/branding/ICONOGRAPHY.md
docs/branding/PRESENTATION_GUIDELINES.md
.agents/skills/branding/SKILL.md
docs/increments/meta-01-branding-foundation.md
docs/plans/meta-01-branding-foundation.md
docs/plans/meta-02-verified-application-icon-rollout.md
docs/reviews/2026-07-15-meta-01-post-increment-review.md
```

Modified:

```text
README.md
index.html
src/components/ApplicationSidebar.tsx
src/styles.css
src/App.test.tsx
ASSISTANT_USAGE.md
AGENTS.md
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
docs/plans/README.md
docs/increments/04u-bind-initial-approval-run-termination.md
docs/plans/04u-bind-initial-approval-run-termination.md
docs/increments/04v-bind-initial-terminal-approval-audit.md
docs/plans/04v-bind-initial-terminal-approval-audit.md
```

No other path may change without renewed approval.

## Implementation steps

- [x] Confirm clean `meta/branding-foundation` at merged `61525bf` and pass the
      baseline repository verification.
- [x] Begin mandatory `meta-01` gate state before repository edits.
- [x] Create and inspect deterministic logo derivatives.
- [x] Replace the sidebar placeholder and add favicon/README references.
- [x] Add the brand reference documents and repository-local skill.
- [x] Reconcile project memory, 4U/4V publication state, and the Meta 2 plan.
- [x] Run focused and complete automated verification.
- [x] Inspect the application in light and dark appearance.
- [x] Review scope, diff, secrets, output, references, identifiers, and icons.
- [x] Run the mandatory gate and require a valid marker.

## Security and privacy considerations

The identity artwork contains no credential or personal content. No data flow
or privilege is introduced. Review must prove no manifest, capability,
permission, IPC, credential, storage, or runtime path changed.

## Risks

- Raster processing could distort, crop, recolor, or upscale the source.
- A dark treatment could incorrectly remove the protected source field.
- Root-level asset imports could work in development but fail in a Vite build.
- The taller sidebar image could shift compact layouts.
- Documentation could imply a compatibility rename or production icon rollout.

## Verification

```bash
shasum -a 256 assets/branding/*.png
sips -g pixelWidth -g pixelHeight -g hasAlpha assets/branding/*.png
python3 /Users/hdang/.codex/skills/.system/skill-creator/scripts/quick_validate.py .agents/skills/branding
npx vitest run src/App.test.tsx
npm run verify
npm audit --audit-level=low
npm run tauri -- dev
git diff --check
python3 .codex/hooks/post_increment_gate.py status
```

Also verify every image reference, inspect built `dist/index.html`, scan for the
removed placeholder, and prove Tauri icons, config, manifests, and lockfiles are
unchanged. Manual visual checks cover the sidebar logo, aspect ratio,
light/dark appearance, compact-window fit, text overlap, and unchanged behavior.

## Rollback

Before commit, delete only the 16 created files and restore only the 18 modified
files to `61525bf`. After commit, revert one bounded Meta Increment 1 commit. No
migration, dependency, remote resource, identifier, or data requires rollback.

## Acceptance criteria

- [x] Only the exact 34 approved paths changed.
- [x] Primary/light/dark assets are byte-identical to the source.
- [x] Favicon and app-icon source use proportional padding, not crop or recolor.
- [x] Every UI and document asset reference exists.
- [x] The sidebar placeholder is removed with no layout regression.
- [x] Tauri production icons and compatibility identifiers are unchanged.
- [x] The complete brand system and repository skill are documented.
- [x] Focused tests, full verification, audit, visual review, and gate pass.
- [x] Memory reflects published 4U, unstarted 4V, complete Meta 1, and Ready
      Meta 2 without implementing a later increment.

## Actual results

The exact 34 approved paths implement the canonical identity assets, six brand
guides, repository-local skill, README/favicon/sidebar references, D-043,
published 4U reconciliation, and the Ready Meta 2 plan. No later increment or
production Tauri icon was implemented.

The primary/light/dark hashes exactly match the 360 x 434 owner source. The
64 x 64 favicon and 512 x 512 app-icon source use proportional padding. Skill
validation, 25 focused App tests, complete `npm run verify` (17 hook, 124
frontend, 95 Rust library, and 21 Rust integration tests), npm audit, native
development launch, light/dark/compact render inspection, build-reference and
exact-scope review, code/security review, and the mandatory gate pass. The
result is `PASS WITH ADVISORIES` only for the documented opaque-raster and
deferred production-icon boundaries.
