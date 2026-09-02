# Personal Assistant v0 containment primitive selection

Status: Complete — `PASS WITH ADVISORIES`
Owner: Henry Dang
Last updated: 2026-09-02
Increment: `personal-assistant-v0-containment-primitive-selection`
Predecessor: completed D-102 build-child-containment planning

## Goal

Review one frozen target-Mac candidate set against D-102 using authoritative
public contracts and record exactly one of two bounded results: one eligible
candidate, or no eligible candidate in the reviewed set. This increment may
make a documentation decision only. It cannot implement, configure, execute,
or prove a containment primitive.

## Scope

1. Deep-review only
   `app_sandbox_build_helper_plus_libsystem_supervision_v1`.
2. Retain `sandbox_exec_supervisor_v1`,
   `posix_process_group_supervisor_v1`, and
   `output_root_posthoc_scan_v1` as fixed negative controls.
3. Screen `privileged_system_extension_v1` and
   `virtual_machine_or_container_v1` only for scope eligibility.
4. Apply D-102's contracts without weighting, compensating controls, empirical
   inference, or residual-risk acceptance.
5. Record D-103 and reconcile exactly the fifteen documentation paths listed
   below.

There is no catch-all or caller-selected candidate. Adding a candidate or
changing an eligibility rule requires a plan amendment and separate owner
approval.

## Explicit non-goals

- No build, helper, child process, probe, runtime observation, tracing, packet
  capture, filesystem/network test, or target-Mac inspection.
- No `sandbox-exec`, `xcrun`, `xcodebuild`, `codesign`, `security`, `launchctl`,
  Cargo, Vite, Tauri, npm lifecycle, or product-runtime command.
- No source, test, dependency, lockfile, configuration, capability, CSP,
  permission, entitlement, runner, workflow, hook, or product change.
- No authenticated or state-changing Apple/external access; no Xcode, Keychain,
  certificate, private-key, signing, credential, provider, model, account,
  identity, external-resource creation, or device action. The approved public
  documentation reads are the only external contact.
- No controller, synthetic proof, disposable no-sign build, P4 signer binding,
  signing proof, V0-3, branch, commit, push, merge, pull request, release, or
  publication.
- No rewrite or downgrade of D-097, its report or digests, its Failed privacy
  finding, the Pending Open Directory boundary, any Not-run signing result, or
  the absence of its completion marker.

## Existing behavior and constraints

- D-102 requires pre-effect filesystem and network denial, complete descendant
  ownership across fork, exec, reparenting, `setsid`, and `setpgid`, terminal
  shutdown/reaping/quiescence, descriptor-bound cleanup, and D-100-minimized
  evidence.
- The repository has no build-child containment controller. The ordinary npm,
  Vite/Tauri, Cargo build-script, compiler, and linker graph is not contained by
  output routing, process groups, clean Git state, or post-hoc scans.
- The target is macOS 14 or later. Current Tauri configuration has no
  entitlement file, and the sole capability file does not grant a build or
  process boundary.
- P3-2 may pass with a bounded negative selection decision. Such a passing
  documentation result does not make P3-3 or an operational successor Ready.

## Current-state evidence

- Baseline `HEAD` and `origin/main` were both
  `355e1efcdd2c5d651dd1609959581e7184b61b60`, ahead/behind `0/0`, with a clean
  working tree before the gate.
- `git fsck --full --no-dangling` passed. The ignored predecessor gate was
  complete, valid, and `PASS WITH ADVISORIES`.
- Repository-pinned toolchains were Node `26.3.0`, npm `11.16.0`, and
  Cargo/Rust `1.90.0`; baseline `npm run docs:check` passed.
- The first `begin` attempt used the descriptive plan name and was rejected
  because the identifier exceeded the gate's 64-character kebab-case bound.
  It changed no state. The same approved scope then began under the shortened
  identifier `personal-assistant-v0-containment-primitive-selection`.
