# Hermes ACP transport spike

Status: Verified complete with advisories; transport verdict NO GO
Owner: Project owner
Last updated: 2026-08-11

This is an isolated architecture and transport spike. It does not implement or
wire `HermesAgentRuntime` and does not modify `NativeAgentRuntime`.

## Goal

Prove or disprove whether the exact pinned Hermes ACP server can support a
future experimental runtime adapter without bypassing Cortexa-owned validation,
policy, approval, restricted execution, cancellation, and audit authority.

## User-visible outcome

None. Native remains sole/default and the desktop application behaves exactly
as before. The output is a proposed/rejected/accepted ACP ADR, a deterministic
fixture-only transport harness, and an evidence report with exactly PASS, PASS
WITH ADVISORIES, NO GO, or BLOCKED.

## Scope

- Preserve the raw-stdio and managed-WebSocket failures as historical evidence.
- Mark the WebSocket transport ADR rejected after its completed NO-GO spike.
- Inspect exact official tagged ACP source and documentation.
- Create the Proposed ACP ADR and this living plan.
- Add an isolated deterministic ACP stdio fixture and Rust integration-test
  target with no production dependency or application wiring.
- Verify the operator-supplied candidate through read-only provenance,
  dependency, and source inspection before any possible execution.
- Stop before real Hermes execution if provenance, imports, startup, or
  capability containment fails.
- Record the ACP verdict and synchronize current project memory.

## Explicit non-goals

- No `HermesAgentRuntime`, runtime selector, Tauri command/event, React/UI,
  provider, credential, model, production process, network, or visible behavior.
- No change to `AgentRuntime`, `NativeAgentRuntime`, `InitialGatewayTurn`, the
  deterministic frontend mock, policy, approval, audit, tools, or execution.
- No Hermes or ACP installation, update, repair, bootstrap, package download,
  permission change, or candidate mutation.
- No owner profile, `.env`, credential, normal session, memory, skill, plugin,
  MCP server, Keychain item, or personal content.
- No shell interpolation, destructive canary action, live provider, external
  network, OpenAI-compatible HTTP evaluation, raw TUI stdio reconsideration, or
  Prompt 4D.
- No commit, push, PR, merge, release, or publication without separate owner
  direction.

## Existing behavior and constraints

- D-079's application-owned runtime foundation and sole/default
  `NativeAgentRuntime` are verified and published.
- The raw TUI-gateway stdio mechanism is NO-GO at the pinned release.
- The D-080 managed `hermes serve` WebSocket spike is verified complete with an
  authoritative FAIL / NO-GO result at Milestone 0.
- The ACP ADR begins Proposed and cannot become Accepted unless this spike
  passes.
- The future adapter plan remains Draft/Blocked throughout this increment.
- Ordinary fixture tests prove host mechanics only; they cannot prove Hermes
  conformance, containment, tool absence, candidate provenance, or packaging.

## Current-state evidence

- Repository baseline: clean synchronized `main` at `0446c02` before the gate.
- Active gate: `hermes-acp-transport-spike`.
- Hermes package/application version: `0.20.0`.
- Release tag: `v2026.8.3`.
- Source commit: `3c27eb6234bf91b8ceee9e9071591b31e9b148cb`.
- Official origin: `https://github.com/NousResearch/hermes-agent.git`.
- Pinned ACP optional dependency: `agent-client-protocol==0.9.0`.
- Candidate provenance and import eligibility must be re-established from
  current read-only evidence; prior WebSocket evidence is not silently promoted
  to ACP success.

## Files expected to change

Experimental paths:

- `src-tauri/tests/hermes_acp_transport_spike.rs` (new)
- `src-tauri/tests/fixtures/hermes_acp_server_stub.py` (new)
- `docs/adr/ADR-HERMES-ACP-TRANSPORT.md` (new)
- `docs/plans/2026-08-11-hermes-acp-transport-spike.md` (new; this plan)
- `docs/spikes/HERMES_ACP_TRANSPORT_SPIKE.md` (new)

Architecture and durable decision paths:

- `docs/adr/ADR-HERMES-SERVE-WEBSOCKET-TRANSPORT.md`
- `docs/adr/ADR-MULTI-RUNTIME-AGENT-ARCHITECTURE.md`
- `docs/plans/2026-08-11-hermes-agent-runtime-adapter.md`
- `ARCHITECTURE.md`
- `DECISIONS.md`

Closeout paths:

- `PLANS.md`
- `HANDOFF.md`
- `PROJECT_STATUS.md`
- `NEXT_STEPS.md`
- `CHANGELOG.md`
- `docs/increments/hermes-acp-transport-spike.md` (new)
- `docs/reviews/2026-08-11-hermes-acp-transport-spike-post-increment-review.md`
  (new)

`TROUBLESHOOTING_LOG.md` changes only if a durable repository or environment
failure and resolution occurs. Production source, manifests, lockfiles,
dependencies, configuration, permissions, and UI paths must remain unchanged.

## Affected components and symbols

Pinned upstream read-only inspection:

- `acp_adapter.entry::{main,_parse_args,_print_version,_run_check,_load_env}`;
- `acp_adapter.server::HermesACPAgent::{initialize,authenticate,new_session,
load_session,resume_session,fork_session,list_sessions,prompt,cancel}`;
- `acp_adapter.session::{SessionManager,SessionState,
_expand_acp_enabled_toolsets}` and `SessionManager::_make_agent`;
- `acp_adapter.events` event translation;
- `acp_adapter.permissions::make_approval_callback`;
- `acp_adapter.tools` tool rendering;
- `hermes_cli.main::cmd_acp` and top-level startup/bootstrap/env behavior;
- `hermes_cli.env_loader::load_hermes_dotenv`;
- `hermes_cli.mcp_startup` discovery paths;
- `toolsets.py` definition of `hermes-acp`; and
- exact tagged ACP docs and upstream tests under `tests/acp/`.

Repository fixture boundary:

- direct-child process ownership and bounded stdin/stdout/stderr readers;
- closed ACP initialize/session/prompt/update/cancel result parsing;
- fixed deterministic scenarios for success, timeout, malformed output,
  forbidden tool update, unknown method, cancellation/late output, and crash;
- sanitized environment and temporary home/cwd evidence; and
- direct-child shutdown/reap only, explicitly not whole-process containment.

## Interfaces and compatibility invariants

- ACP framing is newline-delimited JSON-RPC 2.0 over stdio; stdout is protocol
  only and human logs belong on stderr.
- Only `initialize`, `session/new`, `session/prompt`, `session/cancel`, bounded
  `session/update` text chunks, and terminal prompt results are projected by the
  fixture harness.
- Unknown, malformed, oversized, wrong-ID, wrong-session, late, tool, approval,
  filesystem, terminal, browser, skill, memory, MCP, subagent, and other
  privileged frames fail closed.
- Fixture input is serialized as data and is never interpolated into a shell or
  command argument.
- The fixture performs no network, provider, model, tool, memory, credential,
  Hermes, package-manager, or external filesystem operation.
- Native remains default and no application caller consumes the spike.
- No fixture outcome can establish that the real Hermes process is safe.

## Implementation milestones

### Milestone 0 — provenance and static eligibility

- [ ] Verify exact executable identity, package version, tag, commit, source
      cleanliness, official origin, critical-file digests, ACP imports, and
      installed dependency inventory.
- [x] Confirm no installation, update, repair, or mutation occurred.
- [x] Stop before candidate execution if provenance or required ACP imports do
      not pass.

### Milestone 1 — deterministic process contract

- [x] Add fixture-only `--version`/`--check`, bounded startup, protocol-only
      stdout, bounded stderr, EOF shutdown, crash, timeout, and direct-child
      reap cases.
- [x] Record that a cooperative fixture cannot prove real-process conformance,
      detached-descendant cleanup, or OS containment.

### Milestone 2 — closed ACP protocol projection

- [x] Test initialization/version reporting, request correlation, session/new,
      prompt, streamed text update, terminal completion, error response,
      malformed JSON, disconnect, cancellation, and late-output rejection.
