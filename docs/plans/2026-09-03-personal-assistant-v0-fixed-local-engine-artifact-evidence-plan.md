# Personal Assistant V0 fixed local engine and artifact evidence plan

Status: Complete — **PASS WITH ADVISORIES**; owner accepted the negative
disposition and exact D-121; the completion marker is valid; no candidate is
selected or admitted
Readiness: Blocked; no replacement candidate or operational successor is
selected or Ready
Owner: Project owner
Last updated: 2026-09-03
Planning baseline: `355d42ac8bb9a5ed663f57895675df688507ce9f`
Branch:
`codex/personal-assistant-v0-fixed-local-engine-artifact-evidence-plan`
Gate ID:
`personal-assistant-v0-fixed-local-engine-artifact-evidence-plan`
Decision: D-121; accepted `candidate_not_eligible_or_unproven`
Depends on: D-120 `fixed_local_v2_planning_selected` and the published PR #113
closeout of PR #112

## Goal

Define the smallest fail-closed documentation methodology for assessing exactly
one future application-owned, fixed, nonselectable local/no-auth candidate for
D-094's reserved `real-content-v2` Personal Assistant contract.

The assessment must determine whether one exact engine, model, artifact,
dependency graph, acquisition/removal design, in-process trusted-computing-base
boundary, lifecycle contract, and target-Mac evidence protocol are sufficiently
specified to justify a later separately approved target-Mac evidence plan.

The plan originally named no engine or model and required an owner-supplied
envelope before any candidate-specific research. The owner subsequently
authorized one same-active-increment, nomination-only research phase using
current official public primary sources. That narrow phase may propose exactly
one immutable envelope, but it may not admit the candidate or begin the full
evidence assessment. It may not download, install, load, execute, benchmark,
build, resolve, or change an engine, model, artifact, dependency, or product.

The owner accepted the proposed envelope exactly and separately authorized the
full documentation-only static assessment from its frozen source allowlist.
That assessment has now completed an evidence-resolved tuple without changing
the envelope. No identity or evidence-boundary mismatch occurred, but multiple
mandatory properties are unproved and the pinned engine retains a forbidden
reachable dynamic-loader capability. The owner then accepted the resolved
tuple, evidence matrix, `candidate_not_eligible_or_unproven` disposition, and
exact D-121 wording and authorized only the documented sixteen-file final
reconciliation and required completion gates. No operational or successor
authority follows.

## User-visible outcome

None. This is a documentation-only plan. It creates no local model, text-entry
flow, streamed answer, profile selector, Tauri command or event, filesystem
location, credential, provider connection, network path, process, model
artifact, permission prompt, or device effect.

Even a positive future evidence disposition would mean only that static
evidence supports a separately approved target-Mac planning step. It would not
make `real-content-v2` usable, admit D-119's `local_no_auth` profile, authorize
personal prompts, or make any source increment Ready.

## Scope

### This planning task

Create exactly this one file:

1. `docs/plans/2026-09-03-personal-assistant-v0-fixed-local-engine-artifact-evidence-plan.md`

Do not create a branch or begin a gate during this planning task.

### Separately approved evidence increment

If the owner approves this exact plan, a staged documentation-only evidence
increment may change exactly these sixteen documentation paths:

1. `ARCHITECTURE.md`
2. `CHANGELOG.md`
3. `DECISIONS.md`
4. `HANDOFF.md`
5. `NEXT_STEPS.md`
6. `PLANS.md`
7. `PRODUCT_REQUIREMENTS.md`
8. `PROJECT_STATUS.md`
9. `ROADMAP.md`
10. `SECURITY.md`
11. `SECURITY_CHECKLIST.md`
12. `TESTING_GUIDE.md`
13. `docs/PROJECT_DIRECTION.md`
14. `docs/plans/2026-09-03-personal-assistant-v0-fixed-local-engine-artifact-evidence-plan.md`
15. `docs/increments/personal-assistant-v0-fixed-local-engine-artifact-evidence-plan.md`
16. `docs/reviews/2026-09-03-personal-assistant-v0-fixed-local-engine-artifact-evidence-plan-post-increment-review.md`

`TROUBLESHOOTING_LOG.md` is excluded unless a separately approved amendment is
required for a genuinely new troubleshooting fact. Any other changed or new
path is scope drift and stops the increment.

## Explicit non-goals

Neither this plan nor the proposed documentation evidence increment may:

- change Rust, TypeScript, tests, dependencies, manifests, lockfiles,
  toolchains, workflows, hooks, skills, scripts, Tauri configuration,
  capabilities, CSP, permissions, entitlements, signing, or product
  configuration;
- select, admit, install, download, copy, load, parse, benchmark, execute,
  update, repair, replace, remove, or redistribute an engine, model, tokenizer,
  template, executable, library, container, server, helper, or artifact;
- access an artifact registry beyond separately authorized public metadata,
  retrieve artifact bytes, run build scripts, compile native code, execute a
  candidate, or copy disposable resolver output into the repository;
- access credentials, Keychain, certificates, private keys, Apple/Xcode,
  signing systems, provider accounts, gateways, cloud resources, billing,
  product systems, or operational external systems;
- transmit a prompt, output, derived content, model metadata, identifying
  metadata, credential, or secret;
- create DNS, HTTP, HTTPS, TLS, proxy, localhost, Unix-socket, WebSocket,
  provider, telemetry, update, license-check, crash-upload, remote-embedding,
  or fallback product behavior;
- create or widen a filesystem, model-artifact, process, dynamic-code, JIT,
  plugin, GPU, Tauri IPC, WebView, provider, credential, persistence, audit,
  tool, or device boundary;
- restore D-032's removed synchronous `complete(prompt) -> String` interface or
  make an engine/runtime/profile selectable by a caller, environment, model
  metadata, artifact metadata, path, or ambient configuration;
- admit D-119's `local_no_auth` entry, issue a selection handle, change the
  ten-entry catalog, or change any entry from `candidate_blocked`;
- satisfy or waive Cortexa owner authentication merely because provider
  authentication is absent;
- change, complete, replace, or reinterpret synthetic-v1, historical V0-14,
  D-118, D-119, any D-107 blocker, or any operational `Blocked` state;
- treat fixture tests, source inspection, a target-Mac observation, or a
  successful documentation gate as product, privacy, containment, no-egress,
  cancellation, cleanup, or inference proof; or
- commit, push, merge, publish, release, deploy, begin a later increment, or
  create external state without separate exact owner authority.

## Existing behavior and constraints

### Program and decision lineage

- D-094 keeps synthetic-v1 fixed to the OpenAI-through-Cloudflare synthetic
  proof and keeps the distinct `real-content-v2` fixed and nonselectable.
- D-120 accepts only `fixed_local_v2_planning_selected`. It permits one new
  separately approved local-v2 engine/artifact evidence plan to precede V0-13;
  it selects no engine, model, artifact, dependency, filesystem boundary,
  runtime, transport, Tauri/UI contract, credential, provider, or source work.
- Synthetic-v1, D-066 through D-068, V0-3, V0-7, and V0-13 remain unchanged and
  Blocked for the remote lane.
- The existing V0-14 plan remains historical and Blocked with its recorded
  V0-13 dependency. This plan is additive and does not edit, start, supersede,
  or reclassify V0-14.
- D-118 remains `no_eligible_client` for its exact five frozen hostname-based
  HTTPS variants under the current hard-cancellation contract. A localhost
  service, helper, provider SDK, WebView request, private client, or hidden
  updater is not a local bypass.
- D-119 remains a distinct post-v0
  `personal-assistant-selectable-connection-profile-v3` direction. Exactly ten
  catalog-schema V1 entries remain `candidate_blocked`, and the blocked catalog
  exposes no handle:
  1. `local_no_auth`
  2. `google_gemini_oauth`
  3. `google_gemini_api_key`
  4. `direct_openai_api_key`
  5. `direct_openai_workload_identity`
  6. `azure_openai_entra`
  7. `azure_openai_api_key`
  8. `anthropic_api_key`
  9. `mistral_api_key`
  10. `aws_bedrock_identity`
- Fixed local-v2 is not admission or reuse of D-119's `local_no_auth` catalog
  entry. Direct OpenAI and Azure OpenAI remain separate, and ChatGPT login or
  subscription remains non-authorizing for OpenAI API use.
- D-060 continues to require gateway custody for every cloud/provider
  credential and credential-bearing OAuth result. D-061 remains controlling
  for every external provider.
- Local/no-auth means only no model-provider authentication or external
  inference connection. D-062 and D-094's non-demo Cortexa owner-authentication
  gate remains separately mandatory and Blocked. OS login, process ownership,
  or a personally controlled Mac is not silently accepted as that proof.
- Historical D-107 remains eight documented of eleven, and D-108 remains
  additively nine documented of ten. D-113 through D-117 remain Proposed and
  non-controlling.
- These exact ten D-107 contracts remain unproved and Blocked:
  `opaque_prebound_identity_contract`, `exact_signer_binding_contract`,
  `account_keychain_scope_contract`, `private_key_nonexport_contract`,
  `fixed_algorithm_contract`, `interaction_denial_contract`,
  `hard_deadline_cancellation_contract`, `late_result_rejection_contract`,
  `cleanup_quarantine_contract`, and `platform_effect_contract`.
- `NativeAgentRuntime` remains the sole/default runtime and is not
  caller-selectable. Hermes remains Deferred/Blocked and OpenClaw remains
  evaluation-only.

### Current production boundary

- `src-tauri/Cargo.toml` has no direct local-inference, tokenizer/model,
  provider, or HTTP/TLS client dependency. Its crate-level
  `unsafe_code = "forbid"` does not prove safety or no-egress for a future
  transitive native/FFI dependency.
- `src-tauri/src/lib.rs` exposes no Personal Assistant Tauri command, event, or
  managed host. Its current custom invoke surface remains app information, the
  sealed Research -> Knowledge projection, and four fixed no-input lifecycle
  commands.
- `src-tauri/src/personal_assistant_v0.rs` is a volatile transport-free host
  that performs no I/O and exposes no production response-frame ingress. It
  proves Rust-owned presentation handles, single-flight ownership, bounded
  state, exact returned-runtime identity validation, cancellation disposition,
  and quarantine behavior only for its present boundary.
- Personal Assistant success, failure, stream, and late-event ingress are
  driven by `#[cfg(test)]` fixtures. Existing runtime, gateway, Personal
  Assistant, Research -> Knowledge demo, approval, and orchestration tests are
  non-transferable to a local engine, native thread, model parser, artifact,
  GPU queue, or target-Mac inference path.
- Current Tauri CSP and `core:default` capability constrain the WebView
  boundary; they do not contain same-process Rust, native libraries, FFI,
  filesystem APIs, Keychain APIs, ambient process memory, or network syscalls.
  Hardened Runtime is not a no-egress sandbox.
- The repository contains no tracked GGUF, safetensors, ONNX, Core ML model,
  tokenizer, quantization, model directory, local engine source, artifact
  verifier, acquisition/removal mechanism, or candidate-specific ignore rule.

## Current-state evidence

- Before this plan was added, `HEAD`, local `main`, and the locally recorded
  `origin/main` were exactly
  `355d42ac8bb9a5ed663f57895675df688507ce9f`; ahead/behind was `0/0`, and the
  worktree was clean. No fetch or external query was performed.
- The baseline subject is `docs: close out PR 112 publication (#113)`. Local
  commit evidence shows that PR #113 published exactly the eight-file PR #112
  publication closeout. That closeout intentionally makes the five live
  records publication-stable and does not queue a recursive PR #113
  reconciliation.
