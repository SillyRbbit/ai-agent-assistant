# V0-6 — direct Rust HTTPS dependency decision

Status: Complete (`PASS WITH ADVISORIES`); D-118 accepted, no client selected
Owner: Henry Dang
Last updated: 2026-09-03
Planning baseline: `6c3615c507b58a771f696ebb7bfdb5bd13d1d1b7`
Branch: `codex/personal-assistant-v0-https-dependency-decision`
Gate state: complete and valid for the exact fifteen-file documentation result
Depends on: completed V0-1/V0-2 architecture baseline and separately authorized,
fresh dependency evidence

## Goal and user-visible outcome

Make one closed documentation decision about the direct Rust HTTPS client stack
that a later, separately approved V0-7 could use for one fixed Cortexa gateway
origin. The decision will either select one exact, minimized stack, record that
none of the frozen candidates is eligible, or fail because the evidence boundary
cannot support a trustworthy conclusion.

There is no user-visible product change. This plan does not add a dependency,
open a socket, send a request, expose a provider, or make V0-7 Ready. `Ready`
means only that this documentation decision is sufficiently bounded for the
owner to approve as a separate increment.

## Current-state evidence

- `main`, local `origin/main`, and `HEAD` resolve to
  `6c3615c507b58a771f696ebb7bfdb5bd13d1d1b7`; the planning checkout was clean
  and ahead/behind was `0/0` when this plan was revised.
- V0-1 is published at `dca584e`; V0-2 is published at `1513bd8` with a
  volatile, transport-free Rust session host.
- `src-tauri/Cargo.toml` declares Rust `1.88` as the MSRV and does not declare a
  direct HTTP, HTTPS, TLS, or provider client.
- The repository-pinned Rust toolchain is `1.90.0`. Planning inspection used
  Node `26.3.0`, npm `11.16.0`, rustc `1.90.0`, and cargo `1.90.0` on macOS
  `26.6` (`25G72`).
- `src-tauri/Cargo.lock` contains transitive `reqwest 0.13.4`, `hyper 1.10.1`,
  and `tokio 1.52.3` nodes. Their presence is inventory, not permission or
  evidence that the target-Mac product owns an HTTPS client. Offline inverse
  dependency inspection found no active target-Mac `reqwest` or `hyper` edge;
  Tokio is present through Tauri.
- `src-tauri/src/personal_assistant_v0.rs` remains transport-free and has no
  production response ingress. `src-tauri/src/lib.rs` exposes no Personal
  Assistant Tauri route.
- V0-3 remains paused and Blocked by its signing requirements. Nothing in V0-6
  changes or waives that result.
- D-107 remains historical 8/11, D-108 remains additively 9/10, proposed D-113
  through D-117 remain non-controlling, and all ten D-107 blockers remain.
- V0-7 remains Blocked by V0-3, V0-5, an accepted V0-6 outcome, separate source
  authorization, and the test-plan discrepancy recorded below.

One local `cargo tree --locked --target all --offline` planning attempt stopped
because Cargo reported that uncached `android_system_properties 0.1.5` would
need to be downloaded while offline mode was active. A separate independent
review attempt without offline mode produced no graph because the sandbox
denied creation of the Cargo registry source directory for
`atomic-waker 1.1.2`. Neither attempt produced usable cross-target dependency
evidence; no repository file changed, and no claim is made about unobserved
network contact.

## Scope

### This planning revision

Only these seven existing documentation files may change while making this
ExecPlan current and consistently Ready:

1. `docs/plans/2026-08-28-personal-assistant-v0-https-dependency-decision.md`
2. `PLANS.md`
3. `NEXT_STEPS.md`
4. `PROJECT_STATUS.md`
5. `ROADMAP.md`
6. `HANDOFF.md`
7. `CHANGELOG.md`

This planning revision does not begin the increment or approve evidence
collection.

### Authorized decision increment

The separately approved documentation-only decision is limited to these
fifteen paths:

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
13. `docs/plans/2026-08-28-personal-assistant-v0-https-dependency-decision.md`
14. `docs/increments/personal-assistant-v0-https-dependency-decision.md` (new)
15. `docs/reviews/2026-09-03-personal-assistant-v0-https-dependency-decision-post-increment-review.md`
    (new)

The active increment confirmed that the prospective D-118 slot remains free.
A later collision before final reconciliation stops the work until this plan is
amended.

## Explicit non-goals and phase authority

This seven-file planning revision may not call `begin`, read public candidate
sources or advisory services, run a hypothetical resolver, download candidate
material, or access any operational external system.

Neither this planning revision nor the later documentation decision may:

- change product or test source, `Cargo.toml`, `Cargo.lock`, npm manifests or
  lockfiles, toolchains, dependencies, capabilities, CSP, permissions,
  workflows, runner policy, or signing configuration;
- build, link, install, execute, or run a build script for a candidate crate;
- access credentials, Keychain, certificates, private keys, Apple/Xcode,
  signing, providers, gateways, cloud resources, or product systems;
- send product, gateway, provider, telemetry, credential-bearing, or personal-
  content DNS, TLS, HTTP, or other traffic;
- create a generic network client, caller-selected origin, fallback client,
  provider SDK, Tauri/WebView network route, or new runtime authority;
- modify or accept D-076, D-096 through D-117, the D-107 blocker set, V0-3,
  V0-5, V0-7, or any later V0 milestone; or
- implement source, commit, push, merge, release, publish, or start a successor
  without the distinct authority required for that action.

The active decision's only external-access exception was the owner's explicit
authorization for unauthenticated, read-only retrieval of the exact frozen
public source, registry, license, and advisory materials plus resolver metadata
and source needed by the evidence boundary below. Nothing may be copied into
the repository or retained outside the exact validated scratch boundary. The
branch and `begin` command received separate explicit owner approval; neither
action grants any broader authority.

## Trusted boundary and assets

A future implementation remains constrained as follows:

- Trusted Rust owns a compile-time HTTPS origin and path, method, request body,
  headers, limits, timeouts, client lifecycle, and terminal state.
- The WebView, caller input, process environment, proxy state, DNS, local trust
  state, network, gateway response, crate source/metadata, feature defaults,
  and client-native errors are untrusted.
- Credentials and future synthetic request bytes are protected assets, but
  neither is accessed in V0-6.
- Response bytes cross a trust boundary only after TLS/application checks and
  before retention limits, framing, validation, and terminal-state reduction.
- Cancellation and deadline ownership must cover the request, original socket,
  connection driver, pool, resolver/TLS work, ingress, and cleanup result.

