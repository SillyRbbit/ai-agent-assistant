# Bounded agent parallelism

Status: Verified complete with advisories
Owner: Project owner
Decision: D-091
Gate: `agent-bounded-parallelism`
Last updated: 2026-08-13
Roadmap:
[`NATIVE_MULTI_AGENT_ROADMAP.md`](../roadmap/NATIVE_MULTI_AGENT_ROADMAP.md)

## Goal

Add one application-owned `BoundedParallel` selector to `AgentOrchestrator`
that can keep a finite set of independent depth-one specialist runtime runs
active at the same time, accept their events through one same-thread
multiplexed boundary, collect their results in application-owned ordinal order,
and perform truthful dependent or Personal Assistant synthesis.

This is bounded in-process orchestration above the unchanged `AgentRuntime` and
sole/default `NativeAgentRuntime`. It is not a general workflow engine,
provider-concurrency implementation, thread pool, scheduler, worker system, or
distributed queue.

## User-visible outcome

None. This remains an unwired deterministic Rust application-service contract.
It adds no Tauri command, React consumer, provider, live model, repository or
platform operation, filesystem/network access, persistence, or device effect.

## Selected scenarios

The selector accepts only three immutable application-owned scenario IDs and
their synthetic fixture catalogs:

1. `ResearchKnowledgeIndependentV1`: Research and Knowledge perform independent
   fixture analyses under `ContinuePartial`; Personal Assistant then
   synthesizes both ordered outcomes.
2. `CodeSecurityQaV1`: Coding and Security independently review one synthetic
   proposal; after both terminalize, QA validates their ordered transfers under
   `CancelDependentOnly`; Personal Assistant then synthesizes all three slots.
3. `CloudSystemsSecurityV1`: Cloud Infrastructure and Systems Operations
   independently assess separate synthetic environment facets; after both
   succeed, Security reviews their ordered transfers. A specialist failure,
   cancellation, or timeout applies `FailFast` to the remaining specialist
   lane, skips Security, and still permits a truthful partial Personal
   synthesis.

These are new sealed fixture-only/no-I/O scenarios. They do not reinterpret,
parallelize, compose, or change the already verified D-086, D-087, D-088, or
D-090 selectors. Workflow Automation is not part of the selector and cannot
create a live task.

## Scope

- Add a framework-neutral bounded-parallel domain with closed scenario, work
  item, dependency, failure-policy, result-status, event, attribution, audit,
  synthesis, limit, cancellation-handle, and typed-error contracts.
- Add one immutable application catalog containing the three exact graphs,
  fixture inputs, stage ordinals, dependency requirements, expected outputs,
  and failure policies above.
- Add one private `AgentOrchestrator` child module that owns admission, the
  finite pending-slot queue, active-run multiplexing, deterministic result
  collection, dependency release, deadline checks, cancellation, cleanup, and
  final Personal synthesis.
- Add `AgentWorkflowSelection::BoundedParallel` as a mutually exclusive sealed
  selector. Keep every prior selector and its one-active-child limits
  unchanged.
- Permit only the orchestrator to create root children. Every specialist
  remains depth one and may neither delegate nor invoke this selector.
- Give each admitted child a unique task ID, exact agent definition/profile,
  `AgentExecutionContext`, `RuntimeRunIdentity`, run-owned output buffer,
  application cancellation handle, deadline, ordinal, and isolated task-memory
  identity. No child shares mutable execution context, runtime run, output,
  cancellation state, or task-memory key.
- Accept runtime events only when the exact live task/run binding, expected
  sequence, per-run cap, global cap, workflow deadline, and child deadline all
  validate. Event calls may be interleaved by task ID on the same application
  thread; completion timing never selects result order.
- Enter every sealed slot into the bounded pending set in catalog order and
  emit `Queued` exactly once, including immediately admitted and dependency-
  blocked slots. A slot receives task/run identity only on `Started`
  admission; invalid requests fail before the pending set is created.
- Preserve strict versioned JSON parsing, unknown-field denial, fixture
  provenance, bounded framing, exact application-derived agent attribution,
  and truthful complete/partial synthesis. No hidden reasoning is requested or
  exposed.
