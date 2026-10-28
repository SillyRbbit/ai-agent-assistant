"""One closed dashboard recovery contract, not a generic dependent override.

Receipts provide local integrity, not authentication against the same OS user.
Maintenance does not authorize closure, admission, product transfer or publication.
"""
from __future__ import annotations

import json
import os
from pathlib import Path
from typing import Any

import lifecycle_closure as closure
import post_increment_gate as gate

PREDECESSOR = "usage-dashboard-publication-preparation"
SUCCESSOR = "usage-dashboard-publication-recovery"
MAINTENANCE = "dashboard-dependent-recovery-maintenance"
BRANCH = "codex/usage-dashboard-publication"
HEAD = "faa9f936b78a600c5d5f0dc2d587032fb256d5bf"
ROOT_SHA256 = "e303f38273964ac94132b4cf8922d31e14619e0cd44a7a44a8090ef3c24271c6"
FAILED_STATE = "f0b0f7821ec9747f842e44e7bea8d43e19eacf7fbd4f51823258919e8eca0f30"
FAILED_REPORT = "a17ba967b0800691004ab7ac4787a42652f3e0db2c2d71be52331f6ea5a72453"
STATE_SCHEMA = 6
STORE = Path(".codex/state/dashboard-publication")
POINTER = STORE / "consumed.json"
PLAN = "docs/plans/2026-10-20-usage-dashboard-publication-recovery.md"
REPORT = "docs/reviews/2026-10-20-usage-dashboard-publication-recovery-post-increment-review.md"
READINESS_ID = SUCCESSOR + "-readiness"
READINESS = "docs/reviews/2026-10-20-" + READINESS_ID + "-post-increment-review.md"
PRODUCT = {
    "scripts/repository_health.py": "04ac52b230499b41b168b6ec255749fbd3ff4ca2871be332d498cdc7acf85620",
    "scripts/tests/test_repository_health.py": "210607a57ac16c0b4e2c046bd8656230e101e0a24d422342eebf325a020f1dff",
    "src-tauri/src/lib.rs": "4bc613fac53c3b16e210e4628c445e366c3ad6ea74c9c6c4813cdee570750874",
    "src/features/activity/ActivityPage.tsx": "c10ebc70c29dd5d3f04859a41b9e93895fb77ff4da3dbe0b88c0565180f13924",
    "src-tauri/src/usage_efficiency.rs": "c41210356e264f8cccf3bd0ae4ba7502a6eda99c94325866c0b32337c7f7a01e",
    "src-tauri/tests/usage_efficiency_contract.rs": "150a9a7429e364b9a2b7f37700d122b548942614892ba95d73f6a4e0a91fe928",
    "src/features/usage-efficiency/AccountCapacityPanel.test.tsx": "325797183d1d1f7c2422d07541089116c28aa93308f3d3740b6267368a47d772",
    "src/features/usage-efficiency/AccountCapacityPanel.tsx": "5a3b094d7fa8992a108c9c92dabd6326bccba7b4825bb243f0c0cf0c06264db5",
    "src/features/usage-efficiency/UsageCharts.tsx": "4f72bc3e9f7d984b420bc9fc055af6a3b9390a1a99f76a89165c15bf3f10a73b",
    "src/features/usage-efficiency/UsageEfficiencyPanel.test.tsx": "ec847472f244778a77ab694e9fa469edc7a0253444e396d9a42e923a69b546d0",
    "src/features/usage-efficiency/UsageEfficiencyPanel.tsx": "380c40997d34950d1fd675fa19c97a3219e8677eb93aca93fdcb1edffc01b0f5",
    "src/features/usage-efficiency/UsageSummary.test.tsx": "a063ed6ef3765c96d9485ff8b8e2d89f04af4e31ff8dc7f2366305c91cbe2d41",
    "src/features/usage-efficiency/UsageSummary.tsx": "f4fce6d8739149161ff7620816e67a655083ceebd15046ddcf6862afc93f440c",
    "src/features/usage-efficiency/account-capacity.css": "7b21c9ed44fe3f2c70feaa46ca7267b91be59ee04ce50c806af015754c6954de",
    "src/features/usage-efficiency/usage-disclosure.css": "1cebccbcbf801bb5a745d1cacb2d16ed94d6bbe164415b2b259bd1cc2936ab25",
    "src/features/usage-efficiency/usage-efficiency.css": "443da9fac221ee00632f594f9f5861d908f9cddfdbbfc0979fcd88eef607b774",
    "src/features/usage-efficiency/usage-presentation.test.tsx": "a2ad009bdc68d7b9216347690ebe4bdb56b66f2b5f07d779337bf187c98cf5e0",
    "src/features/usage-efficiency/usage-presentation.ts": "834c3a4a81072ec2b0a19307e8c47dd0b906545e9f279c7e8e580d71967149a8",
    "src/infrastructure/tauri/usage-efficiency-client.test.ts": "4b10b4bf7c949472f09ff217102d717f9a3f4bd0bb34fd92d42b67f8aca0a6aa",
    "src/infrastructure/tauri/usage-efficiency-client.ts": "7ef8d26e8f26570356fddbfa0a84c0f74266a5d5041880fa121516ccc242cf09",
    "src/test/usage-efficiency-fixture.ts": "08b6cf2d73f9a9520875baefb7bc9e27df357a0818c9a27a3814e52920191ccd",
}
OBLIGATIONS = {
    "verification:0fc5830d8e309f9adbbe61d5d5bc58e926d7fa4253630f6031e2406bc80c6402",
    "verification:25443f49d2840b269ab9eac2f3bd604168d7ef383720cabff442647d9831609c",
    "manual_verification:269d4f1492fe6aabe54f2623a13f1e15a133c1f4dc8064c7a39a7cc7c1924051",
    "manual_verification:edb1bab56b1df986fb835bfbaa1f0f9e2556bba534a1003128c8d9e8f36064f2",
    "finding:b02378a33efef9b2e59d44a2ad1e7d7630444a0ba2377020cff1f30e68a23294",
    "finding:d25997b7730916cacede6f572a099319311c93e5c5185869c6316bfa124aedc7",
    "criterion:ad1d0ad67db4929dcd989e1c554751d1986d2471b7c4087d451b97b99d03f43b",
    "criterion:d0c0ccd7cf8a953e88b4a2ed4eb8fb16677567b093ef11ee3ddd5460f1239f80",
    "criterion:ece9d26eff7a86a01299a660f499146e64dca28879c1d4b1b3e27704e9858226",
}
DOCS = {"ARCHITECTURE.md", "CHANGELOG.md", "HANDOFF.md", "NEXT_STEPS.md", "PLANS.md",
        "PROJECT_STATUS.md", "TROUBLESHOOTING_LOG.md", "docs/USAGE_EFFICIENCY_DEMO.md",
        "docs/plans/2026-10-20-capacity-disclosure-label-repair.md"}
