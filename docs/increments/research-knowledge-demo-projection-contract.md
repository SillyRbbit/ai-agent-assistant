# Research/Knowledge demo projection contract

Status: Verified complete with advisories
Owner: Project owner
Last updated: 2026-08-27

## Goal and outcome

Add the smallest truthful Rust-to-WebView prerequisite for the later
interactive demo: one explicit, read-only query of an application-owned
synthetic Research and Knowledge projection. The query is descriptive only and
cannot start or control a workflow.

## Implemented scope

- One no-argument Tauri command,
  `get_research_knowledge_demo_projection()`, constructs a fresh exact v1 DTO
  from private Rust constants.
- The DTO contains only the fixed schema/scenario, exact disclosure and proof
  boundary, synthetic-fixture provenance, three ordered ready roles, and the
  presentation-only `succeeded`, `failed`, and `cancelled` vocabulary.
- A dedicated Tauri client invokes as `unknown`, requires the complete exact
  shape and Unicode bounds, creates fresh frozen local values, and collapses
  native, transport, and malformed responses to one fixed error.
- Only `research-knowledge-active` renders the panel. No request occurs until
  the user activates `Refresh Rust projection`; concurrent requests are
  disabled and late replies after scenario exit are ignored.
- F-12 now pins the complete capability JSON, exact import symbols, sole menu
  event, two-command handler, both no-argument invokes, zero-argument Rust
  signature, and forbidden boundary imports/calls.

## Boundaries preserved

Command Center topology/activity remain frontend fixtures and do not consume
the native DTO. Conversations mock behavior and Rust acceptance workflows are
separate deterministic proofs. The query adds no lifecycle, mutable native
state, event, polling, timer, background work, provider, model, network,
credential, tool, approval dispatch, persistence, durable audit, filesystem,
generic engine, dependency, capability, permission, or device effect.

## Focused evidence

- Projection client, panel, Command Center, and App tests: Passed, 69/69.
- Rust projection module tests: Passed, 3/3.
- Exact public Rust serialization contract: Passed, 1/1.
- Repository-health tests: Passed, 32/32.
- TypeScript typecheck, Rust formatting, exact UI/native static boundary, and
  `git diff --check`: Passed.

## Target-Mac evidence

- Pinned Node 26.3.0, npm 11.16.0, and Rust/Cargo 1.90.0 were used on macOS
  26.6 build 25G72 arm64.
- `npm run tauri -- dev`: Passed launch and source watch. Direct Computer Use
  binding to the raw debug executable is unavailable.
- The source-current local release `.app`: Passed. Before refresh, it rendered
  `Rust projection not requested`; after one explicit refresh, it rendered the
  exact schema, scenario, provenance, three ready roles, three outcomes,
  disclosure, and `No workflow was started` copy.
- Dark/light and reduced-motion off/on checks passed and the original dark,
  standard-motion settings were restored.
- Native resize passed at 1040x700, 803x563, and restored 1040x700 with the
  disclosure and ready projection intact. Browser checks passed minimum
  760x520 and configured 1040x700 viewports, keyboard focus, scroll ownership,
  and topology zoom/reset without horizontal overflow.
- Browser/host page zoom was unavailable and is `Not run`; topology zoom/reset
  is the applicable in-app zoom check and passed.
- A local bundle attempt created the source-current `.app`; subsequent DMG
  packaging failed inside `bundle_dmg.sh`. Distribution is outside this
  increment and no artifact was published.

## Completion state

Complete `npm run verify` passes with 247 frontend tests, 252 Rust library
tests, 242 Rust integration tests, and one intentional opt-in Hermes probe
ignored. Independent architecture, security, and code review pass. Final
documentation, repository, security, diff, and session-end checks pass. The
result is `PASS WITH ADVISORIES` for the two unavailable optional UI-tooling
checks and the non-required DMG packaging failure described above; no required
check failed and nothing was published.
