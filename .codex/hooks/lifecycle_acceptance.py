"""Evidence-bound, documentation-only acceptance; immutable terminal history.

Local approval records are workflow evidence, not authentication. This module
never executes evidence commands, launches applications or grants publication.
"""
from __future__ import annotations

import functools
import hashlib
import json
import os
import re
import stat
import tempfile
from pathlib import Path
from typing import Any

from common import GateError

MAINTENANCE = "evidence-bound-acceptance-maintenance"
ROOT_DOCS = frozenset({"CHANGELOG.md", "HANDOFF.md", "NEXT_STEPS.md", "PLANS.md",
                       "PROJECT_STATUS.md", "TESTING_GUIDE.md", "TROUBLESHOOTING_LOG.md"})
MAINTENANCE_FILES = ROOT_DOCS | frozenset({
    ".codex/hooks/post_increment_gate.py", ".codex/hooks/lifecycle_acceptance.py",
    ".codex/hooks/tests/test_post_increment_gate.py",
    ".codex/hooks/tests/test_lifecycle_acceptance.py", "AGENTS.md", "ENGINEERING_GUIDE.md",
    ".agents/skills/post-increment-gate/SKILL.md", ".agents/skills/verified-increment/SKILL.md",
    "DECISIONS.md"})
REQUIRED_COMMANDS = frozenset({"npm run verify", "npm run docs:check", "npm run repository:check",
    "npm run security:scan", "git diff --check", "python3 -B .codex/hooks/session_end_gate.py"})
REQUIRED_REVIEWS = frozenset({"Architecture review", "Security review", "Code-health review",
    "Preservation and scope review", "Evidence-binding and readiness review"})
MAX_BYTES = 32 * 1024 * 1024
STORE = ".codex/state/acceptance"
POINTER = STORE + "/maintenance.json"


def gate():
    import post_increment_gate
    return post_increment_gate


def require(value: Any, message: str) -> None:
    if not value:
        raise GateError(message)


def guarded(fn):
    @functools.wraps(fn)
    def run(*args, **kwargs):
        try:
            return fn(*args, **kwargs)
        except GateError:
            raise
        except (OSError, ValueError, TypeError, KeyError, UnicodeError, RecursionError):
            raise GateError("invalid or unreadable acceptance evidence") from None
    return run


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def encode(value: Any) -> bytes:
    return (json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False) + "\n").encode()


def keys(value: Any, expected: set[str]) -> None:
    require(isinstance(value, dict) and set(value) == expected, "acceptance fields must be exact")


def hash_value(value: Any) -> str:
    require(isinstance(value, str) and re.fullmatch(r"[0-9a-f]{64}", value), "invalid acceptance digest")
    return value


def relative(name: str) -> Path:
    require(isinstance(name, str) and name and "\\" not in name, "invalid acceptance path")
    p = Path(name)
    require(not p.is_absolute() and p.as_posix() == name and all(v not in {".", ".."} for v in p.parts),
            "unsafe acceptance path")
    return p


def safe(root: Path, name: str) -> Path:
    p = relative(name)
    current = root
    for part in p.parts:
        current = current / part
        require(not current.is_symlink(), "acceptance symlink forbidden")
    return current


def read(root: Path, name: str) -> bytes:
    p = safe(root, name)
    require(p.is_file() and p.stat().st_size <= MAX_BYTES, "missing or oversized acceptance evidence")
    data = p.read_bytes()
    require(len(data) <= MAX_BYTES, "oversized acceptance evidence")
    return data


def parse(data: bytes) -> Any:
    def unique(items):
        result = {}
        for k, v in items:
            require(k not in result, "duplicate acceptance JSON key")
            result[k] = v
        return result
    return json.loads(data, object_pairs_hook=unique)


def blob(root: Path, identity: str) -> bytes:
    identity = hash_value(identity)
    data = read(root, STORE + "/blobs/" + identity)
    require(digest(data) == identity, "acceptance blob mismatch")
    return data