## Frozen candidate register

The active review evaluated exactly these three families. Every frozen variant
is now **Ineligible** under the matrix below; lockfile presence is not
eligibility.

| Candidate family                          | Exact variants or focus                                                                                                                                             | Required distinguishing proof                                                                                                                                                                                                                                                                             |
| ----------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Direct minimized async `reqwest`          | Evaluate `rustls` with platform verifier, `rustls` with static WebPKI roots, and exact feature `native-tls-no-alpn` only when current primary evidence qualifies it | Exact direct version/features with `default-features = false`; cross-target feature unification with Tauri; root/TLS behavior; disabled proxy, redirect, cookie, retry, decompression, ambient-auth, and pooling authority; incremental body limits; per-attempt connection ownership and abort semantics |
| Explicit lower-level `hyper` client stack | One exact HTTP client, body, runtime, and TLS connector stack                                                                                                       | Complete direct/support graph; one owned connection driver; join/quiescence and socket ownership; HTTP mode, bounds, fixed connector, TLS/root behavior, and why the larger explicit surface is justified                                                                                                 |
| Direct synchronous `ureq`                 | Exact current stable family and reader behavior                                                                                                                     | Hard interruptibility during DNS/connect/TLS/read, monotonic idle/total enforcement, streaming bounds, proxy/TLS/decompression behavior, and proof that no stuck thread/process survives cancellation                                                                                                     |

Adding or substituting a family requires a plan amendment and separate owner
approval before further evidence collection.

These approaches are pre-rejected:

- standard-library or hand-written TLS/HTTP;
- WebView `fetch`, a Tauri HTTP plugin, or browser-held credentials;
- shell, `curl`, subprocess, helper-process, or process-kill cancellation;
- a provider SDK;
- use of Tauri's private/transitive `reqwest` without a direct declaration; and
- multiple runtime clients, caller/runtime selection, or automatic fallback.

## Mandatory candidate evidence matrix

Every cell below is conjunctive. Missing, stale, ambiguous, contradictory, or
inferred evidence is `Unproved`, not a qualified assumption.

1. Exact stable release, source/tag, checksum, direct/support crates,
   `default-features = false`, and complete enabled feature set.
2. Maintainer ownership, maintenance/security policy, SPDX licenses and
   notices, and current RustSec/advisory status.
3. Complete hypothetical macOS and Linux dependency/feature graph and lockfile
   delta, including added, removed, duplicated, and feature-unified nodes,
   build scripts, proc macros, native/assembly code, TLS crypto providers, and
   checksums.
4. MSRV no higher than Rust `1.88` and documented support for the repository's
   macOS and Linux targets.
5. One exact TLS backend and root-store strategy; TLS 1.2 or later;
   certificate-chain and hostname verification; SNI and ALPN behavior; whether
   user/admin-managed platform roots or trust settings affect acceptance;
   revocation, OCSP/CRL, and certificate-transparency behavior; and every
   resulting OS-managed network, cache, or log effect. No pinning invention,
   custom production root, permissive callback, verification bypass, or silent
   backend drift is allowed. Mutable platform trust or OS effects must be
   explicitly accepted in the proposed D-118 result or disqualify the variant.
6. One compile-time V0-5-owned HTTPS scheme, hostname, port, and path; no
   caller, WebView, environment, userinfo, fragment, alternate-origin, or
   custom-resolver authority.
7. Redirects, automatic retry/reconnect/fallback, cookies, netrc, ambient
   authentication, environment/platform proxies, decompression, and body
   replay disabled. Exactly one application-level HTTP request and one
   credential/header/body transmission are possible.
8. Exact DNS, connection reuse/pooling, HTTP-version, content-encoding,
   framing, and connection-driver behavior. Any multi-address DNS or Happy-
   Eyeballs connection attempts must be documented, finite, deadline-bound,
   cancellation-owned, and unable to transmit duplicate credentials or body.
   No post-transmission reconnect, replay, retry, redirect, or fallback exists.
9. Incremental response streaming with header, chunk, line, event, and total
   byte limits before application retention, bounded backpressure, and no
   unbounded full-body accumulation in the client or adapter.
10. Active monotonic deadlines owned below the UI: 10-second connect,
    20-second idle, 60-second provider, and 120-second total ceilings.
11. Cancellation closes ingress first, aborts the original exchange, sends no
    replacement request, accounts for resolver/TLS/driver/pool/socket work,
    and produces a bounded cleanup result. Ambiguous cleanup retains a
    quarantined lease and blocks restart.
12. Bytes, callbacks, task completions, and native errors arriving after a
    terminal state are inert before state, evidence, UI, IPC, logging, or
    follow-on mutation.
13. Closed typed and redacted errors/logs that never retain or expose URLs,
    headers, credentials, request/response content, fixture canaries, or raw
    native/client errors.
14. A deterministic production-shaped test seam plus a hermetic way to test the
    actual selected client's TLS, routing, bounds, cancellation, and cleanup
    behavior without external traffic or production trust injection.
15. Exact removal, rollback, and dependency-pruning procedure.

If more than one candidate passes every row, apply a documented security-first
comparison in this order: smaller effective authority, stronger cancellation/
cleanup ownership, fewer dependency/native-code nodes, narrower feature graph,
and simpler complete testability. A selected result requires one unique winner
after that ranking. If the ranking remains tied, select
`evidence_boundary_failed`; never leave a runtime or caller choice.

## Evidence acquisition boundary

The owner granted the active increment explicit authority for exactly two
read-only activities:

1. Read frozen public primary sources and advisory metadata without
   authentication, accounts, credentials, provider access, or product traffic.
2. Resolve hypothetical manifests only in a disposable directory created for
   that review, with separate scratch `CARGO_HOME` and `CARGO_TARGET_DIR`, no
   repository write, no build, no build-script execution, and no copy-back of
   a generated lockfile.

The result may record only sanitized package names, versions, checksums,
features, licenses, advisory identifiers, and aggregate graph facts. Personal
paths, account identifiers, cache contents, tokens, or machine identifiers must
not enter the repository. If either activity is not separately authorized or
cannot be isolated exactly, the closed result is
`evidence_boundary_failed`.

V0-6 may record documented platform support, but target-Mac compilation,
runtime, TLS, cancellation, socket, credential, provider, and traffic checks
remain `Not run`. They belong to later separately approved implementation
evidence and cannot be promoted to Passed.

## Authorized interim evidence — 2026-09-03

