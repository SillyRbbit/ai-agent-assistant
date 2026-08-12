# Bounded agent parallelism

Status: Blocked; draft follow-on, not approved for implementation
Owner: Project owner
Last updated: 2026-08-11
Blocked on: verified sequential task/orchestration and cancellation behavior
plus an exact concurrency/resource decision and fresh readiness review
Roadmap:
[`NATIVE_MULTI_AGENT_ROADMAP.md`](../roadmap/NATIVE_MULTI_AGENT_ROADMAP.md)

## Goal

Future goal: increase active child concurrency from one to a separately
selected explicit finite bound while preserving depth-one orchestration,
workflow total-task caps, deterministic limits, attribution, cancellation, and
failure containment.

## Provisional scope

- Closed per-root and application-wide concurrency/resource limits.
- Deterministic task admission, ordering, result collection, and overload
  rejection.
- Parent and individual-child cancellation with late-event rejection.
- Defined partial-failure, timeout, shutdown, and audit semantics.
- Tests that prove the concurrency limit cannot increase delegation depth or
  total task budgets implicitly.

## Explicit non-goals

- No recursion, specialist spawning, unbounded fan-out, replenishing budgets,
  queue service, worker fleet, remote/distributed agents, scheduler, or hidden
  background autonomy.
- No provider, process, persistence, IPC, UI, or tool implementation.
- No role becomes enabled merely to exercise concurrency.

## Required work before Ready

Derive the smallest justified concurrency value from measured sequential
behavior; specify resource accounting, deterministic scheduling, cancellation,
failure, stress and race tests, portability, rollback, and architecture/security
review.

## Authority and rollback

This draft grants no parallel-execution authority. Removing it retains one
active child and leaves verified sequential behavior unchanged.