- Approved read-only public documentation access was the sole external contact.
  No operational P3, target-Mac, authenticated Apple, signing, or state-changing
  external-system command ran. No target-derived output, path, process, host,
  account, screenshot, trace, packet, credential, or free text is retained as
  review evidence.

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
13. `docs/increments/personal-assistant-v0-containment-primitive-selection.md`
14. this plan
15. `docs/reviews/2026-09-02-personal-assistant-v0-containment-primitive-selection-post-increment-review.md`

## Affected components

| Component                         | Effect                                                        |
| --------------------------------- | ------------------------------------------------------------- |
| Repository governance             | Adds D-103's bounded negative primitive-selection decision.   |
| Build-child security planning     | Records why every frozen candidate is ineligible or unproven. |
| Product runtime, IPC, build graph | None.                                                         |
| Target Mac and external systems   | None.                                                         |

## Interfaces and invariants

### Closed candidate set and result rule

| Candidate                                                | Review depth                                                    | Result   | Exact reason                                                                                                                                                                                                                                                                                                                                                                               |
| -------------------------------------------------------- | --------------------------------------------------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `app_sandbox_build_helper_plus_libsystem_supervision_v1` | Authoritative public-contract review                            | Rejected | App Sandbox/helper inheritance requires entitlements and entitlement-bearing code signatures, violating the independent no-signing eligibility gate. Public `waitpid`, `kqueue` `EVFILT_PROC`, `setpgid`, and process-signal contracts also do not prove complete app-owned descendant membership, containment-wide termination, and race-free quiescence after detachment or reparenting. |
| `sandbox_exec_supervisor_v1`                             | Existing repository evidence plus negative-control confirmation | Rejected | Deprecated/private profile surface; no authoritative membership, containment-wide termination, reaping, or exact full network contract.                                                                                                                                                                                                                                                    |
| `posix_process_group_supervisor_v1`                      | Negative control                                                | Rejected | Does not deny effects and does not retain authoritative membership after session/group escape or reparenting.                                                                                                                                                                                                                                                                              |
| `output_root_posthoc_scan_v1`                            | Negative control                                                | Rejected | Observes after an effect and supplies neither pre-effect denial nor descendant ownership.                                                                                                                                                                                                                                                                                                  |
| `privileged_system_extension_v1`                         | Eligibility screen                                              | Rejected | Requires privileged/entitled/signed system-extension or Network/Endpoint Security state, possible user approval, and external persistent system state.                                                                                                                                                                                                                                     |
| `virtual_machine_or_container_v1`                        | Eligibility screen                                              | Rejected | Apple's reviewed VM route requires a virtualization entitlement plus guest image/storage resources. No exact supported OS-shipped macOS 14+ container candidate or public contract satisfying this plan's boundary was identified, so that branch remains `contract_unproven`.                                                                                                             |

The exact decision is **no eligible candidate in the reviewed set**. It is not
a universal impossibility claim and does not authorize an unreviewed
alternative.

“libSystem supervision” is closed here to `posix_spawn` launch, `waitpid`
direct-child waiting/reaping, `kqueue` `EVFILT_PROC` observation for a known
PID, and `setpgid`/process-group signaling. It does not name an extensible API
family. Only the App Sandbox composition entered the D-100 contract review.
The three negative controls retain D-102's already documented exclusions, and
the two scope-only classes were rejected by the independent eligibility screen;
neither group creates a D-100 candidate attempt or outcome record.

### Exact eligibility gates

A candidate is eligible only if authoritative current public documentation
establishes every item:

1. Public, supported, non-deprecated API on the repository's macOS 14+ target.
2. OS-shipped or already pinned, with no dependency, daemon, image, service,
   lockfile, or toolchain change.
3. No credential, certificate, private key, Apple service, administrator/root
   authority, entitlement, permission, signing prerequisite, or global state.
4. Fixed complete npm/Vite/Tauri/Cargo/compiler/linker graph and shell-free,
   fixed absolute launches with literal arguments, one working directory,
   closed environment/descriptors, and no fallback or retry.
