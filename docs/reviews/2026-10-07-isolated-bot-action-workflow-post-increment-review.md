# Isolated bot action workflow post-increment review

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "isolated-bot-action-workflow",
  "commands_executed": [
    "npm run verify",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "/usr/bin/python3 -B .codex/hooks/session_end_gate.py",
    "/usr/local/bin/python3 -B /private/tmp/cortexa-isolated-action-final-39rz8nvq/preserve.py"
  ],
  "files_changed": [
    ".agents/skills/documentation-sync/SKILL.md",
    ".agents/skills/post-increment-gate/SKILL.md",
    ".agents/skills/quality-gate/SKILL.md",
    ".agents/skills/readiness-review/SKILL.md",
    ".agents/skills/session-end/SKILL.md",
    ".agents/skills/verified-increment/SKILL.md",
    ".codex/hooks/lifecycle_closure.py",
    ".codex/hooks/post_increment_gate.py",
    ".codex/hooks/tests/test_post_increment_gate.py",
    "AGENTS.md",
    "ARCHITECTURE.md",
    "CHANGELOG.md",
    "CODE_REVIEW.md",
    "DECISIONS.md",
    "ENGINEERING_GUIDE.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PRODUCT_REQUIREMENTS.md",
    "PROJECT_STATUS.md",
    "SECURITY.md",
    "SECURITY_CHECKLIST.md",
    "TESTING_GUIDE.md",
    "TROUBLESHOOTING_LOG.md",
    "docs/architecture/current/01-system.svg",
    "docs/architecture/current/02-sequence.svg",
    "docs/architecture/current/03-collaboration.svg",
    "docs/architecture/current/04-context.svg",
    "docs/architecture/current/05-lifecycle.svg",
    "docs/architecture/current/06-observability.svg",
    "docs/architecture/current/07-security.svg",
    "docs/architecture/current/08-delivery.svg",
    "docs/architecture/current/09-future.svg",
    "docs/architecture/current/README.md",
    "docs/architecture/current/capabilities.md",
    "docs/architecture/current/diagrams.json",
    "docs/architecture/current/external-projects.md",
    "docs/architecture/current/overview.pdf",
    "docs/architecture/current/render_diagrams.py",
    "docs/architecture/current/security.md",
    "docs/architecture/current/sources.json",
    "docs/architecture/current/viewer.html",
    "docs/architecture/current/walkthrough.md",
    "docs/governance/MASTER_PROMPT.md",
    "docs/plans/2026-10-07-current-architecture-walkthrough.md",
    "docs/plans/2026-10-07-isolated-bot-action-workflow.md",
    "docs/plans/2026-10-08-d131-qa-worktree-integration.md",
    "docs/reviews/2026-10-07-current-architecture-walkthrough-post-increment-review.md",
    "docs/reviews/2026-10-07-isolated-bot-action-workflow-post-increment-review.md",
    "docs/reviews/2026-10-08-d131-qa-worktree-integration-post-increment-review.md",
    "docs/templates/INCREMENT_TEMPLATE.md",
    "docs/templates/POST_INCREMENT_REVIEW_TEMPLATE.md",
    "docs/templates/READINESS_REVIEW_TEMPLATE.md",
    "docs/workflows/END_SESSION.md",
    "docs/workflows/RESUME_SESSION.md",
    "docs/workflows/START_SESSION.md",
    "scripts/repository_health.py",
    "scripts/tests/test_repository_health.py",
    "src-tauri/Cargo.lock",
    "src-tauri/Cargo.toml",
    "src-tauri/examples/native_approval_dialog.rs",
    "src-tauri/src/agent_adapter.rs",
    "src-tauri/src/agent_chat.rs",
    "src-tauri/src/agent_chat_tauri.rs",
    "src-tauri/src/approvals/decision_source.rs",
    "src-tauri/src/approvals/manager.rs",
    "src-tauri/src/approvals/types.rs",
    "src-tauri/src/audit/approval.rs",
    "src-tauri/src/codex_connection.rs",
    "src-tauri/src/collaboration.rs",
    "src-tauri/src/collaboration_action.rs",
    "src-tauri/src/collaboration_tauri.rs",
    "src-tauri/src/isolated_action/mod.rs",
    "src-tauri/src/isolated_action/sandbox.rs",
    "src-tauri/src/isolated_action/tests.rs",
    "src-tauri/src/isolated_action/workspace.rs",
    "src-tauri/src/lib.rs",
    "src-tauri/src/storage/collaboration.rs",
    "src/features/collaboration/ActionReview.tsx",
    "src/features/collaboration/CollaborationPage.css",
    "src/features/collaboration/CollaborationPage.test.tsx",
    "src/features/collaboration/CollaborationPage.tsx",
    "src/features/command-center/collaborationProjection.test.ts",
    "src/infrastructure/tauri/collaboration-client.test.ts",
    "src/infrastructure/tauri/collaboration-client.ts"
  ],
  "findings": [
    {
      "category": "Code health",
      "severity": "Low",
      "summary": "The isolated action scope copy still says OpenAI API only although separately approved Codex routing is implemented; exact transmission/runtime display and native evidence remain explicit.",
      "risk": "Owner may mistake supported scope from stale paragraph; no automatic fallback occurs.",
      "effort": "Small copy/test correction in a separately selected task.",
      "milestone": "Future owner-selected UI clarification",
      "blocks_completion": false,
      "blocks_next_increment": false
    },
    {
      "category": "Security",
      "severity": "Advisory",
      "summary": "Pinned Docker image and host-bound isolation are not a vulnerability-free image or hostile daemon/kernel guarantee.",
      "risk": "Host compromise and image future advisories remain external trust assumptions.",
      "effort": "Bounded re-review only when dependencies or execution scope change.",
      "milestone": "Future selected dependency review",
      "blocks_completion": false,
      "blocks_next_increment": false
    },
    {
      "category": "Technical debt",
      "severity": "Advisory",
      "summary": "Actual rollback restoration, native Docker-active cancellation, API repetition of shared lifecycle cases and all timing/platform/provider variants remain untested.",
      "risk": "Current passing claim is the bounded matrix, not arbitrary repository or all-runtime equivalence.",
      "effort": "Additional selected native QA with explicit access and usage budget.",
      "milestone": "Future owner-selected coverage extension",
      "blocks_completion": false,
      "blocks_next_increment": false
    },
    {
      "category": "Architecture",
      "severity": "Advisory",
      "summary": "Native Computer Use is app-scoped and cannot prove CoreGraphics-ID-directed input; additional same-PID records are unclassified.",
      "risk": "GUI inventory is not system-wide duplicate absence or per-provider-child PID proof.",
      "effort": "Preserve exact continuity and stop conditions on future QA.",
      "milestone": "All future native QA",
      "blocks_completion": false,
      "blocks_next_increment": false
    },
    {
      "category": "Technical debt",
      "severity": "Advisory",
      "summary": "Process-local Python/SDK27/Cargo workaround, inherited Hermes ignore and Node/Vite warnings remain; historical discarded schema causes remain unknown.",
      "risk": "Do not confuse reproducibility constraints or unknown past cause with broader acceptance.",
      "effort": "Only separately selected environment/runtime work.",
      "milestone": "Deferred; D-125/M1/M2 remain parked",
      "blocks_completion": false,
      "blocks_next_increment": false
    }
  ],
  "verification": [
    {
      "command": "npm run verify",
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
      "command": "/usr/bin/python3 -B .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "/usr/local/bin/python3 -B /private/tmp/cortexa-isolated-action-final-39rz8nvq/preserve.py",
      "required": true,
      "status": "Passed"
    }
  ],
  "manual_verification": [
    {
      "check": "Closed target/edit boundary \u2014 automatically verified",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Actual isolated execution \u2014 native + automated",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Bounded correction \u2014 Codex native",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Exact native Reject \u2014 Codex native",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Exact native Approve/apply/recovery \u2014 Codex native + automated",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Active cancellation and subsequent recovery \u2014 Codex native + automated",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Drift/stale review blocked \u2014 Codex native + automated",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Interruption/restart/no replay and graph \u2014 Codex native + automated",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Direct API route \u2014 API native",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Source-bound verification and cleanup \u2014 automated + native",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Optional extended variants: rollback restoration, native Docker-active cancellation, API lifecycle repetition, all platform/timing/provider variants",
      "required": false,
      "status": "Not run"
    }
  ],
  "quality_gate": "PASS WITH ADVISORIES",
  "next_increment_readiness": "Ready with advisories"
}
-->

