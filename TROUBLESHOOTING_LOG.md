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
Status: Resolved 2026-07-15

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

### Resolution

The verified product increments through 4U and Meta Increment 1 are now
published and merged. At the Meta Increment 2 baseline, `main`, `origin/main`,
and the working branch base all resolve to `5edbf4d`, and the working tree began
clean. Continue to verify synchronization at session start; do not assume this
historical condition remains resolved after future local work.

## TS-011 — Increment 2D actions are not visible in the standard macOS application menu

Date: 2026-07-13
Status: Resolved

### Symptom

The application launches and the left-side **Cortexa** application menu contains standard macOS items such as About, Services, Hide, and Quit, but does not show:

```text
Open Cortexa
New Request
Tasks (Coming Soon)
Quit Cortexa
```

The source still shows that the New Request and Tasks menu items are constructed.

### Cause

The standard application-name menu on the left side of the macOS menu bar is separate from the custom Tauri status-item menu. Increment 2D installs the custom menu under the Cortexa status icon on the right side of the menu bar, near system status items.

### Resolution

Click the Cortexa status icon on the right side of the macOS menu bar. Its menu contains the four fixed Increment 2D actions.

### Verify

1. Hide the main window with its red close control.
2. Open the right-side Cortexa status-item menu.
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

## TS-013 - Tauri development launch reports port 1420 already in use

Date: 2026-07-14
Status: Resolved

### Symptom

`npm run tauri -- dev` exits before launching the native application because Vite reports that port 1420 is already in use.

### Cause

A stale standalone `npm run dev` process for this repository still owns the fixed Vite listener. Starting Tauri launches a second `beforeDevCommand`, which cannot bind the same port.

### Resolution

Identify the listener and its parent before stopping anything:

```bash
lsof -nP -iTCP:1420 -sTCP:LISTEN
ps -p <listener-pid> -o pid=,ppid=,lstart=,command=
ps -p <parent-pid> -o pid=,ppid=,lstart=,command=
```

If the output proves the listener is the stale Vite child of this repository's standalone `npm run dev`, stop that parent process. Confirm the port is free, then rerun the exact Tauri command.

### Verify

```bash
lsof -nP -iTCP:1420 -sTCP:LISTEN
npm run tauri -- dev
```

Expected: the first command has no listener before launch; Vite starts on port 1420, Cargo launches the unchanged `target/debug/ai-agent-assistant` executable, and the native application opens.

### Prevention

Stop standalone Vite sessions when their work ends. Before a native manual gate, identify an existing fixed-port listener instead of starting overlapping dev servers or terminating an unrelated process.

## TS-014 - Completion marker becomes invalid after committing tracked deletions

Date: 2026-07-15
Status: Resolved by Repository Workflow Increment 4J; affected pre-fix increments require reconstruction

### Symptom

A post-increment marker is valid immediately after finalization, but `python3 .codex/hooks/post_increment_gate.py status` reports `valid: false` after committing a change set that deletes tracked files. Re-finalization on the clean committed branch fails with `report file inventory does not match the complete Git change set`.

### Cause

Before commit, `git ls-files --cached` still includes each deleted tracked path. The old fingerprint hashed that path plus a `missing` token. After commit, Git no longer lists the deleted path, so it contributes nothing and the fingerprint changes even though the reviewed working-tree content did not.

The original post-commit regression created or modified files only and did not exercise a tracked deletion.

### Resolution

Fingerprint only repository paths that exist in the current working-tree snapshot. Keep exact deletion evidence in the independent `changed_paths` and report-inventory validation. Add regressions proving both reviewed deletion-commit stability and invalidation when a tracked file is deleted after finalization.

Do not migrate or reinterpret pre-fix markers. Preserve and reconstruct an affected increment on corrected `main`, rerun its required verification and gate, and confirm its marker remains valid after commit before publication.

### Verify

```bash
python3 .codex/hooks/tests/test_post_increment_gate.py -v
npm run test:hooks
python3 .codex/hooks/post_increment_gate.py status
```

Expected: all 17 focused tests pass. The reviewed deletion fixture remains valid after commit, while the post-finalization deletion fixture requests continuation.

