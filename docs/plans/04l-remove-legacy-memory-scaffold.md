# Execution plan - Increment 4L remove legacy memory scaffold

Status: **Complete; verified and uncommitted**

Owner: Project maintainer

Last updated: 2026-07-15

## Goal and outcome

Delete the unused Phase 2 Rust memory module before future product memory or
persistence work can mistake its unbounded arbitrary-content records and
marker-list secret check for an approved trusted boundary.

The outcome removes the current Rust `memory` namespace entirely. The product's
future memory requirement remains intact and must be implemented later through a
separately approved typed, encrypted, user-controlled design.

## User-visible outcome

None. The Rust memory scaffold has no production caller, Tauri registration, IPC
path, frontend connection, or SQLite integration. The existing frontend mock
context disclosure and Memory placeholder remain unchanged.

## Existing behavior and constraints

- `MemoryRecordInput`, `MemoryRecordUpdate`, and `MemoryRecord` expose public,
  clonable, arbitrary `title`, `content`, and `source` strings.
- Records include only an ID, broad memory type, strings, and a version. They omit
  explicit opt-in, creation date, optional expiration, deletion evidence, export
  state, provenance identity, encryption state, and retention semantics required
  by the product brief.
- `InMemoryMemoryStore` has no record, field, or byte bound and clones full content
  from create, list, update, and delete operations.
- `contains_secret_like_content` lowercases input and checks only `password`,
  `api_key`, `apikey`, `access_token`, `refresh_token`, and `private key`.
  Passing that check does not establish that content is safe to store.
- Update validation occurs before mutation, but version overflow is checked after
  applying title/content changes. An artificial exhausted-version state could
  therefore mutate content while returning an error. There is no repository
  caller, so deletion is smaller than expanding scope to repair an unused API.
- Repository search finds no caller outside the memory module and its three
  embedded tests. Historical Increment 2A records remain accurate for their
  original checkpoint.
- The independent storage module uses typed bootstrap metadata, migrations,
  transactions, and release in-memory storage. It intentionally persists no user
  memory before Keychain-backed encryption and reviewed repositories exist.
- The product requires session, working, and long-term preference memory with
  opt-in, visibility, edit, delete, export, source, creation date, optional
  expiration, and sensitive-data restrictions. This scaffold does not satisfy
  that contract.

## Why deletion is the smallest increment

Repairing or adapting the scaffold would require decisions for memory ownership,
consent, provenance, timestamps, expiry, limits, encryption, Keychain key
management, SQLite schema and migrations, retention, deletion, export, UI, and
context-selection authority. Those are Phase 8 capabilities and cannot be safely
inferred in a cleanup increment.

Deleting four source paths removes the misleading unused surface while leaving
the verified storage and frontend mock boundaries unchanged. A future memory
design can start from the product requirements instead of inheriting an
incompatible Phase 2 mock contract.

## Exact source scope

Delete:

```text
src-tauri/src/memory/mod.rs
src-tauri/src/memory/store.rs
src-tauri/src/memory/types.rs
```

Change:

```text
src-tauri/src/lib.rs
```

The `lib.rs` change removes only:

```rust
pub mod memory;
```

Every other crate export must remain unchanged.

## Exact implementation closeout scope

In addition to the four source paths above, implementation closeout may change
only:

```text
AGENTS.md
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
docs/increments/04l-remove-legacy-memory-scaffold.md
docs/plans/04l-remove-legacy-memory-scaffold.md
docs/plans/README.md
docs/reviews/2026-07-15-04l-post-increment-review.md
```

`DECISIONS.md` is limited to proposed D-033 recording that future memory requires
a separately approved bounded, opt-in, provenance-aware, encrypted repository
contract rather than restoration of this arbitrary-content API. Stop and request
approval before changing any other file.

## Implementation steps

1. After project-owner approval, run:

   ```bash
   python3 .codex/hooks/post_increment_gate.py begin --increment 04l
   ```

