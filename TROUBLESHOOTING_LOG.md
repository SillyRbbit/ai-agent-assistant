# Troubleshooting log

## Optional API status repair — 2026-09-22

The owner's time-boxed optional API task split the previously ambiguous
`model_unavailable` mapping for HTTP 400/403/404 into `http_bad_request`,
`http_forbidden` and `http_not_found`, with static messages and no response-body
inspection. Legacy frontend messages remain readable. The latest historical
failure cannot identify the provider/account/request cause. Models, fixed sample,
request limits, endpoint, transport, ownership and no-retry behavior are unchanged.

Focused checks passed (38 frontend, 17 native); full offline `npm run verify`
passed (421 frontend, 337 Rust library tests, integration suite, 74 hook and
85 repository tests, strict lint/type checks and release build). The offline
locked app-only no-sign debug bundle passed identity checks. Its executable
SHA-256 is `e3f329d126153251d618e52d5ccfb815430b300af8280672c6d08a2ea158555e`.
Documentation, repository, security and whitespace checks passed after correcting
new-prefix blank-line formatting in this same task. The repair is ready for
ordinary finalization with advisories. The separately authorized
optional fixed-sample batch is currently 0/1 used, awaiting owner-only private
launch and acknowledgement; no new request has been made. API live success
remains unverified and does not block independently verified Agents/settings/
private-notes behavior. Codex live remains disabled under its existing advisory.

Preserve all inherited changes and terminal records. D-128 custody/abort limits
and D-127 audit debt remain; D-125/M1/M2 stay parked. No commit/publication or
additional API investigation follows automatically. See the
[bounded plan](docs/plans/2026-09-22-optional-api-status-repair.md).

## Direct provider stream stage live-result evidence closeout — 2026-09-21

**Observed evidence:** Direct Computer Use verified the idle, enabled, disclosed
fixed-sample state, then reported an inactive native binding before returning any
post-click state. The owner-supplied screenshot separately shows Cortexa in
terminal `error` with the closed `provider_stream_error_event` message, no
completed answer and the static statement that no automatic retry occurred.

**Reconciliation:** The screenshot resolves request accounting: the final
authorized request reached Cortexa and was consumed, leaving zero authorized
attempts. The Computer Use interruption remains an observation-layer failure and
does not erase the owner-supplied native result.

**Limits:** No raw provider error, request identifier, credential, header, body or
process environment was inspected. The evidence does not identify upstream cause,
exclude earlier transient text, confirm ownership release or establish native
live success. No retry, launch, build, application test or provider request is
part of this documentation-only closeout. D-128, D-127 and native live-success
advisories remain; D-125/M1/M2 stay parked.

## Direct provider stream stage diagnostics evidence closeout — 2026-09-21

The stream-stage diagnostics predecessor did not fail an application test or
build. Its passing completion report first needed two external validation
corrections, then its draft `Readiness` finding category was rejected because
the repository allows only fixed categories. The predecessor was truthfully
closed `FAIL / Blocked`. This successor validated its own report, finalized
separately and preserved the original evidence unchanged. Any later validation
failure still requires a separate bounded decision rather than an automatic repair.

## Direct provider stream stage diagnostics — 2026-09-21

The owner authorized one bounded diagnosis, minimal safe diagnostics, and one
additional acknowledged native request after local validation. The prior request
showed `starting` then `provider_stream`, without displayed text; controls were
released and the process stopped. This does not identify the upstream cause.
The verified predecessor at `3f99165b4dcb0ef18c52f9242b346eb1d711afaa` remains unchanged with its valid
completion record. A new detached worktree at
`/private/tmp/cortexa-direct-provider-stream-stage-diagnostics` received ordinary
admission as `direct-provider-stream-stage-diagnostics`. All predecessor records and D-125/M1/M2 remain preserved;
those parked lanes are not resumed.

The exact 15-path delta separates a top-level `error` event from `response.failed`
with an unknown string code or an absent/malformed/empty code. Existing three
recognized response-error mappings stay intact. Only closed static error codes and
messages cross the existing snapshot boundary; no raw code, body, header, request
ID, prompt, credential or provider message is logged or persisted. The request,
transport, validation order, lifecycle, ownership, no-retry policy and component
production bytes remain unchanged. No parser defect or account cause is claimed.

Application validation passed: 24 focused frontend tests, 13 focused Rust tests,
complete offline verification, documentation/repository/security checks,
whitespace and exact preservation. Full verification included 74 hook tests,
83 repository tests, 395 frontend tests, 315 Rust library tests and 247 Rust
integration tests (one pre-existing ignored test), plus strict lint/type checks
and builds. The app-only debug bundle rebuilt offline with locked dependencies
and no signing. Its identifier is com.aiagentassistant.desktop; executable SHA-256
is f2cf3f5c998b072ffb47dce81c3047c0473307484b0e68006e21c27a3bb58e7d.
An initial extra bundle check incorrectly required unused Rust Display strings
in the executable. One external validation correction checks serialized native
codes and generated frontend static messages instead; both passed. No source
repair, download or repeated build was needed.

Independent application review found no code blocker, but completion failed.
The external report wrapper first failed to import common; adding the hook import
path was external correction 2. The next run rejected the draft finding category
Readiness. With the two-correction allowance treated as exhausted, work stopped
before the paid request. The terminal report records FAIL / Blocked using the
existing schema and ordinary close-failed route; terminal validity and Stop are
checked separately after its freeze. This is not passing completion.

The one additional request remains unused. No app was launched and no test-owned
native process needs cleanup. Preserve the tested code, rebuilt bundle and all
predecessor records. Live success, remaining GUI smoke, D-128 custody/abort limits
and D-127 audit debt remain advisories. No automatic successor, commit or
publication is authorized.

See the [plan](docs/plans/2026-09-21-direct-provider-stream-stage-diagnostics.md) and [review](docs/reviews/2026-09-21-direct-provider-stream-stage-diagnostics-post-increment-review.md).

## 2026-09-21 — Direct provider diagnostics evidence closeout validator retry

**Symptom:** The terminal documentation retry’s preservation validator raised
`TypeError` before proving its scope because it compared the string
`"node_modules"` to bytes returned by its Git helper.

**Cause:** The helper intentionally returns raw bytes for hashes and NUL-safe Git
path parsing; only call sites needing text decode their specific output.

**Correction and result:** The new external validator retained `entry()` returning
`None` for absent paths and changed only the membership assertion to
`b"node_modules"`. The local formatter repaired eight in-scope formatting findings
without installation. Documentation, repository, secret, whitespace,
preservation, session, quality, report-schema and completion evidence passed.

**Boundaries:** No application build, provider request, credential inspection or
live rehearsal occurred. All attempts remain exhausted; native GUI/live-success
advisories remain, and D-125/M1/M2 stay parked.

## Direct provider diagnostics evidence closeout retry — 2026-09-21

The prior closeout failed before formatting because a new detached worktree did
not include local node_modules/.bin/prettier. It also failed because its
external validator called stat on an allowed successor-only path. This retry
uses an APFS clone of the known existing local dependency tree and a new external
validator whose entry helper returns None for absent paths. Its later comparison
used byte output from git status against a string literal and raised TypeError.
The terminal source and failed validator remain unmodified; this retry was not
repaired or retried.

## Direct provider diagnostics evidence closeout — 2026-09-21

The prior candidate's implementation evidence passed, but the ordinary completion
report was initially rejected for a noncanonical `## Scope and preservation`
heading and then for combined review headings. The terminal failure report was
made schema-valid and preserved, but the increment could not be reopened. This
separate successor uses the repository template from the beginning and treats
previous application checks as historical evidence. Its documentation check then
failed before execution because the new detached worktree lacked local `prettier`.
Its initial scope validator also raised `FileNotFoundError` while comparing the
successor-only report to the predecessor. Per the stop condition, no dependency
copy, installation, validator repair or retry occurred.

## Direct provider stream diagnostics message assertion — 2026-09-21

The terminal predecessor's three UI failures were caused by a shared assertion
that required the generic “No mock response was substituted.” suffix after the
new rows had already displayed their approved “No automatic retry was made.”
messages. The isolated corrective successor makes the expected policy suffix an
explicit row value and changes only that shared assertion. No timer, production,
transport or ownership defect was established. All 18 focused frontend tests,
43 focused Rust tests and complete offline verification passed. The ordinary
completion gate then rejected `## Scope and preservation`; its schema requires
exactly one `## Scope and boundaries` section. Per the stop condition, no repair
or retry occurred and the successor was recorded terminal failed. The earlier
failed predecessor remains preserved as historical evidence.

## Direct provider stream diagnostics — 2026-09-21

The owner-approved `direct-provider-stream-diagnostics` successor is isolated in
`/private/tmp/cortexa-direct-provider-stream-diagnostics`, detached at
`0ed15810e90b6a4bd312a8096c61b0abb1ab7eff`. The verified 34-path candidate was
transferred byte-for-byte before ordinary admission. All predecessor checkouts,
finalized reports and raw gate states remain preserved; D-125/M1/M2 stay parked.

The diagnostic delta classifies only response.failed response.error.code values
server_error, rate_limit_exceeded and invalid_prompt into three fixed codes and
static messages. Top-level error and other/malformed codes retain provider_stream.
No request, transport, validation, ownership, dependency or permission change.

Required frontend validation failed: 15 passed and 3 new UI cases failed because
the reused assertion requires “No mock response was substituted.” while the
approved new messages end with “No automatic retry was made.” No repair or retry
was performed. The already-running focused Rust check completed: 43 passed.
Full offline verification was not run. Quality is FAIL; readiness is Blocked.
See the [plan](docs/plans/2026-09-21-direct-provider-stream-diagnostics.md) and
[review](docs/reviews/2026-09-21-direct-provider-stream-diagnostics-post-increment-review.md).

All three live rehearsal attempts are exhausted. The previous native observation
was provider_stream with released ownership and no completed answer; its detailed
cause remains unknown. Native live success, remaining GUI smoke, D-128 custody/
abort limitations and D-127 dependency advisories remain. No launch, credential
inspection, live request or publication occurred in this successor.

## Direct diagnostics test sequencing — 2026-09-21

The owner explicitly authorized a separate isolated corrective successor,
`direct-provider-diagnostics-test-sequencing`, in
`/private/tmp/cortexa-direct-provider-diagnostics-test-sequencing`, detached at
`0ed15810e90b6a4bd312a8096c61b0abb1ab7eff`. The complete 32-path diagnostic
candidate was copied and byte-verified before ordinary admission. Both predecessor
checkouts, finalized reports and raw gate states remain unchanged; the earlier
FAIL / Blocked record is not reopened or promoted.

The only executable delta splits the new UI regression's 500ms fake-timer act
into separate 250ms acts and asserts streaming between them. The next poll is
scheduled by a snapshot-dependent effect; all later error, cleanup, release and
no-retry assertions remain unchanged. All production and dependency bytes remain
identical to the diagnostic candidate. No implementation is repeated.

