# Collapsed sidebar reachability post-increment review

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "collapsed-sidebar-reachability",
  "quality_gate": "FAIL",
  "next_increment_readiness": "Blocked",
  "commands_executed": [
    "npm run test:frontend -- src/App.test.tsx",
    "npm run lint:frontend",
    "npm run typecheck",
    "npm run build:frontend",
    "node scripts/browser/knowledge-check.mjs /private/tmp/cortexa-collapsed-sidebar-reachability-evidence-uh9elj_i/browser-6 --conversations-only --port=4187",
    "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh env -u APPLE_SIGNING_IDENTITY -u APPLE_ID -u APPLE_PASSWORD -u APPLE_API_KEY -u APPLE_API_ISSUER -u APPLE_API_KEY_PATH -u TAURI_SIGNING_PRIVATE_KEY -u TAURI_SIGNING_PRIVATE_KEY_PASSWORD npm run tauri -- build --debug --bundles app --no-sign --config /private/tmp/cortexa-collapsed-sidebar-reachability-evidence-uh9elj_i/native-config.json -- --locked --offline",
    "npm run format:check",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "python3 /private/tmp/cortexa-collapsed-sidebar-reachability-evidence-uh9elj_i/preserve.py",
    "python3 -B .codex/hooks/session_end_gate.py"
  ],
  "files_changed": [
    "CHANGELOG.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "README.md",
    "TESTING_GUIDE.md",
    "TROUBLESHOOTING_LOG.md",
    "assets/branding/app-icon-source.png",
    "assets/branding/approved-september23.png",
    "assets/branding/favicon.png",
    "assets/branding/logo-dark.png",
    "assets/branding/logo-light.png",
    "assets/branding/logo-primary.png",
    "assets/branding/sidebar-symbol.png",
    "docs/branding/APP_ICON_GENERATION.md",
    "docs/branding/BRAND_GUIDELINES.md",
    "docs/plans/2026-10-03-approved-logo-integration.md",
    "docs/plans/2026-10-03-collapsed-sidebar-reachability.md",
    "docs/plans/2026-10-03-conversations-navigation-layout.md",
    "docs/plans/2026-10-03-sidebar-brand-copy-alignment.md",
    "docs/reviews/2026-10-03-approved-logo-integration-post-increment-review.md",
    "docs/reviews/2026-10-03-collapsed-sidebar-reachability-post-increment-review.md",
    "docs/reviews/2026-10-03-conversations-navigation-layout-post-increment-review.md",
    "docs/reviews/2026-10-03-sidebar-brand-copy-alignment-post-increment-review.md",
    "index.html",
    "scripts/branding_assets.py",
    "scripts/browser/knowledge-check.mjs",
    "scripts/tests/test_branding_assets.py",
    "src-tauri/icons/128x128.png",
    "src-tauri/icons/128x128@2x.png",
    "src-tauri/icons/32x32.png",
    "src-tauri/icons/Square107x107Logo.png",
    "src-tauri/icons/Square142x142Logo.png",
    "src-tauri/icons/Square150x150Logo.png",
    "src-tauri/icons/Square284x284Logo.png",
    "src-tauri/icons/Square30x30Logo.png",
    "src-tauri/icons/Square310x310Logo.png",
    "src-tauri/icons/Square44x44Logo.png",
    "src-tauri/icons/Square71x71Logo.png",
    "src-tauri/icons/Square89x89Logo.png",
    "src-tauri/icons/StoreLogo.png",
    "src-tauri/icons/icon.icns",
    "src-tauri/icons/icon.ico",
    "src-tauri/icons/icon.png",
    "src/App.test.tsx",
    "src/components/ApplicationSidebar.tsx",
    "src/styles.css"
  ],
  "verification": [
    {
      "command": "npm run test:frontend -- src/App.test.tsx",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run lint:frontend",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run typecheck",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run build:frontend",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "node scripts/browser/knowledge-check.mjs /private/tmp/cortexa-collapsed-sidebar-reachability-evidence-uh9elj_i/browser-6 --conversations-only --port=4187",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh env -u APPLE_SIGNING_IDENTITY -u APPLE_ID -u APPLE_PASSWORD -u APPLE_API_KEY -u APPLE_API_ISSUER -u APPLE_API_KEY_PATH -u TAURI_SIGNING_PRIVATE_KEY -u TAURI_SIGNING_PRIVATE_KEY_PASSWORD npm run tauri -- build --debug --bundles app --no-sign --config /private/tmp/cortexa-collapsed-sidebar-reachability-evidence-uh9elj_i/native-config.json -- --locked --offline",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run format:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run docs:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run repository:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run security:scan",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "git diff --check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 /private/tmp/cortexa-collapsed-sidebar-reachability-evidence-uh9elj_i/preserve.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -B .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    }
  ],
  "manual_verification": [
    {
      "check": "Verified isolated native short-height sidebar wheel reachability, tooltips, independent workspace composer/disclosure, both navigation states and test-owned app quit/absence",
      "required": true,
      "status": "Failed"
    },
    {
      "check": "Measured 760x520 exact PID/window identity, initial collapsed capture, exact test-owned process cleanup/absence",
      "required": true,
      "status": "Passed"
    }
  ],
  "findings": [
    {
      "category": "Technical debt",
      "severity": "Medium",
      "summary": "Native Computer Use lost window access at first scroll",
      "risk": "Fresh collapsed scrolling, expanded behavior and tooltips remain unobserved; not proof of an application defect.",
      "effort": "Separately authorized supported native acceptance assessment",
      "milestone": "Collapsed sidebar reachability",
      "blocks_completion": true,
      "blocks_next_increment": true
    },
    {
      "category": "Technical debt",
      "severity": "Advisory",
      "summary": "Inherited QA and platform advisories",
      "risk": "Unsigned isolated QA does not verify live-provider success/cancellation or Windows execution.",
      "effort": "Separately scoped owner QA",
      "milestone": "Owner QA",
      "blocks_completion": false,
      "blocks_next_increment": false
    }
  ]
}
-->