def atomic(root: Path, name: str, data: bytes) -> None:
    p = safe(root, name)
    p.parent.mkdir(parents=True, exist_ok=True)
    safe(root, name)
    if p.exists():
        require(read(root, name) == data, "changed acceptance replay")
        return
    fd, temporary = tempfile.mkstemp(prefix=".pending-", dir=p.parent)
    try:
        with os.fdopen(fd, "wb") as f:
            f.write(data)
            f.flush()
            os.fsync(f.fileno())
        # Hard-link creation is atomic and cannot overwrite a concurrent record.
        try:
            os.link(temporary, p)
        except FileExistsError:
            require(read(root, name) == data, "concurrent acceptance drift")
    finally:
        Path(temporary).unlink(missing_ok=True)


def normalize(value: Any) -> dict[str, Any]:
    require(isinstance(value, dict) and bool(value), "empty acceptance inventory")
    for name, row in value.items():
        relative(name)
        require(not name.startswith((".git/", ".codex/state/")), "internal path in inventory")
        keys(row, {"kind", "mode", "sha256"})
        require(row["kind"] in {"file", "symlink"}, "unsupported inventory kind")
        require(type(row["mode"]) is int and row["mode"] & ~0o111 == 0, "invalid inventory mode")
        hash_value(row["sha256"])
    return value


def inventory(root: Path) -> dict[str, Any]:
    g = gate()
    names = g._decode_git_paths(g._run_git(root, "ls-files", "--cached", "--others", "--exclude-standard", "-z"))
    result = {}
    for name in sorted(set(names)):
        p = root / name
        try:
            info = p.lstat()
        except FileNotFoundError:
            continue
        if stat.S_ISLNK(info.st_mode):
            content = os.readlink(p).encode()
            kind = "symlink"
        else:
            require(stat.S_ISREG(info.st_mode), "unsupported workspace file")
            content = p.read_bytes()
            kind = "file"
        result[name] = {"kind": kind, "mode": info.st_mode & 0o111, "sha256": digest(content)}
    return normalize(result)


def fingerprint(value: Any) -> str:
    value = normalize(value)
    h = hashlib.sha256()
    for name, row in sorted(value.items()):
        encoded = name.encode()
        h.update(len(encoded).to_bytes(8, "big")); h.update(encoded)
        h.update(row["mode"].to_bytes(2, "big")); h.update(row["kind"].encode())
        h.update(bytes.fromhex(row["sha256"]))
    return h.hexdigest()


def changed(before, after) -> set[str]:
    return {name for name in set(before) | set(after) if before.get(name) != after.get(name)}


def issue_ids(manifest: dict[str, Any]) -> set[str]:
    issues = set()
    for kind in ("verification", "manual_verification"):
        for row in manifest[kind]:
            if row["required"] and row["status"] != "Passed":
                issues.add(kind + ":" + digest(encode(row)))
    for row in manifest["findings"]:
        if row["blocks_completion"] or row["blocks_next_increment"]:
            issues.add("finding:" + digest(encode(row)))
    return issues


def history(root: Path, items: Any) -> tuple[dict[str, Any], set[str]]:
    require(isinstance(items, list) and bool(items) and len(items) <= 16, "invalid failure history")
    result = {}; issues = set()
    for item in items:
        keys(item, {"state", "report", "manifest"})
        raw = blob(root, item["state"]); state = parse(raw)
        gate().validate_state(state)
        require(state["status"] == "failed" and not any(k in state for k in
                ("completion_marker", "successor_disposition", "predecessor_disposition", "acceptance_lineage")),
                "history must retain terminal failure")
        require(item["state"] not in result, "duplicate failure history")
        require(item["report"] == state["report_sha256"], "historical report binding mismatch")
        report = blob(root, item["report"])
        require(read(root, state["report_path"]) == report, "historical report was rewritten")
        manifest = parse(blob(root, item["manifest"]))
        require(fingerprint(manifest) == state["workspace_fingerprint"], "historical fingerprint mismatch")
        require(manifest[state["report_path"]]["sha256"] == item["report"], "historical report inventory mismatch")
        report_manifest = gate()._extract_manifest(report.decode())
        require(report_manifest["increment_id"] == state["increment_id"] and
                report_manifest["quality_gate"] == "FAIL" and
                report_manifest["next_increment_readiness"] == state["next_increment_readiness"],
                "historical terminal fields mismatch")
        result[item["state"]] = {"state": state, "manifest": manifest, "raw": raw}
        issues |= {item["state"] + "/" + issue for issue in issue_ids(report_manifest)}
    return result, issues