### Prevention

Any future fingerprint or changed-file implementation must test additions, modifications, deletions, pre-commit state, post-commit state, and an unreviewed change after finalization. Keep report inventory and workspace-content fingerprint responsibilities distinct.

## TS-015 - Independently merged dependency updates break clean verification

Date: 2026-07-16
Status: Resolved in the verified dependency baseline repair; publication pending

### Symptom

Pull-request checks fail before product tests. Hosted `npm ci` reports a Vite
peer conflict, local `npm ci` reports invalid `package.json`, and Rust Clippy
fails in `libsqlite3-sys` with unstable `cfg_select` on Rust 1.90.

### Cause

Several Dependabot pull requests were based on overlapping older dependency
states and merged independently. The resulting `main`:

- removed the direct `vitest@3.2.6` manifest entry and left a trailing comma;
- retained duplicate direct `typescript-eslint` and `vite` lockfile keys;
- selected `vite@8.1.4` outside `@vitejs/plugin-react@4.7.0`'s peer range; and
- selected `rusqlite@0.40.1`, whose `libsqlite3-sys@0.38.1` build script does
  not compile on the repository's supported Rust toolchain.

An initial lock regeneration also retained nested Vite `7.3.6` instances under
Vitest, causing duplicate TypeScript plugin type identities against root Vite
`7.3.5`.

### Resolution

Restore the previously verified exact direct versions `vite@7.3.5`,
`vitest@3.2.6`, and `rusqlite@0.37.0`. Regenerate both lockfiles, run `npm
dedupe`, and preserve all unrelated compatible updates already on `main`.

### Verify

```bash
npm ci
npm ls vite @vitejs/plugin-react vitest
npm run typecheck
cargo tree --manifest-path src-tauri/Cargo.toml -i rusqlite --locked
npm run verify
npm audit --audit-level=low
```

Expected: npm reports one deduplicated `vite@7.3.5`, Cargo reports exact
`rusqlite@0.37.0`, and all repository checks pass.

### Prevention

Do not merge overlapping dependency proposals solely because their original
branch checks passed. Refresh each proposal against current `main`, require a
clean install and full check on the merge candidate, and group coupled major
updates such as Vite/plugin/Vitest or rusqlite/toolchain changes into one
reviewed compatibility increment.

## TS-016 - GitHub-hosted jobs fail before runner assignment

Date: 2026-07-17
Status: Resolved

### Symptom

CI, Documentation, and Security checks complete as failures within seconds and
show no checked-out source or command output. GitHub annotates each job that it
was not started because account payments failed or the spending limit must be
increased.

### Cause

The failure occurs before a runner is assigned and is not a repository test
failure. All three workflows selected GitHub-hosted macOS or Ubuntu images. The
registered repository runner was online but could not match those `runs-on`
labels.

### Resolution

Register a dedicated Linux x64 runner, assign it the custom `cortexa-ci` label,
and route the three read-only workflows through the exact four-label selector.
Do not use `pull_request` on the persistent runner; restrict pushes to the
documented maintainer-controlled branch families. Provision Tauri Linux
prerequisites on the host rather than installing system packages with workflow
`sudo`.

### Verify

```bash
gh api repos/SillyRbbit/ai-agent-assistant/actions/runners
npm run test:repository
npm run docs:check
npm run repository:check
npm run verify
```

After publishing the workflow branch, confirm all three GitHub jobs name the
intended runner and pass. Until that remote execution succeeds, this resolution
remains verification pending.

### First self-hosted result

PR #24 commit `80bced4` proved exact routing: Documentation passed on runner 21.
CI run `29624042629` and Security run `29624042656` reached the same runner but
failed at the prerequisite step after finding `git` and `python3` because
`command -v rustup` returned exit code 1. Install or activate Rustup for the
runner service account, refresh the runner-captured path so
`$HOME/.cargo/bin` is visible, restart the service, and rerun the two failed
jobs. This is a host prerequisite failure, not a repository test failure.

