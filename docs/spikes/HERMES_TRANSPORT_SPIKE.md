# Hermes transport spike

Status: Complete; selected raw TUI-gateway stdio mechanism is **NO-GO** for a
production adapter
Spike date: 2026-08-11
Repository baseline:
`f5b3fbe1b50f3269d6843002f0d1c81fe3e9b770`

This is isolated experiment evidence, not a production adapter, runtime,
provider, containment boundary, or implementation authorization. Hermes was not
installed or executed. Native Cortexa source and behavior were not changed.

> **Subsequent decision (2026-08-11):** D-080 preserves this raw-stdio NO-GO
> and conditionally selects managed local `hermes serve` plus the documented
> TUI-gateway JSON-RPC/WebSocket surface for a new contained spike after the
> native runtime boundary is verified. This notice does not change the evidence
> or verdict below and authorizes no Hermes execution or adapter.

## Verdict

**NO-GO for the selected production mechanism at the evaluated release.**

Hermes Agent `0.20.0` documents TUI-gateway JSON-RPC over stdio and its tagged
source implements the wire, but it does not publish a supported raw-gateway
`hermes` command. The only raw stdio launch found is the internal Python module
`python -m tui_gateway.entry`, used behind the Node/Ink TUI. The initial ready
event carries neither a Hermes version nor a protocol version, and there is no
gateway-shutdown RPC. The preferred transport in the Proposed ADR therefore
does not meet the assessment's own supported-launcher and versioned-contract
conditions.

The framing and host-side failure mechanics are viable in an isolated fixture.
That is not enough to promote an internal module path into a production
contract. Stop before a Hermes adapter and revise the Proposed ADR to compare
only supported public entry points, principally `hermes acp` and a strictly
contained loopback `hermes serve` surface, or retain native-only architecture.

## Version and source inspected

The evaluated upstream is the official latest GitHub release as observed on
2026-08-11:

- product version: `0.20.0`;
- signed release tag: `v2026.8.3`;
- release commit:
  `3c27eb6234bf91b8ceee9e9071591b31e9b148cb`;
