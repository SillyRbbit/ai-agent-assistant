# Hermes runtime architecture decisions

Status: Complete with advisories; Phase 2 intentionally remains uncommitted
Owner: Project owner
Last updated: 2026-08-11

## Goal

Record the owner's accepted application-owned multi-runtime architecture and
conditional `hermes serve` WebSocket evaluation direction, make the native
runtime boundary the next Ready implementation plan, and define the blocked
Hermes spike and adapter phases without changing production behavior.

## User-visible outcome

None. Cortexa keeps its existing deterministic frontend mock and verified
native Rust behavior. No runtime, provider, process, socket, credential, model,
tool, dependency, or UI is added or activated.

## Scope

- Mark the multi-runtime ADR Accepted with native-preservation invariants.
- Add an accepted, conditional transport ADR for managed local `hermes serve`
  plus the documented TUI-gateway JSON-RPC/WebSocket surface.
- Record the decisions durably after D-078.
- Make the native-only AgentRuntime plan Ready and add an explicit deterministic
  `MockAgentRuntime` contract-test fixture.
- Create a contained `hermes serve` WebSocket spike plan blocked on verified
  native-runtime completion.
- Create a Hermes adapter plan blocked on the native boundary and a passing
  contained spike.
- Add superseding notices to the historical assessment and raw-stdio spike,
  then synchronize architecture and current project memory.

## Explicit non-goals

- No production source, test fixture, executable hook, workflow, configuration,
  manifest, lockfile, or dependency change.
- No Hermes installation, update, import, execution, process, socket, HTTP,
  WebSocket, provider request, model call, credential, or external action.
- No `AgentRuntime`, `NativeAgentRuntime`, `MockAgentRuntime`, or
  `HermesAgentRuntime` implementation.
- No ACP implementation or spike.
- No tool, memory, skill, plugin, MCP, subagent, scheduling, messaging, shell,
  filesystem, Git, cloud, clipboard, browser, or device capability.
- No branch change, Phase 2 commit, push, PR, merge, release, or publication.

## Existing behavior and constraints

- The committed Phase 1 baseline is `b44b988`, and its completed raw-transport
  ADR revision remains preserved as historical evidence.
- No runtime trait, adapter, selector, live provider, or production coordinator
  exists. `InitialGatewayTurn` remains test-consumed and transport-free.
- The raw TUI-gateway stdio spike is complete and NO-GO for the evaluated
  release. Its result must not be rewritten.
- As of 2026-08-11, the latest official source release inspected is Hermes Agent
  package/application version `0.20.0`, calendar release tag `v2026.8.3`, at
  commit `3c27eb6234bf91b8ceee9e9071591b31e9b148cb`. These are distinct
  identifiers for the same tagged source artifact.
- The owner accepts the runtime target but authorizes documentation only in this
  increment. A Ready plan still grants no implementation authority by itself.

## Current-state evidence

- `docs/PROJECT_DIRECTION.md` and D-078 preserve a conceptual application-owned
  runtime seam and native default/reference/fallback behavior.
- `docs/architecture/HERMES_INTEGRATION_ASSESSMENT.md` inventories the current
  native boundaries and external-runtime risks.
- `docs/spikes/HERMES_TRANSPORT_SPIKE.md` rejects raw TUI-gateway stdio and
  recommends either a supported public surface or native-only behavior.
- Official pinned upstream documentation identifies `hermes serve` as the
  headless backend used by Hermes Desktop and the TUI gateway as JSON-RPC over
  WebSocket. `hermes_cli/web_server.py` mounts and authenticates `/api/ws` and
  delegates to `tui_gateway.ws.handle_ws`; the surface requires strict
  containment and projection.
- Pinned-source review also found lazy dependency installation, an import-time
  update-check path, POSIX `[pty]` alongside `[web]`, background maintenance and
  change-watcher activity, cooperative rather than terminal interrupt
  acknowledgement, a human-oriented `session.status.output`, mandatory
  `session.info`/`message.start` events, active dotenv/managed-secret resolution,
  and workers that may detach with `start_new_session=True`. The accepted
  transport ADR and future plans treat each as an explicit compatibility or
  NO-GO boundary rather than an implementation assumption.
- The current native runtime plan already defines a narrow, dependency-free,
  behavior-preserving boundary but was Blocked on the owner decision.

## Files expected to change

- `docs/plans/2026-08-11-hermes-runtime-architecture-decisions.md`
- `docs/adr/ADR-MULTI-RUNTIME-AGENT-ARCHITECTURE.md`
- `docs/adr/ADR-HERMES-SERVE-WEBSOCKET-TRANSPORT.md`
- `DECISIONS.md`
- `docs/plans/2026-08-11-native-agent-runtime-boundary.md`
- `docs/plans/2026-08-11-hermes-serve-websocket-spike.md`
- `docs/plans/2026-08-11-hermes-agent-runtime-adapter.md`
- `docs/architecture/HERMES_INTEGRATION_ASSESSMENT.md`
- `docs/spikes/HERMES_TRANSPORT_SPIKE.md`
- `ARCHITECTURE.md`
- `PLANS.md`
- `HANDOFF.md`
- `PROJECT_STATUS.md`
- `NEXT_STEPS.md`
- `CHANGELOG.md`
- `docs/reviews/2026-08-11-hermes-runtime-architecture-decisions-post-increment-review.md`

