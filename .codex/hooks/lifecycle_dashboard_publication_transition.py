"""Fixed dashboard publication custody; never a generic completion override.

Receipts are same-user integrity evidence, not operator authentication. This
module never stages, commits, merges, invokes a provider or rewrites live state.
"""
from __future__ import annotations

import hashlib
import json
import os
from pathlib import Path
import stat
import struct
from typing import Any

import lifecycle_closure as c
import lifecycle_dashboard_publication as original
import lifecycle_dashboard_verification_repair as repair
import post_increment_gate as gate

SNAPSHOT_SHA256 = "276cbe7f7add156ca3e1ff463a25031d13475ecbbd288d813d8365fe9c6b9623"
STATE_SHA256 = "b62cc29f55bcd6f7bc53e660dcf71ee0cd9a39be61fddcd3fd8b365df4b11968"
PUBLICATION_SHA256 = "f66d6d91bacf8092b3d3d8aad49b5d750167d0ce34bcc90979818012573d2f01"
# The issued manifest was written in insertion order. Its raw identity above
# must not be compared with the shared sorted-key encoding of the same object.
PUBLICATION_CANONICAL_SHA256 = "dc5f8fbc84f0f583055e677e739cbdee44c7277bd4b4ba09da6057c0f56b9a5e"
STORE = Path(".codex/state/dashboard-publication-transition")
BASE = STORE / "accepted.json"
RECEIPT = STORE / "publication.json"
REQUEST = STORE / "request.json"
MAINTENANCE = "dashboard-publication-transition-maintenance"
PLAN = "docs/plans/2026-10-21-dashboard-publication-transition.md"
REVIEW_ID = "dashboard-publication-transition-readiness"
REVIEW = "docs/reviews/2026-10-21-" + REVIEW_ID + "-post-increment-review.md"
ASSESSMENT = "docs/reviews/2026-10-21-" + MAINTENANCE + "-post-increment-review.md"
BOUNDARY_ASSESSMENT = "docs/reviews/2026-10-21-dashboard-publication-boundary-repair-assessment.md"
FROZEN_REPORTS = {
    ASSESSMENT: "a5f0e9eedcdb2c4b1e93acb70efc27a271b45ddb131ec1f6c0c49b4a6cb4d504",
    "docs/reviews/2026-10-21-dashboard-historical-formatting-recovery-assessment.md": "27c56f58da85a812200518a7cfbc55b33183cd66179ece657a7552bbe8ed9769",
    "docs/reviews/2026-10-21-dashboard-transition-binding-repair-assessment.md": "2c98c881447f3dd4616790351d88936a995f423816b5d061baa95593b4cc9a0a",
    "docs/reviews/2026-10-21-dashboard-transition-custody-accounting-assessment.md": "2889da58e51b64561963d083d930c9b6c5830d10d2d897ca4754cfdd53bbe78b",
    "docs/reviews/2026-10-21-dashboard-transition-diagnostic-recovery-assessment.md": "7b968685f97d13ac1529434b5df3aaa14fd9fa4680d25b144f6eefdafe86c503",
    "docs/reviews/2026-10-21-dashboard-transition-fixture-repair-assessment.md": "110af773a5b5c706338ff7f72b0b79bedc44b7d1565ba2987bd8cea198b5f617",
    "docs/reviews/2026-10-21-dashboard-transition-tracked-fixture-repair-assessment.md": "ca1cf75b20dee6ee30881144f69a8903adadaf2bf02aa1e08dd0ecbf13117c82",
}
IGNORE_BASE_SHA256 = "ae7d8c4fcc8a705484d6608f9a2ca4f6b9baf1fc2d5340409b92e872f66af43c"
IGNORE_SUFFIX = (
    "\n# Owner-authorized exact historical formatting exceptions; pinned raw hashes remain required.\n"
    + "\n".join([ASSESSMENT,
        "docs/reviews/2026-10-21-dashboard-transition-binding-repair-assessment.md",
        "docs/reviews/2026-10-21-dashboard-transition-custody-accounting-assessment.md",
        "docs/reviews/2026-10-21-dashboard-transition-fixture-repair-assessment.md"])
    + "\n"
).encode()
CODE = {".codex/hooks/lifecycle_dashboard_publication_transition.py",
        ".codex/hooks/tests/test_lifecycle_dashboard_publication_transition.py",
        ".codex/hooks/post_increment_gate.py"}
