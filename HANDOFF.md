# Handoff

Last updated: 2026-07-15

## Current state

Phase 3 and Phase 4 Increments 4A through 4F are verified complete on the target Mac. Repository Workflow Increments 4G and 4J and Increment 4H are verified, published, and merged into `main` at `a2b9803`.

Increment 4I remove generic audit scaffold is verified complete after reconstruction on uncommitted, unpushed `codex/phase4-increment-4i`. The fresh branch starts from corrected, synchronized 4J `main` at `a2b9803`. The original pre-fix implementation commit is preserved exactly at `cf9d701` on `codex/phase4-increment-4i-pre-fingerprint-fix` and remains unpushed.

The old change was applied with `git cherry-pick --no-commit cf9d701`. It deletes only the unused public `audit::logger` and `audit::types` modules and removes their exports from `audit::mod`. The typed adapter remains unchanged. No replacement audit API, dependency, persistence, caller, coordinator, dispatch, executor, IPC, UI, network, credential, capability, or permission was added.

## Increment 4I reconstruction state

### Exact source scope

Deleted:

```text
src-tauri/src/audit/logger.rs
src-tauri/src/audit/types.rs
```

Changed:

```text
src-tauri/src/audit/mod.rs
```

The `mod.rs` change removes only the two deleted module exports and preserves `pub mod approval;`. No test file changed.

### Exact files changed

```text
AGENTS.md
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
docs/increments/04i-remove-generic-audit-scaffold.md
docs/plans/04i-remove-generic-audit-scaffold.md
docs/plans/README.md
docs/reviews/2026-07-15-04i-post-increment-review.md
src-tauri/src/audit/logger.rs
src-tauri/src/audit/mod.rs
src-tauri/src/audit/types.rs
```

Merged 4J hook, test, security, troubleshooting, plan, increment, and report files remain unchanged. Manifests, lockfiles, Tauri configuration, frontend, storage, gateway, provider, policy, approval, typed approval-audit implementation, coordinator, dispatch, executor, packaging, and permission files are unchanged.

### Verification classification

Passed:

```text
npm run test:hooks
  corrected-main baseline: 17 passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked audit::
  corrected-main baseline: 10 passed
cargo test --manifest-path src-tauri/Cargo.toml --test approval_audit_binding --locked
  corrected-main baseline: 1 passed
python3 .codex/hooks/post_increment_gate.py begin --increment 04i
  passed; active increment 04i before applying changes
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
  passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked audit::approval::
  6 passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked approvals::decision_source::
  11 passed
cargo test --manifest-path src-tauri/Cargo.toml --test approval_audit_binding --locked
  1 passed
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
  passed
rg -n "AuditEventInput|AuditEvent|AuditLogger|InMemoryAuditLogger|NoopAuditLogger|redact_secret_like_content" src-tauri/src src-tauri/tests
  no matches; required exit status 1
npm run verify
  hook tests: 17 passed
  frontend tests: 124 passed
  Rust library tests: 95 passed
  Rust integration tests: 11 passed
  formatting, ESLint, Clippy, typecheck, builds, and Tauri release no-bundle passed
npm audit --audit-level=low
  network-enabled retry passed with 0 vulnerabilities
git diff --check
  passed
python3 .codex/hooks/post_increment_gate.py status
  complete, valid: true, PASS WITH ADVISORIES after finalization
```

Failed and resolved:

- The non-committing cherry-pick produced expected content conflicts only in the eight shared closeout documents. They were reconciled from corrected 4J `main` while retaining the original exact 4I scope and D-030.
- The first sandboxed npm audit could not resolve the registry or write user-level npm logs. The approved network-enabled retry passed with zero vulnerabilities.

Checks not run:

- No native application or UI interaction was required because the removed scaffold had no production caller or user-visible path.
- No RustSec audit was required because manifests and lockfiles are unchanged.

Manual verification pending: none. Complete scope, conflict, secret, generated-output, code, architecture, and security reviews passed. All merged 4J implementation/evidence files and `audit::approval` remain byte-for-byte unchanged.

