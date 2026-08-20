# Native multi-agent Command Center deterministic prototype

Status: Active — source implemented; mandatory real-browser/Tauri validation pending
Owner: Project owner
Last updated: 2026-08-20

## Goal

Add one reversible, deterministic, fixture-only Command Center route that
explains the existing native multi-agent architecture without connecting React
to Rust agent state or implying live execution.

## User-visible outcome

The sidebar exposes a lazy Command Center. Its non-editable topology and
equivalent grouped structured view show one separate `AgentOrchestrator`, all
nine exact roles in five presentation groups, seven deterministic scenarios,
selected-entity detail, bounded activity, local search/filters, and persistent
`DEMO MODE · SIMULATED AGENT DATA` disclosure.

## Scope and boundaries

The implementation is frontend-only. Projection values are closed, immutable,
bounded, and fixture-derived. Search, filters, selection, viewport controls,
the inspector, and activity are presentation state only. No control crosses
IPC or changes trusted state.

No Rust agent/runtime/governance/workflow source, Tauri command, capability,
CSP, storage, provider, model, tool, approval, policy, audit, permission,
network, filesystem, persistence, Hermes, or external runtime behavior changed.

## Dependencies

The owner approved exactly:

- `@xyflow/react@12.11.3` (MIT);
- `lucide-react@1.33.0` (ISC).

The lockfile adds 19 reviewed transitives. The production dependency audit
reports zero vulnerabilities. Five pre-existing development-only advisories
(one moderate and four high) remain unchanged. React Flow types/imports are
confined to
`src/features/command-center/components/OperationalTopologyAdapter.tsx`.

## Implementation evidence

- one lazy route and accessible loading state;
- one distinct orchestrator, nine exact roles, and five view-only groups;
- seven closed scenarios covering idle, queued, active, approval wait, blocked,
  cancelled, failed, and completed/success presentation;
- fixed non-editable graph with ordinary wheel pass-through;
- one 180 x 88 node geometry contract with two-line authoritative labels, a
  separate status/domain row, and matching deterministic group bounds;
- synchronized grouped structured view and relationship table;
- contextual inspector and bounded structured activity;
- local search and domain/agent/status/entity/scenario/origin filters;
- graph composite keyboard source behavior and explicit zoom/fit/reset/center;
- persistent demo disclosure in the page and detail surfaces;
- application-content remains the primary page scroll owner;
- protected Rust/Tauri/IPC/storage paths are unchanged.

## Automated verification

- projection tests: 64/64;
- Command Center page tests: 14/14;
- focused viewport-adapter tests: 5/5;
- `App` tests: 28/28;
- application-state tests: 31/31;
- focused total: 142/142;
- full frontend: 211/211 across 13 files;
- frontend formatting, lint, typecheck, and production build: Passed;
- strict Rust formatting and Clippy: Passed;
- `cargo test --manifest-path src-tauri/Cargo.toml --all-targets --all-features
--locked --quiet`: 481 passed, zero failed, one intentional Hermes probe
  ignored.

Final current-tree `npm run verify` passes after source and documentation
synchronization, including the Tauri release no-bundle build.

## Bundle evidence

Measured with the M0 ledger's reproducible `gzip -cn` method:

- initial JavaScript: 71,189 gzip bytes;
- initial CSS: 4,994 gzip bytes;
- initial combined: 76,183 gzip bytes, +1,119 from the 75,064 baseline and
  9,121 below the +10 KiB cap;
- lazy Command Center JavaScript: 80,516 gzip bytes;
- lazy Command Center CSS: 5,834 gzip bytes;
- lazy combined: 86,350 gzip bytes, 67,250 below the 150 KiB cap.

The Command Center remains a separate lazy production chunk.

## Required validation not run

The required Browser runtime tool is unavailable in this session. A local Vite
server and the existing Tauri debug executable launched, but macOS denied the
assistive access required for deterministic navigation, resizing, and
screenshots; both processes were stopped. No browser/E2E tooling install was
authorized. The real-browser/Tauri matrix for
2560×1440, 1600×1000, 1040×700, 1040×520, 760×520, and browser-only 640×800,
mouse wheel, trackpad, scrollbar, keyboard, touch where supported, browser
zoom, resize, focus visibility/scrolling, computed overflow, light/dark
contrast, reduced motion, final-item reachability, and obstruction is **Not
run**.

JSDOM/source evidence does not prove those rendered or native behaviors. This
required pending evidence blocks Complete status and a completion marker.

## Review result

Independent source, architecture, security, accessibility, and dependency
review found no source blocker. The checkpoint result is `PASS WITH
ADVISORIES`; the advisory is the mandatory unexecuted real-browser/Tauri
matrix. No later or live integration increment is Ready.

## Rollback

Remove the feature directory and route integration, remove only its shared
tokens/tests, and use npm to uninstall the two exact direct dependencies before
reviewing the generated manifest/lockfile diff. No migration, user data,
credential, remote resource, or Rust/Tauri rollback exists.
