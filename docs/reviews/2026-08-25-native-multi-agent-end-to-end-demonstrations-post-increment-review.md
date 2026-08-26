# Native multi-agent end-to-end demonstrations post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "python3 .codex/hooks/post_increment_gate.py begin --increment native-multi-agent-end-to-end-demonstrations",
    "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::orchestrator::tests::agent_approval_source_resolution_retains_origin_without_execution",
    "cargo test --manifest-path src-tauri/Cargo.toml --test agent_memory_document_contract --locked selected_markdown_and_approved_shared_memory_flow_only_to_knowledge_and_synthesis",
    "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check",
    "npm run test:agent-acceptance",
    "npm run lint:rust",
    "npm run verify",
    "./node_modules/.bin/prettier --write ARCHITECTURE.md CHANGELOG.md DECISIONS.md SECURITY_CHECKLIST.md TESTING_GUIDE.md docs/design/NATIVE_MULTI_AGENT_COMMAND_CENTER_PROPOSAL.md docs/demos/NATIVE_MULTI_AGENT_ACCEPTANCE.md docs/demos/NATIVE_MULTI_AGENT_DEMONSTRATIONS.md docs/demos/NATIVE_MULTI_AGENT_FIXTURES.md docs/increments/native-multi-agent-end-to-end-demonstrations.md docs/plans/2026-08-11-multi-agent-end-to-end-demonstrations.md",
    "./node_modules/.bin/prettier --write ARCHITECTURE.md CHANGELOG.md DECISIONS.md HANDOFF.md NEXT_STEPS.md PLANS.md PROJECT_STATUS.md ROADMAP.md SECURITY_CHECKLIST.md TESTING_GUIDE.md docs/design/NATIVE_MULTI_AGENT_COMMAND_CENTER_PROPOSAL.md docs/demos/NATIVE_MULTI_AGENT_ACCEPTANCE.md docs/demos/NATIVE_MULTI_AGENT_DEMONSTRATIONS.md docs/demos/NATIVE_MULTI_AGENT_FIXTURES.md docs/increments/native-multi-agent-end-to-end-demonstrations.md docs/plans/2026-08-11-multi-agent-end-to-end-demonstrations.md docs/reviews/2026-08-25-native-multi-agent-end-to-end-demonstrations-post-increment-review.md docs/roadmap/NATIVE_MULTI_AGENT_ROADMAP.md",
    "./node_modules/.bin/prettier --write CHANGELOG.md docs/plans/2026-08-11-multi-agent-end-to-end-demonstrations.md docs/reviews/2026-08-25-native-multi-agent-end-to-end-demonstrations-post-increment-review.md",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "python3 .codex/hooks/session_end_gate.py",
    "python3 .codex/hooks/post_increment_gate.py finalize --increment native-multi-agent-end-to-end-demonstrations --report docs/reviews/2026-08-25-native-multi-agent-end-to-end-demonstrations-post-increment-review.md",
    "python3 .codex/hooks/post_increment_gate.py status"
  ],
  "files_changed": [
    "ARCHITECTURE.md",
    "CHANGELOG.md",
    "DECISIONS.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "ROADMAP.md",
    "SECURITY_CHECKLIST.md",
    "TESTING_GUIDE.md",
    "docs/design/NATIVE_MULTI_AGENT_COMMAND_CENTER_PROPOSAL.md",
    "docs/demos/NATIVE_MULTI_AGENT_ACCEPTANCE.md",
    "docs/demos/NATIVE_MULTI_AGENT_DEMONSTRATIONS.md",
    "docs/demos/NATIVE_MULTI_AGENT_FIXTURES.md",
    "docs/increments/native-multi-agent-end-to-end-demonstrations.md",
    "docs/plans/2026-08-11-multi-agent-end-to-end-demonstrations.md",
    "docs/reviews/2026-08-25-native-multi-agent-end-to-end-demonstrations-post-increment-review.md",
    "docs/roadmap/NATIVE_MULTI_AGENT_ROADMAP.md",
    "package.json",
    "src-tauri/src/agent/orchestrator.rs",
    "src-tauri/tests/agent_memory_document_contract.rs"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Architecture",
      "effort": "High",
      "milestone": "Separate owner-approved decision and ExecPlan before any approval-to-dispatch implementation",
      "risk": "The Demo 7 checkpoint-to-approval-to-manual-execution chain is absent. D-093 accepts the two proved branches separately for this deterministic increment, but future combined-chain claims or implementation would require a separate owner-approved decision and ExecPlan.",
      "severity": "Advisory",
      "summary": "E2E-TD-01: Workflow Automation has no combined approval-to-manual-dispatch chain."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Technical debt",
      "effort": "Medium",
      "milestone": "Before a live provider reuses the generic runtime-start path",
      "risk": "A pre-existing legacy start_runtime_run cancellation-error path may drop a rejected runtime run without quarantine; a future external provider run could remain live and unowned.",
      "severity": "Medium",
      "summary": "LEGACY-TD-01: resolve rejected-run cleanup before live or external-runtime work."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Technical debt",
      "effort": "Medium",
      "milestone": "Before another workflow or lifecycle family",
      "risk": "Large private orchestrator and bounded-parallel lifecycle modules increase review coupling if another workflow or concurrency family is added.",
      "severity": "Low",
      "summary": "D091-TD-01: decompose private lifecycle ownership before another workflow expansion."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Architecture",
      "effort": "Medium",
      "milestone": "Before backend-connected approval UI or IPC",
      "risk": "Approval rejection is task-bound, audited, and NotAttempted, but the root remains Running and no denial text is injected; a connected caller must deliberately surface and resolve that state.",
      "severity": "Advisory",
      "summary": "E2E-TD-02: approval rejection is not task-terminal runtime feedback."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Architecture",
      "effort": "Low",
      "milestone": "Before any target-neutral acceptance claim",
      "risk": "A non-macOS run omits the target-gated approval unit and cannot independently reproduce the target-macOS 249/447 matrix; the evidence documents disclose this limit.",
      "severity": "Low",
      "summary": "E2E-TD-03: the complete acceptance count is macOS-specific."
    }
  ],
  "increment_id": "native-multi-agent-end-to-end-demonstrations",
  "manual_verification": [
    {
      "check": "Independent architecture and code-health review of the complete diff",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Independent security and governance review of the complete diff",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Manual application check (not required because backend workflows remain unwired and no Tauri or React behavior changed)",
      "required": false,
      "status": "Not run"
    }
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::orchestrator::tests::agent_approval_source_resolution_retains_origin_without_execution",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --test agent_memory_document_contract --locked selected_markdown_and_approved_shared_memory_flow_only_to_knowledge_and_synthesis",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run test:agent-acceptance",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run lint:rust",
      "required": true,
      "status": "Passed"
    },
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
      "command": "python3 .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    }
  ]
}
-->

