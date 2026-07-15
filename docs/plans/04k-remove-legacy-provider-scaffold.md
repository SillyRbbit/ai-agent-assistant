# Execution plan - Increment 4K remove legacy provider scaffold

Status: **Complete**

Owner: Project maintainer

Last updated: 2026-07-15

## Execution status

The project owner approved the exact source and closeout scopes. Mandatory `04k`
state began before source edits. Implementation deletes only the two legacy
provider modules and their exports, preserves every verified downstream boundary,
and passes focused checks, complete repository verification, dependency audit,
review, documentation synchronization, and the mandatory gate without scope
expansion.

## Goal and outcome

Delete the unused synchronous provider abstraction and its arbitrary-string
request, response, and mock-error values before future production transport work
can mistake them for the approved gateway boundary.

The outcome is an `agent` namespace exposing only the verified normalized gateway
protocol and exact local function-call validation modules. No replacement
provider, transport, or product capability is introduced.

## User-visible outcome

None. The legacy provider has no production caller, Tauri registration, IPC path,
or frontend connection. The shipping application continues to use the verified
frontend mock interaction shell.

## Existing behavior and constraints

- `AgentProvider::complete` synchronously consumes a public `AgentRequest` with
  arbitrary `run_id`, `user_message`, and `available_tools` strings.
- `AgentProviderResponse` contains an arbitrary assistant text string, and
  `AgentProviderError::MockFailure` retains an arbitrary caller-supplied string.
- `MockAgentProvider` is deterministic but validates only non-empty run and user
  message strings. It does not enforce the approved gateway request contract,
  streaming, event sequence, limits, cancellation, correlation, or closed
  redacted errors.
- Repository search finds no caller outside `agent/provider.rs` and
  `agent/types.rs`; their only executable coverage is three embedded unit tests.
- `agent::gateway_protocol` is independent and has 18 passing tests for the
  versioned normalized protocol, strict event validation, conservative limits,
  cancellation, function-call containment, and error redaction.
- Exact local schema validation consumes only `UntrustedFunctionCall` from the
  verified gateway protocol. Policy, approval, and typed approval audit do not
  depend on the legacy provider.
- D-021 requires a future authenticated gateway transport with a closed bounded
  request contract and normalized stream. O-006 and O-007 still block live
  networking and provider traffic choices.

## Why deletion is the smallest increment

Adapting the legacy trait would require defining gateway request construction,
authentication, secret storage, HTTPS transport, streaming callbacks,
cancellation ownership, deadlines, retries, and runtime orchestration. Those are
separate security-sensitive capabilities and are not yet approved.

Deletion removes an unused bypass-shaped abstraction in three source paths while
preserving every verified Phase 4 boundary. A future provider interface can then
be designed from the normalized protocol and exact credential boundary instead
of inheriting an incompatible synchronous string API.

## Exact source scope

Delete:

```text
src-tauri/src/agent/provider.rs
src-tauri/src/agent/types.rs
```

Change:

```text
src-tauri/src/agent/mod.rs
```

The `mod.rs` change removes only:

```rust
pub mod provider;
pub mod types;
```

It must preserve:

```rust
pub mod function_call_validation;
pub mod gateway_protocol;
```

## Exact implementation closeout scope

In addition to the three source paths above, implementation closeout may change
only:

```text
AGENTS.md
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
docs/increments/04k-remove-legacy-provider-scaffold.md
docs/plans/04k-remove-legacy-provider-scaffold.md
docs/plans/README.md
docs/reviews/2026-07-15-04k-post-increment-review.md
```

`DECISIONS.md` is limited to proposed D-032 recording that future provider
transports require closed bounded request and normalized event contracts rather
than restoration of this synchronous arbitrary-string API. Stop and request
approval before changing any other file.

## Implementation steps

1. After project-owner approval, run:

   ```bash
   python3 .codex/hooks/post_increment_gate.py begin --increment 04k
   ```