### Exact next task

Wait for explicit project-owner direction to commit, push, and merge reconstructed Increment 4I. Do not start later work.

## Repository Workflow Increment 4J state

### Exact source/test scope

```text
.codex/hooks/post_increment_gate.py
.codex/hooks/tests/test_post_increment_gate.py
```

The fingerprint now hashes a repository path only after `lstat` confirms it exists. An already reviewed deletion therefore contributes no content before or after commit. A file present at finalization remains hashed, so deleting it afterward invalidates the marker.

### Exact documentation scope

```text
AGENTS.md
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
SECURITY.md
TROUBLESHOOTING_LOG.md
docs/increments/04j-post-increment-deletion-fingerprint.md
docs/plans/04j-post-increment-deletion-fingerprint.md
docs/plans/README.md
docs/reviews/2026-07-15-04j-post-increment-review.md
```

No hook configuration, skill, application source, dependency, manifest, lockfile, Tauri, IPC, storage, gateway, approval, audit, dispatch, executor, capability, or permission file changes.

### Verification classification

Passed:

```text
npm run test:hooks
  baseline: 15 passed
PYTHONDONTWRITEBYTECODE=1 python3 .codex/hooks/tests/test_post_increment_gate.py -v
  implementation: 17 passed
npm run test:hooks
  implementation: 17 passed
PYTHONPYCACHEPREFIX=/private/tmp/cortexa-4j-pycache python3 -m py_compile .codex/hooks/post_increment_gate.py .codex/hooks/tests/test_post_increment_gate.py
  passed
npm run format:check
  passed
npm run verify
  hook tests: 17 passed
  frontend tests: 124 passed
  Rust library tests: 99 passed
  Rust integration tests: 11 passed
  formatting, ESLint, Clippy, typecheck, builds, and Tauri release no-bundle passed
npm audit --audit-level=low
  network-enabled retry passed with 0 vulnerabilities
git diff --check
  passed
python3 .codex/hooks/post_increment_gate.py status
  complete, valid: true, PASS WITH ADVISORIES after finalization
```

Failed and resolved:

- The first direct unittest command used `.codex/hooks/tests/test_post_increment_gate.py` as a module name and failed with `ValueError: Empty module name` before discovery. Running the file directly passed all 17 tests.
- The first format check found layout-only drift in the two new 4J documents and `PLANS.md`. Targeted Prettier formatting and the exact rerun passed.
- The first sandboxed npm audit could not resolve the registry or write user-level npm logs. The approved network-enabled retry passed with zero vulnerabilities.

Checks not run:

- No native application launch or UI interaction was required because 4J changes no application behavior.
- No RustSec audit was required because Cargo manifests and the lockfile are unchanged.

Manual verification pending: none. The exact hook diff, fixed Git-command boundary, 15-file scope, 4I branch ref, and absence of a remote containing `cf9d701` were reviewed and passed.

Architecture, code-health, and security review found no blocking issue. One edge case found during review was resolved before closeout: deletion of the last tracked file can remove its containing directory, so path validation now checks the nearest existing ancestor before omitting the absent path. The strengthened positive and negative fixtures cover that case.

### Exact next task

Repository Workflow Increment 4J was committed as `a2b9803`, pushed, fast-forward merged into `main`, and its marker remained valid. Its publication prerequisite is satisfied; the current task is the bounded 4I reconstruction above.

Historical publication prompt:

```text
Commit, push, and merge Repository Workflow Increment 4J only. Use a concise workflow-fix commit message, push `codex/repository-workflow-increment-4j`, fast-forward merge it into updated `main`, push `main`, and verify clean synchronized `main` plus the valid 4J marker. Preserve `codex/phase4-increment-4i` at `cf9d701`; do not reconstruct or publish 4I yet.
```

## Increment 4H completion state

### Exact runtime/test scope

Create:

```text
src-tauri/src/audit/approval.rs
src-tauri/tests/approval_audit_binding.rs
```

Change:

```text
src-tauri/src/audit/mod.rs
src-tauri/src/approvals/decision_source.rs
```