DOCS = {"AGENTS.md", "ENGINEERING_GUIDE.md", "DECISIONS.md", "PLANS.md", "HANDOFF.md",
        "PROJECT_STATUS.md", "NEXT_STEPS.md", "CHANGELOG.md", "TROUBLESHOOTING_LOG.md",
        PLAN, REVIEW, ASSESSMENT}
# Keep historical assessments and formatting policy out of shared project memory.
GOVERNANCE_ADDITIONS = {".prettierignore", BOUNDARY_ASSESSMENT} | (set(FROZEN_REPORTS) - {ASSESSMENT})
ALLOWED = CODE | DOCS | GOVERNANCE_ADDITIONS
NODE = "/opt/homebrew/Cellar/node/26.3.0/bin/node"
NPM = "/opt/homebrew/lib/node_modules/npm/bin/npm-cli.js"
CHECKS = {
    "hooks": ["/usr/bin/python3", "-B", "-m", "unittest", "discover", "-s", ".codex/hooks/tests", "-p", "test_*.py"],
    **{k: [NODE, NPM, "run", v] for k, v in [("full", "verify"), ("docs", "docs:check"),
       ("repository", "repository:check"), ("security", "security:scan")]},
    "whitespace": ["git", "diff", "--check"],
    "session": ["/usr/bin/python3", "-B", ".codex/hooks/session_end_gate.py"],
}
GROUPS = {"dashboard_product", "dashboard_documentation", "shared_project_memory", "governance"}
MAX_BYTES = 4 * 1024 * 1024
require = c.require


def parse(raw: bytes) -> Any:
    def unique(pairs):
        result = {}
        for key, value in pairs:
            require(key not in result, "publication duplicate JSON key")
            result[key] = value
        return result
    def nonfinite(_):
        raise gate.GateError("publication nonfinite JSON")
    require(len(raw) <= MAX_BYTES, "publication capture bound")
    return json.loads(raw, object_pairs_hook=unique, parse_constant=nonfinite)


def read(path: Path) -> bytes:
    require(path.is_absolute() and not any(p.is_symlink() for p in (path, *path.parents)), "publication unsafe path")
    def identity(s):
        return (s.st_dev, s.st_ino, s.st_mode, s.st_uid, s.st_gid, s.st_nlink,
                s.st_size, s.st_mtime_ns, s.st_ctime_ns)
    before = path.stat()
    require(stat.S_ISREG(before.st_mode) and before.st_uid == os.getuid()
            and before.st_nlink == 1 and not before.st_mode & 0o022
            and before.st_size <= MAX_BYTES, "publication unsafe file")
    fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW)
    try:
        require(identity(before) == identity(os.fstat(fd)), "publication replaced file")
        with os.fdopen(fd, "rb", closefd=False) as stream:
            raw = stream.read(before.st_size + 1)
        require(len(raw) == before.st_size and identity(before) == identity(os.fstat(fd))
                == identity(path.stat()), "publication read race")
        return raw
    finally:
        os.close(fd)


def file(root: Path, name: str) -> Path:
    return c.relative(root, name)


def mode(path: Path) -> int:
    return stat.S_IMODE(path.stat().st_mode)