- release date: 2026-08-03;
- source repository:
  [NousResearch/hermes-agent](https://github.com/NousResearch/hermes-agent);
- release evidence:
  [Hermes Agent v0.20.0](https://github.com/NousResearch/hermes-agent/releases/tag/v2026.8.3);
- pinned package metadata:
  [`pyproject.toml`](https://github.com/NousResearch/hermes-agent/blob/v2026.8.3/pyproject.toml);
- pinned protocol guide:
  [Programmatic Integration](https://github.com/NousResearch/hermes-agent/blob/v2026.8.3/website/docs/developer-guide/programmatic-integration.md);
- pinned raw gateway implementation:
  [`tui_gateway/server.py`](https://github.com/NousResearch/hermes-agent/blob/v2026.8.3/tui_gateway/server.py)
  and
  [`tui_gateway/entry.py`](https://github.com/NousResearch/hermes-agent/blob/v2026.8.3/tui_gateway/entry.py);
- pinned security model:
  [`SECURITY.md`](https://github.com/NousResearch/hermes-agent/blob/v2026.8.3/SECURITY.md).

Mutable `main` was not treated as the evaluated contract. The PyPI distribution
was still `0.19.0` during the preceding assessment, so this spike does not infer
that a PyPI install matches the GitHub release.

## Repository and runtime baseline

- No `AgentRuntime`, `NativeAgentRuntime`, or `HermesAgentRuntime` implementation
  exists. The names remain planned concepts in `ARCHITECTURE.md` and D-078.
- `InitialGatewayTurn` is a transport-free, test-consumed native Rust boundary,
  not a shipping runtime coordinator.
- The desktop exposes no agent/runtime/provider/process Tauri command. Its
  visible assistant remains a deterministic no-I/O frontend mock.
- Local discovery found no `hermes` executable on `PATH`. The spike did not
  download, install, update, import, or launch Hermes.
- The multi-runtime ADR remains Proposed. This result does not accept it.

## Integration mechanism evaluated

The selected mechanism was a version-pinned managed child process with a small
host-owned projection of Hermes's newline-delimited JSON-RPC 2.0 TUI gateway
over stdin/stdout.

This is a structured gateway protocol, not MCP and not human chat-output
parsing. Stderr is a separate diagnostic channel. Version detection is separate:
because the gateway handshake has no version, a host would have to run a bounded
human-oriented `hermes --version` probe and pin its accepted output. That parser
is intentionally isolated in the test-only harness and is not production-ready.

### Upstream commands and internal entry points

| Surface               | Evaluated invocation          | Result                                                                                                                       |
| --------------------- | ----------------------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| TUI gateway raw stdio | `python -m tui_gateway.entry` | Implemented internal module used by the TUI; no supported public `hermes` subcommand found.                                  |
| TUI application       | `hermes --tui`                | Public Node/Ink UI wrapper; not a raw JSON-RPC server for an external host.                                                  |
| ACP                   | `hermes acp`                  | Supported public JSON-RPC stdio entry point, but a different and broader protocol that this spike did not select.            |
| API/backend           | `hermes serve`                | Supported public loopback HTTP/WebSocket surface with additional listener, authentication, and network-local attack surface. |
| Version probe         | `hermes --version`            | Human CLI output, separate from gateway negotiation; requires an exact bounded parser.                                       |

No shell command construction is part of the test harness. Executables are
passed directly to `std::process::Command` with fixed argument vectors.

## Wire evidence

The pinned source's first gateway frame has this conceptual shape:

```json
{
  "jsonrpc": "2.0",
  "method": "event",
  "params": {
    "type": "gateway.ready",
    "payload": {
      "skin": {},
      "change_events": true
    }
  }
}
```

It proves that the process entered its input loop. It does not identify Hermes
`0.20.0`, a TUI protocol version, a capability set, disabled features, a build
hash, or a compatibility range.

The narrow lifecycle projection evaluated by the fixture is:

```text
gateway.ready
  -> session.create
  -> prompt.submit
  -> message.delta
  -> message.complete
  -> session.close
  -> stdin EOF / child exit
```

Cancellation uses `session.interrupt`; the tagged implementation returns an
immediate interrupted result and later emits terminal
`message.complete` state. `process.stop` is not gateway shutdown: it concerns
agent-launched background processes. The raw gateway exits on stdin EOF or an
operating-system signal.

The source deliberately reserves real stdout for JSON-RPC and redirects
ordinary Python stdout to stderr, which supports transport/log separation.
However, raw stderr and upstream error messages remain untrusted and potentially
sensitive; the test host retains only a bounded byte count and closed outcome.

## Capability results

| Question               | Result                                      | Evidence and limitation                                                                                                                                     |
| ---------------------- | ------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Runtime discovery      | **Partial**                                 | An explicit absolute executable can be validated. Automatic `PATH` discovery is intentionally absent. No immutable Hermes artifact/hash is installed.       |
| Version detection      | **Partial / fragile**                       | Bounded `--version` parsing works against the fixture. The gateway itself has no initial version negotiation; real Hermes probe is ignored and unavailable. |
| Health/readiness       | **Partial**                                 | `gateway.ready` is machine-readable, but it is not a versioned capability or disabled-feature attestation.                                                  |
| Startup/connect        | **NO-GO as selected**                       | Raw stdio requires the internal Python module; no supported public raw-gateway command was found.                                                           |
| One basic session      | **Fixture only**                            | The deterministic child proves request correlation and lifecycle mechanics, not Hermes conformance.                                                         |
| Text request/response  | **Fixture only**                            | Bounded text deltas and completion are demonstrated without a model, provider, network, or Hermes.                                                          |
| Timeout                | **Demonstrated in host fixture**            | Readiness and in-run deadlines terminate and reap the direct child.                                                                                         |
| Cancellation           | **Protocol evidence plus fixture**          | `session.interrupt` exists; fixture verifies terminal/idempotent local handling and late-output rejection. No real provider cancellation was attempted.     |
| Clean shutdown         | **Partial**                                 | Session close plus stdin EOF produces clean fixture exit. Upstream publishes no gateway-shutdown RPC.                                                       |
| Malformed response     | **Demonstrated in host fixture**            | Malformed, oversized, unknown, forbidden, and identity-mismatched frames fail closed.                                                                       |
| Unexpected termination | **Demonstrated in host fixture**            | Pre-ready and midstream exits become closed failures; partial text never becomes completion.                                                                |
| Stderr/log separation  | **Demonstrated in host fixture and source** | Stdout framing is separate; host diagnostics keep counts only. This is not proof that every upstream log is content-free.                                   |
| Secret redaction       | **Host-side fixture only**                  | Synthetic sentinels do not enter public errors or debug text. Upstream redaction is a heuristic, not a security boundary.                                   |

## Experimental files

- `src-tauri/tests/hermes_transport_spike.rs`: test-only closed child/protocol
  harness and ignored real-version probe.
- `src-tauri/tests/fixtures/hermes_tui_gateway_stub.py`: deterministic no-network
  child that emits the pinned gateway shapes and failure scenarios.

Neither file is linked into the application. No production crate, manifest,
lockfile, dependency, feature, Tauri surface, or frontend path changed.

## Test evidence

Ordinary tests use only the local fixture. They do not use the network, a model,
Hermes, user content, provider credentials, global Hermes state, host tools,
memory, skills, plugins, subagents, schedules, messaging, or cloud actions.

The actual-Hermes test is explicitly ignored, version-only, requires an
operator-supplied absolute executable and opt-in environment flag, and returns
without execution when unavailable. It was not run because Hermes is absent and
whole-process containment is not approved.

Final command results are recorded in
`docs/reviews/2026-08-11-hermes-transport-spike-post-increment-review.md`.

## Failure modes

- Internal module names, import paths, method payloads, or event order can
  change without a public compatibility promise.
- `gateway.ready` can be accepted from the wrong Hermes version because it has
  no runtime/protocol identity.
- A later `session.info` version is too late and not side-effect-free: agent
  construction can read configuration, initialize state, discover MCP, load
  skills/tools, or perform other setup before the host learns the version.
- The internal server loads Hermes home/configuration and dotenv behavior at
  import, starts optional MCP discovery at entry, maintains persistent session
  state, and contains broad methods including config, CLI, approvals, secrets,
  tools, delegation, and process control.
- Stdin EOF or a signal is a transport/process convention, not an acknowledged
  versioned gateway shutdown transaction.
- Cancellation can race with final output or an upstream call that does not
  interrupt promptly. Host terminal state must remain authoritative.
- A child can hang, flood a pipe, emit invalid UTF-8/JSON, exit after partial
  text, write sensitive diagnostics, spawn descendants, or outlive its parent.
- A version probe can be replaced between validation and execution unless the
  artifact is immutable and hash-verified.

## Portability concerns

- The internal entry requires a compatible Python environment and repository or
  installed module layout; Python support is `>=3.11,<3.14` for the evaluated
  release.
- `hermes --tui` also depends on the Node/Ink bundle and is not equivalent to a
  headless raw-gateway executable.
- Process-group ownership, signal behavior, EOF handling, executable identity,
  filesystem sandboxing, network policy, and child-tree cleanup differ across
  macOS, Linux, and Windows.
- The test harness proves only direct-child cleanup. It does not prove process-
  tree cleanup or a target-Mac whole-process wrapper.
- A loopback alternative adds port selection, bind, token, origin, local-client,
  reconnect, and shutdown semantics; ACP has different method/capability
  semantics. Either needs a fresh spike and threat review.

## Packaging concerns

- GitHub `v2026.8.3` and PyPI `0.19.0` are different distribution states.
- A real bundle would include Python, exact packages, optional/native wheels,
  possibly Node assets, licenses/notices, SBOM, signing/notarization, update,
  rollback, repair, uninstall, and runtime-data deletion responsibilities.
- The internal module entry offers no stable standalone artifact contract.
- `hermes update`, mutable `main`, lazy installers, plugins, skills, and MCP
  packages cannot participate in a reproducible Cortexa runtime.
- No dependency or Hermes artifact was added by this spike.

## Security observations

- Upstream explicitly treats whole-process OS isolation as the load-bearing
  boundary. Environment clearing and a temporary home/cwd are not containment.
- The ordinary fixture command starts from `env_clear`, passes only synthetic
  explicit variables, and uses an isolated temporary home/cwd. The macOS
  system Python launcher adds non-secret SDK/locale key names such as `CPATH`,
  `SDKROOT`, and `LC_CTYPE`; the test records names only and rejects credential,
  proxy, provider, token, and `PATH` keys. No environment value is printed.
- Prompt text is sent in a JSON frame, never interpolated into argv or a shell.
- Stdout lines, event count, message size, stderr bytes, identifiers, methods,
  events, and deadlines are bounded. Unknown or privileged event families fail
  closed.
- The harness never forwards approval, secret, sudo, config, CLI, tool, MCP,
  skill, plugin, memory, subagent, scheduling, messaging, browser, filesystem,
  shell, or cloud methods.
- `env_clear`, isolated paths, direct-child `kill`/`wait`, and redaction cannot
  prevent absolute filesystem access, network access, Keychain access, process
  inspection, descendants, or malicious in-process Python behavior.
- No real Hermes start/session/text test is safe before a separately approved
  whole-process containment profile, immutable artifact/hash, no-capability
  configuration proof, provider/disclosure gate, and target-platform evidence.

## Production-adapter recommendation

Do not implement `HermesAgentRuntime` against raw TUI-gateway stdio at
`v2026.8.3`.

Revise the Proposed ADR before acceptance. The revision should either:

1. retain native-only architecture until Hermes publishes a supported,
   version-negotiated, capability-bounded raw stdio entry point;
2. evaluate `hermes acp` as the supported stdio candidate and prove that its
   lifecycle can be projected without tool/permission/authorization leakage; or
3. evaluate `hermes serve` inside a verified whole-process sandbox with a
   loopback-only authenticated listener and a closed WebSocket/HTTP projection.

That future decision must preserve native as the default/reference path, keep
the provider boundary separate, prohibit automatic fallback, and leave
validation, policy, approval, restricted execution, cancellation truth, audit,
secrets, and device authority in Cortexa-owned Rust. This spike authorizes none
of those follow-ups.

## Cleanup and rollback

The experiment has no external runtime or persistent data to clean up. Rollback
is a bounded reverse patch removing the test target, fixture, this report, its
plan/increment/review records, and additive current-memory notices. No Hermes
package, home, credential, model data, application database, production source,
manifest, or lockfile was created or modified.