Date: 2026-10-03
Increment: collapsed-sidebar-reachability
Branch: codex/collapsed-sidebar-reachability

## Executive summary

Repair implemented; required native acceptance failed on access. Collapsed navigation now owns bounded scrolling. Tooltip labels remain outside the clipping list with hover/focus, bounded coordinates and Escape dismissal. At verified 760x520 points, an initial collapsed capture showed sidebar scrollbar and Settings. First scroll returned noWindowsAvailable; no UI retry. Exact process cleanup/absence verified; no completion claim.

## Scope and boundaries

13 authorized successor paths and 48 cumulative paths. Baseline dcea9df088ddb01c26c1a79ab51e0307190f551c is tree-identical to source HEAD 20961cab5410b749e62e45fa0c765364f5d29ec5. Transferred 46 frozen paths without gate state, profiles, rooms or build output. Original FAIL is preserved and never promoted. No source Rust, IPC, storage, fixture, dependency, branding or execution changes.

## Verification results

New: 43 App tests; final strict lint/typecheck/frontend build; formatting; 28 actual-App resize/state cases including real wheel scrollend, Settings/first route hit testing, every route by Tab/reverse Tab, hover/focus labels and independent workspace scrolling. Offline unsigned app build passed using installed process-local Python/Xcode SDK27/Cargo route. Failed receipts remain: initial callback lint syntax, Testing Library option type errors, tooltip edge offset, hover Escape, and wheel/keyboard settling assertion. Corrected in scope; no assertions weakened. Inherited: Rust/animation/workflow, branding fidelity and prior width/expanded native evidence; not newly executed. Historical sidebar input delivery remains unknown. New native evidence is partial as recorded in observations.json; expanded and fresh scroll/tooltip acceptance is unobserved.

## Architecture findings

PASS for reviewed repair: shell remains presentation-only; one navigation list owns vertical scrolling. Existing semantic controls and route handlers remain. Tooltip elements are siblings outside the clip, with no new portal framework, dependency, timers or generic abstraction. Scroll/resize/keydown listeners exist only while a tooltip is active and are removed on cleanup.

## Security findings

PASS for reviewed repair: no model/device or WebView authority change. No new IPC, permissions, credentials, storage, network or execution handlers. Existing synthetic fixture remains unchanged and deny-by-default. QA helper uses only exact executable identity and PID-filtered CoreGraphics metadata, no arguments/environments/titles/Accessibility/permission prompts. One initial helper extraction NameError was corrected externally before metadata access; no extra launch.

## Code-health findings

Scoped diff reviewed against transferred files. Preserves logo geometry and dimensions, navigation state, footer/new-conversation layout, inspector and route boundaries. Existing tests retained; new tests cover outside-list tooltip association and dismissal; browser checks validate real input and wait on actual scrollend rather than arbitrary delay. No implementation is accepted solely from headless evidence.