- Use an injected monotonic clock for deterministic timeout tests. Deadline
  enforcement is cooperative at trusted application boundaries and never
  claims hard preemption of a synchronous `runtime.start` or `accept_event`
  call already executing.

## Explicit non-goals

- No generic workflow runner, DAG engine, dynamic workflow definition, DSL,
  recursive workflow, nested selector, specialist spawning, replenishing child
  budget, unbounded fan-out, autonomous loop, arbitrary code, shell text,
  callback, or self-modifying workflow.
- No OS thread, async runtime, task executor, process, provider-concurrency
  change, `AgentRuntime`/`RuntimeRun` trait change, runtime fallback, remote
  worker, Redis, RabbitMQ, Kafka, Kubernetes, cloud queue, distributed lock,
  leader election, high availability, or distributed orchestration.
- No background queue service, scheduler, cron, recurring/startup execution,
  remote trigger, webhook, persistence, recovery-after-restart, or durable
  workflow state.
- No tool request or execution, `ToolRegistry` change, policy permission,
  approval request/dispatch, general `AuditLogger`, repository effect,
  infrastructure/system effect, external communication, credential access,
  filesystem/network/platform I/O, dependency, manifest, lockfile, or
  configuration change.
- No provider, live model, IPC, UI, Tauri capability/permission, Codex, Hermes,
  OpenClaw, SaaS, multi-user, deployment, distribution, or public-release work.
- No activation or generic delegation widening. All involved specialists are
  already `Initial` only for existing sealed fixture workflows and become
  eligible here only through this exact application selector.
- No claim of simultaneous CPU work, real provider concurrency, hard timeout
  preemption, separate provider sessions, or production load behavior.

## Pre-implementation baseline and readiness evidence

- Before this planning-only change, Git was clean and synchronized on `main`
  and `origin/main` at full commit
  `1f85264eef3b3a3640b52d2f452ef572f7cb2b91`.
- At that baseline,
  `python3 .codex/hooks/post_increment_gate.py status` reported
  `agent-workflow-automation-proposals` complete with `PASS WITH ADVISORIES`
  and `valid: true`. The later planning diff may invalidate its live workspace
  fingerprint without changing the recorded clean published-tree evidence.
- D-083 supplies the closed task state machine, trusted execution context,
  root/child lineage, run attribution, cancellation, and the rule that only
  `AgentOrchestrator` creates a depth-one child.
- D-084 supplies sealed agent policy-profile attribution. D-085 supplies one
  workflow-local `MemoryStore` keyed by task identity and terminal cleanup.
  Research uses `ResearchWorkingMemoryV1`; Knowledge uses
  `KnowledgeWorkingMemoryV1`; Coding, QA, Security, Cloud, and Systems use
  `MemoryDisabledV1`.
- D-086, D-087, and D-088 prove strict fixture-only specialist result and
  Personal synthesis validation. D-089 gives new lifecycle code a private
  orchestrator child-module home. D-090 is complete and does not grant
  Workflow Automation task authority.
- `AgentRuntime::start` and `RuntimeRun::accept_event` are synchronous. No
  provider or provider session exists. The selected model therefore keeps
  multiple returned `RuntimeRun` values active in the orchestrator and
  multiplexes subsequent events by exact task/run identity on one thread.
- Fresh readiness, architecture, and security review on 2026-08-13 found the
  exact plan Ready with advisories: cooperative deadlines cannot interrupt a
  currently executing synchronous runtime call, and the unwired fixture path
  is not operational concurrency evidence. Neither advisory blocks this
  bounded proof.
- The owner selected D-091 through the exact bounded-parallelism implementation
  prompt. This plan records that selection but does not itself begin the gate
  or authorize scope outside the files and boundaries below.

## Exact files expected to change

Production Rust:

- `src-tauri/src/agent/bounded_parallelism.rs` (new public closed domain)
- `src-tauri/src/agent/bounded_parallelism/catalog.rs` (new private immutable
  scenario and fixture catalog)