The `decision_source.rs` change is test-only. The exact closeout scope, validation matrix, risks, non-goals, verification, and rollback are in [`docs/plans/04h-typed-approval-audit-adapter.md`](docs/plans/04h-typed-approval-audit-adapter.md).

### Exact files changed

```text
AGENTS.md
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
docs/increments/04h-typed-approval-audit-adapter.md
docs/plans/04h-typed-approval-audit-adapter.md
docs/plans/README.md
docs/reviews/2026-07-14-04h-post-increment-review.md
src-tauri/src/approvals/decision_source.rs
src-tauri/src/audit/approval.rs
src-tauri/src/audit/mod.rs
src-tauri/tests/approval_audit_binding.rs
```

`SECURITY.md`, `CODE_REVIEW.md`, `TROUBLESHOOTING_LOG.md`, product/architecture documents, manifests, lockfiles, Tauri configuration, capabilities, frontend, storage, gateway, provider, policy, approval manager, production native-dialog code, and executor files are unchanged.

### Verification classification

Passed:

```text
git status --short --branch
  clean before planning on codex/phase4-increment-4h
npm run typecheck
  planning baseline passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked audit::
  planning baseline: 4 passed
cargo test --manifest-path src-tauri/Cargo.toml --test approval_binding --locked
  planning baseline: 2 passed
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
  passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked audit::approval::
  6 passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked approvals::decision_source::
  11 passed
cargo test --manifest-path src-tauri/Cargo.toml --test approval_audit_binding --locked
  1 passed
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
  passed
npm run verify
  hook tests: 15 passed
  frontend tests: 124 passed
  Rust library tests: 99 passed
  Rust integration tests: 11 passed
  formatting, ESLint, Clippy, typecheck, Vite builds, and Tauri release no-bundle build passed
npm audit --audit-level=low
  network-enabled retry passed with 0 vulnerabilities
git diff --check
  passed
python3 .codex/hooks/post_increment_gate.py status
  complete, valid: true, PASS WITH ADVISORIES after finalization
```

Failed and resolved:

- The first planning `npm run format:check` found Prettier layout drift only in the two new Markdown files. The targeted `npx prettier --write docs/increments/04h-typed-approval-audit-adapter.md docs/plans/04h-typed-approval-audit-adapter.md` correction and exact rerun passed.
- The first sandboxed `npm audit --audit-level=low` failed because the sandbox could not resolve `registry.npmjs.org` or write user-level npm logs. The approved network-enabled retry passed with zero vulnerabilities. This was an execution-environment restriction, not a repository defect.
- The first finalization validated the report but could not write ignored state under the sandbox-protected `.codex` directory. The approved exact retry wrote the marker, and status reports `complete`, `valid: true`, and `PASS WITH ADVISORIES`.

Current failed checks: none.

Checks not run:

- No native launch or dialog interaction matrix was required because production native-dialog and shipping application behavior are unchanged.
- No Rust dependency audit was required because Cargo manifests and the lockfile are unchanged.

Manual verification pending: none. Increment 4H has no user-visible or operating-system permission behavior.

Material inspection commands:

```bash
git status --short --branch
git rev-parse --short HEAD
git log -5 --oneline --decorate
python3 .codex/hooks/post_increment_gate.py status
npm run typecheck
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked audit::
cargo test --manifest-path src-tauri/Cargo.toml --test approval_binding --locked
npm run format:check
npx prettier --write docs/increments/04h-typed-approval-audit-adapter.md docs/plans/04h-typed-approval-audit-adapter.md
npm run format:check
cargo fmt --manifest-path src-tauri/Cargo.toml
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked audit::approval::
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked approvals::decision_source::
cargo test --manifest-path src-tauri/Cargo.toml --test approval_audit_binding --locked
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
npm run verify
npm audit --audit-level=low
git diff --check
rg -n "AuditEvent|AuditLogger|InMemoryAuditLogger|NoopAuditLogger|ApprovalResolution|interaction_evidence" src-tauri/src src-tauri/tests
```

