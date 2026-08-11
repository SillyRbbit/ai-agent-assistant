# Hermes integration architecture assessment

Status: Complete with advisories
Owner: Project owner
Last updated: 2026-08-11

## Goal

Assess, from repository and official upstream evidence, whether Nous Research
Hermes Agent can become an optional experimental runtime without weakening or
replacing Cortexa's verified native boundaries.

## User-visible outcome

None. This is an architecture-documentation increment. It creates an
assessment, a Proposed ADR, and a separate Proposed ExecPlan for a later
native-only runtime-boundary increment.

## Scope

- Inspect current repository guidance, source, tests, configuration, manifests,
  workflows, mocks, and trust boundaries.
- Inspect official Hermes Agent source and documentation without installing or
  executing it.
- Create the requested assessment, Proposed ADR, and Proposed next-phase
  ExecPlan.
- Complete documentation-only validation and the repository completion gate.

## Explicit non-goals

- No Hermes installation, execution, dependency, credential, provider, or
  configuration.
- No production source, test, Tauri, UI, capability, permission, manifest,
  lockfile, runtime, or behavior change.
- No acceptance of the Proposed ADR and no execution of the next-phase plan.
- No OpenClaw selection or implementation.
- No modification of the pre-existing uncommitted project-direction
  documentation set.

## Existing behavior and constraints

- No `AgentRuntime`, `NativeAgentRuntime`, or `HermesAgentRuntime` exists.
- `InitialGatewayTurn` is a transport-free, test-covered Rust boundary and is
  not wired into the shipping Tauri command surface.
- The visible assistant loop is a deterministic frontend mock with no model,
  tool execution, persistence, or network access.
- D-030, D-032, D-033, and D-034 prohibit reviving removed generic audit,
  provider, memory, and platform scaffolds.
- D-078 permits only conceptual runtime-adapter planning and retains native
  Rust ownership of validation, policy, approval, restricted execution,
  cancellation, and audit.

## Current-state evidence

- Repository branch: `main` at `40f04b6` when the assessment began.
- The working tree already contained the separately completed, uncommitted
  project-direction documentation set. Those paths are outside this increment
  and must remain untouched.
- The gate `hermes-integration-assessment` is active.
- Official upstream inspection targets release `v2026.8.3`, package version
  `0.20.0`, release commit
  `3c27eb6234bf91b8ceee9e9071591b31e9b148cb`, plus current official docs only
  where explicitly identified.

## Files expected to change

- `docs/architecture/HERMES_INTEGRATION_ASSESSMENT.md`
- `docs/adr/ADR-MULTI-RUNTIME-AGENT-ARCHITECTURE.md`
- `docs/plans/2026-08-11-native-agent-runtime-boundary.md`
- `docs/plans/2026-08-11-hermes-integration-assessment.md`
- `docs/increments/hermes-integration-assessment.md`
- `docs/reviews/2026-08-11-hermes-integration-assessment-post-increment-review.md`

No pre-existing uncommitted path is edited.

## Affected components

Documentation and repository completion evidence only. The assessment discusses
the Rust agent, tool, policy, approval, audit, storage, credential, Tauri,
frontend mock, build, test, and CI boundaries without modifying them.

## Interfaces and invariants

- All runtime names remain proposed concepts.
- A runtime may emit only untrusted proposals and lifecycle events; it cannot
  grant authorization or execute application tools.
- Provider transport remains separate from runtime orchestration.
- The native path remains the default, reference, deterministic test path, and
  explicit fallback.
- External framework types do not cross the adapter boundary.
- The Proposed ADR cannot authorize implementation.

## Implementation milestones

- [x] Reconcile repository guidance, current state, and dirty-worktree scope.
- [x] Inspect current source, tests, manifests, capabilities, and workflows.
- [x] Inspect official Hermes release, architecture, protocols, configuration,
      tools, memory, skills, and security model.
- [x] Create the assessment, Proposed ADR, and Proposed next-phase ExecPlan.
- [x] Run documentation-only validation and close the active gate.

## Security and privacy considerations

