# Research/Knowledge demo lifecycle Tauri adapter ExecPlan

Status: Verified complete with advisories
Increment: `research-knowledge-demo-lifecycle-tauri-adapter`
Last updated: 2026-08-28

## Goal

Define the smallest truthful native adapter for the completed volatile Research
-> Knowledge lifecycle host. A later source increment may expose only
application-owned no-input lifecycle operations and a bounded content-free
snapshot notification. It must not connect the Command Center or Conversations
mock, which remain separate deterministic proofs.

## User-visible outcome

None in this planning increment. The future adapter must retain the exact
`DEMO MODE · SIMULATED AGENT DATA` disclosure and state that the Command Center,
Conversations mock, and Rust acceptance workflows are separate deterministic
proofs.

## Preconditions and current-state evidence

- The lifecycle core is published at `68de8a5`; its valid marker reports `PASS
WITH ADVISORIES`. `ResearchKnowledgeDemoHost` owns the sealed D-086 workflow,
  fixed fixtures, Native runtime start, F-01 identity checks, F-02
  cleanup/quarantine, and its closed v1 snapshot/journal.
- The projection command/client is argument-free and read-only. It neither
  starts, controls, nor observes the lifecycle host.
- F-07 CSP separation, F-08 runtime narrowing, F-12 static protection, and
  F-15 reconciliation are complete. `core:default` is the only capability; no
  capability or CSP change is needed for this narrow command/event boundary.
- `src-tauri/src/lib.rs` currently registers only app-info and the read-only
  projection command; it manages no lifecycle host and emits no agent event.
- The approved source attempt proved that `ResearchKnowledgeDemoHost` cannot be
  placed in Tauri managed state: Tauri requires managed state to be `Send +
Sync + 'static`, while the host transitively owns the private
  `dyn ApprovalClock` trait, which is not `Send`. The declared adapter path list
  cannot satisfy that prerequisite.

## Proposed source scope

- Add one private Tauri adapter owning exactly one
  `Mutex<ResearchKnowledgeDemoHost>` for the process lifetime. A poisoned or
  unavailable lock maps to the existing closed unavailable error.
- Register only four no-argument commands: current snapshot, start, advance,
  and cancel. None accepts an agent, task, root, run, request, profile, runtime,
  workflow, objective, fixture, script, stage, or outcome.
- Future UI starts only through an explicit user action. Tauri cannot prove a
  human gesture; that is acceptable only because the data/effect remain
  application-owned, simulated, volatile, and non-consequential.
- Return only the existing snapshot or closed lifecycle error. Never serialize
  identities, objective, sources, output, runtime envelope, memory, audit,
  paths, URLs, reasoning, or upstream error text.
- Emit one fixed-name snapshot event after a successful transition. Its payload
  is exactly the response snapshot; it has no caller payload or authority token.
- Add one initially unconnected WebView client which invokes/listens through
  literal names, receives all values as `unknown`, validates and freezes the
  exact v1 schema, and reports one fixed unavailable error.
- Extend F-12 checks atomically for adapter/client imports, literal no-argument
  invocation, registrations, event, prohibited tokens, capability, and CSP.

## Interfaces and invariants

```text
get_research_knowledge_demo_lifecycle_snapshot() -> Snapshot | ClosedError
start_research_knowledge_demo_lifecycle()        -> Snapshot | ClosedError
advance_research_knowledge_demo_lifecycle()      -> Snapshot | ClosedError
cancel_research_knowledge_demo_lifecycle()       -> Snapshot | ClosedError
event: research-knowledge-demo-lifecycle-v1      -> exact Snapshot
```

The mutex serializes host mutation and derives the response before release. A
response is authoritative for its own call; an event is notification only. An
event-emission failure cannot undo a committed transition and must return a
closed unavailable result. The client may not infer success from an event.

