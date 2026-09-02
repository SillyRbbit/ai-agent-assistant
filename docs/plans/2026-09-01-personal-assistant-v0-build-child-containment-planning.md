# Personal Assistant v0 build-child-containment planning

Status: Complete — `PASS WITH ADVISORIES`
Owner: Henry Dang
Last updated: 2026-09-01
Increment: `personal-assistant-v0-build-child-containment-planning`
Predecessor: completed D-101 account-directory boundary planning

## Goal

Define, without operating it, the least future trusted boundary that could
contain one fixed disposable build graph before any signing proof. The future
boundary must deny unsafe effects before they occur, retain authoritative
ownership of descendants even after session escape or reparenting, and emit
only D-100-minimized categorical evidence. It is not an implementation plan
for signing or a claim that a suitable target-Mac primitive is available.

## Scope

1. Add D-102's conceptual `BuildChildContainmentPolicyV1::Required` policy.
2. Define a private future `ContainedBuildAttemptV1` authority model and every
   input, identity, graph, effect, lifecycle, cleanup, and evidence invariant.
3. Define the D-100 closed source/contract review schema and its thirty-two
   predeclared check IDs; collect no operational evidence.
4. Document the blocked dependency order: primitive selection, controller,
   target-Mac synthetic proof, then a disposable no-sign build proof.
5. Preserve all historical Failed, Pending, and Not-run evidence unchanged.
6. Reconcile only the exact fifteen documentation paths in this plan.

## Explicit non-goals

- No build, npm/Cargo/Tauri/Vite command, shell, child process, process-group
  test, descendant probe, filesystem/network test, tracer, packet capture, or
  platform-effect observation.
- No containment primitive, sandbox profile, service manager, FFI, Rust,
  TypeScript, script, configuration, dependency, lockfile, capability, CSP,
  permission, runner, workflow, or product change.
- No Apple, Xcode, Keychain, certificate, private key, signing, credential,
  provider, model, account, identity, external system, or device action.
- No residual-risk acceptance, rerun, modification, or reclassification of
  D-097's failed privacy evidence or Pending Open Directory boundary.
- No P4 immutable signer binding, signing proof, V0-3, branch, commit, push,
  merge, pull request, release, or publication.

## Existing behavior and constraints

- D-099 admits P3 only as a future fail-closed executable-build-child
  containment/observation prerequisite; D-100 requires closed,
  source-minimized, non-authorizing evidence; D-101 keeps the account-directory
  boundary independent and Blocked.
- The ordinary repository graph reaches npm lifecycle hooks, Cargo build
  scripts, Vite/Tauri descendants, compilers, and linkers. Its output routing
  and process groups do not establish containment of that graph.
- npm currently permits the reviewed `esbuild` and `fsevents` lifecycle hooks;
  the Tauri build configuration calls `npm run build`; Cargo has its own build
  graph. These facts identify future review scope only and authorize no run.
- Existing production code has no complete build-child containment controller.
  Historical `sandbox-exec` research cannot be used alone because it does not
  prove membership, detached-child shutdown/reaping, or all required effects.
- The current P3 increment is documentation-only; every operational predicate
  remains Not run and no target-Mac primitive is selected.

## Current-state evidence

- Baseline `HEAD` and `origin/main` are both
  `b26e7b8f243b18533238654934704a6166c7f375`, ahead/behind `0/0`, with a clean
  working tree before this gate.
- `git fsck --full --no-dangling` passed. The ignored P2 gate is complete,
  valid, and `PASS WITH ADVISORIES`.
- Repository-pinned toolchains are Node `26.3.0`, npm `11.16.0`, and
  Cargo/Rust `1.90.0`; baseline `npm run docs:check` passed.
- No operational P3 command has run. No host, account, path, process output,
  build output, certificate, or external-system data is retained as evidence.

## Files expected to change

Exactly:

