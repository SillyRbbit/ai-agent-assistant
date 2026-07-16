# Repository dependency baseline compatibility repair

Date: 2026-07-16
Status: Verified complete; uncommitted and unpublished
Branch: `codex/fix-dependency-baseline-compatibility`
Baseline: `4f23382`
Gate ID: `repo-dependency-baseline-compatibility`

## Completed work

- Restored valid JSON and the direct `vitest@3.2.6` entry in `package.json`.
- Restored the compatible root `vite@7.3.5` pin required by
  `@vitejs/plugin-react@4.7.0`.
- Regenerated and deduplicated `package-lock.json`; the root plugin, Vitest,
  Vite Node, and Vitest mocker all resolve the same `vite@7.3.5` instance.
- Restored exact `rusqlite@0.37.0` and its compatible lockfile family while
  retaining the other dependency updates already merged into `main`.
- Changed no product source, tests, runtime configuration, permissions,
  identifiers, icons, database schema, or application behavior.

## Diagnosed baseline failures

Eight dependency and GitHub Action updates advanced `origin/main` from
`5281fac` to `4f23382` after the Meta Increment 7 branch was created. Overlapping
Dependabot merges removed Vitest from `package.json`, left invalid JSON, created
duplicate npm lock keys, selected Vite outside the React plugin peer range, and
selected a rusqlite transitive build script incompatible with supported Rust.

PR #19 therefore failed before exercising the icon change. The first broad
repair verification also identified and removed a nested Vite `7.3.6` lockfile
artifact that caused duplicate TypeScript plugin identities.

## Verification evidence

- `npm ci`: passed; 275 packages installed and 0 vulnerabilities reported.
- `npm ls vite @vitejs/plugin-react vitest`: passed; every consumer resolves
  one deduplicated `vite@7.3.5`.
- `npm run typecheck`: passed.
- `cargo tree --manifest-path src-tauri/Cargo.toml -i rusqlite --locked`:
  passed; exact `rusqlite@0.37.0` is active.
- `npm run verify`: passed formatting, repository health, ESLint, strict
  Clippy, 28 hook tests, 16 repository-health tests, 124 frontend tests, 95 Rust
  library tests, 21 Rust integration tests, typecheck, Vite builds, and the
  Tauri release no-bundle build.
- `npm run security:scan`: passed.
- `npm audit --audit-level=low`: passed with 0 vulnerabilities.
- Pinned `cargo-audit 0.22.2` returned the expected nonzero advisory result;
  `scripts/cargo_audit_gate.py` passed because only the exact accepted advisory
  baseline remains.
- No manual application check applies to this metadata-only repair.

## Reviews

Architecture: no product module or trust boundary changes. The repair restores
the declared toolchain and dependency boundary.

Security: no new package, network path, permission, capability, credential,
storage, IPC, or execution authority is introduced. Existing exact RustSec
advisories remain visible and accepted only through the existing parser.

Code health: the manifests and lockfiles are internally consistent. No test or
source workaround masks dependency incompatibility.

Technical debt: the dependency proposal/merge process allowed mutually
incompatible updates to land independently. This is an existing repository
process risk; future grouped compatibility review should be handled separately
without expanding this repair.

## Publication and next task

Publish this repair first through a separately approved squash PR. Then rebase
Meta Increment 7 PR #19 onto repaired `main`, rerun all Meta 7 checks and its
post-increment gate, force-update only that feature branch, and merge only after
hosted checks pass. Do not begin Increment 4V during recovery.