5. Pre-effect denial of ambient host-data reads, outside-root writes, and
   undeclared IPv4, IPv6, DNS, Unix-socket, and relevant Mach-service effects.
6. Authoritative descendant ownership across fork, exec, reparenting,
   `setsid`, and `setpgid`.
7. Terminal deadline/cancellation with containment-wide shutdown, direct-child
   reaping, pipe closure, race-free quiescence, and late-effect rejection.
8. Descriptor-bound non-following cleanup with private quarantine and blocked
   retry/replacement after failure.
9. Bounded process-metadata, cache, log, socket, and possible network effects.
10. D-100 source-minimized categorical evidence only, with no circular signing
    or containment prerequisite.

One absent, ambiguous, deprecated, inferred, empirical, or conflicting
contract disqualifies selection. There is no score or compensating control.

### D-100 outcome disposition

The ten implementation-source checks remain `not_run` because P3-3 source does
not exist:

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

For the only deep-review candidate, these mandatory aggregate predicates are
`contract_unproven`: `containment_primitive_contract`,
`filesystem_read_scope_contract`, `filesystem_root_contract`,
`outside_root_write_contract`, `declared_network_scope_contract`,
`undeclared_network_contract`, `descendant_membership_contract`,
`group_shutdown_contract`, `terminal_quiescence_contract`,
`process_metadata_effect_contract`, `platform_cache_effect_contract`,
`platform_log_effect_contract`, `platform_socket_effect_contract`, and
`platform_network_effect_contract`. The eight `not_run` contract checks are
`fixed_build_graph_contract`, `executable_identity_contract`,
`working_directory_binding_contract`, `descriptor_inheritance_contract`,
`direct_child_reap_contract`, `pipe_closure_contract`,
`cleanup_binding_contract`, and `cleanup_failure_contract`. They were not
reviewed after the independent entitlement/signing/no-circular-dependency gate
failed closed. None of these documentation dispositions is an emitted runtime
record or authority.

The review attempt privately binds the fixed deep-review candidate identity,
each predeclared check ID, and this attempt; candidate identity is not a fourth
`evidence_privacy_v1` field. Unexpected source material, incomplete provenance,
target-derived dynamic data, malformed records, or loss of binding is
`boundary_failed` and stops the review without retry or an expanded outcome
vocabulary. `contract_unproven`, `not_run`, and `boundary_failed` are all non-
authorizing.

### Authoritative source register

Reviewed 2026-09-02; all links are Apple public documentation. Apple library
archive manual pages are used only for their narrow Darwin/POSIX semantics and
do not establish current macOS 14+ availability. That availability therefore
remains part of the aggregate `contract_unproven` result rather than an inferred
pass.

