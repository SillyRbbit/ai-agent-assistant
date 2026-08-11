# Hermes integration assessment post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": ["npm run docs:check", "npm run repository:check", "npm run security:scan", "git diff --check", "git diff --exit-code -- src src-tauri package.json package-lock.json src-tauri/Cargo.toml src-tauri/Cargo.lock .github", "python3 .codex/hooks/session_end_gate.py"],
  "files_changed": ["AGENTS.md", "ARCHITECTURE.md", "CHANGELOG.md", "DECISIONS.md", "HANDOFF.md", "NEXT_STEPS.md", "PLANS.md", "PROJECT_STATUS.md", "docs/PROJECT_DIRECTION.md", "docs/adr/ADR-MULTI-RUNTIME-AGENT-ARCHITECTURE.md", "docs/architecture/HERMES_INTEGRATION_ASSESSMENT.md", "docs/increments/hermes-integration-assessment.md", "docs/increments/repository-project-direction-runtime-boundaries.md", "docs/plans/2026-08-11-hermes-integration-assessment.md", "docs/plans/2026-08-11-native-agent-runtime-boundary.md", "docs/plans/2026-08-11-project-direction-runtime-boundaries.md", "docs/reviews/2026-08-11-apple-support-ts-017-owner-contact-d077-contact-1-post-increment-review.md", "docs/reviews/2026-08-11-hermes-integration-assessment-post-increment-review.md", "docs/reviews/2026-08-11-repository-project-direction-runtime-boundaries-post-increment-review.md", "docs/templates/INCREMENT_TEMPLATE.md"],
  "findings": [{"blocks_completion": false, "blocks_next_increment": true, "category": "Roadmap", "effort": "owner review of the Proposed ADR, an additive accepted decision, a reconciled working tree, and a fresh readiness review", "milestone": "before the native runtime-boundary plan can become Active", "risk": "a Proposed architecture document could be mistaken for implementation authority even though no runtime boundary exists and the prior project-memory work remains uncommitted", "severity": "High", "summary": "The proposed native-only runtime-boundary increment is Blocked."}, {"blocks_completion": false, "blocks_next_increment": true, "category": "Security", "effort": "separate native-boundary, fake-process, pinned-containment, protocol, credentials, packaging, and target-platform increments", "milestone": "before any Hermes installation, execution, dependency, adapter, or product activation", "risk": "an ordinary Hermes process can route models, execute tools, load privileged Python, persist memory, and access inherited user authority outside Cortexa's deterministic Rust gates", "severity": "High", "summary": "Hermes integration remains blocked on provenance, containment, protocol, lifecycle, secrets, and packaging evidence."}],
  "increment_id": "hermes-integration-assessment",
  "manual_verification": [{"check": "The assessment contains all twelve requested sections, the ADR contains every requested section and remains Proposed, and the next-phase ExecPlan excludes Hermes.", "required": true, "status": "Passed"}, {"check": "Official upstream release, tagged source and documentation, license, protocols, security model, and the GitHub/PyPI version mismatch were inspected without installing or executing upstream code.", "required": true, "status": "Passed"}, {"check": "Current, mocked, planned, prohibited, and not-verified behavior are distinguished, and the complete change set adds no product source, dependency, configuration, permission, UI, or runtime behavior.", "required": true, "status": "Passed"}],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [{"command": "npm run docs:check", "required": true, "status": "Passed"}, {"command": "npm run repository:check", "required": true, "status": "Passed"}, {"command": "npm run security:scan", "required": true, "status": "Passed"}, {"command": "git diff --check", "required": true, "status": "Passed"}, {"command": "git diff --exit-code -- src src-tauri package.json package-lock.json src-tauri/Cargo.toml src-tauri/Cargo.lock .github", "required": true, "status": "Passed"}, {"command": "python3 .codex/hooks/session_end_gate.py", "required": true, "status": "Passed"}]
}
-->

Date: 2026-08-11
Increment: hermes-integration-assessment
Branch: main

## Executive summary

The documentation-only assessment is complete. It recommends **CONDITIONAL
GO** for preserving an optional future Hermes path through a small,
application-owned runtime boundary, while explicitly withholding approval for
Hermes installation or integration. The Proposed ADR remains unaccepted and
the future native-only plan remains unexecuted. **PASS WITH ADVISORIES**.

No production source, test, dependency, manifest, lockfile, configuration,
permission, UI, runtime behavior, provider, process, or credential changed.

## Scope and boundaries

The authorized increment created an evidence-based architecture assessment, a
Proposed ADR, a Proposed next-phase native-only ExecPlan, its current assessment
plan, an increment record, and this review. It used read-only repository and
official upstream research. It did not install or execute Hermes, add a
dependency, accept the ADR, or begin the future plan.

Fourteen paths from the separately completed project-direction increment were
already dirty when this assessment began. The gate requires their inclusion in
the complete working-tree inventory below; they remained outside this
increment's six-file scope and were not edited here. Because root project-memory
files are among those overlapping paths, this assessment did not rewrite them.

## Verification results

