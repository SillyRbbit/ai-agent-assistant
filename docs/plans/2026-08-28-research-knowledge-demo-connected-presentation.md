# Research/Knowledge connected-presentation ExecPlan

Status: Ready with advisories
Increment: `research-knowledge-demo-connected-presentation`
Last updated: 2026-08-28

## Goal

Connect the verified volatile Research -> Knowledge lifecycle client to only
the selected Command Center Research and Knowledge scenario. The result remains
visibly simulated and process-local, starts only from an explicit user action,
and does not claim the Command Center, Conversations mock, or Rust acceptance
fixtures are one integrated system.

## Current-state evidence

- `ResearchKnowledgeDemoHost` owns fixed application identities, objective,
  fixtures, Native runtime, and envelopes. The Tauri adapter owns one
  mutex-held host and exposes only no-argument snapshot, start, advance, and
  cancel commands plus one notification-only snapshot event.
- The client runtime-narrows exact snapshots, rejects stale/gapped events, and
  requires explicit snapshot recovery. No React component consumes it.
- The production host is success-only. Its deterministic synthesis-failure
  script is private core-test evidence, so current code cannot truthfully show
  a failed terminal run.

## Prerequisite controls

- **F-12 static UI/native boundary protection:** extend the existing static
  allowlist atomically for only the declared panel import and the four existing
  no-argument lifecycle invocations; tests must reject alternate consumers,
  arguments, selectors, and transport expansion.
- **F-07 production/development CSP separation:** retain the verified distinct
  CSPs exactly. This slice adds no connect source, inline script, capability,
  permission, plugin, or configuration change.
- **F-08 runtime IPC narrowing:** preserve the client-side `unknown` boundary,
  exact v1 DTO parser, closed errors, and explicit recovery rule. No caller
  value reaches Rust, and notification events remain non-authoritative.
- **F-01 exact returned-runtime identity validation:** reuse the current native
  host/orchestrator start path unchanged; the panel cannot construct, select,
  or replace runtime, task, run, request, profile, or workflow identity.
- **F-02 rejected-run quarantine and cleanup ownership:** retain the existing
  fail-closed rejected-run cleanup and replacement blocking. The UI may render
  only the closed lifecycle result, never cleanup internals or a retry action.
- **F-15 documentation reconciliation:** update architecture, security, tests,
  increment, plan, handoff, status, next steps, changelog, and troubleshooting
  records only from observed source and completed verification evidence.

## User-visible outcome

Only in the selected scenario, display `DEMO MODE · SIMULATED AGENT DATA`, the
exact separate-proof statement, a bounded content-free lifecycle snapshot and
journal, and three controls: Start simulated lifecycle, Advance simulated
lifecycle, and Cancel simulated lifecycle. They are enabled only when the
accepted snapshot permits their no-argument operation. No control, field, route,
fixture switcher, or URL parameter selects an agent, task, run, profile,
runtime, workflow, objective, fixture, stage, script, or outcome.

## Proposed source scope and invariants

1. Retain the four commands, event name, DTO schema, client parser, and one-host
   mutex. Add no IPC parameter, command, or event.
2. Add a private, application-owned terminal-script schedule: the first
   completed epoch succeeds, the next completed epoch uses the existing
   deterministic synthesis failure, then the schedule repeats. Cancellation
   does not advance the schedule. The WebView can request only start, advance,
   or cancel and cannot select or alter an outcome.
3. Render only bounded state/journal language and one fixed unavailable message.
   Never render objective, sources, findings, output, runtime identity, raw
   errors, memory, audit, paths, URLs, or reasoning.
4. Add one feature-local panel that creates/disposes the client only while the
   selected scenario is mounted. It performs no polling, retry, queue, timer,
   worker, or automatic start. Existing topology, inspector, structured view,
   and Conversations remain separate fixture presentation.
5. Extend F-12 atomically for the declared import and reject alternate
   lifecycle consumers, command arguments, outcome selectors, network/storage
   APIs, capability/CSP changes, or a broader Tauri surface.

A response is authoritative only for its call; an event is notification-only.
Malformed, stale, duplicate, gapped, reordered, or contradictory events never
mutate presentation state. Late-event rejection remains a contract test, not a
user-selectable simulation control. Failure is labeled as an application-owned
synthetic rehearsal outcome, not a provider, model, tool, approval, policy,
audit, or device failure.

## Exact expected paths

- `src-tauri/src/research_knowledge_demo_lifecycle.rs`
- `src-tauri/src/research_knowledge_demo_lifecycle_tauri.rs`
- `src-tauri/tests/research_knowledge_demo_lifecycle_tauri_contract.rs`
- `src/infrastructure/tauri/research-knowledge-demo-lifecycle-client.ts`
- `src/infrastructure/tauri/research-knowledge-demo-lifecycle-client.test.ts`
- `src/features/command-center/ResearchKnowledgeLifecyclePanel.tsx`
- `src/features/command-center/ResearchKnowledgeLifecyclePanel.test.tsx`
- `src/features/command-center/CommandCenterPage.tsx`
- `src/features/command-center/CommandCenterPage.test.tsx`
- `src/features/command-center/command-center.css`
- `scripts/repository_health.py`
- `scripts/tests/test_repository_health.py`

The source increment may update only these paths and its plan, increment,
review, and current-state records. It must not change `src-tauri/src/agent/**`,
Tauri configuration/capabilities/CSP, dependencies/lockfiles, Conversations,
global application state, or any projection DTO.

## Explicit non-goals

No provider, model, network, credential, external runtime, tool execution,
policy or approval dispatch, persistence, filesystem/document access, durable
audit, background autonomy, timer, worker, queue, scheduler, generic engine,
permission, plugin, device effect, packaging, signing, or notarization. No live
catalog, retry, fixture editor, arbitrary command, manually injected event, or
unbounded activity log.

## Verification and manual gates

```bash
cargo test --manifest-path src-tauri/Cargo.toml research_knowledge_demo_lifecycle
cargo test --manifest-path src-tauri/Cargo.toml research_knowledge_demo_lifecycle_tauri
cargo test --manifest-path src-tauri/Cargo.toml --test research_knowledge_demo_lifecycle_tauri_contract
npm run test:frontend
npm run test:repository
npm run test:agent-acceptance
npm run verify
npm run security:scan
git diff --check
```

Target Mac, after source approval: verify selected scenario only; success,
then deterministic failure; cancellation at each active stage; closed controls
at terminal states; disclosure/proof statement; theme, reduced motion, focus,
scroll, viewport, native resize, and no permission prompt. Unavailable harness
checks are `Not run`. Direct synthetic late-event injection is test-only.

## Risks, rollback, and stop conditions

A schedule selector, failure-specific command, second host, timer/worker,
polling/retry, another UI surface, capability/CSP/permission change, dependency,
provider/model/network, credential, tool, approval dispatch, persistence,
filesystem access, or device effect is a stop condition. Pre-commit rollback
restores only declared source/UI/static-test paths; post-commit rollback reverts
only that bounded source increment. All state remains volatile.

## Planning result

This ExecPlan is Ready with advisories for a separately approved source
increment. The alternating schedule is proposed future behavior, not current
behavior or a durable architecture decision. Source implementation still needs
explicit owner approval and a fresh gate.
