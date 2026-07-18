# Troubleshooting workflow

## Goal

Find the smallest verified cause and fix without introducing unrelated upgrades or weakening project controls.

Use `.agents/skills/troubleshoot/SKILL.md` when repository skills are available.
Otherwise use `prompts/increments/bug-fix.md`; use
`prompts/workflows/remediation.md` instead for an advisory backlog.

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
