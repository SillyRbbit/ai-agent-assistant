# Knowledge navigation layout post-increment review

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "knowledge-navigation-layout",
  "quality_gate": "PASS WITH ADVISORIES",
  "next_increment_readiness": "Ready with advisories",
  "files_changed": [
    "src/styles.css",
    "scripts/browser/knowledge-check.mjs",
    "CHANGELOG.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "TESTING_GUIDE.md",
    "TROUBLESHOOTING_LOG.md",
    "docs/plans/2026-10-01-knowledge-navigation-layout.md",
    "docs/reviews/2026-10-01-knowledge-navigation-layout-post-increment-review.md"
  ],
  "commands_executed": [
    "sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh node scripts/browser/knowledge-check.mjs /private/tmp/cortexa-knowledge-navigation-layout-evidence/browser-before --shell-only",
    "sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh node scripts/browser/knowledge-check.mjs /private/tmp/cortexa-knowledge-navigation-layout-evidence/browser-after --shell-only",
    "sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh npm run test:frontend -- src/App.test.tsx src/features/knowledge/KnowledgePage.test.tsx src/features/collaboration/CollaborationPage.test.tsx",
    "sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh npm run lint:frontend",
    "sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh npm run typecheck",
    "sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh npm run tauri -- build --debug --bundles app --no-sign --config /private/tmp/cortexa-knowledge-documents-evidence/isolated-native.json -- --locked --offline --config 'profile.release.build-override.strip=\"none\"'",
    "sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh node scripts/browser/knowledge-check.mjs /private/tmp/cortexa-knowledge-navigation-layout-evidence/browser-final --shell-only",
    "sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh node scripts/browser/knowledge-check.mjs /private/tmp/cortexa-knowledge-navigation-layout-evidence/browser-focus-final --shell-only",
    "sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh npm run docs:check",
    "sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh npm run repository:check",
    "sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh npm run security:scan",
    "git diff --check",
    "python3 -B .codex/hooks/session_end_gate.py",
    "python3 -B /private/tmp/cortexa-knowledge-navigation-layout-evidence/preserve.py",
    "sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh npm run format:frontend"
  ],
  "verification": [
    {
      "command": "sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh node scripts/browser/knowledge-check.mjs /private/tmp/cortexa-knowledge-navigation-layout-evidence/browser-before --shell-only",
      "required": false,
      "status": "Failed"
    },
    {
      "command": "sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh node scripts/browser/knowledge-check.mjs /private/tmp/cortexa-knowledge-navigation-layout-evidence/browser-after --shell-only",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh npm run test:frontend -- src/App.test.tsx src/features/knowledge/KnowledgePage.test.tsx src/features/collaboration/CollaborationPage.test.tsx",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh npm run lint:frontend",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh npm run typecheck",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh npm run tauri -- build --debug --bundles app --no-sign --config /private/tmp/cortexa-knowledge-documents-evidence/isolated-native.json -- --locked --offline --config 'profile.release.build-override.strip=\"none\"'",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh node scripts/browser/knowledge-check.mjs /private/tmp/cortexa-knowledge-navigation-layout-evidence/browser-final --shell-only",
      "required": false,
      "status": "Failed"
    },
    {
      "command": "sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh node scripts/browser/knowledge-check.mjs /private/tmp/cortexa-knowledge-navigation-layout-evidence/browser-focus-final --shell-only",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh npm run docs:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh npm run repository:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh npm run security:scan",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "git diff --check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -B .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -B /private/tmp/cortexa-knowledge-navigation-layout-evidence/preserve.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh npm run format:frontend",
      "required": true,
      "status": "Passed"
    }
  ],
  "manual_verification": [
    {
      "check": "Direct native Knowledge and Collaboration wide/compact readability, inspector states, expanded/collapsed navigation and route/resize behavior",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Verified isolated executable and graceful quit/process absence",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Native minimum 760x520 (resize attempt produced no size change; real browser covers this minimum)",
      "required": false,
      "status": "Not run"
    },
    {
      "check": "New native transmission preview and live provider behavior (not authorized; browser disclosure fixture verified without Start)",
      "required": false,
      "status": "Not run"
    }
  ],
  "findings": [
    {
      "category": "Roadmap",
      "severity": "Advisory",
      "summary": "Retain D-127/D-128, provider/runtime live-success and Codex-isolation limitations, process-local native workaround and parked D-125/M1/M2.",
      "risk": "Native layout QA does not establish live integration or new remote CI success. OpenAI remains parked4/5.",
      "effort": "Separate owner-authorized QA/publication review",
      "milestone": "Owner QA and publication readiness",
      "blocks_completion": false,
      "blocks_next_increment": false
    }
  ]
}
-->

