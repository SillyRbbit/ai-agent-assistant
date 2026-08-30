# Cortexa testing guide

Status: Authoritative testing standard
Last updated: 2026-08-28

## Testing principles

- Test the smallest meaningful boundary first, then run the complete relevant
  suite before completion.
- Keep normal tests deterministic, local, isolated, and independent of network,
  external accounts, production credentials, wall-clock timing, and native UI.
- Test observable contracts and security invariants, not private implementation
  shape.
- Cover success, rejection, malformed input, limits, replay, cancellation,
  expiry, and late outcomes where those states exist.
- Never report a check as passed unless its command completed successfully in
  the current increment.

## Test layers

### TypeScript unit tests

Location: `src/**/*.test.ts` and `src/**/*.test.tsx`.

Use Vitest for pure functions, reducers, state transitions, event narrowing,
mock drivers, and bounded data models. Prefer table-driven cases for closed
state machines and validation rules.

Command:

```bash
npm run test:unit
```

This command also runs Rust library tests. For a frontend-only focused run use:

```bash
npx vitest run path/to/file.test.tsx
```

### React UI tests

Use Testing Library through Vitest and JSDOM. Query by accessible role, name,
label, or visible state. Exercise user-visible navigation, loading, errors,
empty states, Stop, Retry, approvals, and restoration. Do not assert CSS class
names unless the class itself is the behavioral contract.

Visual layout, native menus, window lifecycle, and operating-system icon
contexts require explicit manual or screenshot evidence; JSDOM cannot verify
them.

### Rust unit tests

Location: `#[cfg(test)]` modules beside Rust implementation.

Use unit tests for private validation, typed errors, state transitions, policy
tables, storage behavior, and target-neutral adapters.

Focused command:

```bash
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked module::tests::
```

### Rust integration and contract tests

Location: `src-tauri/tests/*.rs`.

Integration tests exercise the public crate boundary and contracts between
modules. Existing suites cover smoke startup, storage, menu routing, gateway
requests, policy binding, approval binding, and approval-audit binding.

Command:

```bash
npm run test:integration
```

Contract tests must prove both accepted and rejected values across trust
boundaries. A test-only helper may synthesize a sealed native value only when
production construction remains inaccessible and unchanged.

### Native multi-agent acceptance suite

The deterministic nine-agent architecture has one named local acceptance
command:

```bash
npm run test:agent-acceptance
```

It composes the complete Rust library unit suite with the exact ten public
definition, runtime, orchestration, governance, memory/document, sequential
workflow, automation, infrastructure/operations, and bounded-parallel contract
binaries. The executable matrix and current counts are in
[`NATIVE_MULTI_AGENT_ACCEPTANCE.md`](docs/demos/NATIVE_MULTI_AGENT_ACCEPTANCE.md).

This command proves application-owned contracts with deterministic fixtures.
Workflow response contracts use the test-only `MockAgentRuntime`; governance
and private units also exercise the `NativeAgentRuntime` wrapper, which product
source uses only through the sealed no-input demo host. The
mock descriptor uses the sole closed `Native` runtime identity, but the command
is not proof of a configured provider, live model, external operation, real
tool execution, native dialog display, or connected UI. The complete 269/476
evidence is macOS-specific because the approval-source unit is target-gated; a
non-macOS run omits that unit. Run this as focused evidence; it does not replace
`npm run verify` when the increment requires complete verification.

### Connected Research/Knowledge demo lifecycle

The selected Command Center scenario separately mounts a fixed simulated
lifecycle client. Its focused tests must cover exact DTO and journal grammar,
requested-operation responses, response-only presentation authority,
malformed/stale/duplicate/gapped/contradictory/late events, unsolicited valid
terminal events, closed errors, explicit start/advance/cancel actions, private
success/failure scheduling, cancellation without schedule consumption,
mount/disposal, busy controls, accessibility, and the exact F-12 sole-consumer
boundary. This panel is not evidence that the frontend fixture projection,
Conversations mock, and Rust acceptance workflows are integrated.

