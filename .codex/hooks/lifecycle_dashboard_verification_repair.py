"""Fixed verification successor; local custody is not operator authentication."""
from __future__ import annotations

import json
from pathlib import Path
from typing import Any

import lifecycle_closure as closure
import lifecycle_dashboard_verification as previous
import post_increment_gate as gate

PREDECESSOR = previous.SUCCESSOR
SUCCESSOR = "usage-dashboard-publication-verification-repair"
MAINTENANCE = "dashboard-verification-repair-maintenance"
CONTINUATION = MAINTENANCE
FAILED_STATE = "f614fdbfb01e439944b3c5bb96bcd1d58720c1c223bff38b3f6d73971bb9c716"
FAILED_REPORT = "05fc126d56e37393a58ea2bc424d9317277e25056af93a1816270e246b21daae"
PREVIOUS_ADMISSION = "212addfdf38134545013ea3038ca67f08798c1444398389c8148b977bfdd0474"
INDEX_SHA256 = "671155c7cc122cc6221bb63cc2f180edc147a4d07a2afd0c2b8cb9a03f0de4ef"
IGNORE_SHA256 = "ae7d8c4fcc8a705484d6608f9a2ca4f6b9baf1fc2d5340409b92e872f66af43c"
STATE_SCHEMA = 8
LINEAGE = "dashboard_verification_repair_lineage"
STORE = Path(".codex/state/dashboard-verification-repair")
POINTER = STORE / "consumed.json"
PLAN = "docs/plans/2026-10-21-" + SUCCESSOR + ".md"
REPORT = "docs/reviews/2026-10-21-" + SUCCESSOR + "-post-increment-review.md"
READINESS_ID = SUCCESSOR + "-readiness"
READINESS = "docs/reviews/2026-10-21-" + READINESS_ID + "-post-increment-review.md"
MAINTENANCE_REPORT = "docs/reviews/2026-10-21-" + CONTINUATION + "-post-increment-review.md"
EXECUTION_PATHS = {"ARCHITECTURE.md", "CHANGELOG.md", "HANDOFF.md", "NEXT_STEPS.md",
                   "PLANS.md", "PROJECT_STATUS.md", "TROUBLESHOOTING_LOG.md", REPORT}
EVIDENCE_START = "<!-- dashboard-verification-repair-evidence\n"
EVIDENCE_END = "\n/dashboard-verification-repair-evidence -->"
require, keys, digest, encode = closure.require, closure.keys, closure.digest, closure.encoded
read, exclusive = previous.read, previous.exclusive


def parse(data: bytes) -> Any:
    require(len(data) <= closure.MAX_BYTES, "verification JSON exceeds bound")
    def unique(pairs):
        result = {}
        for key, value in pairs:
            require(key not in result, "verification duplicate JSON key")
            result[key] = value
        return result
    def invalid(_):
        raise gate.GateError("verification nonfinite JSON")
    return json.loads(data, object_pairs_hook=unique, parse_constant=invalid)


@closure.guarded
def shape(state: dict[str, Any]) -> None:
    require(type(state.get("schema_version")) is int and state["schema_version"] == STATE_SCHEMA
            and state.get("increment_id") == SUCCESSOR, "verification state identity")
    require(not any(k in state for k in ("closure_lineage", "acceptance_lineage", "dependent_lineage",
                "dashboard_publication_lineage", "dashboard_verification_lineage", "predecessor_disposition", "successor_disposition")),
            "verification mixed lineage")
    keys(state[LINEAGE], {"admission"})
    closure.sha(state[LINEAGE]["admission"])


def unresolved_entries(report: dict[str, Any]) -> dict[str, Any]:
    result = {}
    for kind in ("verification", "manual_verification"):
        for row in report[kind]:
            if row["required"] and row["status"] != "Passed":
                result[kind + ":" + digest(encode(row))] = row
    for row in report["findings"]:
        if row["blocks_completion"] or row["blocks_next_increment"]:
            result["finding:" + digest(encode(row))] = row
    for name, row in report["milestone"]["criteria"].items():
        if row["status"] not in {"automatically_verified", "live_verified"}:
            value = {"criterion": name, **row}
            result["criterion:" + digest(encode(value))] = value
    return result