@c.guarded
def accepted(root: Path, state: dict[str, Any]) -> dict[str, Any]:
    require(c.digest(str(root.resolve()).encode()) == original.ROOT_SHA256
            and gate._run_git(root, "branch", "--show-current").decode().strip() == original.BRANCH,
            "publication checkout identity")
    raw = read(file(root, BASE.as_posix()))
    require(c.digest(raw) == SNAPSHOT_SHA256, "publication accepted snapshot drift")
    base = parse(raw)
    c.keys(base, {"schema", "root", "branch", "head", "raw_state", "index_hex", "manifest",
                  "protected_records", "publication_manifest", "publication_sha256", "retained_acceptance"})
    require(type(base["schema"]) is int and base["schema"] == 1 and base["root"] == str(root.resolve())
            and base["branch"] == original.BRANCH and base["head"] == original.HEAD,
            "publication snapshot identity")
    require(c.digest(base["raw_state"].encode()) == STATE_SHA256
            and read(gate.state_path(root)) == base["raw_state"].encode()
            and parse(base["raw_state"].encode()) == state and state["schema_version"] == 8
            and state["status"] == "complete" and state["increment_id"] == repair.SUCCESSOR,
            "publication accepted state drift")
    gate.validate_state(state)
    require(c.fingerprint(c.normalize_manifest(base["manifest"])) == state["workspace_fingerprint"]
            and c.digest(bytes.fromhex(base["index_hex"])) == repair.INDEX_SHA256,
            "publication accepted workspace/index")
    require(c.digest(read(file(root, state["report_path"]))) == state["report_sha256"], "publication accepted report drift")
    require(base["publication_sha256"] == PUBLICATION_SHA256
            and c.digest(c.encoded(base["publication_manifest"])) == PUBLICATION_CANONICAL_SHA256,
            "publication original manifest drift")
    for name, item in base["protected_records"].items():
        require(name.startswith(".codex/state/"), "publication historical namespace")
        path = file(root, name)
        require(c.digest(read(path)) == item["sha256"] and mode(path) == item["mode"], "publication historical record drift")
    return base