- The ignored predecessor gate reports `complete`, `PASS WITH ADVISORIES`, and
  `valid: true` for
  `personal-assistant-v0-pr112-publication-closeout` before this plan changes
  the workspace fingerprint. Do not finalize, rebind, retarget, or mutate that
  historical gate.
- D-121 and this plan's prospective plan, increment, and review paths were
  unused at the clean baseline. A collision is a stop condition.
- Repository inspection confirms the current production absences and
  non-transferable fixture boundaries listed above.
- Baseline `npm run docs:check`, `npm run repository:check`,
  `npm run security:scan`, and `git diff --check` passed before this file was
  added.

## Files expected to change

This planning task changes only this plan. The later evidence increment has the
exact sixteen-path ceiling listed under Scope. No source, dependency,
configuration, workflow, hook, skill, script, generated output, model, artifact,
or external-system path may change.

## Affected components

Only documentation planning is affected now. A future assessment may describe,
but may not implement or exercise, these boundaries:

- fixed local-v2 candidate identity and provenance;
- engine dependency and native trusted-computing-base reachability;
- model/tokenizer/template artifact integrity and licensing;
- pre-provisioned or bundled acquisition and deterministic removal;
- one-run bounded streaming, resources, cancellation, cleanup, quarantine, and
  late-result contracts;
- Cortexa owner-authentication and local personal-data prerequisites; and
- a later separately authorized target-Mac evidence protocol.

## Interfaces and invariants

### Conceptual trust boundary

The only future topology this evidence plan may assess is conceptually:

```text
explicit authenticated owner action with bounded text
  -> future narrow versioned Tauri request
  -> Rust-owned fixed local-v2 run/profile/artifact snapshot
  -> one fixed in-process engine adapter and its native TCB
  -> untrusted events from one fixed verified artifact
  -> serialized Rust validator/reducer
  -> future bounded presentation DTO
```

This diagram is not an implemented interface. The WebView may eventually
provide only bounded text plus a separate current disclosure acknowledgment.
It may not choose or supply a run, request, agent, runtime, provider, engine,
model, artifact, path, tokenizer, template, quantization, backend, device,
thread count, limit, deadline, retry, fallback, or tool.

Trusted Rust must eventually issue and bind the run, attempt, request,
presentation, generation, and support correlations; immutable instructions;
the single fixed local profile and candidate identity; empty tools; artifact
manifest; limits; deadlines; lifecycle policy; disclosure version; and kill-
switch state. The engine and artifact remain untrusted inputs to that policy
boundary and receive no identity, authorization, configuration, selection, or
follow-on authority.

### Nomination-only research amendment

On 2026-09-03 the owner authorized one same-active-increment amendment before
envelope acceptance. It permits only current official public primary-source
research necessary to propose exactly one security-first fixed in-process
engine/model envelope. It does not authorize the full evidence matrix,
dependency resolution, source/archive retrieval, binary or model-artifact
download, build, build-script execution, parser/load/inference execution,
target-Mac inspection, candidate admission, or product work.

The nomination phase is closed as follows:

- exactly one candidate may be proposed;
- discovery is read-only and relies only on official owner/publisher sources;
  final supporting evidence and any later revisit are limited to the closed
  pages and immutable prefixes listed below;
- observed publisher/upstream metadata is nomination evidence only and must be
  revalidated during a separately authorized full assessment;
- a source conflict, moving-only identity, second candidate, redirect to
  artifact bytes, or need for wider research yields no acceptable envelope;
- no recommendation may be described as safe, compatible, performant,
  selectable, installed, implementation-ready, or admitted; and
- after the envelope is written, work stops for explicit owner acceptance.

The current-source review produced one proposed envelope because official
llama.cpp material exposes an in-process C API and pins release `v0.3.0` to a
full commit, while the official Qwen repository freezes one instruction-tuned
causal language model Q4_K_M GGUF at an immutable revision. This is a
nomination, not proof. In particular, llama.cpp's pinned public header states
that its decode abort callback currently works only with CPU execution. The
proposal therefore excludes Metal and every other accelerator rather than
treating GPU cancellation as proven. The broad upstream repository also
contains server, network, dynamic-backend, multimodal, and other surfaces;
excluding them is an unproved requirement for the later assessment, not an
observed containment result.

### Accepted immutable nomination envelope — frozen for assessment

| Nomination group    | Owner-accepted frozen value                                                                                                                                                                                                                                                                                                                                                                          |
| ------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Governance identity | Candidate ID `fixed_local_v2_llamacpp_0_3_0_qwen2_5_1_5b_q4km_cpu_v1`; documentation-only assessment candidate for fixed `real-content-v2`; no D-119 handle, selector, admission, fallback, or product authority                                                                                                                                                                                     |
| Engine coordinate   | Canonical project `ggml-org/llama.cpp`; release label `v0.3.0`; controlling full commit `c1d0e7a004015f23bc0233470b747b596f29b264`. The full commit wins over the tag or “latest” label and may not move.                                                                                                                                                                                            |
| Model coordinate    | Canonical publisher/project `Qwen/Qwen2.5-1.5B-Instruct-GGUF`; controlling repository revision `91cad51170dc346986eccefdc2dd33a9da36ead9`; sole intended file `qwen2.5-1.5b-instruct-q4_k_m.gguf`; publisher-listed SHA-256 `6a1a2eb6d15622bf3c96857206351ba97e1af16c30d7a74ee38970e434e9407e` and size `1,117,320,736` bytes must be revalidated before any artifact access.                        |
| Topology            | One statically linked in-process `llama` C API boundary; CPU backend only; Metal, Accelerate/BLAS, RPC, dynamic backend discovery/loading, server, CLI, tools, examples, app, multimodal path, HTTPS/OpenSSL/curl, helper, subprocess, plugin, JIT/dynamic code, alternate backend, retry, and fallback are forbidden. Exact build-flag closure remains unproved and belongs to the full assessment. |
| Target class        | Sanitized `macOS 14.0+`, arm64 Apple Silicon, minimum 16 GiB unified memory and 8 GiB available disk; Apple GPU present but unused; CPU-only execution. No target Mac was inspected, and compatibility, resources, latency, thermals, cancellation, and cleanup remain unproved.                                                                                                                     |
| Acquisition mode    | `pre_provisioned` only: a later separately authorized acquisition design may place exactly the verified file into one Rust-owned fixed slot before use. No runtime download/update/repair, file chooser, caller path, alternate source, cache search, or bundled redistribution is proposed.                                                                                                         |
| Source allowlist    | The closed official-primary-source allowlist below. It authorizes evidence-page access only after separate approval; no `/resolve/` URL, release asset, source archive, raw binary, Xet/LFS object, redirect target, model byte, package download, or artifact CDN is allowed.                                                                                                                       |

The accepted target class is a review threshold, not an observation about the
owner's device. The SHA-256 and byte count are publisher metadata, not a local
same-byte verification. The model's instruction-tuned causal-language-model
form, relatively small artifact, and permissive published license make it the
narrowest practical candidate found in this bounded review; whether its output
quality is usable is an inference to test later, not a publisher-backed or
target-Mac result.

The accepted build assertion is also frozen for verification, not
executed now. It expects `LLAMA_BUILD_IS_DEV=OFF`,
`LLAMA_USE_SYSTEM_GGML=OFF`, `BUILD_SHARED_LIBS=OFF`, and `GGML_CPU=ON`; it
expects `GGML_METAL`, `GGML_BLAS`, `GGML_ACCELERATE`, `GGML_OPENMP`,
`GGML_OPENMP_FETCH`, `GGML_LLAMAFILE`, `GGML_CPU_ALL_VARIANTS`,
`GGML_BACKEND_DL`, `GGML_RPC`, every non-CPU backend, `LLAMA_BUILD_COMMON`,
`LLAMA_BUILD_TESTS`, `LLAMA_BUILD_TOOLS`, `LLAMA_BUILD_EXAMPLES`,
`LLAMA_BUILD_SERVER`, `LLAMA_BUILD_APP`, `LLAMA_BUILD_UI`,
`LLAMA_USE_PREBUILT_UI`, `LLAMA_BUILD_MTMD`, `LLAMA_OPENSSL`,
`LLAMA_SUBPROCESS`, and `LLAMA_LLGUIDANCE` all to be `OFF`. No
`ggml_backend_load*` entry point may be called or remain reachable. The later
assessment must prove the exact option names, configured cache, compiled
objects, exported/imported symbols, link graph, and runtime reachability. A
missing option, default-enabled component, or surviving forbidden capability
rejects the candidate; it does not authorize a downstream patch.

### Closed official-primary-source allowlist

The following official sources support the proposed envelope and form the
maximum frozen external source boundary for the later static assessment.
GitHub access is restricted to the named project, exact release/commit,
immutable commit-scoped `blob/`, `tree/`, and raw-source prefixes, plus the
exact security pages. Hugging Face access is restricted to the two named Qwen
projects at the exact revisions and metadata pages below. No host-wide,
organization-wide, search, CDN, redirect, source archive, package, or
artifact-byte access is included.