2. Delete the three files under `src-tauri/src/memory/`.
3. Remove only `pub mod memory;` from `src-tauri/src/lib.rs`.
4. Confirm every legacy memory symbol and crate module export is absent from
   current Rust source and tests while historical records remain unchanged.
5. Run focused storage and startup persistence checks, Clippy, and the complete
   repository verification suite.
6. Review the complete diff against `SECURITY.md` and `CODE_REVIEW.md`, synchronize
   closeout documentation, append D-033, and complete the mandatory gate.

## Test plan

Focused verification must prove:

- the typed SQLite bootstrap storage tests still pass unchanged;
- public storage startup remains idempotent and persists only the bootstrap
  marker;
- no legacy memory type, trait, store, marker-list check, module export, or
  current caller remains;
- no source path outside the exact four-file plan changes; and
- complete repository verification passes after the three deleted embedded tests
  are removed from the expected Rust library count.

No new test file is justified: this increment removes a disconnected module, and
the preserved storage boundary already has unit and public integration coverage.

## Verification commands

Focused during implementation:

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked storage::
cargo test --manifest-path src-tauri/Cargo.toml --test startup_storage_smoke --locked
cargo test --manifest-path src-tauri/Cargo.toml --test storage_smoke --locked
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
rg -n "MemoryStore|InMemoryMemoryStore|MemoryResult|MemoryError|MemoryId|MemoryType|MemoryRecordInput|MemoryRecordUpdate|MemoryRecord|contains_secret_like_content|pub mod memory" src-tauri/src src-tauri/tests
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

Then run `$post-increment-gate`, create and finalize the exact 4L report, and
confirm status is complete and valid.

## Manual verification

None required. The removed scaffold has no production caller, Tauri registration,
IPC path, user-visible state, SQLite integration, network transport, or
operating-system interaction.

## Risks and mitigations

- **Public API removal:** an unsupported external crate consumer could import the
  public memory module even though repository search finds no internal caller.
  Mitigation: the application crate is not a supported external API; rollback
  restores exactly three files and one export.
- **Product-requirement confusion:** deleting the scaffold could be misread as
  deleting the future memory requirement. Mitigation: proposed D-033 and current
  planning records preserve the requirement and define the missing trust
  properties.
- **Storage regression:** an over-broad crate-root edit could remove or alter the
  verified storage module. Mitigation: `lib.rs` changes by one line only, storage
  files remain byte-for-byte unchanged, and focused unit/integration tests run.
- **Premature redesign:** cleanup could expand into migrations, encryption, UI,
  or context collection. Mitigation: every replacement capability is an explicit
  non-goal and requires separate approval.
- **Evidence-count drift:** removal intentionally deletes three memory unit tests.
  Mitigation: closeout records actual counts and does not describe the lower count
  as a regression.

## Explicit non-goals

- Replacement memory types, repository traits, in-memory stores, or secret
  detectors.
- Session, working, or long-term memory behavior; context selection; prompt
  construction; memory suggestions; or automatic memory creation.
- SQLite tables, migrations, repositories, transactions, encryption, database
  keys, Keychain, retention, deletion evidence, export, or synchronization.
- Memory UI, frontend state, Tauri commands/events, IPC, capabilities, CSP,
  packaging, entitlements, or operating-system permissions.
- Changes to storage, gateway, function validation, tools, policy, approval,
  audit, provider transport, coordinator, dispatch, or executor modules.
- Dependencies, manifests, lockfiles, database files, or generated artifacts.
- Rewriting historical Increment 2A records that describe the scaffold at their
  original checkpoint.

## Security and privacy considerations

The change removes an interface that retains and clones arbitrary personal
content while relying on an incomplete marker list as its only sensitive-data
check. It adds no replacement data flow, persistence, or authority. The
implementation must not modify SQLite, Keychain, frontend memory presentation,
context selection, logs, audit, capabilities, or permissions.

## Rollback or failure strategy

