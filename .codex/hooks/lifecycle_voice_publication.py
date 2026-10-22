"""Exact voice CI repair admission; historical FAIL is never promoted.

Local same-user custody records detect drift; they are not authentication,
execution authority, or a replacement for CI and substantive review.
"""
from __future__ import annotations

import json
import os
from pathlib import Path

import lifecycle_closure as closure
import post_increment_gate as gate

SUCCESSOR = "foreground-voice-publication-correction"
PREDECESSOR = "foreground-voice-git-integration"
ROOT = "/Users/hdang/.codex/worktrees/foreground-push-to-talk-voice/ai-agent-assistant"
BRANCH = "codex/foreground-push-to-talk-voice"
HEAD = "7ea1b23ffd1c240ce6504594795a21e5e69cc694"
TREE = "df57ae2c77a0881505b304f29de0a7798205650e"
FAILURE = "e96255e5053674b287eded3ff3913227df340358d1786d6d6e267acc919a8adb"
REPORT = "4f8cbad328761bb87288a3116fab246116a491891ec508620a46b2e26c75fcd4"
AUTHORITY = "c9111718263dc78dc8646ff6fe08741da22fe771f2f184d571e8b69e44bfcc6c"
ACCEPTED_REPORT = "8560cd0cc3b58a2d1292fca497cd72cf5e629064d3e90e6c1119876516794b44"
ACCEPTED_ADMISSION = "3c712cb5b6e2efff1f2877ada85a908db2762f5fc8f45289e9601b695746540c"
PRODUCT = "src-tauri/src/voice_tauri.rs"
STORE = Path(".codex/state/voice-publication-correction")
POINTER = STORE / "consumed.json"
MAINTENANCE = {".codex/hooks/lifecycle_voice_publication.py",
               ".codex/hooks/tests/test_lifecycle_voice_publication.py",
               ".codex/hooks/post_increment_gate.py",
               ".codex/hooks/tests/test_post_increment_gate.py"}
DOCS = {"CHANGELOG.md", "DECISIONS.md", "HANDOFF.md", "NEXT_STEPS.md", "PLANS.md",
        "PROJECT_STATUS.md", "TROUBLESHOOTING_LOG.md",
        "docs/plans/2026-10-22-foreground-voice-git-integration.md",
        "docs/plans/2026-10-22-foreground-voice-publication-correction.md",
        "docs/reviews/2026-10-22-foreground-voice-git-integration-post-increment-review.md",
        "docs/reviews/2026-10-22-foreground-voice-publication-correction-post-increment-review.md"}
# The old failed report is publishable unchanged, never editable.
FAILED_REPORT_PATH = "docs/reviews/2026-10-22-foreground-voice-git-integration-post-increment-review.md"
EDITABLE = (DOCS - {FAILED_REPORT_PATH}) | {PRODUCT}
PUBLISHABLE = DOCS | MAINTENANCE | {PRODUCT}
CRITERIA = {"p1", "p2", "p3", "p4"}
require = closure.require
keys = closure.keys


def parse(data: bytes):
    def unique(pairs):
        result = {}
        for name, value in pairs:
            require(name not in result, "voice duplicate JSON field")
            result[name] = value
        return result
    return json.loads(data, object_pairs_hook=unique)


def reference(root: Path, value):
    keys(value, {"path", "sha256"})
    closure.sha(value["sha256"])
    data = closure.read_bytes(closure.relative(root, value["path"]))
    require(closure.digest(data) == value["sha256"], "voice evidence drift")
    return data


def exclusive(root: Path, name: Path, data: bytes):
    path = closure.relative(root, name.as_posix())
    require(not any(p.is_symlink() for p in [path, *path.parents]), "voice unsafe custody")
    path.parent.mkdir(parents=True, exist_ok=True)
    with os.fdopen(os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600), "wb") as handle:
        handle.write(data)
        handle.flush()
        os.fsync(handle.fileno())


