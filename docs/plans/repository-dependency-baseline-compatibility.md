# Repository dependency baseline compatibility repair

Status: Verified complete; publication pending
Date: 2026-07-16
Gate ID: `repo-dependency-baseline-compatibility`
Branch: `codex/fix-dependency-baseline-compatibility`
Baseline: `4f23382`

## Goal

Restore deterministic clean installation and repository verification after
independently merged Dependabot pull requests produced an invalid JavaScript
manifest, an incompatible Vite/plugin graph, duplicate lockfile keys, and a
Rust dependency that does not compile on the repository's supported toolchain.

The repair must preserve all application behavior and every later compatible
dependency update already present on `main`.

## Baseline evidence

- `package.json` at `4f23382` omits the direct `vitest` entry and leaves a
  trailing comma after `vite`, so local `npm ci --ignore-scripts` fails with
  `EJSONPARSE`.
- The PR merge candidate for Meta Increment 7 reports `ERESOLVE` because
  `vite@8.1.4` is outside `@vitejs/plugin-react@4.7.0`'s declared peer range.
- `package-lock.json` contains duplicate direct `typescript-eslint` and `vite`
  keys created by overlapping dependency merges.
- `rusqlite@0.40.1` resolves `libsqlite3-sys@0.38.1`, whose build script uses
  unstable `cfg_select` and fails under installed Rust 1.90. The crate declares
  Rust 1.88 as its minimum.

## Exact implementation scope

```text
package.json
package-lock.json
src-tauri/Cargo.toml
src-tauri/Cargo.lock
```

The direct dependency repair is exact:

- Restore `vite` from `8.1.4` to the previously verified `7.3.5`.
- Restore the accidentally removed direct `vitest@3.2.6` entry.
- Deduplicate npm's Vite graph so the root plugin, Vitest, and Vite Node use one
  `vite@7.3.5` type identity.
- Restore `rusqlite` from `0.40.1` to the previously verified `0.37.0` and let
  Cargo remove only its incompatible transitive additions.

## Declared closeout scope

```text
CHANGELOG.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
TROUBLESHOOTING_LOG.md
docs/increments/repository-dependency-baseline-compatibility.md
docs/plans/repository-dependency-baseline-compatibility.md
docs/reviews/2026-07-16-repo-dependency-baseline-compatibility-post-increment-review.md
```

`DECISIONS.md` does not change. The repair restores previously accepted exact
pins and introduces no production dependency or new durable architecture
choice.

## Non-goals

- No application source, test, icon, workflow, hook, Tauri configuration,
  capability, CSP, permission, schema, identifier, or product behavior change.
- No upgrade of Vite, Vitest, the React plugin, rusqlite, Rust, or the declared
  minimum supported toolchain.
- No rollback of compatible Tauri, tempfile, jsdom, TypeScript ESLint, or
  GitHub Action updates already merged into `main`.
- No Meta Increment 7 icon change, Increment 4V source edit, or later increment.
- No bypass of required pull-request checks.

## Implementation

1. Begin the mandatory gate on clean `4f23382` before modifying manifests.
2. Correct the two direct manifests with the exact prior compatible pins.
3. Regenerate and deduplicate the npm lockfile.
4. Update only the rusqlite lockfile family to `0.37.0`.
5. Run clean installation, dependency-tree, type, full repository, JavaScript
   audit, secret, and exact Rust advisory-baseline checks.
6. Review all changed paths and finalize the post-increment report.

## Verification

```bash
npm ci
npm ls vite @vitejs/plugin-react vitest
npm run typecheck
cargo tree --manifest-path src-tauri/Cargo.toml -i rusqlite --locked
npm run verify
npm run security:scan
npm audit --audit-level=low
python3 scripts/cargo_audit_gate.py /private/tmp/cortexa-cargo-audit.json --cargo-audit-exit 1
git diff --check
python3 .codex/hooks/session_end_gate.py
python3 .codex/hooks/post_increment_gate.py status
```

No manual application verification is required because the repair changes only
dependency metadata and lock resolution. Full frontend, Rust, and Tauri build
verification remains mandatory.

## Risks

- A regenerated npm lock can retain a nested compatible-but-different Vite
  patch, causing duplicate TypeScript type identities. `npm ls` and typecheck
  must prove one deduplicated `7.3.5` graph.
- An unconstrained Cargo update could alter unrelated transitive packages. The
  lock update must be limited to the exact rusqlite downgrade family.
- Meta Increment 7 was verified on an older baseline. Its completion marker
  cannot be reused after rebasing onto the repaired `main`; its checks and gate
  must run again before PR #19 can merge.

## Rollback

Before commit, restore the four implementation files and declared closeout
records to `4f23382`. After publication, revert the single repair squash commit.
That rollback intentionally restores the broken dependency baseline and is
therefore only an emergency repository-state reversal, not a valid operating
state. No migration, data, credential, remote resource, or compatibility
identifier rollback exists.

## Exit criteria

- The diff contains only the four implementation paths and declared closeout
  paths.
- `npm ci`, one-Vite dependency proof, supported-Rust compilation, complete
  repository verification, and dependency/security checks pass.
- No Critical or High blocking finding remains.
- The post-increment result is `PASS` or `PASS WITH ADVISORIES` with a valid
  completion marker.
- Publication is separately approved under the repository Git naming policy.