Restore `src-tauri/src/memory/mod.rs`, `src-tauri/src/memory/store.rs`, and
`src-tauri/src/memory/types.rs` exactly from baseline `5415444`, and restore
`pub mod memory;` in `src-tauri/src/lib.rs`. Revert only the 4L
planning/closeout documentation and D-033. No migration, persisted data,
dependency, capability, credential, or external state requires rollback.

If any current repository caller is discovered before deletion, stop and revise
the plan rather than widening it to migrate that caller automatically.

## Acceptance criteria

- The exact three memory files are deleted and only `pub mod memory;` is removed
  from `lib.rs`.
- Storage and all other source files remain unchanged.
- Current Rust source and tests contain no legacy memory scaffold symbol or
  module export.
- Existing storage unit and public startup/integration checks pass.
- The implementation adds no replacement memory abstraction or runtime caller.
- No runtime behavior, persistence, network, credential, dependency, migration,
  IPC, UI, capability, or permission changes.
- The exact four-file source scope and declared closeout scope are preserved.
- Focused checks, full repository verification, dependency audit, complete diff,
  code review, security review, documentation synchronization, and mandatory
  post-increment gate pass.
- D-033 is accepted and Increment 4L is marked complete only after project-owner
  approval and all gates pass.

## Planning baseline evidence

Passed on clean synchronized `main` at `5415444`:

```text
python3 .codex/hooks/post_increment_gate.py status
  Increment 04k complete, valid: true, PASS WITH ADVISORIES
npm run typecheck
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked memory::
  3 passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked storage::
  13 passed
cargo test --manifest-path src-tauri/Cargo.toml --test startup_storage_smoke --locked
  1 passed
cargo test --manifest-path src-tauri/Cargo.toml --test storage_smoke --locked
  1 passed
rg -n "MemoryStore|InMemoryMemoryStore|MemoryResult|MemoryError|MemoryId|MemoryType|MemoryRecordInput|MemoryRecordUpdate|MemoryRecord|contains_secret_like_content" src-tauri/src src-tauri/tests -g '!**/memory/**'
  no callers outside the proposed deleted module; required exit status 1
npm run format:check
  passed after planning edits
git diff --check
  passed after planning edits
```

At the planning checkpoint, changes were documentation-only and the previously
valid `04k` marker became stale because the planning documents changed. The
project owner then approved this exact source and closeout scope. The mandatory
`04l` state began before source edits.

## Implementation and verification result

The implementation deletes exactly the three memory files and removes only
`pub mod memory;` from `src-tauri/src/lib.rs`. Repository search finds no legacy
memory symbol or module export in current Rust source or tests. Storage source,
tests, manifests, lockfiles, frontend, Tauri configuration, permissions, and all
other trust boundaries remain unchanged.

Passed:

```text
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked storage::
  13 passed
cargo test --manifest-path src-tauri/Cargo.toml --test startup_storage_smoke --locked
  1 passed
cargo test --manifest-path src-tauri/Cargo.toml --test storage_smoke --locked
  1 passed
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
rg -n "MemoryStore|InMemoryMemoryStore|MemoryResult|MemoryError|MemoryId|MemoryType|MemoryRecordInput|MemoryRecordUpdate|MemoryRecord|contains_secret_like_content|pub mod memory" src-tauri/src src-tauri/tests
  no matches; required exit status 1
npm run verify
  17 hook, 124 frontend, 89 Rust library, and 11 Rust integration tests passed;
  formatting, lint, typecheck, builds, and Tauri release no-bundle passed
npm audit --audit-level=low
  0 vulnerabilities on the network-enabled retry
git diff --check
```

No manual verification applies. D-033 records the separately approved future
memory boundary. The consolidated result is `PASS WITH ADVISORIES`; the advisory
is the theoretical unsupported external consumer of the removed public module.

## Exact next task

Wait for explicit project-owner direction to commit, push, and merge verified
Increment 4L. Do not start later planning or implementation.