| Official owner | Exact allowed source                                                                                                                                                                                                                                              | Nomination-only fact observed                                                                                                                                                             |
| -------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| ggml-org       | `https://api.github.com/repos/ggml-org/llama.cpp/releases/latest` and `https://github.com/ggml-org/llama.cpp/releases/tag/v0.3.0`                                                                                                                                 | Official current-release metadata and release page mapped `v0.3.0` to `c1d0e7a` on 2026-09-03; the release object is not immutable, so the full commit below controls.                    |
| ggml-org       | `https://github.com/ggml-org/llama.cpp/commit/c1d0e7a004015f23bc0233470b747b596f29b264`                                                                                                                                                                           | Exact engine commit exists.                                                                                                                                                               |
| ggml-org       | `https://github.com/ggml-org/llama.cpp/tree/c1d0e7a004015f23bc0233470b747b596f29b264` and only its immutable descendant `tree/` or `blob/` paths                                                                                                                  | Commit-scoped source, build, license, security, and API material is available for a later complete review.                                                                                |
| ggml-org       | `https://raw.githubusercontent.com/ggml-org/llama.cpp/c1d0e7a004015f23bc0233470b747b596f29b264/` and only exact source-text descendants of that immutable prefix                                                                                                  | Commit-pinned source text used during nomination remains bounded to the same engine coordinate; archives, executables, packages, and generated assets are excluded.                       |
| ggml-org       | `https://github.com/ggml-org/llama.cpp/blob/c1d0e7a004015f23bc0233470b747b596f29b264/include/llama.h`                                                                                                                                                             | In-process API exists; load progress and decode abort callbacks are exposed; the decode abort callback is documented as CPU-only.                                                         |
| ggml-org       | `https://github.com/ggml-org/llama.cpp/blob/c1d0e7a004015f23bc0233470b747b596f29b264/CMakeLists.txt` and `https://github.com/ggml-org/llama.cpp/blob/c1d0e7a004015f23bc0233470b747b596f29b264/src/CMakeLists.txt`                                                 | Build surfaces are configurable, but exact feature/dependency closure has not been established.                                                                                           |
| ggml-org       | `https://github.com/ggml-org/llama.cpp/blob/c1d0e7a004015f23bc0233470b747b596f29b264/ggml/CMakeLists.txt` and `https://github.com/ggml-org/llama.cpp/blob/c1d0e7a004015f23bc0233470b747b596f29b264/ggml/src/CMakeLists.txt`                                       | CPU/accelerator, static/dynamic, RPC, examples, tools, and other build switches exist; exact effective closure remains unproved.                                                          |
| ggml-org       | `https://github.com/ggml-org/llama.cpp/blob/c1d0e7a004015f23bc0233470b747b596f29b264/ggml/src/ggml-backend-reg.cpp` and `https://github.com/ggml-org/llama.cpp/blob/c1d0e7a004015f23bc0233470b747b596f29b264/ggml/src/ggml-backend-dl.cpp`                        | Pinned source includes backend registry and dynamic-loading implementation even when dynamic backend builds are disabled; absence or unreachability is a material unresolved blocker.     |
| ggml-org       | `https://github.com/ggml-org/llama.cpp/blob/c1d0e7a004015f23bc0233470b747b596f29b264/common/CMakeLists.txt` and `https://github.com/ggml-org/llama.cpp/blob/c1d0e7a004015f23bc0233470b747b596f29b264/tools/CMakeLists.txt`                                        | Download, HTTP, subprocess, server, and RPC-related components appear in excluded optional surfaces; their complete absence from the proposed library build remains unproved.             |
| ggml-org       | `https://github.com/ggml-org/llama.cpp/blob/c1d0e7a004015f23bc0233470b747b596f29b264/ggml/src/ggml-metal/ggml-metal-device.m`                                                                                                                                     | The excluded Metal implementation includes runtime compilation/fallback and background behavior, reinforcing the fixed CPU-only topology; Metal is not assessed as an alternate.          |
| ggml-org       | `https://github.com/ggml-org/llama.cpp/blob/c1d0e7a004015f23bc0233470b747b596f29b264/LICENSE`, `https://github.com/ggml-org/llama.cpp/blob/c1d0e7a004015f23bc0233470b747b596f29b264/SECURITY.md`, and `https://github.com/ggml-org/llama.cpp/security/advisories` | License/security pages exist; completeness, applicability, and advisory disposition are not yet assessed.                                                                                 |
| Qwen           | `https://huggingface.co/Qwen/Qwen2.5-1.5B-Instruct-GGUF/tree/91cad51170dc346986eccefdc2dd33a9da36ead9` and `https://huggingface.co/Qwen/Qwen2.5-1.5B-Instruct-GGUF/commit/91cad51170dc346986eccefdc2dd33a9da36ead9`                                               | Exact official GGUF repository revision exists and lists the intended file.                                                                                                               |
| Qwen           | `https://huggingface.co/Qwen/Qwen2.5-1.5B-Instruct-GGUF/blob/91cad51170dc346986eccefdc2dd33a9da36ead9/README.md`, `https://huggingface.co/Qwen/Qwen2.5-1.5B-Instruct-GGUF/blob/91cad51170dc346986eccefdc2dd33a9da36ead9/LICENSE`                                  | Publisher describes the model as instruction-tuned and Apache-2.0 licensed; full rights/provenance review remains pending.                                                                |
| Qwen           | `https://huggingface.co/Qwen/Qwen2.5-1.5B-Instruct-GGUF/blob/91cad51170dc346986eccefdc2dd33a9da36ead9/qwen2.5-1.5b-instruct-q4_k_m.gguf`                                                                                                                          | Metadata page reports the proposed filename, rounded size, and SHA-256; artifact bytes were not retrieved.                                                                                |
| Qwen           | `https://huggingface.co/Qwen/Qwen2.5-1.5B-Instruct-GGUF/commit/dd26da440ef0330c47919d1ecae0966d24022222`                                                                                                                                                          | Immutable artifact-introduction commit records the exact pointer byte count and digest used in the proposed envelope; those publisher values remain locally unverified.                   |
| Qwen           | `https://huggingface.co/Qwen/Qwen2.5-1.5B-Instruct/tree/989aa7980e4cf806f80c7fef2b1adb7bc71aa306` and `https://huggingface.co/Qwen/Qwen2.5-1.5B-Instruct/commit/989aa7980e4cf806f80c7fef2b1adb7bc71aa306`                                                         | Exact upstream non-GGUF model revision exists; its binding to the converted GGUF remains unproved.                                                                                        |
| Qwen           | `https://qwen.readthedocs.io/en/v2.5/run_locally/llama.cpp.html`                                                                                                                                                                                                  | Versioned official guidance describes using llama.cpp as a library and identifies Qwen's official GGUF publication; its CLI/server/download examples are expressly outside this topology. |

The frozen future artifact locator is
`https://huggingface.co/Qwen/Qwen2.5-1.5B-Instruct-GGUF/resolve/91cad51170dc346986eccefdc2dd33a9da36ead9/qwen2.5-1.5b-instruct-q4_k_m.gguf`.
It is recorded only to make the proposed `pre_provisioned` identity
unambiguous. It is not in the evidence-access allowlist and must not be followed
unless a later separately approved artifact-acquisition plan authorizes its
exact redirects, bytes, digest verification, custody, placement, and cleanup.

### Candidate nomination envelope and evidence-resolved tuple

Before the full candidate evidence assessment, the increment must record one
immutable nomination envelope and stop for owner acceptance. The narrow
nomination-only research amendment above is the sole exception to the original
pre-retrieval rule and supplies only a proposal. The envelope defines what may
later be assessed without pretending the assessment results are already known.

| Nomination group    | Required frozen fields                                                                                                                                              |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Governance identity | One documentation-only candidate ID; fixed `real-content-v2` purpose; no D-119 handle or product selector                                                           |
| Engine coordinate   | Canonical upstream project and exact intended release, tag, or commit coordinate supplied for verification                                                          |
| Model coordinate    | Canonical upstream owner/project and exact intended immutable model revision supplied for verification                                                              |
| Topology            | In-process-only engine; no helper, external executable, local server, plugin, JIT, dynamic runtime, alternate backend, retry, or fallback                           |
| Target class        | Sanitized macOS version, architecture, CPU/GPU family, RAM, and available-disk class; no serial number, account name, home path, or device identifier               |
| Acquisition mode    | Exactly one proposed `bundled` or `pre_provisioned` mode; no runtime download, update, repair, alternate source, or caller-selected path                            |
| Source allowlist    | Exact canonical domains and URLs permitted for the separately authorized upstream, registry, release, model-card, license, notice, advisory, and metadata retrieval |

Every nomination-envelope field is mandatory. `unknown`, `latest`, a version
range, a moving branch, a mutable registry tag without a controlling immutable
commit, an alternate backend, an optional fallback, a second candidate, or an
open-ended source domain fails closed. Owner acceptance of the envelope
authorizes neither further retrieval nor a product choice; the full
candidate-specific read-only assessment still requires its own exact
authority.

After that authority is granted, the future increment must resolve and
cross-check the following complete tuple from only the accepted source
allowlist:

| Resolved group        | Required evidence-resolved fields                                                                                                                                                                                                                        |
| --------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Governance identity   | Exact accepted nomination-envelope values and evidence retrieval timestamp; no changed candidate coordinate                                                                                                                                              |
| Engine source         | Verified exact release/tag and commit; source archive/tree digest; registry/native package versions and checksums; support/maintenance state                                                                                                             |
| Engine build          | Exact target triple; default-feature disposition; complete enabled/disabled feature set; build flags; build scripts; proc macros; unsafe/FFI surfaces; native libraries; dynamic libraries; GPU/accelerator backend; plugin/JIT/dynamic-code disposition |
| Model identity        | Verified immutable revision; architecture; exact format and version; quantization; context limit; compatibility declaration                                                                                                                              |
| Artifact manifest     | Every required model, tokenizer, vocabulary, configuration, prompt/chat-template, metadata, and auxiliary filename; exact byte size; publisher SHA-256 digest; role; parser/loader version                                                               |
| Provenance and rights | Canonical origin; publisher identity evidence; chain of custody; engine and every artifact license; personal-use rights; modification terms; attribution; redistribution/bundling rights kept distinct                                                   |
| Acquisition           | Exact design for the accepted `bundled` or `pre_provisioned` mode; application-owned fixed-slot policy; verifier-before-parser order; no caller path                                                                                                     |
| Removal               | Rust-owned logical removal owner; complete artifact/cache/temp/orphan inventory; missing/corrupt/substituted disposition; no network repair or replacement                                                                                               |
| Resource envelope     | Maximum artifact bytes, context/tokens, input/output/events/deltas/queues, RAM/VRAM, disk/temp, threads, CPU/GPU use, load time, first-event time, idle time, total run time, cancel/join time, and teardown time                                        |
| Target compatibility  | Exact engine/model/artifact compatibility evidence for the accepted sanitized target class                                                                                                                                                               |

Neither record is a product selection mechanism. The resolved tuple must match
the accepted nomination envelope exactly. Any identity, coordinate, source,
topology, target, acquisition-mode, dependency, feature, file, digest, license,
backend, or allowlist mismatch selects `evidence_boundary_failed`; the
increment may not mutate the envelope or substitute an alternative.

### Evidence vocabulary and closed dispositions

Each evidence row must use exactly one status:

- `documented`: frozen primary or repository evidence satisfies the exact row;
- `contract_unproven`: required proof is missing, ambiguous, contradictory, or
  does not cover the complete frozen candidate;
- `not_run`: an operational check is deliberately outside this documentation
  increment; or
- `boundary_failed`: scope, integrity, authority, source freshness, candidate
  identity, or evidence custody failed.

After completing the authorized static assessment, the increment must propose
exactly one of these closed dispositions and stop for owner acceptance:

| Disposition                                                   | Meaning                                                                                                                                                                                                                        |
| ------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `static_evidence_sufficient_for_separate_target_mac_planning` | Every mandatory static row is `documented`; a separately approved synthetic-only target-Mac evidence plan may be drafted. No candidate/profile is admitted and no artifact, filesystem, source, or execution authority exists. |
| `candidate_not_eligible_or_unproven`                          | At least one candidate property is affirmatively incompatible or remains unproved. The candidate stays unavailable; no substitute, fallback, artifact acquisition, target-Mac run, or implementation may begin.                |
| `evidence_boundary_failed`                                    | Baseline, candidate identity, source custody/freshness, scope, preservation, or evidence integrity failed. No candidate conclusion or successor authority may be inferred.                                                     |

There is no partial, provisional, assumed-safe, implementation-ready,
profile-admitted, or default-open result. Missing or ambiguous evidence cannot
select the positive disposition. D-121 may be added only after the owner
accepts the exact proposed disposition and wording.

### Required static evidence matrix

The future evidence package must be conjunctive. One passing row cannot offset
another unproved or failed row.