Local validation passed: 12 focused frontend tests, 41 focused Rust tests and
full offline verification. Quality is PASS WITH ADVISORIES; require a valid
ordinary completion marker before treating closeout as complete. See the [plan](docs/plans/2026-09-21-direct-provider-diagnostics-test-sequencing.md)
and [review](docs/reviews/2026-09-21-direct-provider-diagnostics-test-sequencing-post-increment-review.md). No app launch, credential inspection or live request is included.
One rehearsal attempt remains. The earlier native provider failure remains
unexplained; the Python/OpenSSL TLS probe is not Rust-client evidence. Native
live success and remaining GUI smoke remain advisories; D-125/M1/M2 stay parked.

## Direct provider failure diagnostics — 2026-09-21

The owner-approved `direct-provider-failure-diagnostics` successor is isolated in
`/private/tmp/cortexa-direct-provider-failure-diagnostics`, detached at
`0ed15810e90b6a4bd312a8096c61b0abb1ab7eff`. The completed implementation candidate
was transferred byte-for-byte before ordinary admission. Its original checkout,
valid completion record, all other checkouts and already-prunable entries remain
preserved. D-125/M1/M2 remain parked.

The bounded change keeps `network` for non-timeout transport failures and adds
payload-free `http_status` and `provider_stream` codes with static messages.
Existing special HTTP mappings, timeouts, stream validation, request construction,
credential handling, ownership, cancellation and retry policy are unchanged.
No raw errors, provider bodies, headers or identifiers are exposed or logged.
The earlier failure remains unexplained. A prior Python/OpenSSL DNS/TCP/TLS
probe passed; it is not Rust/rustls, authentication or successful response proof.
Two rehearsal attempts were used; one remains. This increment makes no requests.
Native live success and the remaining Research/Knowledge GUI advisory stay pending.

Required focused validation failed (9 passed, 3 new UI cases failed); see the [plan](docs/plans/2026-09-21-direct-provider-failure-diagnostics.md)
and [review](docs/reviews/2026-09-21-direct-provider-failure-diagnostics-post-increment-review.md). Quality: FAIL; next-increment readiness: Blocked. Implementation stopped without
repair or retry. The three new cases expected an alert while the rendered state
was still streaming. Test-timing is a hypothesis, not a verified root cause.
Rust tests and full verification were not run after this stop condition.
Preserve these edits and the terminal failure; do not reopen this increment.

## 2026-09-21 — Direct implementation local diagnostics

Scope: isolated `personal-assistant-direct-implementation`, not D-125/M1/M2.
Offline Cargo resolution lacked tokio-macros; the authorized minimal HTTPS
resolution fetched missing registry packages, retaining unrelated old versions.
Intermediate compilation while integration was incomplete and strict frontend
lint findings were corrected in scope. Async React test callbacks now await
their microtask flush; the no-Debug provider event test checks errors without
requiring content Debug. Focused Rust/frontend/checker checks passed afterward.
The pinned cargo-audit binary was reused from
`/private/tmp/cortexa-cargo-audit-0.22.2/bin/cargo-audit`; plain `cargo audit`
was absent, so no toolchain was installed. The unchanged Cargo gate passed.
Final completion outcomes are in the [review](docs/reviews/2026-09-21-personal-assistant-direct-implementation-post-increment-review.md); do not treat
these intermediate diagnostics as final acceptance or native live evidence.

The first full verify stopped at Clippy's existing no-unwrap rule in the new
Rust tests. Tests now propagate Result/Option failures; no lint suppression was
added. Focused strict Clippy passed. Final dependency review removed the unused
tokio macros feature before the completion rerun; only runtime/time are needed.

Use this file for resolved and unresolved environment, build, test, and runtime failures. Preserve history so later sessions do not repeat the same investigation.

## TS-025 - Withdrawn gtk3-rs advisories made the exact Cargo baseline stale

Date: 2026-09-20
Status: Corrected by D-127; required local validation passed

### Symptom

After the JavaScript dependency candidate passed its focused checks, current
cargo-audit 0.22.2 output failed the repository gate because ten warnings in the
accepted set were absent. The tool still reported the exact two quick-xml
vulnerabilities and eight other accepted warnings.

### Cause

RustSec withdrew RUSTSEC-2024-0411 through RUSTSEC-2024-0420 on 2026-08-14
after gtk3-rs resumed maintenance. cargo-audit omits withdrawn advisories by
default, while the repository's exact gate treated their absence as baseline
drift. This was not target filtering or a cargo-audit parsing error.

### Disposition

D-127 removes only those ten tuples, preserves all current accepted findings,
and tests that each withdrawn advisory fails as unexpected if it reappears. It
adds no ignore or automatic advisory synchronization. Withdrawal reflects
maintenance status and does not prove GTK 0.18.2 is vulnerability-free.

### Avoid repeating

Do not restore withdrawn IDs merely to satisfy historical output, use a blanket
ignore, or repeat npm dependency resolution. Reuse the validated package bytes
only when their baseline blobs still match remote main, then require current
live npm/RustSec audits and the exact gate before publication.

## V0-6 frozen HTTPS candidates cannot prove DNS cancellation quiescence

Date: 2026-09-03
Status: Closed as accepted D-118 `no_eligible_client`; V0-7 remains Blocked

### Observation

Five frozen variants across reqwest, explicit Hyper, and ureq can return
control on a timer, but each hostname-based path ultimately starts blocking
operating-system DNS work that cannot be aborted or boundedly joined. Reqwest
0.13.4 also lacks exact pre-retention response-header and read-buffer caps.

### Disposition

Do not select a dependency or transport. A timeout return, dropped future,
detached worker, or rejected late result is not cleanup or quiescence. The
frozen source and isolated resolver corpus is complete enough for this scoped
negative decision; the result is not a universal Rust HTTPS impossibility
claim. V0-3, V0-7, and the live synthetic-text milestone remain `Blocked`.

### Avoid repeating

Do not repeat the same public-evidence retrieval or hypothetical resolution
without a new approved plan and a materially changed architecture, candidate,
or security requirement. Transitive `reqwest`, `hyper`, and `tokio` lock nodes
do not confer authority. Any later proposal must address hard cancellation,
owned cleanup/quiescence, and hermetic actual-client TLS/socket tests before
source or dependency work can be considered.

## Existing late-event rejection does not establish a private-key late-result boundary

Date: 2026-09-02
Status: Documented as proposed D-117; acceptance pending

### Observation

Current source contains no private-key attempt host or ownership-bound result
ingress for the frozen D-107 candidate. Existing agent-runtime, Personal
Assistant fixture, sealed demo, gateway, approval, and orchestration tests
reject late or foreign events only within their own domains.

### Disposition

The repository-only assessment selects
`late_result_rejection_not_accepted`; missing, ambiguous, contradictory,
unbounded, or drifted evidence selects `boundary_failed`. Proposed D-117 does
not treat result disposal as cancellation or cleanup, and it cannot establish
that synchronous private-key use stopped or that OS-managed effects were
absent. D-107 remains 8/11, D-108 remains additively 9/10, all ten blockers
remain, and readiness stays `Blocked`. No signing, product, target-Mac, or
operational external-system check ran; read-only Git synchronization with
`origin` was the sole external contact.

## TS-024 - Post-D-111 D-107 decisions lacked durable canonical lineage

Date: 2026-09-02
Status: Documented with proposed lineage; acceptance pending

### Symptom

Four published documentation-only contract decisions changed only their
plan/increment/review triplets. None added its claimed result to
`DECISIONS.md` or reconciled current project memory. The private-key triplet
called itself D-112, which later became the unrelated GUI wheel decision; the
fixed-algorithm, interaction-denial, and hard-deadline triplets inherited
noncanonical predecessor numbers. The private-key plan also promised an exact
fifteen-file closeout but its commit and report contain only three files; the
other plans did not freeze their promised inventory. All four reports record
no findings despite those limitations.

### Cause

Each closeout used the ignored single-slot gate record as its local completion
mechanism but omitted the durable decision and current-memory updates. Later
gate activity cannot recover or independently validate overwritten marker
history. A syntactically valid historical report and digest are not proof of
unrecorded commands or missing documentation changes.

### Proposed reconciliation

If accepted, proposed D-113 through D-116 would additively reconcile the four
already selected negative dispositions without changing the historical
triplets or GUI D-112. Every D-107 row, the 8/11 historical record, D-108's
additive 9/10 interpretation, all ten blockers, and `Blocked` readiness would
remain unchanged. The proposal does not retroactively validate any historical
report or marker and adds no operational authority.

### Verify

Require exact byte/hash comparison for all twelve historical artifacts, an
append-only check for GUI D-112, the exact fifteen-path current change set,
full documentation/repository/security/verification checks, independent
reviews, and a new valid completion marker for only this reconciliation.

## TS-023 - PR #102 audit reported Browserslist advisories

Date: 2026-09-02
Status: Resolved

### Symptom

PR #102 CI passed repository policy, documentation, frontend, Linux Rust, and
target-Mac Rust validation but failed `npm audit --audit-level=low` on the
existing development-only `browserslist@4.28.2` transitive. The audit reported
GHSA-c83g-rgw3-j3cx / CVE-2026-73089 and GHSA-73wf-gq98-2v4g /
CVE-2026-73088, both patched in 4.28.7.

### Cause

The lockfile still selected a release within the newly published affected
range through the existing Vite/Babel tooling path. This was not a product
runtime dependency, detected exploit, repository secret, or authorization-
boundary change.

### Resolver guard and recovery

The first
`npm update --package-lock-only --ignore-scripts browserslist baseline-browser-mapping caniuse-lite electron-to-chromium node-releases`
attempt selected unauthorized Browserslist 4.28.8, refreshed the four support
nodes beyond the required floors, and moved `update-browserslist-db` to 1.3.2.
The scope guard stopped and fully reversed that generated delta. A subsequent
`--no-save` exact-package attempt did not replace npm's broad lock selection and
was also discarded.

Temporary exact resolver inputs then constrained
`npm install --package-lock-only --ignore-scripts` to Browserslist 4.28.7 and
its four published minimums. The inputs were removed immediately. Final
`package.json` is byte-identical to baseline, and a scripts-disabled clean
install validates the normal parent ranges without an override. The final lock
diff contains exactly five existing top-level nodes, no addition/removal or
nested topology, and retains `update-browserslist-db@1.2.3`.

### Resolution and verification

The resolved versions are `browserslist@4.28.7`,
`baseline-browser-mapping@2.10.44`, `caniuse-lite@1.0.30001806`,
`electron-to-chromium@1.5.393`, and `node-releases@2.0.51`. Exact graph,
metadata, license, engine, integrity, and lifecycle checks pass. Both full and
production-only npm audits report zero vulnerabilities, complete
`npm run verify` passes, and PR #102 CI run `33694943603` passes dependency/
secret job `100461917273` plus every other classified job.

### Verify

Require a scripts-disabled clean install, exact `npm ls` output for all six
Browserslist-related nodes, full and production-only zero-finding audits, an
unchanged `package.json`, a five-node-only lock diff, complete repository
verification, a valid completion marker, and exact-head CI before merge.

## 2026-09-02 — Textarea Return did not submit and Graph wheel bypassed zoom