2. Delete `agent/provider.rs` and `agent/types.rs`.
3. Remove only their module exports from `agent/mod.rs`.
4. Confirm every legacy provider symbol is absent from current Rust source and
   tests while historical records remain unchanged.
5. Run focused gateway, function-validation, and public-boundary tests, Clippy,
   and the complete repository verification suite.
6. Review the complete diff against `SECURITY.md` and `CODE_REVIEW.md`, synchronize
   closeout documentation, append D-032, and complete the mandatory gate.

## Test plan

Focused verification must prove:

- the normalized gateway protocol tests still pass unchanged;
- exact local function-call validation still consumes only gateway-produced
  untrusted calls;
- the public gateway -> schema -> policy boundary still passes unchanged;
- no legacy provider type, trait, mock implementation, module export, or current
  caller remains;
- no source path outside the exact three-file plan changes; and
- complete repository verification passes after the three deleted embedded tests
  are removed from the expected Rust library count.

No new test file is justified: this increment removes a disconnected module, and
the preserved gateway and public validation boundaries already have focused
coverage.

## Verification commands

Focused during implementation:

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_protocol::
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::function_call_validation::
cargo test --manifest-path src-tauri/Cargo.toml --test policy_input_binding --locked
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
rg -n "AgentProvider|MockAgentProvider|AgentProviderResult|AgentProviderError|AgentRequest|AgentProviderResponse" src-tauri/src src-tauri/tests
```

The final `rg` command must return no matches. Exit status `1` means the required
absence check passed; any matching line fails scope completion.

Complete gate:

```bash
npm run verify
npm audit --audit-level=low
git diff --check
python3 .codex/hooks/post_increment_gate.py status
```

Then run `$post-increment-gate`, create and finalize the exact 4K report, and
confirm status is complete and valid.

## Manual verification

None required. The removed scaffold has no production caller, Tauri registration,
IPC path, user-visible state, persistence, network transport, or operating-system
interaction.

## Risks and mitigations

- **Public API removal:** an unsupported external crate consumer could import the
  public scaffold even though repository search finds no internal caller.
  Mitigation: the application crate is not treated as a published compatibility
  surface; rollback restores exactly two files and two exports.
- **Gateway regression:** an over-broad module edit could remove or alter the
  verified normalized protocol or function-call validator. Mitigation: `mod.rs`
  changes by two lines only, both preserved files remain byte-for-byte unchanged,
  and focused unit/integration tests run.
- **Premature transport design:** removal might be widened into a replacement
  provider trait. Mitigation: replacement transport, request construction,
  networking, credentials, and orchestration are explicit non-goals.
- **Future compatibility pressure:** later work might restore arbitrary request,
  response, or error strings for convenience. Mitigation: proposed D-032 requires
  separately approved closed bounded contracts derived from D-021.
- **Evidence-count drift:** removal intentionally deletes three provider unit
  tests. Mitigation: closeout records actual test counts and does not describe the
  lower count as a regression.

## Explicit non-goals

- A replacement provider trait, transport client, gateway request type, stream
  driver, callback, async runtime, or coordinator.
- Live HTTPS networking, gateway deployment, OpenAI Responses calls, model
  selection, identity-provider selection, or retention-mode selection.
- Production API keys, gateway access tokens, Keychain, OAuth, authentication,
  authorization headers, or secret storage.
- Changes to `gateway_protocol`, `function_call_validation`, tool schemas,
  registry, policy, approval, native decision source, typed audit, or storage.
- Dispatch, executor, local-task creation, provider continuation, trusted tool
  results, durable audit, migrations, or persistence.
- Tauri commands/events, WebView changes, IPC, capabilities, CSP, packaging,
  entitlements, or operating-system permissions.
- Dependencies, manifests, lockfiles, database files, or generated artifacts.
- Rewriting historical decisions, completed plans, or backup records that refer
  to the Phase 2 provider scaffold at their original checkpoints.

## Security and privacy considerations

The change removes interfaces that retain arbitrary user, assistant, and mock
error strings without adding any replacement path. It therefore creates no new
content flow or authority. The implementation must not modify the verified
gateway error-redaction, cancellation, strict event validation, tool validation,
policy, approval, or audit boundaries.

## Rollback or failure strategy

Restore `src-tauri/src/agent/provider.rs` and `src-tauri/src/agent/types.rs`
exactly from baseline `99f9279`, and restore their two exports in
`src-tauri/src/agent/mod.rs`. Revert only the 4K planning/closeout documentation
and D-032. No migration, persisted data, dependency, capability, credential, or
external state requires rollback.

If any current repository caller is discovered before deletion, stop and revise
the plan rather than widening it to migrate that caller automatically.

## Acceptance criteria

- The exact two legacy provider files are deleted and only their exports are
  removed from `agent/mod.rs`.
- `gateway_protocol`, `function_call_validation`, and all other source files
  remain unchanged.
- Current Rust source and tests contain no legacy provider scaffold symbol or
  module export.
- Existing gateway, function-validation, and gateway-to-policy checks pass.
- The implementation adds no replacement provider abstraction or runtime caller.
- No runtime behavior, persistence, network, credential, dependency, migration,
  IPC, UI, capability, or permission changes.
- The exact three-file source scope and declared closeout scope are preserved.
- Focused checks, full repository verification, dependency audit, complete diff,
  code review, security review, documentation synchronization, and mandatory
  post-increment gate pass.
- D-032 is accepted and Increment 4K is marked complete only after project-owner
  approval and all gates pass.

## Planning baseline evidence

Passed on clean synchronized `main` at `99f9279`:

```text
python3 .codex/hooks/post_increment_gate.py status
  Increment 04i complete, valid: true, PASS WITH ADVISORIES