`sed`, `rg`, `git diff`, and `git status` were also used to read required repository memory and skills, inspect approval/audit code and tests, review every changed path, and check scope, secrets, generated output, databases, conflicts, and documentation consistency. No commit or push command ran.

### Review result and residual advisory

- Architecture, code-health, and security review found no blocking issue. The dedicated module is cohesive, uses no dependency, and does not couple to IPC, persistence, UI, or execution.
- The generic audit scaffold remains arbitrary-string, unbounded, disconnected, and non-production; 4H deliberately does not redesign it.
- The in-memory adapter is not durable. Its duplicate and capacity guarantees end with the adapter instance, so a future durable repository and authoritative coordinator require separate approval before execution wiring.
- A successful record or receipt remains non-authorizing and cannot replace exact run-liveness, durable audit, dispatch, or executor gates.

### Exact next task

Increment 4H is complete. Wait for the project owner to select and approve one bounded next plan. Do not infer or begin another increment, commit, push, or merge without explicit direction.

Ready-to-paste resume prompt:

```text
Use $session-start.

Resume from HANDOFF.md on uncommitted branch codex/phase4-increment-4h. Increment 4H typed approval-audit adapter is verified complete with a valid PASS WITH ADVISORIES post-increment marker. Reconcile the actual repository, then wait for the project owner to select and approve one bounded next plan. Do not infer or begin another increment, commit, push, or merge without explicit direction.
```

## Increment 4G publication state

Increment 4G adds one trusted repository-local Stop hook, one Python standard-library validator, a consolidated review skill and report schema, focused tests integrated into `npm run verify`, and the corresponding review/security/session/project-memory contracts. It changes no application source or behavior and adds no dependency, network access, transcript parsing, database, credential, permission, Tauri, IPC, provider, gateway, approval, audit, or execution path.

Fifteen focused tests, complete repository verification, and complete diff/security/scope review pass. Direct active-state evaluation emits the exact required continuation prompt, and `stop_hook_active: true` emits no repeated continuation. The project owner passed normal `/hooks` trust and live Stop confirmation. [`docs/reviews/2026-07-14-04g-post-increment-review.md`](docs/reviews/2026-07-14-04g-post-increment-review.md) records `PASS WITH ADVISORIES`; the advisory is the documented operator-controlled project-trust boundary. The ignored completion marker is complete and valid.

## Increment 4G verification classification

### Passed

```text
Python 3.12.1 availability
Codex CLI 0.144.2 hook capability inspection
python3 -m json.tool .codex/hooks.json
python3 -m py_compile .codex/hooks/post_increment_gate.py .codex/hooks/tests/test_post_increment_gate.py
python3 -m unittest discover -s .codex/hooks/tests -p 'test_*.py' -v
  15 passed
npm run test:hooks
  15 passed
npm run verify
  hook: 15 passed
  frontend: 124 passed
  Rust library: 92 passed
  Rust integration: 10 passed
  formatting, ESLint, Clippy with warnings denied, typecheck, Vite build, and Tauri release no-bundle build passed
python3 .codex/hooks/post_increment_gate.py status
  active before finalization; complete and valid after finalization
direct active-state Stop evaluation
  exact continuation prompt emitted
direct stop_hook_active evaluation
  no continuation output
```

### Failed and resolved

- The first direct unittest run failed because its dynamically loaded Python module was not registered in `sys.modules`; the test harness now registers it and the exact command passes with 15 tests.
- The first sandboxed `py_compile` attempt could not create ignored `__pycache__` output under the protected `.codex` tree. The exact approved retry passed; no tracked generated output remains.
- `codex doctor` reported pre-existing user-level state-database, shell-path, and restricted-network diagnostics. These are outside repository state and did not invalidate hook feature/schema inspection or repository verification.
- Security review found that symlinked `docs/reviews` or `.codex/state` parent directories could resolve outside the Git root. Both reads now require their resolved path to remain inside the root, and two focused regressions pass.
- The first high-confidence secret-scan wrapper used zsh's read-only `status` variable and failed before producing evidence. The corrected wrapper uses `rg_status` and passed with no key material found.
- The first finalization attempt passed report validation but could not write the ignored state under the sandbox-protected `.codex` tree. The approved exact retry outside that restriction completed the marker; status reports `complete`, `valid: true`, and `PASS WITH ADVISORIES`.