- `src-tauri/src/agent/bounded_parallelism/validation.rs` (new private strict
  parser and cross-stage/synthesis validation)
- `src-tauri/src/agent/orchestrator/bounded_parallel_workflow.rs` (new private
  admission, multiplexing, dependency, failure, cancellation, and cleanup
  lifecycle)
- `src-tauri/src/agent/orchestrator.rs` (facade wiring, mutually exclusive
  selector, task/run maps, cancellation routing, and fixed limits only)
- `src-tauri/src/agent/task.rs` (add only the typed child-deadline failure code;
  do not widen the generic task status or outcome unions)
- `src-tauri/src/agent/infrastructure_operations/framing.rs` (add only the
  exhaustive `DeadlineExceeded` match arm, projecting it through the existing
  unavailable/runtime-failed path; D-088 never emits it and no D-088 contract,
  lifecycle, or wire schema changes)
- `src-tauri/src/agent/mod.rs`

Tests:

- `src-tauri/tests/agent_bounded_parallelism_contract.rs` (new public contract)
- `src-tauri/tests/agent_orchestration_contract.rs`
- `src-tauri/tests/agent_runtime_contract.rs`
- `src-tauri/tests/support/mock_agent_runtime.rs` (add bounded live-run,
  maximum-live, terminal-disposition, nonterminal-drop, and recoverable
  cancellation-failure instrumentation)

Planning and closeout documentation:

- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `DECISIONS.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PRODUCT_REQUIREMENTS.md`
- `PROJECT_STATUS.md`
- `ROADMAP.md`
- `SECURITY.md`
- `SECURITY_CHECKLIST.md`
- `docs/PROJECT_DIRECTION.md`
- `docs/adr/ADR-NATIVE-MULTI-AGENT-ARCHITECTURE.md`
- `docs/architecture/NATIVE_MULTI_AGENT_ASSESSMENT.md`
- `docs/security/NATIVE_AGENT_GOVERNANCE_MATRIX.md`
- `docs/increments/agent-bounded-parallelism.md` (new at closeout)
- `docs/reviews/2026-08-13-agent-bounded-parallelism-post-increment-review.md`
  (new at closeout)
- `docs/roadmap/NATIVE_MULTI_AGENT_ROADMAP.md`
- this plan

No runtime-trait, native-runtime implementation, provider, tool, governance,
approval, memory-store, document, storage, frontend, Tauri, configuration,
dependency, manifest, lockfile, CI, hook, skill, or external-integration file
is in scope. An unexpected need to change one stops the increment for owner
review.

## Interfaces and invariants

### Closed concurrency and resource limits

The D-091 selector owns these exact constants; no model/runtime field may
raise, replenish, or reinterpret them:

| Limit                                      | Exact value |
| ------------------------------------------ | ----------- |
| Delegation depth                           | 1           |
| Default active specialist children         | 2           |
| Hard active specialist children            | 3           |
| Total specialist children per root         | 3           |
| Total tasks including Personal root        | 4           |
| Runtime-run attempts including synthesis   | 5           |
| Automatic retries                          | 0           |
| Accepted runtime events per run            | 8           |
| Runtime and generic orchestration events   | 32 each     |
| Parallel workflow events and audit records | 32 each     |
| Root workflow duration                     | 120 seconds |
| Child duration after admission             | 60 seconds  |

The public sealed selector uses the default active limit of two. A narrow
trusted application constructor may select one through three before any
parallel work is accepted; tests exercise the hard cap of three. Values above
three, zero, post-start changes, model-supplied values, and a fourth child fail
closed. A child deadline is `min(child_start + 60s, root_deadline)`; time spent
queued consumes the root lease but not the child lease. Every later Personal
run remains bounded by the root lease.

All counters are checked with fallible arithmetic before mutation. Terminal
children do not replenish the total-child, task, run, retry, event, workflow,
or audit budgets. An admitted active slot may free active capacity for an
already-counted pending slot; that is admission, not budget replenishment.

### Content and framing limits

- Objective: 1,024 scalars/4,096 bytes. Catalog: eight fixtures, each 2,048
  scalars/4,096 bytes, with 16,384 aggregate serialized bytes. Specialist raw
  result: 6,144 scalars/12,288 bytes.
