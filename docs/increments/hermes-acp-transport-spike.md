# Hermes ACP transport spike

Status: Verified complete with advisories; transport verdict NO GO
Date: 2026-08-11
Gate ID: `hermes-acp-transport-spike`
Plan: `docs/plans/2026-08-11-hermes-acp-transport-spike.md`
Baseline: clean synchronized `main` at `0446c02`

## Goal

Evaluate whether exact pinned Hermes ACP can provide a structured transport for
a future experimental runtime without bypassing Cortexa-owned validation,
policy, exact approval, restricted execution, cancellation, and audit.

## Boundaries

- Preserve the raw TUI-gateway stdio and managed WebSocket NO-GO evidence.
- Treat the operator candidate as read-only; do not install, update, repair, or
  mutate it.
- Use deterministic fixtures before any possible real-runtime test.
- Stop before real execution if provenance, dependencies, or capability
  containment fails.
- Keep Native sole/default; do not implement `HermesAgentRuntime` or Prompt 4D.

## Observed evidence

Official pinned source for Hermes Agent `0.20.0`, tag `v2026.8.3`, commit
`3c27eb6234bf91b8ceee9e9071591b31e9b148cb` confirms that ACP uses a public
newline-delimited JSON-RPC stdio launcher with initialization/version data,
structured sessions and updates, cancellation, and separate stderr logging.

The supplied candidate matches the pinned source identity and critical hashes,
but its 4,077-file virtual environment and external owner-writable Python
runtime lack a complete immutable content manifest. The installed environment
also lacks the pinned `agent-client-protocol==0.9.0` distribution.

Pinned source independently triggers the decisive NO-GO condition: each normal
ACP session hardcodes the broad `hermes-acp` toolset inside Hermes. It includes
terminal/process, filesystem mutation, browser, memory, skills, code execution,
and delegation. Selected terminal/edit permission callbacks and tool-progress
events do not place every effect behind Cortexa's deterministic pre-execution
governance, and no supported true zero-tool ACP mode exists.

Five deterministic fixture tests pass. They prove bounded host-side framing,
one fake text session, cancellation state handling, malformed/privileged update
rejection, isolation, and direct-child reap only. They are not Hermes
conformance or OS-containment evidence.

## Actions not taken

No Hermes executable, Python interpreter, ACP import, `--version`, `--check`,
server, session, provider, credential, model, tool, socket, package manager,
updater, or normal profile was executed or accessed. No production source,
manifest, lockfile, dependency, Tauri capability, UI, runtime selector,
adapter, or Native behavior changed.

## Result

The transport verdict is **NO GO**. ACP is materially better than raw TUI stdio
as a wire protocol and avoids the WebSocket listener/token surface, but it does
not satisfy Cortexa's authority boundary at the exact pinned release. D-081
records the result, the ACP ADR is Rejected, and the Hermes adapter remains
Draft/Blocked with no selected transport. Native remains sole/default and
Prompt 4D was not started.

The engineering closeout passes with advisories and the completion marker is
valid. That quality result must not be mistaken for a positive transport
verdict.

## Rollback and cleanup

Revert only this increment's ACP fixture/test, ADR/plan/report, D-081, rejected
WebSocket status update, and additive current-memory records. Preserve earlier
published spike evidence. Test temporary directories were cleaned up. No
product migration or external integration state exists to undo.