| Area                       | Required evidence                                                                                                                                                                                    | Positive criterion                                                                                                                                                                                             | Fail-closed result                                                                                                                              |
| -------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| Exact engine identity      | Immutable upstream/release/commit/source digest plus exact package/native versions and build tuple                                                                                                   | Every identity is canonical, immutable, mutually consistent, and reproducible                                                                                                                                  | Drift, moving identity, alternate backend, or unverifiable source rejects the candidate                                                         |
| Exact model identity       | Immutable revision plus complete artifact manifest, format, quantization, tokenizer, config, and prompt/chat template                                                                                | Every byte-bearing input has a fixed size, SHA-256 digest, parser, and compatibility mapping                                                                                                                   | Missing or mutable file, digest, parser, template, or compatibility proof rejects the candidate                                                 |
| Dependency closure         | Complete direct/transitive/default/optional/build/proc-macro/native/dynamic graph for the exact target and features                                                                                  | No unresolved node, feature, build script, unsafe/FFI edge, native binary, plugin, JIT, or dynamic loader remains                                                                                              | Incomplete closure or unexpected executable/dynamic behavior rejects the candidate and may require a separate containment decision              |
| Provenance and licensing   | Official upstream, registry, model-card, release, license, notice, advisory, and chain-of-custody evidence                                                                                           | Personal use is permitted; modification/attribution/removal are understood; redistribution/bundling is separately classified                                                                                   | Unclear provenance, license, rights, notices, or advisory impact rejects the candidate                                                          |
| Artifact verification      | Verifier-before-parser design, fixed app-owned slot, size and digest verification on the same bytes ultimately loaded, permissions and link checks                                                   | No time-of-check/time-of-use gap; no caller path, symlink/hardlink substitution, archive expansion, metadata authority, or parser-before-verifier path                                                         | If the engine must reopen an unbound path or same-byte loading cannot be proved, reject and require a separate boundary decision                |
| Acquisition and removal    | Exact bundled or pre-provisioned lifecycle, staging verification, promotion, rollback, removal, orphan/cache/temp inventory, and ownership                                                           | No runtime network, updater, repair, alternate source, silent replacement, or residual content outside the declared inventory                                                                                  | Any acquisition authority not separately approved, ambiguous removal ownership, or undeclared residue rejects the candidate                     |
| Complete no-egress         | Source and feature review across engine, dependencies, artifact metadata, acquisition, load, inference, cancellation, cleanup, telemetry, crash/support, update, licensing, embeddings, and fallback | No DNS, socket, proxy, HTTP/TLS, localhost/server, Unix-socket-to-service, SDK, telemetry, analytics, update, license, crash upload, remote embedding, or fallback edge exists or can be enabled               | Any reachable, configurable, ambiguous, or transitive egress edge rejects the candidate; target observation alone cannot cure it                |
| Native TCB reachability    | Exact same-process privileges and calls available to every Rust/native/FFI component                                                                                                                 | Source/features exclude unrelated filesystem and Keychain/account/environment/default-credential/gateway/provider/certificate/private-key/signing/secret-memory access; engine has no policy or auth authority | If network or unrelated secret reachability cannot be excluded, remain Blocked pending a separately accepted containment/isolation architecture |
| Streaming and bounds       | Candidate APIs and source prove incremental events, backpressure, UTF-8 handling, and enforceable pre-allocation ceilings                                                                            | Candidate fits no-wider-than-current text/event bounds and an exact candidate-specific RAM/VRAM/disk/thread/CPU/GPU/time envelope                                                                              | Whole-answer-only, unbounded allocation/queue/context/output/resource behavior, or caller-selected limits rejects the candidate                 |
| Deadlines and cancellation | Source-level ownership model for initialization, verification, load, tokenize, prefill, first event, idle, generation, callback, finalization, and teardown                                          | Cancellation closes ingress once, signals only original work, and boundedly joins every thread/callback/GPU queue/mapping/buffer/support owner                                                                 | Timer expiry, dropped future, UI terminal state, or a cooperative flag without bounded join is insufficient and rejects the candidate           |
| Cleanup and quarantine     | Complete owner graph and positive quiescence proof for every success/failure/cancel/timeout/drop phase                                                                                               | Rust retains ownership until all work is quiescent; ambiguity blocks retry/restart/replacement/removal until proof or process termination                                                                      | Any detached owner, orphan, unbounded teardown, or early ownership release rejects the candidate                                                |
| Late-result rejection      | Serialized result ingress keyed by Rust-issued run/attempt/request/generation/sequence and terminal state                                                                                            | Malformed, oversized, duplicate, foreign, stale, gapped, out-of-order, and every post-terminal result is rejected before mutation                                                                              | Filtering after UI/state/log/cache/evidence mutation, or using filtering as termination proof, rejects the candidate                            |
| Owner authentication       | D-062/D-094 prerequisite and future Rust-owned admission point remain explicit and separate from provider auth                                                                                       | No personal request can start without a separately accepted non-demo owner-authentication contract                                                                                                             | OS login, device ownership, local/no-auth, or process identity inferred as authentication keeps real personal use Blocked                       |
| Personal-data lifecycle    | Exact data class, disclosure, volatility, redaction, cache/temp/crash/swap limitations, deletion/removal, incident response, and support logging                                                     | Prompt/output remain bounded and volatile; logs are content-free; every unavoidable residue and limitation is disclosed and gated                                                                              | Undeclared persistence, crash/support upload, cache/temp residue, or unbounded sensitive retention rejects the candidate                        |
| Target-Mac protocol        | Separately approved synthetic-only measurements and observations tied to the exact frozen tuple and sanitized target class                                                                           | All required checks are reproducible and recorded Passed/Failed/Not run without sensitive identifiers                                                                                                          | No operational check runs in this increment; missing later target evidence keeps implementation and personal use Blocked                        |

### Owner-accepted full static assessment checkpoint

The owner accepted the immutable nomination envelope exactly as recorded and
separately authorized one full documentation-only static assessment. Evidence
was retrieved at `2026-09-04T03:11:43Z` only from the frozen allowlist above.
No candidate coordinate, topology, target class, acquisition mode, or source
boundary changed. No artifact locator or redirect was followed.

The accepted source boundary remained intact. One repeat read of the frozen
Qwen README returned HTTP 429; no wider URL, raw endpoint, cache, mirror, or
download workaround was used. Previously recorded and independently
cross-checked official metadata remains usable only for the facts it actually
states. The failed page read leaves any card-only or artifact-internal property
`contract_unproven`; it is not an identity or custody mismatch and therefore
does not select `evidence_boundary_failed`.

Disposable metadata-only dependency resolution was `not_run`. The accepted
envelope names upstream C/C++ source and a direct C API, but no Rust binding,
registry package, package checksum, vendoring method, or build-integration
coordinate. Inventing one would expand or mutate the envelope. Resolution also
could not cure the pinned source's decisive forbidden dynamic-loader surface,
so it was not necessary to reach the closed negative disposition.

#### Matching evidence-resolved tuple

| Resolved group        | Status              | Evidence-resolved result                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| --------------------- | ------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Governance identity   | `documented`        | The accepted candidate remains exactly `fixed_local_v2_llamacpp_0_3_0_qwen2_5_1_5b_q4km_cpu_v1`, for fixed nonselectable `real-content-v2`, with the accepted CPU-only in-process topology, sanitized target class, `pre_provisioned` mode, and closed allowlist. No D-119 handle, admission, retry, fallback, or coordinate changed.                                                                                                                                                                                                                                            |
| Engine source         | `contract_unproven` | Official release metadata still maps `v0.3.0` to full commit `c1d0e7a004015f23bc0233470b747b596f29b264`, and the commit-pinned root declares llama.cpp `0.3.0` plus embedded ggml `0.22.0`. The release object remains mutable. No source archive/tree digest, native package coordinate/checksum, exact support window, or reproducible source package exists in the authorized evidence. No archive was retrieved.                                                                                                                                                             |
| Engine build          | `contract_unproven` | Static `llama`/ggml plus CPU-only configuration is expressible only as an unexecuted assertion. A top-level standalone macOS configure defaults to shared libraries, Metal, BLAS, Accelerate, native CPU tuning, CPU repacking, ccache, common utilities, tools/examples/server/app, UI, OpenSSL, and subprocess unless explicitly closed; embedded/subdirectory defaults differ, and the exact integration mode remains unproved. No configured cache, compiler/SDK/target tuple, source/object inventory, link map, symbol table, build-script closure, or FFI wrapper exists. |
| Model identity        | `contract_unproven` | The official Qwen repository revision remains `91cad51170dc346986eccefdc2dd33a9da36ead9`; the intended Q4_K_M filename, publisher SHA-256 `6a1a2eb6d15622bf3c96857206351ba97e1af16c30d7a74ee38970e434e9407e`, and exact pointer size `1,117,320,736` agree with artifact-introduction commit `dd26da440ef0330c47919d1ecae0966d24022222`. Exact GGUF format version and byte-derived identity were not observed.                                                                                                                                                                  |
| Artifact manifest     | `contract_unproven` | Qwen states that GGUF generally contains weights, hyperparameters, generation configuration, and tokenizer data, but the exact frozen file was not parsed. Its tensor inventory, architecture/config values, tokenizer/vocabulary digests, special-token IDs, prompt/chat template, metadata, auxiliary-file sufficiency, parser version, and compatibility mapping to the pinned engine remain unknown.                                                                                                                                                                         |
| Provenance and rights | `contract_unproven` | The pinned engine publishes MIT terms and the Qwen repository publishes Apache-2.0 metadata/terms that permit personal use subject to their conditions. No exact source-weight-to-GGUF conversion chain, converter/quantizer commit, command/settings, source-weight digest set, reproducibility record, SBOM, signature, attestation, complete inherited notice review, or Cortexa bundling classification exists. The cited upstream revision postdates the GGUF upload and is not bound to it.                                                                                |
| Acquisition           | `contract_unproven` | `pre_provisioned` remains the sole accepted mode, but no authorized source transport, staging location, fixed-slot identity, size/digest verifier, permissions/ownership/link policy, verifier-before-parser ordering, same-byte loader binding, promotion, rollback, or quarantine design exists. Publisher download instructions are evidence only and are forbidden as runtime acquisition behavior.                                                                                                                                                                          |
| Removal               | `contract_unproven` | No Rust owner, mapped-file lifecycle, cache/temp/orphan inventory, deletion order, missing/corrupt/substituted disposition, or positive quiescence rule has been bound to the candidate. Automatic repair, redownload, replacement, or fallback remains forbidden.                                                                                                                                                                                                                                                                                                               |
| Resource envelope     | `contract_unproven` | The publisher byte count and general model-card context ceilings are metadata, not enforced limits. Candidate-specific RAM, KV-cache, staging/temp/disk, allocation, queue, thread, CPU, load, first-event, idle, generation, total, cancellation, bounded-join, cleanup, and teardown ceilings remain unspecified and untested.                                                                                                                                                                                                                                                 |
| Target compatibility  | `not_run`           | Official guidance supports Qwen2.5 with llama.cpp and discusses Apple Silicon generally, but it does not bind this exact GGUF to commit `c1d0e7a...`, the required CPU-only static configuration, or the sanitized macOS 14 arm64 class. No source build, parse, load, inference, benchmark, target inspection, or target-Mac observation ran.                                                                                                                                                                                                                                   |

The tuple matches the accepted identity envelope. Missing proof and an
affirmatively incompatible source property reject this candidate; neither is
candidate drift or an evidence-custody failure.

#### Completed conjunctive static evidence matrix