Markdown formatting and links, repository health, secret-pattern scanning,
whitespace, protected-source exclusion, and the deterministic session inventory
passed. The session inventory reports no conflicts and no staged paths. Manual
review confirms the requested structure, official upstream provenance, and the
absence of executable or behavioral changes.

No manual application check was required or run because this increment changes
no application code, UI, configuration, or runtime behavior.

## Architecture findings

`$architecture-review`: no completion-blocking finding.

- The assessment describes the actual split between the test-only Rust
  `InitialGatewayTurn` and the shipping deterministic React mock; it does not
  overclaim a current native runtime.
- A future `AgentRuntime` is conditional, narrow, and limited to descriptor,
  start, closed untrusted events, and cancellation. Provider transport,
  governance, tools, execution, audit, memory, platform, and secrets remain
  separate.
- `NativeAgentRuntime` would compose rather than replace the verified native
  path. `HermesAgentRuntime` remains optional, experimental, and blocked.
- D-032's deleted synchronous arbitrary-string provider API is not revived.
- The preferred future subprocess mechanism keeps Hermes-specific process,
  protocol, configuration, and errors inside one adapter.

## Security findings

`$security-review`: no completion-blocking finding; future Hermes activation is
blocked by a High advisory.

No Tauri IPC, capability, CSP, hook, approval, policy, audit, storage,
filesystem, operating-system, network, credential, secret, or permission
boundary changed. The assessment treats Hermes output as untrusted and retains
deterministic Rust authority. It identifies direct tool execution, broad RPC,
MCP, provider fallback, inherited environment/home, global `~/.hermes`,
plugins, skills, hooks, memory, logs, secrets, subprocess descendants, network
listeners, and self-update as plausible bypass paths.

Upstream states that OS-level isolation is the load-bearing boundary against an
adversarial LLM. Therefore no Hermes phase may proceed without whole-process
containment, a dedicated empty runtime home, no inherited secrets, disabled
privileged features, a closed method/event allowlist, and adversarial lifecycle
evidence.

## Code-health findings

`$code-review`: no unresolved finding. The review found and removed one
duplicated comparison row before final validation. The new documents use
current/planned/prohibited language consistently, link authoritative source
material, preserve accepted decisions, and do not claim a release package and
PyPI package are interchangeable. Prettier, link validation, repository health,
secret scanning, whitespace, and protected-path checks pass.

## Technical debt

`$technical-debt`: None introduced. No implementation abstraction, dependency,
process supervisor, compatibility shim, feature flag, dead code, or deferred
runtime failure was added. The Proposed ADR's unresolved process, protocol,
packaging, and security work is future scope and a readiness blocker, not debt
created by this documentation increment.

## Roadmap findings

`$readiness-review`: **Blocked**.

The proposed `docs/plans/2026-08-11-native-agent-runtime-boundary.md` is bounded
and has explicit files, invariants, risks, verification, rollback, and non-goals,
but it cannot become Active until the project owner accepts or amends the
Proposed ADR through an additive decision, reconciles the existing dirty
working tree, selects the increment, and obtains a fresh readiness result.
Hermes work remains further blocked on all containment, protocol, provenance,
lifecycle, credential, packaging, and target-platform evidence enumerated in
the assessment.

## Completion decision

**PASS WITH ADVISORIES**

The advisories block subsequent implementation, not completion of the requested
read-only architecture assessment.

## Next-increment readiness

**Blocked** — the smallest next action is owner review of the Proposed ADR. Do
not execute the native-only plan or any Hermes phase from these documents.

## Exact files changed

Current assessment increment:

- `docs/adr/ADR-MULTI-RUNTIME-AGENT-ARCHITECTURE.md`
- `docs/architecture/HERMES_INTEGRATION_ASSESSMENT.md`
- `docs/increments/hermes-integration-assessment.md`
- `docs/plans/2026-08-11-hermes-integration-assessment.md`
- `docs/plans/2026-08-11-native-agent-runtime-boundary.md`
- `docs/reviews/2026-08-11-hermes-integration-assessment-post-increment-review.md`

Preserved pre-existing paths required in the complete Git inventory:

- `AGENTS.md`
- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `DECISIONS.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `docs/PROJECT_DIRECTION.md`
- `docs/increments/repository-project-direction-runtime-boundaries.md`
- `docs/plans/2026-08-11-project-direction-runtime-boundaries.md`
- `docs/reviews/2026-08-11-apple-support-ts-017-owner-contact-d077-contact-1-post-increment-review.md`
- `docs/reviews/2026-08-11-repository-project-direction-runtime-boundaries-post-increment-review.md`
- `docs/templates/INCREMENT_TEMPLATE.md`

## Exact commands executed

- `npm run docs:check` — passed.
- `npm run repository:check` — passed.
- `npm run security:scan` — passed.
- `git diff --check` — passed.
- `git diff --exit-code -- src src-tauri package.json package-lock.json src-tauri/Cargo.toml src-tauri/Cargo.lock .github` — passed.
- `python3 .codex/hooks/session_end_gate.py` — passed with no conflicts.
