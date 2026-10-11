"""One owner-authorized dependent documentation attempt; no executor or waiver.

Same-user evidence integrity is not authentication. Missing custody stays unknown.
"""
from __future__ import annotations

import json
import os
import re
from pathlib import Path
from typing import Any

import lifecycle_closure as closure
import post_increment_gate as gate

PREDECESSOR = "vps-dual-label-operating-mode"
SUCCESSOR = "vps-dual-label-operating-mode-remediation"
BRANCH = "codex/docs/vps-dual-label-operating-mode"
STATE_SCHEMA = 5
STORE = Path(".codex/state/dependent-admission")
POINTER = STORE / "consumed.json"
PRESERVATION_COMMAND = "python3 -B .codex/hooks/post_increment_gate.py dependent-preservation"
ROOT_DOCS = {"CHANGELOG.md", "DECISIONS.md", "HANDOFF.md", "NEXT_STEPS.md", "PLANS.md",
             "PROJECT_STATUS.md", "docs/github/SELF_HOSTED_RUNNER.md"}
EVIDENCE_KEYS = {"comparison", "historical_manifest", "snapshot", "archival_provenance",
                 "unavailable_inventory", "surviving_preservation"}
UNAVAILABLE_HASHES = {
    "adoption-package": "b22baadaac8f9f20bdcacd73c474a0738671cccb6f24859979f5f9c06cc5dfc0",
    "correction-package": "9d492c20200cfd9b852e8e0bd704eea59effb73320d387f246181ed220a72dc3",
    "validation-package": "de56d7fc3ec5fcf555c5d0cbe530566cb932dcd74945285b9bffbb702228b179",
    "bounded-index": "a18d041df1006378b157f15a80ed00193494c150c3eac3aa6c95e6164493d060",
    "publication-index": "c576cf82bb95d5b3f9a9443e0ab359112e81ab7f86c14b71b270d85e507195c1",
    "sidebar-index": "adea2cd545f1344d66f24acaad3a1916b26edc3ccb59b8374fd0a465b9e80984",
    "bounded-ignored-state": None, "publication-ignored-state": None, "sidebar-ignored-state": None,
}
UNAVAILABLE = set(UNAVAILABLE_HASHES)
require = closure.require
keys = closure.keys
digest = closure.digest
encode = closure.encoded


def parse(data: bytes) -> Any:
    def unique(pairs):
        result = {}
        for key, value in pairs:
            require(key not in result, "dependent duplicate JSON key")
            result[key] = value
        return result
    return json.loads(data, object_pairs_hook=unique)


def read(root: Path, path: str) -> bytes:
    return closure.read_bytes(closure.relative(root, path))


def reference(root: Path, value: Any) -> bytes:
    keys(value, {"path", "sha256"})
    closure.sha(value["sha256"])
    data = read(root, value["path"])
    require(digest(data) == value["sha256"], "dependent evidence hash mismatch")
    return data


def branch(root: Path) -> str:
    return gate._run_git(root, "branch", "--show-current").decode().strip()


@closure.guarded
def shape(state: dict[str, Any]) -> None:
    require(state.get("schema_version") == STATE_SCHEMA, "dependent schema mismatch")
    require(state.get("increment_id") == SUCCESSOR, "dependent task mismatch")
    require(not any(k in state for k in ("closure_lineage", "acceptance_lineage",
                "predecessor_disposition", "successor_disposition")), "dependent mixed lineage")
    keys(state["dependent_lineage"], {"admission"})
    closure.sha(state["dependent_lineage"]["admission"])