Date: 2026-10-08
Increment: isolated-bot-action-workflow
Branch: codex/live-provider-qa
HEAD: bc12776412c717613de1fdc42c38bf3312493726

## Executive summary

The bounded required product/native acceptance is satisfied. Review result: **PASS WITH ADVISORIES**. This final review adds documentation and the reserved report only; no product edits, tests/builds, launches or provider requests are repeated. Ledger63/66. Current final-review checks: **Passed**. The ordinary gate and full Stop are evaluated only after those checks actually pass; actual outcomes are externally recorded, not presumed here. Historical failures remain unchanged.

Source-bound direct API and dedicated-home Headless Codex results are separate. Model output remains an untrusted proposal; deterministic Rust validates, executes fixed Docker checks, binds native approval and applies only exact new-file bytes.

## Scope and boundaries

The accepted scope is one new root-level UTF-8 Python file, one unchanged unittest file and one bounded correction in a clean small ordinary Git clone. Existing-file replacement wording in the original plan is historical and superseded, not newly implemented. No generic shell, arbitrary mounts/commands, provider fallback, runtime tools, target overwrite, package installation or device authority is granted.

The full Git inventory below includes inherited architecture-atlas work and separately attributed D-131 governance integration; it is not all new work in this review. Original atlas completion/state, D-133 mechanism, all product bytes, index/worktree links and historical validators are preserved. D-131 governs routine work; active schema3 legacy admission remains unchanged until ordinary finalization. No live-state copying or conversion.