Date: 2026-08-25
Increment: Native multi-agent end-to-end demonstrations
Branch: `codex/native-multi-agent-end-to-end-demonstrations`

## Executive summary

The bounded acceptance layer and test-only evidence changes are implemented.
All twelve demonstrations pass under the owner-approved acceptance scope. For
Demo 7, D-093 accepts the non-executable checkpoint-denial and manual safe A-D
fixture-dispatch branches as separate evidence. D-090 still prevents a
checkpoint proposal from issuing dispatch authority; no combined chain or
approval-to-dispatch bridge exists or is claimed.

All required automated checks pass, and independent review found no production
regression. The quality-gate result is `PASS WITH ADVISORIES`. The absent Demo 7
combined chain remains an explicit architecture advisory and requires a
separate future decision and ExecPlan before any implementation or capability
claim. Gate finalization completed, and live status reported `complete` with a
valid workspace fingerprint.

## Scope and boundaries

The implementation adds one local acceptance alias and strengthens only two
existing test paths: full rejected-approval audit attribution and a
Knowledge-context shared-memory proposal that remains pending and absent from
Personal synthesis. The remaining changes are deterministic demo, fixture,
acceptance, plan, review, and current-state documentation.

The exact 21-path inventory stays within the living plan. No production agent,
workflow, runtime, provider, model, task authority, policy, approval manager,
tool, executor, IPC/UI path, dependency, capability, permission, persistence,
network, device effect, commit, or push was added.

## Verification results

- Focused approval rejection: 1 passed, 0 failed, 248 filtered out.
- Focused approved-document/shared-proposal contract: 1 passed, 0 failed,
  9 filtered out.
- Canonical target-macOS acceptance: 249 Rust library units plus 198 tests in
  ten public contracts, 447 passed, 0 failed, 0 ignored.
- Complete `npm run verify`: formatting and repository health passed; strict
  frontend and Rust lint passed; 28 hook tests, 38 repository-workflow tests,
  211 frontend tests across 13 files, 249 Rust library tests, and 232 Rust
  integration tests passed; one intentional opt-in Hermes probe was ignored;
  typecheck, Vite production build, and Tauri release no-bundle build passed.