The owner authorized the evidence activities above for this increment. The
checkout remained at the synchronized planning baseline, the named branch was
created, and the gate is active. Candidate evidence was retrieved without
authentication from the crates.io registry, published crate sources and
documentation, upstream release/source repositories, and the RustSec advisory
database. The RustSec snapshot is commit
`5a0ebedfe8bdd2e295b171f4162f8c977bcad9a5` dated 2026-09-02.

All hypothetical resolution occurred only beneath `mktemp`-created scratch
roots with a separate `<scratch-root>/cargo-home` and `<scratch-root>/target`.
Each candidate started from an exact copy of the baseline manifest and lockfile
in its scratch root. A second isolated root corrected one independently found
static-root fixture error. `cargo metadata` resolved the macOS
`aarch64-apple-darwin` and Linux `x86_64-unknown-linux-gnu` graphs;
`cargo tree --locked --offline` inspected them. No candidate was compiled,
linked, loaded, or executed, no build script ran, and no resolver artifact was
copied into the repository.

### Exact hypothetical candidates

These declarations are evidence fixtures only. They are not proposed source
changes and grant no dependency or transport authority.

| Variant            | Exact direct declarations and effective design                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| ------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `reqwest_platform` | `reqwest = =0.13.4`, defaults off, `rustls-no-provider`; `rustls = =0.23.43`, defaults off, `ring,std,tls12`; `tokio = =1.53.1`, defaults off, `rt,sync,time`. The client would explicitly select the Rustls backend and the platform verifier.                                                                                                                                                                                                                                                                         |
| `reqwest_webpki`   | The platform fixture above plus `webpki-root-certs = =1.0.9`. The application could convert the crate's exact DER `TLS_SERVER_ROOT_CERTS` values to `reqwest::Certificate` and pass only those values through stable `tls_certs_only`; reqwest documents that this disables native and built-in roots. `rustls-platform-verifier` remains in the resolved graph through the reqwest feature, but this variant would not select it at runtime. Updating the compiled root data would require a rebuild and fresh review. |
| `reqwest_native`   | `reqwest = =0.13.4`, defaults off, `native-tls-no-alpn`; `tokio = =1.53.1`, defaults off, `rt,sync,time`. The client would explicitly select native TLS and HTTP/1.                                                                                                                                                                                                                                                                                                                                                     |
| `hyper_webpki`     | `hyper = =1.11.1` with `client,http1`; `hyper-util = =0.1.20` with only `tokio`; `http-body-util = =0.1.5`; `bytes = =1.12.1` with `std`; `tokio = =1.53.1` with `io-util,net,rt,sync,time`; `rustls = =0.23.43` with `ring,std,tls12`; `tokio-rustls = =0.26.4` with `ring,tls12`; and `webpki-roots = =1.0.9`, all with defaults off where supported. The application would own one `TcpStream`, TLS session, HTTP/1 connection driver, and request; no legacy Hyper client or pool is included.                      |
| `ureq_webpki`      | `ureq = =3.4.0`, defaults off, feature `rustls`. The resolved TLS closure is Rustls 0.23.43, ring 0.17.14, and WebPKI roots 1.0.9.                                                                                                                                                                                                                                                                                                                                                                                      |

Registry checksums were present for every registry node. Key frozen checksums
are reqwest 0.13.4
`219c5811de6525e5416c7d5d53bb656d3afdbc6c5af816e0802bcfa42dbdc1c3`,
hyper 1.11.1
`27b501faa50e7a26c3d3560ca625132f4078a17771f4810baf70475ae48cbe43`,
ureq 3.4.0
`972d7902c8735f2695410b8aed7df6ed12a47394aa1c8d7af49f0497b731a94d`,
Rustls 0.23.43
`0283386ce02abc0151e1761d08802dfe86c173b0b494af5cbc086574e453da06`,
ring 0.17.14
`a4689e6c2294d81e88dc6261c768b63bc4fcdb852be6d1352498b114f61383b7`,
WebPKI roots 1.0.9
`7dcd9d09a39985f5344844e66b0c530a33843579125f23e21e9f0f220850f22a`,
and WebPKI root certificates 1.0.9
`b96554aa2acc8ccdb7e1c9a58a7a68dd5d13bccc69cd124cb09406db612a1c9b`.

### Hypothetical graph and supply-chain result

Full-lock counts below exclude the local root package; target-graph counts
include the unchanged local root so they match the captured `cargo tree`
evidence. `Added` and `removed` are against the unchanged 435-node baseline
lock or the applicable like-for-like active target graph. An update appears as
one removal plus one addition.

| Variant            | Full lock      | macOS active graph | Linux active graph | Notable delta                                                                                                                              |
| ------------------ | -------------- | ------------------ | ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------ |
| Baseline           | 435            | 244                | 285                | Comparison only                                                                                                                            |
| `reqwest_platform` | 455 (`+21/-1`) | 279 (`+36/-1`)     | 317 (`+33/-1`)     | Tokio 1.52.3 -> 1.53.1; adds reqwest's Hyper/Tower stack, Rustls/ring, platform-verifier, and target support                               |
| `reqwest_webpki`   | 455 (`+21/-1`) | 280 (`+37/-1`)     | 318 (`+34/-1`)     | Platform graph plus direct DER root certificates; the platform-verifier dependency remains even though this design would not select it     |
| `reqwest_native`   | 446 (`+12/-1`) | 276 (`+33/-1`)     | 314 (`+30/-1`)     | Tokio update; native-tls/Security Framework on macOS and native-tls/OpenSSL edges on Linux                                                 |
| `hyper_webpki`     | 445 (`+14/-4`) | 266 (`+24/-2`)     | 305 (`+22/-2`)     | Updates bytes 1.11.1 -> 1.12.1, http-body-util 0.1.3 -> 0.1.5, Hyper 1.10.1 -> 1.11.1, and Tokio 1.52.3 -> 1.53.1; adds Rustls/ring/WebPKI |
| `ureq_webpki`      | 448 (`+13/-0`) | 258 (`+14/-0`)     | 299 (`+14/-0`)     | Adds ureq, ureq-proto, Rustls/ring/WebPKI, and base64 0.23.1 without replacing the existing Tokio graph                                    |

The baseline lock has 33 names at multiple versions. The reqwest Rustls graphs
have 34 and add/change the `jni` and `windows-sys` version families; the native
graph has 35 and adds `foreign-types` and `foreign-types-shared` duplicates;
the Hyper and ureq graphs remain at 33 while changing the existing
`windows-sys`, or `base64` and `windows-sys`, version sets respectively.