DOCS |= {"docs/reviews/2026-10-20-" + name + "-post-increment-review.md"
         for name in ("account-capacity-preview", "account-capacity-preview-readiness",
                      "capacity-disclosure-label-repair", "capacity-disclosure-label-repair-readiness",
                      "usage-dashboard-hierarchy", "usage-dashboard-hierarchy-approved-readiness",
                      "usage-dashboard-hierarchy-readiness", "usage-efficiency-demo",
                      "usage-efficiency-demo-readiness", "usage-efficiency-presentation",
                      "usage-efficiency-presentation-readiness", "usage-visual-polish",
                      "usage-visual-polish-readiness")}
require, keys, digest, encode = closure.require, closure.keys, closure.digest, closure.encoded


def parse(data: bytes) -> Any:
    def unique(pairs):
        result = {}
        for key, value in pairs:
            require(key not in result, "dashboard duplicate JSON key")
            result[key] = value
        return result
    return json.loads(data, object_pairs_hook=unique)


def read(root: Path, name: str) -> bytes:
    return closure.read_bytes(closure.relative(root, name))


def identity(root: Path, req: dict[str, Any]) -> None:
    require(digest(str(root.resolve()).encode()) == ROOT_SHA256
            and req["root"] == str(root.resolve()) and req["branch"] == BRANCH
            and gate._run_git(root, "branch", "--show-current").decode().strip() == BRANCH,
            "dashboard checkout binding mismatch")
    require(req["head"] == HEAD == gate.current_head_commit(root), "dashboard HEAD drift")
    require(req["index_sha256"] == digest(closure.read_bytes(closure.git_index_path(root))),
            "dashboard index drift")