**Observation:** The conversation composer used only form submission from its
Send button, so Return in the textarea inserted a newline and required a mouse
click. The React Flow Graph explicitly disabled both `zoomOnScroll` and
`preventScrolling`, assigning ordinary wheel gestures to page scrolling even
when the pointer was over the canvas. A first keyboard correction also allowed
Alt/Control/Meta Return to send, exceeding the requested plain-Return contract.

**Disposition:** The composer now routes only exact unmodified, non-composing
Return through the existing guarded form action; Shift and every other modifier
retain native textarea behavior, and WebKit key code 229 fails safe. React Flow
now enables bounded wheel zoom and canvas-local scroll prevention while keeping
`panOnScroll` disabled and the existing manual viewport transition. Browser QA
confirmed Shift+Return insertion, Return send, up/in and down/out direction,
Graph-local capture, and outside-Graph page scroll. A freshly bundled native
Tauri app independently confirmed Return send and both wheel directions.
Focused 87-test and full 370-test frontend suites plus `npm run verify` pass.

## 2026-09-02 — Sidebar flex growth and Graph lane geometry misaligned tall displays

**Observation:** Owner screenshots at MacBook Pro and 5120x1440 ultrawide
classes showed Workspace navigation pushed down by a large blank sidebar band,
adjacent dashed domain containers touching or overlapping, and the orchestrator
relationship line crossing the domain-heading band. The sidebar's
`.conversation-navigation` used positive flex growth and consumed every spare
pixel. Normal lane padding exceeded the inter-column gap by four pixels, the
orchestrator bottom shared the first lane's top coordinate, and the wide layout
placed every domain in one shallow row. Review also found dense lane rows
overlapped by 24 world pixels and a fixed 280-pixel cutoff selected an unfit
layout at 281 pixels.

**Disposition:** Conversation history is content-bounded and independently
scrollable while the local-first footer alone uses `margin-top: auto`. Graph
geometry now provides at least eight world-space CSS pixels between every lane
pair and at least 32 below AgentOrchestrator, paints semantic nodes above group
lanes, balances ultrawide domains across two agent rows, and allows 150%
ultrawide automatic fit. Dense rows have positive vertical separation and are
selected by measured fit, eliminating the height cliff. Pairwise compact,
workspace, dense, and wide regression tests pass. Rendered browser checks at
760x520, 1280x720, 1678x1038, and the controller's 4096x1440 ultrawide limit
show fitting labels and no document overflow; exact 5120-pixel geometry is
covered by the adapter test. No DPR conversion, native path, or trust boundary
changed.

## 2026-09-02 — Constrained-height Graph and relationship controls collapsed or clipped

**Observation:** Prompt 6 reproduced the Graph with the inspector and activity
dock open at effective 125% and 150% sizes. At 1024x576 the React Flow parent
fell to approximately 579x12, and at 853x480 it reached 441x0 and emitted three
parent-dimension warnings. The Graph Filters popover extended beneath the
activity dock. A separate fixed 1280px breakpoint changed a 1279px workspace
fit at 100% to a 1280px wide-layout fit at 64%. The first keyboard relationship
selector draft also let the clipped Graph panel hide its final controls.

**Disposition:** At constrained heights, the activity-expanded Graph now uses a
760px natural page within the existing `.application-content` scroll owner;
the canvas measures 579x485 at 1024x576 and 441x485 at 853x480. Filters are
viewport/dock-bounded and internally scrollable. Wide layout now activates only
when measured topology bounds plus fit gutters support a full-readable fit, so
1279px and 1280px remain workspace mode at 100% while 2560x1440 uses wide mode
at 100%. The relationship legend is panel-bounded with its own scroll region;
the first and tenth relationship controls are fully reachable by pointer and
keyboard. Clean reload and constrained resize produced zero console warnings
or errors.

## 2026-09-02 — Integrated QA exposed lazy-panel and lifecycle-proof regressions

**Observation:** A cold lazy transition to Command Center could mark shell
panels as custom before page portals mounted, leaving an already-open inspector
unnamed and both panels blank. Separately, reusing a hoisted native-proof JSX
fragment preserved runtime behavior but failed the exact F-12 repository check,
which requires the sole lifecycle panel inside the literal selected-scenario
conditional.

**Disposition:** The Suspense fallback now portals truthful, busy, non-live
inspector and activity states and retains the inspector close action. The reused
native-proof fragment now contains the exact
`research-knowledge-active` conditional and the sole zero-prop lifecycle mount.
Focused loading/lifecycle tests and `npm run repository:check` pass.

## 2026-09-02 — Ambient Keychain behavior does not establish owned scope

**Observation:** File-based default/search-list behavior, access-group
terminology, and disabled feature paths do not prove one application-owned
identity scope.

**Disposition:** D-111 selects `scope_contract_not_accepted`; all remaining
blockers and Blocked readiness remain unchanged. No system operation ran.

## 2026-09-02 — Identity correspondence does not establish expected signer binding

**Observation:** D-107's native identity model can establish
certificate/private-key correspondence, but repository source has no immutable
expected Developer ID Application signer/certificate/public-key binding.
Fixed labels, fingerprints, filters, and ambient/default Keychain state are
selectors or metadata, not application-owned expected-signer provenance.

**Disposition:** D-110 selects `signer_binding_not_accepted`. The result is
limited to the reviewed repository state; it preserves D-097, D-107 8/11,
D-108 9/10, D-109, all ten blockers, and Blocked readiness. No certificate,
Keychain, private-key, signing, Apple/Xcode, build, target-Mac, provider,
product, or external operation ran.

## 2026-09-02 — Opaque object shape does not establish identity provenance

**Observation:** D-107 requires a separately proven application-owned,
no-input, attempt-bound opaque signing-identity reference. Current repository
source contains only a distinct fixed-label Cloudflare credential reader; it
does not issue, bind, or own a signing identity. No current component provides
the required issuer without lookup, enumeration, selection, fallback, or
ambient default/search-list/account/home/path authority.

**Disposition:** D-109 selects
`reference_issuance_not_accepted`. This is a bounded repository-state result,
not a universal impossibility claim. D-107's factual `contract_unproven` row,
D-108's 9/10 additive interpretation, all remaining blockers, and Blocked
readiness remain unchanged. No Keychain, certificate, private-key, signing,
Apple/Xcode, build, target-Mac, provider, product, or external operation ran.

## 2026-09-02 — D-102 lacked a closed disposition for an exact non-build proof class

**Observation:** D-107 froze an in-process, childless, fileless conceptual
challenge proof but correctly left `d102_applicability_split_contract`
unproved. D-102 governs build-child graphs and supplied no generic waiver or
`not_applicable` result. Treating scope reduction as containment would weaken
the policy, while applying build-child predicates to a class with no build or
child would conflate distinct subjects. The approved positive governance token
also exceeds D-100's 32-byte outcome limit.

**Disposition:** D-108 defines a documentation-only, exact-candidate split with
three closed governance dispositions and fail-closed reattachment triggers. A
definitive build, candidate-launched process, artifact, application- or Rust-
dependency-authored filesystem/network/socket/IPC API, application- or Rust-
dependency-selected dynamic/JIT/plugin/external-code load, product-signing, or
caller-selected feature makes D-102 mandatory. Ambiguity or drift records
`boundary_failed` and also makes D-102 mandatory. The long governance token is
not serialized as D-100 evidence. OS-managed downstream effects remain
independently unproved. Historical D-107 stays 8/11; only the additive current
interpretation is 9/10. Ten blockers remain, the candidate is not admitted,
and no successor is Ready.

## 2026-09-02 — D-107 scope wording conflated repository work with operational effects

**Observation:** Post-publication review found unqualified `process` and
`filesystem` absence language in four D-107 documents. Required-chain review
found one equivalent current summary in `PLANS.md`, bringing the complete
inventory to five. Read literally, that wording conflicts with the documented
facts that repository documentation was written and local validation and gate
processes ran.

**Disposition:** Preserve the published D-107 decision, plan, increment, review,
report digest, evidence totals, result, and Blocked readiness. Add one explicit
reconciliation and update only mutable current-state summaries: repository
documentation writes and local validation/gate processes occurred, while no
product/build/signing/Keychain/target-Mac operational process or state-changing
external action ran. This is a documentation-accuracy correction, not new
operational evidence or authority.

## 2026-09-02 — In-process challenge primitives do not establish the complete key-use boundary

**Observation:** Current Apple documentation and the pinned Rust crate establish
random-byte, opaque key-reference, data-signature, and verification primitives.
They do not jointly establish one D-101-compliant application credential-domain
identity, immutable Developer ID binding, prompt-free signing, hard
cancellation, terminal cleanup, platform-effect bounds, or a D-102
applicability split. The first drafted increment identifier was 67 characters;
the gate rejected it before state changed because identifiers are limited to 64.

**Disposition:** The semantically equivalent 56-character identifier
`personal-assistant-v0-key-use-containment-classification` was used without
changing the candidate or scope. D-107 records `not_eligible_or_unproven` with
eight `documented` and eleven `contract_unproven` rows. No operational action
ran. The result is a bounded negative source classification, not proof of
universal impossibility, and no successor is Ready. The first finalization
attempt also failed closed because the report used the unsupported combined
finding category `Security architecture`; changing only that schema value to
the accepted `Security` category preserved the finding and readiness result.
The next finalization attempt failed closed on a duplicate identical gate-status
entry in the machine-readable command list. Deduplicating that list preserved
both status observations in prose and changed no evidence or readiness result.
The following attempt failed closed because the report schema requires every
verification command identifier in the command inventory even when its status
is `Not run`. Adding those identifiers satisfied the schema without executing
the checks or changing their truthful `Not run` status.

## 2026-09-02 — Codeless signing fixture is distinct but contract-insufficient

**Observation:** The frozen Apple sources directly establish that a codeless
bundle has no executable code, can hold a signature, and stores a no-Mach-O
signature under `_CodeSignature` with hash-sealed resources. They do not
establish eight exact candidate-shape, provenance, Developer ID, private-key-
use, verification, identifier, no-build, and D-102 applicability predicates.
The draft also initially presented governance consequences as D-100 evidence;
those tokens were not valid factual evidence outcomes under D-100's semantics
and length bound.

**Disposition:** The D-100 record now uses exact canonical compact JSON with
factual outcome `contract_unproven`, while D-106 separately records the
governance result `not_eligible_or_unproven`. Five rows are `documented` and
eight are `contract_unproven`. No candidate or successor is admitted. This is a
bounded negative documentation result, not an operational failure or universal
impossibility claim. The only external contacts were approved read-only Git
remote synchronization/checks and reads of the three frozen first-party Apple
public-documentation pages. No fixture, source, build, or product/signing/
target-Mac operational process or state change occurred; all operational work
remains Blocked.

## 2026-09-02 — D-104 does not make the App Sandbox candidate eligible

**Observation:** Re-reviewing the exact additive v2 candidate against current
first-party Apple documentation removes only the former Developer ID
circularity classification. The sources establish narrow sandbox, entitlement,
helper, signature, direct-child, file-handle, process-event, and process-group
semantics. They do not establish D-102's complete bootstrap, effect-denial,
graph-ownership, shutdown, quiescence, cleanup, or platform-effect conjunction.

