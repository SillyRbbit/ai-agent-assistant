# Hermes ACP transport spike

Date: 2026-08-11
Verdict: **NO GO**
Plan: `docs/plans/2026-08-11-hermes-acp-transport-spike.md`

## Executive result

ACP is a public structured transport at the pinned Hermes release, but it fails
Cortexa's runtime-governance requirement. Exact pinned source hardcodes the
broad `hermes-acp` toolset inside the Hermes `AIAgent`; that set includes
terminal/process, filesystem mutation, browser, memory, skills, code execution,
and delegation. ACP reports tool activity and supports selected terminal/edit
permission callbacks, but Cortexa cannot intercept and authorize every effect
before Hermes executes it internally.

The supplied candidate also remains ineligible for execution. Its complete
Python runtime is not content-manifested and the required pinned
`agent-client-protocol==0.9.0` distribution is absent. No install, update, or
repair was authorized. The real Hermes executable was not run.

Five deterministic fixture tests demonstrate bounded host-side ACP framing,
session/text mechanics, cancellation handling, stderr separation, isolation,
malformed/privileged update rejection, failure closure, and direct-child reap.
They do not prove Hermes conformance, tool containment, full process-tree
cleanup, or candidate packaging.

## Exact version and source inspected

- package/application version: `0.20.0`;
- release tag: `v2026.8.3`;
- source commit: `3c27eb6234bf91b8ceee9e9071591b31e9b148cb`;
- source origin: `https://github.com/NousResearch/hermes-agent.git`;
- source archive SHA-256:
  `1e9319c58a7f5e95808546af1091d58472be7437adc63fae0cbb53316e2711aa`;
- `pyproject.toml` SHA-256:
  `64d1085ee1c23caf0ae0d9e65c73e280f466362ed43fdda1531f18f3af1d9869`;
- `uv.lock` SHA-256:
  `aab3c83f71b683507a590b6315b23bdc0abd6b63b76b2349eae15bf00dfbaf2b`;
- main `hermes` launcher SHA-256:
  `6ba08e9b84ce23d42175b873a18bad7ebeeb1bfb782fdbbf57400b0fb2abaa0c`;
- `hermes-acp` launcher SHA-256:
  `65ae3b1bf7ec0786b5ca0cb83c7ee2dfea0a9bc62832caecbca9dedc164fd5b4`;
- pinned optional dependency: `agent-client-protocol==0.9.0`, wheel SHA-256
  `06911500b51d8cb69112544e2be01fc5e7db39ef88fecbc3848c5c6f194798ee`.

Official pinned files and symbols inspected include:

- `website/docs/developer-guide/acp-internals.md` and
  `website/docs/user-guide/features/acp.md`;
- `pyproject.toml` ACP extra and `hermes-acp` script;
- `hermes_cli/subcommands/acp.py::build_acp_parser`;
- `hermes_cli/main.py::cmd_acp` and top-level startup/env path;
- `acp_adapter/entry.py::{main,_load_env,_run_check,_print_version}`;
- `acp_adapter/server.py::HermesACPAgent::{initialize,authenticate,
new_session,load_session,resume_session,fork_session,list_sessions,prompt,
cancel}`;
- `acp_adapter/session.py::{SessionManager,SessionState,
_expand_acp_enabled_toolsets}` and `SessionManager::_make_agent`;
- `acp_adapter/events.py`, `permissions.py`, `tools.py`, and `auth.py`;
- `toolsets.py` definition of `hermes-acp`;
- `hermes_cli/env_loader.py` and `hermes_cli/mcp_startup.py`; and
- pinned tests under `tests/acp/`.

## Candidate provenance and dependency result

The operator-supplied path is omitted because it contains a local username. The
candidate source checkout is detached, clean, on the exact tag and commit, and
uses the official origin. The same transport-independent provenance failure
recorded by the WebSocket spike still applies:

- the virtual environment contains 4,077 files and three symlinks;
- only selected launchers and critical files have supplied/observed hashes;
- `.venv/bin/python` resolves to an external owner-writable Python 3.13.15
  interpreter whose runtime and standard library are not in a complete
  immutable manifest; and
- the enclosing distribution root is owner-writable, so replacement-race
  closure is unproved.

The installed environment has 61 distribution metadata directories. Neither an
ACP module/package directory nor `agent-client-protocol` distribution metadata
is present, even though `pyproject.toml` and `uv.lock` pin version `0.9.0`. The
existing `hermes-acp` launcher would reach `import acp` during `--check` or
startup and is therefore incomplete. Missing ACP support is an ineligible
candidate result, not permission to install it.