def history(root: Path) -> dict[str, Any]:
    """Require V's own closure and archived authority; never reauthorize history."""
    own = closure.historical(root, FAILED_STATE)
    state = parse(own["raw_state"].encode())
    previous.shape(state)
    require(state["increment_id"] == PREDECESSOR and state["report_sha256"] == FAILED_REPORT
            and state[previous.LINEAGE]["admission"] == PREVIOUS_ADMISSION,
            "verification repair historical failure binding")
    require(own["request"]["verification"] == {"increment_id": MAINTENANCE,
            "report_path": MAINTENANCE_REPORT}, "verification repair own maintenance closure required")
    raw = read(root, (previous.STORE / (PREVIOUS_ADMISSION + ".json")).as_posix())
    require(digest(raw) == PREVIOUS_ADMISSION, "verification repair schema-7 receipt drift")
    rec = parse(raw)
    keys(rec, {"request", "raw_request", "raw_state", "raw_decision", "baseline_manifest"})
    pointer_raw = read(root, previous.POINTER.as_posix())
    keys(parse(pointer_raw), {"admission"})
    require(parse(pointer_raw)["admission"] == PREVIOUS_ADMISSION, "verification repair consumed drift")
    req = rec["request"]
    prior = previous.history(root)
    require(parse(rec["raw_request"].encode()) == req
            and digest(rec["raw_state"].encode()) == previous.FAILED_STATE
            and rec["raw_state"] == prior["own"]["raw_state"]
            and req["closure_id"] == previous.FAILED_STATE
            and req["increment_id"] == PREDECESSOR,
            "verification repair archived ancestry drift")
    keys(req["owner_authorization"], {"admission", "execution"})
    for auth in req["owner_authorization"].values():
        keys(auth, {"approved", "source", "decision_path", "decision_sha256"})
        require(auth["approved"] is True and auth["decision_path"] == "DECISIONS.md"
                and bool(auth["source"].strip())
                and auth["decision_sha256"] == digest(rec["raw_decision"].encode()),
                "verification repair archived authority drift")
    baseline = closure.normalize_manifest(rec["baseline_manifest"])
    require(baseline["DECISIONS.md"]["sha256"] == digest(rec["raw_decision"].encode())
            and closure.fingerprint(baseline) == req["workspace_fingerprint"]
            == state["baseline_fingerprint"], "verification repair archived baseline drift")
    require(req["historical_bindings"] == prior["bindings"]
            and req["original_obligations"] == prior["previous"]["request"]["obligations"]
            and req["recovery_obligations"] == prior["unresolved"],
            "verification repair archived obligations drift")
    require(digest(read(root, previous.READINESS)) == req["readiness_report_sha256"]
            and digest(read(root, previous.MAINTENANCE_REPORT)) == req["maintenance_report_sha256"],
            "verification repair historical preparation drift")
    ignore_raw = read(root, ".prettierignore")
    require(digest(ignore_raw) == IGNORE_SHA256, "verification repair formatting policy drift")
    ignore = ignore_raw.decode().splitlines()
    require(ignore.count(previous.REPORT) == 1, "verification repair exact formatting exception missing")
    failed = gate._extract_manifest(own["raw_report"])
    return {"own": own, "prior": prior, "reports": prior["reports"] + [failed],
            "unresolved": previous.unresolved_entries(failed),
            "bindings": {"v_closure": digest(closure.read_bytes(closure.receipt_path(root, FAILED_STATE))),
                         "schema7_admission": digest(raw), "schema7_pointer": digest(pointer_raw),
                         "schema7_decision": digest(rec["raw_decision"].encode()), **prior["bindings"]}}


def contract(root: Path, req: dict[str, Any], old: dict[str, Any]) -> None:
    keys(req, {"schema_version", "increment_id", "root", "branch", "head", "index_sha256",
               "workspace_fingerprint", "failed_state_sha256", "failed_report_sha256", "closure_id",
               "maintenance_report", "maintenance_report_sha256", "readiness_report", "readiness_report_sha256",
               "readiness_increment", "owner_authorization", "plan", "report", "milestone", "allowed_paths",
               "protected_source_bindings", "original_obligations", "recovery_obligations", "v_obligations", "historical_bindings"})
    require(type(req["schema_version"]) is int and req["schema_version"] == 1
            and req["increment_id"] == SUCCESSOR and req["closure_id"] == FAILED_STATE
            and req["failed_state_sha256"] == FAILED_STATE and req["failed_report_sha256"] == FAILED_REPORT,
            "verification request identity")
    previous.previous.identity(root, req)
    require(req["index_sha256"] == INDEX_SHA256, "verification frozen index required")
    require(req["plan"] == PLAN and req["report"] == REPORT and req["readiness_report"] == READINESS
            and req["readiness_increment"] == READINESS_ID and req["maintenance_report"] == MAINTENANCE_REPORT,
            "verification preparation identity")
    require(digest(read(root, READINESS)) == req["readiness_report_sha256"]
            and digest(read(root, MAINTENANCE_REPORT)) == req["maintenance_report_sha256"],
            "verification frozen review drift")
    prior = old["prior"]["older"]["original_contract"]
    closure.milestone_contract(req["milestone"])
    require(req["milestone"]["acceptance"] == prior["acceptance"]
            and req["milestone"]["exclusions"] == prior["exclusions"]
            and req["milestone"]["authorized_destructive_paths"] == [], "verification criterion waiver")
    require(set(prior["protected_paths"]).issubset(req["milestone"]["protected_paths"]),
            "verification original protection omitted")
    allowed = set(closure.path_list(req["allowed_paths"]))
    require(REPORT in allowed and allowed <= EXECUTION_PATHS, "verification execution scope")
    require(all(not closure.protected_match(p, req["milestone"]["protected_paths"]) for p in allowed),
            "verification protected scope")
    require(req["protected_source_bindings"] == previous.previous.PRODUCT
            and all(digest(read(root, p)) == sha for p, sha in previous.previous.PRODUCT.items()),
            "verification product drift")
    require(req["original_obligations"] == old["prior"]["previous"]["request"]["obligations"]
            and set(req["original_obligations"]) == previous.previous.OBLIGATIONS,
            "verification original obligations changed")
    require(req["recovery_obligations"] == old["prior"]["unresolved"] and req["v_obligations"] == old["unresolved"], "verification recovery obligations changed")
    require(req["historical_bindings"] == old["bindings"], "verification historical binding drift")
    keys(req["owner_authorization"], {"admission", "execution"})
    for auth in req["owner_authorization"].values():
        closure.authorization(root, auth)
    read(root, PLAN)