@closure.guarded
def shape(state: dict[str, Any]) -> None:
    require(type(state.get("schema_version")) is int and state.get("schema_version") == STATE_SCHEMA
            and state.get("increment_id") == SUCCESSOR,
            "dashboard state identity mismatch")
    require(not any(k in state for k in ("closure_lineage", "acceptance_lineage", "dependent_lineage",
                "predecessor_disposition", "successor_disposition")), "dashboard mixed lineage")
    keys(state["dashboard_publication_lineage"], {"admission"})
    closure.sha(state["dashboard_publication_lineage"]["admission"])


def history(root: Path) -> dict[str, Any]:
    old = closure.historical(root, FAILED_STATE)
    state = parse(old["raw_state"].encode())
    require(state["increment_id"] == PREDECESSOR and state["report_sha256"] == FAILED_REPORT,
            "dashboard original failure mismatch")
    closure.validate_lineage(root, state, scope=False)
    require(old["request"]["verification"]["increment_id"] == MAINTENANCE,
            "dashboard passing maintenance closure required")
    require(set(closure.unresolved(old["raw_report"])) == OBLIGATIONS
            and set(old["request"]["disposition"]["unresolved"]) == OBLIGATIONS,
            "dashboard original obligations changed")
    prior = closure.read_json(root / closure.DIRECTORY /
                              ("admission-" + state["closure_lineage"]["admission"] + ".json"))
    require(prior["request"]["schema_version"] == 2, "dashboard original schema-2 binding required")
    old["original_contract"] = prior["request"]["milestone"]
    return old


def contract(root: Path, req: dict[str, Any], old: dict[str, Any]) -> None:
    keys(req, {"schema_version", "increment_id", "root", "branch", "head", "workspace_fingerprint",
               "index_sha256", "closure_id", "owner_authorization", "milestone", "allowed_paths",
               "plan", "report", "readiness_report", "readiness_increment", "obligations"})
    require(type(req["schema_version"]) is int and req["schema_version"] == 1
            and req["increment_id"] == SUCCESSOR and req["closure_id"] == FAILED_STATE,
            "dashboard request identity mismatch")
    identity(root, req)
    require(req["plan"] == PLAN and req["report"] == REPORT
            and req["readiness_report"] == READINESS and req["readiness_increment"] == READINESS_ID,
            "dashboard fresh plan/report/readiness identity required")
    closure.milestone_contract(req["milestone"])
    prior = old["original_contract"]
    require(req["milestone"]["acceptance"] == prior["acceptance"]
            and req["milestone"]["exclusions"] == prior["exclusions"]
            and req["milestone"]["authorized_destructive_paths"] == [],
            "dashboard acceptance/exclusion change or destruction forbidden")
    require(set(prior["protected_paths"]).issubset(req["milestone"]["protected_paths"]),
            "dashboard original protection omitted")
    allowed = set(closure.path_list(req["allowed_paths"]))
    require(set(PRODUCT) | {REPORT} <= allowed <= set(PRODUCT) | DOCS | {PLAN, REPORT},
            "dashboard scope not attributed")
    keys(req["obligations"], OBLIGATIONS)
    for value in req["obligations"].values():
        keys(value, {"criterion", "required"})
        require(value["required"] is True and value["criterion"] in prior["acceptance"],
                "dashboard obligation omitted or waived")
    # Fixed integrity boundaries also apply when callers omit them from their list.
    protected = {READINESS, parse(old["raw_state"].encode())["report_path"],
                 old["request"]["verification"]["report_path"]}
    require(not allowed.intersection(protected), "dashboard protected evidence in scope")
    require(all(not closure.protected_match(p, req["milestone"]["protected_paths"]) for p in allowed),
            "dashboard scope conflicts with protection")
    read(root, PLAN)