npm run typecheck
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::provider::
  3 passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_protocol::
  18 passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::function_call_validation::
  6 passed
cargo test --manifest-path src-tauri/Cargo.toml --test policy_input_binding --locked
  2 passed
npm run format:check
  passed after planning edits
git diff --check
  passed after planning edits
rg -n "AgentProvider|MockAgentProvider|AgentProviderResponse|AgentRequest" src-tauri/src src-tauri/tests -g '!**/agent/provider.rs' -g '!**/agent/types.rs'
  no matches outside the proposed deleted files; required exit status 1
```

At the planning checkpoint, changes are documentation-only and source
implementation remains blocked on project-owner approval. The previously valid
`04i` marker is now stale because the planning documents changed; no `04k` state
is active and no implementation-gate result is claimed.

The first caller-absence scan used non-path-aware exclusion globs and printed the
definitions in the proposed deleted files. The corrected path-aware command above
returned no matches; this was command syntax rather than a repository defect.

## Completion result

The exact three-file source scope is complete. Current Rust source and tests
contain no legacy provider symbol or export. Eighteen normalized gateway tests,
six exact function-validation tests, two public gateway-to-policy tests, rustfmt,
Clippy with warnings denied, and complete `npm run verify` pass. The full suite
contains 17 hook, 124 frontend, 92 Rust library, and 11 Rust integration tests,
plus lint, typecheck, Vite builds, and the Tauri release no-bundle build. The
network-enabled npm audit retry reports zero vulnerabilities.

The complete 14-path scope, conflict, secret, generated-output, architecture,
code-health, security, and diff reviews have no blocking finding. No manual gate
applies because no production caller, Tauri registration, IPC, UI, persistence,
network, or operating-system behavior changed. D-032 records the future closed
provider-transport boundary. The consolidated result is `PASS WITH ADVISORIES`;
the advisory is the theoretical unsupported external consumer of the removed
public scaffold.

The first sandboxed gate begin could not write ignored state, and the first
sandboxed npm audit could not access the registry or user-level logs. Their exact
approved retries succeeded. No repository file changed as a result of either
environment restriction.

## Exact next task

Wait for explicit project-owner direction to commit, push, and merge Increment
4K. Do not start later work.