| Area                       | Status              | Static finding and fail-closed consequence                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| -------------------------- | ------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Exact engine identity      | `contract_unproven` | Release and commit identity are documented, but source/tree digest, package/native versions and checksums, exact target/compiler/SDK, and reproducible build tuple are absent. A mutable release label cannot replace the commit, and the commit alone does not satisfy the complete row.                                                                                                                                                                                              |
| Exact model identity       | `contract_unproven` | Revision, intended filename, quantization label, publisher digest, and byte count are documented metadata. Exact GGUF version, complete embedded manifest, template/tokenizer identity, byte verification, parser mapping, and pinned-engine compatibility are absent.                                                                                                                                                                                                                 |
| Dependency closure         | `contract_unproven` | The target-specific direct/transitive/native/build graph was not resolved. More decisively, pinned `ggml/src/CMakeLists.txt` always compiles `ggml-backend-dl.cpp` and `ggml-backend-reg.cpp` into `ggml`; `GGML_BACKEND_DL=OFF` changes backend construction but does not remove that loader implementation. The public API still exposes `ggml_backend_load`, `ggml_backend_load_all`, and `ggml_backend_load_all_from_path`. This fails the accepted no-reachable-loader criterion. |
| Provenance and licensing   | `contract_unproven` | MIT and Apache-2.0 identities are documented, but exact conversion provenance, attestation, complete native/transitive notices, advisory applicability, and bundling/redistribution classification are incomplete. The allowlisted advisory index lists parser and model-load issues, but the frozen boundary does not establish their exact applicability or remediation state for this tuple.                                                                                        |
| Artifact verification      | `contract_unproven` | Publisher metadata is not a local digest of acquired bytes. Although the C API includes a `FILE *` loader, no fixed-slot, regular-file/link, ownership, same-open-object digest/load, mutation exclusion, or other TOCTOU-safe design proves that the parser consumes the exact verified bytes.                                                                                                                                                                                        |
| Acquisition and removal    | `contract_unproven` | `pre_provisioned` is fixed, but staging, verification, promotion, ownership, rollback, quarantine, mapped-memory release, cache/temp/orphan inventory, and removal remain undesigned. No acquisition or filesystem authority exists.                                                                                                                                                                                                                                                   |
| Complete no-egress         | `contract_unproven` | The proposed flags exclude common/server HTTP and RPC surfaces only on paper. Pinned registry code can load a caller path and can scan the executable/current directories plus `GGML_BACKEND_PATH`; a loaded native module could add arbitrary behavior. No configured graph, symbol closure, syscall containment, or complete lifecycle proof excludes DNS/socket/proxy/telemetry/update/license/crash/embedding/fallback egress.                                                     |
| Native TCB reachability    | `contract_unproven` | The C/C++ parser and engine execute in the Cortexa process with its memory and ambient user privileges. Pinned loader and filesystem APIs remain available, upstream security guidance recommends isolating untrusted models, and no accepted sandbox or capability boundary excludes unrelated filesystem, Keychain/account, environment/default-credential, gateway/provider, certificate/private-key/signing, or secret-memory reachability.                                        |
| Streaming and bounds       | `contract_unproven` | The C API permits iterative decode/sampling and exposes context, batch, output, and thread parameters, but no Rust adapter, serialized bounded event ingress, backpressure, UTF-8 assembly, pre-allocation enforcement, queue ceiling, candidate-specific resource envelope, or caller-authority exclusion exists.                                                                                                                                                                     |
| Deadlines and cancellation | `contract_unproven` | Model-load progress can cooperatively abort, and a decode abort callback is documented for CPU execution. That does not cover all phases or prove a hard deadline, original-work signaling, bounded join, teardown, or quiescence. Aborted decode may leave processed ubatches in context, and `llama_synchronize` has no documented time bound.                                                                                                                                       |
| Cleanup and quarantine     | `contract_unproven` | No complete Rust/native owner graph or bounded teardown proof exists. Pinned registry source warns dynamically loaded backend threads may still be running when resources would otherwise unload. No candidate-specific success/failure/cancel/timeout/drop path proves positive quiescence or retains a quarantine owner until process termination.                                                                                                                                   |
| Late-result rejection      | `contract_unproven` | Existing Personal Assistant fixture logic is non-transferable. The candidate has no Rust-issued run/attempt/request/generation/sequence binding or serialized adapter ingress that rejects malformed, oversized, foreign, duplicate, stale, gapped, out-of-order, or post-terminal events before every mutation. Filtering would not prove engine termination.                                                                                                                         |
| Owner authentication       | `contract_unproven` | D-062/D-094 keep non-demo Cortexa owner authentication mandatory and Blocked. Local/no-auth describes provider authentication only; personal control of a Mac, OS login, or process identity is not accepted admission proof.                                                                                                                                                                                                                                                          |
| Personal-data lifecycle    | `contract_unproven` | No accepted disclosure, prompt/output buffer lifecycle, content-free logging proof, allocator/mapping/swap/cache/temp/crash/support boundary, deletion/removal design, or incident response exists for this engine and artifact.                                                                                                                                                                                                                                                       |
| Target-Mac protocol        | `not_run`           | Every target-Mac, artifact-byte, build, parse, load, inference, performance, resource, cancellation, cleanup, no-egress, filesystem, permission, accessibility, and UI observation remained outside this documentation-only authorization.                                                                                                                                                                                                                                             |

Matrix totals are exactly zero `documented`, fourteen `contract_unproven`, one
`not_run`, and zero `boundary_failed`. Operational source build, artifact-byte
verification, parse/load/inference, target inspection, and target-Mac checks
were all `not_run`. The negative result is conjunctive: the forbidden loader
capability is independently decisive, while every other unproved mandatory row
also prevents the positive disposition.

#### Accepted closed disposition

The owner accepted exactly `candidate_not_eligible_or_unproven`.

The source identity and evidence boundary were preserved, so
`evidence_boundary_failed` would be inaccurate. The positive disposition is
unavailable because no mandatory static row is fully `documented`, and the
pinned engine's compiled/public dynamic-loader surface conflicts with the
accepted topology. This disposition admits no candidate, authorizes no patch or
substitute, and does not permit target-Mac planning, acquisition, build,
implementation, or personal prompts.

#### Accepted D-121 wording

The owner accepted the following text verbatim. Final reconciliation appends it
once to `DECISIONS.md` without altering any preceding decision bytes:

```markdown
## D-121 - Do not admit the frozen llama.cpp and Qwen local-v2 candidate from current static evidence

Date: 2026-09-03
Status: Accepted documentation-only static-evidence decision; no candidate admitted and no implementation authority

### Context

D-120 permits one separately approved documentation assessment of a fixed,
nonselectable local/no-auth candidate for `real-content-v2`. The owner accepted
the immutable envelope
`fixed_local_v2_llamacpp_0_3_0_qwen2_5_1_5b_q4km_cpu_v1`: llama.cpp `v0.3.0`
at full commit `c1d0e7a004015f23bc0233470b747b596f29b264`, official Qwen2.5 1.5B
Instruct Q4_K_M GGUF at revision
`91cad51170dc346986eccefdc2dd33a9da36ead9`, one static in-process CPU-only
topology, a sanitized macOS 14 arm64 target class, `pre_provisioned` acquisition,
and one closed official-source allowlist.

The full static assessment preserved that envelope and its evidence boundary.
It retrieved no source archive, package, binary, model byte, artifact redirect,
credential, or operational evidence; built and executed nothing; and did not
inspect the target Mac. Isolated dependency resolution was not run because the
envelope names no Rust/package binding and the pinned source already fails a
mandatory topology criterion.

### Decision

Select `candidate_not_eligible_or_unproven`.

The exact engine release/commit and model revision/filename/publisher digest and
size metadata match the accepted envelope, but none of the fifteen conjunctive
matrix rows is fully documented: fourteen are `contract_unproven`, the target-
Mac row is `not_run`, and no row is `boundary_failed`.

Pinned ggml source always compiles `ggml-backend-dl.cpp` and
`ggml-backend-reg.cpp` into the core library. Setting `GGML_BACKEND_DL=OFF`
changes whether selected backends are modules; it does not remove the dynamic-
loader implementation or the public `ggml_backend_load`,
`ggml_backend_load_all`, and `ggml_backend_load_all_from_path` entry points.
The loader can open an explicit path, scan executable/current directories, and
honor `GGML_BACKEND_PATH`. That reachable capability conflicts with the
accepted no-dynamic-loading topology. No downstream source patch is authorized.

The evidence also does not establish a complete target build/dependency graph,
model conversion provenance and exact embedded manifest, same-byte artifact
verification, acquisition/removal ownership, complete no-egress or native-TCB
containment, bounded resources, hard deadlines and bounded joins, positive
cleanup quiescence, pre-mutation late-result rejection, Cortexa owner
authentication, personal-data lifecycle, exact target compatibility, or any
target-Mac result. Load/decode callbacks are cooperative partial mechanisms,
not proof of the required cancellation and cleanup contract.

### Consequences

The frozen candidate remains unavailable and is not selected, admitted,
installed, or implementation-ready. No replacement, fallback, source patch,
artifact acquisition, filesystem boundary, target-Mac plan, Rust/TypeScript
source work, Tauri/UI work, owner-authentication work, personal prompt, or
operational successor may begin from this decision. A future reconsideration
requires a separately owner-approved architecture or candidate plan; it may not
silently mutate this envelope or reuse its negative evidence as positive proof.

### Preservation

D-094 synthetic-v1, historical V0-14, D-118 `no_eligible_client`, D-119's
exactly ten `candidate_blocked` entries and no handle, D-120, V0-3/V0-7, all
ten D-107 blockers, D-113 through D-117 Proposed/non-controlling status, the
sole/default Native runtime, empty tools, explicit foreground action,
volatility, no fallback, no-egress, no device effect, and every operational
`Blocked` boundary remain unchanged. Fixed local-v2 is not D-119
`local_no_auth` admission. D-060/D-061 continue to control cloud paths, and
D-062/D-094 owner authentication remains separately mandatory and Blocked.

### Supersedes or is superseded by

D-121 dispositions only the exact D-120-authorized frozen candidate assessment.
It does not supersede D-060, D-061, D-062, D-094, D-107, D-118, D-119, D-120,
synthetic-v1, historical V0-14, or any security, testing, or readiness gate.
```

### Streaming and resource invariants

The future assessment must preserve the current Personal Assistant bounds as
hard maximum ceilings unless a separately accepted decision narrows them:

- input: 4,096 Unicode scalar values and 16,384 UTF-8 bytes;
- events/presentation sequence: 128;
- one delta: 1,024 Unicode scalar values and 4,096 UTF-8 bytes;
- final output: 8,192 Unicode scalar values and 32,768 UTF-8 bytes; and
- presentation update batch: 16.

The candidate record must add exact enforceable limits for context tokens,
generated tokens, engine queues, allocation before and during load, RAM, VRAM,
temporary and final disk, worker threads, CPU/GPU utilization policy,
initialization, artifact verification, model open, tokenization, prefill,
first-event, stream idle, generation, total run, cancellation, join, cleanup,
and teardown. A limit that can only be observed after over-allocation is not an
enforced bound.

There remains one explicit foreground request process-wide, one immutable
candidate snapshot, no automatic retry, no fallback, no delegation, no tools,
no memory, no persistence, no scheduling, no background autonomy, and no
follow-on action.

### Cancellation, cleanup, and late-result invariants

1. Trusted Rust owns one serialized ingress-close linearization point.
2. Cancellation is terminal and idempotent, targets the original work only,
   and cannot issue a second inference request.
3. Cancellation must reach initialization, artifact open/load, tokenization,
   prefill, generation, native callback, accelerator work, finalization, and
   teardown wherever those phases can own work.
4. A deadline, returned error, UI `Cancelled` state, dropped future, closed
   channel, or late-result filter does not prove the engine stopped.
5. Every engine thread, native callback, accelerator/GPU queue, memory mapping,
   buffer, and support task must stop or join within its exact bound.
6. Cleanup ownership remains private to Rust through positive quiescence.
   Ambiguity quarantines the complete owner graph and denies retry, fallback,
   restart, replacement, update, removal, and new admission. If quiescence is
   never proved, ownership remains until process termination.
7. Result ingress validates exact Rust-issued identity, generation, sequence,
   size, transition, and terminal status before any state, UI, IPC, log,
   evidence, cache, persistence, retry, removal, or follow-on mutation.
8. Late-result rejection is not cancellation, cleanup, or absence of platform
   effects; all three contracts require independent evidence.

### Acquisition and removal invariants

- The app may eventually accept only one fixed manifest from trusted Rust
  source, never a caller-supplied path or model metadata.
- The allowed future acquisition mode is exactly `bundled` or
  `pre_provisioned`, chosen by a later accepted decision. Neither mode is
  authorized here.
- Staging, size/digest verification, permissions, ownership, regular-file and
  link checks, same-byte loader binding, promotion, use, quarantine, rollback,
  removal, and orphan cleanup must have one explicit Rust owner.
- Missing, corrupt, substituted, incompatible, unlicensed, unreadable, or
  drifted artifacts produce a closed redacted unavailable result. They never
  trigger a download, repair, update, alternate path, different model, cloud
  request, provider profile, or user-selected replacement.
- Archive expansion, executable content, remote code, dynamic plugins, model-
  supplied scripts, arbitrary templates, and metadata-directed configuration
  are prohibited unless a separate architecture and security decision is
  accepted first.
