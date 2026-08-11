# Hermes transport spike

Status: Verified complete with advisories
Date: 2026-08-11
Gate ID: `hermes-transport-spike`
Plan: `docs/plans/2026-08-11-hermes-transport-spike.md`
Baseline: synchronized `main` at
`f5b3fbe1b50f3269d6843002f0d1c81fe3e9b770`

## Goal

Prove or disprove the recommended Hermes integration mechanism without
installing Hermes, connecting an external runtime to Cortexa, or changing
native application behavior.

## Authorized boundaries

- Inspect official tagged Hermes documentation and source for one exact
  release.
- Add one Unix-only Rust integration-test target and one deterministic Python
  fixture under `src-tauri/tests/`.
- Exercise only fake newline-delimited JSON-RPC process mechanics in ordinary
  tests.
- Keep any real-Hermes test ignored, explicit, version-only, and unavailable by
  default.
- Add the spike report, additive assessment/ADR notices, plan, review, and
  current-memory synchronization.
- Do not add or run Hermes, a production runtime, provider, dependency,
  manifest, feature, UI, IPC, permission, tool, credential, network path, or
  external action.

## Evidence basis

Official evidence is pinned to Hermes Agent `0.20.0`, signed tag `v2026.8.3`,
release commit `3c27eb6234bf91b8ceee9e9071591b31e9b148cb`, and the tagged programmatic
guide, TUI gateway source, TUI client, package metadata, and security policy.
The tagged guide describes TUI-gateway JSON-RPC over stdio, but the release
publishes no public command for a raw TUI-gateway child. The only raw launch
found is the internal `python -m tui_gateway.entry` module used by Hermes's own
Node/Ink TUI.

Repository evidence confirms there is no implemented `AgentRuntime`,
`NativeAgentRuntime`, or `HermesAgentRuntime`; the Tauri application has no
agent/runtime/process IPC; and the visible assistant remains a deterministic
no-I/O mock.

## Actual result

The selected raw TUI-gateway stdio mechanism is **NO-GO as a supported
production contract** at the pinned release. It lacks a public raw launcher, an
initial version/capability negotiation, and a gateway-shutdown RPC. This result
does not reject Hermes generally and does not select ACP or `hermes serve`.

The fixture-led host mechanics pass: explicit executable/version validation,
ready/session/text lifecycle, bounded frames and event count, readiness/run
deadlines, terminal idempotent cancellation with late-output rejection, clean
EOF shutdown, malformed/oversized/forbidden/identity rejection, early and
midstream exit mapping, count-only bounded stderr, closed errors, environment
isolation, and direct-child reap. Seven ordinary tests pass and one real-Hermes
version probe is ignored. The fixture is not Hermes conformance, containment,
descendant-cleanup, packaging, or target-platform evidence.

No production source, dependency, manifest, lockfile, feature, UI, IPC, Tauri
permission, runtime behavior, native behavior, or external state changed.
Hermes was not installed or executed. The multi-runtime ADR remains Proposed
and must be revised before it could be accepted.

## Residual advisories

- `env_clear`, a temporary home/cwd, and direct-child kill/reap are not OS
  containment and do not deny absolute filesystem, network, process, Keychain,
  or descendant access.
- The test harness is Unix-only and uses the repository's system Python solely
  for a cooperative fake. Cross-platform production process ownership remains
  unproved.
- The ignored real-version probe validates a bounded human CLI line, not a
  machine-negotiated protocol or immutable artifact hash.
- ACP and `hermes serve` have different protocol, listener, authentication,
  lifecycle, and security consequences and require separate owner-selected
  analysis if considered.

## Rollback

Remove the two test-only files, spike report, this increment/review, plan, and
additive memory notices. No Hermes installation, dependency, runtime state,
credential, application database, manifest, lockfile, or external resource
requires cleanup.
