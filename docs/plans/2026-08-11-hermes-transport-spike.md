# Hermes transport spike

Status: Complete with advisories
Owner: Project owner
Last updated: 2026-08-11

## Goal

Prove or disprove whether the exact Hermes Agent release selected by the
architecture assessment exposes a sufficiently supported, machine-readable
managed-subprocess transport for a possible future Cortexa adapter, without
installing Hermes, connecting it to the application, or changing native agent
behavior.

## User-visible outcome

None. This is an isolated, test-only technical experiment. It adds no runtime
selection, model traffic, application UI, Tauri command, tool, permission,
credential, or production process behavior.

## Scope

- Reconcile the current assessment, Proposed ADR, repository runtime state, and
  official tagged Hermes release documentation and source.
- Pin the evaluated upstream release, tag, and commit.
- Exercise bounded newline-delimited JSON-RPC process mechanics against a
  deterministic local fixture: discovery/version rejection, readiness, one
  session, text request/response, timeout, cancellation, clean direct-child
  shutdown, malformed output, unexpected exit, stderr separation, environment
  isolation, and diagnostic redaction.
- Add an ignored, explicit opt-in version probe for an operator-supplied Hermes
  executable; normal tests must neither discover nor execute Hermes.
- Record an evidence-based `GO`, `CONDITIONAL GO`, or `NO-GO` conclusion and
  the smallest production-adapter recommendation.
- Synchronize repository memory and completion evidence from observed results.

## Explicit non-goals

- No `AgentRuntime`, `NativeAgentRuntime`, `HermesAgentRuntime`, provider,
  coordinator, production supervisor, or adapter implementation.
- No Hermes installation, update, import, vendoring, dependency, package,
  feature, manifest, lockfile, configuration, credential, or live execution.
- No model/provider traffic or external transmission, including synthetic
  traffic.
- No host tool, shell, filesystem, memory, skill, plugin, hook, MCP, subagent,
  schedule, messaging, cloud, browser, clipboard, approval, secret, or device
  capability.
- No WebView, Tauri IPC, capability, permission, React, or native application
  change.
- No claim that a cooperative fake proves Hermes conformance, OS containment,
  descendant cleanup, network denial, filesystem denial, Keychain isolation,
  or packaging readiness.
- No acceptance of the Proposed ADR. If its preferred transport is disproved,
  this increment records the blocker and recommends a separate ADR revision.
- No commit, push, publication, deployment, or later increment.

## Existing behavior and constraints

- The repository has no implemented `AgentRuntime`, `NativeAgentRuntime`, or
  `HermesAgentRuntime`. `InitialGatewayTurn` remains a transport-free,
  test-consumed native Rust boundary.
- The shipping Tauri command surface exposes no agent/runtime/provider/process
  IPC. The React assistant remains a deterministic no-I/O mock.
- The ADR is Proposed and non-authoritative. D-078 permits a future optional
  adapter direction only; D-032 prohibits reviving the removed synchronous
  arbitrary-string provider contract.
- The assessment recommends a pinned, whole-process-contained managed
  subprocess and asks a spike to verify a supported noninteractive TUI-gateway
  stdio launcher and a versioned schema.
- Normal tests must remain deterministic and network-free. Hermes is not
  installed on the current machine and must not be installed automatically.
- Upstream's security policy treats whole-process OS isolation as the only
  load-bearing boundary against adversarial model behavior.

## Current-state evidence

- Baseline: clean synchronized `main` at
  `f5b3fbe1b50f3269d6843002f0d1c81fe3e9b770`.
- Prior `hermes-integration-assessment` marker: complete, valid, `PASS WITH
ADVISORIES`.
- Active gate: `hermes-transport-spike`.
- Baseline native contract:
  `cargo test --manifest-path src-tauri/Cargo.toml --test gateway_request_contract --locked`
  passed 10 tests.
- Repository search found no runtime implementation; `ARCHITECTURE.md`
  explicitly labels the runtime names planned only.
- Local discovery found no `hermes` executable on `PATH`; Python is 3.12.1.
- Official latest release inspected on 2026-08-11: Hermes Agent `0.20.0`, tag
  `v2026.8.3`, release commit
  `3c27eb6234bf91b8ceee9e9071591b31e9b148cb`.
- The tagged programmatic-integration guide names TUI-gateway JSON-RPC over
  stdio but publishes no raw-gateway `hermes` command. Tagged source uses the
  internal module entry `python -m tui_gateway.entry`; `hermes --tui` is the
  Node/Ink UI wrapper rather than a raw JSON-RPC server.
- The first tagged `gateway.ready` frame has no Hermes or protocol version.
  `desktop_contract` and later version metadata are application/TUI fields, not
  an initial side-effect-free protocol negotiation.

## Files expected to change

Experimental and primary output:

- `src-tauri/tests/hermes_transport_spike.rs`
- `src-tauri/tests/fixtures/hermes_tui_gateway_stub.py`
- `docs/spikes/HERMES_TRANSPORT_SPIKE.md`

Plan, current evidence, and closeout:

- `docs/plans/2026-08-11-hermes-transport-spike.md`
- `docs/architecture/HERMES_INTEGRATION_ASSESSMENT.md`
- `docs/adr/ADR-MULTI-RUNTIME-AGENT-ARCHITECTURE.md`
- `docs/increments/hermes-transport-spike.md`
- `docs/reviews/2026-08-11-hermes-transport-spike-post-increment-review.md`
- `PLANS.md`
- `HANDOFF.md`
- `PROJECT_STATUS.md`
- `NEXT_STEPS.md`
- `CHANGELOG.md`

No production source, frontend, Tauri configuration/capability, manifest,
lockfile, workflow, hook, skill, or dependency file may change.

## Affected components

- One Rust integration-test binary under the established test-only boundary.
- One deterministic Python child fixture invoked only by that test binary.
- Architecture/spike documentation and repository completion evidence.

Production application components are inspected but unaffected.

## Interfaces and invariants

- Normal validation spawns only the repository fixture by an absolute path; it
  never searches for or starts Hermes.
- The optional real-version probe is `#[ignore]`, requires explicit opt-in and
  an absolute operator-supplied executable path, performs version detection
  only, and skips cleanly when unavailable.
- Child processes are built with `std::process::Command` and fixed argv values;
  no shell, command-string concatenation, or prompt text in argv is allowed.
- The ordinary fixture command starts from `env_clear`, uses an isolated
  temporary home/cwd, and passes only explicit non-secret variables. Tests must
  reject credential, proxy, provider, token, and `PATH` keys while recognizing
  that the macOS system Python launcher may add non-secret SDK/locale key names.
- Stdout is bounded newline-delimited JSON only. Stderr is separately drained,
  capped, counted, and never copied verbatim into public failures.
- Frames, event count, identifiers, allowed methods/events, and deadlines are
  closed and bounded. Malformed, oversized, unknown, forbidden, late, or
  mismatched data fails closed.
- Cancellation is terminal and idempotent in the test host. Timeout, parse
  failure, unexpected exit, and drop paths terminate and reap the direct child.
- No restart, automatic native fallback, provider fallback, or continuation is
  attempted.
- The fixture may demonstrate transport mechanics only. It cannot satisfy the
  missing supported launcher, version negotiation, whole-process containment,
  immutable artifact, or process-tree guarantees.
- Native source, deterministic mocks, current tests, UI behavior, and all
  trusted Rust governance ownership remain unchanged.

## Implementation milestones

- [x] Reconcile repository state, authoritative guidance, current runtime
      absence, Proposed ADR, and upstream release provenance.
- [x] Complete readiness review and begin the bounded gate.
- [x] Add the deterministic fixture and test-only process/protocol harness.
- [x] Add focused success, timeout, cancellation, malformed, exit, separation,
      redaction, environment, and ignored real-version cases.
- [x] Record the pinned upstream evidence and transport verdict.
- [x] Run focused and complete applicable validation.
- [x] Complete architecture, security, code, debt, readiness, documentation,
      session-end, quality, and post-increment review.

## Security and privacy considerations

The child boundary is untrusted. The test host must not interpolate input into a
shell, inherit the developer environment, expose values through arguments, or
relay stdout/stderr verbatim into errors. Synthetic sentinels must demonstrate
redaction. No real provider secret, environment value, user content, or Hermes
state may be read or recorded.

`env_clear`, a temporary home, a temporary working directory, and direct-child
kill/reap are test hardening, not containment. They do not prevent absolute
host filesystem access, networking, process inspection, Keychain access, or
descendant escape. No actual Hermes session may start until a separate
owner-approved whole-process containment design and external-processing gate
exist.

## Test plan

- Complete one ready -> session -> prompt -> text delta -> completion ->
  shutdown sequence and verify stdout/stderr separation and direct-child reap.
- Reject a missing or wrong version and show the actual Hermes probe remains
  ignored/unavailable by default.
- Bound readiness and in-run timeouts; force termination and reap.
- Make cancellation terminal/idempotent and reject late output.
- Reject malformed JSON, unknown or forbidden methods/events, wrong identities,
  and oversized frames.
- Map early and midstream child exit to closed failures without exposing raw
  exit/stderr content.
- Cap and redact synthetic prompt and stderr sentinels; expose environment key
  names only and never their values.
- Verify the fixture observes every explicit environment key, no credential,
  proxy, provider, token, or `PATH` key, and the isolated canonical home/cwd.
- Rerun the native contract and full repository verification as regression
  evidence.
- Do not run the ignored real-Hermes test because Hermes is unavailable and no
  contained execution was approved.

