# Repository dependency baseline compatibility post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "python3 .codex/hooks/post_increment_gate.py begin --increment repo-dependency-baseline-compatibility",
    "npm install --package-lock-only --ignore-scripts",
    "cargo update --manifest-path src-tauri/Cargo.toml -p rusqlite --precise 0.37.0",
    "npm ci",
    "npm ls vite @vitejs/plugin-react vitest",
    "npm dedupe --ignore-scripts",
    "npm run typecheck",
    "cargo tree --manifest-path src-tauri/Cargo.toml -i rusqlite --locked",
    "npm run verify",
    "npm run security:scan",
    "npm audit --audit-level=low",
    "cargo install cargo-audit --version 0.22.2 --locked --root /private/tmp/cortexa-cargo-audit",
    "/private/tmp/cortexa-cargo-audit/bin/cargo-audit audit --file src-tauri/Cargo.lock --json > /private/tmp/cortexa-cargo-audit.json",
    "python3 scripts/cargo_audit_gate.py /private/tmp/cortexa-cargo-audit.json --cargo-audit-exit 1",
    "npx prettier --write CHANGELOG.md HANDOFF.md NEXT_STEPS.md PLANS.md PROJECT_STATUS.md TROUBLESHOOTING_LOG.md docs/increments/repository-dependency-baseline-compatibility.md docs/plans/repository-dependency-baseline-compatibility.md docs/reviews/2026-07-16-repo-dependency-baseline-compatibility-post-increment-review.md",
    "npm run docs:check",
    "npm run repository:check",
    "python3 scripts/repository_health.py generated",
    "git diff --check",
    "git status --short -- src src-tauri/src src-tauri/tests src-tauri/tauri.conf.json src-tauri/capabilities assets/branding .github",
    "python3 .codex/hooks/session_end_gate.py",
    "complete architecture, security, code-health, technical-debt, readiness, exact-scope, and complete-diff review",
    "python3 .codex/hooks/post_increment_gate.py finalize --increment repo-dependency-baseline-compatibility --report docs/reviews/2026-07-16-repo-dependency-baseline-compatibility-post-increment-review.md"
  ],
  "files_changed": [
    "CHANGELOG.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "TROUBLESHOOTING_LOG.md",
    "docs/increments/repository-dependency-baseline-compatibility.md",
    "docs/plans/repository-dependency-baseline-compatibility.md",
    "docs/reviews/2026-07-16-repo-dependency-baseline-compatibility-post-increment-review.md",
    "package-lock.json",
    "package.json",
    "src-tauri/Cargo.lock",
    "src-tauri/Cargo.toml"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Security",
      "effort": "Medium: isolate and verify dependency remediation before production release",
      "milestone": "Before production release",
      "risk": "The Rust lockfile retains the exact previously accepted vulnerabilities and warning advisories",
      "severity": "Medium",
      "summary": "Accepted RustSec findings remain unresolved"
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Technical debt",
      "effort": "Small to medium: require refreshed merge-candidate checks or grouped review for coupled dependency updates",
      "milestone": "Repository dependency governance",
      "risk": "Independently passing dependency branches can produce an invalid or incompatible combined main branch",
      "severity": "Medium",
      "summary": "Coupled dependency proposals lack combined compatibility enforcement"
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Roadmap",
      "effort": "Small: publish this repair, then rebase and rerun the existing Meta 7 verification matrix and gate",
      "milestone": "Meta Increment 7 publication",
      "risk": "The existing Meta 7 completion fingerprint and failed PR checks describe an older broken merge baseline",
      "severity": "Advisory",
      "summary": "Meta Increment 7 requires rebase and full reverification"
    }
  ],
  "increment_id": "repo-dependency-baseline-compatibility",
  "manual_verification": [
    {
      "check": "Review the complete thirteen-path diff for exact scope, lockfile integrity, architecture, security, code health, technical debt, secrets, generated output, personal data, and unrelated changes",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Launch and inspect the native Cortexa application",
      "required": false,
      "status": "Not run"
    }
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {
      "command": "npm ci",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm ls vite @vitejs/plugin-react vitest",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run typecheck",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo tree --manifest-path src-tauri/Cargo.toml -i rusqlite --locked",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run verify",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm audit --audit-level=low",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 scripts/cargo_audit_gate.py /private/tmp/cortexa-cargo-audit.json --cargo-audit-exit 1",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run docs:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run repository:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run security:scan",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 scripts/repository_health.py generated",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "git diff --check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "git status --short -- src src-tauri/src src-tauri/tests src-tauri/tauri.conf.json src-tauri/capabilities assets/branding .github",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    }
  ]
}
-->