1. `ARCHITECTURE.md`
2. `CHANGELOG.md`
3. `DECISIONS.md`
4. `HANDOFF.md`
5. `NEXT_STEPS.md`
6. `PLANS.md`
7. `PROJECT_STATUS.md`
8. `ROADMAP.md`
9. `SECURITY.md`
10. `SECURITY_CHECKLIST.md`
11. `TESTING_GUIDE.md`
12. `TROUBLESHOOTING_LOG.md`
13. `docs/increments/personal-assistant-v0-build-child-containment-planning.md`
14. this plan
15. `docs/reviews/2026-09-01-personal-assistant-v0-build-child-containment-planning-post-increment-review.md`

## Affected components

| Component                             | Effect                                                                  |
| ------------------------------------- | ----------------------------------------------------------------------- |
| Repository governance                 | Documents D-102's future prerequisite and fail-closed stop rule.        |
| Build-child security planning         | Defines complete-graph/effect/cleanup predicates; no controller exists. |
| Privacy evidence                      | Reuses D-100 closed categorical records without collecting any.         |
| Product runtime, IPC, and build graph | None.                                                                   |
| Target Mac and external systems       | None.                                                                   |

## Interfaces and invariants

### Conceptual future policy

```text
BuildChildContainmentPolicyV1::Required
```

This is documentation-only, not an implemented type, serialized DTO, command,
or authorization. A future private `ContainedBuildAttemptV1` would be created
only by application-owned trusted code after a separately approved plan. It is
opaque, non-serializable, non-cloneable, attempt-bound, and cannot cross IPC,
persistence, logs, evidence, tests, or a caller-selected boundary.

- No caller, model, WebView, environment, command output, task, run, profile,
  runtime, workflow, or user input may choose an executable, argument, working
  directory, environment, graph, retry, network policy, or cleanup target.
- The full transitive build graph is frozen to one reviewed commit, lockfiles,
  and repository-pinned toolchains. Launches are shell-free fixed absolute
  executables with literal reviewed arguments, no `PATH` lookup, no generic
  runner, and no fallback or retry.
- The future boundary must close inherited environment and file descriptors and
  deny ambient account, credential, Keychain, clipboard, socket, filesystem,
  and IPC authority. It must constrain both reads and writes; a disposable
  output root alone is insufficient.
- Outside-root writes and undeclared network effects must be denied before the
  effect. Post-hoc scans, clean Git status, caches, or logs can diagnose a
  failure but cannot convert it into passing containment evidence.
- Authoritative descendant membership must survive fork, exec, reparenting,
  `setsid`, and `setpgid`. Process-group signaling is only an adjunct, never
  the membership proof. Deadline and explicit cancellation are terminal.
- On every terminal path, the controller must terminate the owned graph, reap
  all direct children, close pipes, await race-free quiescence, and prevent late
  effects/events. An escaped, unreaped, ambiguous, or still-live descendant is
  a boundary failure.
- Cleanup targets must be descriptor-bound and non-following. Cleanup failure
  quarantines private ownership, blocks retry/replacement, and cannot silently
  expand deletion authority or discard unresolved evidence.
- Evidence uses only D-100 `evidence_privacy_v1`: a fixed check ID, allowed
  categorical outcome, and fixed predicate. It contains no raw output, path,
  process/account/host identity, command line, trace, packet, secret, or free
  text. `boundary_failed` is mandatory and non-authorizing.

### Future closed review table

Each source row permits `observed`, `not_observed`, `not_run`, or
`boundary_failed`. Each contract row permits `documented`,
`contract_unproven`, `not_run`, or `boundary_failed`. All 32 IDs are lowercase
ASCII, unique, bounded below D-100's 64-byte ID limit, and paired to one
specific direct predicate. Every outcome is **Not run** in this increment.