Current review attribution: additive extensions to ARCHITECTURE.md, CHANGELOG.md, HANDOFF.md, NEXT_STEPS.md, PLANS.md, PROJECT_STATUS.md, TESTING_GUIDE.md, TROUBLESHOOTING_LOG.md and the existing action plan; this reserved review is new. A recoverable before-byte snapshot, index and raw active state are in `/private/tmp/cortexa-isolated-action-final-39rz8nvq`.

## Verification results

D-131 permits input-bound evidence reuse. Required `npm run verify` is Passed, reused from `/private/tmp/cortexa-qa-shape-1cv9b4i4/verify-host.json` (exit0,303.832seconds), with its saved log, wrapper and unchanged relevant product/config/dist input hashes. Environment: installed Node/npm, dedicated external Cargo output, installed Python3.12, Xcode SDK27 and process-local offline workaround; no toolchain modification. 145 hook tests,95 repository tests,612 frontend tests,491 Rust library passes and255 integration passes. The integration invocation repeats library execution; do not add duplicate counts. Two opt-in Docker tests and one inherited Hermes probe were ignored in the ordinary suite.

The unchanged two Docker opt-in tests were separately exercised as part of the 11/11 isolated-action run (`/private/tmp/cortexa-isolated-action-g_e2rnqv/rust-focused-004.log`,exit0): actual isolation/resource/output/time/network/host/credential/socket boundaries, child cancellation cleanup and correction orchestration with synthetic model events plus real Docker. This is automated execution evidence, not live native model/approval evidence. Later verification includes approved parenting/example and fixed shape diagnostics. No new product tests were run for documentation.

Inherited provider milestone/A-B Codex recheck remains valid against its frozen snapshots; it is not relabeled action acceptance. Historical failed model contracts, preparation errors, parser causes unknown from discarded responses, original failed external readers and permission-denied cleanup probes are retained. Successful later scoped evidence does not rewrite those receipts.

Current docs/repository/security/whitespace/session and preservation command results are recorded in the machine manifest and external receipts. Schema checking binds the complete changed inventory and all required rows. Gate status and full-payload Stop are separate final actions, not tests implicitly claimed by this report.

## Architecture findings

No blocking architecture finding. Existing Conductor, collaboration executor, adapters, generation lease, profiles, room storage and graph are extended; no second orchestration framework. Closed `isolated_action` modules own target snapshot, edit contract, sandbox and application. Native trusted presenter owns the invoking main-window lifetime and validated borrowed handles; workspace logic gains no Tauri/WebView authority. Captures fail closed without deliberate unparented fallback or speculative threading change.

Review hashes bind baseline, file/test bytes, all attempts, actual checks and strict QA. Request counts include children/corrections. Storage retains failures and marks interrupted or uncertain apply/review state recovery_required on restart, never replaying execution/application. User-visible graph/polling replaces snapshots monotonically. Native restart confirms retained truthful status. Portability remains macOS-native for folder/approval routes; Docker/host assumptions remain explicit.

## Security findings