- [x] Reject tool/approval/privileged update families.

### Milestone 3 — isolated harness state

- [x] Use a temporary home, Hermes home, working directory, XDG roots, and
      sanitized allowlisted environment with bytecode and user-site disabled.
- [x] Verify fixture-observed paths and environment names without recording
      values or user-specific absolute paths.

### Milestone 4 — real capability-containment decision

- [x] Determine from exact pinned source whether conversation-only ACP can omit
      shell, filesystem, browser, memory, skills, code execution, delegation,
      MCP, updater, credential, and child-process authority.
- [x] Return NO GO if Hermes can execute those internally before Cortexa can
      validate and authorize them or if no supported zero-tool mode exists.

### Milestone 5 — minimal real-runtime evidence

- [ ] Run only if Milestones 0 through 4 pass. Use no normal profile or
      credential and stop BLOCKED if a dedicated provider credential is needed.

Not run. Milestone 0 failed and Milestone 4 independently produced the
mandatory NO GO result. No credential was requested or needed.

### Milestone 6 — decision and closeout

- [x] Record PASS, PASS WITH ADVISORIES, NO GO, or BLOCKED in the spike report.
- [x] Accept, reject, or retain Proposed ACP ADR exactly as the evidence permits.
- [x] Keep the adapter Draft/Blocked and synchronize project memory.
- [x] Run session-end, quality, and post-increment workflows.

## Security and privacy considerations

- The model and Hermes remain untrusted. ACP structure grants no authority.
- Candidate paths containing a local username are excluded from repository
  documentation; evidence uses `<operator-distribution-root>`.
- Do not print environment values, credentials, provider configuration, normal
  profile data, raw stderr, prompt content, or tool arguments.
- Use direct `Command` argv only; never use a shell.
- `env_clear`, a temporary home/cwd, and direct-child cleanup are isolation
  evidence, not OS containment.
- A permission notification is too late if Hermes already owns tool execution.
- Missing optional dependencies are a closed ineligible result, never permission
  to install.

## Test plan

- Fixture version/check success and mismatch/failure.
- Successful initialize/session/prompt/text-update/completion/EOF shutdown.
- Startup and in-run timeouts with direct-child reap.
- Cancellation is idempotent and terminal; late output is rejected.
- Unknown request errors, malformed/oversized frames, wrong identity/ID, and
  unexpected exit are closed typed failures.
- Tool/approval/privileged updates are rejected before leaving the harness.
- Environment allowlist, isolated paths, stderr bound, redaction, and shell
  metacharacter inertness are asserted.
- Existing raw Hermes fixture and Native runtime contracts remain passing.

## Verification commands

Focused during implementation:

```bash
python3 -m py_compile src-tauri/tests/fixtures/hermes_acp_server_stub.py
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo test --manifest-path src-tauri/Cargo.toml --test hermes_acp_transport_spike --locked
cargo test --manifest-path src-tauri/Cargo.toml --test hermes_transport_spike --locked
cargo test --manifest-path src-tauri/Cargo.toml --test agent_runtime_contract --locked
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
```

Completion gate after the final relevant edit:

```bash
npm run verify
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
python3 .codex/hooks/session_end_gate.py
```

The real Hermes command is Not run unless every preceding milestone passes.
No ignored or skipped real-runtime case counts as a pass.

## Risks

- The supplied candidate may lack the optional ACP dependency or complete
  runtime provenance.
- A structured protocol can still hide internal privileged execution.
- Hermes may load normal configuration, credentials, state, memory, skills,
  plugins, or MCP before the host can establish a session.
- Cancellation may not stop children or side effects already started by tools.
- Direct-child fixture cleanup cannot prove detached-descendant cleanup.
- ACP's unstable protocol option and optional SDK version create compatibility
  and packaging risk.

## Rollback or failure strategy

If the spike is NO GO or BLOCKED, preserve its ADR/plan/report as historical
evidence and remove no earlier evidence. Rollback of experimental code deletes
only the new test target and fixture. No product behavior, production source,
manifest, lockfile, dependency, Hermes installation, profile, credential, or
application data requires rollback.