| Source                                                                                                                                                                                                                                                                                 | Contract used                                                                                                                     |
| -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| [Configuring the macOS App Sandbox](https://developer.apple.com/documentation/xcode/configuring-the-macos-app-sandbox)                                                                                                                                                                 | App Sandbox is kernel-enforced and entitlement-configured.                                                                        |
| [App Sandbox entitlement inheritance](https://developer.apple.com/library/archive/documentation/Miscellaneous/Reference/EntitlementKeyReference/Chapters/EnablingAppSandbox.html)                                                                                                      | A spawned child helper requires the sandbox and inherit entitlements; the main app must not carry inherit.                        |
| [Embedding a helper tool in a sandboxed app](https://developer.apple.com/documentation/xcode/embedding-a-helper-tool-in-a-sandboxed-app)                                                                                                                                               | The helper is embedded, entitlement-bound, and signed, including Developer ID distribution.                                       |
| [Accessing files from the macOS App Sandbox](https://developer.apple.com/documentation/security/accessing-files-from-the-macos-app-sandbox)                                                                                                                                            | Container and user-selected/security-scoped access are signature-associated; arbitrary external program execution is constrained. |
| [App Sandbox network-server entitlement](https://developer.apple.com/documentation/bundleresources/entitlements/com.apple.security.network.server)                                                                                                                                     | Network entitlements grant broad connection categories, not D-102's exact destination/data-flow contract.                         |
| [Creating XPC services](https://developer.apple.com/library/archive/documentation/MacOSX/Conceptual/BPSystemStartup/Chapters/CreatingXPCServices.html)                                                                                                                                 | Each sandboxed service has its own signed entitlement context and launchd lifecycle.                                              |
| [Foundation Process](https://developer.apple.com/documentation/foundation/process)                                                                                                                                                                                                     | A direct child inherits its parent's sandbox; the environment is inherited unless closed by application code.                     |
| [`wait(2)`](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/wait.2.html)                                                                                                                                                            | Waiting/reaping applies to child processes; remaining children reparent when the parent exits.                                    |
| [`kqueue(2)`](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/kqueue.2.html)                                                                                                                                                        | Process filters observe a known PID's events; observation is not a containment identity.                                          |
| [`setpgid(2)`](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setpgid.2.html)                                                                                                                                                      | Process-group membership is mutable within the documented constraints.                                                            |
| [Endpoint Security](https://developer.apple.com/documentation/endpointsecurity) and [monitoring events](https://developer.apple.com/documentation/endpointsecurity/monitoring-system-events-with-endpoint-security)                                                                    | Endpoint Security requires an entitled, signed system extension and user/system authorization state.                              |
| [Network Extension entitlement](https://developer.apple.com/documentation/bundleresources/entitlements/com.apple.developer.networking.networkextension)                                                                                                                                | Developer ID use requires capability, provisioning, certificate, signing, and entitlement state.                                  |
| [Installing system extensions](https://developer.apple.com/documentation/systemextensions/installing-system-extensions-and-drivers/)                                                                                                                                                   | Activation validates signatures, entitlements, team identity, and may require user approval.                                      |
| [Virtualization entitlement](https://developer.apple.com/documentation/virtualization/adding-the-virtualization-entitlement-to-your-project) and [running macOS in a VM](https://developer.apple.com/documentation/virtualization/running-macos-in-a-virtual-machine-on-apple-silicon) | Virtualization requires an entitlement plus guest image/storage and related VM resources.                                         |

Official documentation may establish only the narrow claim stated here. Search
summaries, blogs, forums, target observation, generated output, and “worked on
this Mac” are not selection evidence. An unstated guarantee is
`contract_unproven`.

App Sandbox entitlements are represented in an entitlement-bearing code
signature; Developer ID distribution additionally uses Developer ID signing.
This review does not equate ad hoc/development signing with the later P4
Developer ID signer-binding proof. Instead, it applies the stricter approved
P3-2 eligibility rule: any prerequisite signing state or new entitlement is
independently disqualifying before P4.

## Implementation milestones

- [x] Confirm the clean synchronized baseline and valid predecessor gate.
- [x] Begin only the approved documentation gate.
- [x] Freeze and review the exact candidate set against authoritative public
      contracts.
- [x] Record the bounded negative D-103 decision and synchronize the declared
      documentation paths.
- [x] Run documentation-tier validation and independent reviews.
- [x] Record the deterministic post-increment report for finalization.

## Security and privacy considerations

| Threat                                                        | Required control                                                 |
| ------------------------------------------------------------- | ---------------------------------------------------------------- |
| Entitlement/signing circularity is overlooked                 | Reject any primitive needing signing or entitlements before P4.  |
| Observation is described as containment                       | Require pre-effect denial and authoritative membership.          |
| Direct-child/process-group handling is generalized to a graph | Require explicit detached/reparented descendant contracts.       |
| Broad network entitlement is called exact confinement         | Require exact protocol/address/name/socket/service contracts.    |
| A negative decision becomes permission to improvise           | Freeze candidate IDs and prohibit an “other” path.               |
| Source text or target data leaks into evidence                | Retain only public citations and fixed categorical dispositions. |
| Historical failure is silently healed                         | Preserve D-097/D-098 and all Failed/Pending/Not-run facts.       |
| Passing documentation starts P3-3                             | Keep every operational successor Blocked.                        |

## Test plan

- Verify the candidate set is exact, each eligibility rule is conjunctive, and
  the result says only “no eligible candidate in the reviewed set.”
- Verify all ten source-review checks remain `not_run`; review the exact
  `contract_unproven`/`not_run` split without emitting target-derived evidence.
- Verify cited claims are limited to official public contracts and do not infer
  complete containment from process observation or direct-child APIs.
- Verify D-097 failed/FAIL/Blocked, its report and digests, missing completion
  marker, Failed privacy finding, Pending Open Directory boundary, and Not-run
  signing remain unchanged.
- Verify exactly the declared fifteen paths changed and protected source,
  dependency, configuration, workflow, hook, and script paths did not.
- Run `npm run docs:check`, `npm run repository:check`,
  `npm run security:scan`, `git diff --check`, session inventory, independent
  review, and the post-increment gate.
- Keep `npm run verify`, all build/process/probe checks, and all
  Apple/Xcode/Keychain/signing/external checks Not run by approved scope.

## Manual gates and dependencies

| Gate                                | Status   | Required next evidence                                                                                                       |
| ----------------------------------- | -------- | ---------------------------------------------------------------------------------------------------------------------------- |
| P3-1 policy documentation           | Complete | D-102 and its valid completion record.                                                                                       |
| P3-2 primitive selection            | Complete | D-103's bounded negative decision and valid documentation gate.                                                              |
| P3-3 controller                     | Blocked  | A separately approved decision that changes the failed eligibility conditions and then selects a fully qualifying primitive. |
| P3-4 target-Mac synthetic proof     | Blocked  | Passing P3-3 source and a separate safe operational approval.                                                                |
| P3-5 disposable no-sign build proof | Blocked  | Passing P3-4 evidence and separate exact approval.                                                                           |
| P4 signer binding/signing/V0-3      | Blocked  | All independent P1-P4 prerequisites.                                                                                         |

## Rollback

Before publication, reverse only the uncommitted exact-path documentation edits
with `apply_patch`. After publication, use a separately approved additive
superseding/revert commit. Never reset, discard, or rewrite historical evidence.

## Stop conditions

Stop without a passing finalization if the source corpus or candidate inventory
is incomplete, a mandatory contract is treated as inferred, raw or target-
derived evidence appears, a candidate requires operation or an unapproved
dependency/entitlement/signing/privilege/external resource, a protected path
changes, historical evidence drifts, a required validation fails, or review
finds a completion-blocking defect. Do not begin P3-3 or perform any operational
action.

## Decisions, discoveries, and progress

- D-103 records no eligible candidate in the frozen reviewed set. It does not
  claim macOS has no possible containment mechanism.
- The most plausible publicly documented composition fails the plan before
  operational review because its helper boundary is entitlement- and signing-
  dependent.
- Apple documents direct-child wait/reap and known-PID observation, but the
  reviewed public contracts do not establish D-102's complete app-owned graph
  identity and quiescence across detachment and reparenting.
- Endpoint Security, Network Extension, and system extensions are not silent
  substitutes because they change privilege, entitlement, signing, or system-
  state boundaries. The reviewed VM route requires an entitlement and guest
  resources; no exact qualifying container contract was identified.
- No operational evidence was collected.

## Final result

`PASS WITH ADVISORIES`. The final `npm run docs:check`,
`npm run repository:check`, `npm run security:scan`, `git diff --check`,
protected-path diff, and session inventory passed. Independent architecture,
security, code-health, technical-debt, documentation, and readiness reviews
found no completion-blocking defect. The approved public-documentation reads
were the sole external contact. `npm run verify`, build/child/probe checks, and
all target-Mac/Apple/Xcode/Keychain/signing/provider/product operations remained
Not run by scope. The advisory is D-103's exact negative result: no eligible
candidate in the reviewed set, so next-increment readiness is `Blocked`.