**Disposition:** D-105 records
`no_eligible_candidate_after_d104_rereview`. All 22 contracts remain
`contract_unproven`, and all ten P3-3 implementation-source checks remain
`not_run`. This is a valid negative documentation result, not a universal
impossibility claim or operational failure. No source, dependency, entitlement,
build, process, probe, target-Mac, Apple, signing, credential, provider,
product, or state-changing external action ran. P3-3 remains Blocked.

During closeout, the first `docs:check` and `repository:check` runs each found
the same three Apple method URLs parsed as local targets because of URL
parentheses. Correcting only those URL forms preserved the frozen sources and
claims; the required checks were then rerun.

## 2026-09-02 — P3 bootstrap signature must not be conflated with P4 identity

**Observation:** D-103's frozen App Sandbox helper review correctly rejected
entitlement-bearing signing as a P3-2 prerequisite. Official public Apple
documentation distinguishes an ad-hoc code seal, which has no signing identity,
from Developer ID identity binding. That distinction alone does not prove that
a sandbox helper, its bootstrap, or its descendants satisfy D-102.

**Disposition:** D-104 permits only a future static candidate re-review to
consider a narrowly defined identity-free sandbox-activation seal separately
from P4. D-102's pre-effect effect control, bootstrap provenance, descendant
membership, termination, reaping, quiescence, cleanup, and evidence predicates
remain mandatory and unproved. No signing, entitlement, build, probe,
target-Mac, Apple, Keychain, or external action has run; P3-3 remains Blocked.

## 2026-09-02 — Frozen P3-2 set has no eligible containment primitive

**Observation:** Apple documents App Sandbox as entitlement-configured and
documents sandboxed embedded helpers as entitlement-bound and signed. That
makes the only plausible frozen candidate fail P3-2's independent no-new-
entitlement/no-prerequisite-signing rule; it does not equate that signature
class with P4's later signer proof. Public direct-child waiting/reaping, known-
PID event observation, and mutable process-group contracts also do not establish
D-102's complete application-owned graph membership and terminal quiescence
after detachment or reparenting.

**Disposition:** D-103 selects no eligible candidate in the reviewed set.
Deprecated/private `sandbox-exec`, process groups, and post-hoc scans remain
negative controls. Privileged extensions and the reviewed VM route are scope-
ineligible because they add entitlement, signing, privilege, persistent-state,
or guest-resource boundaries; no exact qualifying container contract was
identified. This is a bounded documentation result, not a universal
impossibility claim or operational failure. P3-3 remains Blocked; no build,
probe, source, entitlement, target-Mac, authenticated Apple, signing, or state-
changing external action ran. Approved read-only public documentation access
was the sole external contact.

## 2026-09-01 — Build output routing is not build-child containment

**Observation:** The ordinary verification/build graph reaches npm lifecycle
scripts, Cargo build scripts, Vite/Tauri descendants, compilers, and linkers.
Routing `dist`, Cargo target, caches, or logs beneath a disposable root and
killing a process group does not prevent outside-root writes, undeclared
connections, reads of ambient host data, or a descendant that detaches with a
new session.

**Disposition:** The owner-approved P3 documentation increment records D-102's
future fail-closed policy only. No supported target-Mac no-new-dependency
primitive is selected. A later operational path must establish pre-effect
filesystem/network control, authoritative descendant membership across
reparenting/session escape, bounded quiescence, descriptor-bound cleanup, and
D-100-minimized evidence. Post-hoc scans, clean Git status, output routing,
process groups, and deprecated `sandbox-exec` alone cannot pass. No build,
probe, process, network, signing, or external operation ran.

## 2026-09-01 — Explicit account resolution is not acceptable future containment

**Observation:** The consumed wrapper needed only a Keychain scope but called
`pwd.getpwuid()`, potentially materializing a full account record and invoking
configured local or remote directory services plus OS cache/socket/log state.
Using only the home field did not contain that boundary, and rerunning the query
cannot repair the incomplete historical disclosure.

**Disposition:** Accepted D-101 defines a documentation-only application-
resolution prohibition plus independent authoritative input, exact scope-
provenance, and one-predicate effect gates. A future application-owned opaque
no-input capability cannot substitute for any gate. Application source can be
reviewed for prohibited calls, but that does not prove the absence of OS-
internal effects. Ambiguity stays blocked, owner acceptance is a separate
decision rather than fallback, the historical finding remains Pending, and no
query or external operation ran.

## 2026-09-01 — P1 evidence privacy requires source minimization

**Observation:** The historical D-097 privacy failure shows that a private
screenshot or raw capture has already crossed the intended boundary before it
can be summarized or redacted. Free-text observation and extensible report
fields have the same uncontrolled-content problem.

**Disposition:** D-100 defines `evidence_privacy_v1` as one fixed version, one
future-plan-owned check ID, and one closed outcome. Future evidence must be
minimized at its approved local source; screenshots, recordings, transcripts,
raw output, target-derived sensitive identifiers/metadata, private paths,
credentials, and content are prohibited. Static protocol literals are exact
bounded ASCII and carry no standalone authority; a future consumer must bind
one record privately to one plan/check/attempt and reject replay or late data.
Unknown or unexpected evidence stops without retry and records only
`boundary_failed`. This is documentation policy only: no sanitizer or operation
exists, D-097 remains Failed/Blocked, Open Directory remains Pending, and
signing remains Not run.

## 2026-09-01 — Exact D-098 documentation successor is completed

**Observation:** A clean, synchronized `main` at
`7fd4812fb02de7fee19e53a50b8f7bf96bd38709` had valid D-098 schema-v3 state.
The predecessor reconstructed to the immutable D-097 v2 digest, retained
`failed` / `FAIL` / `Blocked`, and had no completion marker. The disposition
admitted only `personal-assistant-v0-signing-security-prerequisite-planning`.

**Disposition:** Begin only that exact documentation increment. Preserve the
historical screenshot/privacy finding as Failed and the
`getpwuid`/`opendirectoryd` boundary as Manual verification pending. Do not
rerun the consumed query or perform any Apple, Keychain, signing, build,
credential, provider, network, product, or external-system operation. Future
privacy, directory, build-child-containment, and signer-contract work is
documented as separate Proposed/Blocked prerequisites, not resolved evidence.

## 2026-08-29 — Published Blocked failure cannot admit the exact documentation disposition

**Observation:** D-097 truthfully published the Xcode recovery as `failed` /
`FAIL` / `Blocked` with no completion marker. Its post-commit state cannot be
reclosed, and ordinary successor admission rejects the Blocked record. A fresh
clone is not an authorized bypass. The historical screenshot/privacy failure
and undisclosed `getpwuid`/`opendirectoryd` boundary must remain visible, while
the separate executable-build-script containment problem still blocks any
operational signing proof.

**Tracked-evidence disposition:** The owner authorized D-098 from synchronized
baseline `a417e5f1c1c602b917ca27c65af71480e3db6a45`. The recovery is limited to
one argument-free `record-failed-disposition` operation and one exact target,
`personal-assistant-v0-signing-security-prerequisite-planning`. It must preserve
the original status, quality, Blocked readiness, and absence of a completion
marker while carrying the two historical blockers only to that documentation
target. The tracked report freezes before the command and therefore records it
as Not run; only valid ignored schema-v3 state and redacted `status` output can
resolve the admission discrepancy after freeze. Do not infer that outcome from
this tracked entry, begin the successor, rerun the consumed query, or perform
Apple, Xcode, Keychain, build, signing, credential, external, or product work.

## 2026-08-29 — Required report heading check counted prose references

**Observation:** The first D-097 `validate_failed_report` run rejected the
otherwise exact report because the new required-section check counted a literal
`` `## Scope and boundaries` `` mention in Technical debt as a second heading.

**Resolution:** Count only complete Markdown heading lines, not substring
mentions. Add a regression proving prose may name the section while missing and
duplicate actual headings remain rejected. The final read-only failed-report
validation Passed.

## 2026-08-29 — Failed report stopped before truthful quality evaluation

**Observation:** Three machine-manifest findings used categories outside the
hook's closed allowlist, and the report omitted the template-required scope
section. The hook therefore rejected report structure before it could reach the
intended blocking-evidence result.

**Resolution:** Map the findings to `Technical debt`, `Security`, and `Code
health`, add `## Scope and boundaries`, and run the hook's validator read-only.
It reached exactly `post-increment report contains blocking evidence`. That
proved structural reconciliation only. D-097 later added a valid terminal
failed record; the quality remains `FAIL`, and no completion marker exists.

## 2026-08-29 — Failed increment has no terminal gate-state representation

**Observation:** The active recovery increment has an immutable Failed privacy
check. The gate schema accepts only `active` or a passing `complete`, and
finalization rejects every report whose computed quality is `FAIL`.

**Resolution:** The owner authorized D-097 inside the same active gate. Add an
exact state-schema-v2 `failed` variant and `close-failed` command that validates
computed `FAIL`, stores bounded report/workspace/HEAD/readiness evidence, writes
no completion marker, forbids failed-to-complete promotion, and makes ordinary
unreclosed drift fail closed. The local record is not authentication or durable
audit. The current report remains `FAIL`/Blocked, so no successor or operational
signing proof is authorized. No second gate began.

## 2026-08-29 — Scoped identity wrapper omitted Open Directory disclosure

**Observation:** The consumed wrapper used `pwd.getpwuid()` to resolve the
account home. On macOS, that lookup may be served through `opendirectoryd`,
transiently return a full account record, consult configured local or remote
directory systems, and use OS cache/socket/log state. The wrapper used only the
home field and emitted no account value; no evidence proves remote traffic.

**Disposition:** Record the boundary additively as Manual verification pending.
Do not repeat the consumed query. Future wrappers must disclose or contain
account-directory resolution and test resolver failure/drift before execution.

## 2026-08-29 — Developer ID “never exported” evidence is not locally provable

**Observation:** The active recovery records treated a non-exported,
owner-controlled private key as a pending yes/no proof. Apple documents that
some Keychain certificates and keys can be exported. Current pairing, a valid-
identity query, and even a successful signature establish present visibility or
use, not historical absence of export, exclusive custody, or absence of a prior
copy. The separate current-item extractability attribute was not queried and
remains `not_proven`.

**Disposition:** Preserve the historical gate and privacy failure, but do not
mark that criterion Passed as written. Accepted D-096 and the
[present-use/local-signing plan](docs/plans/2026-08-29-v0-developer-id-present-use-local-signing-proof.md)
define the additive prospective evidence standard. It permits only bounded
owner attestation, workflow-private-key-no-export, present-use evidence, and
explicit `not_proven` categories; signing remains Blocked pending every recorded
gate, an exact sanitizer, and separate one-attempt approval. Never attempt an
export to test the claim.

## 2026-08-28 — V0-2 duplicate artifacts appeared outside the approved diff

**Symptom:** Four untracked files with a ` 2` suffix appeared during V0-2: one
Rust duplicate plus three planning/review duplicates. Their presence made the
active workspace ambiguous and prevented completion.

**Resolution:** With explicit owner authorization, record each SHA-256 and move
only those four untracked duplicates to
`/private/tmp/cortexa-v0-2-duplicates-2026-08-28-01a04094`. Destination hashes
matched exactly. No tracked file was overwritten, deleted, reset, cleaned, or
stashed; the repository returned to the exact approved four-file source/plan
diff before work resumed.

