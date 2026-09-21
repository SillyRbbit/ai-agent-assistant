# Demo cleanup publication post-increment review

Date: 2026-09-20. Increment: `demo-cleanup-publication`.
Branch: `codex/demo-cleanup-publication`.
Base: `f176c36cc701b5a296162331cfcb2600b2157663`.

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "demo-cleanup-publication",
  "quality_gate": "PASS WITH ADVISORIES",
  "next_increment_readiness": "Ready with advisories",
  "verification": [
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
      "command": "npm run test:frontend -- src/features/command-center src/App.test.tsx",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run build:frontend",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run format:frontend",
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
      "command": "python3 -B .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    }
  ],
  "manual_verification": [
    {
      "check": "Actual 1400x900 browser Fit View: nine equal agent heights, common row edges, visible titles, consistent status insets and clear contained group headers",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Actual 820x800 and 600x800 browser Fit View: equal heights, visible titles, no card overlaps, group containment and no Structured fallback",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "QA inspector on desktop and narrow view, keyboard selection, pan, zoom, Fit View, rendered connector and event activity selection",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Architecture, security, code-health, technical-debt and readiness review of the exact candidate",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Exact 22-path candidate scope and byte-identical UI extraction; original-checkout file hashes, HEAD and Git status preserved",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Native GUI smoke and Research/Knowledge lifecycle controls",
      "required": false,
      "status": "Manual verification pending"
    }
  ],
  "findings": [
    {
      "category": "Code health",
      "severity": "Advisory",
      "summary": "Native GUI smoke remains pending.",
      "risk": "Browser verification does not prove native WebView interaction.",
      "effort": "Small",
      "milestone": "Owner native demo walkthrough",
      "blocks_completion": false,
      "blocks_next_increment": false
    }
  ],
  "files_changed": [
    "CHANGELOG.md",
    "DECISIONS.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "ROADMAP.md",
    "docs/plans/2026-09-20-demo-cleanup-publication.md",
    "docs/reviews/2026-09-20-demo-cleanup-publication-post-increment-review.md",
    "src/features/command-center/CommandCenterPage.test.tsx",
    "src/features/command-center/CommandCenterPage.tsx",
    "src/features/command-center/command-center.css",
    "src/features/command-center/components/CommandCenterActivityStream.tsx",
    "src/features/command-center/components/CommandCenterHeader.tsx",
    "src/features/command-center/components/CommandCenterOverview.tsx",
    "src/features/command-center/components/ContextualInspector.tsx",
    "src/features/command-center/components/OperationalTopologyAdapter.test.tsx",
    "src/features/command-center/components/OperationalTopologyAdapter.tsx",
    "src/features/command-center/components/OperationalTopologyPanel.test.tsx",
    "src/features/command-center/components/OperationalTopologyPanel.tsx",
    "src/features/command-center/components/TopologyStructuredView.tsx",
    "src/features/command-center/useCommandCenterState.ts"
  ],
  "commands_executed": [
    "npm run lint:frontend",
    "npm run typecheck",
    "npm run test:frontend -- src/features/command-center src/App.test.tsx",
    "npm run build:frontend",
    "npm run format:frontend",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "python3 -B .codex/hooks/session_end_gate.py"
  ]
}
-->

## Executive summary

The completed 13-file UI cleanup is extracted onto verified remote main.
Structured view is removed, group headers are plain and all nine agent cards
have one height with aligned standard rows. No implementation was repeated.
All required candidate checks and reviews passed. Quality is
PASS WITH ADVISORIES for pending native GUI smoke. Final completion is
authoritative only when the ordinary gate reports complete and valid.

## Scope and boundaries

The [plan](../plans/2026-09-20-demo-cleanup-publication.md) fixes exactly 22
paths: 13 existing UI changes, seven additive memory documents and this
plan/report pair. The original checkout, seven unpublished commits and dirty
D-125 work remain separate. No governance, harness, dependency, native, IPC,
capability, credential, runtime or provider change is included. No merge.

## Verification results

- Passed: 21 baseline adapter/panel tests before transfer.
- Passed: candidate ESLint, strict typecheck, 191 tests in nine affected
  frontend files and production build (2,039 modules).
- Passed: actual browser screenshots and DOM geometry at 1400x900, 820x800
  and 600x800. At desktop Fit View, all nine heights were 112.1875 rendered
  pixels at approximately 93% graph zoom; both standard rows shared exact
  top and bottom coordinates, with uniform 12.1536px status inset.
- Passed: at 820px the heights were 69.4792px at approximately 58% zoom.
  At 600px the existing compact arrangement remained, with heights
  60.9545px at approximately 51% zoom. Height spreads stayed below 0.001px.
  All title content fit, all cards remained inside their groups, title borders
  were zero, heading clearance stayed positive and no cards overlapped.
- Passed: QA selection opened its inspector at desktop and narrow width;
  Home/Enter selected the orchestrator; drag changed viewport translation;
  zoom changed 93% to 112% and back; Fit View restored framing. The connector
  remained rendered. Activity event selection opened its corresponding
  inspector. Captured browser warning/error logs were empty.