NONMAC_TEST = '''    #[cfg(not(target_os = "macos"))]
    #[test]
    fn non_mac_helper_stays_unsupported() {
        assert!(matches!(
            verify_helper(Path::new("not-a-helper")),
            Err(VoiceError::Unsupported)
        ));
    }
'''


def corrected_product(original: bytes) -> bytes:
    text = original.decode()
    old = '#[cfg(not(target_os = "macos"))]\ntype HelperIdentity = ();'
    new = '#[cfg(not(target_os = "macos"))]\n#[derive(PartialEq, Eq)]\nstruct HelperIdentity;'
    anchor = '    #[test]\n    fn ipc_unknown_fields_and_unknown_operations_reject()'
    require(text.count(old) == 1 and text.count(anchor) == 1, "voice exact repair context missing")
    return text.replace(old, new, 1).replace(anchor, NONMAC_TEST + anchor, 1).encode()


@closure.guarded
def shape(state):
    require(state.get("schema_version") == 5 and state.get("increment_id") == SUCCESSOR,
            "voice state identity")
    require(not any(k in state for k in ("closure_lineage", "acceptance_lineage",
                "predecessor_disposition", "successor_disposition")), "voice mixed lineage")
    keys(state["dependent_lineage"], {"admission"})
    closure.sha(state["dependent_lineage"]["admission"])


def checkout(root):
    require(str(root.resolve()) == ROOT, "voice checkout mismatch")
    require(gate._run_git(root, "branch", "--show-current").decode().strip() == BRANCH,
            "voice branch mismatch")
    require(not gate.has_merge_conflicts(root), "voice conflicts")


def evidence(root, req):
    keys(req, {"profile", "head", "tree", "fingerprint", "index_sha256", "owner",
               "failure", "accepted", "product_before", "maintenance"})
    require(req["profile"] == SUCCESSOR and req["head"] == HEAD and req["tree"] == TREE,
            "voice request identity")
    keys(req["owner"], {"approved", "authorization", "decision"})
    require(req["owner"]["approved"] is True and
            req["owner"]["authorization"]["sha256"] == AUTHORITY, "voice owner authority")
    reference(root, req["owner"]["authorization"])
    reference(root, req["owner"]["decision"])
    keys(req["failure"], {"state", "report", "admission"})
    require(req["failure"]["state"]["sha256"] == FAILURE and
            req["failure"]["report"]["sha256"] == REPORT and
            req["failure"]["report"]["path"] == FAILED_REPORT_PATH,
            "voice exact failure required")
    old = parse(reference(root, req["failure"]["state"]))
    gate.validate_state(old)
    require(old["status"] == "failed" and old["quality_gate"] == "FAIL" and
            old["increment_id"] == PREDECESSOR and old["head_commit"] == HEAD and
            old["report_sha256"] == REPORT, "voice failure history changed")
    reference(root, req["failure"]["report"])
    admission_data = reference(root, req["failure"]["admission"])
    require(closure.digest(admission_data) == old["closure_lineage"]["admission"],
            "voice historical admission mismatch")
    closure.validate_lineage(root, old, scope=False)
    keys(req["accepted"], {"report", "admission"})
    require(req["accepted"]["report"]["sha256"] == ACCEPTED_REPORT,
            "voice accepted report identity")
    reference(root, req["accepted"]["report"])
    require(req["accepted"]["admission"]["sha256"] == ACCEPTED_ADMISSION,
            "voice accepted admission identity")
    reference(root, req["accepted"]["admission"])
    original = reference(root, req["product_before"])
    require(original == gate._run_git(root, "show", HEAD + ":" + PRODUCT),
            "voice original product mismatch")
    keys(req["maintenance"], {"files", "validation"})
    keys(req["maintenance"]["files"], MAINTENANCE)
    for name, ref in req["maintenance"]["files"].items():
        require(ref["path"] == name, "voice maintenance role")
        reference(root, ref)
    validation = parse(reference(root, req["maintenance"]["validation"]))
    keys(validation, {"result", "input_hashes", "command", "receipt"})
    require(validation["result"] == "Passed" and validation["input_hashes"] ==
            {n: r["sha256"] for n, r in req["maintenance"]["files"].items()},
            "voice maintenance validation binding")
    receipt = parse(reference(root, validation["receipt"]))
    require(receipt["exit"] == 0 and receipt["boundary"] is None and
            receipt["owned_group_absent"] is True and receipt["source_before"] == receipt["source_after"] and
            validation["command"] == " ".join(receipt["argv"]),
            "voice actual maintenance verification missing")
    return old, original