- Each child result has at most three findings and one unresolved issue; each
  text is 256 scalars/1,024 bytes and their aggregate text is 4,096 bytes. One
  serialized child transfer is 6,144 bytes; the ordinal transfer has at most
  three entries/18,432 aggregate bytes.
- Fixed stage/synthesis framing is at most 2,048 bytes, so maximum specialist
  selected text is 22,528 bytes and objective + maximum transfer + synthesis
  framing is at most 24,576 selected-text bytes. Final synthesis raw
  output is 4,096 scalars/8,192 bytes; its exact status table has at most three
  entries/2,048 serialized bytes, summary is 1,024 scalars/2,048 bytes, and at
  most three application-derived unresolved fields are 322 scalars/1,090
  bytes after the bounded work-item prefix is added.
- Parsers enforce scalar, byte, list, and aggregate limits before allocation or
  state mutation. Maximum quote/backslash/newline and multibyte fixtures prove
  each stage/synthesis `RuntimeTurnRequest`, including application instructions
  and JSON escaping, remains at or below Native's unchanged 65,536-byte encoded
  gateway-request limit. N and N+1 tests cover every bound. A terminal success
  is not accepted until its candidate result, ordered transfer, successor input,
  and final status table pass this prepared-input preflight.

### Admission, queueing, and duplicate prevention

- The immutable catalog assigns unique `ParallelWorkItemId` values and stable
  ordinals. Dependency references must point to an earlier known work item,
  and scenario construction validates no self-edge, cycle, duplicate logical
  request, duplicate agent/role slot, or unknown output reference.
- Pending slots use a bounded in-memory ordinal set inside the selected root;
  dependency-blocked/capacity-waiting slots remain planned, and eligible slots
  admit immediately. There is no worker, timer, or application-wide queue.
- Admission checks exact root attribution, selector exclusivity, scenario ID,
  target activation, source role, depth, active count, total count, task/run
  capacity, event capacity, dependency state, root deadline, and duplicate key
  before task creation or `runtime.start`.
- Capacity reached below the hard limit produces `Queued` for an already
  cataloged slot. A configured/hard/total/task/run violation produces a typed
  rejection. Nothing silently exceeds a limit.
- A specialist execution context is never accepted as the source of scenario
  selection, admission, dependency release, or synthesis. Workflow Automation
  likewise has no entry point.

### Same-thread run multiplexing and isolation

- `AgentRuntime` and `RuntimeRun` remain unchanged. Calls are synchronous and
  no thread or async task is created. Parallelism means multiple live run
  values are retained simultaneously and their later closed events may be
  delivered in any interleaving through an exact task/run-addressed method.
- Each active entry owns its task, run, bounded output, next expected sequence,
  ordinal, child deadline, status, and non-serializable process-local
  cancellation handle. A handle is validated against the exact orchestrator
  instance, root, task, and live run; a runtime/model result cannot construct
  or redirect it.
- Every child receives its own `AgentExecutionContext`, policy-profile binding,
  exact memory-profile binding, task key, and `RuntimeRunIdentity`. Scenario A
  may use only existing live-grant-governed per-agent/task namespaces: sibling
  reads fail, task-temporary data is cleaned at terminal state, and memory is
  never copied into results/synthesis, auto-written, proposed, or promoted to
  shared memory. Scenario B/C specialist profiles are disabled and reject
  writes. D-091 changes no production memory-store behavior. There is no
  provider-session registry; distinct runtime-run identity is the complete
  supported session boundary.
- Events with an unknown/terminal task, wrong run identity, stale sequence,
  late deadline, excess count, or post-cancellation state are rejected. One
  child's output/event state cannot mutate another child.
- Terminalization removes the run, cleans task-temporary memory and document
  descriptors through existing cleanup, records exactly one outcome, and then
  admits or skips only application-selected successors. Root completion
  requires zero active runs, zero pending task objects, and no unresolved
  dependency slot.

### Result and generic task outcome decision