| Family   | `check_id`                              | Direct predicate                                                                     |
| -------- | --------------------------------------- | ------------------------------------------------------------------------------------ |
| Source   | `caller_selected_process_source_review` | Complete commit-bound scope accepts caller-selected process authority.               |
| Source   | `shell_launch_source_review`            | Complete scope invokes a shell or generic command interpreter.                       |
| Source   | `inherited_environment_source_review`   | Complete scope inherits undeclared environment authority.                            |
| Source   | `unreviewed_spawn_source_review`        | Complete scope can spawn an executable outside the fixed graph.                      |
| Source   | `retry_fallback_source_review`          | Complete scope has unreviewed retry, fallback, or replacement launch.                |
| Source   | `raw_child_output_source_review`        | Complete scope exposes raw child output outside the trusted boundary.                |
| Source   | `deadline_enforcement_source_review`    | Complete scope lacks fixed terminal deadline enforcement.                            |
| Source   | `cancellation_terminal_source_review`   | Complete scope lacks terminal explicit cancellation handling.                        |
| Source   | `late_effect_rejection_source_review`   | Complete scope accepts a post-terminal effect or event.                              |
| Source   | `cleanup_ownership_source_review`       | Complete scope releases unresolved cleanup ownership.                                |
| Contract | `containment_primitive_contract`        | Authoritative contract documents pre-effect control for the selected primitive.      |
| Contract | `fixed_build_graph_contract`            | Authoritative contract binds one fixed reviewed transitive graph.                    |
| Contract | `executable_identity_contract`          | Authoritative contract binds fixed executable identities and literal arguments.      |
| Contract | `working_directory_binding_contract`    | Authoritative contract binds one fixed non-caller working directory.                 |
| Contract | `descriptor_inheritance_contract`       | Authoritative contract closes undesired inherited descriptors.                       |
| Contract | `filesystem_read_scope_contract`        | Authoritative contract denies ambient host-data reads.                               |
| Contract | `filesystem_root_contract`              | Authoritative contract confines intended filesystem roots.                           |
| Contract | `outside_root_write_contract`           | Authoritative contract denies outside-root writes before effect.                     |
| Contract | `declared_network_scope_contract`       | Authoritative contract binds the declared network scope.                             |
| Contract | `undeclared_network_contract`           | Authoritative contract denies undeclared network effects before effect.              |
| Contract | `descendant_membership_contract`        | Authoritative contract retains descendants across fork/exec/reparent/session escape. |
| Contract | `group_shutdown_contract`               | Authoritative contract supports owned-graph terminal shutdown.                       |
| Contract | `direct_child_reap_contract`            | Authoritative contract requires direct-child reaping.                                |
| Contract | `terminal_quiescence_contract`          | Authoritative contract establishes race-free terminal quiescence.                    |
| Contract | `pipe_closure_contract`                 | Authoritative contract closes all child communication pipes.                         |
| Contract | `process_metadata_effect_contract`      | Authoritative contract bounds process-metadata exposure.                             |
| Contract | `cleanup_binding_contract`              | Authoritative contract binds cleanup to non-following descriptors.                   |
| Contract | `cleanup_failure_contract`              | Authoritative contract requires quarantine after cleanup failure.                    |
| Contract | `platform_cache_effect_contract`        | Authoritative contract bounds platform cache effects.                                |
| Contract | `platform_log_effect_contract`          | Authoritative contract bounds platform log effects.                                  |
| Contract | `platform_socket_effect_contract`       | Authoritative contract bounds platform socket effects.                               |
| Contract | `platform_network_effect_contract`      | Authoritative contract bounds platform network effects.                              |

An incomplete source inventory produces `boundary_failed`, never
`not_observed`. An absent, ambiguous, or unsupported contract produces
`contract_unproven`, never a passing claim. Policy state keeps successors
Blocked; no evidence record grants authority.

## Implementation milestones

- [x] Confirm clean synchronized baseline, toolchains, prior gate, and P3
      current-state constraints.
- [x] Begin the separately approved documentation-only P3 gate.
- [x] Add D-102, this policy, the closed review table, and reconciled records.
- [x] Run documentation-tier validation and independent reviews.
- [x] Record the post-increment review and finalize the passing documentation
      result.

## Security and privacy considerations

