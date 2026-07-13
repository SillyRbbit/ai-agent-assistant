# Troubleshooting log

Use this file for resolved and unresolved environment, build, test, and runtime failures. Preserve history so later sessions do not repeat the same investigation.

## TS-001 — npm EBADENGINE on Node.js 26

Date: 2026-06-18
Status: Resolved

### Symptom

```text
npm error code EBADENGINE
Required: {"node":">=22.12.0 <23","npm":">=10 <11"}
Actual:   {"node":"v26.3.0","npm":"11.16.0"}
```

### Cause

The initial repository engine declaration accepted only Node.js 22 and npm 10 while `.npmrc` enabled `engine-strict=true`.

### Resolution

The repository now declares:

```json
{
  "node": "^22.12.0 || ^24.0.0 || >=26.0.0 <27",
  "npm": ">=10 <12"
}
```

The preferred versions are Node.js 26.3.0 and npm 11.16.0.

### Verify

```bash
node --version
npm --version
rm -rf node_modules
npm ci
```

Expected: installation completes without `EBADENGINE`.

## TS-002 — Tauri cannot run cargo metadata

Date: 2026-06-18
Status: Resolved

### Symptom

```text
failed to run 'cargo metadata' command
failed to run command cargo metadata --no-deps --format-version 1:
No such file or directory (os error 2)
```

### Cause

`cargo` was not available on the interactive shell's `PATH`.

### Resolution

Install or activate Rustup, then verify Cargo before starting Tauri. For the Homebrew `rustup` package on Apple Silicon:

```bash
export PATH="$(brew --prefix rustup)/bin:$PATH"
rehash
rustup default 1.90.0
which cargo
cargo --version
rustc --version
```

Persist the path in Zsh:

```bash
grep -qxF 'export PATH="$(brew --prefix rustup)/bin:$PATH"' "$HOME/.zshrc" ||   printf '
export PATH="$(brew --prefix rustup)/bin:$PATH"
' >> "$HOME/.zshrc"
```

Open a new shell or run:

```bash
exec zsh
```

### Verify

```bash
which cargo
cargo --version
cd /path/to/ai-agent-assistant
npm run tauri -- dev
```

Expected: the native application compiles and launches.

## TS-003 — rustup reports an installed toolchain but cargo is not found

Date: 2026-06-18
Status: Resolved

### Symptom

Rustup reports that `1.90.0-aarch64-apple-darwin` is installed, followed by:

```text
cargo not found
zsh: command not found: cargo
zsh: command not found: rustc
```

### Cause

Homebrew installed Rustup outside the shell's active `PATH`. The presence of a toolchain does not make the shims discoverable unless the Rustup `bin` directory is available.

### Resolution

```bash
export PATH="$(brew --prefix rustup)/bin:$PATH"
rehash
rustup default 1.90.0
```

If Rustup was installed with the official installer instead, use:

```bash
source "$HOME/.cargo/env"
```

Do not add both approaches blindly; use the path that contains the actual `rustup`, `cargo`, and `rustc` executables.

## TS-004 — Zsh treats pasted comment lines as commands

Date: 2026-06-18
Status: Resolved

### Symptom

```text
zsh: command not found: #
```

### Cause

The interactive shell did not have Zsh's `interactivecomments` option enabled.

### Resolution

Either paste commands without comment lines or enable comments:

```bash
setopt interactivecomments
grep -qxF 'setopt interactivecomments' "$HOME/.zshrc" ||   printf '
setopt interactivecomments
' >> "$HOME/.zshrc"
```

## TS-005 — Cargo unavailable in artifact-generation environment

Date: 2026-07-09
Status: Open for artifact host; expected to be resolved on target Mac

### Symptom

The required Increment 2A Rust verification commands failed in the artifact-generation environment with:

```text
bash: line 1: cargo: command not found
```

Affected commands:

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
```

### Cause

The artifact-generation host had Node.js and npm available but did not have Cargo, Rustup, or Rust installed on `PATH`.

### Resolution

Run the Rust checks on the target Mac where Rust 1.90.0 is available, or install/activate Rustup before verification:

```bash
export PATH="$(brew --prefix rustup)/bin:$PATH"
rustup default 1.90.0
which cargo
cargo --version
```

### Verify

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
```

Expected: all three commands pass before Increment 2A is marked complete.

## TS-006 — tsc command not found after applying source ZIP

Date: 2026-07-13
Status: Resolved

### Symptom

```text
> ai-agent-assistant@0.1.0 typecheck
> tsc -b --pretty false

sh: tsc: command not found
```

### Cause

The repository's locked npm dependencies had not been installed in the local checkout after applying the source ZIP. `tsc` is provided by the local `typescript` dev dependency under `node_modules/.bin`.

### Resolution

From the repository root, run:

```bash
npm ci
```

Then rerun:

```bash
npm run typecheck
npm run build
```

Expected: `tsc` is found through npm's local package-bin path and both commands pass.

## TS-007 — cargo fmt --check prints diffs after applying Increment 2A ZIP

Date: 2026-07-13
Status: Resolved

### Symptom

`cargo fmt --check` prints diffs in the new Increment 2A Rust modules, including files under:

```text
src-tauri/src/audit/logger.rs
src-tauri/src/memory/store.rs
src-tauri/src/platform/adapter.rs
src-tauri/src/policy/engine.rs
```

### Cause

The Increment 2A source compiled and tested after formatting, but the applied ZIP contained Rust files that needed standard `rustfmt` formatting on the target Mac.

### Resolution

Run:

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
```

Expected: the first command applies formatting and the second command passes without diff output.

## Quick diagnostic snapshot

Run this before troubleshooting an install or build failure:

```bash
pwd
uname -a
uname -m
sw_vers
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
git status --short --branch
```

Redact usernames, access tokens, private paths, and personal data before sharing logs.

## New entry template

Copy `docs/templates/TROUBLESHOOTING_ENTRY_TEMPLATE.md` and append the completed entry below this section. Include the exact symptom, environment, root cause, smallest fix, verification command, and any prevention step.