No candidate executable, Python interpreter, ACP import, `--version`, `--check`,
or server command was run. No candidate file or permission changed.

## Official ACP contract verified

Pinned source documents and implements this public shape:

```text
hermes acp
hermes-acp
python -m acp_adapter
```

The adapter uses newline-delimited JSON-RPC over stdin/stdout. Human-readable
logging is configured for stderr. `HermesACPAgent.initialize` returns ACP
protocol version, Hermes implementation name/version, declared capabilities,
and authentication methods. Session methods include new, load, resume, fork,
list, prompt, and cancel. Prompt execution runs the synchronous `AIAgent` in a
thread pool and emits structured `session/update` events for text, thought,
tool progress, plans, and editor content. `cancel` sets a session event and
requests a hard agent interrupt.

The process ends when the stdio agent loop ends or the host terminates the
child. No ACP-specific containment-wide shutdown and descendant-reap contract
was found. `SessionManager.cleanup` exists but is not evidence of an OS-level
process-tree boundary. The adapter opts into `use_unstable_protocol=True`, so
the pinned SDK and exact message fixtures remain compatibility-sensitive.

## ACP compared with earlier mechanisms

### Raw TUI-gateway stdio

ACP is materially different and stronger as a wire contract. It has a public
launcher, standardized JSON-RPC methods, an initialization handshake with
protocol and implementation version, capability advertisement, session
lifecycle, structured updates, and cancellation. The rejected raw TUI-gateway
stdio path has no supported public raw launcher, initial version/capability
negotiation, or gateway shutdown RPC.

Both are stdio subprocess protocols, but they are not the same protocol. ACP's
better wire shape does not compensate for its internal privileged tool loop.

### Managed `hermes serve` WebSocket

ACP avoids the broad HTTP server, authenticated WebSocket listener, dynamic
port, and associated inbound network policy. It retains Hermes configuration,
provider, persistence, tool, child-process, filesystem, credential, MCP, skill,
memory, and plugin behavior. The primary authority problem therefore remains.

### OpenAI-compatible HTTP

That interface is provider-shaped, not a runtime lifecycle boundary. It does
not supply application-owned control over Hermes's internal tools and would
blur the separately governed provider boundary. It was not tested or selected.

## Process and protocol fixture evidence

The new Rust integration target launches only a deterministic Python fixture
with direct argv. It clears the environment, uses a temporary home, Hermes
home, working directory, and XDG roots, sets bytecode/user-site/safe-path
controls, and passes prompt text only inside serialized JSON.

Five ordinary tests passed and demonstrate:

- fixed version and check probes against the fixture;
- bounded initialization with ACP protocol version 1 and fixture version;
- correlated `session/new` and `session/prompt` requests;
- one streamed text `session/update` and terminal `end_turn` result;
- protocol-only stdout and separately bounded/truncated stderr;
- startup and in-run timeout closure;
- malformed and oversized stdout rejection;
- unknown-method error handling;
- tool update, wrong-session, wrong-ID, late-output, early-exit, and midstream
  exit rejection;
- idempotent local cancellation decision with a terminal cancelled result;
- inert shell metacharacters in prompt content; and
- direct-child EOF shutdown, kill, wait, and reap.

The fixture is not Hermes and does not establish ACP conformance, real version
behavior, imports, provider behavior, filesystem access denial, network denial,
Keychain denial, package-manager denial, descendant cleanup, or OS containment.

## Isolation, state, and credential findings

- `acp_adapter.entry._load_env` loads the standard Hermes dotenv path.
- The main `hermes acp` launcher traverses broader `hermes_cli.main` startup
  before subcommand dispatch, including top-level dotenv/config and recovery
  behavior; `--version` and `--check` are not assumed side-effect-free under the
  unmanifested runtime.
- Provider detection and session construction reuse Hermes runtime provider
  resolution and its configured credentials.
- ACP advertises an interactive terminal setup authentication method when a
  provider is not configured.
- ACP sessions use a `SessionDB` at the Hermes home and bind the client-supplied
  working directory to terminal/file tool context.
- Configured MCP discovery starts unless one exact host environment marker is
  set; session-supplied MCP servers and later refresh behavior remain separate.