`presentationEpoch` and `revision` are display correlation only, never trusted
inputs. For one epoch, accept only a direct successor; ignore stale/duplicate
events and treat a gap as unavailable until an argument-free snapshot query
validates a replacement. A newer epoch replaces a completed prior epoch only
through an accepted snapshot. A malformed, foreign, reordered, missing, or
extra-field reply/event never changes client state. The client unsubscribes on
disposal and must not retry, poll, queue, or schedule background work.

The adapter retains the host while it is `cleanup-pending`; it must not replace,
drop, reset, or recreate a quarantined/nonterminal host. The Drop sentinel stays
a process replacement blocker, not a concurrency coordinator.

## Exact expected source and test paths

- `src-tauri/src/research_knowledge_demo_lifecycle_tauri.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/approvals/manager.rs`
- `src-tauri/tests/research_knowledge_demo_lifecycle_tauri_contract.rs`
- `src/infrastructure/tauri/research-knowledge-demo-lifecycle-client.ts`
- `src/infrastructure/tauri/research-knowledge-demo-lifecycle-client.test.ts`
- `scripts/repository_health.py`
- `scripts/tests/test_repository_health.py`

The owner separately approved adding `src-tauri/src/approvals/manager.rs` for one
private prerequisite: require `ApprovalClock: Send`, replace its deterministic
test-only `Rc<Cell<Instant>>` clock with `Arc<Mutex<Instant>>`, and add a
compile-time Send assertion. This amendment must not change approval behavior,
public interfaces, production clock semantics, policy, persistence, or
authority.

The source increment may update only those paths plus its plan, increment,
current-state records, and review evidence. It must not alter the lifecycle-core
public contract, `src-tauri/src/agent/**`, projection command/client/panel,
React pages, Tauri configuration/capabilities/CSP, dependencies, or lockfiles.

## Explicit non-goals

- No Command Center/Conversations connection, fixture mutation, visible control,
  rendered UI, or unified-proof claim.
- No provider, model, network, credential, external runtime, tool execution,
  approval dispatch, policy change, audit, persistence, filesystem/document
  access, timer, worker, polling, queue, scheduler, generic engine, parallelism,
  background autonomy, device effect, plugin, permission, or dependency.
- No caller-selected trusted identity or arbitrary task/workflow/objective/data.
- No capability, production/dev CSP, asset-CSP, window, packaging, signing, or
  entitlement change.

## Test plan

- Rust: exact command signatures/registration, one synchronized host,
  start/advance/cancel/snapshot, lock-unavailable closure, event payload
  closure, event failure after committed transition, cleanup retention, and
  no internal identity/content in serialization or Debug output.
- Client: literal command/event use; valid data; malformed, missing, extra,
  wrong-type, oversized, stale, duplicate, gap/recovery, newer-epoch,
  listener-disposal, and native-error-redaction cases.
- F-12 negative fixtures: command argument, alternate event, extra handler,
  broadened import, network/storage API, capability expansion, or CSP change
  fails repository health.
- Existing lifecycle core, D-086, and `npm run test:agent-acceptance` remain
  unchanged.

Required source completion commands:

```bash
cargo test --manifest-path src-tauri/Cargo.toml research_knowledge_demo_lifecycle_tauri
cargo test --manifest-path src-tauri/Cargo.toml --test research_knowledge_demo_lifecycle_tauri_contract
npm run test:frontend
npm run test:repository
npm run test:agent-acceptance
npm run verify
npm run security:scan
git diff --check
```

Target Mac: run `npm run tauri -- dev`, confirm startup/no permission prompt,
and use only an approved harness or future separately approved UI to observe the
adapter. If tooling cannot observe the unconnected event, mark it `Not run`.
Viewport, theme, motion, focus, scroll, zoom, resize, and connected Command
Center checks belong to the later presentation increment.

## Risks, rollback, and stop conditions

- Event loss/reordering can misrepresent state; response/snapshot recovery is
  mandatory and does not authorize polling or a queue.