## 2026-08-28 — V0-2 npm audit required approved network retry

**Symptom:** The required sandboxed `npm audit --audit-level=low` failed with
`ENOTFOUND registry.npmjs.org` and could not write its ordinary user npm log.

**Resolution:** Repeat the exact command with approved network access. It
passed with `found 0 vulnerabilities`; no dependency or lockfile changed.

## 2026-08-28 — V0-2 readiness wording lagged V0-1 publication

**Resolution:** Reconciled current-state records against the observed PR #79
squash commit `dca584e`; no executable behavior changed.

## 2026-08-28 — Raw debug executable unavailable to approved UI binding

**Observation:** `npm run tauri -- dev` compiled and launched the source-current
debug executable, but approved Computer Use bundle binding repeatedly selected
a stale bundled release `.app` instead of the raw debug process. A process check
confirmed the development processes were stopped after inspection.

**Disposition:** Do not use the stale release UI as evidence for current source.
The source-current Vite browser fallback passed disclosure, selected-scenario
mount/unmount, closed unavailable state without a Tauri bridge, structured
table, focus, scroll, topology zoom/reset, dark theme, reduced-motion query,
viewport overflow, and console checks. Native success/failure/cancellation,
alternate native theme/reduced motion, page zoom, and native resize are `Not
run` advisories; browser fallback is not a substitute for those checks.

## 2026-08-28 — First connected-source verify stopped on increment formatting

**Symptom:** The first `npm run verify` stopped at Prettier because the newly
created connected-presentation increment record was not formatted.

**Resolution:** Format only the declared documentation file, inspect its diff,
and rerun the unchanged complete command. Subsequent complete verification
passes. No product source, lint rule, test, build setting, or security control
was changed to obtain the pass.

## 2026-08-28 — Sandboxed npm audit could not resolve the registry

**Symptom:** The sandboxed `npm audit --audit-level=low` attempt failed with
`ENOTFOUND` and could not write its ordinary user-level npm log.

**Resolution:** Repeat the same audit with approved network access. It passed
with `found 0 vulnerabilities`; no dependency or lockfile changed.

## 2026-08-28 — Connected lifecycle failure is not current production behavior

**Observation:** `ResearchKnowledgeDemoHost::new()` constructs only the success
script. The deterministic synthesis-failure path exists only through a private
test-core constructor, while the Tauri adapter exposes no argument through
which a WebView could select a script.

**Disposition at that planning checkpoint:** The connected-presentation plan
proposed a private application-owned alternating completed-epoch schedule. The
owner subsequently approved and locally verified that bounded source increment;
the current implementation still adds no failure command, outcome parameter,
test-event control, timer, or retry.

## 2026-08-28 — Lifecycle host cannot enter Tauri managed state

**Symptom:** Focused compilation of the approved lifecycle Tauri adapter failed
before tests ran because `Mutex<ResearchKnowledgeDemoHost>` did not satisfy
Tauri's `Send + Sync + 'static` managed-state bound.

**Cause:** The host transitively owns the governance `InMemoryApprovalManager`,
whose private `Box<dyn ApprovalClock>` is not `Send`. The production clock is
Send-safe, but the private trait does not require `Send`, and its deterministic
test clock uses `Rc<Cell<Instant>>`.

**Resolution:** The owner approved the exact private prerequisite.
`ApprovalClock` now requires `Send`; its deterministic test clock uses
`Arc<Mutex<Instant>>`; compile-time assertions prove the approval manager and
lifecycle host satisfy the required bounds. Approval-manager tests pass 7/7 and
lifecycle-core tests pass 9/9 without a public approval or behavior change. No
unsafe wrapper, thread-local host, worker, queue, or duplicate host was added.

## 2026-08-26 — Transient Cargo incremental-cache write during F-07 verification

**Symptom:** One `npm run verify` attempt reached strict Clippy and failed to
create two `dep-graph.part.bin` files because Cargo's generated incremental
working directories were absent.

**Cause:** No persistent source, toolchain, capacity, or permission defect was
reproduced. The exact Clippy command passed unchanged immediately afterward,
indicating a transient generated incremental-cache working-directory race.

**Resolution and verification:** No source, lockfile, toolchain, security
setting, or cache deletion was used. The exact strict Clippy command passed,
then a fresh complete `npm run verify` passed. If this recurs, inspect competing
Cargo processes and the generated incremental directory before considering any
bounded cache cleanup; do not weaken or skip strict Clippy.

## 2026-08-26 — F-01/F-02 runtime-start containment

**Observation:** Legacy runtime starts accepted adapter-returned identity without
comparing it to the application request and could drop a rejected nonterminal
run when cancellation failed.

**Resolution:** Generalize the existing D-091 exact-identity and quarantine path
to every runtime start. New contracts cover foreign identities, blocked
fallback, one-shot and permanent cancellation failure, contradictory
nonterminal dispositions, and explicit cleanup retry. No environment failure or
external runtime was involved.

## 2026-08-26 — F-15 documentation reconciliation

**Observation:** The native nine-agent architecture review identified duplicate
`FR-020`, stale Command Center rendered-matrix wording, and architecture claims
that exceeded current app-info/CSP source behavior.

**Resolution:** Correct the factual documentation and enforce the affected
markers with static repository-health tests. No runtime issue or target-Mac
failure was involved.

## TS-001 — npm EBADENGINE on Node.js 26

Date: 2026-06-18
Status: Resolved

### Symptom

```text
npm error code EBADENGINE
Required: {"node":">=22.12.0 <23","npm":">=10 <11"}
Actual:   {"node":"v26.3.0","npm":"11.16.0"}
```

### Cause

The initial repository engine declaration accepted only Node.js 22 and npm 10 while `.npmrc` enabled `engine-strict=true`.

### Resolution

The repository now declares:

```json
{
  "node": "^22.12.0 || ^24.0.0 || >=26.0.0 <27",
  "npm": ">=10 <12"
}
```

The preferred versions are Node.js 26.3.0 and npm 11.16.0.

### Verify

```bash
node --version
npm --version
rm -rf node_modules
npm ci
```

Expected: installation completes without `EBADENGINE`.

## TS-002 — Tauri cannot run cargo metadata

Date: 2026-06-18
Status: Resolved

### Symptom

```text
failed to run 'cargo metadata' command
failed to run command cargo metadata --no-deps --format-version 1:
No such file or directory (os error 2)
```

### Cause

`cargo` was not available on the interactive shell's `PATH`.

### Resolution

Install or activate Rustup, then verify Cargo before starting Tauri. For the Homebrew `rustup` package on Apple Silicon:

```bash
export PATH="$(brew --prefix rustup)/bin:$PATH"
rehash
rustup default 1.90.0
which cargo
cargo --version
rustc --version
```

Persist the path in Zsh:

```bash
grep -qxF 'export PATH="$(brew --prefix rustup)/bin:$PATH"' "$HOME/.zshrc" ||   printf '
export PATH="$(brew --prefix rustup)/bin:$PATH"
' >> "$HOME/.zshrc"
```

Open a new shell or run:

```bash
exec zsh
```

### Verify

```bash
which cargo
cargo --version
cd /path/to/ai-agent-assistant
npm run tauri -- dev
```

Expected: the native application compiles and launches.

## TS-003 — rustup reports an installed toolchain but cargo is not found

Date: 2026-06-18
Status: Resolved

### Symptom

Rustup reports that `1.90.0-aarch64-apple-darwin` is installed, followed by:

```text
cargo not found
zsh: command not found: cargo
zsh: command not found: rustc
```

### Cause

Homebrew installed Rustup outside the shell's active `PATH`. The presence of a toolchain does not make the shims discoverable unless the Rustup `bin` directory is available.

### Resolution

```bash
export PATH="$(brew --prefix rustup)/bin:$PATH"
rehash
rustup default 1.90.0
```

If Rustup was installed with the official installer instead, use:

```bash
source "$HOME/.cargo/env"
```

Do not add both approaches blindly; use the path that contains the actual `rustup`, `cargo`, and `rustc` executables.

## TS-004 — Zsh treats pasted comment lines as commands

Date: 2026-06-18
Status: Resolved

### Symptom

```text
zsh: command not found: #
```

### Cause

The interactive shell did not have Zsh's `interactivecomments` option enabled.

### Resolution

Either paste commands without comment lines or enable comments:

```bash
setopt interactivecomments
grep -qxF 'setopt interactivecomments' "$HOME/.zshrc" ||   printf '
setopt interactivecomments
' >> "$HOME/.zshrc"
```

## TS-005 — Cargo unavailable in artifact-generation environment

Date: 2026-07-09
Status: Open for artifact host; expected to be resolved on target Mac

### Symptom

The required Increment 2A Rust verification commands failed in the artifact-generation environment with:

```text
bash: line 1: cargo: command not found
```

Affected commands:

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
```

### Cause

The artifact-generation host had Node.js and npm available but did not have Cargo, Rustup, or Rust installed on `PATH`.

### Resolution

Run the Rust checks on the target Mac where Rust 1.90.0 is available, or install/activate Rustup before verification:

```bash
export PATH="$(brew --prefix rustup)/bin:$PATH"
rustup default 1.90.0
which cargo
cargo --version
```

### Verify

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
```

Expected: all three commands pass before Increment 2A is marked complete.

## TS-006 — tsc command not found after applying source ZIP

Date: 2026-07-13
Status: Resolved

### Symptom

```text
> ai-agent-assistant@0.1.0 typecheck
> tsc -b --pretty false

sh: tsc: command not found
```

### Cause

The repository's locked npm dependencies had not been installed in the local checkout after applying the source ZIP. `tsc` is provided by the local `typescript` dev dependency under `node_modules/.bin`.

### Resolution

From the repository root, run:

```bash
npm ci
```

Then rerun:

```bash
npm run typecheck
npm run build
```

Expected: `tsc` is found through npm's local package-bin path and both commands pass.

## TS-007 — cargo fmt --check prints diffs after applying Increment 2A ZIP

Date: 2026-07-13
Status: Resolved

### Symptom

`cargo fmt --check` prints diffs in the new Increment 2A Rust modules, including files under:

```text
src-tauri/src/audit/logger.rs
src-tauri/src/memory/store.rs
src-tauri/src/platform/adapter.rs
src-tauri/src/policy/engine.rs
```

### Cause

The Increment 2A source compiled and tested after formatting, but the applied ZIP contained Rust files that needed standard `rustfmt` formatting on the target Mac.

### Resolution

Run:

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
```

Expected: the first command applies formatting and the second command passes without diff output.

## TS-008 — Full Increment 2B-1 native verification unavailable on artifact host

Date: 2026-07-13
Status: Resolved on target Mac; artifact-host limitation remains

### Symptom

The storage-only Rust crate compiles, passes Clippy, and passes its focused tests, but the artifact-generation host cannot complete the full Tauri crate's native dependency build or produce target-Mac verification evidence.

### Cause