def authorize(root: Path, value: Any, purpose: str, decision: str | None = None) -> None:
    keys(value, {"approved", "purpose", "source", "decision_sha256"})
    require(value["approved"] is True and value["purpose"] == purpose, "separate owner authorization required")
    require(isinstance(value["source"], str) and 0 < len(value["source"]) <= 4096, "owner instruction source missing")
    expected = digest(read(root, "DECISIONS.md")) if decision is None else decision
    require(hash_value(value["decision_sha256"]) == expected, "owner decision binding drift")


def review_checks(m, full: bool) -> None:
    commands = REQUIRED_COMMANDS if full else REQUIRED_COMMANDS - {"npm run verify"}
    for command in commands:
        require(any(e == {"command": command, "required": True, "status": "Passed"}
                    for e in m["verification"]), "required acceptance verification missing")
    for check in REQUIRED_REVIEWS:
        require(any(e == {"check": check, "required": True, "status": "Passed"}
                    for e in m["manual_verification"]), "required acceptance review missing")


def document_scope(increment: str, plan: str, report: str) -> set[str]:
    gate().validate_increment_id(increment)
    require(re.fullmatch(r"docs/plans/\d{4}-\d{2}-\d{2}-" + re.escape(increment) + r"\.md", plan), "invalid acceptance plan")
    require(re.fullmatch(r"docs/reviews/\d{4}-\d{2}-\d{2}-" + re.escape(increment) + r"-post-increment-review\.md", report), "invalid acceptance report")
    return set(ROOT_DOCS) | {plan, report}



def artifact(root: Path, binding: Any, catalog: Any, product: dict[str, Any], head: str | None = None) -> None:
    keys(binding, {"source_manifest", "artifact_sha256", "artifact_receipt", "transfer_receipt", "evidence"})
    require(isinstance(binding["evidence"], list) and binding["evidence"] and
            set(binding["evidence"]).issubset(catalog.values()), "artifact evidence missing")
    require(binding["artifact_receipt"] in binding["evidence"], "artifact receipt not catalogued")
    receipt = parse(blob(root, binding["artifact_receipt"]))
    require(isinstance(receipt, dict), "invalid artifact receipt")
    require(hash_value(binding["artifact_sha256"]) == receipt.get("executable_sha256"),
            "artifact executable binding mismatch")
    require(binding["source_manifest"] == product, "artifact candidate binding incomplete")
    bundle = Path(receipt["bundle"])
    prefix = root / "src-tauri/target"
    require(bundle.is_absolute() and bundle.suffix == ".app", "invalid artifact bundle")
    bundle.relative_to(prefix)
    # Read only hash-bound packaging files inside the task's existing bundle.
    files = receipt["files"]
    require(isinstance(files, dict) and 0 < len(files) <= 256, "invalid artifact file inventory")
    require(receipt["executable"] == str(bundle / "Contents/MacOS/ai-agent-assistant") and
            files.get("Contents/MacOS/ai-agent-assistant") == binding["artifact_sha256"],
            "artifact executable identity mismatch")
    for name, identity in files.items():
        hash_value(identity)
        path = safe(root, (bundle / relative(name)).relative_to(root).as_posix())
        require(path.is_file(), "missing artifact representation")
        h = hashlib.sha256()
        with path.open("rb") as handle:
            for block in iter(lambda: handle.read(1024 * 1024), b""):
                h.update(block)
        require(h.hexdigest() == identity, "artifact bytes changed")
    sources = receipt["source_files"]
    require(isinstance(sources, dict) and bool(sources), "artifact source inventory missing")
    for name, identity in sources.items():
        relative(name); hash_value(identity)
        if name in product:
            require(product[name]["sha256"] == identity, "artifact source mismatch")
    required = {n for n in product if n.startswith(("src/", "src-tauri/src/", "assets/", "public/"))}
    # The original pre-build transfer receipt also binds source-only artwork and
    # README files deliberately absent from the runtime artifact's source list.
    # Never synthesize those hashes or modify either historical receipt.
    require(binding["transfer_receipt"] in binding["evidence"], "transfer evidence missing")
    transfer = parse(blob(root, binding["transfer_receipt"]))
    require(transfer["destination"] == str(root.resolve()) and
            transfer["head"] == (head or gate().current_head_commit(root)), "transfer identity mismatch")
    retained = transfer["all_source_files"]
    require(isinstance(retained, dict), "transfer source inventory missing")
    for name in required - set(sources):
        require(name.startswith("assets/") and retained.get(name) == product[name]["sha256"],
                "artifact source inventory incomplete")


