# Personal Assistant v0 App Sandbox containment re-review

Status: Complete (`PASS WITH ADVISORIES`)
Owner: Henry Dang
Last updated: 2026-09-02
Increment: `personal-assistant-v0-app-sandbox-containment-rereview`
Predecessor: completed D-104 containment bootstrap trust decision

## Goal

Re-review exactly one versioned documentation candidate,
`app_sandbox_build_helper_plus_libsystem_supervision_v2`, against every D-102
contract after D-104 separated an identity-free disposable App Sandbox
activation seal from P4's later Developer ID signer-binding proof. The review
may select the candidate only for a later separately approved P3-3 planning
increment or conclude that no eligible candidate exists.

This is an authoritative-public-source review only. It adds no containment
primitive, controller, product behavior, operational proof, entitlement,
signature, target-Mac action, or external-system state.

## User-visible outcome

None. The only output is a closed repository-governance decision and its
source-minimized documentation evidence.

## Scope

1. Freeze exactly
   `app_sandbox_build_helper_plus_libsystem_supervision_v2`. D-103's historical
   `..._v1` candidate and negative result remain immutable.
2. Change only the D-104 assumption: the conceptual
   `sandbox_activation_adhoc_v1` seal is not P4's
   `product_signer_binding_v1` identity proof.
3. Re-review every one of D-102's 22 contract IDs from a fixed current official
   Apple public-source register. No other candidate may enter this attempt.
4. Keep all ten P3-3 implementation-source checks `not_run` because no
   controller source exists.
5. Record one additive D-105 decision: either the candidate is eligible only
   for later P3-3 planning, or no eligible candidate exists after the bounded
   re-review.
6. Reconcile only the exact fifteen documentation paths declared below.

## Explicit non-goals

- No product or test source, dependency, lockfile, configuration, capability,
  CSP, permission, entitlement file, helper, controller, build artifact,
  process, target-Mac evidence, or external resource.
- No `codesign`, `security`, `xcodebuild`, `xcrun`, `sandbox-exec`, compiler,
  npm lifecycle, Cargo, Vite, Tauri, helper, child-process, target-Mac/product
  filesystem or network effect, tracing, packet-capture, or probe operation.
- No Apple account, Xcode, Keychain, certificate, private key, Team ID,
  provisioning profile, Developer ID, authentication, credential, provider,
  model, product, release, distribution, or state-changing external action.
- No root or administrator authority, daemon, service, system extension, XPC
  substitution, VM/container, guest image, persistent global state, new
  dependency, broad entitlement, residual-risk waiver, or compensating-control
  path.
- No caller-, model-, WebView-, environment-, target-, or source-selected
  candidate, executable, graph, entitlement, path, policy, retry, cleanup
  target, or result.
- No rewrite or downgrade of D-097 through D-104, their reports or digests, the
  Failed privacy finding, Pending Open Directory boundary, Not-run signing
  evidence, or D-097's absent completion marker.
- No P3-3 through P3-5, P4, signing, V0-3, commit, push, merge, pull request,
  release, or publication. The already authorized branch creation and gate
  begin are the only repository-control actions in this start step.

## Existing behavior and constraints

- D-102 requires a fixed reviewed graph, shell-free fixed launches, closed
  environment and descriptors, pre-effect filesystem/network denial,
  complete descendant membership across fork/exec/detachment/reparenting,
  terminal shutdown/reaping/quiescence, descriptor-bound cleanup, quarantine,
  and D-100-minimized evidence.
- D-103 selected no candidate in its frozen set. Its `..._v1` App Sandbox
  candidate failed both the former independent signing/entitlement
  classification and separate unresolved containment contracts.
- D-104 changes only the first classification for a future static review. It
  neither selects a candidate nor proves helper bootstrap provenance,
  pre-effect control, graph ownership, termination, reaping, or quiescence.
- The repository has no P3-3 controller source. Output routing, process groups,
  clean Git state, and post-hoc scans are not containment.
- Public documentation may support only its stated narrow contract. Archived
  manual pages cannot establish current macOS 14+ availability, and source
  silence cannot establish a denial guarantee.

## Current-state evidence

