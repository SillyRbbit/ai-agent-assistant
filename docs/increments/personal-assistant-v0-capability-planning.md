# Personal Assistant v0 capability planning increment

Status: Verified complete with advisories
Owner: Henry Dang
Date: 2026-08-28
Baseline: `41ff7141007c8c0a684a5e2434ecf81dd4707418`
Program: [2026-08-28-personal-assistant-v0-program.md](../plans/2026-08-28-personal-assistant-v0-program.md)

## Goal

Define the smallest genuinely usable text-only Personal Assistant v0, separate
synthetic, real-content, and later product milestones, and produce a
dependency-ordered set of independently approvable plans without changing
executable behavior or external state.

## Scope

- Record D-094's exact Personal Assistant-only, empty-tool v0 boundary.
- Reconcile product, architecture, security, roadmap, handoff, and plan memory.
- Produce exact V0-1 through V0-14 plans with limits, files, threats, tests,
  manual gates, dependencies, rollback, and stop conditions.
- Compare an unauthorized local-model lane only because it could remove the
  remote identity/gateway/ZDR chain for a private prototype.
- Run the documentation-tier completion gate.

## Non-goals

No product/test/source code, dependency, manifest, lockfile, capability, CSP,
permission, credential, signing state, Keychain item, provider/gateway account,
cloud resource, external transmission, Tauri IPC, persistence, memory, tool,
device effect, commit, push, PR, merge, release, or publication.

## Baseline evidence

- Clean synchronized `main`; `HEAD` and `origin/main` equal
  `41ff7141007c8c0a684a5e2434ecf81dd4707418` after fetch.
- Git connectivity is healthy; only unreachable dangling objects were observed.
- Pinned toolchains: Node 26.3.0, npm 11.16.0, Rust/Cargo 1.90.0.
- Baseline `npm run docs:check` passed.
- Current Native request has two tools; the lower gateway validator can reject
  all tools, but no sealed empty-tool request exists.
- No direct Rust HTTPS client, provider adapter, live coordinator, Personal
  Assistant Tauri host, or live conversation reducer exists.
- D-076/TS-017, external auth/resources, provider evidence, D-061 ZDR, and
  real-content auth remain live blockers.

## Planned result

Only V0-1, the transport-free sealed empty-tool turn plus minimal identity/start
host, may be marked Ready after this planning gate. V0-2 through V0-14 remain
Blocked. The program deliberately separates local lifecycle, signed identity,
JWT/JWKS source, no-traffic provisioning, the HTTPS dependency decision,
fixed-origin transport, real credential handoff, auth-only traffic, fake
provider mapping, disclosure-bound Tauri presentation, no-traffic provider
state, live synthetic traffic, and real-content admission. Milestone 3 receives
no ExecPlan because action-taking and production scope are neither selected nor
bounded.

## Validation

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
python3 .codex/hooks/session_end_gate.py
python3 .codex/hooks/post_increment_gate.py status
```

Target-Mac application/UI, signing, Keychain, gateway, provider, network,
credential, kill-switch, and rollback checks are `Not run` because this
increment changes documentation only and creates no reachable boundary.

## Acceptance criteria

- [x] First usable v0 and three milestone truths are explicit.
- [x] Every requested security/transport/IPC/UI/operations blocker is resolved
      in design or identified as blocking with an owner and stop condition.
- [x] Every planned increment is small, ordered, exact-file-scoped, and
      independently approval-bound.
- [x] At most one plan is Ready.
- [x] The local alternative is not authorized and its decision impact is exact.
- [x] Documentation, repository, security, whitespace, session-end, and
      completion-marker checks pass.

## Result

Complete. Documentation, repository, secret, whitespace, and session-end checks
passed; independent architecture/readiness, frontend/F-12, and security reviews
found no completion blocker. The completion marker validates. Target-Mac,
signing, credential, gateway, provider, network, Tauri/UI, kill-switch, and
rollback checks were Not run because this increment changed documentation only.
V0-1 alone is Ready for separate owner approval; V0-2 through V0-14 remain
Blocked.