No blocking security finding in the approved boundary. Strict serde unknown/duplicate/type/version/hash rejection remains authoritative; secondary diagnostics run only after rejection, produce fixed labels and never relax acceptance or retain raw response/field/parser content. Codex catalog and dedicated-home authentication are explicit; no personal-home substitution, runtime tools or API fallback. Live API keys were owner-private and never observed by this review.

Fixed Docker invocation uses pinned pull-never image, nonroot identity, network none, dropped capabilities, no-new-privileges, read-only root/mount, bounded tmp/PIDs/memory/CPU/output/deadline. Provider auth/home, owner target, Git metadata and daemon socket are not mounted. Owned exact-ID cleanup precedes success. Host daemon/kernel compromise is outside this isolation claim.

No-follow held root descriptors, clean HEAD/index/raw content checks, link/path/control/size constraints and repeated rechecks reject drift. Native approval binds exact run/review/baseline/candidate/expiry and one-use consuming grant; WebView/model flags cannot approve. Durable intent and NOREPLACE atomic new-file publication prevent overwriting concurrent owner files. Completion bytes are reverified; ambiguous restart needs manual recovery, never automatic overwrite/replay.

Cargo changed only the macOS root raw-window-handle0.6.2 edge; retained registry versions/checksums are unchanged. IPC exact allowlists and payload tests remain strict. Tauri CSP/capabilities, unsafe-code prohibition, validation/security gates and private data boundaries are unchanged. Preserved error/privacy and race regressions plus exact native approval evidence support the boundary; no hostile-host or vulnerability-free-image claim.

## Code-health findings

No blocking code-health finding. Focused strict-contract, schema diagnostics, path/command containment, parent/error/outcome, approval replay/drift, cleanup and restart tests are retained, along with frontend malformed/native-only/monotonic-state tests. ActionReview uses escaped React text/preformatted diff and actual immutable attempt evidence; review controls require review_ready and bound reviewHash. Fixed public errors provide actionable stop/recovery guidance. No new speculative abstraction, journal coalescing or unrelated refactor.

Low advisory: the isolated scope paragraph still says OpenAI API only; exact runtime/transmission fields remain correct for separately approved Codex. Preserve this observation for a separately selected copy/test correction; no unauthorized product edit is made during final review.

## Technical debt

- Code health / Low: The isolated action scope copy still says OpenAI API only although separately approved Codex routing is implemented; exact transmission/runtime display and native evidence remain explicit. Risk: Owner may mistake supported scope from stale paragraph; no automatic fallback occurs. Effort: Small copy/test correction in a separately selected task. Milestone: Future owner-selected UI clarification. Blocks completion: no; blocks next increment: no.

- Security / Advisory: Pinned Docker image and host-bound isolation are not a vulnerability-free image or hostile daemon/kernel guarantee. Risk: Host compromise and image future advisories remain external trust assumptions. Effort: Bounded re-review only when dependencies or execution scope change. Milestone: Future selected dependency review. Blocks completion: no; blocks next increment: no.

- Technical debt / Advisory: Actual rollback restoration, native Docker-active cancellation, API repetition of shared lifecycle cases and all timing/platform/provider variants remain untested. Risk: Current passing claim is the bounded matrix, not arbitrary repository or all-runtime equivalence. Effort: Additional selected native QA with explicit access and usage budget. Milestone: Future owner-selected coverage extension. Blocks completion: no; blocks next increment: no.

- Architecture / Advisory: Native Computer Use is app-scoped and cannot prove CoreGraphics-ID-directed input; additional same-PID records are unclassified. Risk: GUI inventory is not system-wide duplicate absence or per-provider-child PID proof. Effort: Preserve exact continuity and stop conditions on future QA. Milestone: All future native QA. Blocks completion: no; blocks next increment: no.

- Technical debt / Advisory: Process-local Python/SDK27/Cargo workaround, inherited Hermes ignore and Node/Vite warnings remain; historical discarded schema causes remain unknown. Risk: Do not confuse reproducibility constraints or unknown past cause with broader acceptance. Effort: Only separately selected environment/runtime work. Milestone: Deferred; D-125/M1/M2 remain parked. Blocks completion: no; blocks next increment: no.

## Roadmap findings

Readiness review: the private owner-only bounded milestone is accepted with the stated advisories; the next meaningful action is a read-only publication-readiness assessment, followed only by separately authorized publication. This review does not reorder NEXT_STEPS, select a later product milestone or prioritize productionization. Unfinished branding/rollback disposition and D-125/M1/M2 remain parked, not passed or waived. The historical API credential blocker is superseded only for this successfully completed one-action checkpoint; it is not a promise of future private access. Dedicated Codex results remain separate.