The first repair installed Rustup `1.29.0`, Cargo, and default toolchain 1.90.0
under `/home/henry-dang/.cargo/bin`. `svc.sh stop/start` then failed because the
runner had never been installed as a `systemd` service; GitHub still reported it
online through the earlier interactive listener. Stop that listener, run
`./env.sh` after sourcing `$HOME/.cargo/env`, install the service for
`henry-dang`, and start it before rerunning the failed jobs.

CI and Security attempt 2 still failed at `command -v rustup` after the service
setup was reported complete. Before another rerun, inspect `.service`,
`svc.sh status`, `.path`, and all `Runner.Listener` or `runsvc.sh` processes.
This distinguishes a captured-PATH defect from an old interactive listener that
is still receiving jobs.

The inspection found both stale interactive PID `7699` and managed service
listener PID `36245`. The service was active and its `.path` correctly began
with `/home/henry-dang/.cargo/bin`, while its journal repeatedly reported that a
session for the runner already existed. Stop only the stale interactive
listener, restart the service, and require one remaining listener with
`--startuptype service` before rerunning workflows.

Stopping the stale listener and restarting the service fixed runner routing and
PATH inheritance. Attempt 3 passed all host prerequisites. Documentation and
Security pass. CI then exposed a distinct repository portability issue: strict
Linux Clippy rejects five private approval-source support items because their
only consumer is the macOS-gated decision-source module. Do not suppress or
weaken Clippy; handle that source correction only through separately approved
scope.

The project owner approved the exact two-file correction. Target-gating only
the private import, presentation marker/parts and conversion, and native
evidence constructors removes their non-macOS compile presence while preserving
the complete macOS path. Focused approval-manager tests, strict Clippy, and
`npm run verify` pass locally. At that checkpoint the correction remained
uncommitted, so PR #24 still needed a successful Linux CI rerun.

Commit `1621a55` published the correction. Security run `29629669283`,
Documentation run `29629669305`, and CI run `29629669300` all passed on runner 21. The corrected CI completed full Linux verification in 9 minutes 57 seconds,
resolving the runner-host and strict-Clippy portability incident.

### Prevention

Keep the runner-specific label, no-pull-request rule, and push allowlist covered
by repository-health tests. Reapply the label after runner replacement, keep
the service account unprivileged and credential-free, and preserve separate
target-Mac verification for native behavior.

## TS-017 - Certificate Assistant cannot create the Developer ID CSR

Date: 2026-07-31
Status: Unresolved; operation stopped safely

### Symptom

On the macOS 26.6 arm64 target Mac, Keychain Access Certificate Assistant
reported `The specified item could not be found in the keychain.` while the
owner attempted to save the separately approved Developer ID Application CSR.
No CSR file became available.

### Observed evidence

- The user keychain list and default-keychain read-only checks identified the
  login keychain, but keychain-info checks returned parameter-related errors.
- A read-only code-signing identity query found zero valid identities, which was
  expected before certificate creation and does not explain the CSR failure.
- A generic `<key>` row was visible before the failed attempt. It is not
  evidence of a key created by this attempt.
- The owner confirmed: no CSR file created, no certificate created, and no new
  named private key observed.

### Cause

Not determined. The observed error and read-only diagnostics do not prove
Keychain corruption, a missing keychain item, an access-control defect, or any
other root cause. No such cause should be inferred without a separately
approved diagnostic plan and reproducible evidence.

### Safe disposition

Stop the operational increment as `unavailable`. Do not retry CSR creation,
reset, unlock, replace, or delete Keychain state, generate a private key through
Terminal or OpenSSL, create a different certificate type, contact Apple support,
or continue to certificate creation under this increment. No rollback action is
needed because the owner observed no CSR file, certificate, or new named private
key.

### Verify

Use only the owner's sanitized confirmation that no CSR file, certificate, or
new named private key was created. Repository closure must pass documentation,
repository-policy, secret-scan, whitespace, product-path, session-end, and
post-increment checks without recording account, certificate, key, or Keychain
identifiers.

### Prevention

