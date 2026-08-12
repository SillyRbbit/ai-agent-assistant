# Native multi-agent architecture documentation

Status: Verified complete with advisories; uncommitted for owner review
Verification date: 2026-08-12
Gate ID: `native-multi-agent-architecture`
Plan: `docs/plans/2026-08-11-native-multi-agent-architecture.md`
Baseline: clean synchronized `main` at `641ccac`

## Goal

Accept an evidence-backed application-owned native multi-agent direction above
the existing one-run runtime foundation, record the phased roadmap, and ready
only the definition/registry implementation plan without changing application
behavior.

## Boundaries

- Preserve the implemented `AgentRuntime`, sole/default
  `NativeAgentRuntime`, unchanged `InitialGatewayTurn`, trust boundaries, and
  deterministic mocks.
- Keep `AgentOrchestrator`, task lifecycle, delegation, governance, memory, and
  UI outside the runtime and unimplemented.
- Keep the initial Personal-to-Research depth, total-child budget, and active
  concurrency at one; only the orchestrator may create tasks. Later workflows
  stay depth one and require exact finite stage budgets.
- Preserve every Hermes assessment, ADR, spike, fixture, increment, review, and
  NO-GO conclusion.
- Add no production/test source, dependency, provider, process, Tauri/React
  path, permission, or external action.
- Leave the completed documentation uncommitted for owner review.

## Result

D-082 and the Accepted native multi-agent ADR place an application-owned
`AgentOrchestrator` above the existing runtime. Agents are application-owned
privilege-free definitions; tools, policy, approvals, execution, audit, memory,
providers, credentials, and device authority remain separate.

The authoritative root roadmap retains ten gated phases and the subordinate
catalog roadmap expands their staged work. The sole Ready plan covers nine closed
IDs/definitions, versioned embedded instructions, non-authorizing activation,
a deterministic immutable registry, validation, redaction, and tests. Only
Personal Assistant and Research are initially selected for a future flow; all
nine remain inert. Every later plan remains Blocked.

Hermes integration is **Deferred — evaluated transport and containment
requirements not met**. The exact evaluated `0.20.0` / `v2026.8.3` raw stdio,
managed WebSocket, and ACP rejections remain preserved without generalizing to
every future Hermes release.

No production or test source, dependency, manifest, lockfile, configuration,
workflow, hook, skill, provider, process, IPC, UI, or behavior changed. Native
remains sole/default and unwired to a live application flow.

## Verification

- Documentation formatting and internal links: Passed.
- Repository structure and current-memory consistency: Passed.
- Secret/security scan: Passed.
- Whitespace/error diff check: Passed.
- Protected production/test/dependency/configuration/workflow/hook/skill path
  assertion: Passed.
- Session-end inventory: Passed with the exact documentation-only scope.
- Architecture/security/code-health/technical-debt/readiness review: Passed
  with the publication advisory below.
- Manual application check: not required because no application path changed.

## Advisory

The AgentDefinition/AgentRegistry plan is Ready and not Active. It may start
only after this intentionally uncommitted documentation increment is reviewed,
committed, and pushed to a clean synchronized baseline and the owner provides a
separate exact implementation task. This publication advisory blocks starting
the next increment, not the plan's Ready status or safe completion of this
documentation scope.

## Rollback

Before publication, restore/remove only this increment's declared documentation
paths. After publication, use one bounded revert plus an additive superseding
decision. Preserve D-078 through D-081 and every Hermes evidence artifact. No
source, data, dependency, migration, credential, process, or external state
requires rollback.