@closure.guarded
def begin(root: Path, increment: str, path: str) -> None:
    require(increment == SUCCESSOR, "verification successor is task-bound")
    store = root / STORE
    require(not store.is_symlink() and (not store.exists() or not any(store.iterdir())),
            "verification replay or interrupted admission")
    raw_state = closure.read_bytes(gate.state_path(root))
    require(digest(raw_state) == FAILED_STATE, "verification exact failed state required")
    closure.reject_d133_context(root, gate.read_state(root))
    old = history(root)
    raw_request = read(root, path)
    req = parse(raw_request)
    contract(root, req, old)
    require(req["workspace_fingerprint"] == gate.workspace_fingerprint(root), "verification input drift")
    ready, _, _ = gate.validate_report(root, READINESS, READINESS_ID)
    closure.reviews(ready, full=False)
    require(ready["next_increment_readiness"] in {"Ready", "Ready with advisories"}, "verification readiness blocked")
    baseline = closure.manifest(root)
    require(closure.fingerprint(baseline) == req["workspace_fingerprint"], "verification snapshot drift")
    rec = {"request": req, "raw_request": raw_request.decode(), "raw_state": raw_state.decode(),
           "raw_decision": read(root, "DECISIONS.md").decode(), "baseline_manifest": baseline}
    data = encode(rec)
    require(len(data) <= closure.MAX_BYTES, "verification receipt exceeds bound")

    def unchanged():
        contract(root, req, history(root))
        require(read(root, path) == raw_request and closure.read_bytes(gate.state_path(root)) == raw_state
                and gate.workspace_fingerprint(root) == req["workspace_fingerprint"], "verification admission race")

    unchanged()
    admission = digest(data)
    exclusive(root, STORE / (admission + ".json"), data)
    unchanged()
    exclusive(root, POINTER, encode({"admission": admission}))
    unchanged()
    gate.write_state(root, {"schema_version": STATE_SCHEMA, "status": "active", "increment_id": SUCCESSOR,
        "baseline_fingerprint": req["workspace_fingerprint"], LINEAGE: {"admission": admission}})


@closure.guarded
def record(root: Path, state: dict[str, Any]) -> dict[str, Any]:
    shape(state)
    admission = state[LINEAGE]["admission"]
    pointer = parse(read(root, POINTER.as_posix()))
    keys(pointer, {"admission"})
    require(pointer["admission"] == admission, "verification consumed identity")
    data = read(root, (STORE / (admission + ".json")).as_posix())
    require(digest(data) == admission, "verification receipt drift")
    rec = parse(data)
    keys(rec, {"request", "raw_request", "raw_state", "raw_decision", "baseline_manifest"})
    req = rec["request"]
    require(parse(rec["raw_request"].encode()) == req and digest(rec["raw_state"].encode()) == FAILED_STATE,
            "verification raw record drift")
    contract(root, req, history(root))
    require(read(root, "DECISIONS.md") == rec["raw_decision"].encode(), "verification authority drift")
    require(state["baseline_fingerprint"] == req["workspace_fingerprint"]
            == closure.fingerprint(closure.normalize_manifest(rec["baseline_manifest"])), "verification baseline drift")
    delta = closure.validate_milestone_delta(rec, closure.manifest(root))
    require(delta <= set(req["allowed_paths"]), "verification unauthorized execution change")
    require(all((root / p).is_file() for p in delta), "verification deletion forbidden")
    return rec