- Final documentation formatting/link, repository-health, secret-pattern,
  Rust-format/Clippy, session-end, and diff checks passed.
- The post-increment finalizer accepted this `PASS WITH ADVISORIES` report; live
  status returned `complete` and `valid: true`.
- Target evidence is macOS 26.6 build 25G72 on Apple silicon (`arm64`). The
  approval unit is macOS-gated, so another target cannot reproduce the complete
  249/447 matrix by itself.
- No backend manual application check was required because no connected UI or
  production behavior changed. Existing Command Center M5 evidence remains
  separate frontend-fixture validation.

## Architecture findings

The production architecture review passes. `AgentOrchestrator` remains the
only task creator; specialists cannot spawn; Workflow Automation cannot execute
or self-dispatch; QA cannot approve; Security cannot authorize; and Native
remains sole/default and unwired. No module ownership or trust boundary moved.

D-093 resolves the acceptance classification without changing architecture.
`MAX_WORKFLOW_EXECUTABLE_TOOL_STEPS` remains zero; checkpoint-containing
proposals fail closed without a token, while the manual A-D token path remains
separate. The absent combined sequence is an advisory, not a production defect
or authority path that this increment may silently repair.

## Security findings

The changed implementation paths pass security review. Unknown and ineligible
tools fail closed, execution stays `NotAttempted`, destructive cloud/system
capabilities remain inert, task/run identity substitution fails closed, and
memory/document proposals gain no silent approval or durable persistence. The
rejection audit now binds the exact action, policy, full live root attribution,
`Rejected`, `NotAttempted`, and no error.

No secret, credential, production document, private repository, cloud account,
host, provider call, governed-tool execution, or consequential external-system
effect participates. Cargo is locked but not offline and may fetch locked
crates on a cold cache; the document test creates and cleans one bounded local
temporary fixture.

## Code-health findings

The new npm alias is explicit and reproducible. Test assertions are typed,
content-bounded, and attached to existing scenario contracts. The Knowledge
proposal is transparently a deterministic application test value submitted
through the real Knowledge-context API, not parsed from mock output. Documents
accurately distinguish `MockAgentRuntime`, the unwired native wrapper, real
governed boundaries, simulated operating-system results, and unavailable live
capabilities.

No active-diff correctness, type-safety, error-handling, duplication,
accessibility, dead-code, or production maintainability defect remains. The
missing Demo 7 combined chain is disclosed as a future architecture advisory.

## Technical debt

The machine manifest records five non-completion-blocking findings.
`E2E-TD-01` preserves the absent Demo 7 combined-chain advisory under D-093.
`LEGACY-TD-01` and `D091-TD-01` are preserved pre-existing risks that block
live-provider or new lifecycle expansion, not this test-only patch.
`E2E-TD-02` records the disclosed nonterminal approval-feedback semantics, and
`E2E-TD-03` records the target-macOS count limit. The patch introduces no new
production-path debt.

## Roadmap findings

Phase 8's deterministic Command Center prototype is validated, but live UI/IPC
remains Blocked. Phase 9's deterministic architecture acceptance passes under
D-093, subject to this final gate. No owner-selected successor is Ready.
Provider, tool, execution, durable audit/memory, backend-connected UI, and
Hermes work remain Blocked.

## Completion decision

`PASS WITH ADVISORIES`. Automated validation, all twelve owner-approved
demonstrations, and the implemented-architecture acceptance criteria pass. The
absent Demo 7 combined chain, nonterminal approval feedback, target-specific
count, and preserved legacy risks remain explicit; none grants authority or
blocks this deterministic acceptance increment. The completion marker is
complete and fingerprint-valid.

## Next-increment readiness

`Blocked`. No successor plan is owner-selected or Ready. Any future combined
approval-to-manual-dispatch capability requires a separate project-owner
decision, bounded ExecPlan, and readiness review. Do not implement a bridge,
begin another gate, commit, or push automatically.

## Exact files changed

The machine manifest contains the complete 21-path tracked/untracked inventory.

## Exact commands executed

The machine manifest records the required commands and actual outcomes. The
initial `npm run verify` attempt stopped at four Markdown formatting findings;
the reported files were formatted and the final-source rerun passed completely.
All final verification rows are `Passed`; gate finalization and the subsequent
status check also passed.
