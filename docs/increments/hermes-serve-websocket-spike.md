# Hermes serve WebSocket containment spike

Status: Verified complete with advisories; spike verdict FAIL / NO-GO at
Milestone 0
Date: 2026-08-11
Gate ID: `hermes-serve-websocket-spike`
Plan: `docs/plans/2026-08-11-hermes-serve-websocket-spike.md`
Baseline: clean synchronized `main` at `293aa04`

## Goal

Prove or disprove whether the pinned public `hermes serve` launcher and
TUI-gateway JSON-RPC/WebSocket protocol can support a conversation-only
experimental transport inside the approved target-Mac containment boundary,
without changing production behavior or beginning `HermesAgentRuntime`.

## Boundaries

- Treat the operator-supplied distribution as read-only.
- Do not install, update, repair, change permissions, or use another Hermes.
- Use no normal Hermes profile, credentials, external model, or provider.
- Require complete runtime provenance and exact containment before launch.
- Stop before the fake harness and real runtime if Milestone 0 fails.
- Keep Native sole/default and Prompt 4D blocked.

## Observed Milestone 0 evidence

Static checks passed for the official source origin, detached clean commit
`3c27eb6234bf91b8ceee9e9071591b31e9b148cb`, exact tag `v2026.8.3`, package
version `0.20.0`, source archive digest, three critical-file hashes, and all 61
installed package metadata records. A sanitized isolated Python `-I`
metadata/module-discovery check found the required FastAPI, Uvicorn, Starlette,
multipart, WebSocket, PTY, Hermes CLI, and TUI-gateway modules. Critical hashes
and clean Git state still passed afterward.

The supplied provenance was incomplete for this plan: only one of 4,077
regular virtual-environment files was hashed, and the launcher uses an external
owner-writable Python runtime absent from the supplied content manifest. The
editable source loaders, installed package contents, standard library, and
replacement-race boundary were not fully attested.

Pinned source review found no supported startup mode that disables update
prefetch, dotenv/managed-secret initialization, credential keepalive, plugin
discovery, skill synchronization, and every tool. A credential-free loopback
fake provider is supported, but that does not remove those startup paths.

Target-Mac security review found that deprecated `sandbox-exec` alone cannot
prove exact port-zero listener restriction, package-manager execution denial,
blanket Unix-socket denial, or containment membership and cleanup for detached
descendants. These findings triggered the approved stop conditions.

## Actions not taken

No Hermes executable/server process, listener, HTTP request, WebSocket frame,
session, prompt, provider, model, tool, credential, plugin, skill, child, or
detached descendant was started. No fake server, Rust harness, ignored real
test, production source, dependency, manifest, lockfile, Tauri capability, UI,
runtime selector, adapter, or external state was added.

## Result

The spike verdict is **FAIL / NO-GO** and next-increment readiness is
**Blocked**. This is a NO-GO for the selected contained integration under the
approved D-080 conditions, not dynamic proof that the WebSocket wire itself
fails. The bounded engineering closeout is **PASS WITH ADVISORIES** because the
prove-or-disprove objective reached a safe evidence-backed negative answer.
D-080 is not silently switched to another transport. The Draft Hermes adapter
plan remains Blocked and Native remains sole/default.

## Rollback and cleanup

Revert only this plan/report/current-memory documentation. The disposable
preflight home was removed. Candidate critical hashes and source cleanliness
were rechecked. No product or external migration exists to undo.