The artifact host is not the Apple Silicon macOS target and does not provide the complete native desktop dependency environment required by the Tauri crate. The bundled SQLCipher/OpenSSL feature must also be confirmed with the repository's pinned Rust toolchain on the target Mac.

### Safe resolution

Apply the Increment 2B-1 source overlay on the target Mac, then run one unlocked check to resolve the new exact dependencies and update the lockfile:

```bash
export PATH="$(brew --prefix rustup)/bin:$PATH"
rustup default 1.90.0
cargo check --manifest-path src-tauri/Cargo.toml
```

Review the lockfile:

```bash
git diff -- src-tauri/Cargo.lock
```

Then run the required locked verification:

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
npm run typecheck
npm run build
npm run tauri -- dev
```

### Prevention

- Commit the target-Mac-generated lockfile with the increment.
- Keep native dependency changes in isolated increments.
- Do not mark the increment complete based only on the storage harness.
- Do not remove SQLCipher features to make an unrelated host pass; record a superseding decision if the target Mac exposes a real blocker.

## TS-009 — libsqlite3-sys 0.38.1 fails on pinned Rust 1.90

Date: 2026-07-13
Status: Resolved

### Symptom

The first Increment 2B-1 dependency selection failed while compiling `libsqlite3-sys 0.38.1`:

```text
error[E0658]: use of unstable library feature `cfg_select`
```

The same checkout could also report missing `agent`, `approvals`, `audit`, `memory`, `platform`, `policy`, and `tools` modules when an overlay was applied to a public baseline that did not contain Increment 2A.

### Cause

- `rusqlite 0.40.1` resolved to a `libsqlite3-sys` build script requiring a newer Rust standard-library feature than the repository's pinned Rust 1.90.0 provides.
- The original storage overlay assumed the verified Increment 2A module tree already existed locally.

### Resolution

- Restore the complete Increment 2A module tree.
- Pin `rusqlite` to 0.37.0 with the same bundled SQLCipher and vendored OpenSSL feature.
- Regenerate the lockfile with Cargo.

Verified dependency tree:

```text
rusqlite v0.37.0
libsqlite3-sys v0.35.0
```

### Verify

```bash
cargo tree --manifest-path src-tauri/Cargo.toml | grep -E 'rusqlite|libsqlite3-sys'
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
npm run typecheck
npm run build
npm run tauri -- dev
```

The project owner confirmed all checks and the native launch passed on the target Mac.

## Quick diagnostic snapshot

Run this before troubleshooting an install or build failure:

```bash
pwd
uname -a
uname -m
sw_vers
which node
node --version
which npm
npm --version
which rustup
rustup show active-toolchain
which cargo
cargo --version
which rustc
rustc --version
xcode-select -p
git status --short --branch
```

Redact usernames, access tokens, private paths, and personal data before sharing logs.

## New entry template

Copy `docs/templates/TROUBLESHOOTING_ENTRY_TEMPLATE.md` and append the completed entry below this section. Include the exact symptom, environment, root cause, smallest fix, verification command, and any prevention step.

## TS-010 — Public repository lags the verified local checkout

Date: 2026-07-13
Status: Resolved 2026-07-15

### Symptom

The public GitHub page still reports one commit and describes the original runnable shell, while the project owner's local checkout contains verified Increments 2A through 2D.

### Cause

The verified local changes have not all been pushed to the public branch, or the public page has not caught up with the local working state.

### Safe handling

- Treat the project owner's verified local checkout and current project-memory files as the implementation baseline.
- Apply overlays only to `/Users/hdang/Desktop/Projects/ai-agent-assistant` after confirming `git status`.
- Do not reconstruct a later increment solely from the public branch.
- Commit and push verified checkpoints before relying on GitHub as the source of truth.

### Verify

```bash
git status --short --branch
git log -5 --oneline --decorate
git remote -v
```

Do not publish secrets, local databases, credentials, certificates, or environment files when synchronizing the public repository.

### Resolution

The verified product increments through 4U and Meta Increment 1 are now
published and merged. At the Meta Increment 2 baseline, `main`, `origin/main`,
and the working branch base all resolve to `5edbf4d`, and the working tree began
clean. Continue to verify synchronization at session start; do not assume this
historical condition remains resolved after future local work.

## TS-011 — Increment 2D actions are not visible in the standard macOS application menu

Date: 2026-07-13
Status: Resolved

### Symptom

The application launches and the left-side **Cortexa** application menu contains standard macOS items such as About, Services, Hide, and Quit, but does not show:

```text
Open Cortexa
New Request
Tasks (Coming Soon)
Quit Cortexa
```

The source still shows that the New Request and Tasks menu items are constructed.

### Cause

The standard application-name menu on the left side of the macOS menu bar is separate from the custom Tauri status-item menu. Increment 2D installs the custom menu under the Cortexa status icon on the right side of the menu bar, near system status items.

### Resolution

Click the Cortexa status icon on the right side of the macOS menu bar. Its menu contains the four fixed Increment 2D actions.

### Verify

1. Hide the main window with its red close control.
2. Open the right-side Cortexa status-item menu.
3. Select **New Request** and confirm the window returns and receives focus.
4. Hide the window again.
5. Select **Tasks (Coming Soon)** and confirm the window returns and receives focus.

The project owner confirmed both actions worked without an error.

### Prevention

Manual test instructions should consistently use the term **menu-bar status item** and distinguish it from the standard macOS application menu.

## TS-012 — React component tests retain prior rendered shells

Date: 2026-07-13
Status: Resolved

### Symptom

When multiple Increment 2E component tests run in one Vitest process, role and text queries can find elements from an earlier render, producing ambiguous-match failures even though each test passes by itself.

### Cause

The test environment was not explicitly cleaning React Testing Library's rendered DOM after every test. Depending on test-runner integration alone made cleanup behavior implicit.

### Resolution

Add an explicit test-only cleanup hook in `src/test/setup.ts`:

```ts
import { cleanup } from "@testing-library/react";
import { afterEach } from "vitest";