Before any future attempt, approve a documentation-only remediation plan that
defines the exact read-only Keychain diagnostics, expected results, privacy
limits, stop conditions, and recovery/rollback decision points. Continue to
prohibit command-line private-key file generation and any unplanned Apple,
signing, Keychain, credential, Cloudflare, provider, deployment, traffic, or
runtime action.

The documentation-only
[`macos-certificate-assistant-csr-remediation-plan.md`](docs/plans/macos-certificate-assistant-csr-remediation-plan.md)
now defines that future diagnostic boundary. It does not approve execution;
separate explicit owner approval remains required before any observation runs.

### Approved read-only diagnostic outcome

Date: 2026-08-01

The owner performed each of the plan's three local read-only observations once
and reported only the approved sanitized categories:

- user Keychain configuration: `observed`;
- default Keychain configuration: `observed`;
- valid code-signing identities: `zero`;
- authorization prompt: `not observed`; and
- state changed: `not observed`.

These observations show that the configured user/default Keychain state was
readable without an authorization prompt and that no valid code-signing identity
was present. They do not reproduce the original Certificate Assistant failure
or distinguish among possible causes. The cause remains `not determined`; no
resolution has been performed, and no retry or remediation is authorized.

### Owner decision

Date: 2026-08-01

D-076 records the owner's decision to defer the signed macOS identity path.
Apple Support assistance and an alternate CSR workflow were considered but are
not authorized. The outcome remains unresolved and does not justify a Keychain
repair, CSR retry, signing action, or alternate key-generation path.

The documentation-only
[`apple-support-ts-017-assistance-plan.md`](docs/plans/apple-support-ts-017-assistance-plan.md)
defines a possible future owner-only support contact. It does not authorize that
contact or any response action.

### Stopped contact and filesystem signing-material outcome

Date: 2026-08-02

The owner did not contact Apple Support or access Apple Developer. One CSR file
and one filesystem private-key file were created outside the approved contact
scope. Neither was uploaded, used, copied, exported, or backed up; no
certificate exists. Encryption and permissions were not inspected and remain
undetermined. The material does not satisfy D-072's non-exported Keychain
boundary. No disposition action is authorized.

The documentation-only
[`filesystem-signing-material-disposition-plan.md`](docs/plans/filesystem-signing-material-disposition-plan.md)
selects future abandonment and paired deletion. Execution remains unauthorized.

### Paired filesystem signing-material disposition

Date: 2026-08-02

Under a separate owner-operated approval, the owner privately identified exactly
the unuploaded CSR and its filesystem private-key file, observed no additional
signing material, deleted both as one paired disposition, and verified their
absence. Sanitized evidence reports no remaining copy, upload, use, or
certificate. No filename, path, content, key material, account detail, or other
private evidence entered the repository.

This outcome closes custody of the known filesystem pair only. It does not prove
cryptographic erasure from APFS/SSD remnants or snapshots, establish TS-017's
cause, satisfy D-072, or supersede D-076. No recovery, regeneration, signing, or
credential action is authorized.

### Conditional Apple Support-contact consideration

Date: 2026-08-02

D-077 records the owner's choice to conditionally reopen consideration of one
future owner-operated Apple Support contact under the existing TS-017 assistance
plan. It does not authorize that contact, Apple Developer access, diagnostic
repetition, CSR work, Keychain action, or any signing action. TS-017 remains
`not determined`; D-076 continues to defer the signed-identity path.

Any future operational approval must preserve the plan's minimum sanitized
summary, privacy limits, no-screen-share/no-upload/no-device-access rule,
no-execution rule, stop conditions, and closed outcome reporting. End the
contact without acting on advice if any state-changing or prohibited request is
made.

### D-077 owner-contact closed without contact

Date: 2026-08-04

The separately approved owner-operated contact increment ended without an Apple
Support contact. The owner reported only the approved closed categories:

- contact attempted: `no`;
- guidance: `none`;
- state changed: `not observed`; and
- cause: `not determined`.

No Apple Support or Apple Developer access, disclosure, diagnostic, or state
change occurred. The operational approval is closed and does not carry forward.
TS-017 remains unresolved, and any future contact requires another separately
approved exact operational increment under D-077 and the existing assistance
plan.