def carry(source: dict[str, Any], target: dict[str, Any]) -> None:
    if LINEAGE in source:
        target["schema_version"] = STATE_SCHEMA
        target[LINEAGE] = source[LINEAGE]


def required_checks(old: dict[str, Any]) -> dict[str, set[str]]:
    checks = previous.required_checks(old["prior"])
    for report in old["reports"]:
        for kind, key in [("verification", "command"), ("manual_verification", "check")]:
            checks[kind] |= {row[key] for row in report[kind] if row["required"]}
    checks["manual_verification"] |= {"V repair evidence " + n for n, row in old["unresolved"].items()
        if not (n.startswith("finding:") and row["blocks_completion"] is False)}
    return checks


def evidence(root: Path, report: dict[str, Any], checks: dict[str, set[str]]) -> None:
    """Check custody and declared reuse compatibility; semantic review remains required."""
    text = read(root, REPORT).decode()
    require(text.count(EVIDENCE_START) == text.count(EVIDENCE_END) == 1, "verification provenance missing")
    values = parse(text.split(EVIDENCE_START)[1].split(EVIDENCE_END)[0].encode())
    keys(values, {"verification", "manual_verification"})
    current = closure.manifest(root)
    # Evidence excludes the report containing its digest to avoid a circular seal.
    current.pop(REPORT, None)
    for kind, key in [("verification", "command"), ("manual_verification", "check")]:
        keys(values[kind], checks[kind])
        for name in checks[kind]:
            rows = [r for r in report[kind] if r[key] == name]
            require(len(rows) == 1 and rows[0]["required"] is True, "verification required evidence omitted or downgraded")
            item = values[kind][name]
            keys(item, {"path", "sha256", "reuse", "review"})
            require(type(item["reuse"]) is bool and isinstance(item["review"], str) and bool(item["review"].strip()),
                    "verification reuse review missing")
            require(item["path"].startswith(".codex/state/dashboard-verification-repair-evidence/"),
                    "verification evidence location")
            raw = read(root, item["path"])
            require(digest(raw) == closure.sha(item["sha256"]), "verification provenance drift")
            proof = parse(raw)
            keys(proof, {"name", "status", "input_bindings", "environment", "result", "scope_review"})
            require(proof["name"] == name and proof["status"] == rows[0]["status"]
                    and isinstance(proof["environment"], str) and bool(proof["environment"].strip())
                    and isinstance(proof["result"], str) and bool(proof["result"].strip())
                    and isinstance(proof["scope_review"], str) and bool(proof["scope_review"].strip()),
                    "verification unsupported evidence reuse")
            bound = closure.normalize_manifest(proof["input_bindings"])
            require(all(current.get(p) == entry for p, entry in bound.items()), "verification evidence input drift")
            # Whole-workspace checks must not reuse a selective product-only binding.
            whole = name in closure.REQUIRED_COMMANDS or name in closure.REQUIRED_REVIEWS
            require(not whole or bound == current, "verification incomplete whole-workspace provenance")
            require(whole or set(previous.previous.PRODUCT) <= set(bound), "verification incomplete product provenance")


@closure.guarded
def milestone_report(root: Path, increment: str, report: dict[str, Any]) -> bool:
    state = gate.read_state(root)
    require(state is not None and increment == SUCCESSOR, "verification report identity")
    rec = record(root, state)
    old = history(root)
    checks = required_checks(old)
    evidence(root, report, checks)
    carried = [r for r in old["reports"][2]["findings"] if not r["blocks_completion"] and r["blocks_next_increment"]]
    require(bool(carried) and all(report["findings"].count(r) == 1 for r in carried)
            and report["next_increment_readiness"] == "Blocked", "verification post-commit blocker omitted or resolved")
    ev = report.get("milestone")
    keys(ev, {"criteria", "paths"})
    keys(ev["criteria"], set(rec["request"]["milestone"]["acceptance"]))
    blocked = False
    for value in ev["criteria"].values():
        keys(value, {"status", "evidence"})
        require(value["status"] in {"automatically_verified", "live_verified", "implemented", "deferred", "blocked"}
                and isinstance(value["evidence"], str) and bool(value["evidence"].strip()), "verification missing criterion")
        blocked |= value["status"] not in {"automatically_verified", "live_verified"}
    delta = closure.changed(rec["baseline_manifest"], closure.manifest(root))
    keys(ev["paths"], delta)
    for value in ev["paths"].values():
        keys(value, {"criterion", "rationale", "within_objective", "preserves_existing"})
        require(value["criterion"] in ev["criteria"] and value["within_objective"] is True
                and value["preserves_existing"] is True and isinstance(value["rationale"], str)
                and bool(value["rationale"].strip()), "verification path attribution")
    return blocked