Also passed:

- exact 24-file tracked/untracked scope review;
- no application-source change check;
- high-confidence secret-material scan;
- ignored generated-state/cache and non-ignored generated/build/database/environment/certificate/log review;
- `git diff --check` and complete diff review; and
- code and security review with the parent-symlink issue resolved and no remaining blocking finding.

### Checks not run

- None of the required automated checks.

### Manual verification passed

- The project owner opened `/hooks`, reviewed and normally trusted the exact project hook, and confirmed the live active-state continuation behavior.

## Increment 4F publication baseline

Increment 4F - Cortexa product display rename is complete by explicit project-owner direction. Implementation commit `972a874` was pushed on `phase4/increment-4f`, fast-forward merged into `main`, and pushed to `origin/main`. D-026 preserves every compatibility identifier, and D-027 records the one-time skipped-gate sequencing exception that Increment 4G replaces for future work.

## Increment 4F exact files

See `docs/plans/04f-cortexa-product-display-rename.md` for the declared 43-file list. The implementation has not expanded beyond that list.

## Increment 4F verification classification

### Passed

```text
npm run build
  pre-edit baseline passed
npx vitest run src/App.test.tsx
  25 passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked app_info::tests
  1 passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked menu_bar::
  9 passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked approvals::
  16 passed
cargo test --manifest-path src-tauri/Cargo.toml --test smoke --locked
  1 passed
npm run format:check
  passed after the targeted correction below
npm run lint
  passed
npm run typecheck
  passed
npm run test:unit
  124 frontend and 92 Rust library tests passed
npm run test:integration
  92 Rust library and 10 Rust integration tests passed
npm run build
  post-edit build passed
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
  passed
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
  92 library and 10 integration tests passed; example target compiled
npm run tauri -- dev
  passed on retry; native application launched
```

Also passed:

- exact all-filesystem former-name search with no matches;
- exact 43-file scope comparison with no missing or unexpected path;
- preserved npm/Cargo package, binary, library crate, bundle ID, tray ID, event, command, storage, database, and path identifiers;
- `cargo metadata --manifest-path src-tauri/Cargo.toml --no-deps --format-version 1`, confirming binary `ai-agent-assistant` and library `ai_agent_assistant_lib`;
- no manifest/lockfile/capability/icon drift outside `src-tauri/Cargo.toml` display metadata;
- secret-pattern, generated-output, database, build-output, and `git diff --check` scans;
- complete tracked and untracked diff review;
- code review with no correctness, portability, test, or documentation finding; and
- security review with no trust-boundary, permission, credential, persistence, IPC, or execution finding.

### Failed and resolved

- The first `npm run format:check` found only `index.html` Prettier layout drift. `npx prettier --write index.html` fixed it, and the exact required check passed on rerun.
- The first `npm run tauri -- dev` failed because port 1420 was occupied by a stale standalone project Vite process. `lsof` and `ps` identified its `npm run dev` parent; `kill 12592` stopped it, the port-free check passed, and the exact launch command succeeded. TS-013 records the diagnosis.
- `pgrep -af "vite|tauri dev|ai-agent-assistant"` could not access the process list in this environment. The narrower approved `ps -p` checks supplied the required evidence.

### Check not run and explicitly deferred

- `$post-increment-gate` could not run because no matching skill or report workflow exists under `.agents/skills`. The project owner explicitly confirmed 4F complete and deferred skill creation to the next clean branch under D-027. No gate result is claimed.

### Manual verification