- Clean synchronized `main` and live `origin/main` were both
  `e1b2ff5c06a4c5bc6ad7fc19b63968668fff4923`, ahead/behind `0/0`, before branch
  creation.
- `git fsck --full --no-dangling` passed.
- The D-104 predecessor gate reported `complete`, `valid: true`, and
  `PASS WITH ADVISORIES`.
- Repository-pinned toolchains are Node `26.3.0`, npm `11.16.0`, and
  Rust/Cargo `1.90.0`.
- The owner explicitly authorized branch
  `codex/p3-app-sandbox-containment-rereview`, recording this exact plan, and
  beginning only this documentation increment. The exact begin command passed.
- No operational or state-changing external action ran.

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
13. `docs/increments/personal-assistant-v0-app-sandbox-containment-rereview.md`
14. this plan
15. `docs/reviews/2026-09-02-personal-assistant-v0-app-sandbox-containment-rereview-post-increment-review.md`

## Affected components

| Component                         | Effect                                                         |
| --------------------------------- | -------------------------------------------------------------- |
| Repository governance             | Records a bounded D-105 candidate re-review decision.          |
| Future containment architecture   | Classifies one versioned candidate from public contracts only. |
| Product runtime, IPC, build graph | None.                                                          |
| Target Mac and external systems   | None; all operational checks remain Not run.                   |

## Interfaces and invariants

### Frozen candidate

```text
app_sandbox_build_helper_plus_libsystem_supervision_v2
```

The candidate composition is closed to:

- one application-owned host conceptually carrying only
  `com.apple.security.app-sandbox=true`, with
  `com.apple.security.inherit` absent;
- one directly spawned helper conceptually carrying exactly
  `com.apple.security.app-sandbox=true` and
  `com.apple.security.inherit=true`;
- the D-104 conceptual `sandbox_activation_adhoc_v1` seal;
- `posix_spawn` launch, `waitpid` direct-child wait/reap, `kqueue`
  `EVFILT_PROC` observation for a known PID, and `setpgid` plus process-group
  signaling as supervision adjuncts only.

These are documentation labels, not implemented types, files, commands, or
authority. All other entitlements are absent, including network client/server,
arbitrary or user-selected filesystem, executable-file, temporary exception,
automation, Keychain, app group, Mach lookup, device, contact, calendar,
camera, and microphone entitlements.

`sandbox_activation_adhoc_v1` contains no signing identity, certificate,
private key, Team ID, Keychain, provisioning, authentication, distribution, or
product authority. It is categorically inadmissible as P4 identity, custody,
provenance, signing-success, release, or notarization evidence.

### Closed final decision

The static review produces exactly one of two documentation decisions:

1. `candidate_eligible_for_p3_3_planning`: every contract row is
   `documented` from authoritative current public sources, with no prohibited
   authority or bootstrap circularity; or
2. `no_eligible_candidate_after_d104_rereview`: one or more rows are
   `contract_unproven`, `not_run`, or `boundary_failed`.

The first result only permits proposing a separately approved P3-3 plan. It is
not operational containment evidence. There is no partial eligibility,
weighted score, fallback, other-candidate branch, retry, residual-risk
acceptance, or automatic successor.

### Complete D-102 contract table

Every ID must appear exactly once with only `documented`,
`contract_unproven`, `not_run`, or `boundary_failed`:

| Contract ID                          | Direct question                                                                                                |
| ------------------------------------ | -------------------------------------------------------------------------------------------------------------- |
| `containment_primitive_contract`     | Does a supported current contract establish the exact candidate boundary?                                      |
| `fixed_build_graph_contract`         | Can one commit/lockfile/toolchain-bound transitive graph exist inside the boundary without circular bootstrap? |
| `executable_identity_contract`       | Are every executable and literal argument fixed without search or caller choice?                               |
| `working_directory_binding_contract` | Is one application-owned working directory bound without ambient or caller authority?                          |
| `descriptor_inheritance_contract`    | Can all undeclared inherited descriptors be closed before launch?                                              |
| `filesystem_read_scope_contract`     | Are ambient host-data reads denied before effect?                                                              |
| `filesystem_root_contract`           | Are intended read/write roots exact and non-broad?                                                             |
| `outside_root_write_contract`        | Are outside-root writes denied before effect?                                                                  |
| `declared_network_scope_contract`    | Is the permitted network scope exactly closed?                                                                 |
| `undeclared_network_contract`        | Are undeclared IPv4, IPv6, DNS, Unix-socket, and relevant Mach-service effects denied before effect?           |
| `descendant_membership_contract`     | Is every descendant owned across fork, exec, reparenting, `setsid`, and `setpgid`?                             |
| `group_shutdown_contract`            | Can the complete owned graph be terminated on every terminal path?                                             |
| `direct_child_reap_contract`         | Can every direct child be deterministically reaped?                                                            |
| `terminal_quiescence_contract`       | Can the controller establish race-free graph quiescence and reject late effects?                               |
| `pipe_closure_contract`              | Can every child communication pipe be closed on every terminal path?                                           |
| `process_metadata_effect_contract`   | Are process-metadata effects bounded and source-minimized?                                                     |
| `cleanup_binding_contract`           | Can cleanup remain descriptor-bound and non-following?                                                         |
| `cleanup_failure_contract`           | Does cleanup failure retain private quarantine and block retry/replacement?                                    |
| `platform_cache_effect_contract`     | Are OS and toolchain cache effects bounded?                                                                    |
| `platform_log_effect_contract`       | Are OS and toolchain log effects bounded?                                                                      |
| `platform_socket_effect_contract`    | Are platform socket effects bounded?                                                                           |
| `platform_network_effect_contract`   | Is possible OS-managed network traffic bounded and disclosed?                                                  |

An incomplete or changed source corpus is `boundary_failed`. An absent,
ambiguous, archived-only, inferred, deprecated, unsupported, or conflicting
guarantee is `contract_unproven`. Either result fails candidate eligibility.

### P3-3 source checks remain Not run

Because no controller source exists, these ten D-102 checks remain `not_run`:

- `caller_selected_process_source_review`
- `shell_launch_source_review`
- `inherited_environment_source_review`
- `unreviewed_spawn_source_review`
- `retry_fallback_source_review`
- `raw_child_output_source_review`
- `deadline_enforcement_source_review`
- `cancellation_terminal_source_review`
- `late_effect_rejection_source_review`
- `cleanup_ownership_source_review`

No static decision may represent them as passed.

## Authoritative public-source register

Only current first-party Apple pages may establish current platform behavior.
Archived Apple material may support only narrow historical API semantics and
cannot establish current macOS 14+ availability. Search summaries, blogs,
forums, generated prose, runtime observation, and unstated implications are
not decision evidence.