The main question is whether an external Python process with provider, tool,
memory, skill, plugin, and execution features can be kept subordinate to
Cortexa's deterministic Rust governance. The assessment must enumerate bypass
paths, require an application-owned allowlist and protocol projection, keep
secrets outside WebView and ordinary logs, and treat process containment as a
separate prerequisite. No sensitive values are collected or recorded.

## Test plan

- Validate Markdown, links, repository policy, tracked secret patterns, and
  whitespace.
- Prove no protected production source, test, manifest, lockfile, Tauri
  configuration, capability, workflow, or dependency path changed in this
  increment.
- Inspect the complete diff and all pre-existing working-tree paths separately.
- No manual application check is required because no executable behavior or UI
  changes.

## Verification commands

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
python3 .codex/hooks/session_end_gate.py
python3 .codex/hooks/post_increment_gate.py status
```

## Risks

- Documentation could overstate a volatile upstream protocol as stable.
- A runtime abstraction could accidentally duplicate provider transport or
  revive D-032's removed arbitrary-string provider contract.
- Hermes's own approval/tool controls could be mistaken for Cortexa security
  boundaries.
- Existing user-authored documentation could be overwritten if scopes overlap.

## Rollback or failure strategy

Remove only the six new documentation/evidence files listed in this plan and
restore the prior completion marker through the repository gate workflow. Do
not touch pre-existing user changes. If upstream facts cannot be verified,
classify them as not verified rather than infer them.

## Decisions made

- Use the established root `PLANS.md` plus `docs/plans/` convention; do not
  create `.agent/PLANS.md` or another plan system.
- Treat the requested ADR as Proposed and the next-phase plan as Proposed.
- Keep the current assessment plan separate from the next-phase plan so the
  latter is not accidentally executed.

## Discoveries

- Hermes release `v2026.8.3` documents three external programmatic surfaces:
  ACP JSON-RPC over stdio, TUI-gateway JSON-RPC over stdio/WebSocket, and an
  HTTP/SSE API server.
- The TUI gateway exposes much more authority than Cortexa may delegate,
  including approval, secret, configuration, CLI, tool, and subagent methods.
- Hermes's security policy states that OS-level isolation is the load-bearing
  boundary against an adversarial LLM; in-process approval, redaction,
  scanners, and allowlists are heuristics.
- The release page reported 1,150 commits on `main` after the release during
  this assessment, making exact version pinning and contract fixtures
  mandatory for any future spike.

## Progress

- 2026-08-11: Active gate reconciled; repository and upstream read-only
  discovery completed.
- 2026-08-11: Assessment, Proposed ADR, Proposed native-only ExecPlan, and
  increment record completed. Documentation, repository, security, whitespace,
  protected-source, and session inventory checks passed; post-increment review
  classified the next implementation as Blocked.

## Acceptance criteria

- [x] The assessment contains all twelve requested sections and an
      evidence-backed recommendation.
- [x] The Proposed ADR contains every requested decision section and remains
      unaccepted.
- [x] The Proposed next-phase ExecPlan introduces only a native runtime
      boundary and explicitly excludes Hermes.
- [x] Official upstream source/version/license/protocol/configuration evidence
      is identified precisely.
- [x] No production code, dependency, configuration, UI, or behavior changes.
- [x] Documentation-only verification passes and the active marker is valid.

## Final results

The assessment concludes **CONDITIONAL GO** for preserving a future optional
Hermes path through a small application-owned runtime boundary. It does not
authorize Hermes. The preferred later mechanism is a version-pinned,
whole-process-contained managed subprocess with a closed TUI-gateway JSON-RPC
projection over stdio; the native Rust path remains default, reference,
deterministic-test, and explicit fallback.

All documentation-tier checks passed and the quality result is **PASS WITH
ADVISORIES**. The future native-only plan remains Blocked pending owner
acceptance of the Proposed ADR, reconciliation of the existing dirty
documentation set, and a fresh readiness review. No product source, test,
dependency, manifest, lockfile, Tauri configuration, UI, or behavior changed.

## Documentation updates

- [x] Assessment created.
- [x] Proposed ADR created.
- [x] Proposed next-phase ExecPlan created.
- [x] Increment record and completion review created.
- [x] Root project-memory files unchanged because their pre-existing
      uncommitted edits belong to the preceding increment and this assessment
      changes no current product capability or readiness.