- Event emission after mutation cannot be atomic; return a closed error and
  preserve the host for snapshot recovery.
- Roll back only adapter/client/static-test/closeout paths; there is no durable
  state or migration.
- Stop if a UI connection, replay store, timer/worker, automatic retry,
  additional host, caller input, agent-internal change, capability/CSP/
  permission/dependency change, provider, network, credential, tool, approval,
  persistence, filesystem, or device effect is required.

## Planning result and approval boundary

Readiness is **Verified complete with advisories**. The approved adapter and
private approval-clock Send prerequisite pass focused, complete, and target-Mac
startup verification. Approved UI tooling cannot directly observe the
deliberately unconnected event, which remains notification-only and is `Not
run`. A later plan must still separately connect any client to the Command
Center.

## Progress

- 2026-08-28: Owner approved documentation-only planning. The gate began from
  clean merged `origin/main` at `68de8a5`; no source/runtime behavior changed.
- 2026-08-28: Owner approved the source increment. Focused adapter compilation
  failed because Tauri managed state requires `Send + Sync + 'static` and the
  host transitively contains non-Send `dyn ApprovalClock`. The partial adapter
  and registration were rolled back; the existing lifecycle-core tests then
  passed 9/9. No product-source change remains.
- 2026-08-28: The required post-increment workflow recorded `FAIL`. Baseline
  frontend, repository, acceptance, full verification, security, diff, release
  no-bundle build, and target-Mac process startup pass, but the adapter contract
  target is absent, the name-filter command executes zero adapter tests, and
  visual/native lifecycle checks remain pending or Not run. The source gate
  remains active and unfinalized.
- 2026-08-28: The owner approved the exact private prerequisite amendment:
  `ApprovalClock: Send`, deterministic `Arc<Mutex<Instant>>` test-clock storage,
  a compile-time Send assertion, and governance regression tests. No public
  approval interface or behavior change is authorized.
- 2026-08-28: The prerequisite and adapter are implemented. Focused evidence
  passes 7 approval-manager tests, 9 lifecycle-core tests, 5 adapter tests, 1
  public Rust contract, 30 client tests, strict typecheck/lint, and 61
  repository/F-12 tests. One transient Cargo incremental-cache failure passed
  on exact unchanged retry. Complete verification and target-Mac evidence are
  pending.
- 2026-08-28: Complete verification passes with 28 hook tests, 61 repository
  tests, 277 frontend tests, 267 Rust library tests, 244 Rust integration tests,
  one intentional ignored Hermes probe, the production frontend build, and the
  Tauri release no-bundle build. Target-Mac development startup and visual
  no-permission-prompt inspection pass. Direct unconnected command/event
  observation is `Not run`.

## Acceptance criteria

- [x] One bounded future adapter, exact files, ordering/cleanup invariants,
      non-goals, tests, target-Mac boundaries, rollback, and stop conditions exist.
- [x] F-01/F-02, F-07, F-08, F-12, and F-15 are retained.
- [x] The lifecycle core, projection panel, Command Center, and Conversations
      mock remain separate deterministic proofs.
- [x] Source implementation received separate owner approval.
- [x] The newly discovered private approval-clock Send prerequisite is
      separately owner-approved.
- [x] The prerequisite passes focused tests without behavior change.
- [x] Complete verification, target-Mac startup evidence, and consolidated
      review pass with the documented unconnected-event advisory.

## Final results

The earlier pre-approval source attempt and
[`FAIL` review](../reviews/2026-08-28-research-knowledge-demo-lifecycle-tauri-adapter-blocked-post-increment-review.md)
remain historical evidence. After separate owner approval, the private Send
prerequisite and exact adapter scope were implemented and verified without
adding any prohibited boundary. Final result: `PASS WITH ADVISORIES`; the sole
manual advisory is direct observation of the deliberately unconnected event.