def publication_history(root, baseline, original):
    current = gate.current_head_commit(root)
    require(gate._is_ancestor(root, HEAD, current), "voice unsupported ancestry")
    commits = gate._run_git(root, "rev-list", "--reverse", HEAD + ".." + current).decode().splitlines()
    require(len(commits) <= 8, "voice bounded publication history")
    parent = HEAD
    for commit in commits:
        parents = gate._run_git(root, "rev-list", "--parents", "-n", "1", commit).decode().split()
        require(parents == [commit, parent], "voice unexpected history or merge")
        paths = set(gate._decode_git_paths(gate._run_git(root, "diff", "--name-only", "-z", parent, commit)))
        require(paths.issubset(PUBLISHABLE), "voice unrelated committed scope")
        for name in paths:
            entry = gate._run_git(root, "ls-tree", commit, "--", name).decode().split()
            require(len(entry) == 4 and entry[0] == "100644" and entry[1] == "blob",
                    "voice deletion or mode change")
            data = gate._run_git(root, "show", commit + ":" + name)
            if name in MAINTENANCE:
                require(closure.digest(data) == baseline[name]["sha256"], "voice unreviewed gate commit")
            if name == PRODUCT:
                require(data in {original, corrected_product(original)}, "voice unreviewed product commit")
        parent = commit
    staged = set(gate._decode_git_paths(gate._run_git(root, "diff", "--cached", "--name-only", "-z")))
    require(staged.issubset(PUBLISHABLE), "voice unrelated staged scope")
    for name in staged:
        row = gate._run_git(root, "ls-files", "--stage", "--", name).decode().split()
        require(len(row) == 4 and row[0] == "100644" and row[2] == "0",
                "voice staged deletion, mode or conflict")
        require(gate._run_git(root, "show", ":" + name) == closure.read_bytes(root / name),
                "voice staged content differs from reviewed workspace")
    # Hidden index entries can conceal workspace changes; reject them explicitly.
    flags = gate._run_git(root, "ls-files", "-v", "-z").split(b"\0")
    require(all(not row or row.startswith(b"H ") for row in flags), "voice hidden index entry")


@closure.guarded
def begin(root, increment, path):
    require(increment == SUCCESSOR, "voice successor mismatch")
    checkout(root)
    require(not (root / STORE).exists() or not any((root / STORE).glob("*.json")),
            "voice admission replay or incomplete custody")
    require(gate.current_head_commit(root) == HEAD and
            gate._run_git(root, "rev-parse", "HEAD^{tree}").decode().strip() == TREE,
            "voice starting commit drift")
    req = parse(closure.read_bytes(closure.relative(root, path)))
    old, original = evidence(root, req)
    raw_state = closure.read_bytes(gate.state_path(root))
    require(closure.digest(raw_state) == FAILURE and parse(raw_state) == old,
            "voice current failure mismatch")
    require(req["fingerprint"] == gate.workspace_fingerprint(root) and
            req["index_sha256"] == closure.digest(closure.read_bytes(closure.git_index_path(root))),
            "voice admission snapshot drift")
    require(closure.read_bytes(root / PRODUCT) == original, "voice premature product repair")
    baseline = closure.manifest(root)
    require(baseline["DECISIONS.md"]["sha256"] == req["owner"]["decision"]["sha256"],
            "voice current owner decision drift")
    record_data = closure.encoded({"request": req, "baseline_manifest": baseline})
    identity = closure.digest(record_data)
    exclusive(root, STORE / (identity + ".json"), record_data)
    require(gate.workspace_fingerprint(root) == req["fingerprint"] and
            closure.read_bytes(gate.state_path(root)) == raw_state, "voice admission race")
    exclusive(root, POINTER, closure.encoded({"admission": identity}))
    gate.write_state(root, {"schema_version": 5, "status": "active", "increment_id": SUCCESSOR,
        "baseline_fingerprint": req["fingerprint"], "dependent_lineage": {"admission": identity}})


