# Research and knowledge workflow

Status: Blocked; draft follow-on, not approved for implementation
Owner: Project owner
Last updated: 2026-08-11
Blocked on: verified D-085 volatile memory and approved-document boundaries,
then a separate exact Research-to-Knowledge workflow readiness review and owner
authorization

D-085 activates only a direct Personal Assistant-to-Knowledge document task.
It does not satisfy or implement this two-specialist sequence.
Roadmap:
[`NATIVE_MULTI_AGENT_ROADMAP.md`](../roadmap/NATIVE_MULTI_AGENT_ROADMAP.md)

## Goal

Future goal: stage a bounded Personal Assistant, Research Agent, and Knowledge &
Document Agent workflow that produces one attributable evidence-backed result
for Personal Assistant synthesis.

## Provisional sequence

```text
Personal Assistant
  -> Research Agent
  -> Knowledge & Document Agent
  -> Personal Assistant synthesis
```

The arrows are logical ordering controlled by `AgentOrchestrator`. Specialists
never create the next task. Each accepted specialist task remains a depth-one
child of the Personal Assistant root, and initial active-child concurrency
remains one. This workflow's provisional total-child cap is exactly two.

## Provisional scope

- Preserve the earlier deterministic Personal-to-Research milestone.
- Add Knowledge & Document only after its access and memory boundaries pass.
- Support internal or external research only through separately approved,
  read-only governed tools.
- Bind sources, comparisons, results, cancellation, and synthesis inputs to the
  exact task lineage.
- Treat every specialist result as bounded untrusted data, not authority.

## Explicit non-goals

- No specialist spawning, recursion, parallel children, silent memory write,
  unrestricted browser/network/filesystem use, or source-free factual claim.
- No live provider, retrieval tool, memory store, IPC, or UI through this draft.
- No activation of other specialist workflows.

## Required work before Ready

Specify exact source/provenance contracts, content and count limits, failure and
cancellation ordering, deterministic fixtures, synthesis boundaries, source
disclosure, full validation, and rollback from the implemented prerequisites.

## Authority and rollback

This draft authorizes no workflow execution or agent activation. Removing it
preserves the initial catalog and returns research/knowledge work to roadmap
status.