afterEach(() => {
  cleanup();
});
```

### Verify

```bash
npx vitest run
```

Expected:

```text
3 test files passed
30 tests passed
0 failed
```

### Prevention

Keep global DOM-test cleanup explicit in the shared Vitest setup and avoid relying on test order or prior component unmount behavior.

## TS-013 - Tauri development launch reports port 1420 already in use

Date: 2026-07-14
Status: Resolved

### Symptom

`npm run tauri -- dev` exits before launching the native application because Vite reports that port 1420 is already in use.

### Cause

A stale standalone `npm run dev` process for this repository still owns the fixed Vite listener. Starting Tauri launches a second `beforeDevCommand`, which cannot bind the same port.

### Resolution

Identify the listener and its parent before stopping anything:

```bash
lsof -nP -iTCP:1420 -sTCP:LISTEN
ps -p <listener-pid> -o pid=,ppid=,lstart=,command=
ps -p <parent-pid> -o pid=,ppid=,lstart=,command=
```

If the output proves the listener is the stale Vite child of this repository's standalone `npm run dev`, stop that parent process. Confirm the port is free, then rerun the exact Tauri command.

### Verify

```bash
lsof -nP -iTCP:1420 -sTCP:LISTEN
npm run tauri -- dev
```

Expected: the first command has no listener before launch; Vite starts on port 1420, Cargo launches the unchanged `target/debug/ai-agent-assistant` executable, and the native application opens.

### Prevention

Stop standalone Vite sessions when their work ends. Before a native manual gate, identify an existing fixed-port listener instead of starting overlapping dev servers or terminating an unrelated process.

## TS-014 - Completion marker becomes invalid after committing tracked deletions

Date: 2026-07-15
Status: Resolved by Repository Workflow Increment 4J; affected pre-fix increments require reconstruction

### Symptom

A post-increment marker is valid immediately after finalization, but `python3 .codex/hooks/post_increment_gate.py status` reports `valid: false` after committing a change set that deletes tracked files. Re-finalization on the clean committed branch fails with `report file inventory does not match the complete Git change set`.

### Cause

Before commit, `git ls-files --cached` still includes each deleted tracked path. The old fingerprint hashed that path plus a `missing` token. After commit, Git no longer lists the deleted path, so it contributes nothing and the fingerprint changes even though the reviewed working-tree content did not.

The original post-commit regression created or modified files only and did not exercise a tracked deletion.

### Resolution

Fingerprint only repository paths that exist in the current working-tree snapshot. Keep exact deletion evidence in the independent `changed_paths` and report-inventory validation. Add regressions proving both reviewed deletion-commit stability and invalidation when a tracked file is deleted after finalization.

Do not migrate or reinterpret pre-fix markers. Preserve and reconstruct an affected increment on corrected `main`, rerun its required verification and gate, and confirm its marker remains valid after commit before publication.

### Verify

```bash
python3 .codex/hooks/tests/test_post_increment_gate.py -v
npm run test:hooks
python3 .codex/hooks/post_increment_gate.py status
```

Expected: all 17 focused tests pass. The reviewed deletion fixture remains valid after commit, while the post-finalization deletion fixture requests continuation.

### Prevention

Any future fingerprint or changed-file implementation must test additions, modifications, deletions, pre-commit state, post-commit state, and an unreviewed change after finalization. Keep report inventory and workspace-content fingerprint responsibilities distinct.

## TS-015 - Independently merged dependency updates break clean verification

Date: 2026-07-16
Status: Resolved in the verified dependency baseline repair; publication pending

### Symptom

Pull-request checks fail before product tests. Hosted `npm ci` reports a Vite
peer conflict, local `npm ci` reports invalid `package.json`, and Rust Clippy
fails in `libsqlite3-sys` with unstable `cfg_select` on Rust 1.90.

### Cause

Several Dependabot pull requests were based on overlapping older dependency
states and merged independently. The resulting `main`:

- removed the direct `vitest@3.2.6` manifest entry and left a trailing comma;
- retained duplicate direct `typescript-eslint` and `vite` lockfile keys;
- selected `vite@8.1.4` outside `@vitejs/plugin-react@4.7.0`'s peer range; and
- selected `rusqlite@0.40.1`, whose `libsqlite3-sys@0.38.1` build script does
  not compile on the repository's supported Rust toolchain.

An initial lock regeneration also retained nested Vite `7.3.6` instances under
Vitest, causing duplicate TypeScript plugin type identities against root Vite
`7.3.5`.

### Resolution

Restore the previously verified exact direct versions `vite@7.3.5`,
`vitest@3.2.6`, and `rusqlite@0.37.0`. Regenerate both lockfiles, run `npm
dedupe`, and preserve all unrelated compatible updates already on `main`.

### Verify

```bash
npm ci
npm ls vite @vitejs/plugin-react vitest
npm run typecheck
cargo tree --manifest-path src-tauri/Cargo.toml -i rusqlite --locked
npm run verify
npm audit --audit-level=low
```

Expected: npm reports one deduplicated `vite@7.3.5`, Cargo reports exact
`rusqlite@0.37.0`, and all repository checks pass.

### Prevention

Do not merge overlapping dependency proposals solely because their original
branch checks passed. Refresh each proposal against current `main`, require a
clean install and full check on the merge candidate, and group coupled major
updates such as Vite/plugin/Vitest or rusqlite/toolchain changes into one
reviewed compatibility increment.

## TS-016 - GitHub-hosted jobs fail before runner assignment

Date: 2026-07-17
Status: Resolved

### Symptom

CI, Documentation, and Security checks complete as failures within seconds and
show no checked-out source or command output. GitHub annotates each job that it
was not started because account payments failed or the spending limit must be
increased.

### Cause

The failure occurs before a runner is assigned and is not a repository test
failure. All three workflows selected GitHub-hosted macOS or Ubuntu images. The
registered repository runner was online but could not match those `runs-on`
labels.

### Resolution

Register a dedicated Linux x64 runner, assign it the custom `cortexa-ci` label,
and route the three read-only workflows through the exact four-label selector.
Do not use `pull_request` on the persistent runner; restrict pushes to the
documented maintainer-controlled branch families. Provision Tauri Linux
prerequisites on the host rather than installing system packages with workflow
`sudo`.

### Verify

```bash
gh api repos/SillyRbbit/ai-agent-assistant/actions/runners
npm run test:repository
npm run docs:check
npm run repository:check
npm run verify
```

After publishing the workflow branch, confirm all three GitHub jobs name the
intended runner and pass. Until that remote execution succeeds, this resolution
remains verification pending.

### First self-hosted result

PR #24 commit `80bced4` proved exact routing: Documentation passed on runner 21.
CI run `29624042629` and Security run `29624042656` reached the same runner but
failed at the prerequisite step after finding `git` and `python3` because
`command -v rustup` returned exit code 1. Install or activate Rustup for the
runner service account, refresh the runner-captured path so
`$HOME/.cargo/bin` is visible, restart the service, and rerun the two failed
jobs. This is a host prerequisite failure, not a repository test failure.

The first repair installed Rustup `1.29.0`, Cargo, and default toolchain 1.90.0
under `/home/henry-dang/.cargo/bin`. `svc.sh stop/start` then failed because the
runner had never been installed as a `systemd` service; GitHub still reported it
online through the earlier interactive listener. Stop that listener, run
`./env.sh` after sourcing `$HOME/.cargo/env`, install the service for
`henry-dang`, and start it before rerunning the failed jobs.

CI and Security attempt 2 still failed at `command -v rustup` after the service
setup was reported complete. Before another rerun, inspect `.service`,
`svc.sh status`, `.path`, and all `Runner.Listener` or `runsvc.sh` processes.
This distinguishes a captured-PATH defect from an old interactive listener that
is still receiving jobs.

The inspection found both stale interactive PID `7699` and managed service
listener PID `36245`. The service was active and its `.path` correctly began
with `/home/henry-dang/.cargo/bin`, while its journal repeatedly reported that a
session for the runner already existed. Stop only the stale interactive
listener, restart the service, and require one remaining listener with
`--startuptype service` before rerunning workflows.

Stopping the stale listener and restarting the service fixed runner routing and
PATH inheritance. Attempt 3 passed all host prerequisites. Documentation and
Security pass. CI then exposed a distinct repository portability issue: strict
Linux Clippy rejects five private approval-source support items because their
only consumer is the macOS-gated decision-source module. Do not suppress or
weaken Clippy; handle that source correction only through separately approved
scope.

The project owner approved the exact two-file correction. Target-gating only
the private import, presentation marker/parts and conversion, and native
evidence constructors removes their non-macOS compile presence while preserving
the complete macOS path. Focused approval-manager tests, strict Clippy, and
`npm run verify` pass locally. At that checkpoint the correction remained
uncommitted, so PR #24 still needed a successful Linux CI rerun.

Commit `1621a55` published the correction. Security run `29629669283`,
Documentation run `29629669305`, and CI run `29629669300` all passed on runner 21. The corrected CI completed full Linux verification in 9 minutes 57 seconds,
resolving the runner-host and strict-Clippy portability incident.

### Prevention

Keep the runner-specific label, no-pull-request rule, and push allowlist covered
by repository-health tests. Reapply the label after runner replacement, keep
the service account unprivileged and credential-free, and preserve separate
target-Mac verification for native behavior.

### PR #57 recurrence: private macOS-only items fail strict Linux Clippy

Date: 2026-08-25
Status: Resolved

PR #57 run `32917746165`, Linux job `98027487903`, reached the configured
self-hosted runner after its missing `cortexa-ci` label was restored. Strict
all-target Clippy then reported one test import plus private approval and
Cloudflare credential helpers whose consumers exist only on macOS or in tests.
The target-Mac job passed, confirming this was a cross-target compile-scope
failure rather than a target-Mac behavior failure.

The project owner approved a separate bounded remediation instead of reopening
the completed D-093 gate. The correction target-gates only the private test
import and approval matcher and retains the private Cloudflare seam under
`cfg(test)` or macOS. It does not suppress Clippy, gate the public credential
API, or change approval, Keychain, credential, error, dependency, permission,
or execution behavior. Local focused tests, strict Clippy, all-target Rust, and
complete repository verification pass. Published correction
`6b2675343db8518587068e7175ce0cec9d2f6107` then passed CI run `32921400121`:
Linux Rust job `98035560462` completed strict Clippy and all-target tests in
6m55s, target-Mac job `98035560489` passed in 2m18s, and frontend job
`98035560481` passed in 57s. Documentation run `32921400102`, job
`98035529472`, passed in 26s. The remaining failed dependency job is a separate
approved remediation and does not reopen this resolved portability recurrence.

## TS-017 - Certificate Assistant cannot create the Developer ID CSR

Date: 2026-07-31
Status: Unresolved; operation stopped safely

### Symptom

On the macOS 26.6 arm64 target Mac, Keychain Access Certificate Assistant
reported `The specified item could not be found in the keychain.` while the
owner attempted to save the separately approved Developer ID Application CSR.
No CSR file became available.

### Observed evidence

- The user keychain list and default-keychain read-only checks identified the
  login keychain, but keychain-info checks returned parameter-related errors.
- A read-only code-signing identity query found zero valid identities, which was
  expected before certificate creation and does not explain the CSR failure.
- A generic `<key>` row was visible before the failed attempt. It is not
  evidence of a key created by this attempt.
- The owner confirmed: no CSR file created, no certificate created, and no new
  named private key observed.

### Cause

Not determined. The observed error and read-only diagnostics do not prove
Keychain corruption, a missing keychain item, an access-control defect, or any
other root cause. No such cause should be inferred without a separately
approved diagnostic plan and reproducible evidence.

### Safe disposition

Stop the operational increment as `unavailable`. Do not retry CSR creation,
reset, unlock, replace, or delete Keychain state, generate a private key through
Terminal or OpenSSL, create a different certificate type, contact Apple support,
or continue to certificate creation under this increment. No rollback action is
needed because the owner observed no CSR file, certificate, or new named private
key.

### Verify

Use only the owner's sanitized confirmation that no CSR file, certificate, or
new named private key was created. Repository closure must pass documentation,
repository-policy, secret-scan, whitespace, product-path, session-end, and
post-increment checks without recording account, certificate, key, or Keychain
identifiers.

### Prevention

Before any future attempt, approve a documentation-only remediation plan that
defines the exact read-only Keychain diagnostics, expected results, privacy
limits, stop conditions, and recovery/rollback decision points. Continue to
prohibit command-line private-key file generation and any unplanned Apple,
signing, Keychain, credential, Cloudflare, provider, deployment, traffic, or
runtime action.

The documentation-only
[`macos-certificate-assistant-csr-remediation-plan.md`](docs/plans/macos-certificate-assistant-csr-remediation-plan.md)
now defines that future diagnostic boundary. It does not approve execution;
separate explicit owner approval remains required before any observation runs.

### Approved read-only diagnostic outcome

Date: 2026-08-01

The owner performed each of the plan's three local read-only observations once
and reported only the approved sanitized categories:

- user Keychain configuration: `observed`;
- default Keychain configuration: `observed`;
- valid code-signing identities: `zero`;
- authorization prompt: `not observed`; and
- state changed: `not observed`.

These observations show that the configured user/default Keychain state was
readable without an authorization prompt and that no valid code-signing identity
was present. They do not reproduce the original Certificate Assistant failure
or distinguish among possible causes. The cause remains `not determined`; no
resolution has been performed, and no retry or remediation is authorized.

### Owner decision

Date: 2026-08-01

D-076 records the owner's decision to defer the signed macOS identity path.
Apple Support assistance and an alternate CSR workflow were considered but are
not authorized. The outcome remains unresolved and does not justify a Keychain
repair, CSR retry, signing action, or alternate key-generation path.

The documentation-only
[`apple-support-ts-017-assistance-plan.md`](docs/plans/apple-support-ts-017-assistance-plan.md)
defines a possible future owner-only support contact. It does not authorize that
contact or any response action.

### Stopped contact and filesystem signing-material outcome

Date: 2026-08-02

The owner did not contact Apple Support or access Apple Developer. One CSR file
and one filesystem private-key file were created outside the approved contact
scope. Neither was uploaded, used, copied, exported, or backed up; no
certificate exists. Encryption and permissions were not inspected and remain
undetermined. The material does not satisfy D-072's non-exported Keychain
boundary. No disposition action is authorized.

The documentation-only
[`filesystem-signing-material-disposition-plan.md`](docs/plans/filesystem-signing-material-disposition-plan.md)
selects future abandonment and paired deletion. Execution remains unauthorized.

### Paired filesystem signing-material disposition

Date: 2026-08-02

Under a separate owner-operated approval, the owner privately identified exactly
the unuploaded CSR and its filesystem private-key file, observed no additional
signing material, deleted both as one paired disposition, and verified their
absence. Sanitized evidence reports no remaining copy, upload, use, or
certificate. No filename, path, content, key material, account detail, or other
private evidence entered the repository.

This outcome closes custody of the known filesystem pair only. It does not prove
cryptographic erasure from APFS/SSD remnants or snapshots, establish TS-017's
cause, satisfy D-072, or supersede D-076. No recovery, regeneration, signing, or
credential action is authorized.

### Conditional Apple Support-contact consideration

Date: 2026-08-02

D-077 records the owner's choice to conditionally reopen consideration of one
future owner-operated Apple Support contact under the existing TS-017 assistance
plan. It does not authorize that contact, Apple Developer access, diagnostic
repetition, CSR work, Keychain action, or any signing action. TS-017 remains
`not determined`; D-076 continues to defer the signed-identity path.

Any future operational approval must preserve the plan's minimum sanitized
summary, privacy limits, no-screen-share/no-upload/no-device-access rule,
no-execution rule, stop conditions, and closed outcome reporting. End the
contact without acting on advice if any state-changing or prohibited request is
made.

### D-077 owner-contact closed without contact

Date: 2026-08-04

The separately approved owner-operated contact increment ended without an Apple
Support contact. The owner reported only the approved closed categories:

- contact attempted: `no`;
- guidance: `none`;
- state changed: `not observed`; and
- cause: `not determined`.

No Apple Support or Apple Developer access, disclosure, diagnostic, or state
change occurred. The operational approval is closed and does not carry forward.
TS-017 remains unresolved, and any future contact requires another separately
approved exact operational increment under D-077 and the existing assistance
plan.

### D-095 Xcode-managed recovery planning

Date: 2026-08-28

The owner selected a documentation-only candidate that retains the existing
Developer ID Application decision for a private Mac proof while using Xcode's
documented certificate-creation route rather than retrying Certificate
Assistant. This is not evidence that Xcode avoids, explains, or resolves
TS-017. No Xcode, Apple service, certificate, Keychain, signing, credential,
or source action occurred. D-076 remains in force until a separately approved
owner-operated execution increment produces sanitized target-Mac evidence.

### Xcode and Keychain show a Developer ID pair but CLI reports no usable identity

Date: 2026-08-28
Status: Blocked; cause not determined

After exact owner confirmation, Xcode created and listed one Developer ID
Application certificate record without a visible error. Sanitized
metadata-only checks for the current macOS user found zero matching local
certificates and zero usable code-signing identities. On 2026-08-29, the owner
reported the approved closed category that Keychain Access shows the Developer
ID Application certificate with a private key beneath it. This confirms local
pairing without exposing an identifier or screenshot, but does not prove CLI
usability, non-exported owner control, or signing.

The macOS sign-in Apple Account differs from the Apple Developer account used
in Xcode. That difference is not itself evidence of the cause: Xcode separately
selects a developer account and team, while signing requires the corresponding
private key to be available locally. A wrong Xcode account/team or a key
unavailable to the current macOS user's Keychain could matter, but neither was
established. No operator-initiated import, export, revocation, removal,
alternate certificate, post-discrepancy account/Keychain mutation, or retry was
attempted. At that point, the Keychain/CLI discrepancy remained unresolved.

Later on 2026-08-29, the owner confirmed the target Mac remained personally
controlled, accepted the exact documented OS trust-service and local process-
metadata residual boundaries, acknowledged the identity-metadata scope, and
approved one execution of the exact sanitized `keychain_identity_v1` wrapper.
The single default-user-Keychain query returned
`passed_one_label_matched_valid_codesigning_identity` and was not retried. This
establishes current scoped visibility of one label-matched valid code-signing
identity and supersedes the earlier zero result only for current visibility. It
does not determine why the earlier checks differed, prove provenance,
non-exportability or custody, complete a signed build, resolve TS-017, or
authorize remediation. The owner separately reported
`authorization_prompt=not_observed` and `state_changed=not_observed`.

Private screenshots used during the session crossed the plan's intended
identifier-free chat boundary. No identifier or signing material entered the
repository, and future evidence must use sanitized categories or private owner
attestation rather than screenshots. Any further discrepancy investigation
requires a new documentation-only plan and separate owner approval.

## TS-018 - Sandboxed post-increment gate state write is denied

Date: 2026-08-11
Status: Resolved for the active Codex session

### Symptom

Starting the documentation-only Hermes ADR transport revision with the required
post-increment gate command returned:

    post-increment-gate: post-increment state could not be written

The working tree was clean and .codex/state plus its existing state file were
owned by the repository user and had ordinary writable Unix modes.

### Cause

The Codex workspace sandbox denied creating the gate's temporary state file in
.codex/state, independently of Unix ownership and mode. This is an execution
environment restriction, not a repository permission, product, or gate defect.

### Resolution

Run the same required gate command with explicitly approved elevated workspace
permission. It created the active hermes-adr-transport-revision marker without
changing the hook, reducing gate checks, or altering repository controls.

The same sandbox restriction recurred on 2026-08-20 before Command Center M5
closeout as `post-increment state directory is unsafe` because `.codex/state`
did not yet exist and sandboxed directory creation failed inside the hook's
safety guard. The exact elevated `begin` command succeeded and `status` then
reported the expected active increment; no hook or permission check changed.

### Verify

Run the post-increment gate status command. Expected: the increment is active
until its required review and closeout workflow writes a valid completion state.

### Prevention

When a required repository hook can read but cannot atomically write its local
state under a managed sandbox, inspect the state path and rerun that exact hook
with explicit elevated permission. Do not bypass, edit, or disable the gate.

## TS-019 - In-app Browser Control does not expose browser-chrome zoom

Date: 2026-08-20
Status: Resolved

### Symptom

The installed in-app Browser Control runtime can set exact rendered viewport
sizes and operate page content, but Command/Control `+`, `=`, and `0` leave
`innerWidth`, `devicePixelRatio`, and computed heading size unchanged. The
packaged Tauri WebView also ignores its application zoom shortcut.

### Cause

The approved Browser Control surface sends input to the rendered page viewport;
it does not expose the surrounding browser chrome or a browser zoom capability.
The native WebView is not a substitute for the required real-browser zoom row.

### Resolution

Keep M5 and gate `native-multi-agent-command-center-prototype` Active. All other
rendered browser/Tauri rows passed with approved tooling. Resume only when an
approved rendered-control capability can exercise browser chrome; do not
substitute standalone Playwright, source inspection, JSDOM, or CSS transforms.

On 2026-08-25 the owner applied host zoom while Browser Control held the
approved 1040×700 frame. The embedded page inherited the scale: Browser Control
measured DPR 1.25 and an 832×560 CSS viewport, captured the rendered state, and
verified no horizontal overflow or clipped controls, real page/sidebar
scrolling, final-control reachability, and visible keyboard focus. Owner reset
restored exactly 1040×700 at DPR 1. This resolved the M5 evidence gap without a
source change or substitute rendering mechanism.

### Verify

At an approved viewport, apply real browser zoom through browser chrome and
confirm that rendered scale changes while controls, labels, focus, scroll
ownership, and horizontal overflow remain correct. Restore zoom to 100% before
closeout.

## TS-020 - PR #57 audit reports new development-transitive advisories

Date: 2026-08-25
Status: Resolved

### Symptom

PR #57 CI run `32921400121`, dependency job `98035560426`, passes repository
secret scanning and then fails `npm audit --audit-level=low`. A fresh local
audit reproduces five vulnerable package-level findings across six lockfile
nodes: four High and one Moderate.

### Observed evidence

- Both `brace-expansion` major lines, `js-yaml`, `nanoid`, `postcss`, and
  `undici` are indirect development-only lockfile entries.
- The production-only audit reports zero vulnerabilities.
- Current parent ranges admit patched resolutions without a direct, parent, or
  major upgrade: `brace-expansion@1.1.18` and `5.0.9`, `js-yaml@4.3.1`,
  `nanoid@3.3.18`, `postcss@8.5.26`, and `undici@7.29.0`.
- Repository secret scanning passes; this failure is advisory-registry
  evidence, not a detected repository secret.

The bounded resolver advanced exactly those six nodes. A scripts-disabled
clean install now resolves every expected safe version without an invalid or
extraneous package. Each changed node remains development-only, MIT,
integrity-bound, engine-compatible, and without an install hook. Full and
production-only npm audits report zero, and complete `npm run verify` passes.
The manifest, parent graph, lockfile version, and existing install-script
allowlist remain unchanged.

### Cause

The exact locked development-tool versions now fall inside current published
npm advisory ranges. This does not establish that untrusted input exploited the
tooling or that product runtime dependencies are affected.

### Approved resolution boundary

Gate `pr57-transitive-advisory-remediation` used the npm resolver with install
scripts disabled to advance only those six nodes within their existing parent
ranges. No override, direct dependency, parent-graph, audit-policy, or CI
change was needed. The resolution required PR #57 to remain unmerged until
independent review, a valid marker, and exact-head PR evidence passed. Those
conditions and the later closeout-docs check passed before merge.

Published remediation `c3cc49ee28444397ac957d7279ddcfb3ce608548` passes CI
run `32923751481`: classifier job `98042347127` in 9s, target-Mac Rust job
`98042378918` in 2m12s, Linux Rust job `98042378943` in 6m38s, frontend job
`98042378946` in 1m10s, and dependency/secret job `98042378964` in 4m39s.
Documentation run `32923751571`, job `98042347401`, passes in 27s. The
dependency job passed repository secret scanning, the full npm audit, and the
unchanged accepted Rust advisory-baseline gate. This resolves the audit failure
without an exception, override, parent upgrade, or policy change. Deterministic
closeout is complete and valid.

### Verify

Require `npm ls` to show the six exact safe resolutions with no invalid or
extraneous package, full and production-only npm audits to report zero, the
manifest and install-script allowlist to remain unchanged, complete repository
verification to pass, the exact published remediation head's dependency audit
to pass, and the later closeout-docs head's applicable documentation check to
pass before merge.

### Publication result

Exact closeout head `3a0ee66b12df531002f829f6905aff10744f4cee` passed
Documentation run `32928080852`. PR #57 squash-merged to `main` at
`3987387b7d203cb155a00c2718e1b1fe92585bdb`. Merged-main Documentation run
`32928154686` and CI run `32928154706` both pass, including secret scanning,
the zero-finding npm audit, and the unchanged accepted Rust advisory-baseline
gate. TS-020 remains Resolved without an exception, override, parent upgrade,
or policy change.

## TS-021 - Optional JSON parsing collapsed prohibited retry metadata

Date: 2026-08-28
Status: Resolved

### Symptom

The V0-1 Personal Assistant failure contract prohibits any
`retry_after_ms` field because the sealed profile owns zero retries. The shared
gateway event type represents that field as `Option`, so explicit
`"retry_after_ms": null` deserialized to the same `None` value as an absent
field and could not be distinguished by the typed event check alone.

### Cause

An optional typed field preserves value semantics but not JSON key-presence
semantics. Reusing it as proof that a forbidden key was absent would have made
the Personal Assistant protocol less strict than its closed wire contract.

### Resolution

The transport-free Personal Assistant turn now performs a bounded, local raw-
JSON preflight before the shared validator. For `response_failed` events it
rejects the presence of `retry_after_ms` regardless of whether the value is
null, numeric, or another invalid shape. The shared gateway protocol and the
existing Initial profile remain unchanged.

### Verify

Require focused tests for absent metadata and explicit null/numeric metadata,
then run strict Clippy, the gateway/runtime contract suites, complete
repository verification, security scanning, and diff inspection. A rejected
frame must terminal-fail transactionally and no later event may resume it.

## TS-022 - Linux Clippy rejected macOS-only test imports

Date: 2026-08-28
Status: Resolved locally; corrected-head CI pending

### Symptom

PR #79 CI run `33217662961`, Linux Rust job `99004869413`, failed the
warning-denied Clippy step with unused imports in the V0-1 gateway-request and
Native-runtime test modules. Formatting passed and Rust tests were skipped
after lint failed. The target-Mac Rust job `99004869430` passed Clippy and all
tests.

### Cause

The imported symbols were used only by test helpers and assertions already
guarded with `#[cfg(target_os = "macos")]`. They were therefore used on macOS
but remained unused imports when the same test modules compiled on Linux.

### Resolution

Move only the macOS-only imports behind the same target guard as their
consumers. Keep cross-platform imports unconditional. Add no lint allowance,
test skip, production branch, dependency, or workflow change.

### Verify

Run formatting, focused gateway and Native tests, strict all-target/all-feature
Clippy, agent acceptance, complete verification, audit/security/repository/docs
checks, diff inspection, and the completion gate. Push only the verified
correction, then require PR #79's corrected head to pass Linux Rust and every
other required check before squash merge.