Date: 2026-10-01
Increment: knowledge-navigation-layout
Branch: codex/knowledge-documents

## Executive summary

PASS WITH ADVISORIES. Knowledge/Collaboration now reserve the actual expanded or
collapsed navigation width at <=960px. The new actual-shell test reproduced the
old overlap before correction and passed the corrected resize matrix. Direct
native Computer Use observed the previous 960x1410 clipping resolved. No live
request, data mutation, workflow start or Git publication occurred.

## Scope and boundaries

Exactly eleven successor paths in the existing clean worktree at
`e02df763ac2522198c8b9c3169dd51fbf9956cc7`; live main verified at
`02dc35e62ac083a49f0802e8d340c39500f45e78`. Only production CSS changed.
No automatic navigation collapse, timers, forced remounts, dependencies, Rust/IPC,
storage, execution, credential, graph or provider changes. All unrelated tracked
bytes and historical document bodies match the frozen baseline. Prior complete/valid
raw state is byte-identical in the external archive; no historical record reopened.
Previous bundle is preserved; new bundle/product CSS remain unchanged after native
QA. A subsequent harness-only focus wait is bound in artifact-final.json.

## Verification results

Fresh results: 65 affected frontend tests; strict ESLint/typecheck/format; actual-App
resize/geometry/disclosure/focus regression; one locked offline unsigned debug bundle
including frontend build; documentation/repository/security/whitespace; exact scope,
preservation and session checks. Exact commands and exit codes below and in external
executed.json. Ordinary report/finalization/status/full Stop must validate this freeze.

Failed attempts retained truthfully: browser-before reproduced the old sidebar
occlusion; initial lint found an unused harness global and was corrected; browser-final
raced the existing requestAnimationFrame focus restoration. The harness now waits
for that exact focus condition while retaining the original assertion; browser-focus-final
and strict lint passed. No arbitrary delay or application focus change was added.
Historical failed attempts are non-required evidence, not relabeled as passes.

Direct native QA: exact isolated executable SHA-256
`4399832b9f299d3fb9c716fc5143da21471bb9db9e2da31710006fc86ad70be0`;
Knowledge at 1440x1000 and both screens at 2560x1410/960x1410; expanded navigation,
compact collapsed navigation, open/closed inspector, selected existing draft/version,
retention/source-selection disclosures, existing completed room, lower controls and
scrolling. Compact-to-wide return retained collapsed navigation. One right-edge
resize had no effect; fresh corner selection achieved 960x1410. Native 760x520
attempt had no effect and remains unobserved. The matching QA process quit.
No profile, note, room or source save, export, preparation or generation occurred.
Native receipt: `/private/tmp/cortexa-knowledge-navigation-layout-evidence/native-observations.json`.

Browser matrix includes 1600/961/960/959/760 widths and reverse resize, expanded
and collapsed navigation, inspector open/closed, both routes, controls, disclosure,
scrolling, Escape focus and graph docking. Only synthetic browser inputs were used;
acknowledgement remained unchecked and Start disabled. Standalone import/export/
workflow cases were not rerun. Screenshots are external synthetic evidence.

Not run: Rust suites/Clippy, full npm run verify, audits, unchanged fixtures and
prior Simulation workflows. Frontend-tier scope changes no native, dependency,
IPC, storage or toolchain configuration bytes; reuse the bound successful Knowledge
native/full/audit evidence explicitly. Inherited npm audits record zero findings,
and unchanged Rust audit gate accepted only its exact advisory baseline. These are
inherited commands, not claims of execution during this increment. Remote CI,
live providers and a new native transmission preview were not run.

## Architecture findings

PASS. Route-specific CSS uses the existing navigation-state class and width tokens.
Existing App reducer, resize ordering, authority/routing/cancellation and graph
contracts remain byte-identical. No speculative state or architecture added.