Target-Mac evidence must distinguish the source-current raw debug executable
from any stale bundled release app. When approved tooling cannot bind to the
raw debug process, record native interaction, alternate appearance/reduced
motion, page zoom, and resize as `Not run`; a browser-rendered fallback can
verify layout and disclosure but cannot substitute for native lifecycle proof.

### Personal Assistant v0 evidence separation

D-094's Personal Assistant program preserves separate evidence for each trust
boundary. V0-1 transport-free Rust fixtures prove the exact empty-tool profile,
success/failure/cancellation, bounds, transactional rejection, late-event
behavior, exact returned runtime identity and initial status, and terminal
cleanup versus run-and-lease quarantine for every rejected-start outcome. The
public V0-1 host has no response-frame ingress, so its observable production
path proves only fixed request construction, checked Native start ownership,
closed status, and local cancellation. Fixture success/failure/stream results
do not prove a user-visible session. Neither evidence class proves signing,
Keychain, TLS, Access, Worker, OpenAI, Tauri, WebView, or target-Mac behavior.

V0-2 preserves that separation while adding a production-private bounded
record reducer and public Rust-only start/snapshot/update/cancel surface.
Focused evidence distinguishes the 12 module tests that use the private finite
fixture driver from the 3 integration tests that observe only the compiled
public host. Fixture events cross `RuntimeRun::accept_event`, exact
identity/status checks, and the production-private reducer, but remain
application-owned test data. Production still has no response-frame ingress,
provider stream, Tauri command/event, or user-visible conversation. Deadline
tests use a private monotonic manual clock; production uses `Instant` sampling
without threads or timers, while active network-enforced deadlines remain V0-7
scope.

Generated JWT/JWKS tests prove the local verifier only. A route-free control-
plane inspection proves no-traffic configuration only. Fixed zero-content
authentication probes prove Access/JWT behavior only, and their evidence keeps
the Service Auth token-resource `id` distinct from JWT/binding `client_id`.
Fake HTTP/OpenAI streams prove only the exact closed event grammar, private
sequence origin/contiguity, identity consistency, intermediary handling,
terminal mapping, and refusal/reasoning/tool rejection of the bounded adapter.
None substitutes for the separately approved live synthetic Stage C rehearsal,
and that rehearsal never authorizes real personal content.

The authentication-only probe has its own exact terminal disclosure/version
and one-use Rust admission. Tests prove absent/wrong/non-terminal acknowledgment
performs zero network operations and cannot start model transport. Live evidence
also distinguishes new-admission denial through
`PA_V0_TRAFFIC_ENABLED=false` from owned abort of an already admitted request.
Log evidence classifies Access/Worker runtime metadata (content-free, at most
seven days) separately from the current 18-month Cloudflare admin-action audit
trail and stops on content, secret, field, or retention drift.

The synthetic-v1 Tauri contract must be tested before first transmission. Rust
tests pin the no-text command, exact disclosure admission, private identity,
single-flight host, journal/cursors, cancellation, and closed errors.
TypeScript treats every reply as `unknown` and tests exact keys/types, Unicode
scalar and UTF-8 bounds, sequences/transitions, prefix/final equality,
single-flight 250 ms polling, route/visibility generations, focus, chronology,
plain-text rendering, and explicit recovery. F-12 tests pin the sole Personal
Assistant consumer and absence of Personal Assistant events, alternate
bridge/network surfaces, storage, capability, CSP, or permission expansion;
existing approved event boundaries remain unchanged.

Target-Mac Stage C evidence records every required check as Passed, Failed, or
Not run: signed identity, disclosure, fixed prompt, TLS/JWT, bounded streaming,
closed failure, cancellation at practical phases, active deadlines, cleanup,
late rejection, route-away/back, keyboard/focus, resize, 200% zoom, theme,
reduced motion, scrolling, console/log redaction, new-admission flag, owned
active abort, revocation,
rollback, no permission prompt, and no device effect. Each explicit Start is
one model request. There is no automatic preflight, retry, fallback, or second
cancel request.

Real-content-v2 requires distinct command/parser/profile/disclosure and fresh
tests after D-061, non-demo authentication, and explicit provider/hosting
authority. Synthetic-v1 evidence cannot be relabelled as real-content evidence.