- Error and support DTOs may contain only a closed code and bounded opaque
  Rust-issued correlation. They may not expose prompts, outputs, model paths,
  artifact names/digests, native errors, stacks, device/account identifiers, or
  filesystem layout.

### Target-Mac evidence protocol — future and Not run

No target-Mac engine or artifact operation is authorized by this plan. A later
separately approved plan must use only nonpersonal synthetic canaries and bind
every observation to the accepted immutable candidate tuple and sanitized
target class. It must record each item as Passed, Failed, Pending, or Not run:

- repository-pinned toolchain and exact OS/architecture/CPU/GPU/RAM/disk class;
- fixed artifact manifest, pre-open size/digest, permissions, ownership,
  regular-file/link rejection, same-byte load, missing/corrupt/substituted
  rejection, and no arbitrary path;
- cold and warm model open, first-event latency, stream idle, total latency,
  final answer, and deterministic closed failure;
- peak and terminal RAM/VRAM, disk/temp/cache, thread count, CPU/GPU use,
  thermal behavior, and resource-limit rejection;
- cancellation during staging verification, model open, tokenization, prefill,
  first-event wait, generation, callback/accelerator work, finalization, and
  teardown, with measured bounded join and positive quiescence;
- malformed, oversized, duplicate, foreign, stale-generation, gapped,
  out-of-order, post-cancel, post-timeout, post-failure, and post-success result
  rejection before mutation;
- restart only after proved quiescence, plus restart denial while ownership is
  quarantined;
- static-source evidence plus runtime observation of no DNS, socket, proxy,
  localhost service, telemetry, updater, license check, remote embedding,
  crash/support upload, or fallback;
- before/after filesystem, log, cache, temporary, crash-report, permission-
  prompt, and device-effect inventory, including truthful limits on swap and
  OS-managed behavior;
- disclosure, owner-authentication dependency, kill switch, removal, orphan
  cleanup, rollback, and re-verification after relaunch; and
- future native UI focus, chronology, recovery, resize, 200% zoom, theme,
  reduced-motion, scrolling, and console/log redaction only after a separate
  Tauri/UI plan exists.

Target-Mac non-observation cannot prove universal absence of source,
dependency, native, telemetry, or secret-reachability paths. A source pass
cannot substitute for target observation. Both remain required before any
source implementation or personal prompt activation can be considered.

## Implementation milestones

- [x] Confirm the clean synchronized planning baseline, published PR #113
      closeout, valid completed predecessor, unused D-121 and artifact paths,
      current source/dependency absences, and documentation-tier baseline.
- [x] Draft this exact one-file Ready ExecPlan without creating a branch,
      beginning a gate, selecting a candidate, or accessing an external system.
- [x] Obtain explicit owner approval for this exact staged documentation
      evidence increment.
- [x] Reconfirm clean synchronized `main`, exact plan-only delta, valid
      predecessor state, D-121/path availability, and controlling decisions.
- [x] Create the approved `codex/` branch and run the prospective `begin`
      command exactly once. Do not retrieve evidence or name a candidate during
      that act.
- [x] Obtain a same-active-increment owner amendment for one bounded
      nomination-only official-primary-source research phase. Do not extend it
      into the full evidence assessment.
- [x] Record exactly one complete proposed nomination envelope from that
      bounded research and stop for owner acceptance. The proposal selects or
      admits nothing.
- [x] After the owner accepts the envelope and separately authorizes retrieval,
      conduct the full assessment only against its exact public official source
      allowlist and, if needed, perform isolated disposable metadata-only
      dependency resolution. Do not retrieve binaries/model bytes, compile,
      run build scripts, execute the candidate, or copy resolver artifacts into
      the repository.
- [x] Complete the evidence-resolved tuple and static matrix for only the
      accepted envelope. Any mismatch selects `evidence_boundary_failed` rather
      than mutating the envelope. Record every operational and target-Mac row
      `not_run`, propose exactly one closed disposition, and stop for owner
      acceptance of the tuple, matrix, disposition, and D-121 wording.
- [x] Only after the owner accepts that exact disposition and wording, add
      accepted D-121 and reconcile the fifteen pre-report paths within the
      sixteen-file ceiling. A negative result records the blocker without
      nominating a replacement.
- [x] Run the full documentation, repository, security, complete verification,
      exact-scope, preservation, independent-review, session, quality,
      report-validation, and post-increment gates.
- [x] Verify a complete valid marker and stop for owner review without
      publication, artifact acquisition, target-Mac execution, source work, or
      successor start.

## Security and privacy considerations

### Threat model

| Threat                                  | Fail-closed treatment                                                                                                                                                                                 |
| --------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Milestone laundering                    | State that local evidence neither completes nor replaces synthetic-v1 and cannot activate personal prompts                                                                                            |
| Catalog laundering                      | Keep fixed local-v2 outside D-119; preserve ten blocked entries and no handle                                                                                                                         |
| D-118 evasion                           | Reject localhost, helper, SDK, WebView, private client, updater, hidden socket, or cloud fallback as an undeclared transport                                                                          |
| Candidate substitution                  | Freeze one tuple; reject any alternate engine/model/backend/version/file/digest/feature/hardware or fallback                                                                                          |
| Artifact substitution and parser attack | Verify size/digest/ownership/link state before parsing; require same-byte load and treat parser/native code as untrusted TCB input                                                                    |
| Supply-chain execution                  | Inventory build scripts, unsafe/FFI, native code, proc macros, dynamic libraries, plugins, JIT, and advisories without building or executing them                                                     |
| Hidden egress                           | Require complete source/dependency and target evidence across acquisition, load, infer, cancel, cleanup, telemetry, crash, update, licensing, embeddings, and fallback                                |
| In-process privilege collapse           | Treat engine/native code as same-process TCB; require exclusion of network, unrelated filesystem, Keychain/account, credential, signing, and secret-memory reachability or reject pending containment |
| Caller-selected authority               | Accept no trusted identity, path, model, engine, profile, runtime, limits, template, device, retry, or fallback from WebView/user/model/environment/artifact metadata                                 |
| Owner-authentication confusion          | Keep D-062/D-094 separately Blocked; local/no-auth means provider auth only                                                                                                                           |
| Resource and thermal denial             | Require enforceable pre-allocation and runtime ceilings plus target-Mac rejection evidence                                                                                                            |
| Cancellation laundering                 | Require original-work signalling, bounded joins, and positive quiescence; UI timeout or late filtering is insufficient                                                                                |
| Late-result mutation                    | Validate identity/sequence/terminal state before every mutation and test every post-terminal class later                                                                                              |
| Personal-data residue                   | Require visible disclosure, volatility, content-free logs, cache/temp/crash/swap limits, deletion/removal, and incident response                                                                      |
| Fallback drift                          | Missing or failed local state remains unavailable; never retry, download, repair, switch model, or invoke a cloud profile                                                                             |
| Fixture transference                    | Label every existing deterministic test as design evidence only, not engine/artifact/target proof                                                                                                     |
| Documentation overclaim                 | Use the closed row/status vocabulary, preserve Blocked readiness, and require independent architecture/security/readiness review                                                                      |

### Native trusted-computing-base rule

An in-process engine is not contained merely because a narrow Rust adapter
wraps it. It shares the application process's memory, user context, current
filesystem reachability, linked frameworks, crash boundary, and potential
network APIs. App Sandbox, Hardened Runtime, Tauri CSP, WebView capabilities,
and `unsafe_code = "forbid"` cannot be cited as complete containment of
transitive native code.

If the exact source/feature/native graph cannot exclude network and unrelated
secret or filesystem reachability, the positive disposition is unavailable.
The candidate must remain Blocked pending a separately accepted containment or
isolation architecture. This plan may not switch to a helper, external process,
local server, plugin, JIT, WebSocket, or other runtime as a workaround.

### Evidence handling

- Use current official upstream, registry, release, license, advisory, and
  model-publisher primary sources only after separate authorization.
- Freeze retrieval timestamps, exact URLs, immutable versions/commits, digests,
  and relevant source excerpts without copying secrets, binaries, model bytes,
  or excessive copyrighted content into the repository.
- Treat registry metadata, model cards, release notes, dependency manifests,
  source, and runtime self-reports as untrusted claims requiring cross-checks.
- A runtime-reported engine or model identity is corroborative only; trusted
  expected identity must originate in Rust-owned immutable configuration and
  be compared exactly.
- Disposable dependency resolution must run in a fresh temporary directory,
  metadata-only where supported, without build scripts or execution. It may
  produce evidence text only; manifests, lockfiles, caches, binaries, and
  resolver artifacts must not enter the repository.
- No personal, credential, account, home-directory, device-identifier, or
  private artifact data may appear in docs, logs, terminal captures, tests, or
  ordinary CI.

## Test plan

### Documentation evidence tests

- Confirm the nomination envelope contains exactly one candidate, exact
  supplied engine/model coordinates, one target class, one acquisition mode,
  and one closed exact source allowlist, with no mutable value, fallback,
  second candidate, D-119 handle, or product selector.
- Confirm the evidence-resolved tuple is complete, uses only allowlisted
  evidence, and matches every accepted envelope field exactly. Any mismatch
  must select `evidence_boundary_failed` without mutation or substitution.
- Confirm each required evidence row has exactly one allowed status and source
  lineage; all mandatory static rows must be `documented` for the positive
  disposition.
- Confirm the chosen closed disposition is exactly one of the three values in
  this plan and that no value implies admission, execution, or personal-use
  readiness.
- Confirm D-121 is append-only, Proposed/accepted only after explicit owner
  acceptance, and accurately names static evidence limitations and every
  operational `Not run` item.
- Confirm synthetic-v1, historical V0-14, D-118 `no_eligible_client`, D-119's
  exact ten `candidate_blocked` entries/no handle, D-107 8/11, additive D-108
  9/10, the ten blocker identifiers, D-113 through D-117 status, and every
  operational `Blocked` boundary remain unchanged.
- Confirm D-060/D-061 cloud rules and D-062/D-094 owner authentication remain
  controlling and no local/no-auth statement weakens them.
- Confirm no source, dependency, manifest, lockfile, configuration, workflow,
  hook, skill, script, generated output, artifact, or external-system path
  changed.
- Confirm the final complete change set equals the exact sixteen-path allowlist
  and preserves all dated D-120 and PR #113 closeout evidence.

### Future source and target-Mac tests — Not run

The following are requirements for later separately approved plans, not tests
run while drafting or executing this documentation evidence assessment:

- compile/load/inference success or deterministic candidate failure;
- exact artifact size/digest/provenance/license and missing, corrupt,
  substituted, incompatible, arbitrary-path, symlink, hardlink, and TOCTOU
  rejection;
- actual no-DNS/socket/proxy/telemetry/update/license/crash/embedding/fallback
  behavior across the complete lifecycle;
- native/FFI secret, Keychain, filesystem, account, environment, credential,
  certificate, private-key, signing, and memory reachability;
- bounded cold/warm load, tokenization, prefill, generation, stream,
  backpressure, resource, thermal, and teardown behavior;
- cancellation and bounded join at every practical engine phase;
- cleanup, quarantine, positive quiescence, restart denial, and eventual safe
  restart;
- pre-mutation rejection of every malformed, foreign, stale, duplicate,
  out-of-order, oversized, and post-terminal result;
- non-demo Cortexa owner authentication;
- personal-prompt disclosure, processing, deletion, logging, cache/temp/crash,
  swap, incident, and recovery behavior; and
- target-Mac native UI, accessibility, resize, theme, reduced-motion, zoom,
  scrolling, permission, console/log, kill-switch, removal, and rollback checks.