The retry launched unchanged executable `target/debug/ai-agent-assistant` and logged the fixed `Cortexa` startup prefix. The project owner confirmed the window title, standard application menu, right-side `Open Cortexa` and `Quit Cortexa` menu actions, `C` sidebar mark, sidebar name, Settings application value, generic composer placeholder, existing navigation/mock interaction, and absence of an operating-system permission prompt all passed. The fixed `Cortexa approval` title is covered by the passing approval test subset; the standalone dialog example was not manually rerun because it was outside the requested manual gate.

## Increment 4F command record

Repository state and the separately authorized workflow commit:

```bash
git status --short --branch
git diff -- AGENTS.md
git diff --cached --stat
git add AGENTS.md
git diff --cached --check
git commit -m "Add mandatory post-increment gate"
git show --stat --oneline --decorate --no-renames HEAD
```

Preflight and review used `git grep`, `rg`, `rg --files`, `sed`, `git diff`, `git status`, and `git ls-files --others --exclude-standard` to read every required document, inventory the former name, inspect source/configuration/tests, verify the exact file list, review all diffs, and classify preserved identifiers. Their material outcomes are recorded above and in `docs/increments/04f-cortexa-product-display-rename.md`.

Implementation and verification commands:

```bash
npm run build
npx vitest run src/App.test.tsx
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked app_info::tests
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked menu_bar::
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked approvals::
cargo test --manifest-path src-tauri/Cargo.toml --test smoke --locked
npm run format:check
npx prettier --write index.html
npm run format:check
npm run lint
npm run typecheck
npm run test:unit
npm run test:integration
npm run build
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
cargo metadata --manifest-path src-tauri/Cargo.toml --no-deps --format-version 1
git diff --check
npm run tauri -- dev
lsof -nP -iTCP:1420 -sTCP:LISTEN
pgrep -af "vite|tauri dev|ai-agent-assistant"
ps -p 12621 -o pid=,ppid=,lstart=,command=
ps -p 12592 -o pid=,ppid=,lstart=,command=
kill 12592
lsof -nP -iTCP:1420 -sTCP:LISTEN
npm run tauri -- dev
```

Publication commands:

```bash
git add -A
git diff --cached --check
git commit -m "Rename product display name to Cortexa"
git push -u origin phase4/increment-4f
git switch main
git merge --ff-only phase4/increment-4f
git push origin main
```

Result: implementation commit `972a874` is published on the feature branch and merged `main`.

## Increment 4E publication baseline

Increment 4E is complete within its approved eight-file runtime/test scope and was merged into `main` before Increment 4F.

Increment 4E completion evidence:

- the complete repository gate and focused automated checks pass;
- the JavaScript dependency audit passes;
- exact temporary RustSec scanning ran and its two pre-existing `quick-xml 0.39.4` findings have the scoped D-025 baseline disposition; and
- the project-owner native-dialog interaction matrix passed.

Implementation commit `b0a3036` was pushed on `phase4/increment-4e`, fast-forward merged into `main`, and pushed to `origin/main` at the project owner's request. This documentation-only publication-state closeout records that merged state. D-025 is accepted, no advisory ignore or dependency remediation change was added, and no later increment has started.

## Increment 4E completed implementation

- Removed public `ApprovalChoice` and the raw `ApprovalManager::decide` path.
- Added one owned, non-cloneable, non-serializable, redacted `ApprovalPresentation` issued only once by the pending manager.
- Added one sealed, non-cloneable, non-serializable `TrustedApprovalSourceOutcome` whose constructor is private to the macOS decision source.
- Moved one private `Arc` marker through manager -> presentation -> source outcome, compare it with `Arc::ptr_eq` before exact approval/run/gateway-request/function-call identity, and require that the one presentation was issued.
- Preserved one pending approval, 1,024 lifetime subjects, a 120-second monotonic TTL, no TTL extension on presentation, cancellation/expiry precedence, and non-evicting replay rejection.
- Added closed `RunTerminated`, `EditRequested`, `NativeNoDecision`, and `SourceFailed` cancellation reasons.
- Added precise native-dialog source, optional recognized-button, `NotEvaluated` authentication, and optional fixed source-failure evidence without title, actor-identity, run-liveness, audit, dispatch, or execution claims.
- Added a macOS-only direct `rfd` adapter with fixed `Cortexa approval` title and exact Reject/Approve/Edit custom-button order. Reject is first/default.
- Mapped only recognized custom results to Approve, Reject, or Edit. `Cancel` is `NativeNoDecision`; every unexpected result is a fixed `SourceFailed(UnexpectedDialogResult)`.
- Rendered every trusted preview fact before the final affected-title row, rejected the exact approved presentation-format set, allowed ordinary non-ASCII text, and capped the complete message at 1,024 Unicode scalar values.
- Added a standalone main-thread example using the public gateway -> schema -> policy -> approval -> native source -> manager-resolution path. It performs no action or persistence and prints only bounded approval identity plus a fixed terminal label.
- Added 16 approval unit tests and two approval-binding integration tests for exact layout, every denied code point, ordinary non-ASCII, message limit, every dialog result, cross-manager collision, identity mismatch, replay, late cancellation, expiry equality, Edit freshness, capacity, overflow, and redaction.