- The ordered public projection is `ParallelChildResultStatus::{Succeeded,
Failed, Cancelled, TimedOut, Skipped}`. Each slot carries its catalog ordinal,
  expected source agent, application-derived status, bounded validated findings
  when successful, redacted typed failure/skip reason when not, and bounded
  unresolved issues. It never carries hidden reasoning, raw runtime errors, or
  trusted authority supplied by the runtime.
- A timed-out admitted task uses the existing generic terminal
  `AgentTaskStatus::Failed` and `AgentTaskOutcome::Failed`, with one new exact
  `AgentTaskFailureCode::DeadlineExceeded`. The parallel projection preserves
  `TimedOut`, so callers do not mistake the deadline for an arbitrary failure.
  D-091 does not add a seventh generic task status or a fourth generic outcome.
- A `Skipped` dependent has no `AgentTask`, context, run, cancellation handle,
  or task outcome because it was never admitted. It exists only as an ordered
  workflow-result slot with an application-derived typed reason. Creating a
  synthetic task merely to skip it is prohibited.
- Results are stored and returned by catalog ordinal, never completion time,
  runtime event arrival, task-ID lexical order, or map iteration. Reverse and
  mixed completion tests must return the same ordered vector and synthesis
  transfer.

### Failure policies and dependent stages

- `ContinuePartial` applies only to scenario A. Research and Knowledge are
  independent: failure, cancellation, or timeout of one never cancels the
  other. Personal synthesis starts after both are terminal and must disclose
  both statuses and all application-derived unresolved issues.
- `CancelDependentOnly` applies only to scenario B. Coding and Security are
  independent and neither cancels the other. QA is admitted only after both
  succeed and their exact ordered transfers validate. If either prerequisite
  fails, is cancelled, or times out, QA is `Skipped(DependencyUnavailable)`;
  if a future invariant ever finds it active, only that dependent is cancelled.
  Personal synthesis still runs truthfully within the root lease.
- `FailFast` applies to the specialist lane of scenario C. Failure,
  cancellation, or timeout of either initial specialist cancels the other
  active initial specialist in ordinal order, skips Security, and blocks any
  later specialist admission. If both succeed, Security runs; a Security
  failure is recorded. Fail-fast never suppresses the application-controlled
  final Personal partial synthesis when the root remains live and within its
  deadline.
- A malformed specialist result is a typed failure under the scenario policy;
  it is never silently repaired. Final synthesis validates the exact ordered
  source-agent/status table and must expose failures, cancellations, timeouts,
  skipped stages, missing findings, and unresolved issues. Invalid synthesis
  fails the root; an application-authored fixed fallback may report only
  already validated status facts.
- Retries are exactly zero for start, event, parse, timeout, cancellation, and
  synthesis failures.
- `runtime.start` failure is typed and terminal without a retry or retained
  run. At either scenario-A ordinal it records failure and the other sibling
  continues; at either B prerequisite ordinal it lets the other finish and
  skips QA; at either C initial ordinal it cancels the sibling and skips
  Security. QA or Security dependent-start failure records that slot failed and
  permits truthful partial Personal synthesis. Personal synthesis/root start
  failure fails the root. Every ordinal is tested for no orphan task/run.

### Cancellation and deadlines

- Root cancellation sweeps active children in catalog ordinal order before the
  root/synthesis run. Unadmitted slots project `Skipped(RootCancelled)`, never
  `Cancelled`. A successful sweep cleans every terminal namespace and emits one
  root cancellation. If cancellation fails at any child or root/synthesis
  ordinal, earlier successful cancels stay terminal while the failing and later
  runs plus root remain live in closed-cancelling state; no slot admission,
  dependent, timeout, synthesis, or root-terminal event occurs. The next
  trusted ingress resumes at that ordinal.
- An explicit child cancellation validates its process-local handle. Under
  `ContinuePartial` or `CancelDependentOnly`, unrelated active siblings keep
  running; only dependent work is skipped/cancelled. Under `FailFast`, the
  policy cancels the remaining specialist lane. Root synthesis remains
  truthful unless the root itself is cancelled or expired.