- Stderr logs session and prompt diagnostics; pinned prompt logging includes a
  prompt prefix and exceptions may include raw text. Stderr therefore requires
  bounding and redaction rather than being treated as safe merely because it is
  separate from stdout.
- Cancellation requests agent interruption but does not prove cleanup of
  terminal/process/execute-code/delegation children or detached descendants.

No normal Hermes home, profile, dotenv, provider credential, state database,
memory, skill, plugin, or MCP configuration was read during this spike.

## Tool and capability containment result

**Failed — decisive NO GO.** `SessionManager._make_agent` constructs every
normal ACP session with `enabled_toolsets=["hermes-acp"]`. The pinned
`hermes-acp` toolset includes:

- `terminal` and `process`;
- `read_file`, `write_file`, `patch`, and `search_files`;
- web and browser tools;
- `memory`, `skills_list`, `skill_view`, and `skill_manage`;
- `execute_code`; and
- `delegate_task`.

ACP permission bridging covers dangerous terminal commands and selected file
edits. It does not transfer every tool through Cortexa's registered schema,
deterministic policy, exact approval, restricted executor, and audit chain.
Tool progress events describe Hermes-owned execution; rejecting those events in
the host cannot undo an effect that already occurred.

No supported ACP launcher flag, client capability, session parameter, or
`platform_toolsets.acp` configuration narrows session construction to a true
zero-tool conversation runtime at this release. Config/allowlist workarounds
would be Hermes-controlled defense in depth, not the required application-owned
authority boundary. Exact whole-process containment and detached-descendant
cleanup also remain unproved on the target Mac.

Therefore Hermes could silently exercise privileged host capabilities before
Cortexa intercepts and authorizes them. This satisfies the prompt's mandatory
NO GO condition without a real runtime launch.

## Cancellation, failure, and shutdown

Fixture evidence proves only host state handling: one cancellation request,
one cancelled terminal result, late-output rejection, timeout closure, and
direct-child reap. Pinned source sets a cancel event and calls the agent's hard
interrupt compatibility helper. It does not establish rollback of prior side
effects, process-tree cleanup, or containment-wide shutdown. No real
cancellation or shutdown was attempted.

## Validation results

- candidate source/tag/commit/origin and clean Git checks: passed;
- source archive and critical-file hashes: passed;
- complete interpreter/runtime manifest: failed;
- required installed `agent-client-protocol==0.9.0`: failed/absent;
- supported zero-tool ACP mode: failed/not present;
- application-owned pre-execution governance for all tools: failed;
- exact target-Mac whole-process containment: failed/not available;
- deterministic ACP fixture syntax: passed;
- deterministic ACP fixture tests: 5 passed, 0 failed;
- real Hermes version/check/start/session/prompt/cancel/shutdown: not run;
- dedicated provider credential: not required because the earlier NO GO stop
  condition fired;
- existing raw-stdio fixture and Native runtime regressions: recorded by the
  associated post-increment review.

## Limitations and portability/packaging concerns

- A cooperative fixture does not prove real Hermes behavior.
- ACP wire version and SDK package version are separate compatibility axes;
  Hermes uses unstable protocol extensions.
- The editable Python source, external interpreter, optional dependency, native
  wheels, platform subprocess semantics, and process-tree cleanup would all
  require immutable packaging and target-platform evidence.
- Windows and Linux would need their own process, path, environment, and
  containment evidence; target-Mac evidence does not generalize.
- A future upstream release could materially change this outcome, but only a
  new owner-approved evaluation may establish that.

## Verdict and adapter implication

**NO GO** for Hermes ACP at version `0.20.0` / tag `v2026.8.3` / commit
`3c27eb6234bf91b8ceee9e9071591b31e9b148cb` under Cortexa's current runtime
governance requirements.

The ACP ADR is Rejected. `HermesAgentRuntime` remains Draft/Blocked. Do not
automatically evaluate another transport. Hermes may not currently satisfy the
project's external-runtime governance requirements without a separately
approved, fully content-manifested build that provides an upstream/enforced
conversation-only ACP mode and exact target-platform containment.

## Production and cleanup confirmation

No Hermes executable, Python runtime, ACP import, server, session, provider,
credential, model, tool, socket, package manager, updater, or normal profile was
executed or accessed. No production source, manifest, lockfile, dependency,
Tauri capability, UI, runtime selector, or Native behavior changed. Fixture
temporary directories were removed by test cleanup. Prompt 4D and
`HermesAgentRuntime` implementation were not started.