## Dependency review

Added exactly:

```toml
[target.'cfg(target_os = "macos")'.dependencies]
rfd = { version = "=0.17.2", default-features = false }
```

- `rfd 0.17.2` is MIT licensed.
- The lockfile diff adds only `rfd`; its native transitive crates were already resolved.
- The macOS target feature tree and duplicate graph were reviewed.
- The dependency's AppKit, raw-handle, and native `unsafe` internals are a new trust boundary, but application `unsafe` remains forbidden.
- The wrapper does not use `set_parent`, raw handles, dependency `Debug`, file-open/save APIs, a Tauri plugin, JavaScript API, command registration, or WebView capability.

## Exact files changed

Created runtime/test files:

```text
src-tauri/src/approvals/decision_source.rs
src-tauri/examples/native_approval_dialog.rs
```

Changed runtime/test files:

```text
src-tauri/src/approvals/mod.rs
src-tauri/src/approvals/types.rs
src-tauri/src/approvals/manager.rs
src-tauri/tests/approval_binding.rs
src-tauri/Cargo.toml
src-tauri/Cargo.lock
```

Changed closeout documentation:

```text
AGENTS.md
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
docs/increments/04e-trusted-approval-decision-source.md
docs/plans/04e-trusted-approval-decision-source.md
```

D-025 records the exact native-source boundary and the project-owner-approved scoped RustSec baseline exception. `TROUBLESHOOTING_LOG.md` remains unchanged because it was outside the approved closeout file list and no repository defect was diagnosed.

## Increment 4E verification classification

### Passed

```text
cargo check --manifest-path src-tauri/Cargo.toml
  passed
cargo check --manifest-path src-tauri/Cargo.toml --lib --locked
  passed
cargo tree --manifest-path src-tauri/Cargo.toml --target aarch64-apple-darwin --locked -p rfd -e features
  passed; resolved features reviewed
cargo tree --manifest-path src-tauri/Cargo.toml --target aarch64-apple-darwin --duplicates --locked
  passed; duplicates reviewed
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
  passed
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
  passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked approvals::
  16 passed; 0 failed
cargo test --manifest-path src-tauri/Cargo.toml --test approval_binding --locked
  2 passed; 0 failed
npm run verify
  passed
  frontend: 124 passed
  Rust library: 92 passed
  Rust integration: 10 passed
  TypeScript, Vite production builds, and Tauri release no-bundle build passed
npm audit --audit-level=low
  passed after network-enabled retry; 0 vulnerabilities
```

Also passed:

- exact dependency source and MIT-license review;
- lockfile review proving Increment 4E adds only `rfd`;
- stale `ApprovalChoice` and raw `.decide` Rust-symbol search;
- focused code review and security review with no 4E runtime-boundary finding; and
- final `npm run format:check` and `git diff --check`;
- secret-pattern scan of all 17 changed files with no matches;
- exact scope/status review with no generated files, databases, build output, or unrelated paths; and
- complete tracked and untracked diff review with no accidental scope expansion.

### Failed