@guarded
def seal(root: Path, request_name: str) -> str:
    require(request_name.startswith(".codex/state/"), "maintenance request must be local gate evidence")
    request_raw = read(root, request_name); request = parse(request_raw)
    keys(request, {"schema_version", "increment_id", "root", "head", "index_sha256", "history",
                   "current_failure", "before_manifest", "allowed_paths", "plan", "report",
                   "owner_authorization", "evidence", "artifact_binding", "successor"})
    require(type(request["schema_version"]) is int and request["schema_version"] == 1 and
            request["increment_id"] == MAINTENANCE, "invalid maintenance identity")
    require(request["root"] == str(root.resolve()) and request["head"] == gate().current_head_commit(root), "maintenance checkout drift")
    state_raw = read(root, ".codex/state/post_increment_gate.json")
    require(digest(state_raw) == request["current_failure"], "maintenance failed state drift")
    records, issues = history(root, request["history"])
    require(request["current_failure"] in records, "current failure not archived")
    before = parse(blob(root, request["before_manifest"]))
    require(before == records[request["current_failure"]]["manifest"], "maintenance baseline mismatch")
    expected = set(MAINTENANCE_FILES) | (document_scope(MAINTENANCE, request["plan"], request["report"]) - ROOT_DOCS)
    require(isinstance(request["allowed_paths"], list) and len(request["allowed_paths"]) == 18 and
            set(request["allowed_paths"]) == expected, "maintenance scope must be exactly eighteen paths")
    current = inventory(root)
    require(changed(before, current) == expected, "maintenance scope drift")
    for name in expected & set(before):
        if name.endswith(".md"):
            old = blob(root, before[name]["sha256"])
            require(read(root, name).count(old) == 1, "historical document body changed")
    index_name = gate()._run_git(root, "rev-parse", "--git-path", "index").decode().strip()
    index = Path(index_name)
    if not index.is_absolute(): index = root / index
    require(digest(index.read_bytes()) == request["index_sha256"], "Git index drift")
    authorize(root, request["owner_authorization"], "maintenance-only")
    evidence = request["evidence"]
    require(isinstance(evidence, dict) and bool(evidence), "missing inherited evidence")
    for name, identity in evidence.items():
        require(isinstance(name, str) and re.fullmatch(r"[a-z0-9-]{1,80}", name), "invalid evidence label")
        blob(root, identity)
    binding = parse(blob(root, request["artifact_binding"]))
    product = {n: v for n, v in before.items() if n not in expected}
    artifact(root, binding, evidence, product, request["head"])
    successor = request["successor"]
    keys(successor, {"increment_id", "plan", "report", "allowed_paths"})
    scope = document_scope(successor["increment_id"], successor["plan"], successor["report"])
    require(successor["increment_id"] not in {v["state"]["increment_id"] for v in records.values()} | {MAINTENANCE}, "failed identity cannot be reused")
    require(len(successor["allowed_paths"]) == 9 and set(successor["allowed_paths"]) == scope, "successor must be documentation-only")
    require(not any(n in before for n in scope - ROOT_DOCS), "successor report or plan already exists")
    m, _, report_hash = gate().validate_report(root, request["report"], MAINTENANCE)
    review_checks(m, True)
    require(not gate().has_merge_conflicts(root) and not gate().suspicious_changed_paths(gate().changed_paths(root)), "unsafe maintenance workspace")
    record = {"schema_version": 1, "request": request, "request_sha256": digest(encode(request)),
              "workspace_manifest": current, "report_sha256": report_hash, "issues": sorted(issues)}
    data = encode(record); identity = digest(data)
    require(inventory(root) == current and read(root, ".codex/state/post_increment_gate.json") == state_raw, "maintenance changed before sealing")
    for name in ROOT_DOCS:
        body = read(root, name)
        atomic(root, STORE + "/blobs/" + digest(body), body)
    atomic(root, STORE + "/blobs/" + identity, data)
    atomic(root, POINTER, encode({"receipt": identity}))
    return identity


