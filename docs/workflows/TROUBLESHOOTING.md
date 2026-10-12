# Troubleshooting workflow

## Goal

Find the smallest verified cause and fix without introducing unrelated upgrades or weakening project controls.

Use `.agents/skills/troubleshoot/SKILL.md` when repository skills are available.
Otherwise use `prompts/increments/bug-fix.md`; use
`prompts/workflows/remediation.md` instead for an advisory backlog.

## Current application triage — 2026-10-12

Use the [documented main baseline](../../ARCHITECTURE.md#current-and-future-capability-matrix)
and [provider guide](../provider-milestone.md#current-provider-guide--2026-10-12)
before diagnosing a different worktree/build as the same application.

| Symptom                                            | Evidence-based next check                                                                                                                                                                         |
| -------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Browser view lacks native features                 | `npm run dev` is browser-only; normal native development uses `npm run tauri -- dev`. Do not launch solely for a docs audit.                                                                      |
| Hosted connection blocked                          | Verify debug build, enable flag and private native credential/store setup; Save/discovery is not a successful live request.                                                                       |
| Codex model/effort looks unsupported after restart | Refresh is explicit metadata access; saved effort may be unverified until discovery. The old version hint is not a compatibility guarantee.                                                       |
| Codex fails before first response                  | Record sanitized setup/runtime category and exact installed version/build; managed-policy/configuration failure can precede transport. Never substitute an API key or personal assistant profile. |
| `resource_limit`, partial or failed stream         | Inspect fixed Settings diagnostics; the 128-update journal can cap fragmented responses. Partial text is not completion; recovery needs explicit new Send.                                        |
| New Send blocked after Stop                        | Native generation/cleanup may still own the session. UI terminal state alone does not prove transport cleanup; do not auto-resubmit.                                                              |
| Knowledge source appears missing or historical     | Verify saved item/version/hash and selected passage; imports are snapshots, and historical links do not silently follow a changed title.                                                          |
| Unsaved Knowledge edit disappears on Quit          | In-app navigation guard is not an app-wide native Quit guard; explicitly save before quitting.                                                                                                    |
| Main versus development CI waits                   | Main Linux jobs select `cortexa-linux`; other refs select `cortexa-ci`. Runner labels do not provide failover or isolation. See [runner guide](../github/SELF_HOSTED_RUNNER.md).                  |

Diagnostics contain bounded event/category metadata, not prompts, transcripts,
credentials or raw upstream errors. Explicit export is sanitized; do not attach
owner databases or auth files. Preserve historical failing receipts. External
QA/build temporary paths may no longer exist: cite retained reports and label
artifact verification unavailable rather than recreating evidence or claiming
current success. Environment workarounds belong to their recorded builds; do
not change global Xcode/Python/Cargo settings to make a documentation check pass.

## 1. Capture the failure exactly

Record:

- Full command.
- Exact error text.
- Working directory.
- Operating system and architecture.
- Node, npm, Rust, Cargo, and Xcode tool versions.
- Whether the failure is new or previously working.
- Relevant uncommitted changes.

Never paste secrets, tokens, personal files, or private service payloads into the log.

## 2. Classify the failing layer

| Layer               | Typical evidence                                       |
| ------------------- | ------------------------------------------------------ |
| Shell/PATH          | Command not found, wrong executable path               |
| Toolchain           | Unsupported engine, missing compiler or target         |
| Dependency install  | Lockfile, registry, native package failure             |
| Frontend build      | TypeScript, Vite, React, ESLint failure                |
| Rust build          | Cargo metadata, compiler, Clippy, linker failure       |
| Tauri configuration | Capability, CSP, bundle, window startup failure        |
| Runtime IPC         | Invoke rejection, serialization, command registration  |
| Platform            | macOS permission, SDK, architecture, deployment target |

## 3. Establish a minimal reproduction

Run the narrowest failing command directly. Examples:

```bash
npm ci
npm run typecheck
npm run test:unit
cargo metadata --manifest-path src-tauri/Cargo.toml --no-deps --format-version 1
cargo check --manifest-path src-tauri/Cargo.toml --locked
npm run tauri -- dev
```

Do not begin with a dependency upgrade or delete lockfiles unless evidence points there.

## 4. Inspect environment before source

```bash
which node
node --version
which npm
npm --version
which rustup
rustup show active-toolchain
which cargo
cargo --version
which rustc
rustc --version
xcode-select -p
```

On Homebrew Rustup installations, confirm:

```bash
export PATH="$(brew --prefix rustup)/bin:$PATH"
```

## 5. Form and test one hypothesis

State:

```text
Observed evidence:
Hypothesis:
Smallest change or command to test it:
Expected result:
Actual result:
```

Change one variable at a time.

## 6. Verify the fix

Re-run:

1. The original failing command.
2. The nearest related check.
3. A regression check for the affected layer.

For a Tauri launch issue, verify both a targeted Cargo command and `npm run tauri -- dev`.

## 7. Preserve the learning

Append an entry to `TROUBLESHOOTING_LOG.md` with:

- Symptom.
- Root cause.
- Resolution.
- Verification.
- Prevention.

Update setup instructions if another developer could encounter the same issue.

## Stop conditions

Stop broad implementation work when:

- The baseline project no longer compiles.
- A required tool or SDK is missing.
- The fix would require weakening a security control.
- The root cause is unknown and multiple unrelated edits would be needed.
- A production credential or private data would be required.

Document the blocker and leave a reproducible handoff instead of guessing.