### Hook and repository-workflow tests

Location: `.codex/hooks/tests/test_*.py`.

Use Python `unittest` for shared Git-status classification, JSON validation,
path containment, fingerprint, deletion, conflict, loop-guard, report, and
session-end inventory behavior. Tests must use isolated temporary Git
repositories and must prove the hook scripts do not modify application source.

Command:

```bash
npm run test:hooks
```

Repository hooks are developer workflow guardrails, not product security or
authorization evidence.

The focused hook suite covers missing reports, failed verification, pending
mandatory manual checks, PASS, PASS WITH ADVISORIES, outside-repository path
handling, merge conflicts, deletion-stable fingerprints, stale workspaces, and
loop prevention.

### Repository health tests

Location: `scripts/tests/test_*.py`.

The standard-library repository checks cover Markdown and image targets, secret
pattern redaction, tracked generated output, licensing evidence, documented npm
commands, prompt metadata and placeholders, stale active prompt paths,
immutable GitHub Action references, the exact two-workflow layout,
least-privilege dual-runner workflow policy, and the exact accepted Cargo-audit
baseline. Positive and negative fixtures use isolated temporary paths and
synthetic values.

Commands:

```bash
npm run test:repository
npm run docs:check
npm run repository:check
npm run security:scan
```

These checks are read-only. A passing pattern scan or link audit is bounded
evidence, not a security certification or proof of remote GitHub settings.

### Risk-based GitHub workflow verification

The active CI and Documentation workflows use dedicated Linux and macOS
`cortexa-ci` runners only for reviewed branch pushes, schedule, and explicit
dispatch. They do not subscribe to `pull_request`. Workflow changes must prove
locally that YAML parses, selectors and trusted branches are exact, action
references remain immutable, permissions remain read-only, secrets and write
operations remain absent, concurrency cancellation is active, and repository
health accepts only the exact two-workflow layout.

`scripts/ci_change_scope.py` uses fixed Git comparisons and closed path classes.
Its fixtures cover documentation-only, frontend-only, Rust-only, IPC/Tauri,
security-sensitive Rust, dependency, CI-workflow, deletion, unknown-path,
scheduled-audit, and manual-dispatch behavior. Unknown non-documentation paths
run both application jobs instead of being silently skipped.

Every current production Rust path under `src-tauri/src/**` and native or
trust-boundary example under `src-tauri/examples/**` selects frontend, Rust,
and dependency/security audit jobs. This is the fail-closed default for future
files in either tree. `src-tauri/tests/**` remains the isolated Rust-only test
family. The production Rust exception allowlist is intentionally empty; any
future exception requires a separately reviewed exact `src-tauri/src/*.rs`
file path, focused regression coverage, and synchronized documentation.
Examples and tests are not exception-eligible; wildcard and directory
exceptions are rejected.

After publication, inspect actual GitHub runs before claiming either registered
runner passed. Linux Rust validation preserves portability; target-Mac Rust
validation compiles and tests macOS-gated code but never replaces native menus,
windows, dialogs, permissions, signing, notarization, or installer evidence.
The active trust policy and operating record are in
`docs/github/SELF_HOSTED_RUNNER.md`.

Path-filtered workflows are conditional checks. GitHub may leave a skipped
required workflow pending, so do not claim they are universal branch-protection
requirements. Current remote protection is also unverified when the hosting
plan prevents authenticated inspection. Reviewers must require every applicable
job from the change-to-test matrix and use manual dispatch plus the local final
gate for ambiguous or oversized changes.

### Security tests

Security-sensitive increments require focused tests for every affected
boundary, including:

- unknown and additional fields;
- malformed and duplicate JSON keys;
- unregistered tools and versions;
- caller-supplied risk, permission, identity, approval, or evidence;
- sequence, size, turn, retry, and terminal-state limits;
- policy denial and exact approval binding;
- replay, expiry, cancellation, and late events;
- error and debug redaction;
- SQLite transaction, migration, foreign-key, and checksum failure;
- path traversal, symlink escape, scheme, and executable-content rejection when
  file tools are eventually introduced.

Use `$security-review` and `SECURITY_CHECKLIST.md` for the review evidence.