Candidate-active registry manifests contain no missing license field and no
declared MSRV above Rust 1.88. The direct/support MSRV maximum is 1.85. Some
transitive manifests do not declare `rust-version`; no compatibility guarantee
is inferred for those nodes. Observed license expressions are limited to MIT,
Apache-2.0, ISC, BSD-3-Clause, CDLA-Permissive-2.0, and their documented dual
or conjunctive combinations. Rustls variants add the ring 0.17.14 build script
and bundled C/assembly; HTTP parsing and Rustls also expose build targets.
Native TLS adds platform framework/OpenSSL bindings and, on Linux, the
`openssl-macros` proc macro. No candidate build target was run.

A deterministic lock-to-advisory comparison against the frozen RustSec commit
found the same 20 entries in every graph as in the baseline: the exact two
accepted `quick-xml 0.39.4` vulnerabilities and 18 accepted warnings already
enumerated by `scripts/cargo_audit_gate.py`. It found no candidate-added
advisory. The withdrawn ring advisory RUSTSEC-2025-0007 was not treated as an
active finding; ring 0.17.14 and rustls-webpki 0.103.15 satisfy the active
patched/unaffected ranges in the snapshot. `cargo-audit` was not installed and
was not installed or executed; the comparison was a read-only scratch analysis
validated by reproducing the repository's exact accepted baseline.

### Completed mandatory evidence matrix

`Documented` means frozen primary-source or resolver evidence, not runtime
proof. `Designable / Not run` means a later source increment could impose the
control, but V0-6 did not implement or exercise it. A single `Failed` mandatory
row makes a variant ineligible.

| #   | Mandatory concern                                                        | Reqwest family                                                                                                                                                                                                                                                                                                                                                                            | Explicit Hyper stack                                                                                                                                                                           | Ureq                                                                                                                                                                                                                               |
| --- | ------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1   | Exact release, source, checksum, graph and features                      | **Documented.** Reqwest 0.13.4 and all three exact graphs are frozen above.                                                                                                                                                                                                                                                                                                               | **Documented.** Exact direct/support pins and graph are frozen above.                                                                                                                          | **Documented.** Ureq 3.4.0 tag/registry identity and exact graph are frozen above.                                                                                                                                                 |
| 2   | Ownership, maintenance, license and advisories                           | **Documented with advisory.** Current release activity and licenses are present; no upstream SECURITY policy was found; no added RustSec finding.                                                                                                                                                                                                                                         | **Documented.** Current Hyper/Tokio/Rustls releases, licenses, and no added RustSec finding.                                                                                                   | **Documented with advisory.** Current release activity and licenses are present; no dedicated upstream SECURITY policy was found; no added RustSec finding.                                                                        |
| 3   | macOS/Linux graph, lock delta, scripts, macros and native code           | **Documented** by the target and lock counts above.                                                                                                                                                                                                                                                                                                                                       | **Documented** by the target and lock counts above.                                                                                                                                            | **Documented** by the target and lock counts above.                                                                                                                                                                                |
| 4   | Rust 1.88 MSRV and target support                                        | **Partly documented / Unproved.** Declared direct/support MSRVs pass; undeclared transitive MSRVs and compilation are not inferred.                                                                                                                                                                                                                                                       | **Partly documented / Unproved.** Declared direct/support MSRVs pass; compilation was prohibited.                                                                                              | **Partly documented / Unproved.** Ureq declares 1.85; compilation was prohibited.                                                                                                                                                  |
| 5   | Exact TLS, roots, identity, protocols, revocation, CT and OS effects     | **Mixed.** Platform/native variants fail because they accept mutable OS trust and expose OS-managed verification effects that reqwest cannot close. The static variant can use only the fixed DER roots through stable `tls_certs_only`; TLS 1.2/1.3, SNI, and HTTP/1 are configurable, while revocation and CT are absent. Static roots avoid Keychain trust, not DNS/socket OS effects. | **Documented limitation.** Ring, TLS 1.2/1.3, SNI, no ALPN, and static Mozilla roots can be fixed; revocation and CT are absent. Static roots avoid Keychain trust, not DNS/socket OS effects. | **Failed.** Static roots, TLS 1.2/1.3, SNI, and no ALPN are documented, but the stable configuration can honor a process-installed Rustls provider and does not freeze the provider identity; revocation and CT are absent.        |
| 6   | One application-owned fixed HTTPS origin/path                            | **Designable / Not run.** Must construct the URI internally and reject all caller/environment authority before connect.                                                                                                                                                                                                                                                                   | **Designable / Not run.** Must use origin-form request targets plus exact Host/SNI owned by V0-5.                                                                                              | **Designable / Not run.** `https_only` is insufficient alone; application must own the whole URI.                                                                                                                                  |
| 7   | No redirects, retries, fallback, cookies, proxy, decompression or replay | **Designable / Not run.** Defaults off plus explicit no-proxy, no-redirect, no-retry, and no-decompression controls are available. `pool_max_idle_per_host(0)` prevents idle reuse, but does not remove reqwest's pool abstraction or supply a driver/join contract; exact behavior still needs fixtures.                                                                                 | **Designable / Not run.** The direct one-connection stack includes none of those policy layers; adapter tests remain mandatory.                                                                | **Designable / Not run.** Redirects, proxy, pool and encoding can be disabled; ureq 3 has no built-in retry.                                                                                                                       |
| 8   | Bounded DNS/connect/pool/HTTP/framing ownership                          | **Failed.** Default GAI resolution uses non-abortable blocking Tokio work; Hyper-util may race or serially try addresses, and reqwest exposes no connector/driver join.                                                                                                                                                                                                                   | **Failed.** An application can own one socket and driver and choose one resolved address, but Tokio hostname resolution still uses non-abortable blocking work.                                | **Failed.** The returned address set is finite and sequential connection attempts are stage-deadline-limited, but timed OS resolution can detach a worker that may continue and is not cancellation-owned.                         |
| 9   | Incremental bounded response and backpressure                            | **Failed.** `Response::chunk` is incremental, but reqwest 0.13.4 exposes neither an exact HTTP/1 response-header byte/count boundary nor the underlying read-buffer cap before allocation.                                                                                                                                                                                                | **Designable / Not run.** Hyper exposes HTTP/1 header/buffer controls and demand-driven frames; Cortexa must add all chunk/line/event/total bounds and prohibit collection.                    | **Partly documented / Unproved.** Pull-based `Read`, body/header limits, and caller pacing exist; aggregate TLS/protocol/application memory bounds need proof.                                                                     |
| 10  | 10s connect, 20s idle, 60s provider and 120s hard total deadlines        | **Failed.** Timers can return control, but already-started resolver work can survive every deadline.                                                                                                                                                                                                                                                                                      | **Failed.** Async phase timers cannot impose a hard deadline on already-started blocking resolver work.                                                                                        | **Failed.** Stage/global timers document bounded connect/TLS, receive-idle, and total waits, but explicit cancellation exposes no abort handle and timed OS resolver work can survive every deadline; hard total quiescence fails. |
| 11  | Cancellation, original-exchange abort, cleanup and quiescence            | **Failed.** Dropping/aborting a future cannot stop started GAI work; reqwest offers no complete resolver/pool/socket/driver join contract.                                                                                                                                                                                                                                                | **Failed.** The owned socket and driver are abortable and joinable after DNS, but started GAI work is neither abortable nor boundedly joinable.                                                | **Failed.** The stable blocking API exposes no request cancellation or socket handle; in-flight TLS/read can end only through configured timeout, while resolver timeout can leave a detached worker.                              |
| 12  | Post-terminal bytes/results/errors are inert                             | **Designable / Not run.** A trusted generation lease can reject publication, but it cannot convert surviving library work into cleanup.                                                                                                                                                                                                                                                   | **Designable / Not run.** The same lease can close state mutation, but not the surviving resolver task.                                                                                        | **Designable / Not run.** State publication can be rejected; the underlying blocking work may still continue.                                                                                                                      |
| 13  | Closed redacted errors and logs                                          | **Designable / Not run.** Reqwest errors may contain URLs; only closed predicates may be mapped, never raw error text/source.                                                                                                                                                                                                                                                             | **Designable / Not run.** Boxed connector/native errors and tracing must be closed at the adapter.                                                                                             | **Designable / Not run.** DEBUG/TRACE can expose URI, resolver or wire data; logging must be off/WARN and errors closed.                                                                                                           |
| 14  | Production-shaped seam plus hermetic real-client tests                   | **Designable / Not run.** The static DER-root variant can exercise TLS hermetically with a synthetic root set without mutating OS trust. Exact platform-verifier behavior still requires target-platform/manual evidence, and no test seam repairs production resolver cancellation.                                                                                                      | **Designable / Not run.** Loopback TLS can exercise the post-DNS path, but a test resolver cannot prove production GAI cancellation.                                                           | **Designable / Not run.** Loopback can cover ordinary behavior, not repair the documented non-cancellable resolver.                                                                                                                |
| 15  | Exact removal and pruning                                                | **Documented.** Remove the direct/support declarations and adapter/tests, then resolve back to the no-direct-client baseline; transitive Tauri lock nodes may remain.                                                                                                                                                                                                                     | **Documented.** Reverse the four version updates and remove the added Rustls/direct-client closure.                                                                                            | **Documented.** Remove the direct ureq edge and its 13 lock additions.                                                                                                                                                             |