def exclusive(root: Path, name: Path, data: bytes) -> None:
    target = closure.relative(root, name.as_posix())
    require(not any(p.is_symlink() for p in [target, *target.parents]), "dependent unsafe store")
    target.parent.mkdir(parents=True, exist_ok=True)
    # Exclusive creation is intentionally fail-closed after interruption/replay.
    try:
        fd = os.open(target, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
        with os.fdopen(fd, "wb") as handle:
            handle.write(data)
            handle.flush()
            os.fsync(handle.fileno())
    except FileExistsError as error:
        raise gate.GateError("dependent replay rejected") from error



def prospective_report(root: Path, name: str) -> None:
    """Admission may bind an absent report; finalization still requires its bytes."""
    gate.validate_relative_path(name)
    relative = Path(name)
    expected = rf"{gate.REPORT_DATE_PATTERN}-{re.escape(SUCCESSOR)}-post-increment-review\.md"
    require(relative.parent == gate.REPORTS_RELATIVE_PATH
            and re.fullmatch(expected, relative.name) is not None,
            "dependent prospective report identity mismatch")
    path = root / relative
    require(not any(p.is_symlink() for p in [path, *path.parents]),
            "dependent prospective report symlink")
    require(path.parent.is_dir() and path.parent.resolve().is_relative_to(root.resolve()),
            "dependent prospective report parent unsafe")
    require(not path.exists() or path.is_file(), "dependent prospective report not regular")


def snapshot_copies(root: Path, request: dict[str, Any]) -> None:
    snapshot = parse(reference(root, request["evidence"]["snapshot"]))
    keys(snapshot, {"files"})
    files = snapshot["files"]
    require(isinstance(files, list) and bool(files), "dependent snapshot evidence missing")
    hashes = set()
    paths = set()
    for item in files:
        reference(root, item)
        require(item["path"] not in paths, "dependent duplicate snapshot copy")
        paths.add(item["path"])
        hashes.add(item["sha256"])
    required = {request["predecessor"]["state"]["sha256"],
                request["predecessor"]["report"]["sha256"], request["index_sha256"]}
    require(required.issubset(hashes), "dependent essential snapshot copy missing")
    survival = parse(reference(root, request["evidence"]["surviving_preservation"]))
    keys(survival, {"result", "scope", "files"})
    require(survival["result"] == "PASS" and survival["scope"] == "surviving-evidence"
            and survival["files"] == files, "dependent surviving copy inventory mismatch")


def obligations(original: dict[str, Any], request: dict[str, Any]) -> None:
    # unresolved consumes a report body, so construct its exact manifest wrapper.
    body = gate.MANIFEST_START + json.dumps(original) + gate.MANIFEST_END
    expected = set(closure.unresolved(body))
    keys(request["obligations"], expected)
    for value in request["obligations"].values():
        keys(value, {"criterion", "required"})
        require(value["required"] is True and value["criterion"] in request["milestone"]["acceptance"],
                "dependent obligation omitted or waived")
    prior_criteria = original.get("milestone", {}).get("criteria", {})
    require(bool(prior_criteria) and set(prior_criteria).issubset(request["milestone"]["acceptance"]),
            "dependent acceptance criterion omitted")
    change = request["acceptance_change"]
    keys(change, {"criterion", "original_sha256", "replacement", "replacement_command", "owner_authorization"})
    require(change["criterion"] == "preservation" and "preservation" in prior_criteria,
            "dependent acceptance change out of scope")
    require(change["original_sha256"] == digest(encode(prior_criteria["preservation"])),
            "dependent original criterion mismatch")
    require(change["replacement"] == "surviving-evidence-and-explicit-unavailable-custody"
            and change["replacement_command"] == PRESERVATION_COMMAND,
            "dependent acceptance replacement out of scope")


def custody(data: bytes) -> None:
    inventory = parse(data)
    keys(inventory, {"records", "complete_historical_preservation", "absence_of_loss"})
    require(inventory["complete_historical_preservation"] is False and inventory["absence_of_loss"] is False,
            "dependent historical preservation overclaim")
    rows = inventory["records"]
    require(isinstance(rows, list) and len(rows) == len(UNAVAILABLE), "dependent unavailable inventory incomplete")
    seen = set()
    for row in rows:
        keys(row, {"id", "availability", "recorded_sha256"})
        require(row["id"] in UNAVAILABLE and row["id"] not in seen
                and row["availability"] == "unavailable", "dependent custody disposition mismatch")
        seen.add(row["id"])
        require(row["recorded_sha256"] == UNAVAILABLE_HASHES[row["id"]], "dependent custody reference mismatch")


def identity(root: Path, request: dict[str, Any]) -> None:
    require(request["root"] == str(root.resolve()) and request["branch"] == BRANCH
            and branch(root) == BRANCH, "dependent root/branch identity mismatch")
    require(request["head"] == gate.current_head_commit(root), "dependent HEAD mismatch")


@closure.guarded
def begin(root: Path, increment: str, path: str) -> None:
    require(increment == SUCCESSOR, "dependent successor is task-bound")
    require(not (root / POINTER).exists(), "dependent replay rejected")
    store = root / STORE
    require(not store.is_symlink(), "dependent unsafe store")
    require(not store.exists() or not any(store.iterdir()),
            "dependent replay rejected: interrupted admission evidence exists")
    state = gate.read_state(root)
    require(state is not None and state["status"] == "failed"
            and state["increment_id"] == PREDECESSOR and state["quality_gate"] == "FAIL",
            "dependent original terminal FAIL required")
    closure.reject_d133_context(root, state)
    require("successor_disposition" not in state and "dependent_lineage" not in state,
            "dependent historical route mismatch")
    closure.validate_lineage(root, state, scope=False)
    raw_request = read(root, path)
    request = parse(raw_request)
    keys(request, {"schema_version", "increment_id", "root", "branch", "head",
        "workspace_fingerprint", "index_sha256", "predecessor", "evidence", "input_bindings", "milestone",
        "allowed_paths", "plan", "report", "readiness_report", "readiness_increment",
        "obligations", "acceptance_change", "owner_authorization"})
    require(type(request["schema_version"]) is int and request["schema_version"] == 1
            and request["increment_id"] == SUCCESSOR, "dependent request identity mismatch")
    identity(root, request)
    require(request["index_sha256"] == digest(closure.read_bytes(closure.git_index_path(root))), "dependent index binding mismatch")
    require(gate._is_ancestor(root, state["head_commit"], request["head"]), "dependent historical HEAD lineage mismatch")
    require(request["workspace_fingerprint"] == gate.workspace_fingerprint(root), "dependent input drift")
    keys(request["owner_authorization"], {"admission", "execution"})
    for authorization in request["owner_authorization"].values(): closure.authorization(root, authorization)
    closure.authorization(root, request["acceptance_change"]["owner_authorization"])
    closure.milestone_contract(request["milestone"])
    require(request["milestone"]["authorized_destructive_paths"] == [], "dependent destruction forbidden")
    keys(request["predecessor"], {"state", "report"})
    raw_state = reference(root, request["predecessor"]["state"])
    require(raw_state == closure.read_bytes(gate.state_path(root)) and parse(raw_state) == state,
            "dependent predecessor binding mismatch")
    raw_report = reference(root, request["predecessor"]["report"])
    require(request["predecessor"]["report"]["path"] == state["report_path"]
            and digest(raw_report) == state["report_sha256"], "dependent original report mismatch")
    original = gate._extract_manifest(raw_report.decode())
    require(original["increment_id"] == PREDECESSOR and original["quality_gate"] == "FAIL",
            "dependent original report must remain FAIL")
    require("closure_lineage" in state, "dependent predecessor needs schema-2 admission")
    prior_admission = closure.read_json(root / closure.DIRECTORY / ("admission-" + state["closure_lineage"]["admission"] + ".json"))
    require(prior_admission["request"]["schema_version"] == 2, "dependent predecessor needs schema-2 admission")
    prior_acceptance = prior_admission["request"]["milestone"]["acceptance"]
    require(set(request["milestone"]["acceptance"]) == set(prior_acceptance), "dependent acceptance criterion omitted or expanded")
    for name, description in prior_acceptance.items():
        if name != "preservation":
            require(request["milestone"]["acceptance"][name] == description, "dependent acceptance text drift")
    require(request["milestone"]["acceptance"]["preservation"] == "surviving-evidence-and-explicit-unavailable-custody", "dependent replacement checklist mismatch")
    obligations(original, request)
    keys(request["evidence"], EVIDENCE_KEYS)
    evidence = {label: reference(root, value) for label, value in request["evidence"].items()}
    historical = closure.normalize_manifest(parse(evidence["historical_manifest"]))
    require(closure.fingerprint(historical) == state["workspace_fingerprint"], "dependent historical inventory mismatch")
    comparison = parse(evidence["comparison"])
    require(comparison.get("pass") is False and comparison.get("exit_code") == 1,
            "dependent failed comparison missing")
    custody(evidence["unavailable_inventory"])
    survival = parse(evidence["surviving_preservation"])
    require(survival.get("result") == "PASS" and survival.get("scope") == "surviving-evidence",
            "dependent surviving preservation unverified")
    provenance = parse(evidence["archival_provenance"])
    require(provenance.get("owner_intent") == "intentional-archival"
            and provenance.get("complete_custody") is False, "dependent archival provenance unqualified")
    snapshot_copies(root, request)
    prospective_report(root, request["report"])
    gate.validate_relative_path(request["plan"])
    require(request["plan"].startswith("docs/plans/") and request["plan"].endswith(".md"), "dependent plan path")
    allowed = closure.path_list(request["allowed_paths"])
    require(set(allowed) == ROOT_DOCS | {request["plan"], request["report"]}, "dependent documentation scope mismatch")
    protected = {state["report_path"], request["readiness_report"]}
    require(not protected.intersection(allowed) and protected.issubset(request["milestone"]["protected_paths"]),
            "dependent historical evidence not protected")
    baseline = closure.manifest(root)
    expected_inputs = {n: v for n, v in baseline.items() if n not in allowed}
    require(request["input_bindings"] == expected_inputs, "dependent implementation input bindings mismatch")
    ready, _, _ = gate.validate_report(root, request["readiness_report"], request["readiness_increment"])
    closure.reviews(ready, full=False)
    require(ready["next_increment_readiness"] in {"Ready", "Ready with advisories"}, "dependent readiness blocked")
    record = {"request": request, "raw_request": raw_request.decode(), "raw_state": raw_state.decode(),
              "raw_report": raw_report.decode(), "raw_decision": read(root, "DECISIONS.md").decode(),
              "baseline_manifest": baseline}
    data = encode(record)
    admission = digest(data)
    exclusive(root, STORE / (admission + ".json"), data)
    require(gate.workspace_fingerprint(root) == request["workspace_fingerprint"]
            and closure.read_bytes(gate.state_path(root)) == raw_state, "dependent admission race")
    exclusive(root, POINTER, encode({"admission": admission}))
    gate.write_state(root, {"schema_version": STATE_SCHEMA, "status": "active", "increment_id": SUCCESSOR,
        "baseline_fingerprint": request["workspace_fingerprint"], "dependent_lineage": {"admission": admission}})


@closure.guarded
def record(root: Path, state: dict[str, Any], *, scope: bool = True) -> dict[str, Any]:
    shape(state)
    admission = state["dependent_lineage"]["admission"]
    pointer = parse(read(root, POINTER.as_posix()))
    keys(pointer, {"admission"})
    require(pointer["admission"] == admission, "dependent consumed identity mismatch")
    data = read(root, (STORE / (admission + ".json")).as_posix())
    require(digest(data) == admission, "dependent sealed admission tampered")
    result = parse(data)
    keys(result, {"request", "raw_request", "raw_state", "raw_report", "raw_decision", "baseline_manifest"})
    req = result["request"]
    require(state["baseline_fingerprint"] == req["workspace_fingerprint"], "dependent baseline binding mismatch")
    require(req["index_sha256"] == digest(closure.read_bytes(closure.git_index_path(root))), "dependent index drift")
    require(parse(result["raw_request"].encode()) == req, "dependent raw request mismatch")
    old = parse(result["raw_state"].encode())
    gate.validate_state(old)
    require(old["status"] == "failed" and old["quality_gate"] == "FAIL" and old["increment_id"] == PREDECESSOR,
            "dependent failure history mismatch")
    require(digest(result["raw_state"].encode()) == req["predecessor"]["state"]["sha256"], "dependent history hash mismatch")
    closure.validate_lineage(root, old, scope=False)
    require(digest(result["raw_report"].encode()) == old["report_sha256"], "dependent failed report binding mismatch")
    require(reference(root, req["predecessor"]["report"]) == result["raw_report"].encode(), "dependent failed report drift")
    require(req["root"] == str(root.resolve()) and req["branch"] == branch(root), "dependent checkout drift")
    require(gate._is_ancestor(root, req["head"], gate.current_head_commit(root)), "dependent HEAD lineage drift")
    if state["status"] == "active": require(req["head"] == gate.current_head_commit(root), "dependent active HEAD drift")
    for auth in [*req["owner_authorization"].values(), req["acceptance_change"]["owner_authorization"]]:
        require(auth["approved"] is True and auth["decision_sha256"] == digest(result["raw_decision"].encode()),
                "dependent authority history mismatch")
    for ref in req["evidence"].values(): reference(root, ref)
    snapshot_copies(root, req)
    after = closure.manifest(root)
    for name, value in req["input_bindings"].items():
        require(after.get(name) == value, "dependent protected implementation drift")
    if scope:
        delta = closure.validate_milestone_delta({"request": req, "baseline_manifest": result["baseline_manifest"]}, after)
        require(delta.issubset(req["allowed_paths"]), "dependent scope drift")
    return result


def carry(source: dict[str, Any], target: dict[str, Any]) -> None:
    if "dependent_lineage" in source:
        target["schema_version"] = STATE_SCHEMA
        target["dependent_lineage"] = source["dependent_lineage"]


@closure.guarded
def milestone_report(root: Path, increment: str, report: dict[str, Any]) -> bool:
    state = gate.read_state(root)
    require(state is not None and increment == SUCCESSOR, "dependent report identity mismatch")
    rec = record(root, state)
    req = rec["request"]
    original = gate._extract_manifest(rec["raw_report"])
    for kind, key in [("verification", "command"), ("manual_verification", "check")]:
        for prior in original[kind]:
            if not prior["required"]: continue
            name = prior[key]
            if kind == "verification" and name.endswith("cortexa-dual-label-final-preservation.py"):
                name = PRESERVATION_COMMAND
            rows = [row for row in report[kind] if row[key] == name]
            require(len(rows) == 1 and rows[0]["required"] is True, "dependent required check omitted")
    evidence = report.get("milestone")
    keys(evidence, {"criteria", "paths"})
    keys(evidence["criteria"], set(req["milestone"]["acceptance"]))
    blocked = False
    for value in evidence["criteria"].values():
        keys(value, {"status", "evidence"})
        require(value["status"] in {"automatically_verified", "live_verified", "implemented", "deferred", "blocked"}
                and isinstance(value["evidence"], str) and bool(value["evidence"].strip()), "dependent criterion evidence missing")
        blocked |= value["status"] not in {"automatically_verified", "live_verified"}
    delta = closure.changed(rec["baseline_manifest"], closure.manifest(root))
    keys(evidence["paths"], delta)
    for value in evidence["paths"].values():
        keys(value, {"criterion", "rationale", "within_objective", "preserves_existing"})
        require(value["criterion"] in evidence["criteria"] and value["within_objective"] is True
                and value["preserves_existing"] is True and isinstance(value["rationale"], str)
                and bool(value["rationale"].strip()), "dependent path attribution missing")
    return blocked