### Native macOS tests

Native verification runs on the target Mac when a change affects menus, window
lifecycle, dialogs, icons, permissions, signing, notarization, installers, or
platform adapters. Typical commands include:

```bash
npm run tauri -- dev
npm run tauri -- build --no-bundle
```

Record the macOS version, architecture, command, visible outcome, and any
appearance or lifecycle matrix. Do not infer native success from Rust unit tests
or a frontend build.

### Manual tests

A plan must declare each manual check before implementation. Record it as one of
`Passed`, `Failed`, `Not run`, or `Manual verification pending`. Required pending
manual checks block completion.

Manual checks are appropriate for accessibility flows not modeled in JSDOM,
native prompts, target-platform lifecycle, rendered branding, installation,
upgrade, signing, notarization, and external integration sandboxes.

## Naming and location

- TypeScript: `subject.test.ts` or `Component.test.tsx` beside the subject.
- Rust unit: a `tests` module in the implementation file.
- Rust integration: descriptive snake-case file under `src-tauri/tests/`.
- Hook tests: `test_<subject>.py` under `.codex/hooks/tests/`.
- Test names describe behavior and expected result, not implementation steps.
- Synthetic fixtures use obviously fake bounded data and contain no secrets or
  personal content.

## Mocking rules

- Mock only the nearest untrusted, platform, time, or transport boundary.
- Keep domain validation and state transitions real.
- Inject typed services rather than patching global behavior.
- Deterministic mocks must be visibly labeled and must not perform network,
  filesystem, database, credential, or operating-system actions.
- Native dialogs are not opened in automated unit tests. Use closed test-only
  constructors with production visibility unchanged.
- Do not use a mock result as proof that a provider, gateway, permission,
  integration, or operating-system action works.

## No-network test policy

Normal unit, integration, hook, lint, typecheck, and build tests must pass with
network unavailable after dependencies are installed. Tests shall not call live
OpenAI, gateway, OAuth, telemetry, package, calendar, email, or other external
services.

Network-dependent checks such as package advisory retrieval are explicit
verification commands, not test fixtures. A future gateway integration suite
must use a local deterministic server by default; separately approved sandbox
tests must use non-production accounts and redacted evidence.

The scheduled CI dependency-audit job may retrieve npm and Rust advisory data. It
contains no repository secrets and fails on any Rust advisory outside D-025's
exact vulnerability baseline and D-046's exact warning baseline. Accepted
findings remain reported remediation debt.

## Required commands

During implementation, run the narrowest relevant command and batch related
edits before expensive checks. At completion, run the sequence required by the
applicable change class once after the final relevant edit. The complete
cross-cutting sequence is:

```bash
npm run format:check
npm run lint
npm run typecheck
npm run test
npm run build
npm run tauri -- build --no-bundle
npm run docs:check
npm run repository:check
```

The canonical combined command for cross-cutting, security-sensitive,
dependency, Tauri-configuration, and release changes is:

```bash
npm run verify
```

Useful exact lower-level commands are:

```bash
npx vitest run
python3 -m unittest discover -s .codex/hooks/tests -p 'test_*.py' -v
python3 -m unittest discover -s scripts/tests -p 'test_*.py' -v
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
```

Use the package scripts when their coverage is equivalent. Do not substitute a
partial command for `npm run verify` when the change class or approved plan
requires the complete suite. Do not run unrelated source tests or builds for a
documentation-only change unless executable tooling, generated artifacts,
tested examples, or another explicit policy requires them.

## Change-to-test matrix