Any production, test, dependency, configuration, workflow, hook, skill, or
additional documentation path requires owner approval before editing.

## Affected components

- Architecture decisions and current target-architecture documentation.
- Future implementation and spike ExecPlans.
- Current project-memory and completion evidence.

No application, runtime, platform, provider, storage, credential, or security
component changes.

## Interfaces and invariants

- Accepted target: `AgentRuntime -> NativeAgentRuntime | HermesAgentRuntime`.
- The types remain unimplemented. Acceptance is architecture authority, not
  current capability or permission to begin a later phase automatically.
- Native remains default, reference, deterministic contract-test path, explicit
  fallback, and potential standalone runtime. Fallback means a new explicit
  selection, never automatic failover or replay.
- The native implementation must compose verified current boundaries and may
  not revive D-032's deleted provider API or D-030/D-033/D-034 scaffolds.
- Raw TUI-gateway stdio remains rejected for Hermes Agent package/application
  version `0.20.0`, release tag `v2026.8.3`.
- Managed local `hermes serve` plus TUI-gateway JSON-RPC/WebSocket is selected
  only for a contained evaluation after native completion.
- ACP remains deferred and unimplemented.
- All Hermes-specific process, protocol, configuration, compatibility, and
  error types remain adapter-local. No raw JSON-RPC reaches application
  services, Tauri IPC, or React.
- Deterministic Rust retains validation, policy, approval, restricted
  execution, cancellation truth, audit, credentials, and device authority.
- The Hermes spike must use a numeric-loopback listener, an isolated runtime home and working
  directory, a sanitized environment, the exact private token-query WebSocket,
  closed protocol/event allowlists, and whole-process containment. It must prove
  complete-distribution provenance, preinstalled `[web]`/POSIX `[pty]`, no lazy
  install/update side effect, denial of every pinned dotenv/managed-secret
  source, exact fake-provider-only egress, Unix-socket denial, and cleanup of
  detached descendants. Tool or privileged activity is a failure.

## Implementation milestones

- [x] Milestone 0 - publish the prior completed ADR-revision increment and
      confirm a clean synchronized `b44b988` baseline.
- [x] Milestone 1 - accept the multi-runtime and conditional transport decisions
      with canonical pinned upstream identifiers.
- [x] Milestone 2 - ready the native plan and create the blocked spike and
      adapter plans with exact prerequisites, lifecycle, tests, and rollback.
- [x] Milestone 3 - add historical superseding notices and synchronize
      architecture, queue, status, handoff, and changelog.
- [x] Milestone 4 - run documentation validation, reviews, session-end, and the
      post-increment completion gate without committing Phase 2.

## Security and privacy considerations

This increment handles no runtime process, network, credential, personal data,
or secret. Its decisions keep Hermes untrusted and require OS-level
whole-process containment because configuration and method allowlists are not a
security boundary. The future spike must prove loopback binding, token secrecy,
environment and state isolation, capability denial, bounded parsing, redaction,
cancellation, crash handling, and descendant cleanup before an adapter can be
proposed Ready.

## Test plan

- Validate Markdown format, internal links, repository structure, and secrets.
- Manually verify both ADR statuses and D-079/D-080 references.
- Verify exactly one implementation plan is Ready: the native boundary.
- Verify the WebSocket spike is Blocked on native completion and the adapter is
  Blocked on both native completion and a passing spike.
- Verify the raw-stdio report remains NO-GO and receives only an additive
  superseding notice.
- Prove no production, test, dependency, manifest, lockfile, configuration,
  workflow, hook, or skill path changed.

