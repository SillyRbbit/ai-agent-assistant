# Hermes ADR transport revision

Status: Complete
Owner: Project owner
Last updated: 2026-08-11

## Goal

Revise the Proposed multi-runtime ADR after the completed Hermes transport spike
disproved its raw TUI-gateway stdio premise. Keep the ADR transport-neutral and
non-authorizing while documenting the evidence required before any future
runtime or Hermes-adapter implementation can be selected.

## User-visible outcome

No application behavior changes. Repository guidance accurately states that
Hermes raw TUI-gateway stdio is not a supported production transport at the
evaluated release, and that ACP, `hermes serve`, and native-only remain
unselected options pending separate evidence and an owner decision.

## Scope

- Revise `docs/adr/ADR-MULTI-RUNTIME-AGENT-ARCHITECTURE.md` without changing
  its Proposed status or accepting a transport.
- Record the active documentation-only increment and its current evidence in
  the plan and project-memory records.
- Add only evidence-backed documentation about the required decision sequence,
  transport alternatives, isolation, provenance, containment, and rollback.

## Explicit non-goals

- No `AgentRuntime`, `NativeAgentRuntime`, or `HermesAgentRuntime` source.
- No Hermes execution, installation, dependency, process, socket, credential,
  gateway, IPC, UI, runtime selection, provider, tool, MCP, memory, skill,
  subagent, scheduling, messaging, or file capability.
- No transport selection, ADR acceptance, `DECISIONS.md` change, or revision of
  D-078.
- No modification to verified native Rust boundaries, deterministic mocks,
  tests, manifests, lockfiles, or application behavior.

## Existing behavior and constraints

- `AgentRuntime`, `NativeAgentRuntime`, and `HermesAgentRuntime` remain
  conceptual only under D-078.
- The completed `hermes-transport-spike` gate is valid with
  `PASS WITH ADVISORIES`; its raw TUI-gateway stdio result is NO-GO for supported
  production use at Hermes Agent `0.20.0` / `v2026.8.3`.
- The ADR remains Proposed and grants no implementation authority. ACP and
  `hermes serve` must not be selected by inference.
- Native is the future default/reference/explicit-fallback direction; fallback
  never means automatic failover.

## Current-state evidence

- `PLANS.md` recorded no active plan before this increment.
- `docs/spikes/HERMES_TRANSPORT_SPIKE.md` documents no public raw stdio
  launcher, no initial version/capability negotiation, and no gateway-shutdown
  RPC for the evaluated release.
- `docs/adr/ADR-MULTI-RUNTIME-AGENT-ARCHITECTURE.md` is Proposed and explicitly
  requires revision before acceptance.
- Current Rust contains no runtime seam or Hermes production integration.

## Files expected to change

- `docs/plans/2026-08-11-hermes-adr-transport-revision.md`
- `docs/adr/ADR-MULTI-RUNTIME-AGENT-ARCHITECTURE.md`
- `PLANS.md`
- `HANDOFF.md`
- `PROJECT_STATUS.md`
- `NEXT_STEPS.md`
- `CHANGELOG.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/reviews/2026-08-11-hermes-adr-transport-revision-post-increment-review.md`

## Affected components

- Architecture-decision documentation and project-memory records only.
- No Rust, React, Tauri, dependency, runtime, or production-security component.

## Interfaces and invariants

- The ADR remains `Status: Proposed`; no document treats it as implementation
  authority.
- Raw TUI-gateway stdio remains rejected for production at the evaluated
  version. ACP and `hermes serve` remain unselected.
- Any future external runtime remains untrusted and adapter-local; deterministic
  Rust retains validation, policy, exact approval, restricted execution,
  cancellation, audit, credentials, and platform authority.
- Native remains conceptual default/reference/explicit fallback; no automatic
  runtime or provider failover is authorized.

## Implementation milestones

- [x] Confirm clean baseline, completed spike marker, current ADR status, and
      absence of runtime source.
- [x] Revise the Proposed ADR to remove the failed raw-stdio preference and
      state the evidence gates for each unselected alternative.
- [x] Synchronize plan indexes, current status, handoff, next steps, changelog,
      and applicable troubleshooting evidence.
- [x] Run documentation checks and required completion workflow.

## Security and privacy considerations

No external runtime, process, network, credential, or personal data is used.
The ADR must continue to require immutable provenance, whole-process OS
containment, restricted capabilities, bounded protocol translation, redaction,
and explicit cancellation/termination evidence before any Hermes adapter can be
considered.

## Test plan

- Verify Markdown and repository documentation consistency.
- Verify no protected source, manifest, lockfile, configuration, or test asset
  changed.
- Review the ADR for a retained Proposed status, no silent transport selection,
  and no implementation authorization.

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

- Wording could accidentally imply that ACP or `hermes serve` is selected or
  safe without an independent spike.
- Wording could erase the raw-stdio negative result or overstate fixture-only
  evidence.
- Project-memory updates could drift from the Proposed ADR or valid prior spike
  marker.

## Rollback or failure strategy

Revert only this increment's documentation paths. The completed spike, D-078,
native boundaries, source, dependencies, and valid prior gate marker remain
unchanged. If the ADR wording cannot remain transport-neutral, stop with the
ADR Proposed and leave all runtime work blocked.

## Decisions made

- 2026-08-11: The project owner selected the recorded documentation-only ADR
  revision as the next task. This increment does not select a transport or
  accept the ADR.

## Discoveries

- The repository gate state directory is protected from ordinary sandboxed
  writes even though its Unix mode is writable; the required gate began only
  after an explicitly approved elevated invocation. This is local execution
  environment evidence, not a product or security-control change.

## Progress

- 2026-08-11: Confirmed clean `main` at `a153e77`, the valid completed
  `hermes-transport-spike` marker, the Proposed ADR, and no runtime source.
- 2026-08-11: Began gate `hermes-adr-transport-revision` after the selected
  documentation-only task was confirmed.
- 2026-08-11: Revised the ADR and synchronized project memory without selecting
  a Hermes transport or changing runtime code. Documentation, repository, and
  secret checks passed; the review result is PASS WITH ADVISORIES because no
  runtime increment is Ready.
- 2026-08-11: During the owner-authorized precommit review, corrected an
  accidental literal `+` before the TS-018 Markdown heading and reran the
  documentation, repository, secret, whitespace, session-end, and completion
  checks before publication.

## Acceptance criteria

- [x] The ADR remains Proposed and no longer identifies raw TUI-gateway stdio
      as a preferred production transport.
- [x] ACP, `hermes serve`, and native-only are described as unselected paths
      with explicit evidence and authorization gates.
- [x] No production source, dependency, runtime configuration, or application
      behavior changes.
- [x] Project-memory records describe the exact next decision task and do not
      mark runtime implementation Ready.
- [x] Documentation validation and the post-increment workflow pass with a
      valid completion marker.

## Final results

The ADR now rejects raw TUI-gateway stdio as a supported production transport
for the evaluated Hermes release and leaves native-only, Hermes ACP, and Hermes
serve unselected. No runtime source, dependency, configuration, process, UI,
credential, or external action changed. The quality result is PASS WITH
ADVISORIES because the next runtime increment remains Blocked pending an
owner-selected architecture decision, a separately bounded plan, and fresh
readiness evidence.

## Documentation updates

- [x] `HANDOFF.md`
- [x] `PROJECT_STATUS.md`
- [x] `NEXT_STEPS.md`
- [x] `DECISIONS.md`, not required because the ADR remains Proposed
- [x] `CHANGELOG.md`
- [x] `TROUBLESHOOTING_LOG.md`