### Candidate conclusions

| Candidate                          | Closed candidate result | Decisive mandatory failures |
| ---------------------------------- | ----------------------- | --------------------------- |
| Reqwest with platform verifier     | Ineligible              | 5, 8, 9, 10, and 11         |
| Reqwest with static WebPKI roots   | Ineligible              | 8, 9, 10, and 11            |
| Reqwest with native TLS            | Ineligible              | 5, 8, 9, 10, and 11         |
| Explicit Hyper/Tokio/Rustls/WebPKI | Ineligible              | 8, 10, and 11               |
| Ureq/Rustls/WebPKI                 | Ineligible              | 5, 8, 10, and 11            |

The common decisive defect is not merely a missing test. Every hostname-based
path eventually delegates default DNS to blocking operating-system resolution.
Tokio documents that a started `spawn_blocking` task cannot be aborted and may
survive a runtime shutdown timeout; ureq's timed resolver similarly returns
after a channel timeout without stopping its worker. Ignoring a late result is
necessary at the state boundary but does not prove cleanup or quiescence.
Custom resolver authority, a hard-coded address, helper-process termination, or
a fourth transport design would change the frozen candidate or violate this
plan and therefore was not treated as a fallback.

The source, registry, lock, license, advisory, and target-graph corpus required
for a negative decision was acquired successfully. Remaining `Unproved` and
`Not run` runtime cells do not conceal a possible qualifying candidate because
each candidate already has a primary-source-documented mandatory failure.
Therefore the accepted closed disposition is `no_eligible_client`, not
`evidence_boundary_failed` and not `selected_exact_client`.

### Accepted D-118 wording

**D-118: V0-6 has no eligible direct Rust HTTPS client under the current hard
cancellation contract**

- **Status:** Accepted by the owner in this documentation-only V0-6 decision.
- **Decision:** Record V0-6 as `no_eligible_client`. Select no dependency or
  transport. Reqwest 0.13.4 with platform, static WebPKI, or native TLS; the
  explicit Hyper 1.11.1/Tokio 1.53.1/Rustls 0.23.43/WebPKI stack; and ureq 3.4.0
  with Rustls/WebPKI are all ineligible.
- **Reason:** Each frozen hostname-based design reaches operating-system DNS
  through blocking work that cannot be aborted or boundedly joined after it
  starts. It can outlive the applicable 10-second connect and 120-second run/
  total ceilings and terminal cancellation. This violates bounded cleanup,
  complete work ownership/accounting, and quiescence and would force an
  ambiguous, potentially unbounded quarantine that blocks safe restart.
  Reqwest 0.13.4 also lacks exact pre-retention response header/buffer caps.
- **Consequence:** V0-7, the live synthetic-text milestone, and every transport
  successor remain Blocked. No source, manifest, lockfile, dependency,
  credential, signing, provider, network, product, or external-system authority
  is granted. A future attempt requires a separately approved plan that either
  changes the architecture or explicitly reconsiders the hard cancellation
  constraint; neither change is authorized here.
- **Preservation:** V0-3 remains paused/Blocked; D-107 remains historical 8/11,
  D-108 remains additively 9/10, D-113 through D-117 remain Proposed/non-
  controlling, all ten D-107 blockers remain, and no prior decision is waived
  or superseded.

### Primary evidence register