| Source                                                                                                                                                                            | Permitted narrow claim                                                                                    |
| --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| [Configuring the macOS App Sandbox](https://developer.apple.com/documentation/xcode/configuring-the-macos-app-sandbox)                                                            | App Sandbox is kernel-enforced and entitlement-configured.                                                |
| [App Sandbox](https://developer.apple.com/documentation/security/app-sandbox)                                                                                                     | Resource categories are limited or restored through entitlements.                                         |
| [Entitlements](https://developer.apple.com/documentation/bundleresources/entitlements)                                                                                            | Entitlements declare executable rights or privileges and are carried with signed code.                    |
| [App Sandbox entitlement](https://developer.apple.com/documentation/bundleresources/entitlements/com.apple.security.app-sandbox)                                                  | The Boolean entitlement opts one executable into App Sandbox.                                             |
| [Network client entitlement](https://developer.apple.com/documentation/bundleresources/entitlements/com.apple.security.network.client)                                            | The entitlement permits opening outbound TCP and UDP connections.                                         |
| [Network server entitlement](https://developer.apple.com/documentation/bundleresources/entitlements/com.apple.security.network.server)                                            | The entitlement permits listening for inbound TCP and UDP connections.                                    |
| [App Groups entitlement](https://developer.apple.com/documentation/bundleresources/entitlements/com.apple.security.application-groups)                                            | App Groups can add shared-container and interprocess-communication authority.                             |
| [Embedding a command-line tool in a sandboxed app](https://developer.apple.com/documentation/xcode/embedding-a-helper-tool-in-a-sandboxed-app)                                    | A helper can be embedded and the helper entitlement pair is documented.                                   |
| [Accessing files from the macOS App Sandbox](https://developer.apple.com/documentation/security/accessing-files-from-the-macos-app-sandbox)                                       | Container access, signature association, external executable restrictions, and broader access mechanisms. |
| [Protecting user data with App Sandbox](https://developer.apple.com/documentation/security/protecting-user-data-with-app-sandbox)                                                 | Sandboxed containers retain unrestricted internal access while host resources remain restricted.          |
| [Discovering and diagnosing App Sandbox violations](https://developer.apple.com/documentation/security/discovering-and-diagnosing-app-sandbox-violations)                         | Sandbox diagnostics can produce platform log and report effects.                                          |
| [`kSecCodeSignatureAdhoc`](https://developer.apple.com/documentation/security/seccodesignatureflags/adhoc)                                                                        | An ad-hoc seal has no signing identity.                                                                   |
| [Foundation `Process`](https://developer.apple.com/documentation/foundation/process)                                                                                              | Direct subprocess sandbox/environment inheritance and one-instance launch semantics only.                 |
| [`Process.terminate()`](https://developer.apple.com/documentation/foundation/process/terminate)                                                                                   | Termination requests apply to one `Process` receiver and its documented subtasks only.                    |
| [`Process.waitUntilExit()`](https://developer.apple.com/documentation/foundation/process/waituntilexit)                                                                           | Synchronous waiting applies to one `Process` receiver.                                                    |
| [`Process.terminationHandler`](https://developer.apple.com/documentation/foundation/process/terminationhandler)                                                                   | Handler timing and one-process termination notification only.                                             |
| [`DispatchSourceProcess`](https://developer.apple.com/documentation/dispatch/dispatchsourceprocess)                                                                               | A dispatch source can observe events for one known process identifier.                                    |
| [`Pipe`](https://developer.apple.com/documentation/foundation/pipe) and [`FileHandle.close()`](https://developer.apple.com/documentation/foundation/filehandle/close)             | Individual pipe/file-handle construction and close semantics only.                                        |
| [`FileDescriptor.OpenOptions.noFollow`](https://developer.apple.com/documentation/system/filedescriptor/openoptions/nofollow)                                                     | A no-follow option can reject a final symbolic-link component for one open operation.                     |
| [`URL.cachesDirectory`](https://developer.apple.com/documentation/foundation/url/cachesdirectory)                                                                                 | A platform-provided cache-directory location exists.                                                      |
| [Generating log messages from your code](https://developer.apple.com/documentation/os/generating-log-messages-from-your-code)                                                     | Unified logging can create platform-retained log effects.                                                 |
| [App Sandbox entitlement inheritance](https://developer.apple.com/library/archive/documentation/Miscellaneous/Reference/EntitlementKeyReference/Chapters/EnablingAppSandbox.html) | Archived narrow inheritance semantics; not current-target availability proof.                             |
| [`posix_spawn(2)`](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/posix_spawn.2.html)                                         | Fixed direct-process launch mechanics only.                                                               |
| [`wait(2)`](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/wait.2.html)                                                       | Direct-child wait/reap semantics only.                                                                    |
| [`kqueue(2)`](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/kqueue.2.html)                                                   | Known-PID process-event observation only.                                                                 |
| [`setpgid(2)`](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setpgid.2.html)                                                 | Mutable process-group semantics only.                                                                     |
| [`killpg(2)`](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/killpg.2.html)                                                   | Process-group signaling mechanics only.                                                                   |

Official text establishes only the stated claim. It does not automatically
prove bootstrap provenance, exact effect denial, complete graph ownership,
containment-wide termination, quiescence, cleanup, or target availability.

## Frozen contract dispositions

The source corpus above is frozen for this attempt. Every D-102 contract is
independently necessary; none may borrow a stronger claim from another row.

| Contract ID                          | Disposition         | Source-bound reason                                                                                   |
| ------------------------------------ | ------------------- | ----------------------------------------------------------------------------------------------------- |
| `containment_primitive_contract`     | `contract_unproven` | App Sandbox is supported, but the sources do not establish the candidate's complete D-102 boundary.   |
| `fixed_build_graph_contract`         | `contract_unproven` | The helper recipe presupposes build/sign/embed steps and does not prove a contained first graph.      |
| `executable_identity_contract`       | `contract_unproven` | No source fixes every transitive executable byte identity and literal argument without lookup.        |
| `working_directory_binding_contract` | `contract_unproven` | No source binds and retains one exact application-owned working directory for the complete graph.     |
| `descriptor_inheritance_contract`    | `contract_unproven` | The sources do not prove closure of every undeclared inherited descriptor before launch.              |
| `filesystem_read_scope_contract`     | `contract_unproven` | General sandbox restrictions do not enumerate and deny every ambient build/toolchain read.            |
| `filesystem_root_contract`           | `contract_unproven` | No source establishes exact repository, toolchain, dependency, cache, and output roots.               |
| `outside_root_write_contract`        | `contract_unproven` | Container-wide write access and possible broader access do not prove exact-root pre-effect denial.    |
| `declared_network_scope_contract`    | `contract_unproven` | TCP/UDP entitlement semantics do not close inherited descriptors or the complete allowed scope.       |
| `undeclared_network_contract`        | `contract_unproven` | No source closes IPv4, IPv6, DNS, Unix-socket, and relevant Mach-service effects together.            |
| `descendant_membership_contract`     | `contract_unproven` | Direct-child, known-PID, and process-group APIs do not own every detached or reparented descendant.   |
| `group_shutdown_contract`            | `contract_unproven` | Process-group membership is mutable and signaling does not guarantee complete graph termination.      |
| `direct_child_reap_contract`         | `contract_unproven` | Direct-child wait semantics do not prove deterministic reaping for the candidate's complete graph.    |
| `terminal_quiescence_contract`       | `contract_unproven` | No source establishes race-free graph quiescence and rejection of every late effect.                  |
| `pipe_closure_contract`              | `contract_unproven` | Closing one handle does not prove closure of every copied or inherited pipe descriptor.               |
| `process_metadata_effect_contract`   | `contract_unproven` | Known-PID/process APIs expose metadata without bounding its logs, caches, lifetime, or visibility.    |
| `cleanup_binding_contract`           | `contract_unproven` | A final-component no-follow open option does not prove descriptor-rooted recursive cleanup.           |
| `cleanup_failure_contract`           | `contract_unproven` | Private quarantine and retry blocking are controller duties for which no implementation exists.       |
| `platform_cache_effect_contract`     | `contract_unproven` | A cache directory exists, but OS and build-tool cache destinations and contents are not bounded.      |
| `platform_log_effect_contract`       | `contract_unproven` | Sandbox diagnostics and unified logging can persist effects whose content and lifetime are unbounded. |
| `platform_socket_effect_contract`    | `contract_unproven` | Inherited, resolver, Unix, Mach/XPC, and other platform socket effects are not completely bounded.    |
| `platform_network_effect_contract`   | `contract_unproven` | OS-managed resolver, trust, signature, container, and network traffic is not completely bounded.      |

All ten implementation-source checks remain absent and therefore `not_run`:

| Source check                            | Disposition |
| --------------------------------------- | ----------- |
| `caller_selected_process_source_review` | `not_run`   |
| `shell_launch_source_review`            | `not_run`   |
| `inherited_environment_source_review`   | `not_run`   |
| `unreviewed_spawn_source_review`        | `not_run`   |
| `retry_fallback_source_review`          | `not_run`   |
| `raw_child_output_source_review`        | `not_run`   |
| `deadline_enforcement_source_review`    | `not_run`   |
| `cancellation_terminal_source_review`   | `not_run`   |
| `late_effect_rejection_source_review`   | `not_run`   |
| `cleanup_ownership_source_review`       | `not_run`   |

The closed result is `no_eligible_candidate_after_d104_rereview`. This is a
successful fail-closed documentation review, not proof of universal
impossibility and not an operational failure.

## Implementation milestones

- [x] Confirm clean synchronized baseline, Git integrity, toolchains, and valid
      D-104 predecessor gate.
- [x] Create only the owner-authorized branch and begin the exact documentation
      gate.
- [x] Record this exact owner-approved Ready plan as Active without changing
      product or external state.
- [x] Freeze the v2 candidate attempt and authoritative source corpus.
- [x] Disposition all 22 contract rows and retain all ten source checks as
      `not_run`.
- [x] Record additive D-105 and synchronize the declared documentation paths.
- [x] Run documentation-tier checks and independent architecture, security,
      code-health, debt, documentation, and readiness reviews.
- [x] Record the truthful passing or terminal-failed post-increment result.

## Security and privacy considerations

| Threat                                           | Required control                                                                            |
| ------------------------------------------------ | ------------------------------------------------------------------------------------------- |
| D-104 becomes signing permission                 | Keep the seal conceptual and prohibit every operational signing action.                     |
| Identity-free is called authority-free           | Treat App Sandbox entitlements as kernel-enforced authority changes requiring exact review. |
| Inheritance is generalized to graph ownership    | Require independent membership, shutdown, and quiescence contracts.                         |
| Helper bootstrap is assumed safe                 | Require fixed provenance without an uncontained build, unreviewed binary, or dependency.    |
| Broad access is added for a usable build         | Freeze the exact two-key helper ceiling and reject every broader entitlement.               |
| Missing documentation becomes a denial guarantee | Use `contract_unproven`; never infer a pass.                                                |
| Direct-child APIs become containment             | Limit wait/event/group APIs to their documented narrow semantics.                           |
| Raw host or process evidence leaks               | Retain public citations and D-100 categorical dispositions only.                            |
| A positive document starts implementation        | Require a new owner-approved P3-3 plan and gate.                                            |
| Historical failure is silently healed            | Preserve D-097 through D-104 and all Failed/Pending/Not-run facts.                          |

## Test plan

- Verify the only candidate is the additive v2 identity and D-103's v1 result
  remains untouched.
- Verify the sole changed assumption is D-104's ad-hoc-versus-Developer-ID
  classification.
- Verify every 22 contract ID appears once, with one allowed closed outcome,
  and one absent or ambiguous contract forces the negative decision.
- Verify all ten P3-3 source checks remain `not_run`.
- Verify the conceptual host/helper entitlement ceiling is exact and all
  broader authority is explicitly absent.
- Verify every source claim is bound to a first-party link and no archived page
  establishes current availability.
- Verify no target-derived path, host, process, account, screenshot, trace,
  packet, command output, credential, or free text enters evidence.
- Verify D-097 through D-104 and all historical Failed/Pending/Not-run evidence
  remain unchanged.
- Verify exactly the declared fifteen documentation paths change and every
  source, dependency, configuration, workflow, hook, skill, and script path is
  unchanged.

## Verification commands

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
git diff --exit-code -- src src-tauri package.json package-lock.json .npmrc rust-toolchain.toml .github .codex/hooks .agents scripts eslint.config.js vite.config.ts tsconfig.app.json tsconfig.json tsconfig.node.json
python3 .codex/hooks/session_end_gate.py
python3 .codex/hooks/post_increment_gate.py status
```

`npm run verify`, builds, process/probe checks, target-Mac checks, and all
Apple/Xcode/Keychain/certificate/signing/provider/product/external checks remain
Not run by scope.

## Manual gates and dependencies

| Gate                     | Status   | Required evidence                                                                         |
| ------------------------ | -------- | ----------------------------------------------------------------------------------------- |
| D-104 predecessor        | Complete | Valid passing predecessor state at the synchronized baseline.                             |
| Candidate/source freeze  | Passed   | Exact v2 identity, fixed source register, and private attempt binding.                    |
| Complete contract review | Passed   | All 22 rows are `contract_unproven`; all ten source checks are `not_run`.                 |
| Historical preservation  | Passed   | D-097 through D-104 and all prior evidence remain unchanged.                              |
| Independent review       | Passed   | Architecture, security, code-health, debt, documentation, and readiness reviews passed.   |
| P3-3 controller          | Blocked  | A new additive candidate-selection decision, exact plan, and owner approval are required. |
| P3-4 through V0-3        | Blocked  | Their existing independent prerequisites and approvals.                                   |

## Risks

The dominant risk is representing a documented App Sandbox inheritance or
ad-hoc-seal fact as proof of complete containment. The candidate also appears
to face independent bootstrap, executable-location, filesystem-scope,
network/IPC, descendant-membership, termination, and quiescence gaps. The plan
permits a negative result and requires one unproved predicate to reject
selection.

## Rollback or failure strategy

Before publication, reverse only uncommitted in-scope documentation edits with
`apply_patch`. After publication, use a separately approved additive
superseding or revert commit. Never reset, discard, or rewrite historical
evidence. A negative candidate result may still complete this documentation
increment truthfully with Blocked successor readiness. A validation or review
failure must use the gate's terminal-failed disposition.

## Stop conditions

Stop and record no eligible candidate if the candidate/source identity changes,
any mandatory contract is absent or inferred, bootstrap requires an
uncontained build or unreviewed input, a broader entitlement or authority is
needed, or raw/target-derived evidence would be required.

Stop the increment without passing finalization if a protected path changes,
historical evidence drifts, the source corpus loses provenance, an operational
command becomes necessary, documentation validation fails, or independent
review finds a completion blocker. Do not substitute another candidate, widen
scope, retry a boundary failure, begin P3-3, or perform any operational action.

## Decisions made

The owner approved the exact Ready plan, branch, and begin operation. D-105
records `no_eligible_candidate_after_d104_rereview`; no candidate or successor
is selected.

## Discoveries

Current first-party documentation confirms only narrow sandbox, helper,
signature, direct-child, and process-group semantics. None is preclassified as
proof of D-102's complete conjunction.

The D-104 identity distinction removes only one prior classification issue.
Independent bootstrap, effect-control, graph-ownership, shutdown, quiescence,
cleanup, and platform-effect gaps still reject the v2 candidate.

The first documentation-health pass found three Apple method links whose URL
parentheses were parsed as local targets. The URLs were corrected without
changing the frozen sources or permitted claims; both `docs:check` and
`repository:check` then required passing reruns.

## Progress

- 2026-09-02: Clean synchronized baseline, Git integrity, toolchain pins, and
  valid D-104 completion were confirmed.
- 2026-09-02: The owner-authorized branch was created, the exact gate began,
  and this Ready plan was recorded as Active. No operational action ran.
- 2026-09-02: The source corpus was frozen; all 22 D-102 contracts were
  dispositioned `contract_unproven`, all ten source checks remained `not_run`,
  and D-105 recorded the bounded negative result.
- 2026-09-02: Initial documentation/repository checks each reported the same
  three malformed link targets; the three URLs were corrected in scope for a
  passing rerun.
- 2026-09-02: Independent review found two plan/report reconciliation defects;
  both were corrected without changing D-105, scope, or authority.
- 2026-09-02: The exact documentation scope, protected-path review, repository
  health, formatting, links, secret scan, session inventory, and active-gate
  status passed. Quality is `PASS WITH ADVISORIES`; next readiness is Blocked.

## Acceptance criteria

- [x] Exactly the v2 candidate is reviewed under the sole D-104 assumption
      change; D-103 v1 remains immutable.
- [x] All 22 D-102 contract rows receive closed source-bound dispositions and
      all ten source rows remain honestly `not_run`.
- [x] D-105 records one closed decision without granting operational authority.
- [x] D-097 through D-104 and every Failed/Pending/Not-run fact remain intact.
- [x] The exact documentation scope and required validation pass.
- [x] P3-3 and every operational successor remain Blocked unless a new
      additive candidate-selection decision and separately approved plan first
      establish eligibility.

## Final results

`PASS WITH ADVISORIES`: the bounded static review concluded
`no_eligible_candidate_after_d104_rereview`. All required documentation checks
and independent reviews passed after the recorded three-link correction. No
successor is Ready; operational containment remains Blocked.

## Documentation updates

- [x] `HANDOFF.md`
- [x] `PROJECT_STATUS.md`
- [x] `NEXT_STEPS.md`
- [x] `DECISIONS.md`
- [x] `CHANGELOG.md`
- [x] `TROUBLESHOOTING_LOG.md`
