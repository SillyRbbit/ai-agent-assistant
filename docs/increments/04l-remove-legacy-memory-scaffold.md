# Phase 4 Increment 4L - Remove legacy memory scaffold

Last updated: 2026-07-15

Status: **Verified complete; uncommitted**

## Goal

Remove the disconnected Phase 2 Rust memory scaffold before future memory,
context, or persistence work can adopt its unbounded arbitrary-content API or
short marker-list secret check as a trusted memory boundary.

## Planning result

- `memory::types` exposes clonable records with public arbitrary title, content,
  and source strings and omits opt-in evidence, creation time, expiration,
  visibility, export, encryption, and retention state required by the product.
- `memory::store` retains those strings in an unbounded in-memory map and treats a
  six-marker substring list as secret rejection. That check is neither a complete
  sensitive-data policy nor a safe basis for persistent memory.
- Repository search finds no source or integration caller outside the memory
  modules and their three embedded unit tests.
- The verified SQLite storage boundary is independent and intentionally stores
  only bootstrap metadata until reviewed encryption and repository contracts
  exist.
- The smallest coherent change removes the complete unused memory module and its
  crate export. It does not design replacement memory or alter storage.
- Future memory work must separately define opt-in, provenance, timestamps,
  expiration, sensitive-data classification, encryption, deletion, export, and
  authoritative repository semantics.

## Source scope

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

The `lib.rs` change removes only `pub mod memory;`. The exact design, risks,
non-goals, verification, closeout files, and rollback are recorded in
[`docs/plans/04l-remove-legacy-memory-scaffold.md`](../plans/04l-remove-legacy-memory-scaffold.md).

## Baseline evidence

Passed on clean synchronized `main` at `5415444`:

```text
python3 .codex/hooks/post_increment_gate.py status
  Increment 04k complete, valid: true, PASS WITH ADVISORIES
npm run typecheck
  passed
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

Toolchains and platform:

```text
Node.js v26.3.0
npm 11.16.0
Cargo 1.90.0
rustc 1.90.0
rustfmt 1.8.0-stable
Clippy 0.1.90
arm64 macOS 26.5.2
Xcode Command Line Tools: /Library/Developer/CommandLineTools
```

## Planning file scope

Created:

```text
docs/increments/04l-remove-legacy-memory-scaffold.md
docs/plans/04l-remove-legacy-memory-scaffold.md
```

Updated:

```text
AGENTS.md
CHANGELOG.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
docs/plans/README.md
```

No source, test, decision, security, troubleshooting, dependency, lockfile,
Tauri, frontend, storage, gateway, policy, approval, audit, coordinator, dispatch,
executor, IPC, capability, CSP, packaging, or permission file changes during
planning.

The `04k` marker was complete and valid on merged clean `main` before planning.
The nine documentation edits intentionally made that prior workspace fingerprint
stale. After project-owner approval, mandatory `04l` gate state began before
source edits.

Implementation closeout may additionally change only:

```text
DECISIONS.md
docs/reviews/2026-07-15-04l-post-increment-review.md
src-tauri/src/lib.rs
src-tauri/src/memory/mod.rs
src-tauri/src/memory/store.rs
src-tauri/src/memory/types.rs
```

## Completion gates

- [x] Required repository, product, architecture, security, review, decision,
      troubleshooting, workflow, 4K, and actual source state reconciled.
- [x] Clean synchronized Git baseline, valid 04k marker, toolchains, platform,
      caller search, and focused baseline checks recorded.
- [x] Exact four-file source scope, closeout scope, risks, non-goals,
      verification, and rollback defined.
- [x] Project owner approves the exact plan and source/closeout file lists.
- [x] Post-increment state begins before source edits.
- [x] Source implementation completes without scope expansion.
- [x] Focused and complete automated checks pass.
- [x] Complete diff, code, security, scope, secret, and generated-output reviews
      pass.
- [x] Documentation and D-033 are synchronized with actual evidence.
- [x] The post-increment report passes and the 4L marker is complete and valid.

## Completion evidence

- The three memory files are deleted and `src-tauri/src/lib.rs` loses only the
  memory export.
- The final legacy-symbol scan returns no matches with required exit status 1.
- Thirteen focused storage unit tests and both public storage smoke tests pass.
- Clippy passes with warnings denied.
- Complete `npm run verify` passes with 17 hook, 124 frontend, 89 Rust library,
  and 11 Rust integration tests plus formatting, lint, typecheck, frontend builds,
  and Tauri release no-bundle.
- Network-enabled `npm audit --audit-level=low` reports zero vulnerabilities.
- No manual verification is required because no production or user-visible path
  changed.
- Complete source, scope, conflict, secret, generated-output, architecture,
  security, code-health, and documentation reviews have no blocking finding.
- D-033 preserves the future bounded, opt-in, provenance-aware, encrypted,
  user-controlled memory requirement.
- The consolidated result is `PASS WITH ADVISORIES`; the only completion advisory
  is theoretical unsupported external use of the removed public module.

## Exact next task

Wait for explicit project-owner direction to commit, push, and merge verified
Increment 4L. Do not start later planning or implementation.