@guarded
def maintenance(root: Path, current: bool = True) -> dict[str, Any] | None:
    p = safe(root, POINTER)
    if not p.exists(): return None
    pointer = parse(read(root, POINTER));keys(pointer, {"receipt"})
    rec = parse(blob(root, pointer["receipt"]))
    keys(rec, {"schema_version", "request", "request_sha256", "workspace_manifest", "report_sha256", "issues"})
    require(type(rec["schema_version"]) is int and rec["schema_version"] == 1, "invalid maintenance receipt schema")
    req = rec["request"]
    keys(req, {"schema_version", "increment_id", "root", "head", "index_sha256", "history",
               "current_failure", "before_manifest", "allowed_paths", "plan", "report",
               "owner_authorization", "evidence", "artifact_binding", "successor"})
    require(digest(encode(req)) == rec["request_sha256"], "maintenance request binding mismatch")
    require(type(req["schema_version"]) is int and req["schema_version"] == 1 and req["increment_id"] == MAINTENANCE, "maintenance identity mismatch")
    records, issues = history(root, req["history"])
    require(sorted(issues) == rec["issues"] and req["current_failure"] in records, "maintenance history mismatch")
    require(req["root"] == str(root.resolve()), "receipt belongs to another checkout")
    require(digest(read(root, req["report"])) == rec["report_sha256"], "maintenance report changed")
    m = gate()._extract_manifest(read(root, req["report"]).decode());review_checks(m, True)
    require(m["quality_gate"] in {"PASS", "PASS WITH ADVISORIES"}, "maintenance is not passing")
    for identity in req["evidence"].values(): blob(root, identity)
    binding = parse(blob(root, req["artifact_binding"]))
    before = parse(blob(root, req["before_manifest"]))
    require(before == records[req["current_failure"]]["manifest"], "archived baseline drift")
    expected = set(MAINTENANCE_FILES) | (document_scope(MAINTENANCE, req["plan"], req["report"]) - ROOT_DOCS)
    require(len(req["allowed_paths"]) == 18 and set(req["allowed_paths"]) == expected and
            changed(before, rec["workspace_manifest"]) == expected, "sealed maintenance scope drift")
    successor = req["successor"]
    keys(successor, {"increment_id", "plan", "report", "allowed_paths"})
    allowed = document_scope(successor["increment_id"], successor["plan"], successor["report"])
    require(len(successor["allowed_paths"]) == 9 and set(successor["allowed_paths"]) == allowed,
            "sealed successor scope drift")
    require(successor["increment_id"] not in {v["state"]["increment_id"] for v in records.values()} | {MAINTENANCE}, "sealed successor identity reused")
    for name in expected & set(before):
        if name.endswith(".md"):
            require(read(root, name).count(blob(root, before[name]["sha256"])) == 1, "historical body drift")
    artifact(root, binding, req["evidence"], {n:v for n,v in before.items() if n not in expected}, req["head"])
    authorize(root, req["owner_authorization"], "maintenance-only", rec["workspace_manifest"]["DECISIONS.md"]["sha256"])
    require(not gate().has_merge_conflicts(root) and not gate().suspicious_changed_paths(gate().changed_paths(root)), "unsafe sealed workspace")
    if current:
        require(inventory(root) == rec["workspace_manifest"], "sealed maintenance workspace drift")
        require(digest(read(root, ".codex/state/post_increment_gate.json")) == req["current_failure"], "sealed failure state drift")
        require(gate().current_head_commit(root) == req["head"], "maintenance HEAD drift")
    rec["identity"] = pointer["receipt"]
    return rec


@guarded
def shape(state):
    require(not any(k in state for k in ("predecessor_disposition", "successor_disposition")), "mixed acceptance lineage forbidden")
    keys(state["acceptance_lineage"], {"maintenance", "admission"})
    for value in state["acceptance_lineage"].values(): hash_value(value)