def exclusive(root: Path, name: Path, data: bytes) -> None:
    target = closure.relative(root, name.as_posix())
    require(not any(p.is_symlink() for p in [target, *target.parents]), "dashboard unsafe store")
    target.parent.mkdir(parents=True, exist_ok=True)
    try:
        fd = os.open(target, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
        with os.fdopen(fd, "wb") as handle:
            handle.write(data)
            handle.flush()
            os.fsync(handle.fileno())
    except FileExistsError as error:
        raise gate.GateError("dashboard replay rejected") from error


@closure.guarded
def begin(root: Path, increment: str, path: str) -> None:
    require(increment == SUCCESSOR, "dashboard successor is task-bound")
    store = root / STORE
    require(not store.is_symlink() and (not store.exists() or not any(store.iterdir())),
            "dashboard replay or interrupted admission rejected")
    state = gate.read_state(root)
    closure.reject_d133_context(root, state)
    raw_state = closure.read_bytes(gate.state_path(root))
    require(digest(raw_state) == FAILED_STATE, "dashboard original terminal state required")
    old = history(root)
    raw_request = read(root, path)
    req = parse(raw_request)
    contract(root, req, old)
    keys(req["owner_authorization"], {"admission", "execution"})
    for auth in req["owner_authorization"].values(): closure.authorization(root, auth)
    require(req["workspace_fingerprint"] == gate.workspace_fingerprint(root), "dashboard readiness input drift")
    ready, _, _ = gate.validate_report(root, READINESS, READINESS_ID)
    closure.reviews(ready, full=False)
    require(ready["next_increment_readiness"] in {"Ready", "Ready with advisories"}, "dashboard readiness blocked")
    baseline = closure.manifest(root)
    require(closure.fingerprint(baseline) == req["workspace_fingerprint"], "dashboard snapshot drift")
    rec = {"request": req, "raw_request": raw_request.decode(), "raw_state": raw_state.decode(),
           "raw_decision": read(root, "DECISIONS.md").decode(), "baseline_manifest": baseline,
           "closure_sha256": digest(closure.read_bytes(closure.receipt_path(root, FAILED_STATE)))}
    data = encode(rec)
    require(len(data) <= closure.MAX_BYTES, "dashboard receipt exceeds capture bound")

    def unchanged():
        identity(root, req)
        for auth in req["owner_authorization"].values(): closure.authorization(root, auth)
        require(gate.workspace_fingerprint(root) == req["workspace_fingerprint"]
                and closure.read_bytes(gate.state_path(root)) == raw_state
                and digest(closure.read_bytes(closure.receipt_path(root, FAILED_STATE))) == rec["closure_sha256"],
                "dashboard admission race")

    unchanged()
    admission = digest(data)
    exclusive(root, STORE / (admission + ".json"), data)
    unchanged()
    exclusive(root, POINTER, encode({"admission": admission}))
    unchanged()
    gate.write_state(root, {"schema_version": STATE_SCHEMA, "status": "active", "increment_id": SUCCESSOR,
        "baseline_fingerprint": req["workspace_fingerprint"], "dashboard_publication_lineage": {"admission": admission}})


@closure.guarded
def record(root: Path, state: dict[str, Any]) -> dict[str, Any]:
    shape(state)
    admission = state["dashboard_publication_lineage"]["admission"]
    pointer = parse(read(root, POINTER.as_posix()))
    keys(pointer, {"admission"})
    require(pointer["admission"] == admission, "dashboard consumed identity mismatch")
    data = read(root, (STORE / (admission + ".json")).as_posix())
    require(digest(data) == admission, "dashboard sealed receipt drift")
    rec = parse(data)
    keys(rec, {"request", "raw_request", "raw_state", "raw_decision", "baseline_manifest", "closure_sha256"})
    req = rec["request"]
    require(parse(rec["raw_request"].encode()) == req and digest(rec["raw_state"].encode()) == FAILED_STATE,
            "dashboard raw evidence mismatch")
    old = history(root)
    contract(root, req, old)
    require(digest(closure.read_bytes(closure.receipt_path(root, FAILED_STATE))) == rec["closure_sha256"],
            "dashboard closure drift")
    require(state["baseline_fingerprint"] == req["workspace_fingerprint"]
            == closure.fingerprint(closure.normalize_manifest(rec["baseline_manifest"])), "dashboard baseline mismatch")
    keys(req["owner_authorization"], {"admission", "execution"})
    for auth in req["owner_authorization"].values():
        closure.authorization(root, auth)
        require(auth["decision_sha256"] == digest(rec["raw_decision"].encode()), "dashboard authority history drift")
    after = closure.manifest(root)
    delta = closure.validate_milestone_delta({"request": req, "baseline_manifest": rec["baseline_manifest"]}, after)
    require(delta.issubset(req["allowed_paths"]), "dashboard unauthorized scope drift")
    require(all(after.get(p, {}).get("exists") is True for p in delta), "dashboard deletion forbidden")
    return rec


def carry(source: dict[str, Any], target: dict[str, Any]) -> None:
    if "dashboard_publication_lineage" in source:
        target["schema_version"] = STATE_SCHEMA
        target["dashboard_publication_lineage"] = source["dashboard_publication_lineage"]


@closure.guarded
def milestone_report(root: Path, increment: str, report: dict[str, Any]) -> bool:
    state = gate.read_state(root)
    require(state is not None and increment == SUCCESSOR, "dashboard report identity mismatch")
    rec = record(root, state)
    req = rec["request"]
    original = gate._extract_manifest(history(root)["raw_report"])
    checks = {"verification": set(closure.REQUIRED_COMMANDS),
              "manual_verification": set(closure.REQUIRED_REVIEWS) | {"Recovery obligation " + n for n in OBLIGATIONS}}
    for kind, key in [("verification", "command"), ("manual_verification", "check")]:
        checks[kind] |= {row[key] for row in original[kind] if row["required"]}
        for name in checks[kind]:
            rows = [r for r in report[kind] if r[key] == name]
            require(len(rows) == 1 and rows[0]["required"] is True, "dashboard required evidence omitted")
    evidence = report.get("milestone")
    keys(evidence, {"criteria", "paths"})
    keys(evidence["criteria"], set(req["milestone"]["acceptance"]))
    blocked = False
    for value in evidence["criteria"].values():
        keys(value, {"status", "evidence"})
        require(value["status"] in {"automatically_verified", "live_verified", "implemented", "deferred", "blocked"}
                and isinstance(value["evidence"], str) and bool(value["evidence"].strip()), "dashboard criterion evidence missing")
        blocked |= value["status"] not in {"automatically_verified", "live_verified"}
    delta = closure.changed(rec["baseline_manifest"], closure.manifest(root))
    keys(evidence["paths"], delta)
    for value in evidence["paths"].values():
        keys(value, {"criterion", "rationale", "within_objective", "preserves_existing"})
        require(value["criterion"] in evidence["criteria"] and value["within_objective"] is True
                and value["preserves_existing"] is True and isinstance(value["rationale"], str)
                and bool(value["rationale"].strip()), "dashboard path attribution missing")
    if report["quality_gate"] != "FAIL":
        require(all(digest(read(root, p)) == sha for p, sha in PRODUCT.items()),
                "dashboard accepted product binding missing or changed")
    return blocked
