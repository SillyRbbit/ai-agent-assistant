# Vitest development-tool security repair

Status: Local security patch, full verification and final documentation checks passed. This frozen source is input to actual gate/Stop and authorized Git publication; their actual result is recorded externally.

## Objective and baseline

Patch exact Vitest4.1.11 to4.1.12 and its necessary matching locked graph to resolve GHSA-qxg6-89mg-rqv8 and GHSA-xp26-3454-mjgp. Baseline is integrated main31b7148afe075f2c304dae715c08eaff2129164d, exact reviewed Tasks tree5e2e95a53b654d7551075819efcc8d2ec749e070. PR152's six checks passed; its later main npm audit failed. Preserve both results and all original acceptance.

## Scope and ownership

Only package.json/package-lock.json, this plan/reviews and additive current-state documentation. No product, Rust, RAG/Knowledge, migrations, IPC, permissions, gate implementation, provider, credential, image or HP changes. The parallel RAG chat is doing CM successor-admission maintenance in Desktop/memory gates/docs; those checkouts remain untouched. A separate worktree is not itself an independence claim. This repair directly resolves the inherited public audit findings, not an unrelated failed gate history.

## Acceptance

- V1: Exact coherent Vitest4.1.12 graph and unchanged unrelated locked packages; registry URLs and package integrity verified.
- V2: Fresh npm audit --audit-level=low and all frontend tests pass, with strict parsing/rejection unchanged.
- V3: Required npm run verify passes under pinned existing Node/npm/Rust/SDK and offline Cargo; no application launch, native acceptance or model request.
- V4: Recoverable backup, preservation, review, additive docs and supported finalization/Stop pass.
- V5: Scoped commit/PR and squash merge only after checks/review, with exact reviewed tree/ancestry and remote-result verification.

## Resources and execution

Final fresh envelope:8,000,000,000bytes, within the original8GiB estimate. Caps: target5,800,000,000; public Cargo registry1,200,000,000; node_modules320,000,000; npm cache268,435,456; tmp134,217,728; source100,663,296; evidence67,108,864 including1MiB closeout. Preserve8GiB available floor,64MiB headroom and the concurrent RAG remaining hold. Current measurements and future demand remain distinct. Historical8GiB/5GB/2GiB phase records and rejections remain external and immutable.

Copied public Cargo archives and dependency artifacts use independent clonefile inodes, with no full-copy fallback, no old-cache writes, no QA bundles or application outputs. Verified472 locked archive checksums and50,340 copied-file hashes; Cargo remains locked/offline. Logical clone allocations count fully; physical-growth accounting conservatively observes available-space decline without attributing it to this task. The original baseline is preserved through the finite envelope amendments. No disk recovery or release of older reservations is claimed.

Commands use finite timeouts: preparation/checks600seconds, frontend600seconds, install900seconds, full verify7200seconds and CI observation45minutes. Pinned Node26.3.0/npm11.16.0, Rust1.90.0 and Xcode/SDK use a cleared task environment, two build jobs and the existing process-local non-stripping workaround. Local npm install scripts remain disabled; the unchanged Linux CI esbuild install route was source-reviewed. No generic audit fix, global tool/config change, fallback model, live QA or credentials access.

## Verification and recovery

Readiness: documentation, repository, secret-pattern, whitespace and session checks. Then normal schema-2 begin before dependency edits. Inspect exact package delta and lock integrity, run npm audit, frontend tests and npm run verify. Use scoped architecture/security/code-health/readiness review, final documentation checks, ordinary finalization/current status/full Stop. Preserve original Tasks and memory reports byte-identically. Publication must preserve snapshot-bound evidence; do not silently rebind admission.

Before-copies are byte-verified in external BEFORE.zip/BEFORE.json. Never reset, clean, stash, delete retained artifacts or roll back unrelated work. Failed checks require diagnosis and the smallest attributable correction; no unknown warning is waived.

## Progress

- [x] Current main, user authority, ownership and source-bound public advisories reconciled.
- [x] Fresh resource reservation and verified recoverable baseline snapshot.
- [x] Readiness checks and supported admission.
- [x] Minimal dependency repair and integrity audit: exactly eight matching package entries, zero audit vulnerabilities,44 frontend files/629 tests passed.
- [x] Required full verification and scoped implementation review.
- [ ] Formal finalization, scoped Git integration and truthful closeout.

## Preserved preparation failures

The exact locked installation exited0 but its final monitor rejected an undersized134,217,728-byte npm-cache subcap. The159,346,688-byte cache is preserved; no installation retry occurred. Reallocated134,217,728 bytes from target to cache within the unchanged aggregate; installed graph, audit and frontend checks then passed. Earlier readiness environment/capacity failures remain separate. Node26 experimental localStorage warnings remain visible; they did not fail tests and grant no native protocol qualification.

## Current results and publication boundary

Fresh full verification passed against unchanged293 bound product inputs. Local final documentation, current gate/Stop and exact staging/commit/remote verification remain required. V5 is a supported-publication-preflight criterion; actual Git integration is an independent post-local-acceptance obligation and is not claimed by the local completion marker. Preserve all interrupted commands and the rejected install monitor; no historical cause or exit is reconstructed.