No existing fixture, runtime, demo, gateway, approval, orchestration, CI, or
documentation result may be reported as passing one of these future tests.

## Verification commands

For this one-file planning task, run before editing and after the final plan
edit:

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
git status --short --branch
git diff --name-only
git ls-files --others --exclude-standard
```

The exact final changed-path set must contain only this plan. Do not run
`begin`, a session-end gate, a quality gate, or a post-increment finalizer for
this planning task.

For the later separately approved staged documentation evidence increment,
rerun those checks and also require:

- complete `npm run verify`, with every result recorded;
- exact sixteen-path inventory and protected-source/dependency/configuration
  preservation;
- append-only D-121 and exact candidate/disposition assertions;
- SHA-256 preservation of synthetic-v1, historical V0-14, D-118/D-119/D-120,
  their plan/increment/review artifacts, D-107 evidence, and the PR #113
  closeout;
- exact ten-candidate, ten-blocker, Proposed/non-controlling, and Blocked-state
  assertions;
- independent architecture, security, documentation, code-health,
  technical-debt, quality, and readiness reviews;
- `python3 .codex/hooks/session_end_gate.py`;
- exact report validation; and
- the exact post-increment finalizer only after the final authorized edit.

Every command and manual check must be recorded as Passed, Failed, Pending, or
Not run. A completion marker is valid only if it binds the final report and
exact workspace fingerprint.

## Manual gates

### This planning task

- Clean synchronized baseline: Passed.
- Complete required project-memory/security/testing review: Passed.
- Current source/dependency and PR #113 closeout inspection: Passed.
- Documentation-tier baseline validation: Passed.
- Same-active-increment nomination-only amendment: Passed.
- Current official public primary-source retrieval needed to propose one
  envelope: Passed; confined to the closed pages recorded above.
- Full evidence assessment, resolver work, model/artifact bytes or operations,
  candidate admission, source work, product tests, target-Mac inference, owner
  authentication, credentials, signing, providers, and operational external
  systems: at the nomination checkpoint, Not run by scope.
- Owner approval of this exact Ready plan: Passed.
- Candidate nomination envelope: Passed; proposed exactly once and accepted
  exactly by the owner without mutation. No candidate is selected or admitted.
- Separate full static-assessment authority: Passed; limited to the frozen
  source allowlist and optional disposable metadata-only resolution.
- Full allowlisted static assessment: Passed; matching tuple and all fifteen
  rows completed, with zero positive static rows.
- Disposable metadata-only dependency resolution: Not run; no package/binding
  coordinate exists and resolution could not cure the decisive source failure.
- Source archives, binaries, packages, model bytes, artifact redirects,
  candidate/source-engine builds or execution, target-Mac inspection,
  credentials/providers/signing/product systems, and operational checks: Not
  run by scope. Repository verification builds are separate and passed.
- Disposition and exact D-121 wording: Passed; explicitly accepted by the owner
  before final reconciliation.

### Future evidence increment

1. Owner approval of this exact plan, branch, gate, and sixteen-file ceiling.
2. Completed same-active-increment authorization for the narrow nomination-only
   research amendment; it grants no continuing retrieval authority.
3. Separate owner acceptance of the one proposed immutable candidate
   nomination envelope before the full evidence assessment. Passed.
4. Separate owner authorization for the full read-only assessment from the
   envelope's public source allowlist and any disposable metadata-only
   dependency resolution. Passed.
5. Separate owner acceptance of the matching evidence-resolved tuple,
   completed matrix, exact closed disposition, and exact D-121 wording
   before final reconciliation. Passed.
6. Independent architecture, security, documentation, code-health,
   technical-debt, quality, and readiness acceptance.
7. Valid completion marker and owner review before any Git publication.
8. A later separate plan and approval before artifact acquisition, filesystem
   access, candidate build/execution, target-Mac testing, source implementation,
   owner-authentication work, Personal Assistant IPC/UI, or personal prompts.

## Risks

- A planning candidate may be mistaken for a selected or admitted model.
- A static source review may miss feature-conditional, generated, native,
  dynamically loaded, telemetry, updater, crash, or licensing behavior.
- A model card or runtime self-report may be mistaken for trusted exact
  identity.
- Artifact verification may contain a TOCTOU gap if the engine reopens a path
  rather than consuming the verified bytes or descriptor.
- An in-process engine may inherit all process privileges and expose memory-
  safety, credential, filesystem, Keychain, network, and crash-boundary risk.
- “Offline” or “local” marketing may be mistaken for complete no-egress proof.
- Existing fixture cancellation may be transferred incorrectly to synchronous
  native, thread, callback, or GPU work.
- Late-result filtering may be mistaken for actual termination and cleanup.
- Resource ceilings may be descriptive rather than enforced before allocation.
- Personal content may persist in mappings, allocator reuse, swap, caches,
  temp files, logs, crash reports, screenshots, or support captures.
- Personal-use licensing may be confused with redistribution or app bundling
  rights.
- A missing/corrupt artifact may cause an implicit download, repair, alternate
  model, or cloud fallback.
- Target-Mac non-observation may be overstated as universal source proof.
- Documentation status may make the historical V0-14 or blocked selectable-v3
  catalog appear active.

These risks remain closed by the single-candidate freeze, conjunctive evidence
matrix, staged owner gates, exact scope, closed dispositions, no operational
authority, and preservation assertions.

## Rollback or failure strategy

- Before publication, rollback of this planning task is removal of this one new
  untracked plan file. Do not perform that rollback without owner direction.
- Before publication of the future evidence increment, reverse only its exact
  sixteen documentation paths to the recorded baseline; do not reset or
  rewrite history.
- After publication, correction requires a separately owner-authorized
  documentation revert or additive successor decision through normal Git
  review.
- Disposable metadata work, if later authorized, must use a fresh temporary
  directory. Delete it only under the explicit authority and exact safe path of
  that later plan; if cleanup ownership is ambiguous, quarantine and report it.
- `candidate_not_eligible_or_unproven` records the exact failed/unproved rows
  and selects no replacement.
- `evidence_boundary_failed` records the integrity or authority failure and
  supports no candidate conclusion.
- No failure may trigger candidate substitution, external search beyond the
  authorization, download, installation, build, execution, repair, source edit,
  artifact operation, cloud fallback, or successor start.

## Stop conditions

Stop immediately and report rather than infer, repair, substitute, or widen
scope if:

- Git is dirty beyond the exact approved plan, divergent, ambiguous, not
  synchronized at an authorized descendant, or contains overlapping user work;
- the ignored gate is active or failed, identifies an unexpected predecessor,
  or is invalid for a reason other than the exact approved workspace
  fingerprint change;
- work would finalize, rebind, retarget, or mutate the completed PR #112
  publication-closeout gate or create a recursive PR #113 closeout;
- D-121 or any exact plan/increment/review path is occupied;
- D-120, the PR #113 closeout, current source, dependencies, or any controlling
  project-memory/security/testing record differs materially from this plan;
- more than one candidate, a mutable candidate, a fallback, an alternate
  backend, or a caller-selected path/configuration is proposed;
- the owner has not explicitly accepted the complete proposed nomination
  envelope before the full candidate assessment, or has not separately
  authorized that assessment against the exact source allowlist;
- retrieved evidence falls outside the accepted allowlist or changes any
  nomination-envelope coordinate, topology, target class, or acquisition mode;
- any further external retrieval or disposable resolution lacks exact separate
  authority, exceeds the frozen official-public-source boundary, retrieves
  artifact bytes, runs a build script, executes code, or changes repository
  manifests/locks/caches;
- source, features, dependencies, native libraries, build scripts, unsafe/FFI,
  plugin/JIT/dynamic code, advisories, provenance, digest, license, rights,
  parser, template, or target compatibility is incomplete or ambiguous;
- the candidate needs a helper, external executable, localhost/server,
  WebSocket, Unix service, provider SDK, network client, runtime downloader,
  updater, telemetry, license check, remote embedding, crash upload, plugin,
  JIT, dynamic runtime, or cloud fallback;
- complete source evidence cannot exclude DNS/socket/network or unrelated
  filesystem, Keychain/account, credential, certificate, private-key, signing,
  environment-secret, default-chain, gateway/provider-token, or secret-memory
  reachability;
- exact same-byte verification and loading cannot exclude substitution,
  symlink/hardlink, archive, permission, ownership, or TOCTOU risk;
- streaming, allocation, queue, context, output, resource, deadline,
  cancellation, bounded join, cleanup ownership, positive quiescence,
  quarantine, or pre-mutation late-result behavior is absent or unproved;
- owner authentication is represented as satisfied by local/no-auth, OS login,
  personal device control, or process identity;
- target-Mac observation, fixture tests, documentation checks, or runtime self-
  reporting is represented as universal engine, artifact, no-egress,
  cancellation, cleanup, or identity proof;
- final reconciliation or a completion claim would precede the separately
  accepted evidence disposition and exact D-121 wording;
- synthetic-v1, historical V0-14, D-118, D-119's exact ten entries/status/no-
  handle state, any D-107 blocker, D-113 through D-117, or any operational
  `Blocked` boundary would change;
- any credential, secret, personal prompt/output, account/device identifier,
  private path, or sensitive system evidence would enter source, docs, logs,
  screenshots, terminal output, tests, or ordinary CI;
- any source, dependency, manifest, lockfile, configuration, workflow, hook,
  skill, script, generated output, model, artifact, filesystem, credential,
  provider, signing, product, or external-system path would change; or
- any required validation, preservation assertion, independent review,
  session, quality, report, or completion gate fails or remains required and
  pending.

## Decisions made

- Make this plan Ready only as a staged documentation evidence methodology;
  keep candidate selection, admission, artifact acquisition, target-Mac
  execution, source implementation, and personal use Blocked.
- Require exactly one immutable owner-accepted candidate tuple with no
  substitution, alternate backend, retry, fallback, or D-119 selection handle.
- Treat an in-process engine and every native dependency as part of the trusted
  computing base, not as contained by a Rust adapter, CSP, capabilities,
  Hardened Runtime, or crate-level unsafe linting.
- Require conjunctive static source/dependency/artifact evidence and later
  target-Mac observation. Neither evidence class can substitute for the other.
- Use only the three closed non-admitting evidence dispositions defined above.
- Keep owner authentication, acquisition/filesystem authority, containment,
  target-Mac execution, IPC/UI, and real personal-prompt activation behind
  separate accepted plans.
- Nominate exactly
  `fixed_local_v2_llamacpp_0_3_0_qwen2_5_1_5b_q4km_cpu_v1` for static
  assessment, using the full immutable engine/model coordinates, CPU-only
  topology, sanitized target class, `pre_provisioned` mode, and closed source
  allowlist above. The owner accepted this exact envelope; keep it frozen and
  non-admitted.
- Record the owner-accepted `candidate_not_eligible_or_unproven` because the pinned
  engine retains a reachable dynamic-loader surface and every other mandatory
  static row also lacks complete proof. Do not record D-121 until the owner
  accepts this exact disposition and wording.

## Discoveries

- The requested prospective gate ID is 63 characters and fits the repository's
  64-character limit; its plan, increment, review, and D-121 slots are unused.
- PR #113 published the internally named PR #112 publication closeout. Its
  stable live wording intentionally prevents another recursive publication
  reconciliation.
- Current Personal Assistant Rust code already provides useful ownership and
  reducer design evidence, but production exposes no response ingress and all
  success/failure stream events remain fixture-only.
- An in-process candidate is the only topology D-120 allows this plan to assess
  without another architecture decision, but same-process privilege and
  termination risk may still make every concrete candidate ineligible.
- D-118 is not a universal impossibility claim for a truly network-free local
  lane, yet any localhost or hidden transport remains a network boundary and
  cannot be called a bypass.
- Official current-release evidence on 2026-09-03 maps llama.cpp `v0.3.0` to
  `c1d0e7a004015f23bc0233470b747b596f29b264`; the full commit is the only
  controlling engine coordinate because release metadata is not immutable.
- The pinned llama.cpp header offers load and decode cancellation callbacks but
  explicitly limits decode abort behavior to CPU execution. This supports
  nominating CPU-only assessment and rejects Metal as a fallback; it does not
  prove hard deadlines, bounded joins, cleanup, or quiescence.
- Pinned llama.cpp build/source records expose broad optional server, network,
  subprocess, accelerator, and dynamic-backend surfaces. The dynamic-loader
  implementation appears in the pinned source even with dynamic backend builds
  disabled. If full assessment cannot prove the forbidden capability absent or
  unreachable in the exact build, the candidate must fail closed; no downstream
  patch is authorized.
- The exact Qwen revision lists one 1.5B instruction-tuned Q4_K_M GGUF with
  publisher metadata for filename, byte count, SHA-256, and Apache-2.0 license.
  Artifact bytes, conversion provenance, parser/template compatibility,
  performance, output quality, and target-Mac behavior remain unobserved.
- Full static review confirmed that `GGML_BACKEND_DL=OFF` does not exclude
  `ggml-backend-dl.cpp` or `ggml-backend-reg.cpp` from the pinned core library.
  The public loader API and its explicit-path, executable/current-directory,
  and `GGML_BACKEND_PATH` loading paths remain present. The accepted topology
  therefore cannot be proven without a source change that this increment does
  not authorize.
- The frozen Qwen publisher metadata still agrees on the intended file's digest
  and exact byte count, but neither the artifact's complete embedded manifest
  nor its source-weight/conversion chain is documented. The cited upstream
  non-GGUF revision postdates the GGUF upload and is not bound to that
  conversion.
- The allowlisted llama.cpp advisory index includes parser and model-load
  findings; exact applicability and remediation for the frozen tuple are not
  established within the closed source boundary. The pinned security policy
  also recommends isolation for untrusted models, which an in-process topology
  does not provide.

## Progress

- 2026-09-03: confirmed clean synchronized `main` at
  `355d42ac8bb9a5ed663f57895675df688507ce9f`, ahead/behind `0/0`, and a valid
  completed predecessor marker.
- 2026-09-03: read the complete required project-memory, security, testing, and
  gate chain; D-120; the D-120 plan; and the PR #113-published PR #112 closeout.
- 2026-09-03: inspected current Personal Assistant, runtime, dependency, Tauri,
  capability, artifact, and fixture boundaries without modifying or executing
  them.
- 2026-09-03: passed pre-edit documentation formatting/link, repository policy,
  secret scan, and diff-hygiene checks.
- 2026-09-03: drafted this one-file Ready ExecPlan without creating a branch,
  beginning a gate, selecting or installing a model, accessing an external
  system, or modifying product state.
- 2026-09-03: the first post-draft documentation check reported Prettier drift
  in this new file only. Formatted only this plan. Independent readiness review
  then found a circular pre-retrieval candidate-freeze requirement; split it
  into an owner-accepted nomination envelope and a post-retrieval evidence-
  resolved tuple. Independent re-review accepted the correction.
- 2026-09-03: final documentation formatting/link, repository policy, secret
  scan, diff hygiene, and exact one-file scope checks passed. No product test,
  build, gate, model/artifact operation, or external-system check ran.
- 2026-09-03: after this sole untracked plan changed the workspace fingerprint,
  read-only gate status preserved the completed predecessor identity and result
  while reporting `valid: false`, as expected. No finalizer or gate mutation
  ran; a later approved `begin` must revalidate that this plan is the only
  cause.
- 2026-09-03: the owner approved this exact Ready ExecPlan, branch, gate, and
  nomination-envelope checkpoint. Reconfirmed baseline
  `355d42ac8bb9a5ed663f57895675df688507ce9f`, local `main`, and locally recorded
  `origin/main` at ahead/behind `0/0`, with this plan as the sole workspace
  change. Created the approved branch and began
  `personal-assistant-v0-fixed-local-engine-artifact-evidence-plan` exactly
  once.
- 2026-09-03: stopped at the candidate-nomination-envelope checkpoint because
  no exact owner-supplied engine/model coordinates, target class, acquisition
  mode, or source allowlist exist. No candidate was inferred, and no evidence
  retrieval, model/artifact, source, dependency, product, or external-system
  work began.
- 2026-09-03: checkpoint `npm run docs:check`,
  `npm run repository:check`, `npm run security:scan`, `git diff --check`, exact
  two-file scope, baseline ancestry, and active-gate checks passed. Completion
  verification and finalization remain intentionally not run while the manual
  nomination-envelope gate is Pending.
- 2026-09-03: the owner authorized one same-active-increment documentation-only
  amendment for a nomination-only research phase using current official public
  primary sources. Discovery used only public official owner/publisher pages;
  the final candidate evidence and any proposed later revisit are frozen to the
  closed boundary above. No binary, source archive, model artifact, dependency,
  credential, provider, signing, product, or operational system was accessed or
  changed.
- 2026-09-03: proposed the single immutable llama.cpp `v0.3.0`/Qwen2.5 1.5B
  Q4_K_M CPU-only envelope above. Recorded the pinned API's CPU-only abort
  limitation and the pinned source's unresolved dynamic-loader reachability as
  material blockers. Stopped before envelope acceptance and the full evidence
  assessment.
- 2026-09-03: the owner accepted the immutable envelope exactly and separately
  authorized the full static assessment from its frozen source allowlist. The
  active gate was not restarted.
- 2026-09-03: revalidated the engine/model coordinates and publisher metadata,
  reviewed the pinned API, build, backend-registry/loader, CPU, license,
  security, advisory, model, artifact-metadata, and Qwen guidance evidence, and
  completed the matching tuple plus all fifteen matrix rows. No coordinate or
  custody mismatch occurred. A frozen README refresh returned HTTP 429 and was
  not worked around; affected claims remain unproved.
- 2026-09-03: proposed exactly `candidate_not_eligible_or_unproven` and the
  exact D-121 wording above. No dependency resolution was needed or run. No
  archive, binary, package, model byte, artifact redirect, build, execution,
  target inspection, credential/provider/signing/product, or other operational
  access occurred. Stopped before final reconciliation and completion gates.
- 2026-09-03: formatted only the plan and increment record, then passed interim
  `npm run docs:check`, `npm run repository:check`, `npm run security:scan`,
  `git diff --check`, exact two-file scope, and active-gate validation. These
  are checkpoint checks, not the deferred completion gates or finalizer.
- 2026-09-03: the owner accepted the matching tuple, completed matrix, sole
  negative disposition, and exact D-121 wording. Reconciled the thirteen live
  records, plan, and increment; appended D-121 without changing any preceding
  decision byte; and added the sixteenth review path.
- 2026-09-03: initial closeout review found pending header chronology and
  ambiguous generic Not-run wording. Corrected only the approved documentation
  to distinguish passing repository verification builds from unrun
  candidate-specific operations. Independent re-review accepted the corrected
  result.
- 2026-09-03: final documentation, repository, security, complete verification,
  exact-scope, preservation, session, quality, report-validation, and
  post-increment gates passed. The completion marker is complete and valid.

## Acceptance criteria

- [x] The planning baseline, published predecessor, valid gate, unused paths,
      and current product absences are verified locally.
- [x] This plan changes exactly one documentation path and is Ready only for a
      separately approved staged documentation evidence increment.
- [x] The plan defines one immutable pre-full-assessment nomination envelope,
      one bounded nomination-only research exception,
      matching evidence-resolved tuple, a complete static evidence matrix,
      native TCB/no-egress analysis, future target-Mac protocol, closed
      dispositions, threats, invariants, tests, rollback, and fail-closed stop
      conditions.
- [x] Candidate nomination, evidence retrieval, disposition acceptance,
      artifact acquisition, target-Mac operation, implementation, owner
      authentication, and personal-prompt activation remain distinct gates.
- [x] Synthetic-v1, historical V0-14, D-118 `no_eligible_client`, D-119's exact
      ten `candidate_blocked` entries/no handle, all ten D-107 blockers, D-113
      through D-117, and every operational `Blocked` boundary are preserved.
- [x] No source, dependency, manifest, lockfile, credential, provider, signing,
      model, artifact, product, or external-system state changes.
- [x] Nomination-checkpoint documentation-tier validation passes with only this
      plan and its active increment record changed.
- [x] The owner explicitly approves the exact Ready plan before a branch is
      created or its gate begins.
- [x] The owner explicitly accepts the exact immutable nomination envelope and
      separately authorizes the full static assessment from its frozen
      allowlist.
- [x] The matching evidence-resolved tuple, fifteen-row matrix, exactly one
      proposed closed disposition, and exact proposed D-121 wording are
      complete without final reconciliation.
- [x] The owner explicitly accepts that tuple, matrix, disposition, and exact
      D-121 wording before the sixteen-file final reconciliation begins.
- [x] The thirteen live records and active plan/increment append or reconcile
      D-121 without selecting a replacement or weakening a Blocked boundary.

## Final results

**PASS WITH ADVISORIES for planning only.** Baseline and final
`npm run docs:check`, `npm run repository:check`, `npm run security:scan`, and
`git diff --check` passed. Exact scope contains only this untracked plan. The
initial post-draft documentation check reported formatting drift in this file;
formatting-only correction passed. Independent architecture review accepted
the plan. Independent readiness review found one staging inconsistency, and
accepted the corrected nomination-envelope/evidence-resolved-tuple sequence.

The historical predecessor reported `complete` and `PASS WITH ADVISORIES`; its
marker reported `valid: false` only because this new untracked plan changed the
workspace fingerprint. No historical finalizer ran and no historical evidence
was altered. The exact approved successor gate then became active.

The approved branch and gate began exactly once. One owner-authorized
nomination-only research amendment produced exactly one proposal, and the
owner then accepted that immutable envelope and separately authorized its full
static assessment. The matching ten-component tuple contains one `documented`,
eight `contract_unproven`, and one `not_run` result. The fifteen-row matrix
contains zero `documented`, fourteen `contract_unproven`, one `not_run`, and
zero `boundary_failed` rows. The owner accepted exactly
`candidate_not_eligible_or_unproven` and the D-121 text above; D-121 is now
appended once without changing earlier decision bytes.

Post-assessment formatting, documentation/link checking, repository policy,
secret scanning, diff hygiene, exact two-file scope, and active-gate status
passed. After the required owner acceptance, final reconciliation expanded the
change set only to the exact sixteen-file ceiling. Complete repository
verification, exact-scope and preservation assertions, independent review,
session, quality, report validation, and the post-increment finalizer passed;
the post-increment review and gate state record the valid completion marker.

Source/archive or artifact-byte retrieval, candidate/source-engine build,
load, or inference, dependency resolution, candidate admission,
product/model-artifact filesystem operations, target-Mac inference, owner
authentication, credentials, providers, signing, product systems, and
operational external systems were `Not run` by scope. Repository tests and
application/Tauri verification builds passed. Every candidate and operational
successor remains `Blocked`.

## Documentation updates

- [x] Add this one Ready ExecPlan.
- [x] Add the active increment record after the owner-approved `begin`.
- [x] Record the accepted envelope, matching tuple, completed static matrix,
      proposed negative disposition, and exact proposed D-121 wording in the
      existing plan and increment record.
- [x] Update the thirteen live architecture/security/project-memory documents
      only after the owner accepts the exact disposition and D-121 wording.
- [x] Add the post-increment review only after the evidence disposition is
      separately accepted and final reconciliation is complete.