## Technical debt

Required native walkthrough was stopped on access failure and blocks completion. Retain D-127/D-128, native/provider/runtime/Codex-isolation, icon/artwork/chunk advisories and the process-local build workaround. Live QA parked 3/10; D-125/M1/M2 parked. No historical or unrelated advisory fixed.

## Roadmap findings

Owner selected only this bounded isolated repair. No next increment, publication or governance authority follows. Original terminal failure and prior completion records remain separate immutable evidence.

## Completion decision

FAIL. Freeze this report and close-failed; no completion marker and no successor authority. Do not reopen or promote this record.

## Next-increment readiness

Blocked: required native acceptance remains incomplete. No automatic successor, repair, retry or publication.

## Exact files changed

- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `README.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `assets/branding/app-icon-source.png`
- `assets/branding/approved-september23.png`
- `assets/branding/favicon.png`
- `assets/branding/logo-dark.png`
- `assets/branding/logo-light.png`
- `assets/branding/logo-primary.png`
- `assets/branding/sidebar-symbol.png`
- `docs/branding/APP_ICON_GENERATION.md`
- `docs/branding/BRAND_GUIDELINES.md`
- `docs/plans/2026-10-03-approved-logo-integration.md`
- `docs/plans/2026-10-03-collapsed-sidebar-reachability.md`
- `docs/plans/2026-10-03-conversations-navigation-layout.md`
- `docs/plans/2026-10-03-sidebar-brand-copy-alignment.md`
- `docs/reviews/2026-10-03-approved-logo-integration-post-increment-review.md`
- `docs/reviews/2026-10-03-collapsed-sidebar-reachability-post-increment-review.md`
- `docs/reviews/2026-10-03-conversations-navigation-layout-post-increment-review.md`
- `docs/reviews/2026-10-03-sidebar-brand-copy-alignment-post-increment-review.md`
- `index.html`
- `scripts/branding_assets.py`
- `scripts/browser/knowledge-check.mjs`
- `scripts/tests/test_branding_assets.py`
- `src-tauri/icons/128x128.png`
- `src-tauri/icons/128x128@2x.png`
- `src-tauri/icons/32x32.png`
- `src-tauri/icons/Square107x107Logo.png`
- `src-tauri/icons/Square142x142Logo.png`
- `src-tauri/icons/Square150x150Logo.png`
- `src-tauri/icons/Square284x284Logo.png`
- `src-tauri/icons/Square30x30Logo.png`
- `src-tauri/icons/Square310x310Logo.png`
- `src-tauri/icons/Square44x44Logo.png`
- `src-tauri/icons/Square71x71Logo.png`
- `src-tauri/icons/Square89x89Logo.png`
- `src-tauri/icons/StoreLogo.png`
- `src-tauri/icons/icon.icns`
- `src-tauri/icons/icon.ico`
- `src-tauri/icons/icon.png`
- `src/App.test.tsx`
- `src/components/ApplicationSidebar.tsx`
- `src/styles.css`

## Exact commands executed

- `npm run test:frontend -- src/App.test.tsx` — Passed
- `npm run lint:frontend` — Passed
- `npm run typecheck` — Passed
- `npm run build:frontend` — Passed
- `node scripts/browser/knowledge-check.mjs /private/tmp/cortexa-collapsed-sidebar-reachability-evidence-uh9elj_i/browser-6 --conversations-only --port=4187` — Passed
- `sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh env -u APPLE_SIGNING_IDENTITY -u APPLE_ID -u APPLE_PASSWORD -u APPLE_API_KEY -u APPLE_API_ISSUER -u APPLE_API_KEY_PATH -u TAURI_SIGNING_PRIVATE_KEY -u TAURI_SIGNING_PRIVATE_KEY_PASSWORD npm run tauri -- build --debug --bundles app --no-sign --config /private/tmp/cortexa-collapsed-sidebar-reachability-evidence-uh9elj_i/native-config.json -- --locked --offline` — Passed
- `npm run format:check` — Passed
- `npm run docs:check` — Passed
- `npm run repository:check` — Passed
- `npm run security:scan` — Passed
- `git diff --check` — Passed
- `python3 /private/tmp/cortexa-collapsed-sidebar-reachability-evidence-uh9elj_i/preserve.py` — Passed
- `python3 -B .codex/hooks/session_end_gate.py` — Passed
