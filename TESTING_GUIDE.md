# Cortexa testing guide

Status: Authoritative testing standard
Last updated: 2026-07-15

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

### Hook and repository-workflow tests

Location: `.codex/hooks/tests/test_*.py`.

Use Python `unittest` for hook JSON validation, path containment, fingerprint,
deletion, conflict, loop-guard, and report behavior.

Command:

```bash
npm run test:hooks
```

Repository hooks are developer workflow guardrails, not product security or
authorization evidence.

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

## Required commands

During development, run the narrowest relevant command. Before completing an
increment, run the complete relevant sequence:

```bash
npm run format:check
npm run lint
npm run typecheck
npm run test
npm run build
npm run tauri -- build --no-bundle
```

The canonical combined command is:

```bash
npm run verify
```

Useful exact lower-level commands are:

```bash
npx vitest run
python3 -m unittest discover -s .codex/hooks/tests -p 'test_*.py' -v
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
```

Use the package scripts when their coverage is equivalent. Do not substitute a
partial command for `npm run verify` while calling the result complete.

## Change-to-test matrix

| Change                           | Minimum focused evidence before full verification                                           |
| -------------------------------- | ------------------------------------------------------------------------------------------- |
| Pure TypeScript model or reducer | Adjacent Vitest unit cases                                                                  |
| React behavior                   | Testing Library interaction and accessibility cases                                         |
| IPC client or event parser       | Unknown, valid, invalid, and failure tests                                                  |
| Rust validation or state machine | Unit table plus public contract case                                                        |
| Tauri command or event           | Rust boundary test and frontend narrowing test                                              |
| SQLite or migration              | In-memory and file-backed success/failure integration tests                                 |
| Policy or approval               | Exact identity, denial, replay, expiry, cancellation, and redaction cases                   |
| Native macOS behavior            | Automated portable policy tests plus target-Mac manual evidence                             |
| Documentation-only               | Markdown formatting, link/path audit, protected-path diff, and full repository verification |
| Dependency change                | Focused behavior, full verification, audit, license, and lockfile review                    |

## Completion gate

Before ending implementation:

1. Run every required automated command and manual check.
2. Review failures and rerun only after the repository or environment issue is
   understood.
3. Record passed, failed, not-run, and pending manual checks separately.
4. Review the complete diff and confirm tests cover the changed contract.
5. Synchronize documentation with actual results.
6. Run `$post-increment-gate` and require the expected valid marker.

Flaky, skipped, ignored, quarantined, or environment-blocked tests are not
passes. Record the limitation and keep the increment incomplete unless its plan
explicitly permits the missing evidence.