## Verification commands

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo test --manifest-path src-tauri/Cargo.toml --test hermes_transport_spike --locked
cargo test --manifest-path src-tauri/Cargo.toml --test gateway_request_contract --locked
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
npm run verify
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
python3 .codex/hooks/session_end_gate.py
python3 .codex/hooks/post_increment_gate.py status
```

The quality-gate and post-increment-gate workflows run only after the final
relevant edit. The ignored real-Hermes probe is `Not run` by design.

## Risks

- A fake child could be misreported as proof that Hermes itself works.
- Internal tagged source may be mistaken for a stable supported public API.
- TUI-gateway methods expose approval, secret, CLI, tool, configuration, MCP,
  skill, plugin, and subagent authority far beyond the allowed projection.
- The gateway-ready event lacks protocol/version negotiation, so silent drift
  could be accepted accidentally.
- Direct-child cleanup does not guarantee descendant cleanup or OS containment.
- Test-only process code can deadlock or leak a child if stdout/stderr are not
  drained and every terminal path is not reaped.
- Upstream release, PyPI distribution, mutable `main`, and internal module
  layout are different upgrade channels.

## Rollback or failure strategy

Reverse only this plan's bounded diff: remove the new integration test, fixture,
spike report, increment/review records, and plan; restore the additive
assessment/ADR notices and the current-memory paragraphs. No production file,
manifest, lockfile, dependency, runtime state, Hermes installation, or user data
requires cleanup. If validation reveals an orphan, terminate only the exact
recorded fixture child and stop; never use a broad process kill.

## Decisions made

- Use `src-tauri/tests/` because it is the established contract-test boundary,
  is not linked into the application, needs no manifest change, and is covered
  by existing Rust validation.
- Use a fixture child rather than Hermes for ordinary tests. A cooperative fake
  tests host mechanics, not upstream behavior or containment.
- Pin evaluation to the official GitHub release rather than mutable `main` or
  the older PyPI distribution.
- Keep the Proposed ADR Proposed. A negative result recommends a separate
  owner-reviewed revision rather than silently choosing another transport.

## Discoveries

- The public tagged guide documents the TUI-gateway wire but not a public raw
  stdio launcher.
- The internal tagged launch is `python -m tui_gateway.entry`; it is an
  implementation module, not a supported `hermes` subcommand.
- `gateway.ready` does not negotiate a protocol or runtime version.
- `session.interrupt` is the run-cancellation method, while `process.stop`
  concerns agent-launched background processes rather than gateway shutdown.
  Gateway termination relies on stdin EOF or process signal.
- The fixed macOS system Python launcher adds non-secret SDK and locale key
  names after `env_clear`; the test closes those names to an enumerated set and
  verifies isolated path equality without exposing values.
- Direct-child process mechanics can be bounded and tested, but neither they nor
  a cooperative fake close the missing supported launcher, negotiation,
  containment, descendant, packaging, or target-platform evidence.

## Progress

- 2026-08-11: Readiness classified `Ready with advisories`; clean baseline,
  current runtime absence, pinned upstream provenance, and native regression
  evidence were confirmed. Gate `hermes-transport-spike` began.
- 2026-08-11: Added the fixture-only managed-child harness. Focused review found
  and resolved terminal late-output enforcement, Linux interpreter-symlink, and
  closed environment-key evidence gaps.
- 2026-08-11: Seven ordinary spike tests pass and one real-Hermes version probe
  remains ignored. Strict Clippy, the ten-test native gateway regression, and
  full repository/Tauri verification pass. Hermes was not installed or run.
- 2026-08-11: Recorded **NO-GO** for raw TUI-gateway stdio as the selected
  supported production contract and synchronized the Proposed ADR, assessment,
  repository memory, increment, and consolidated review.

## Acceptance criteria

- [x] The exact release/tag/commit and inspected official sources are recorded.
- [x] Ordinary tests demonstrate every bounded fake-process scenario without
      network, Hermes, secrets, or application wiring.
- [x] The actual-Hermes probe is ignored, explicit, version-only, and skips
      cleanly when unavailable.
- [x] The report distinguishes protocol mechanics, official upstream evidence,
      unsupported assumptions, and unproved containment.
- [x] The report reaches `GO`, `CONDITIONAL GO`, or `NO-GO` without forcing a
      preferred result.
- [x] Native production source and behavior remain unchanged.
- [x] Required verification and completion workflows pass.

## Final results

The selected production raw TUI-gateway stdio mechanism is **NO-GO** for Hermes
Agent `0.20.0` / `v2026.8.3`. The pinned release has no public raw-gateway
launcher, initial version/capability negotiation, or gateway-shutdown RPC. The
test-only fixture shows that a closed host can bound framing, lifecycle,
timeouts, cancellation, malformed/forbidden output, stderr, redaction,
environment names, and direct-child cleanup, but it does not prove Hermes,
containment, descendants, packaging, or portability.

No production source, dependency, manifest, lockfile, feature, UI, IPC,
permission, runtime path, native behavior, or external state changed. The ADR
remains Proposed. The quality result is `PASS WITH ADVISORIES`; subsequent
runtime work is `Blocked` pending a separately selected ADR revision and fresh
readiness review.

## Documentation updates

- [x] Required spike report.
- [x] Additive assessment and Proposed-ADR status notices, if the preferred
      transport is disproved.
- [x] `HANDOFF.md`
- [x] `PROJECT_STATUS.md`
- [x] `NEXT_STEPS.md`
- [x] `CHANGELOG.md`
- [x] Increment and post-increment review records.
- [x] `DECISIONS.md` not required unless the owner separately accepts a revised
      architecture decision.
- [x] `TROUBLESHOOTING_LOG.md` not required unless an unexpected repository or
      tool failure occurs.