Date: 2026-07-16
Increment: Repository dependency baseline compatibility repair
Branch: `codex/fix-dependency-baseline-compatibility`
Baseline: `4f23382`

## Executive summary

The repair restores a valid, reproducible dependency baseline after overlapping
Dependabot merges broke both clean npm installation and supported-Rust
compilation. It changes exactly four implementation paths and mandatory
closeout documentation. Application source and behavior remain unchanged.

The result is **PASS WITH ADVISORIES**. All required checks pass. Existing exact
RustSec advisories remain visible, dependency merge-candidate governance remains
separate debt, and Meta Increment 7 must be rebased and fully reverified after
this repair is published.

## Scope and boundaries

The implementation changes only `package.json`, `package-lock.json`,
`src-tauri/Cargo.toml`, and `src-tauri/Cargo.lock`. It restores direct
`vite@7.3.5`, `vitest@3.2.6`, and `rusqlite@0.37.0`, normalizes both lockfiles,
and retains the other compatible updates already merged into `main`.

No product source, test, schema, database migration, IPC, permission, CSP,
capability, credential, icon, identifier, workflow, hook, or runtime behavior
changes. No 4V work begins. Meta 7 remains isolated in PR #19.

## Verification results

Passed:

- `npm ci`: 275 packages installed; zero npm vulnerabilities.
- Dependency proof: the React plugin, Vitest, Vite Node, and Vitest mocker use
  one deduplicated `vite@7.3.5`.
- Supported-Rust proof: exact `rusqlite@0.37.0` resolves and compiles.
- `npm run verify`: formatting, repository health, ESLint, strict Clippy, 28
  hook tests, 16 repository-health tests, 124 frontend tests, 95 Rust library
  tests, 21 Rust integration tests, typecheck, Vite builds, and Tauri release
  no-bundle build.
- npm audit, secret scan, generated-output scan, documentation checks, exact
  scope, conflict inspection, and diff checks.
- Pinned cargo-audit plus the repository parser: only the exact accepted RustSec
  advisory baseline remains.

Failed during diagnosis and corrected before the completion run:

- Baseline `npm ci --ignore-scripts` failed because `package.json` was invalid.
- Baseline full verification failed because `libsqlite3-sys@0.38.1` used
  unstable `cfg_select` on Rust 1.90.
- The first repaired full verification reached typecheck and failed because the
  inherited npm lock retained nested Vite `7.3.6` identities. `npm dedupe`
  removed them; the subsequent clean install and full verification passed.

Not run:

- Native application walkthrough. It is non-required because the repair changes
  dependency metadata only and complete frontend, Rust, and Tauri builds pass.

Manual pending: none.

## Architecture findings

No application module, ownership boundary, trust boundary, runtime flow, or
storage architecture changes. Restoring the declared supported dependency
boundary reduces drift. Architecture review passes with no blocking finding.

## Security findings

No new dependency or authority is added. No credential, network, filesystem,
IPC, capability, permission, approval, policy, audit, SQLite, logging, or
execution path changes. npm audit and secret scanning pass. The existing exact
RustSec advisory baseline remains a Medium non-blocking finding and is not
silently ignored or expanded.

## Code-health findings

The repaired manifests parse, direct and locked versions agree, npm resolves one
Vite type identity, and Cargo resolves the exact compatible rusqlite version.
No source workaround, relaxed compiler setting, peer override, legacy-peer
installation, or warning suppression was introduced. Code-health review passes.

## Technical debt

- **Medium, dependency governance:** independently merged coupled dependency
  proposals can break their combined merge candidate. Risk is another invalid
  `main`; effort is Small to Medium; address in repository dependency
  governance. It does not block completion after this repair.
- **Medium, dependency security:** the exact accepted RustSec baseline remains.
  Effort is Medium and belongs before production release. It does not block this
  compatibility restoration.

## Roadmap findings

Next-increment readiness is **Blocked** until repository publication recovery is
complete. First publish this repair alone. Then rebase and fully reverify Meta 7
PR #19 on repaired `main`; its older marker cannot be reused. Increment 4V must
not begin during either publication step.

## Completion decision

**PASS WITH ADVISORIES**

All required automated checks and the complete-diff manual review pass. No
mandatory manual check is pending and no Critical or High blocking finding
remains.

## Next-increment readiness

**Blocked**

The exact next task is publication of this repair under the approved naming and
squash-PR policy. Meta 7 recovery follows as a separate verified step.

## Exact files changed

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
package-lock.json
package.json
src-tauri/Cargo.lock
src-tauri/Cargo.toml
```

## Exact commands executed

The machine manifest records the completion commands. Diagnostic attempts are
also recorded under Verification results with their actual failures. The final
clean-install, full-verification, audit, documentation, scope, conflict, and
gate commands all pass.