## Security findings

PASS. No IPC/capability/CSP/approval/storage/network/credential change. Disclosure
and controls remain outside navigation. Browser fixture is synthetic; no workflow
Start. No owner data or secrets in candidate files. Previous security evidence and
advisories remain; security scan passed without gate weakening.

## Code-health findings

PASS. The old regression silently collapsed navigation. Expanded and collapsed
paths now cross the actual breakpoint without changing state for assertions.
Geometry checks include sidebar separation and hit tests at both control edges
and center. Readiness waits observe existing animation completion/focus conditions.
Initial unused declaration and focus race were repaired within this task.
No new correctness or accessibility blocker found in review.

## Technical debt

No new blocking debt. Existing Advisory items retain owner approval, separate
live/runtime/isolation verification and process-local native workaround. Native
minimum size remains browser-only coverage; risk is unobserved platform behavior,
not a demonstrated defect. Follow-up is optional owner QA, not acceptance waiver.

## Roadmap findings

Ready with advisories for a read-only publication readiness review. No roadmap
lane advanced; OpenAI parked4/5, D-127/D-128 and D-125/M1/M2 remain. Current repair
has no remote CI evidence or permission to commit/publish.

## Completion decision

PASS WITH ADVISORIES. All required checks and direct native wide/compact acceptance
passed. Ordinary schema/finalization/status and full Stop receipts must validate
this report and final workspace. No historical completion claim is rewritten.

## Next-increment readiness

Ready with advisories. Next: inspect exact eleven-path publication scope and valid
completion read-only; obtain separate publication authority. Do not repeat tests,
builds, QA or live requests.

## Exact files changed

- `src/styles.css`
- `scripts/browser/knowledge-check.mjs`
- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-10-01-knowledge-navigation-layout.md`
- `docs/reviews/2026-10-01-knowledge-navigation-layout-post-increment-review.md`

## Exact commands executed

- Failed (exit 1): `sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh node scripts/browser/knowledge-check.mjs /private/tmp/cortexa-knowledge-navigation-layout-evidence/browser-before --shell-only`
- Passed (exit 0): `sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh node scripts/browser/knowledge-check.mjs /private/tmp/cortexa-knowledge-navigation-layout-evidence/browser-after --shell-only`
- Passed (exit 0): `sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh npm run test:frontend -- src/App.test.tsx src/features/knowledge/KnowledgePage.test.tsx src/features/collaboration/CollaborationPage.test.tsx`
- Failed (exit 1): `sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh npm run lint:frontend`
- Passed (exit 0): `sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh npm run typecheck`
- Passed (exit 0): `sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh npm run tauri -- build --debug --bundles app --no-sign --config /private/tmp/cortexa-knowledge-documents-evidence/isolated-native.json -- --locked --offline --config 'profile.release.build-override.strip="none"'`
- Passed (exit 0): `sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh npm run lint:frontend`
- Failed (exit 1): `sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh node scripts/browser/knowledge-check.mjs /private/tmp/cortexa-knowledge-navigation-layout-evidence/browser-final --shell-only`
- Passed (exit 0): `sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh node scripts/browser/knowledge-check.mjs /private/tmp/cortexa-knowledge-navigation-layout-evidence/browser-focus-final --shell-only`
- Passed (exit 0): `sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh npm run lint:frontend`
- Passed (exit 0): `sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh npm run docs:check`
- Passed (exit 0): `sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh npm run repository:check`
- Passed (exit 0): `sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh npm run security:scan`
- Passed (exit 0): `git diff --check`
- Passed (exit 0): `python3 -B .codex/hooks/session_end_gate.py`
- Passed (exit 0): `python3 -B /private/tmp/cortexa-knowledge-navigation-layout-evidence/preserve.py`
- Passed (exit 0): `sh /private/tmp/cortexa-knowledge-documents-evidence/offline.sh npm run format:frontend`

Inspection, ordinary begin, authorized scoped formatting and process/artifact
identity checks were also performed. The machine manifest includes only executed
verification commands, never inherited or skipped commands. Evidence is external
at `/private/tmp/cortexa-knowledge-navigation-layout-evidence`.