| Threat                                           | Required control                                                              |
| ------------------------------------------------ | ----------------------------------------------------------------------------- |
| Output routing is mistaken for containment       | Require pre-effect read/write/network control and complete membership.        |
| Child escapes process group/session              | Treat group signaling as adjunct; prove membership across escape/reparenting. |
| Ambient credentials or host data are read        | Close environment/descriptors and require explicit read scope.                |
| Unsafe effect is discovered only after it occurs | Deny outside-root/undeclared-network effect before operation.                 |
| Cleanup deletes or abandons the wrong state      | Use descriptor-bound non-following ownership and quarantine on failure.       |
| Raw build/process data leaks into governance     | D-100 categorical fixed-predicate evidence only.                              |
| Historical failure is silently healed            | Preserve D-097/D-098 and all Failed/Pending/Not-run facts.                    |
| A policy document authorizes execution           | Keep P3 operational and every successor Blocked.                              |

## Test plan

- Inspect the full diff and verify exactly the declared fifteen paths changed.
- Review every 32-row ID and allowed outcome against D-100: unique IDs,
  bounded lowercase tokens, one predicate per record, `boundary_failed`, and no
  raw/free-text payload design.
- Verify all operational P3, Apple, Xcode, Keychain, signing, provider,
  process, build, network, and filesystem checks are recorded Not run.
- Verify P4/signing/V0-3 remain Blocked and D-097 failed/FAIL/Blocked,
  historical privacy failure, Pending Open Directory finding, and no completion
  marker remain unchanged.
- Run `npm run docs:check`, `npm run repository:check`, `npm run security:scan`,
  `git diff --check`, and a protected-path diff check. Do not run `npm run
verify`, a build, or a probe for this documentation-only increment.

## Manual gates and dependencies

| Gate                                | Status  | Required next evidence                                                          |
| ----------------------------------- | ------- | ------------------------------------------------------------------------------- |
| P3-1 policy documentation           | Active  | This plan's checks and reviews.                                                 |
| P3-2 primitive selection            | Blocked | Separate owner-approved plan; supported no-new-dependency pre-effect primitive. |
| P3-3 controller                     | Blocked | P3-2 decision, fixed graph, closed interfaces, focused tests/review.            |
| P3-4 target-Mac synthetic proof     | Blocked | P3-3 passing controller and explicit safe operational approval.                 |
| P3-5 disposable no-sign build proof | Blocked | P3-4 evidence plus separate explicit approval.                                  |
| P4 signer binding/signing/V0-3      | Blocked | All P3 gates plus their independent approved prerequisites.                     |

## Rollback

No operational state is created. If documentation validation or review fails,
retain the active gate's truthful failure disposition; do not change historical
evidence, run a substitute probe, or start a successor. A later owner-approved
documentation amendment may supersede D-102 without treating an unselected
primitive as accepted.

## Stop conditions

Stop without finalizing as passing if an exact declared path cannot be
reconciled, a historical result would be rewritten, any operational command is
needed to support a claim, a primitive must be selected, raw evidence would be
needed, validation fails, or a review finds a material inconsistency. Do not
commit, push, merge, begin P3-2, or perform any Apple/signing/external action.

## Decisions, discoveries, and progress

- D-102 is additive and does not supersede D-099, D-100, D-101, D-097, or
  D-098.
- Output roots, caches, logs, post-hoc scans, clean Git status, process groups,
  and `sandbox-exec` alone do not establish the future containment claim.
- The exact 32 planned IDs are unique; the longest ID is below 64 bytes and the
  longest planned canonical record is below 256 bytes. This is a design review,
  not emitted evidence.
- No operational P3 evidence has been collected.

## Final result

`PASS WITH ADVISORIES`. `npm run docs:check`, `npm run repository:check`,
`npm run security:scan`, `git diff --check`, the protected-path diff check, and
the session inventory passed. Architecture, security, code-health, and
documentation reviews found no completion-blocking defect. The sole advisory is
the documented absence of a selected supported no-new-dependency target-Mac
containment primitive and its controller/proof; it blocks P3-2 but not this
documentation closeout. Every build/process/filesystem/network/Apple/signing/
provider/product check remained Not run by approved scope.