## Decisions made

- ACP is evaluated independently from both rejected TUI-gateway transports.
- Native remains sole/default and Prompt 4D is excluded.
- The ACP ADR began Proposed and is Rejected after the NO GO result.
- D-081 records the owner-directed evaluation outcome without selecting another
  transport.

## Discoveries

- The candidate remains transport-independently ineligible: its 4,077-file
  virtual environment and external Python runtime lack a complete immutable
  content manifest.
- The installed 61-package environment does not contain the pinned optional
  `agent-client-protocol==0.9.0` package or importable `acp` module. The
  `hermes-acp` launcher alone is not availability evidence.
- ACP is materially stronger than raw TUI stdio as a protocol: it has a public
  launcher, newline-delimited JSON-RPC, protocol/version initialization,
  structured sessions and updates, cancellation, and stdout/stderr separation.
- `SessionManager._make_agent` hardcodes `enabled_toolsets=["hermes-acp"]`.
  The toolset contains terminal/process, file mutation, browser, memory,
  skills, code execution, and delegation, and no supported true zero-tool ACP
  mode exists.
- Tool notifications and selected terminal/edit permission callbacks observe or
  mediate only part of Hermes-owned execution; they do not place every effect
  behind Cortexa's deterministic pre-execution governance.
- The adapter uses `use_unstable_protocol=True`, has no ACP health method, and
  has no explicit containment-wide shutdown/descendant-reap contract.

## Progress

- 2026-08-11: Owner authorized the architecture revision and isolated ACP
  spike. Clean synchronized baseline and valid prior gate were confirmed; gate
  `hermes-acp-transport-spike` began before edits.
- 2026-08-11: Read-only candidate/source inspection confirmed exact
  version/tag/commit and critical hashes but found incomplete runtime
  provenance and an absent ACP SDK. No candidate command or import ran.
- 2026-08-11: Added a test-only Rust host harness and deterministic Python ACP
  fixture. Five focused tests pass; existing raw-stdio fixture tests pass 7/7
  with one ignored opt-in probe, and Native runtime contracts pass 20/20.
- 2026-08-11: Static pinned-source review triggered the mandatory NO GO: every
  ACP session hardcodes privileged internal tools without a supported
  conversation-only mode or Cortexa-owned gate. The real-runtime milestone was
  not run.
- 2026-08-11: Marked the WebSocket and ACP ADRs Rejected for their exact
  evaluated conditions, added D-081, and kept the adapter Draft/Blocked with no
  selected transport.

## Acceptance criteria

- [x] WebSocket NO-GO is preserved and its ADR reflects the completed result.
- [x] Exact pinned ACP contract and source symbols are recorded.
- [x] Fixture tests cover the bounded process/protocol/isolation contract.
- [x] Real candidate is not run after any failed prerequisite.
- [x] ACP verdict and ADR status follow evidence without rationalization.
- [x] Adapter remains Draft/Blocked, Native unchanged/default, Prompt 4D
      unstarted, and complete applicable validation passes.

## Final results

**NO GO.** ACP has a supported structured stdio wire and materially better
protocol semantics than raw TUI-gateway stdio, but the pinned implementation
hardcodes broad privileged tool execution inside Hermes. The host cannot
intercept and authorize every effect through Cortexa's existing deterministic
Rust governance before execution, and no supported conversation-only ACP mode
exists. The supplied candidate additionally lacks complete immutable runtime
provenance and its pinned ACP SDK. Five deterministic fixture tests pass; they
prove host mechanics only. No real Hermes command, provider, credential,
session, tool, or process ran. The ACP ADR is Rejected, Native remains
sole/default, the adapter remains Draft/Blocked, and Prompt 4D was not started.

## Documentation updates

- [x] `HANDOFF.md`
- [x] `PROJECT_STATUS.md`
- [x] `NEXT_STEPS.md`
- [x] `DECISIONS.md`
- [x] `CHANGELOG.md`
- [x] `TROUBLESHOOTING_LOG.md` not changed; no new repository failure required
      a durable troubleshooting entry