## Verification commands

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
python3 .codex/hooks/session_end_gate.py
python3 .codex/hooks/post_increment_gate.py status
```

No application build, frontend test, Rust test, Tauri validation, or manual
application check is required because this is a documentation-only increment.

## Risks

| Risk                                                     | Severity | Mitigation                                                                                                         |
| -------------------------------------------------------- | -------- | ------------------------------------------------------------------------------------------------------------------ |
| Accepted ADR is mistaken for current implementation      | High     | State current absence in both ADRs, architecture, plans, and project memory                                        |
| Conditional transport becomes implicit adapter approval  | High     | Keep the spike and adapter Blocked with separate evidence gates                                                    |
| `hermes serve` broad surface bypasses Cortexa governance | High     | Require OS containment, strict allowlists, backend-only token, and forbidden-event failure                         |
| Version labels are treated as conflicting releases       | Medium   | Record package version, calendar tag, and commit as separate identifiers for one artifact                          |
| Native plan becomes speculative or rewrites current code | High     | Derive the interface from current call sites, compose `InitialGatewayTurn`, and use a test-only deterministic mock |
| Historical raw-stdio evidence is erased                  | Medium   | Add superseding notices only; retain the report, tests, and NO-GO verdict unchanged                                |
| Pinned launch side effects or detached workers escape    | High     | Block spike GO on read-only/offline preflight, endpoint containment, and detached-descendant cleanup evidence      |

Any unresolved High finding blocks completion.

## Rollback or failure strategy

Revert only the 16 documentation paths in this plan. D-078, the committed raw-
stdio spike and Phase 1 ADR revision, source, tests, dependencies, runtime
behavior, and external state remain unchanged. If the documents cannot keep the
transport conditional or the phases independently gated, leave the ADRs
unaccepted and the native plan Blocked.

## Decisions made

- 2026-08-11: The owner accepted the application-owned multi-runtime target and
  native-preservation invariants.
- 2026-08-11: The owner rejected raw TUI-gateway stdio for production and
  conditionally selected managed local `hermes serve` plus TUI-gateway
  JSON-RPC/WebSocket for a contained spike after native completion.
- 2026-08-11: ACP remains a documented deferred fallback.

## Discoveries

- `0.20.0` is the package/application version; `v2026.8.3` is the calendar
  release tag; commit `3c27eb6234bf91b8ceee9e9071591b31e9b148cb` is their
  immutable source revision.
- The pinned `hermes serve` command is a supported public headless backend, but
  headless mode retains a broad authenticated HTTP/WebSocket agent surface.
- `/api/health` is liveness/version evidence, while `gateway.ready` is protocol
  connection readiness; neither proves provider or capability safety.
- WebSocket authentication uses the `?token=` query populated from
  `HERMES_DASHBOARD_SESSION_TOKEN`; `session.interrupt` is only a cooperative
  acknowledgement and must be followed by bounded status reconciliation.
- `session.status` exposes activity through the pinned human-oriented
  `Agent Running: Yes|No` line rather than a structured field, and a normal turn
  emits `session.info` plus `message.start`; fixtures must model those facts and
  fail closed on drift.
- The pinned launcher can attempt lazy installation/update work, and its worker
  can create a new session. A read-only distribution and containment-wide
  membership/termination proof are therefore spike prerequisites.
- The pinned environment loader checks distribution-root, isolated-home,
  machine-managed, and configured external secret sources. Their explicit
  absence/denial is required; a sanitized inherited environment is insufficient.

## Progress

- 2026-08-11: Phase 1 was corrected, revalidated, committed at `b44b988`, and
  pushed to `origin/main`; Phase 2 began from a clean synchronized baseline.
- 2026-08-11: Re-read the current architecture evidence and verified the pinned
  official upstream release and transport documentation. No Hermes command was
  executed.
- 2026-08-11: Pinned-source review corrected route ownership, token-query
  authentication, POSIX extras, cancellation semantics, install/update side
  effects, dotenv/managed-secret loading, status-text parsing, mandatory
  asynchronous events, endpoint containment, and detached-descendant cleanup
  requirements.
- 2026-08-11: Synchronized all 16 documentation paths, completed required
  checks and reviews, and retained Phase 2 as an uncommitted owner-reviewable
  working-tree scope.

## Acceptance criteria

- [x] Both architecture decisions are Accepted and recorded durably.
- [x] Canonical Hermes version, tag, commit, and upstream sources are consistent.
- [x] Raw TUI-gateway stdio remains rejected and its historical spike unchanged
      except for an additive superseding notice.
- [x] The native runtime plan is Ready and explicitly includes a deterministic
      no-I/O, failure-capable `MockAgentRuntime` contract fixture.
- [x] The WebSocket spike is Blocked until verified native completion.
- [x] The Hermes adapter is Draft/Blocked until native and spike prerequisites
      pass and tool execution remains disabled.
- [x] Current-state documentation does not claim any runtime implementation.
- [x] No production source, test, dependency, configuration, or external state
      changes.
- [x] Documentation validation and required completion workflow pass.

## Final results

PASS WITH ADVISORIES. D-079 and D-080 are Accepted, the native-only boundary
plan is Ready for a separately authorized later run, the contained WebSocket
spike remains Blocked on verified native completion and fresh review, and the
Hermes adapter remains Draft/Blocked on both phases. Official evaluated source
is package/application version `0.20.0`, calendar tag `v2026.8.3`, commit
`3c27eb6234bf91b8ceee9e9071591b31e9b148cb`. Raw TUI-gateway stdio remains
NO-GO. No production source, test, dependency, configuration, Hermes execution,
process, socket, credential, provider, UI, or behavior changed. Phase 2 is
intentionally uncommitted and must be reviewed/published separately before the
Ready implementation can begin.

## Documentation updates

- [x] `HANDOFF.md`
- [x] `PROJECT_STATUS.md`
- [x] `NEXT_STEPS.md`
- [x] `DECISIONS.md`
- [x] `CHANGELOG.md`
- [x] `TROUBLESHOOTING_LOG.md`, not required; no durable failure needed a new
      entry
