# Agent orchestration and task lifecycle

Status: Blocked; draft follow-on, not approved for implementation
Owner: Project owner
Last updated: 2026-08-11
Blocked on: verified, published AgentDefinition/AgentRegistry completion plus a
fresh architecture, security, and readiness review

## Goal

Future goal: introduce the smallest application-owned task lifecycle,
execution context, bounded delegation request, limits, and orchestrator above
the existing `AgentRuntime`, without adding a live provider, tools, memory, or
UI.

## Why this plan is blocked

- Agent definitions and the registry do not exist yet.
- Current `RuntimeTurnRequest` has no agent ID, task ID, parent task ID, policy
  profile, or memory namespace.
- `RuntimeRunStatus` is a one-turn state and cannot be adopted as task status
  without resolving pending-approval and synthesis semantics.
- Cancellation propagation, task/result bounds, and exact attribution require
  a fresh security and architecture review.
- The exact files and public interfaces must be derived from the implemented
  definition/registry phase, not guessed now.

This document records only the immediate follow-on boundary. It is not an
implementation-ready ExecPlan and grants no editing authority.

## Provisional scope

- Closed `AgentTaskId` and task lineage.
- `AgentTask`, `AgentTaskStatus`, and `AgentTaskResult` with one terminal result.
- `AgentExecutionContext` binding agent, task, optional parent, runtime, policy
  profile, and memory namespace identities.
- `DelegationRequest` with bounded trusted parent/target/input/limits.
- `AgentLimits` with depth one, one total child per root, and one active child;
  terminal child work does not replenish the total budget.
- Application-owned route policy with exactly one Personal Assistant root and
  one Personal Assistant to Research Agent child edge.
- Concrete `AgentOrchestrator` as an application service above
  `AgentRuntime`.
- Deterministic contract tests for task creation, route validation, child-budget
  enforcement, result collection, cancellation, and attribution without an
  end-to-end Personal/Research synthesis flow.

## Provisional non-goals

- No live provider, model, network, process, Hermes, external framework, or
  runtime selector.
- No tool proposal/execution, policy-profile enforcement, approval change,
  audit persistence, memory store, storage migration, Tauri IPC, React, or UI.
- No activation or behavior for Coding, Cloud Infrastructure, Systems
  Operations, Knowledge & Document, QA & Validation, Security & Risk, or
  Workflow Automation.
- No parallel children, depth above one, recursion, scheduler, queue, worker,
  retry engine, background autonomy, or durable task resumption.
- No `agent.delegate` host tool and no orchestration methods added to
  `AgentRuntime`.

## Provisional invariants

- Only `AgentOrchestrator` creates a child task.
- Registry membership is non-authorizing. The root must be Personal Assistant;
  only Personal Assistant to Research Agent is allowed; Research Agent cannot
  delegate; self, reverse, unknown, and other routes fail before task creation.
- `AgentRuntime` remains the one-run execution boundary.
- Every accepted task/run/result/cancellation carries exact agent and task
  identity; child work also carries exact parent identity.
- Unknown, missing, duplicate, stale, or mismatched identities fail closed.
- Agent definitions remain privilege-free; runtime capabilities remain
  descriptive and non-authorizing.
- A child result is bounded untrusted data for parent synthesis, not authority.
- Parent cancellation reaches at most one active child exactly once; late
  events/results are rejected.
- A second child request is rejected even after the first child completes or is
  cancelled.
- Native remains sole/default and ordinary tests are deterministic and no-I/O.

## Required decisions before Ready

- Exact task status machine and its mapping to runtime state.
- Whether orchestration owns runtime events directly or a narrower adapter.
- Exact task/result/content/count/depth/deadline limits.
- Exact cancellation ordering and pending-approval behavior.
- Whether policy/memory identity can be represented as closed placeholders
  without implying enforcement; otherwise defer those fields until the
  governance/memory phases.
- Exact source/test/document paths after the registry implementation exists.
- Whether `AgentInstance` or a distinct orchestration `AgentEvent` is actually
  needed. Default is to omit both.
- Exact deterministic denial coverage for self, reverse, unknown, out-of-route,
  Research-originated, and second-child requests before task creation.

The named Personal-to-Research execution and Personal synthesis demonstration
belongs to roadmap Phase 3 and remains outside this Phase 2 draft.

## Expected verification before Ready

- Fresh `$readiness-review` on a clean synchronized definition/registry
  baseline.
- Architecture and security review of identity, lineage, cancellation, and
  trust boundaries.
- Exact focused deterministic task/orchestration test matrix.
- Full Rust and repository completion-gate commands, rollback, and closeout
  inventory.

## Rollback

No implementation exists. Removing this draft returns orchestration to roadmap
only and leaves the Accepted architecture, runtime foundation, and Ready first
plan unchanged.