## Completion decision

**PASS WITH ADVISORIES**. All required bounded acceptance rows below pass; optional variants remain unclaimed. No required criterion is waived. Ordinary finalization is permitted only after actual passing current verification/schema/preservation; completion is not granted by this Markdown alone. Preserve original schema3 legacy admission and original raw state. The finalizer alone may write the completed state, which is frozen by its exact recorded hash; independent post-finalization preservation and complete/valid/full Stop checks must pass.

- Passed — Closed target/edit boundary (automatically verified): Clean ordinary clone; up to32 regular root files; one new Python file only. Strict four-key edit, path/link/index/branch/extra-file and stale-content rejection. Evidence: `src-tauri/src/isolated_action/tests.rs; unchanged full verify`.
- Passed — Actual isolated execution (native + automated): API and Codex actual pinned Docker exit0/three tests. Nonroot/network-none/read-only/no credential/socket mounts; resource/output/time bounds and child cancellation exercised separately with actual Docker. Evidence: `/private/tmp/cortexa-api-action-wfza90xc; /private/tmp/cortexa-isolated-action-g_e2rnqv/rust-focused-004.log`.
- Passed — Bounded correction (Codex native): Multiplication failed actual validation; addition correction passed; both checks and strict QA retained; four dispatches. Evidence: `/private/tmp/cortexa-shape-native-2arkqf3s/FINAL-HANDOFF.md`.
- Passed — Exact native Reject (Codex native): Exact parented sheet Reject; target unchanged, no apply journals. Evidence: `/private/tmp/cortexa-parenting-acceptance-r8vhrkso/FINAL-HANDOFF.md`.
- Passed — Exact native Approve/apply/recovery (Codex native + automated): Exact reviewed new-file bytes applied once; bound approval, intent/completion journals; unrelated files unchanged; recovery records retained. Replay/race/partial-apply safeguards automated. Actual rollback restoration not exercised. Evidence: `/private/tmp/cortexa-approve-contract-eqb7c_aq/FINAL-HANDOFF.md; apply-verification.json`.
- Passed — Active cancellation and subsequent recovery (Codex native + automated): Observed active Coding cancelled, truthful terminal, target unchanged; new action reached review_ready afterward. No Docker container had yet started in this native cancellation; actual Docker child cleanup covered by separate automated evidence. Evidence: `/private/tmp/cortexa-lifecycle-acceptance-k7affrbs/FINAL-HANDOFF.md`.
- Passed — Drift/stale review blocked (Codex native + automated): Only owner-drift.txt added to fresh disposable fixture; Review rejected drift before native approval/application; original evidence retained. Evidence: `/private/tmp/cortexa-lifecycle-acceptance-k7affrbs/FINAL-HANDOFF.md`.
- Passed — Interruption/restart/no replay and graph (Codex native + automated): Quit while QA active; one restart showed interrupted/recovery_required, Coding retained, unchanged request count and evidence, no execution/apply replay or running graph state. Evidence: `/private/tmp/cortexa-lifecycle-acceptance-k7affrbs/FINAL-HANDOFF.md`.
- Passed — Direct API route (API native): API/gpt-5.6-luna/low Coding and QA completed strict contracts; real Docker pass; completed/review_ready; target unchanged; no additional approval/apply claimed. Evidence: `/private/tmp/cortexa-api-action-wfza90xc/FINAL-HANDOFF.md`.
- Passed — Source-bound verification and cleanup (automated + native): Full verify reused for identical relevant source/config/dist. All task-owned app/container cleanup already verified; no new launch this review. Historical failures, fixtures and artifacts immutable. Evidence: `/private/tmp/cortexa-qa-shape-1cv9b4i4/verify-host.json; latest preservation and cleanup receipts`.

## Next-increment readiness

**Ready with advisories** for read-only publication readiness. Commit/push/PR/publication are not authorized by this task. Preserve input identities and all advisories; do not repeat native QA or builds merely because a new session begins. Current external final handoff contains actual completion/Stop outcomes and exact resume instructions. If any final gate fails, that failure blocks publication readiness despite passing product evidence.

## Exact files changed

