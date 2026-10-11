# Isolated action publication readiness review

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "isolated-action-publication-readiness",
  "quality_gate": "PASS WITH ADVISORIES",
  "next_increment_readiness": "Ready with advisories",
  "verification": [
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
    },
    {
      "command": "publication scope and preservation comparison",
      "required": true,
      "status": "Passed"
    }
  ],
  "manual_verification": [
    {
      "check": "Preservation and scope review",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Architecture, security, code-health and readiness review",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Unchanged product verification provenance review",
      "required": true,
      "status": "Passed"
    }
  ],
  "findings": [
    {
      "category": "Security",
      "severity": "Advisory",
      "summary": "Retain same-user receipt integrity and inherited isolated-action platform/runtime limitations.",
      "risk": "Receipt integrity does not authenticate a malicious same-user actor; bounded native evidence is not all-platform or all-timing acceptance.",
      "effort": "No expansion in this preparation.",
      "milestone": "Separately authorized publication review.",
      "blocks_completion": false,
      "blocks_next_increment": false
    }
  ],
  "files_changed": [
    "docs/reviews/2026-10-08-isolated-action-publication-readiness-post-increment-review.md"
  ],
  "commands_executed": [
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "python3 -B .codex/hooks/session_end_gate.py",
    "publication scope and preservation comparison"
  ]
}
-->

Date: 2026-10-08
Increment: `isolated-action-publication-readiness`
Branch: `codex/isolated-action-publication`

## Executive summary

Readiness preparation only; no candidate source carried before schema-2 admission.
Result: PASS WITH ADVISORIES. This is publication preparation, not a commit, push, PR, merge or publication authorization.

## Scope and boundaries

Required upstream main and this branch HEAD: `fc239b6161d9379915891e04f7c56fcaa9c4971c`.
Original candidate HEAD remains `bc12776412c717613de1fdc42c38bf3312493726` in `/Users/hdang/.codex/worktrees/live-provider-qa/ai-agent-assistant`.
Original frozen85 inventory: `/private/tmp/cortexa-isolated-action-final-39rz8nvq/completed-candidate.json`.
Carry58 unique paths byte-identically, reconcile eight shared documents additively,
and add only this readiness report and the reconciliation report:68 paths.
Exclude19 gate/policy/skill/template/workflow paths; preserve upstream bodies and
all other upstream files including D-134 plan/review. Original checkout, index,
links, complete/valid legacy gate, artifacts and historical receipts are immutable.

Use ordinary schema-2 D-134 admission for the separately authorized
`isolated-action-publication-reconciliation` objective. No copied private state,
legacy conversion, D-133 adoption or failed-criterion waiver. Original action
required acceptance passed; known D-125/M1/M2 failures are unrelated parked work,
not dependencies. Upstream D-133 is unchanged and unrelated. Historical Desktop
D-131 names retain provenance; D-134 controls this worktree. Decision-number
collisions remain qualified by checkout and dated plan rather than renumbered.

## Verification results

Fresh affected commands are listed in the manifest and saved with outputs and exit
statuses under `/private/tmp/cortexa-action-publication-prep-8cfs2y51`. Installed Python3.12 is used only for npm checker commands;
system Python `/usr/bin/python3 -B` handles schema/admission/session/Stop. Existing
installed Prettier is used via process-local PATH; no installs or symlinked cache.
No repeated product tests, builds or native QA. Original completion preservation
uses the existing E0 checker; new preservation verifies current Git68, copied58,
main-prefix/additive bodies, upstream exclusions and all other main inputs.