| Change                           | Minimum completion evidence                                                                        |
| -------------------------------- | -------------------------------------------------------------------------------------------------- |
| Pure TypeScript model or reducer | Formatting, lint, typecheck, adjacent Vitest cases, and frontend build when output can change      |
| React behavior                   | Formatting, lint, typecheck, Testing Library interaction/accessibility cases, and relevant build   |
| IPC client or event parser       | Complete verification plus unknown, valid, invalid, and failure contract cases                     |
| Rust validation or state machine | Rust formatting, strict Clippy, unit table, and affected public contract cases                     |
| Tauri command or event           | Complete verification plus Rust boundary and frontend narrowing tests                              |
| SQLite or migration              | Complete verification plus in-memory and file-backed success/failure integration tests             |
| Policy or approval               | Complete verification plus identity, denial, replay, expiry, cancellation, and redaction cases     |
| Native macOS behavior            | Affected automated checks plus target-Mac manual evidence                                          |
| Documentation-only               | Status, diff check, Markdown formatting, link/path audit, and protected-path scope proof           |
| GitHub workflow or template      | YAML parse, immutable actions, permissions/triggers, no-secret/no-write policy, and command checks |
| Dependency change                | Focused behavior, complete verification, audit, license, and lockfile review                       |

## GitHub change-to-workflow matrix

| Changed paths or event                                     | Documentation | Frontend      | Rust          | Dependency audit |
| ---------------------------------------------------------- | ------------- | ------------- | ------------- | ---------------- |
| Markdown, prompts, project memory, or governance only      | Yes           | No            | No            | No               |
| React, TypeScript, CSS, brand assets, Vite, Vitest, ESLint | When mixed    | Yes           | No            | No               |
| Rust tests or deliberately allowlisted isolated Rust       | When mixed    | No            | Yes           | No               |
| Production Rust or native/trust-boundary examples          | When mixed    | Yes           | Yes           | Yes              |
| Tauri, IPC, policy, approval, storage, migration, security | When mixed    | Yes           | Yes           | Yes              |
| JavaScript, Rust, or action dependency metadata            | When mixed    | Yes           | Yes           | Yes              |
| CI workflow or executable CI-validation script             | When mixed    | Yes           | Yes           | Yes              |
| Documentation workflow                                     | Yes           | No            | No            | Yes              |
| Repository governance validator or its tests               | Yes           | No            | No            | Yes              |
| Repository hooks or hook tests                             | No            | No            | No            | Yes              |
| Mixed documentation and application paths                  | Yes           | As classified | As classified | As classified    |
| Weekly schedule                                            | No            | No            | No            | Yes              |
| Manual CI dispatch                                         | No            | Yes           | Yes           | Yes              |
| Manual Documentation dispatch                              | Yes           | No            | No            | No               |

Both workflows use event-level path filters so documentation-only pull
requests never start Application CI. Within Application CI,
`scripts/ci_change_scope.py` decides which jobs run. The classifier treats an
unknown non-documentation path as cross-cutting and runs both application jobs.
When a new source or governance path is introduced, update event paths,
classifier rules, fixtures, and this matrix together.

## Completion gate

Before ending implementation:

1. Run every required automated command and manual check.
2. Review failures and rerun only after the repository or environment issue is
   understood.
3. Do not rerun an identical successful check unless relevant content changed
   afterward or an explicit policy requires it.
4. Record passed, failed, not-run, and pending manual checks separately. Record
   the risk-based reason for checks that are not applicable.
5. Review the complete diff and confirm tests cover the changed contract.
6. Synchronize documentation with actual results.
7. Run `python3 .codex/hooks/session_end_gate.py` and resolve conflicts or
   unexpected paths.
8. Run `$quality-gate`, then `$post-increment-gate`. Require the expected valid
   passing completion marker, or for a truthful `FAIL`, require a valid terminal
   failed record with no completion marker.

Flaky, skipped, ignored, quarantined, or environment-blocked tests are not
passes. Record the limitation and keep the increment incomplete unless its plan
explicitly permits the missing evidence.

Terminal failure preserves the non-passing result; it does not convert a failed,
not-run, skipped, ignored, quarantined, or pending requirement into a pass.

The D-098 recovery additionally requires focused tests for exact schema-v3
keys, v1/v2 compatibility, immutable predecessor evidence, exact allowlisted
paths and identities, passing-report enforcement, blocked-evidence rejection,
argument-free command parsing, idempotent exact replay, altered replay denial,
report/state/workspace drift, conflict and suspicious-path rejection, redacted
status, Stop behavior, no completion marker, and clean admission of only the
recorded successor with `predecessor_disposition` lineage. Do not exercise the
real transition until every focused and complete check passes and the recovery
report is frozen.