@closure.guarded
def record(root, state, *, scope=True):
    shape(state)
    checkout(root)
    identity = state["dependent_lineage"]["admission"]
    pointer = parse(closure.read_bytes(root / POINTER))
    keys(pointer, {"admission"})
    require(pointer["admission"] == identity, "voice consumed identity drift")
    data = closure.read_bytes(root / STORE / (identity + ".json"))
    require(closure.digest(data) == identity, "voice admission tamper")
    value = parse(data)
    keys(value, {"request", "baseline_manifest"})
    req, before = value["request"], value["baseline_manifest"]
    _, original = evidence(root, req)
    require(state["baseline_fingerprint"] == req["fingerprint"], "voice baseline drift")
    after = closure.manifest(root)
    delta = closure.changed(before, after)
    require(delta.issubset(EDITABLE), "voice protected or unrelated workspace drift")
    for name in delta:
        require(after[name]["exists"] and after[name]["mode"] == before.get(name, {"mode": 0o644})["mode"],
                "voice deletion or metadata drift")
    require(closure.read_bytes(root / PRODUCT) in {original, corrected_product(original)},
            "voice product change exceeds exact non-Mac correction")
    publication_history(root, before, original)
    return value


def carry(source, target):
    target["schema_version"] = 5
    target["dependent_lineage"] = source["dependent_lineage"]


@closure.guarded
def milestone_report(root, increment, report):
    require(increment == SUCCESSOR, "voice report identity")
    value = record(root, gate.read_state(root))
    evidence_value = report.get("milestone")
    keys(evidence_value, {"criteria", "paths"})
    keys(evidence_value["criteria"], CRITERIA)
    blocked = False
    for criterion in evidence_value["criteria"].values():
        keys(criterion, {"status", "evidence"})
        require(criterion["status"] in {"automatically_verified", "live_verified", "blocked",
                "implemented", "deferred"} and isinstance(criterion["evidence"], str) and
                bool(criterion["evidence"].strip()), "voice criterion evidence")
        blocked |= criterion["status"] not in {"automatically_verified", "live_verified"}
    delta = closure.changed(value["baseline_manifest"], closure.manifest(root))
    keys(evidence_value["paths"], delta)
    for attribution in evidence_value["paths"].values():
        keys(attribution, {"criterion", "rationale", "within_objective", "preserves_existing"})
        require(attribution["criterion"] in CRITERIA and attribution["within_objective"] is True and
                attribution["preserves_existing"] is True and isinstance(attribution["rationale"], str)
                and bool(attribution["rationale"].strip()), "voice path attribution")
    if not blocked:
        original = reference(root, value["request"]["product_before"])
        require(closure.read_bytes(root / PRODUCT) == corrected_product(original), "voice correction not verified")
        required = {"npm run verify", "python3 -B .codex/hooks/session_end_gate.py",
                    "npm run docs:check", "npm run repository:check", "npm run security:scan", "git diff --check"}
        passed = {row["command"] for row in report["verification"] if row["required"] and row["status"] == "Passed"}
        require(required.issubset(passed), "voice required verification omitted")
    return blocked