- `.agents/skills/documentation-sync/SKILL.md`
- `.agents/skills/post-increment-gate/SKILL.md`
- `.agents/skills/quality-gate/SKILL.md`
- `.agents/skills/readiness-review/SKILL.md`
- `.agents/skills/session-end/SKILL.md`
- `.agents/skills/verified-increment/SKILL.md`
- `.codex/hooks/lifecycle_closure.py`
- `.codex/hooks/post_increment_gate.py`
- `.codex/hooks/tests/test_post_increment_gate.py`
- `AGENTS.md`
- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `CODE_REVIEW.md`
- `DECISIONS.md`
- `ENGINEERING_GUIDE.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PRODUCT_REQUIREMENTS.md`
- `PROJECT_STATUS.md`
- `SECURITY.md`
- `SECURITY_CHECKLIST.md`
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
- `docs/governance/MASTER_PROMPT.md`
- `docs/plans/2026-10-07-current-architecture-walkthrough.md`
- `docs/plans/2026-10-07-isolated-bot-action-workflow.md`
- `docs/plans/2026-10-08-d131-qa-worktree-integration.md`
- `docs/reviews/2026-10-07-current-architecture-walkthrough-post-increment-review.md`
- `docs/reviews/2026-10-07-isolated-bot-action-workflow-post-increment-review.md`
- `docs/reviews/2026-10-08-d131-qa-worktree-integration-post-increment-review.md`
- `docs/templates/INCREMENT_TEMPLATE.md`
- `docs/templates/POST_INCREMENT_REVIEW_TEMPLATE.md`
- `docs/templates/READINESS_REVIEW_TEMPLATE.md`
- `docs/workflows/END_SESSION.md`
- `docs/workflows/RESUME_SESSION.md`
- `docs/workflows/START_SESSION.md`
- `scripts/repository_health.py`
- `scripts/tests/test_repository_health.py`
- `src-tauri/Cargo.lock`
- `src-tauri/Cargo.toml`
- `src-tauri/examples/native_approval_dialog.rs`
- `src-tauri/src/agent_adapter.rs`
- `src-tauri/src/agent_chat.rs`
- `src-tauri/src/agent_chat_tauri.rs`
- `src-tauri/src/approvals/decision_source.rs`
- `src-tauri/src/approvals/manager.rs`
- `src-tauri/src/approvals/types.rs`
- `src-tauri/src/audit/approval.rs`
- `src-tauri/src/codex_connection.rs`
- `src-tauri/src/collaboration.rs`
- `src-tauri/src/collaboration_action.rs`
- `src-tauri/src/collaboration_tauri.rs`
- `src-tauri/src/isolated_action/mod.rs`
- `src-tauri/src/isolated_action/sandbox.rs`
- `src-tauri/src/isolated_action/tests.rs`
- `src-tauri/src/isolated_action/workspace.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/storage/collaboration.rs`
- `src/features/collaboration/ActionReview.tsx`
- `src/features/collaboration/CollaborationPage.css`
- `src/features/collaboration/CollaborationPage.test.tsx`
- `src/features/collaboration/CollaborationPage.tsx`
- `src/features/command-center/collaborationProjection.test.ts`
- `src/infrastructure/tauri/collaboration-client.test.ts`
- `src/infrastructure/tauri/collaboration-client.ts`

## Exact commands executed

- `npm run verify` — Passed, reused unchanged source-bound evidence; not rerun.
- `npm run docs:check` — Passed, current final-review receipt in external evidence directory.
- `npm run repository:check` — Passed, current final-review receipt in external evidence directory.
- `npm run security:scan` — Passed, current final-review receipt in external evidence directory.
- `git diff --check` — Passed, current final-review receipt in external evidence directory.
- `/usr/bin/python3 -B .codex/hooks/session_end_gate.py` — Passed, current final-review receipt in external evidence directory.
- `/usr/local/bin/python3 -B /private/tmp/cortexa-isolated-action-final-39rz8nvq/preserve.py` — Passed, current final-review receipt in external evidence directory.

The Docker-focused log and exit receipt are retained. Its exact invocation string is not reconstructed here; the log identifies both opt-in tests as actually executed. Same-task begin used `/usr/bin/python3 -B .codex/hooks/post_increment_gate.py begin --increment isolated-bot-action-workflow` and passed with byte-identical active state. Read-only report-schema validation imports the existing authoritative gate validator; ordinary finalize, status and complete-payload Stop are recorded externally after report freeze.