- FailFast cancellation uses the same resumable ordinal sweep: failure keeps
  the failing/later runs and root live in closed-cancelling state, preserves
  prior successful cancels, and starts no dependent, synthesis, or terminal.
- Event linearization is exact: validate identity/sequence and prepare output,
  terminal transition, journal capacity, transfer, and successor input; then
  take the final monotonic sample immediately before `accept_event`. An accepted
  terminal event wins and is never converted by a post-accept sample. If
  `runtime.start` returns after expiry, or an accepted nonterminal run is found
  expired, `TimedOut` is committed only after `run.cancel()` succeeds. Cancel
  failure retains the task/run in closed-cancelling state with no timeout,
  successor, synthesis, or competing terminal result; later trusted ingress
  retries cancellation.
- Root expiry uses the same resumable ordinal sweep. After every active cancel
  succeeds, admitted active slots project
  `TimedOut(RootDeadlineExceeded)`, unadmitted slots project
  `Skipped(RootDeadlineExpired)`, the root uses generic
  `Failed(DeadlineExceeded)`, and the workflow terminal is `Failed`; no
  successor or synthesis starts. Cancellation failure leaves the root live and
  emits no root terminal/timeout until retry succeeds.
- The injected monotonic clock is private application state. Tests advance it
  without sleeps. A lease is live only while `now < deadline`: N-1 is live and
  N is expired.

### Events, attribution, and audit

- `Queued`, `Started`, `StartAttemptFailed`, `Progress` (accepted bounded text
  only), `Completed`, `Failed`, `Cancelled`, `TimedOut`, and `Skipped` for an
  exact work item or closed Personal-synthesis subject as applicable;
- `DependencySatisfied` for an exact predecessor/dependent pair;
- `ParentResumed`, `SynthesisStarted`, `SynthesisProgress`, and
  `SynthesisCompleted`;
- one terminal workflow `Cancelled`, `Partial`, `Failed`, or `Completed` event.

Each event has one matching content-free record. `Skipped` carries exact work-
item ID, ordinal, expected agent, and closed reason. `Queued`/`Skipped` use
sealed root/scenario/ordinal/expected-agent/policy-profile/memory-profile
attribution and explicitly no task, run, runtime, or live-context identity.
`Started` is emitted only after `runtime.start` succeeds and its returned run
identity validates. On `Err`, the admitted task becomes
`Failed(RuntimeStartFailed)` and one `StartAttemptFailed` event/audit outcome
carries root/scenario/subject/task/agent/profiles/runtime attempted attribution,
explicitly no `RuntimeRunIdentity`; it replaces `Started` plus terminal.
Live `Progress`/terminal records carry exact run attribution. Content-free
`SynthesisProgress` and its matching audit outcome use the live Personal root
run. No identity is fabricated and each run emits at most one progress variant.

The exact worst-case paired workflow/audit count is 19: three queued slots +
three lifecycle records for each of three admitted children + two dependency
records + one parent-resumed record + three synthesis lifecycle records + one
workflow terminal. Failure/cancel/expiry paths replace terminal records; a
cancellation failure updates only the closed-cancelling cursor and emits no
journal entry. Runtime events stay at eight per run. Capacity for every
prepared transition is preflighted before admission or terminal mutation; N
and N+1 unit proofs cover the 32-entry cap.
The journal contains no content/raw error and is not a general/durable
`AuditLogger`, authority, or persistence.

## Milestones

- [x] M0: begin `agent-bounded-parallelism`; reconfirm D-090, scope, and clean
      baseline with no overlapping work.
- [x] M1: add the closed domain/catalog/parser, exact limits, ordinal transfer,
      bounded framing, and only `AgentTaskFailureCode::DeadlineExceeded`.
- [x] M2: implement private admission/multiplexing, three failure policies,
      cooperative deadlines, resumable cancellation, cleanup, and synthesis.
- [x] M3: prove every scenario, bound, failure ordinal, cancellation/timeout,
      ordering, isolation, attribution, cleanup, prior selector, and Native
      regression.
- [x] M4: run completion/review/documentation gates, require a valid marker,
      and stop before UI, tools, providers, remote execution, commit, or push.

## Verification