Exact RustSec scan:

```text
CARGO_HOME=/private/tmp/ai-agent-assistant-cargo-audit-home \
  /private/tmp/ai-agent-assistant-cargo-audit/bin/cargo-audit \
  audit --file src-tauri/Cargo.lock
  exited 1
```

Reported:

- RUSTSEC-2026-0194 in `quick-xml 0.39.4`: quadratic duplicate-attribute checking.
- RUSTSEC-2026-0195 in `quick-xml 0.39.4`: unbounded namespace declarations in `NsReader`.
- 18 allowed warnings for unmaintained or unsound transitive crates.

Review established that these findings predate Increment 4E: the 4E lockfile diff adds only `rfd`, while `quick-xml` is reached through existing `plist 1.9.0 -> Tauri`. The existing `plist` source uses plain `quick_xml::Reader`, not `NsReader`, and does not iterate attributes, so the cited APIs appear unreachable through that path. This is a source-based reachability assessment, not a passing RustSec result. No ignore, Tauri/plist upgrade, or lockfile remediation was added.

D-025 records the project owner's scoped baseline exception for these two findings. It permits Increment 4E completion without calling the scanner passed, declaring the advisories fixed, or treating `quick-xml` as generally safe. Re-review is mandatory if the affected APIs become reachable or the dependency path changes.

The first sandboxed `npm audit --audit-level=low` attempt also failed on DNS resolution. The required network-enabled retry passed with zero vulnerabilities, so the final npm audit result is passed.

The first secret-scan invocation omitted the `rg --` option terminator and failed because the private-key pattern began with hyphens. The corrected command passed with no matches; this was command syntax, not a repository finding.

### Checks not run

- No packaged application gate was run; packaging is outside Increment 4E and `npm run verify` already passed the Tauri release no-bundle build.
- No shipping Tauri/WebView interaction test was run because Increment 4E adds no shipping-app wiring.
- No non-macOS cross-target runtime launch was run; `cfg(target_os = "macos")` and the fixed fallback compiled under all-target Clippy/build checks.
- No RustSec ignore or dependency remediation command was run because the approved disposition is a documented baseline exception with no repository suppression or dependency change.

### Manual verification

The command below compiled and opened the native prompt:

```bash
cargo run --manifest-path src-tauri/Cargo.toml --example native_approval_dialog --locked
```

Owner results are now recorded: Approve -> `approved`, Reject -> `rejected`, Edit -> `cancelled: edit requested`, and Return/default -> `rejected`. Escape had no effect and no window-close control was available. The final Edit run exited successfully.

The project owner confirmed the fixed window title, exact trusted-fields-first/title-last content, post-resolution terminal redaction, no action or persistence, no shipping-app change, and no operating-system permission prompt all passed.

## Increment 4E residual risks

- The visible native prompt cannot be programmatically closed after manager cancellation. Manager state invalidates the subject immediately and rejects every late outcome, but stale visible UI remains possible. Production orchestration is excluded.
- Native interaction proves only that this Rust-owned source returned a recognized button. It does not establish who interacted, LocalAuthentication, device-owner presence, active-run state, dispatch eligibility, or execution authority.
- The exact display deny set is intentionally not a complete Unicode confusable, normalization, font, or visual-width analysis.
- The two pre-existing RustSec advisories remain present under D-025's scoped reviewed baseline exception and require re-review if reachability or the dependency path changes.
- The target-platform manual gate is complete. Escape/close behavior is recorded above and did not create an approval path.

## Exact next task

Wait for explicit project-owner direction to commit, push, and merge reconstructed Increment 4I. Do not infer a later increment.

## Ready-to-paste resume prompt

```text
Use $session-start.

Resume from HANDOFF.md on verified, uncommitted `codex/phase4-increment-4i`. Preserve `codex/phase4-increment-4i-pre-fingerprint-fix` at `cf9d701`. Wait for explicit direction to commit, push, and fast-forward merge reconstructed 4I into updated `main`, then verify the corrected 04i marker remains valid after the deletion commit. Do not begin later work.
```