- [Reqwest 0.13.4 release and source](https://github.com/seanmonstar/reqwest/releases/tag/v0.13.4),
  [published manifest](https://github.com/seanmonstar/reqwest/blob/v0.13.4/Cargo.toml),
  [client builder source](https://raw.githubusercontent.com/seanmonstar/reqwest/v0.13.4/src/async_impl/client.rs),
  [sole-root `tls_certs_only` API](https://docs.rs/reqwest/0.13.4/reqwest/struct.ClientBuilder.html#method.tls_certs_only),
  and [closed retry policy](https://docs.rs/reqwest/0.13.4/reqwest/retry/fn.never.html).
- [Hyper 1.11.1 HTTP/1 connection handshake](https://docs.rs/hyper/1.11.1/hyper/client/conn/http1/fn.handshake.html),
  [connection ownership](https://docs.rs/hyper/1.11.1/hyper/client/conn/http1/struct.Connection.html),
  [HTTP/1 bounds](https://docs.rs/hyper/1.11.1/hyper/client/conn/http1/struct.Builder.html),
  and [Hyper-util GAI resolver source](https://docs.rs/hyper-util/0.1.20/src/hyper_util/client/legacy/connect/dns.rs.html).
- [Tokio hostname-resolution source](https://docs.rs/tokio/1.53.1/src/tokio/net/addr.rs.html)
  and [non-abortable blocking-task contract](https://docs.rs/tokio/1.53.1/tokio/task/fn.spawn_blocking.html).
- [Ureq 3.4.0 published source](https://docs.rs/crate/ureq/3.4.0),
  [configuration/timeout API](https://docs.rs/ureq/3.4.0/ureq/config/struct.ConfigBuilder.html),
  and [resolver source](https://docs.rs/ureq/3.4.0/src/ureq/unversioned/resolver.rs.html).
- [Rust `JoinHandle` semantics](https://doc.rust-lang.org/std/thread/struct.JoinHandle.html)
  document that dropping a handle detaches the thread and allows it to outlive
  its spawner.
- [Rustls client configuration](https://docs.rs/rustls/0.23.43/rustls/client/struct.ClientConfig.html),
  [WebPKI roots 1.0.9](https://docs.rs/webpki-roots/1.0.9/webpki_roots/),
  [WebPKI DER root certificates 1.0.9](https://docs.rs/webpki-root-certs/1.0.9/webpki_root_certs/constant.TLS_SERVER_ROOT_CERTS.html),
  and [RustSec snapshot](https://github.com/RustSec/advisory-db/tree/5a0ebedfe8bdd2e295b171f4162f8c977bcad9a5).
- The official
  [crates.io index](https://github.com/rust-lang/crates.io-index) and the
  checksum-verified registry archives supplied release, checksum, manifest,
  feature, license, and MSRV metadata for the complete resolved graphs.

## Closed dispositions

The decision has exactly three terminal documentation outcomes:

- `selected_exact_client`: at least one candidate passes every mandatory row
  and the security-first ranking produces exactly one selected winner. Record
  every qualifying candidate, the deterministic ranking, the winner's exact
  versions, features, TLS/root/HTTP behavior, full proposed graph and lock
  delta, disqualifications or ranking of the alternatives, test obligations,
  and removal procedure. This is input to a later V0-7 source proposal, not
  permission to change a dependency or send traffic.
- `no_eligible_client`: the complete frozen source corpus and hypothetical
  graph were acquired and reviewed successfully, and every candidate has at
  least one documented failed mandatory criterion. An official source that
  expressly denies a required control or does not offer a mandatory guarantee
  counts as that candidate's failed criterion. This is a valid negative
  documentation result; V0-7 and every transport successor remain Blocked.
- `evidence_boundary_failed`: acquisition, provenance, freshness, integrity,
  resolver isolation, or graph completeness cannot be established; the
  evidence is inconsistent or drifts; or multiple qualifying candidates remain
  tied after the complete deterministic ranking. Stop without classifying
  missing evidence as candidate ineligibility and without a selection,
  fallback, positive decision, or completion claim.

The owner accepted the exact `no_eligible_client` disposition and D-118 wording
before final documentation reconciliation. That acceptance closes only this
documentation decision; it does not authorize a dependency, source change,
transport, or successor.

## Interfaces and invariants for later work

V0-6 creates no executable interface. Any later selected-client decision must
preserve these invariants:

- one application-owned client type behind a narrow Rust adapter;
- no caller-selected origin, client, TLS profile, root, proxy, redirect,
  retry, runtime, provider, model, identity, workflow, or fallback;
- one application-level HTTP request and one credential/header/body
  transmission at a time, with no implicit replay; any finite pre-transmission
  connection attempts remain deadline-bound, cancellation-owned, and unable to
  duplicate credentials or body;
- fixed content and response-type expectations with limits before retention;
- cancellation and deadline behavior independent of UI polling;
- cleanup ownership that cannot report quiescence while client-owned work may
  still mutate state;
- no late post-terminal mutation or disclosure;
- no WebView network or secret authority, Tauri HTTP capability, persistence,
  filesystem, tool, approval, audit, background, or device-effect expansion;
- provider-specific and client-native types remain inside their adapters; and
- complete dependency removal restores the current no-direct-client state.

## Threats and mitigations

| Threat                                                                                                      | Required fail-closed treatment                                                                                                                                                                                            |
| ----------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Transitive dependency laundering or feature-unification drift                                               | Require a direct exact declaration proposal and whole-graph feature evidence; transitive presence conveys no authority                                                                                                    |
| SSRF or credential exfiltration through redirects, proxies, DNS, URL input, cookies, netrc, or ambient auth | Fixed application-owned origin and method; deny every ambient/caller routing or authentication input before connection                                                                                                    |
| MITM through TLS/root drift or bypass                                                                       | Freeze backend/root semantics and verification; reject any permissive callback, custom production trust, or unproved platform behavior                                                                                    |
| Duplicate or replayed provider requests                                                                     | Prove exactly one application-level HTTP request and one credential/header/body transmission; bound and own any multi-address connection attempts; deny post-transmission reconnect, retry, redirect, fallback, or replay |
| Memory or CPU exhaustion                                                                                    | Bound headers, frames, decompression, chunks, retained bytes, buffering, backpressure, pools, and task count before application retention                                                                                 |
| Slowloris, stalled DNS/TLS, or abandoned work                                                               | Enforce all four monotonic deadlines below the UI and account for every client-owned task/socket during cleanup                                                                                                           |
| Late-result mutation after cancellation or terminal state                                                   | Close ingress first; validate attempt/terminal state before every mutation; quarantine ambiguous cleanup                                                                                                                  |
| Secret or content disclosure                                                                                | Map all native failures to closed redacted errors and canary-test every diagnostic surface                                                                                                                                |
| Supply-chain, license, MSRV, build-script, or native-code growth                                            | Require exact primary evidence and hypothetical full graph; any unreviewed node or advisory makes the candidate ineligible                                                                                                |
| Stale public evidence or scratch-workspace contamination                                                    | Freeze evidence dates/versions, isolate resolution, verify the repository remains unchanged, and stop on drift                                                                                                            |
| Documentation result overstated as live proof                                                               | Keep target-Mac, socket, TLS, provider, and product tests explicitly Not run and V0-7 Blocked                                                                                                                             |

## Ordered milestones for the active decision increment

1. Reconfirm clean synchronized `main`, the exact baseline, no overlapping
   user changes, no active gate, the D-118 slot, and the fifteen-file ceiling.
2. With separate owner approval, create the named branch and run
   `python3 .codex/hooks/post_increment_gate.py begin --increment personal-assistant-v0-https-dependency-decision`.
3. Freeze exact releases and collect only separately authorized primary-source,
   license, advisory, maintenance, MSRV, platform, TLS, streaming,
   cancellation, cleanup, and testability evidence.
4. Generate and inspect the hypothetical dependency graph and lock delta only
   in the authorized isolated scratch workspace; verify the repository remains
   byte-unchanged outside the plan's documentation scope.
5. Complete every matrix cell and select exactly one closed disposition.
6. Draft the candidate D-118 result and stop for explicit owner acceptance.
7. If accepted, reconcile only the exact fifteen documentation paths. If
   rejected, record the applicable negative or evidence-failure outcome; do
   not substitute a candidate or widen scope.
8. Run documentation, repository, security, readiness, architecture, and exact-
   scope reviews plus the required completion checks.
9. Record actual Passed, Failed, Pending, and Not-run evidence. Stop without
   dependency/source changes, publication, or successor start.

## Required tests for a later V0-7 proposal

The selected decision must make V0-7 specify, but does not itself run, tests for:

- exact fixed scheme/host/port/path/method and rejection before connection of
  wrong or caller-controlled values; exactly one application-level request and
  credential/header/body transmission even if bounded pre-transmission multi-
  address connection attempts occur;
- denial of every redirect class, proxy environment spelling, platform proxy,
  cookie, netrc, ambient auth, custom resolver/root, retry, reconnect, and
  fallback;
- TLS success with a synthetic test chain and rejection of wrong hostname,
  expired/not-yet-valid/unknown-root/malformed chains, and disallowed protocol;
- exact HTTP version and content type/status; header-count/byte caps; malformed,
  conflicting, truncated, compressed, or oversized framing;
- arbitrary byte, UTF-8, CRLF, SSE-line/event splits, N/N+1 limits,
  backpressure, and no unbounded buffering;
- deadlines and cancellation during DNS, connect, TLS, headers, before first
  byte, mid-frame, between frames, and terminal races;
- original-exchange abort, owned-task join/quiescence, cleanup failure
  quarantine, restart denial/allowance, late-result rejection, and zero second
  request; and
- canary redaction across URLs, headers, credentials, request/response bytes,
  native errors, logs, traces, metrics, and support output.

### V0-7 plan discrepancy

The current V0-7 plan requires proof of actual TLS, redirect, proxy, and
original-socket abort semantics while also saying all tests use a fake seam and
no socket is opened. A fake seam can prove application state, but not an HTTP
client's TLS or socket cleanup. Before V0-7 may begin, a separately approved
documentation correction must require hermetic loopback HTTP/TLS integration
tests with synthetic certificates and values and zero external network.
Production code must not accept test roots, resolver injection, or equivalent
test authority. This advisory does not block V0-6; it independently keeps V0-7
Blocked.

## Validation

### This planning revision

Run documentation-tier baseline validation only:

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
```

Also verify that the diff contains exactly the seven planning paths and no
source, manifest, lockfile, gate, or ignored evidence change. Do not run a
session-end or post-increment gate because the increment is not begun.

### Active decision increment after owner acceptance

After its last authorized documentation edit, run:

```bash
npm run docs:check
npm run repository:check
npm run security:scan
npm run verify
git diff --check
python3 .codex/hooks/session_end_gate.py
```

Then run the repository-prescribed independent documentation, architecture,
security, readiness, exact-path, session, and post-increment reviews. Because
this is a security-sensitive dependency decision, `npm run verify` is required
even though no dependency changes. Candidate compilation, target-Mac transport,
TLS, credential, provider, and external-network checks remain `Not run`.

## Manual gates

- **Passed:** the owner separately approved the documentation increment,
  branch, evidence boundary, and `begin` command.
- **Passed:** the owner explicitly accepted the proposed closed disposition and
  exact D-118 wording after the matrix was complete and before closeout.
- A later owner approval must separately authorize any V0-7 plan correction,
  dependency/source edit, test socket, build, credential use, or traffic.
- No later milestone begins automatically.

## Rollback

For this planning revision, rollback is an exact reversal of the seven
documentation changes. For the later decision increment, rollback is an exact
reversal of its fifteen documentation paths. Before removing an isolated
resolver workspace, validate the exact recorded `mktemp` root, confirm it is
the expected non-symlink scratch directory, and remove only that root without
copying artifacts into the repository. Any uncertainty leaves the directory
quarantined and reported rather than permitting broad deletion. Since no
manifest, lockfile, source, credential, provider, or operational external state
may change, rollback requires no product, dependency, signing, provider, or
cloud operation.

## Stop conditions

Stop immediately if:

- the checkout is dirty in an overlapping path, divergent, unsynchronized, or
  does not match the recorded authority;
- the D-118 slot collides, the fifteen-path ceiling must expand, or a fourth
  candidate is needed before a plan amendment;
- public-source or resolver access is not separately authorized, cannot be
  isolated, or would expose personal/sensitive state;
- any evidence cell is missing, stale, ambiguous, contradictory, or depends on
  an undocumented default;
- a candidate requires unsafe TLS, custom production trust, caller or ambient
  routing/authentication, automatic retry/fallback, multiple clients, an
  unowned task/socket, process-kill cancellation, or unbounded buffering;
- a candidate exceeds MSRV, license, advisory, target, dependency, or rollback
  constraints;
- tracked source, manifest, lockfile, configuration, capability, credential,
  signing, provider, or external-system work becomes necessary;
- validation fails or the diff exceeds the authorized paths; or
- the owner has not accepted the proposed disposition before reconciliation.

If an approved active increment reaches `evidence_boundary_failed`, record a
truthful Failed/Blocked completion disposition under the repository gate rules;
do not finalize it as a passing selection or start V0-7.

## Acceptance criteria

This planning revision is complete when:

- [x] the current-baseline ExecPlan and authoritative memory/index files agree
      that V0-6 completed with `no_eligible_client` and no transport authority;
- [x] the candidate register, evidence matrix, three dispositions, threats,
      invariants, tests, owner gates, rollback, and stop conditions are closed;
- [x] V0-3 and V0-7 remain Blocked and no source/dependency/external authority is
      implied;
- [x] documentation, repository, security, complete verification, exact-scope,
      independent-review, session, and completion-gate validation passes; and
- [x] the repository contains exactly the fifteen authorized documentation
      changes and no protected product/dependency path change.

The active decision increment is complete only after its separate approvals,
an owner-accepted closed disposition, exact fifteen-file reconciliation,
truthful evidence, and valid required completion gate. A selected dependency
still requires a separately approved V0-7 correction and implementation plan.

## Progress

- 2026-09-02: Reconfirmed clean synchronized `main` at
  `6c3615c507b58a771f696ebb7bfdb5bd13d1d1b7` and inspected current manifest,
  lockfile, Personal Assistant source boundaries, V0 plans, decisions, and
  project memory.
- 2026-09-02: Confirmed no direct HTTPS client or production Personal Assistant
  response ingress exists; transitive lock nodes convey no authority.
- 2026-09-02: Froze the three-family candidate register, three closed outcomes,
  fifteen-file future decision ceiling, owner checkpoints, and V0-7 test-plan
  advisory.
- 2026-09-02: The first `npm run docs:check` found only Prettier formatting in
  this revised plan. The repository formatter corrected that authorized file;
  the subsequent documentation, repository, security, and diff checks passed.
- 2026-09-03: Reconfirmed the exact synchronized baseline, preserved the seven
  approved documentation changes, created the authorized branch, confirmed the
  D-118 slot was free, and began the named increment. The first sandboxed
  `begin` attempt could not write ignored gate state; the identical escalated
  command passed and the gate reports this increment `active`.
- 2026-09-03: Retrieved frozen public release, source, registry, license, and
  advisory evidence without authentication. The RustSec snapshot is
  `5a0ebedfe8bdd2e295b171f4162f8c977bcad9a5`.
- 2026-09-03: Resolved all five exact hypothetical variants in an isolated
  scratch Cargo home for macOS and Linux, inspected target graphs and lock
  deltas offline, and ran no compilation, candidate code, or build script.
  One optional unfiltered all-target metadata diagnostic failed because
  `android_system_properties 0.1.5` was not cached; the scoped macOS/Linux
  metadata and tree commands passed and are the controlling evidence.
- 2026-09-03: Compared each full hypothetical lock against the frozen RustSec
  database. Every graph reproduced the baseline's exact 20 accepted entries
  and added none. `cargo-audit` was unavailable and was not installed.
- 2026-09-03: Independent security review identified that the first static-root
  reqwest fixture used the wrong root-data crate and understated reqwest's
  stable API. Re-resolution with `webpki-root-certs 1.0.9` confirmed the stable
  `tls_certs_only` path, corrected the full-lock count to 455 (`+21/-1`), and
  preserved the 280-node macOS and 318-node Linux graphs. The corrected package-
  version set matches the already scanned platform graph, so its advisory
  result remains the same exact 20 accepted baseline entries and no candidate-
  added finding. The correction removes TLS row 5 from that variant's failures;
  rows 8, 9, 10, and 11 still make it ineligible and do not change the proposed
  disposition.
- 2026-09-03: Completed the mandatory evidence matrix. Every candidate has a
  documented mandatory cancellation/cleanup failure. At that interim
  checkpoint, the only proposed closed disposition was `no_eligible_client`;
  proposed D-118 remained Pending explicit owner acceptance and non-controlling.
- 2026-09-03: The first interim `npm run docs:check` found only formatting in
  this active plan. The repository formatter changed only this authorized file;
  the rerun and `git diff --check` passed. Exact-scope inspection confirmed only
  the original seven approved documentation paths differ from the synchronized
  baseline and all protected source/manifests/lockfiles are unchanged.
- 2026-09-03: Validated each isolated scratch root as its exact expected
  non-symlink directory, removed only those roots after capturing the sanitized
  evidence, and confirmed neither remains. No scratch or resolver artifact was
  copied into the repository.
- 2026-09-03: The owner explicitly accepted `no_eligible_client` and the exact
  D-118 wording. Reconciled only the fifteen authorized documentation paths;
  D-118 closes V0-6 without selecting a dependency or transport.
- 2026-09-03: Documentation, repository, security, complete verification,
  exact-scope, protected-path, append-only decision, independent architecture/
  security/documentation/code/debt/readiness, session, and deterministic
  post-increment checks passed. The completion marker validates for this exact
  workspace. V0-3 and V0-7 remain `Blocked`.

## Discoveries and decisions during planning

- A one-file Ready label would contradict `PLANS.md`, `NEXT_STEPS.md`, and the
  current memory chain. Seven documentation files are therefore the minimum
  truthful planning scope.
- V0-6 is technically separable from the paused V0-3 signing lane because it
  selects documentation constraints only. It does not make the live transport
  path executable.
- Existing transitive HTTP/runtime packages may reduce or enlarge a future
  feature graph, but their presence cannot select a client or replace a direct
  dependency review.
- Real-client TLS, redirect, proxy, cancellation, and socket-quiescence proof
  requires hermetic loopback integration tests in a later increment; fixture-
  only tests cannot establish those properties.

## Final results

Five exact variants across the three frozen families were evaluated, and every
one is ineligible. The owner accepted the closed result `no_eligible_client`
and exact D-118 wording; no dependency or transport is selected.

Final reconciliation changes exactly fifteen authorized documentation paths.
Documentation, repository, security, complete `npm run verify`, diff,
exact-scope, protected-path, append-only decision, independent review, session,
and post-increment validation pass. The result is `PASS WITH ADVISORIES`, and
the completion marker is valid for the exact workspace. Candidate build/
runtime, `cargo-audit`, target-Mac transport/TLS/socket/cancellation,
credential, signing, provider, gateway, and external product-system checks are
`Not run`. One optional unfiltered all-target metadata diagnostic failed due an
uncached Android-only crate; the controlling macOS/Linux scoped evidence
passed. The validated disposable scratch roots were removed. No source,
manifest, lockfile, dependency, credential, signing, provider, product, or
operational external state changed.

## Readiness

**Blocked for every operational successor.** The V0-6 documentation decision is
complete with `PASS WITH ADVISORIES`, and D-118 controls only its accepted
negative dependency result. V0-3, V0-7, the live synthetic-text milestone, and
every operational successor remain `Blocked`. No successor is Ready or active.