Focused implementation checks:

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check
cargo test --manifest-path src-tauri/Cargo.toml --lib bounded_parallel
cargo test --manifest-path src-tauri/Cargo.toml --test agent_bounded_parallelism_contract
cargo test --manifest-path src-tauri/Cargo.toml --test agent_orchestration_contract
cargo test --manifest-path src-tauri/Cargo.toml --test agent_runtime_contract
```

Required deterministic coverage:

- two successes; three successes at the hard cap via private `cfg(test)`
  catalog data; mixed success/failure; all A/B/C dependent/partial paths; and
  reverse/interleaved completion with identical ordinal results;
- queueing plus hard-active, total, depth, duplicate, retry, and reachable
  per-run/global/workflow/audit guards; one Queued per sealed slot; and one coalesced
  Progress per run; active=2 waits while active=3 admits the three-slot fixture;
- objective/catalog/finding/unresolved/raw/transfer/synthesis reachable
  boundary/guard, aggregate, quote/backslash/newline, multibyte, prepared-
  terminal, and exact generic Native 65,536/65,537-byte gateway proofs. Sealed
  natural maxima remain below some generic caps and are not fabricated as
  unreachable production N/N+1 cases;
- runtime-start failure at every initial/dependent/synthesis ordinal, with
  exact policy, zero retry/orphan/`Started`, and attempted attribution without a
  run identity;
- N-1/N/advancing-clock child/root expiry; timeout cancel failure; root and
  FailFast cancel failure at every child plus synthesis/root ordinal; retry to
  completion with no premature terminal/successor;
- root and individual cancellation, unadmitted skip reasons, deterministic
  cancellation order, partial synthesis, and late-event rejection;
- scenario-A sibling memory isolation, existing live grants, terminal cleanup,
  no copy/auto-write/proposal/promotion, and disabled-role write denial;
- planned-slot versus live-run attribution, no fabricated run identity, exact
  event/audit order, and no leaked task/run/memory/document/handle;
- generic delegation, D-086 through D-090 selectors, and `NativeAgentRuntime`
  request/event/cancellation/session regression without trait changes.

Completion-gate checks after the last source edit:

```bash
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked
npm run verify
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
python3 .codex/hooks/session_end_gate.py
```

Manual evidence:

- inspect the complete diff and prove no runtime trait, Native implementation,
  provider, tool, policy permission, approval dispatch, memory-store, document,
  storage, dependency, configuration, IPC/UI, process, thread, network,
  filesystem, platform, scheduler, remote/distributed, or effect path changed;
- confirm the same-thread event interleaving is described as an unwired
  cooperative fixture proof, not simultaneous provider work or production load
  evidence;
- confirm every partial synthesis visibly attributes all ordered specialist
  statuses and never conceals a failure, cancellation, timeout, or skipped
  dependent.

## Risks and mitigations

- **False concurrency claim:** multiple retained runs are not simultaneous CPU
  or provider execution. Mitigation: name the same-thread event-multiplexed
  model in contracts, tests, events, and closeout evidence.
- **Cross-run event or identity confusion:** an event could mutate the wrong
  child. Mitigation: exact orchestrator/root/task/run/profile binding, stable
  ordinals, per-run sequences, terminal rejection, and adversarial tests.
- **Cancellation race or orphan:** one terminal path could leave a sibling run
  or memory entry live. Mitigation: prepared transitions, ordinal child-first
  cancellation, one terminal cleanup function, idempotence, advancing-clock
  tests, and post-terminal inventory assertions.
- **Deadline overclaim:** synchronous runtime calls cannot be hard-preempted.
  Mitigation: cooperative before/after checks, injected clock, no timer or hard
  guarantee, and no provider/live-model activation.
- **Completion-order nondeterminism:** interleaving could affect output order or
  synthesis. Mitigation: catalog ordinals own storage, dependency transfers,
  events, cancellation order, and final projection.
- **Failure concealment:** Personal synthesis could omit a failed specialist.
  Mitigation: strict exact ordered status reconciliation and fixed truthful
  fallback facts.
- **Queue/general-engine drift:** bounded pending slots could become a scheduler
  or reusable DAG abstraction. Mitigation: private three-scenario state, no
  worker/timer/persistence/trait/DSL, exact file stop condition, and review.
- **Limit arithmetic or budget replenishment:** active-slot release could be
  confused with new task budget. Mitigation: separate monotonic total counters,
  checked arithmetic, explicit no-replenishment tests, and immutable caps.

## Stop conditions

Stop and return to owner review if implementation needs any file outside the
declared scope; a runtime/provider/session/thread/async trait; a generic graph
or scheduler; more than three active/total specialist children; depth above
one; a retry; specialist or Workflow Automation task creation authority; a
tool, policy, approval, general audit, memory capability, persistence, I/O,
IPC/UI, dependency, configuration, credential, process, network, platform,
remote/distributed, or device boundary; or suppression of a required failure,
timeout, cancellation, cleanup, attribution, or regression test.

## Rollback

Revert only the bounded D-091 source, tests, and closeout documentation listed
above. Remove `AgentWorkflowSelection::BoundedParallel`, its domain/catalog/
validation/private lifecycle modules, the exact deadline failure code, and its
single exhaustive D-088 framing projection arm.
Restore the prior selector facade and module export. All D-083 through D-090
sequential/proposal selectors, their one-active-child limits, the unchanged
runtime traits and Native implementation, catalog activation, governance,
memory/document boundary, and fixture contracts remain intact. No data,
dependency, migration, configuration, permission, external resource, or
durable state requires cleanup.

## Decisions and discoveries

- 2026-08-13: the production default active limit is two; the immutable hard
  and total-child limit is three; depth remains one; total tasks are four;
  run attempts are five; retries are zero.
- 2026-08-13: timeout remains a generic failed task with the exact
  `DeadlineExceeded` code and an explicit parallel `TimedOut` projection.
  Skipped work is a workflow slot only and never a fabricated task.
- 2026-08-13: `FailFast` stops/cancels the specialist lane but preserves a
  truthful Personal partial synthesis when the root is still live.

## Progress

- [x] Clean synchronized D-090 baseline `1f85264`, complete/valid marker, fresh
      reviews, accepted D-091, and owner-selected Ready plan recorded.
- [x] `agent-bounded-parallelism` gate begun.
- [x] Source and test implementation begun with the deadline failure code,
      exhaustive D-088 projection, and deterministic runtime recorder.
- [x] Closed domain/catalog/parser and private same-thread lifecycle implemented
      for all three scenarios with exact limits, policies, ordinal results,
      cancellation, deadlines, quarantine, cleanup, and synthesis.
- [x] Focused bounded-parallel library tests 41/41; public D-091 contract 41/41;
      strict Clippy and formatting pass; all-target Rust 481 passed with one
      ignored; `npm run verify` and independent reviews pass.
- [x] Validation, closeout, memory sync, and completion marker created.

## Final results

Implementation and closeout are verified complete with advisories. The exact
same-thread event-multiplexed selector, three sealed scenarios, explicit
failure policies, deterministic ordinal results, cooperative deadlines,
resumable cancellation, rejected-run quarantine, memory cleanup, strict
synthesis, and Native regression coverage pass. Focused library and public
contracts pass 41/41 each; formatting, strict Clippy, 481 all-target Rust tests
with one ignored, `npm run verify`, and independent code/architecture/security
review pass with expected `PASS WITH ADVISORIES`.

The accepted advisories are: same-thread event multiplexing is not simultaneous
provider/CPU work; deadlines are cooperative rather than hard preemption;
limits are per orchestrator without app-global provider/session coordination;
the private lifecycle is 5,188 lines; six public typed error variants are not
naturally reached by sealed production catalogs; lexical claim filtering is
defense in depth, not authorization; and the pre-existing legacy
`start_runtime_run` cancellation-error orphan risk is unchanged. None blocks
D-091 completion. Final documentation, repository, security, diff, and
session-end checks pass. Deterministic finalization completed and status reports
`complete`, `valid: true`, and `PASS WITH ADVISORIES`. No owner-selected
successor plan is Ready.