def index_semantics(raw: bytes) -> list[tuple[str, str, str]]:
    require(len(raw) >= 32 and raw[:4] == b"DIRC"
            and hashlib.sha1(raw[:-20]).digest() == raw[-20:], "publication index checksum")
    version, count = struct.unpack(">II", raw[4:12])
    require(version == 2 and count <= 10000, "publication index format")
    position = 12
    entries = []
    for _ in range(count):
        start = position
        require(position + 62 < len(raw) - 20, "publication truncated index")
        flags = struct.unpack(">H", raw[position+60:position+62])[0]
        require(flags & 0xf000 == 0 and flags < 0xfff, "publication index flags/stage")
        end = raw.find(b"\0", position + 62, len(raw)-20)
        require(end >= 0 and end-position-62 == flags, "publication index path length")
        name = raw[position+62:end].decode("utf-8")
        gate.validate_relative_path(name)
        file_mode = struct.unpack(">I", raw[position+24:position+28])[0]
        require(file_mode in {0o100644, 0o100755}, "publication index object mode")
        entries.append((name, format(file_mode, "o"), raw[position+40:position+60].hex()))
        position = start + ((end+1-start+7)//8)*8
        require(not any(raw[end:position]), "publication index padding")
    require(entries == sorted(entries) and len({n for n, _, _ in entries}) == len(entries), "publication index duplicate/order")
    while position < len(raw)-20:
        require(position+8 <= len(raw)-20 and raw[position:position+4] == b"TREE", "publication index extension")
        size = struct.unpack(">I", raw[position+4:position+8])[0]
        position += 8 + size
    require(position == len(raw)-20, "publication index bounds")
    return entries


def tree(root: Path, revision: str) -> list[tuple[str, str, str]]:
    entries = []
    for row in gate._run_git(root, "ls-tree", "-r", "-z", revision).split(b"\0"):
        if row:
            fields, name = row.split(b"\t", 1)
            file_mode, kind, oid = fields.decode().split()
            path = name.decode(); gate.validate_relative_path(path)
            require(kind == "blob" and file_mode in {"100644", "100755"}, "publication tree object")
            entries.append((path, file_mode, oid))
    return sorted(entries)


def expected_tree(root: Path, manifest: dict[str, Any]) -> list[tuple[str, str, str]]:
    result = []
    for name, item in sorted(manifest.items()):
        path = file(root, name); raw = read(path)
        require(item["exists"] and c.digest(raw) == item["sha256"] and mode(path) == item["mode"], "publication current file drift")
        result.append((name, "100755" if item["mode"] & 0o111 else "100644",
                       hashlib.sha1(b"blob " + str(len(raw)).encode() + b"\0" + raw).hexdigest()))
    return result


def check_ref(root: Path, ref: Any, *, decode: bool = True) -> Any:
    c.keys(ref, {"path", "sha256"})
    require(isinstance(ref["path"], str) and ref["path"].startswith((STORE / "evidence").as_posix()+"/"), "publication proof namespace")
    raw = read(file(root, ref["path"]))
    require(c.digest(raw) == c.sha(ref["sha256"]), "publication proof drift")
    return parse(raw) if decode else raw


def review(root: Path, request: dict[str, Any]) -> None:
    raw = read(file(root, REVIEW))
    require(c.digest(raw) == c.sha(request["review_sha256"]), "publication readiness drift")
    report = gate._extract_manifest(raw.decode())
    require(report["increment_id"] == REVIEW_ID and report["quality_gate"] in {"PASS", "PASS WITH ADVISORIES"}, "publication readiness nonpassing")
    for name in ["Preservation and scope review", "Architecture, security, code-health and readiness review"]:
        rows = [r for r in report["manual_verification"] if r["check"] == name]
        require(len(rows) == 1 and rows[0]["required"] is True and rows[0]["status"] == "Passed", "publication substantive review missing")
    require(not any(f["blocks_completion"] for f in report["findings"]), "publication blocking finding")


def maintenance_history(root: Path, base: dict[str, Any], manifest: dict[str, Any]) -> None:
    """Fixed issued history and exact formatting amendment, not write permission."""
    require(base["manifest"].get(".prettierignore") == {
        "exists": True, "mode": 0o644, "sha256": IGNORE_BASE_SHA256},
        "publication original ignore binding")
    raw = read(file(root, ".prettierignore"))
    require(raw.endswith(IGNORE_SUFFIX)
            and c.digest(raw[:-len(IGNORE_SUFFIX)]) == IGNORE_BASE_SHA256
            and manifest.get(".prettierignore") == {
                "exists": True, "mode": 0o644, "sha256": c.digest(raw)},
            "publication exact formatting amendment")
    for name, pin in FROZEN_REPORTS.items():
        require(manifest.get(name) == {"exists": True, "mode": 0o644, "sha256": pin},
                "publication immutable assessment drift")


@c.guarded
def contract(root: Path, request: dict[str, Any], base: dict[str, Any]) -> list[tuple[str, str, str]]:
    c.keys(request, {"schema", "increment_id", "accepted_sha256", "parent", "manifest", "owner_authorization",
                     "plan_sha256", "review_sha256", "checks", "publication_manifest"})
    require(type(request["schema"]) is int and request["schema"] == 1
            and request["increment_id"] == MAINTENANCE and request["accepted_sha256"] == SNAPSHOT_SHA256
            and request["parent"] == original.HEAD, "publication request identity/ancestry")
    c.authorization(root, request["owner_authorization"])
    require(c.digest(read(file(root, PLAN))) == c.sha(request["plan_sha256"]), "publication frozen plan drift")
    manifest = c.normalize_manifest(request["manifest"])
    require(manifest == request["manifest"] and c.manifest(root) == manifest, "publication candidate inventory")
    delta = c.changed(base["manifest"], manifest)
    require(delta <= ALLOWED and all(item["exists"] for item in manifest.values())
            and all(name in manifest for name in base["manifest"]), "publication unrelated/deleted scope")
    for name in delta:
        if name in base["manifest"]:
            require(manifest[name]["mode"] == base["manifest"][name]["mode"], "publication maintenance mode drift")
    maintenance_history(root, base, manifest)
    for name in CODE | {PLAN, REVIEW, ASSESSMENT}:
        require(name in manifest, "publication required maintenance file missing")
    publication = request["publication_manifest"]
    c.keys(publication, {"original_sha256", "groups"})
    require(publication["original_sha256"] == PUBLICATION_SHA256 and set(publication["groups"]) == GROUPS, "publication scope groups")
    old_groups = base["publication_manifest"]["groups"]
    seen = set()
    for group, items in publication["groups"].items():
        require(isinstance(items, dict), "publication scope shape")
        expected = {item["path"] for item in old_groups[group]}
        expected |= delta & (DOCS-{"AGENTS.md", "ENGINEERING_GUIDE.md", PLAN, REVIEW, ASSESSMENT} if group == "shared_project_memory" else set())
        expected |= delta - (DOCS-{"AGENTS.md", "ENGINEERING_GUIDE.md", PLAN, REVIEW, ASSESSMENT}) if group == "governance" else set()
        require(set(items) == expected and not seen & set(items), "publication scope omitted/duplicated")
        seen |= set(items)
        require(all(items[n] == manifest[n]["sha256"] for n in items), "publication scope content drift")
    require(seen == {row["path"] for rows in old_groups.values() for row in rows} | delta, "publication scope incomplete")
    review(root, request)
    c.keys(request["checks"], set(CHECKS))
    inputs = {n: v for n, v in manifest.items() if n != REVIEW}
    for label, ref in request["checks"].items():
        proof = check_ref(root, ref)
        c.keys(proof, {"label", "argv", "cwd", "input_bindings", "exit_status", "timed_out", "complete_capture", "cleanup", "elapsed_seconds", "log"})
        require(proof["label"] == label and proof["argv"] == CHECKS[label] and proof["cwd"] == str(root.resolve())
                and proof["input_bindings"] == inputs and type(proof["exit_status"]) is int and proof["exit_status"] == 0
                and proof["timed_out"] is False and proof["complete_capture"] is True
                and proof["cleanup"] == {"root_reaped": True, "group_absent": True}
                and type(proof["elapsed_seconds"]) in {int, float}
                and 0 <= proof["elapsed_seconds"] <= (1800 if label == "full" else 600), "publication check incomplete/stale/failed")
        check_ref(root, proof["log"], decode=False)
    return expected_tree(root, manifest)


def present(root: Path) -> bool:
    return os.path.lexists(root / STORE)


@c.guarded
def validate(root: Path, state: dict[str, Any]) -> dict[str, Any]:
    base = accepted(root, state)
    raw = read(file(root, RECEIPT.as_posix()))
    receipt = parse(raw); c.keys(receipt, {"request_sha256", "request"})
    request_raw = read(file(root, REQUEST.as_posix()))
    require(c.digest(request_raw) == receipt["request_sha256"] and parse(request_raw) == receipt["request"], "publication request drift")
    expected = contract(root, receipt["request"], base)
    current = gate.current_head_commit(root)
    index_path = c.git_index_path(root); raw_index = read(index_path)
    staged = index_semantics(raw_index)
    if current == original.HEAD:
        require(staged == index_semantics(bytes.fromhex(base["index_hex"])) or staged == expected, "publication partial/unrelated staging")
        phase = "staged" if staged == expected else "prepared"
    else:
        parents = gate._run_git(root, "show", "-s", "--format=%P", current).decode().strip().split()
        require(parents == [original.HEAD] and staged == expected and tree(root, current) == expected, "publication unsupported commit/tree/parents")
        phase = "committed"
    require(read(index_path) == raw_index and gate.current_head_commit(root) == current, "publication index/HEAD race")
    return {"historical_acceptance_valid": True, "current_publication_valid": True,
            "phase": phase, "head": current, "upstream_reconciliation_supported": False}


@c.guarded
def seal(root: Path, request_path: str) -> None:
    require(request_path == REQUEST.as_posix() and not os.path.lexists(root / RECEIPT), "publication replay/request identity")
    state = gate.read_state(root)
    require(state is not None, "publication state missing")
    base = accepted(root, state)
    request_raw = read(file(root, REQUEST.as_posix())); request = parse(request_raw)
    contract(root, request, base)
    index_path = c.git_index_path(root); before = read(index_path)
    require(before == bytes.fromhex(base["index_hex"]) and gate.current_head_commit(root) == original.HEAD, "publication seal requires original unstaged index/HEAD")
    require(read(index_path) == before and read(file(root, REQUEST.as_posix())) == request_raw, "publication pre-effect drift")
    original.exclusive(root, RECEIPT, c.encoded({"request_sha256": c.digest(request_raw), "request": request}))
    validate(root, state)  # A post-effect failure leaves consumed evidence, never retries.
