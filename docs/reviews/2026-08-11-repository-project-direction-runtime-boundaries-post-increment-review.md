# Repository project direction and runtime boundaries post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": ["npm run docs:check", "npm run repository:check", "npm run security:scan", "git diff --check", "git diff --exit-code -- src src-tauri package.json package-lock.json src-tauri/Cargo.toml src-tauri/Cargo.lock", "python3 .codex/hooks/session_end_gate.py", "test \"$(wc -l < AGENTS.md | tr -d ' ')\" -le 160"],
  "files_changed": ["AGENTS.md", "ARCHITECTURE.md", "CHANGELOG.md", "DECISIONS.md", "HANDOFF.md", "NEXT_STEPS.md", "PLANS.md", "PROJECT_STATUS.md", "docs/PROJECT_DIRECTION.md", "docs/increments/repository-project-direction-runtime-boundaries.md", "docs/plans/2026-08-11-project-direction-runtime-boundaries.md", "docs/reviews/2026-08-11-apple-support-ts-017-owner-contact-d077-contact-1-post-increment-review.md", "docs/reviews/2026-08-11-repository-project-direction-runtime-boundaries-post-increment-review.md", "docs/templates/INCREMENT_TEMPLATE.md"],
  "findings": [{"blocks_completion": false, "blocks_next_increment": true, "category": "Roadmap", "effort": "a separate owner-selected bounded plan, readiness review, and explicit implementation approval", "milestone": "before any Hermes, OpenClaw, provider, runtime, deployment, or product implementation", "risk": "the new direction could be mistaken for implementation readiness even though no runtime seam, adapter, transport, coordinator, dispatcher, or executor exists", "severity": "High", "summary": "Project direction is complete, but every external-runtime and product implementation remains Blocked."}],
  "increment_id": "repository-project-direction-runtime-boundaries",
  "manual_verification": [{"check": "Current, mocked, planned, and prohibited behavior remain explicitly distinguished.", "required": true, "status": "Passed"}, {"check": "The complete diff changes no production source, dependency, configuration, permission, or runtime behavior.", "required": true, "status": "Passed"}, {"check": "The pre-existing D-077 contact-1 orphan-gate review remained outside the new increment scope and was not modified.", "required": true, "status": "Passed"}],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [{"command": "npm run docs:check", "required": true, "status": "Passed"}, {"command": "npm run repository:check", "required": true, "status": "Passed"}, {"command": "npm run security:scan", "required": true, "status": "Passed"}, {"command": "git diff --check", "required": true, "status": "Passed"}, {"command": "git diff --exit-code -- src src-tauri package.json package-lock.json src-tauri/Cargo.toml src-tauri/Cargo.lock", "required": true, "status": "Passed"}, {"command": "python3 .codex/hooks/session_end_gate.py", "required": true, "status": "Passed"}, {"command": "test \"$(wc -l < AGENTS.md | tr -d ' ')\" -le 160", "required": true, "status": "Passed"}]
}
-->

Date: 2026-08-11
Increment: repository-project-direction-runtime-boundaries
Branch: main

## Executive summary

The documentation-only project-direction increment establishes Cortexa's
present owner-only personal scope, the requested guiding principle, native
architecture preservation, a conceptual framework-neutral runtime-adapter
direction, and a complete living ExecPlan convention. **PASS WITH ADVISORIES**.

No `AgentRuntime`, `NativeAgentRuntime`, `HermesAgentRuntime`, or OpenClaw
adapter was implemented. No product source, dependency, configuration,
permission, runtime behavior, external action, or current capability changed.

## Scope and boundaries

The increment creates project direction, a dated plan, an increment record, and
this review. It updates concise root instructions, planned architecture, D-078,
the living plan convention and template, and required current project-memory
records. It preserves accepted future consumer, cloud, provider, enterprise,
signing, and release targets without implementing or canceling them.

The separately completed
`docs/reviews/2026-08-11-apple-support-ts-017-owner-contact-d077-contact-1-post-increment-review.md`
was present when this gate began. It is listed because the validator requires
the complete working-tree inventory, but it remained unmodified and outside
this increment's scope.

## Verification results

Documentation formatting and links, repository health, secret scanning,
whitespace, protected-source exclusion, concise-root size, and session inventory
checks passed. The inventory reports no merge conflicts. Manual review confirms
that current, mocked, planned, and prohibited states remain distinct and that no
production path changed.

## Architecture findings

`$architecture-review`: no unresolved finding. The review confirmed that the
runtime names are conceptual, the verified native path is preserved without
reviving deleted scaffolds, the future runtime seam stays distinct from
D-032's deleted provider API and future provider transport, and external
framework output remains untrusted. Deterministic Rust retains validation,
policy, approval, cancellation, restricted-execution, and audit authority.

The review initially identified premature completion wording while the gate was
active. Live records were returned to Active/verification-pending wording before
this report and marker step.

## Security findings

`$security-review`: no finding. No IPC, Tauri capability, CSP, hook, credential,
secret, Keychain, storage, network, filesystem, model-data, approval, policy,
audit, operating-system, deployment, or permission boundary changed. Unknown
tools, privileged actions, framework events, identities, and states remain
fail-closed. Hermes types remain adapter-local in the planned direction.

## Code-health findings

`$code-review`: no unresolved finding. The root instructions remain concise,
links pass, and current-state claims are consistent. The review initially found
that the increment template omitted an explicit Risks heading; the template was
corrected before final verification. No source, test, dependency, manifest,
lockfile, or generated artifact changed.

## Technical debt

None introduced. The prior orphan-gate review remains preserved as separate
pre-existing documentation evidence. D-032's deleted provider scaffold and the
other intentionally removed generic scaffolds remain deleted.

## Roadmap findings

`$readiness-review`: **Blocked**. D-078 is direction and documentation evidence,
not implementation authority. No runtime seam or adapter exists, OpenClaw is not
selected, and no Hermes, provider, transport, coordinator, dispatcher, executor,
multi-agent, deployment, or product increment is Ready. A later task requires a
separate owner-selected bounded plan, readiness review, and explicit approval.

## Completion decision

**PASS WITH ADVISORIES**

The only remaining High advisory blocks the next implementation, not this
documentation closeout: project direction must not be mistaken for runtime or
product readiness.

## Next-increment readiness

**Blocked** — no product or external-runtime implementation is Ready. The owner
may separately select another bounded documentation task under existing
governance.

## Exact files changed

Current increment:

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
- `docs/reviews/2026-08-11-repository-project-direction-runtime-boundaries-post-increment-review.md`
- `docs/templates/INCREMENT_TEMPLATE.md`

Preserved pre-existing path required in the complete Git inventory:

- `docs/reviews/2026-08-11-apple-support-ts-017-owner-contact-d077-contact-1-post-increment-review.md`

## Exact commands executed

- `npm run docs:check` — passed.
- `npm run repository:check` — passed.
- `npm run security:scan` — passed.
- `git diff --check` — passed.
- `git diff --exit-code -- src src-tauri package.json package-lock.json src-tauri/Cargo.toml src-tauri/Cargo.lock` — passed.
- `python3 .codex/hooks/session_end_gate.py` — passed with no conflicts.
- `test "$(wc -l < AGENTS.md | tr -d ' ')" -le 160` — passed.
