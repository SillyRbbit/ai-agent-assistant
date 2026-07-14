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

## TS-008 — Full Increment 2B-1 native verification unavailable on artifact host

Date: 2026-07-13
Status: Resolved on target Mac; artifact-host limitation remains

### Symptom

The storage-only Rust crate compiles, passes Clippy, and passes its focused tests, but the artifact-generation host cannot complete the full Tauri crate's native dependency build or produce target-Mac verification evidence.

### Cause

The artifact host is not the Apple Silicon macOS target and does not provide the complete native desktop dependency environment required by the Tauri crate. The bundled SQLCipher/OpenSSL feature must also be confirmed with the repository's pinned Rust toolchain on the target Mac.

### Safe resolution

Apply the Increment 2B-1 source overlay on the target Mac, then run one unlocked check to resolve the new exact dependencies and update the lockfile:

```bash
export PATH="$(brew --prefix rustup)/bin:$PATH"
rustup default 1.90.0
cargo check --manifest-path src-tauri/Cargo.toml
```

Review the lockfile:

```bash
git diff -- src-tauri/Cargo.lock
```

Then run the required locked verification:

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
npm run typecheck
npm run build
npm run tauri -- dev
```

### Prevention

- Commit the target-Mac-generated lockfile with the increment.
- Keep native dependency changes in isolated increments.
- Do not mark the increment complete based only on the storage harness.
- Do not remove SQLCipher features to make an unrelated host pass; record a superseding decision if the target Mac exposes a real blocker.

## TS-009 — libsqlite3-sys 0.38.1 fails on pinned Rust 1.90

Date: 2026-07-13
Status: Resolved

### Symptom

The first Increment 2B-1 dependency selection failed while compiling `libsqlite3-sys 0.38.1`:

```text
error[E0658]: use of unstable library feature `cfg_select`
```

The same checkout could also report missing `agent`, `approvals`, `audit`, `memory`, `platform`, `policy`, and `tools` modules when an overlay was applied to a public baseline that did not contain Increment 2A.

### Cause

- `rusqlite 0.40.1` resolved to a `libsqlite3-sys` build script requiring a newer Rust standard-library feature than the repository's pinned Rust 1.90.0 provides.
- The original storage overlay assumed the verified Increment 2A module tree already existed locally.

### Resolution

- Restore the complete Increment 2A module tree.
- Pin `rusqlite` to 0.37.0 with the same bundled SQLCipher and vendored OpenSSL feature.
- Regenerate the lockfile with Cargo.

Verified dependency tree:

```text
rusqlite v0.37.0
libsqlite3-sys v0.35.0
```

### Verify

```bash
cargo tree --manifest-path src-tauri/Cargo.toml | grep -E 'rusqlite|libsqlite3-sys'
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
npm run typecheck
npm run build
npm run tauri -- dev
```

The project owner confirmed all checks and the native launch passed on the target Mac.

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

## TS-010 — Public repository lags the verified local checkout

Date: 2026-07-13
Status: Open operational condition

### Symptom

The public GitHub page still reports one commit and describes the original runnable shell, while the project owner's local checkout contains verified Increments 2A through 2D.

### Cause

The verified local changes have not all been pushed to the public branch, or the public page has not caught up with the local working state.

### Safe handling

- Treat the project owner's verified local checkout and current project-memory files as the implementation baseline.
- Apply overlays only to `/Users/hdang/Desktop/Projects/ai-agent-assistant` after confirming `git status`.
- Do not reconstruct a later increment solely from the public branch.
- Commit and push verified checkpoints before relying on GitHub as the source of truth.

### Verify

```bash
git status --short --branch
git log -5 --oneline --decorate
git remote -v
```

Do not publish secrets, local databases, credentials, certificates, or environment files when synchronizing the public repository.

## TS-011 — Increment 2D actions are not visible in the standard macOS application menu

Date: 2026-07-13
Status: Resolved

### Symptom

The application launches and the left-side **AI Agent Assistant** application menu contains standard macOS items such as About, Services, Hide, and Quit, but does not show:

```text
Open AI Agent Assistant
New Request
Tasks (Coming Soon)
Quit AI Agent Assistant
```

The source still shows that the New Request and Tasks menu items are constructed.

### Cause

The standard application-name menu on the left side of the macOS menu bar is separate from the custom Tauri status-item menu. Increment 2D installs the custom menu under the AI Agent Assistant status icon on the right side of the menu bar, near system status items.

### Resolution

Click the AI Agent Assistant status icon on the right side of the macOS menu bar. Its menu contains the four fixed Increment 2D actions.

### Verify

1. Hide the main window with its red close control.
2. Open the right-side AI Agent Assistant status-item menu.
3. Select **New Request** and confirm the window returns and receives focus.
4. Hide the window again.
5. Select **Tasks (Coming Soon)** and confirm the window returns and receives focus.

The project owner confirmed both actions worked without an error.

### Prevention

Manual test instructions should consistently use the term **menu-bar status item** and distinguish it from the standard macOS application menu.

## TS-012 — React component tests retain prior rendered shells

Date: 2026-07-13
Status: Resolved

### Symptom

When multiple Increment 2E component tests run in one Vitest process, role and text queries can find elements from an earlier render, producing ambiguous-match failures even though each test passes by itself.

### Cause

The test environment was not explicitly cleaning React Testing Library's rendered DOM after every test. Depending on test-runner integration alone made cleanup behavior implicit.

### Resolution

Add an explicit test-only cleanup hook in `src/test/setup.ts`:

```ts
import { cleanup } from "@testing-library/react";
import { afterEach } from "vitest";

afterEach(() => {
  cleanup();
});
```

### Verify

```bash
npx vitest run
```

Expected:

```text
3 test files passed
30 tests passed
0 failed
```

### Prevention

Keep global DOM-test cleanup explicit in the shared Vitest setup and avoid relying on test order or prior component unmount behavior.