Inherited `npm run verify` at `/private/tmp/cortexa-qa-shape-1cv9b4i4/verify-host.json`
exited0,303.832s using the recorded offline Python3.12/Xcode SDK27/Cargo workaround:
145 hook,95 repository,612 frontend,491 Rust library and255 integration passes.
Library coverage repeated by integration is not summed twice. Separate11/11 actual
Docker opt-in checks are retained at `/private/tmp/cortexa-isolated-action-g_e2rnqv`.
This is reuse after input comparison, not a fresh full-suite exit0.
D-134 governance evidence is the unchanged upstream review and145-hook/94-repository
verification; its lower repository count excludes this candidate's action allowlist
regression. Product/artifact/input identities remain bound to original evidence.

## Architecture findings

PASS for reconciliation boundaries. No new runtime selection or device authority.
Trusted Rust still owns strict handoff/edit validation, Docker execution, native
approval, drift rejection, journals and cleanup. Frontend atlas is copied as a
historical source-bound walkthrough, not a new runtime or authority. D-134 policy
is retained without a parallel governance implementation.

## Security findings

PASS WITH ADVISORIES. No credential/profile inspection, provider requests, capability,
CSP, authentication, permission or model-to-device boundary change. Preserve the
pinned official Python image, nonroot/network-none/read-only/cap-drop/no-new-privileges
and resource bounds. No unrestricted execution, fallback or waived parser rules.
Original approval identity/hash/expiry and recoverable application evidence remain.
Local hash receipts are not malicious-same-user authentication.

## Code-health findings

PASS for byte-preserving carry and additive reconciliation; original focused tests
and final review remain frozen, not revalidated against a different historical Git
inventory. Existing product/source/test/dependency/config files are exact accepted
bytes. The eight shared documents preserve both histories with qualified labels;
old pending prose is historical and current facts are additive.

## Technical debt

Retain all inherited warnings and advisories: Vite chunk size, Node experimental
localStorage, opt-in Hermes skip, API-only UI scope copy in Codex, process-local native
workaround, runtime internal retry/remote cancellation limits and same-user receipts.
None is repaired or waived here. Owner: project owner; effort: separate bounded review
if selected; blocks this reconciliation: no. D-125/M1/M2 remain parked.

## Roadmap findings

Ready with advisories for the already-authorized admission and reconciliation.
No later product milestone, request budget or publication action is selected.
Original acceptance is reused: native Codex correction retained actual fail/pass,
Reject unchanged target, exact Approve/apply with bound journals/recovery, active
cancellation and subsequent recovery, drift block, interruption/restart without
replay. Direct API separately passed strict Coding/QA, actual Docker and review_ready
with unchanged target. Native observations, automated results and owner reports are
separate. Prior provider milestone and A/B recheck are inherited, not new acceptance.

Limits: supported small ordinary Git clones and new UTF8 root Python file only;
macOS native dialog evidence; Computer Use app-scoped, unclassified extra windows,
no CoreGraphics-ID-directed input or system-wide absence claim. Container isolation
does not defend against hostile host/kernel/daemon. Actual rollback restoration,
native Docker-active cancellation, API repetitions of shared lifecycle and all
provider/platform/timing variants remain untested. Historical discarded rejection
causes remain unknown. All required bounded criteria had distinct retained evidence.

## Completion decision

PASS WITH ADVISORIES; readiness does not issue a completion marker or grant publication.
Freeze this report and use supported schema-2 begin before carrying candidate files.

## Next-increment readiness

Ready with advisories. Continue the expressly authorized reconciliation after admission.
Ledger63/66;0 new requests. ECC hooks/MCP disabled; all advisories and parked work retained.

## Exact files changed

- `docs/reviews/2026-10-08-isolated-action-publication-readiness-post-increment-review.md`

## Exact commands executed

- `npm run docs:check`
- `npm run repository:check`
- `npm run security:scan`
- `git diff --check`
- `python3 -B .codex/hooks/session_end_gate.py`
- `publication scope and preservation comparison`

See `/private/tmp/cortexa-action-publication-prep-8cfs2y51` for actual output/exit receipts; inherited product commands are labeled
reuse above. Preserve failed historical commands; no repair, rerun or relabeling.