- Not run: Rust suites, full verify, native build and historical browser
  harness acceptance; this extraction changes only frontend and documentation.
- Manual verification pending: native GUI and lifecycle controls, advisory.
- Environment setup: sandbox localhost binding returned EPERM. The approved
  unsandboxed launch succeeded; verification used only the candidate on port 1421. Ctrl-C stopped that server after checks. Temporary browser viewport
  override was reset and the task-created tab closed.
- Passed: formatting, documentation/link checks, repository policy, secret
  scanning, whitespace and session inventory; no conflicts or staged surprises.
- Passed: exact 22-path scope and byte-identical transfer of all 13 UI changes.
  All 879 recorded original-checkout paths retained their bytes or absence,
  and original HEAD/status matched the pre-transfer snapshot. Protected
  governance, harness, dependency, configuration and native paths are unchanged
  from remote main in the candidate. This scoped comparison does not claim
  recovery or preservation of previously unknown historical contents.

## Architecture findings

The source diff retains the single React Flow adapter and existing layout
mechanism. One agent height is used for positioning, rendering and group
bounds; orchestrator sizing remains independent. No new abstractions or
authority cross the React/native boundary. The product remains deterministic
fixture presentation, not a live or native-connected agent graph.

## Security findings

No native client, command, listener, permission, CSP, dependency, credential,
network, persistence, filesystem or execution path changed. Shared read-only
inspector and activity remain bounded fixture UI. Security scan passed.

## Code-health findings

Structured-only branches and types are removed while graph interaction tests
remain. The new sizing regression checks nine cards, unchanged widths,
independent orchestrator height, row coordinates, gap and container bottoms.
Actual browser geometry complements JSDOM coverage. Candidate frontend checks
passed without source repairs. Complete-diff review found no blocking defect;
all seven memory-file changes are additive and historical baseline content
is unchanged. The original UI patch is byte-identical in this candidate.

## Technical debt

Advisory: native GUI smoke is pending. Risk: browser results do not establish
native WebView behavior. Effort: Small. Owner: project owner. Milestone: native
demo walkthrough. Blocks completion: no. Blocks next PR review: no.
No new architecture or implementation debt was identified in the UI delta.

## Roadmap findings

The owner explicitly selected this publication task, superseding dated
Structured-retention instructions only within its scope. Unpublished
D-125/M1/M2 work remains parked and unaccepted. No live Personal Assistant
connection or operational successor is selected or authorized.

## Completion decision

PASS WITH ADVISORIES. All required automated and manual checks passed.
Native GUI smoke remains a non-blocking advisory. Finalization and Stop
must succeed before the authorized commit, push and PR creation. No merge.

## Next-increment readiness

Ready with advisories for PR review after passing candidate completion.
Merge requires separate owner approval and applicable passing remote checks.
This does not grant readiness to M1/M2 or live operational work.

## Exact files changed

- `CHANGELOG.md`
- `DECISIONS.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `ROADMAP.md`
- `docs/plans/2026-09-20-demo-cleanup-publication.md`
- `docs/reviews/2026-09-20-demo-cleanup-publication-post-increment-review.md`
- `src/features/command-center/CommandCenterPage.test.tsx`
- `src/features/command-center/CommandCenterPage.tsx`
- `src/features/command-center/command-center.css`
- `src/features/command-center/components/CommandCenterActivityStream.tsx`
- `src/features/command-center/components/CommandCenterHeader.tsx`
- `src/features/command-center/components/CommandCenterOverview.tsx`
- `src/features/command-center/components/ContextualInspector.tsx`
- `src/features/command-center/components/OperationalTopologyAdapter.test.tsx`
- `src/features/command-center/components/OperationalTopologyAdapter.tsx`
- `src/features/command-center/components/OperationalTopologyPanel.test.tsx`
- `src/features/command-center/components/OperationalTopologyPanel.tsx`
- `src/features/command-center/components/TopologyStructuredView.tsx`
- `src/features/command-center/useCommandCenterState.ts`

## Exact commands executed

Every required verification command in the manifest returned exit 0.
Documentation checks are rerun after freezing these observed results; no
product source changed after frontend/browser verification. The baseline command was
`npm run test:frontend -- src/features/command-center/components/OperationalTopologyAdapter.test.tsx src/features/command-center/components/OperationalTopologyPanel.test.tsx`
and returned exit 0 with 21 tests. Setup used
`npm ci --offline --no-audit --no-fund` (exit 0, 296 packages),
ordinary gate begin (exit 0), and a checked 13-file Git patch (exit 0).
The candidate server command was
`npm run dev -- --host 127.0.0.1 --port 1421 --strictPort`.
Its first sandbox launch returned exit 1 (EPERM); the approved launch served
the candidate successfully and later ended with intentional Ctrl-C, exit 130.
Browser actions and read-only geometry measurements ran through the existing
browser-control API; no visual harness or dependency was added.