@guarded
def validate_lineage(root: Path, state: dict[str, Any]) -> None:
    shape(state);rec = maintenance(root, False)
    require(rec is not None and rec["identity"] == state["acceptance_lineage"]["maintenance"], "missing acceptance history")
    admission = parse(blob(root, state["acceptance_lineage"]["admission"]))
    keys(admission, {"request", "baseline_manifest", "maintenance"})
    require(admission["maintenance"] == rec["identity"], "admission receipt mismatch")
    req = admission["request"];successor = rec["request"]["successor"]
    keys(req, {"schema_version", "increment_id", "maintenance", "workspace_fingerprint", "owner_authorization", "resolutions"})
    require(type(req["schema_version"]) is int and req["schema_version"] == 1 and req["maintenance"] == rec["identity"] and
            req["workspace_fingerprint"] == fingerprint(admission["baseline_manifest"]), "admission request mismatch")
    require(req["increment_id"] == state["increment_id"] == successor["increment_id"], "acceptance identity drift")
    require(admission["baseline_manifest"] == rec["workspace_manifest"], "admission baseline mismatch")
    require(state["baseline_fingerprint"] == fingerprint(admission["baseline_manifest"]), "active baseline mismatch")
    authorize(root, req["owner_authorization"], "acceptance-only", admission["baseline_manifest"]["DECISIONS.md"]["sha256"])
    resolutions(root, rec, req["resolutions"])
    if state["status"] != "complete":
        require(gate().current_head_commit(root) == rec["request"]["head"], "acceptance HEAD drift")
    now = inventory(root);before = admission["baseline_manifest"]
    require(changed(before, now).issubset(set(successor["allowed_paths"])), "acceptance scope drift")
    for name in ROOT_DOCS:
        # Seal includes the source bytes of root documents, allowing additive updates only.
        old = blob(root, before[name]["sha256"])
        require(read(root, name).count(old) == 1, "acceptance rewrote document history")
    require(not gate().has_merge_conflicts(root), "acceptance conflicts")
    if state["status"] != "active":
        require(state["report_path"] == successor["report"], "acceptance report identity drift")


def resolutions(root, rec, value):
    require(isinstance(value, dict) and set(value) == set(rec["issues"]), "complete blocker resolution required")
    catalog = rec["request"]["evidence"]
    for resolution in value.values():
        keys(resolution, {"status", "evidence", "rationale", "reviewed_by_owner"})
        require(resolution["status"] == "resolved" and resolution["reviewed_by_owner"] is True, "unresolved blocker")
        require(isinstance(resolution["rationale"], str) and 0 < len(resolution["rationale"]) <= 4096, "resolution rationale missing")
        refs = resolution["evidence"]
        require(isinstance(refs, list) and bool(refs) and len(refs)==len(set(refs)) and set(refs).issubset(catalog), "resolution evidence missing")
        for name in refs: blob(root, catalog[name])


@guarded
def begin(root: Path, increment: str, request_name: str) -> None:
    require(request_name.startswith(".codex/state/"), "admission request must be local gate evidence")
    req = parse(read(root, request_name))
    state = gate().read_state(root)
    if state and state["status"] == "active" and "acceptance_lineage" in state:
        validate_lineage(root, state)
        prior = parse(blob(root, state["acceptance_lineage"]["admission"]))
        require(state["increment_id"] == increment and prior["request"] == req and
                inventory(root) == prior["baseline_manifest"], "changed admission replay")
        return
    rec = maintenance(root);require(rec is not None, "passing maintenance seal required")
    keys(req, {"schema_version", "increment_id", "maintenance", "workspace_fingerprint", "owner_authorization", "resolutions"})
    require(type(req["schema_version"]) is int and req["schema_version"] == 1, "unsupported admission schema")
    require(req["increment_id"] == increment == rec["request"]["successor"]["increment_id"], "wrong acceptance successor")
    require(req["maintenance"] == rec["identity"], "wrong maintenance receipt")
    authorize(root, req["owner_authorization"], "acceptance-only")
    resolutions(root, rec, req["resolutions"])
    require(req["workspace_fingerprint"] == gate().workspace_fingerprint(root), "admission fingerprint drift")
    record = {"request": req, "baseline_manifest": rec["workspace_manifest"], "maintenance": rec["identity"]}
    data = encode(record);identity = digest(data)
    atomic(root, STORE + "/blobs/" + identity, data)
    # Recheck after the only preparatory write; interruption leaves failure untouched.
    maintenance(root)
    gate().write_state(root, {"schema_version": 4, "status": "active", "increment_id": increment,
        "baseline_fingerprint": req["workspace_fingerprint"],
        "acceptance_lineage": {"maintenance": rec["identity"], "admission": identity}})
