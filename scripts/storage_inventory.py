#!/usr/bin/env python3
"""On-demand local artifact observation. Never authorizes or performs removal."""
from __future__ import annotations

import argparse
from contextlib import contextmanager
from dataclasses import dataclass
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import stat
import sys
import time
import unicodedata
from typing import Any, Iterator, Sequence

MAX_INPUT = 1024 * 1024
MAX_CANDIDATES = 64
MAX_SCOPE_DEPTH = 128
MAX_SCOPE_PATHS = 4096
ScopeAnchor = tuple[tuple[int, int], tuple[str, ...]]
# macOS SDK sys/stat.h: SF_DATALESS. Some supported Python versions omit it.
DATALESS = 0x40000000
UNKNOWN = ["ownership_not_verified", "local_holds_require_review",
           "task_process_runner_consumers_unknown", "mount_registry_not_checked",
           "registered_worktrees_not_checked", "private_cloud_availability_not_checked",
           "source_archive_coverage_not_verified", "operation_authority_absent"]
PROTECTED = frozenset({".git", ".ssh", ".aws", ".gnupg", "node_modules",
                       "Application Support", "Keychains", "CloudStorage",
                       "OneDrive", "evidence", "PRIVATE-runtime-tmp-recovery"})
EVIDENCE_NAMES = frozenset({"MANIFEST.json", "VERIFIED.json", "FINAL-SEAL.json",
                            "HANDOFF.md", "CLEANUP-ALLOWLIST.json"})
SOURCE_SUFFIXES = frozenset({".rs", ".ts", ".tsx", ".py", ".sqlite", ".sqlite3",
                             ".db", ".key", ".pem", ".p12"})


class BoundaryError(Exception):
    """Fixed error category, with conservative paths kept out of error text."""

    def __init__(self, category: str):
        super().__init__(category)
        self.exclusions: list[str] = []
        self.scope_unknown = False


@dataclass(frozen=True)
class Limits:
    entries: int = 10_000
    hashed_bytes: int = 64 * 1024 * 1024
    depth: int = 64
    seconds: float = 15.0


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def encoded(value: Any) -> bytes:
    return json.dumps(value, sort_keys=True, ensure_ascii=True,
                      separators=(",", ":"), allow_nan=False).encode("utf-8")


def absolute(value: str) -> str:
    if (not isinstance(value, str) or not value.startswith("/") or "\0" in value
            or value != os.path.normpath(value) or value.startswith("//")
            or any(p in {".", ".."} for p in value.split("/"))):
        raise BoundaryError("invalid_absolute_path")
    return value


def folded(value: str) -> str:
    return unicodedata.normalize("NFC", value).casefold()


def scope_parts(path: str) -> tuple[str, ...]:
    absolute(path)
    parts = Path(path).parts[1:]
    if len(parts) > MAX_SCOPE_DEPTH:
        raise BoundaryError("scope_depth_exceeded")
    return tuple(folded(part) for part in parts)


def prefix_overlap(a: tuple[str, ...], b: tuple[str, ...]) -> bool:
    return a[:len(b)] == b or b[:len(a)] == a


def file_key(observed: os.stat_result) -> tuple[int, int]:
    return observed.st_dev, observed.st_ino


def open_directory(parent: int, name: str) -> int:
    before = os.stat(name, dir_fd=parent, follow_symlinks=False)
    if getattr(before, "st_flags", 0) & DATALESS:
        raise BoundaryError("dataless_ancestor_not_opened")
    fd = os.open(name, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW, dir_fd=parent)
    try:
        after = os.fstat(fd)
        if file_key(after) != file_key(before) or getattr(after, "st_flags", 0) & DATALESS:
            raise BoundaryError("scope_ancestor_identity_drift")
    except BaseException:
        os.close(fd)
        raise
    return fd


def path_scope(path: str) -> list[ScopeAnchor]:
    """Existing no-follow prefix identities plus conservative unresolved names."""
    folded_names = scope_parts(path)
    names = Path(path).parts[1:]
    fd = os.open("/", os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW)
    anchors: list[ScopeAnchor] = []
    try:
        anchors.append((file_key(os.fstat(fd)), folded_names))
        for index, name in enumerate(names):
            try:
                observed = os.stat(name, dir_fd=fd, follow_symlinks=False)
            except FileNotFoundError:
                break  # The unresolved suffix stays bound to its existing parent.
            if index == len(names) - 1:
                anchors.append((file_key(observed), ()))
                break
            new = open_directory(fd, name)
            os.close(fd)
            fd = new
            anchors.append((file_key(os.fstat(fd)), folded_names[index + 1:]))
    finally:
        os.close(fd)
    return anchors


def scopes_overlap(a: list[ScopeAnchor], b: list[ScopeAnchor]) -> bool:
    by_identity: dict[tuple[int, int], list[tuple[str, ...]]] = {}
    for key, tail in b:
        by_identity.setdefault(key, []).append(tail)
    return any(prefix_overlap(tail, other) for key, tail in a
               for other in by_identity.get(key, []))


def overlaps(a: str, b: str) -> bool:
    # Similar missing names are conservatively overlapping, even on a case-sensitive volume.
    if prefix_overlap(scope_parts(a), scope_parts(b)):
        return True
    return scopes_overlap(path_scope(a), path_scope(b))


def identity(s: os.stat_result) -> dict[str, int]:
    return {"device": s.st_dev, "inode": s.st_ino, "mode": stat.S_IMODE(s.st_mode),
            "type": stat.S_IFMT(s.st_mode), "size": s.st_size, "nlink": s.st_nlink,
            "uid": s.st_uid, "gid": s.st_gid, "mtime_ns": s.st_mtime_ns,
            "ctime_ns": s.st_ctime_ns, "flags": getattr(s, "st_flags", 0)}


def protected(path: str) -> bool:
    parts = tuple(folded(part) for part in Path(path).parts)
    name = parts[-1]
    codex_stores = {"auth.json", "sessions", "state", "hooks", "skills", "memory"}
    return (folded(path) in {folded(p) for p in ("/", str(Path.home()), "/Users", "/System", "/Library", "/Volumes")}
            or any(p in {folded(v) for v in PROTECTED} or p.startswith("onedrive-") for p in parts)
            or name in {folded(v) for v in EVIDENCE_NAMES} or name.startswith(".env")
            or Path(name).suffix in SOURCE_SUFFIXES
            or any(parts[i] == ".codex" and parts[i + 1] in codex_stores
                   for i in range(len(parts) - 1)))


@contextmanager
def parent_fd(path: str, *, anchors: list[ScopeAnchor] | None = None) -> Iterator[tuple[int, str]]:
    """Open every ancestor with no-follow and optionally bind its physical scope."""
    folded_names = scope_parts(path)
    parts = Path(path).parts
    fd = os.open("/", os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW)
    try:
        if anchors is not None:
            anchors.append((file_key(os.fstat(fd)), folded_names))
        for index, part in enumerate(parts[1:-1]):
            new = open_directory(fd, part)
            os.close(fd)
            fd = new
            if anchors is not None:
                anchors.append((file_key(os.fstat(fd)), folded_names[index + 1:]))
        yield fd, parts[-1] if path != "/" else "."
    finally:
        os.close(fd)


def same_path(path: str, expected: dict[str, int]) -> bool:
    with parent_fd(path) as (fd, name):
        return identity(os.stat(name, dir_fd=fd, follow_symlinks=False)) == expected


def read_regular(path: str, limit: int = MAX_INPUT) -> bytes:
    """Bounded no-follow read; never opens devices/FIFOs or multiplies hardlinks."""
    parts = tuple(folded(part) for part in Path(path).parts)
    if (any(p in {".ssh", ".aws", ".gnupg", "keychains", "application support", "cloudstorage"} for p in parts)
            or folded(Path(path).name).startswith(".env") or folded(Path(path).suffix) in {".key", ".pem", ".p12", ".db", ".sqlite", ".sqlite3"}
            or any(parts[i] == ".codex" and parts[i + 1] in {"auth.json", "sessions", "memory"} for i in range(len(parts) - 1))):
        raise BoundaryError("protected_input_store")
    with parent_fd(path) as (parent, name):
        before = os.stat(name, dir_fd=parent, follow_symlinks=False)
        if getattr(before, "st_flags", 0) & DATALESS:
            raise BoundaryError("dataless_input_not_opened")
        if not stat.S_ISREG(before.st_mode) or before.st_nlink != 1:
            raise BoundaryError("unsafe_evidence_kind_or_links")
        if before.st_size > limit:
            raise BoundaryError("input_budget_exceeded")
        fd = os.open(name, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK, dir_fd=parent)
        try:
            if identity(os.fstat(fd)) != identity(before):
                raise BoundaryError("input_identity_drift")
            data = bytearray()
            while True:
                chunk = os.read(fd, min(65536, limit + 1 - len(data)))
                if not chunk:
                    break
                data.extend(chunk)
                if len(data) > limit:
                    raise BoundaryError("input_budget_exceeded")
            if identity(os.fstat(fd)) != identity(before):
                raise BoundaryError("input_identity_drift")
        finally:
            os.close(fd)
    if not same_path(path, identity(before)):
        raise BoundaryError("input_ancestry_drift")
    return bytes(data)


def unique_object(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise BoundaryError("duplicate_json_key")
        result[key] = value
    return result


def parse_json(data: bytes) -> Any:
    try:
        return json.loads(data, object_pairs_hook=unique_object,
                          parse_constant=lambda _: (_ for _ in ()).throw(BoundaryError("invalid_json")))
    except (ValueError, UnicodeError, RecursionError) as error:
        raise BoundaryError("invalid_json") from error


def valid_scope_paths(values: Sequence[Any]) -> list[str]:
    result = []
    for value in values:
        try:
            result.append(absolute(value))
        except BoundaryError:
            continue  # Invalid syntax denotes no inspected filesystem path.
    return list(dict.fromkeys(result))


def context_exclusions(value: Any) -> list[str]:
    """Keep recognized restrictions; uncertain scope shapes cannot clear a hold."""
    if not isinstance(value, dict):
        raise BoundaryError("invalid_context_scope")
    paths: list[Any] = []
    for key in ("holds", "required_local_sealed_desktop_inputs", "candidates"):
        if key not in value:
            continue
        if not isinstance(value[key], list):
            raise BoundaryError("invalid_context_scope")
        for item in value[key]:
            if key == "holds":
                paths.append(item)
                continue
            if not isinstance(item, dict) or "path" not in item:
                raise BoundaryError("invalid_context_scope")
            paths.append(item["path"])
            if key == "candidates" and "evidence" in item:
                if not isinstance(item["evidence"], list):
                    raise BoundaryError("invalid_context_scope")
                for ref in item["evidence"]:
                    if not isinstance(ref, dict) or "path" not in ref:
                        raise BoundaryError("invalid_context_scope")
                    paths.append(ref["path"])
    return list(dict.fromkeys(absolute(value) for value in paths))


def load_context(path: str | None) -> tuple[dict[str, Any], str | None]:
    if path is None:
        return {"candidates": [], "holds": []}, None
    try:
        data = read_regular(absolute(path))
        c = parse_json(data)
    except (BoundaryError, OSError) as problem:
        error = BoundaryError("context_scope_unavailable")
        error.scope_unknown = True
        raise error from problem
    try:
        return validate_context(c), digest(data)
    except BoundaryError as error:
        try:
            error.exclusions = context_exclusions(c)
        except BoundaryError as problem:
            blocked = BoundaryError("context_scope_unavailable")
            blocked.scope_unknown = True
            raise blocked from problem
        raise


def validate_context(c: Any) -> dict[str, Any]:
    if (not isinstance(c, dict) or type(c.get("schema_version")) is not int
            or c["schema_version"] != 1
            or set(c) - {"schema_version", "candidates", "holds", "required_local_sealed_desktop_inputs"}):
        raise BoundaryError("invalid_context_schema")
    holds = c.get("holds", [])
    sealed = c.get("required_local_sealed_desktop_inputs", [])
    candidates = c.get("candidates", [])
    if not all(isinstance(v, list) for v in (holds, sealed, candidates)):
        raise BoundaryError("invalid_context_schema")
    for item in sealed:
        if not isinstance(item, dict) or "path" not in item:
            raise BoundaryError("invalid_hold")
        holds = holds + [item["path"]]  # optional flags can NEVER release a hold.
    for hold in holds:
        absolute(hold)
    seen: set[str] = set()
    for item in candidates:
        if (not isinstance(item, dict) or set(item) - {"path", "owner", "task", "evidence", "expected_manifest_sha256"}
                or "path" not in item):
            raise BoundaryError("invalid_candidate_context")
        p = absolute(item["path"])
        if p in seen:
            raise BoundaryError("duplicate_candidate_context")
        seen.add(p)
        for key in ("owner", "task"):
            if key in item and (not isinstance(item[key], str) or not 1 <= len(item[key]) <= 200):
                raise BoundaryError("invalid_ownership_assertion")
        if "expected_manifest_sha256" in item and not is_sha(item["expected_manifest_sha256"]):
            raise BoundaryError("invalid_expected_manifest")
        refs = item.get("evidence", [])
        if not isinstance(refs, list) or len(refs) > 64:
            raise BoundaryError("invalid_evidence_references")
        for ref in refs:
            if (not isinstance(ref, dict)
                    or set(ref) != {"role", "path", "sha256", "archive_filename", "archive_sha256", "archive_bytes"}
                    or not isinstance(ref["role"], str)
                    or ref["role"] not in {"payload", "recovery-records"}
                    or not isinstance(ref["archive_filename"], str)
                    or not 1 <= len(ref["archive_filename"]) <= 200
                    or Path(ref["archive_filename"]).name != ref["archive_filename"]
                    or not is_sha(ref["sha256"]) or not is_sha(ref["archive_sha256"])
                    or type(ref["archive_bytes"]) is not int or ref["archive_bytes"] <= 0):
                raise BoundaryError("invalid_evidence_reference")
            absolute(ref["path"])
    return {"candidates": candidates, "holds": holds}


def is_sha(value: Any) -> bool:
    return isinstance(value, str) and len(value) == 64 and all(c in "0123456789abcdef" for c in value)


def evidence_status(ref: dict[str, Any]) -> dict[str, Any]:
    result = {"role": ref["role"], "role_source": "context_assertion_unverified",
              "receipt_sha256_expected": ref["sha256"],
              "archive_filename": ref["archive_filename"],
              "archive_sha256": ref["archive_sha256"], "archive_bytes": ref["archive_bytes"],
              "status": "unverified"}
    try:
        data = read_regular(ref["path"])
        result["receipt_sha256_observed"] = digest(data)
        if digest(data) != ref["sha256"]:
            raise BoundaryError("receipt_identity_mismatch")
        receipt = parse_json(data)
        if not isinstance(receipt, dict):
            raise BoundaryError("invalid_receipt")
        matches = (receipt.get("status") == "PASS"
                   and receipt.get("archive_sha256") == ref["archive_sha256"]
                   and type(receipt.get("archive_bytes")) is int
                   and receipt["archive_bytes"] == ref["archive_bytes"]
                   and receipt.get("content_modes_links_match") is True
                   and receipt.get("source_stable") is True
                   and type(receipt.get("executed_restored_files")) is int
                   and receipt["executed_restored_files"] == 0
                   and type(receipt.get("restored_entries")) is int
                   and receipt["restored_entries"] > 0
                   and all(isinstance(receipt.get(k), str) and bool(receipt[k])
                           for k in ("independent_download", "restore")))
        if matches:
            download = absolute(receipt["independent_download"])
            restore = absolute(receipt["restore"])
            matches = Path(download).name == ref["archive_filename"] and not overlaps(download, restore)
        if not matches:
            raise BoundaryError("receipt_not_passing_or_not_bound")
        result["status"] = "recorded_pass_identity_matches"
    except (OSError, BoundaryError) as error:
        result["reason"] = str(error) if isinstance(error, BoundaryError) else "receipt_inaccessible"
    return result


class Inventory:
    def __init__(self, root: str, limits: Limits):
        self.root, self.limits = root, limits
        self.entries: list[dict[str, Any]] = []
        self.reasons: set[str] = set()
        self.complete = True
        self.bytes_read = 0
        self.deadline = time.monotonic() + limits.seconds
        self.device: int | None = None

    def fail(self, reason: str) -> None:
        self.complete = False
        self.reasons.add(reason)

    def names(self, fd: int) -> list[str]:
        names = []
        with os.scandir(fd) as it:
            for entry in it:
                if len(names) >= self.limits.entries:
                    raise BoundaryError("membership_budget_exceeded")
                names.append(entry.name)
        return sorted(names)

    def walk(self, parent: int, name: str, relative: str, depth: int = 0) -> None:
        if len(self.entries) >= self.limits.entries or depth > self.limits.depth:
            raise BoundaryError("entry_or_depth_budget_exceeded")
        if time.monotonic() > self.deadline:
            raise BoundaryError("observation_time_budget_exceeded")
        before = os.stat(name, dir_fd=parent, follow_symlinks=False)
        observed = identity(before)
        entry: dict[str, Any] = {"member": relative, "identity": observed,
                                 "allocated_bytes_nonadditive": getattr(before, "st_blocks", 0) * 512}
        self.entries.append(entry)
        if getattr(before, "st_flags", 0) & DATALESS:
            entry["kind"] = "dataless_placeholder"
            self.fail("dataless_member_not_opened")
            return
        if self.device is None:
            self.device = before.st_dev
        full = self.root if relative == "." else self.root + "/" + relative
        if protected(full):
            entry["kind"] = "protected"
            self.fail("protected_local_resource")
            return
        if before.st_dev != self.device or before.st_dev != os.fstat(parent).st_dev:
            entry["kind"] = "mount_boundary"
            self.fail("mount_boundary")
            return
        if stat.S_ISLNK(before.st_mode):
            entry.update(kind="symlink", target=os.readlink(name, dir_fd=parent))
            self.reasons.add("symlink_dependency_unresolved")
        elif stat.S_ISDIR(before.st_mode):
            entry["kind"] = "directory"
            fd = os.open(name, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW, dir_fd=parent)
            try:
                if identity(os.fstat(fd)) != observed:
                    raise BoundaryError("entry_identity_drift")
                names = self.names(fd)
                entry["children"] = names
                if any(folded(child) == ".git" for child in names):
                    self.fail("checkout_or_git_directory")
                    entry["membership_not_expanded"] = True
                    return
                for child in names:
                    rel = child if relative == "." else relative + "/" + child
                    try:
                        self.walk(fd, child, rel, depth + 1)
                    except (OSError, BoundaryError) as error:
                        self.fail(str(error) if isinstance(error, BoundaryError) else "member_inaccessible_or_changed")
                        # Continue independent siblings; cap exhaustion stops this directory.
                        if len(self.entries) >= self.limits.entries:
                            break
                if self.names(fd) != names or identity(os.fstat(fd)) != observed:
                    raise BoundaryError("directory_membership_or_identity_drift")
            finally:
                os.close(fd)
        elif stat.S_ISREG(before.st_mode):
            entry["kind"] = "file"
            if before.st_nlink != 1:
                self.fail("hardlink_dependency_unresolved")
                return
            if before.st_size > self.limits.hashed_bytes - self.bytes_read:
                self.fail("hash_budget_exceeded")
                return
            fd = os.open(name, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK, dir_fd=parent)
            try:
                if identity(os.fstat(fd)) != observed:
                    raise BoundaryError("entry_identity_drift")
                h = hashlib.sha256()
                total = 0
                while total < before.st_size:
                    if time.monotonic() > self.deadline:
                        raise BoundaryError("observation_time_budget_exceeded")
                    chunk = os.read(fd, min(65536, before.st_size - total))
                    if not chunk:
                        break
                    self.bytes_read += len(chunk)
                    total += len(chunk)
                    if self.bytes_read > self.limits.hashed_bytes:
                        raise BoundaryError("hash_budget_exceeded")
                    h.update(chunk)
                if identity(os.fstat(fd)) != observed or total != before.st_size:
                    raise BoundaryError("entry_identity_drift")
                entry["sha256"] = h.hexdigest()
            finally:
                os.close(fd)
        else:
            entry["kind"] = "unsupported"
            self.fail("unsupported_file_kind")
        if identity(os.stat(name, dir_fd=parent, follow_symlinks=False)) != observed:
            raise BoundaryError("entry_identity_drift")

    def run(self) -> dict[str, Any]:
        try:
            with parent_fd(self.root) as (fd, name):
                self.walk(fd, name, ".")
            if self.entries and not same_path(self.root, self.entries[0]["identity"]):
                self.fail("candidate_ancestry_drift")
        except (OSError, BoundaryError) as error:
            self.fail(str(error) if isinstance(error, BoundaryError) else "candidate_inaccessible_or_unsafe_ancestor")
        return {"complete": self.complete, "entries": self.entries,
                "manifest_sha256": digest(encoded(self.entries)), "hashed_bytes": self.bytes_read,
                "reasons": sorted(self.reasons), "snapshot_or_quiescence_proof": False}


def inspect_candidates(paths: Sequence[str], context: dict[str, Any] | None = None,
                       context_error: str | None = None, limits: Limits = Limits()) -> dict[str, Any]:
    if not 1 <= len(paths) <= MAX_CANDIDATES:
        raise BoundaryError("candidate_count_out_of_bounds")
    c = context or {"candidates": [], "holds": []}
    results = []
    remaining_entries, remaining_bytes = limits.entries, limits.hashed_bytes
    for path in paths:
        result: dict[str, Any] = {"path": path, "disposition": "defer", "removal_eligible": False,
                                  "dependencies": {"status": "incomplete", "reasons": UNKNOWN},
                                  "recovery_evidence": [], "ownership": {"status": "unknown"},
                                  "retention_holds": {"other_holds_status": "unknown", "overriding_paths": []}}
        reasons = set(UNKNOWN)
        try:
            absolute(path)
            if context_error:
                raise BoundaryError(context_error)
            item = next((v for v in c["candidates"] if v["path"] == path), {})
            result["ownership"] = {"status": "asserted_not_verified" if item.get("owner") and item.get("task") else "unknown",
                                   "assertion_sha256": digest(encoded({k: item[k] for k in ("owner", "task") if k in item}))}
            held = [p for p in c["holds"] if overlaps(path, p)]
            result["retention_holds"] = {"overriding_paths": held, "other_holds_status": "unknown"}
            if held or protected(path):
                result["disposition"] = "retain"
                raise BoundaryError("explicit_local_hold" if held else "protected_local_resource")
            if list(paths).count(path) > 1 or sum(overlaps(path, other) for other in valid_scope_paths(paths)) > 1:
                raise BoundaryError("duplicate_or_overlapping_candidates")
            if remaining_entries <= 0:
                raise BoundaryError("run_entry_budget_exceeded")
            current_limits = Limits(remaining_entries, remaining_bytes, limits.depth, limits.seconds)
            inv = Inventory(path, current_limits).run()
            remaining_entries -= len(inv["entries"])
            remaining_bytes = max(0, remaining_bytes - inv["hashed_bytes"])
            result["inventory"] = inv
            reasons.update(inv["reasons"])
            if item.get("expected_manifest_sha256") and inv["manifest_sha256"] != item["expected_manifest_sha256"]:
                reasons.add("expected_inventory_identity_mismatch")
                inv["complete"] = False
            if any(x in reasons for x in ("protected_local_resource", "checkout_or_git_directory")):
                result["disposition"] = "retain"
            for ref in item.get("evidence", []):
                result["recovery_evidence"].append(evidence_status(ref))
            for role in ("payload", "recovery-records"):
                refs = [v for v in result["recovery_evidence"] if v["role"] == role]
                if not refs or any(v["status"] != "recorded_pass_identity_matches" for v in refs):
                    reasons.add(role + "_evidence_missing_or_unverified")
            reasons.add("recorded_receipts_do_not_prove_current_coverage_or_availability")
        except (BoundaryError, OSError) as error:
            reasons.add(str(error) if isinstance(error, BoundaryError) else "candidate_check_inaccessible")
        result["reasons"] = sorted(reasons)
        results.append(result)
    return {"schema_version": 1, "observed_utc": datetime.now(timezone.utc).isoformat(),
            "operation": "read_only_inventory", "removal_authority": False, "candidates": results,
            "limitations": ["Point-in-time observations; no execution or remote verification.",
                            "Metadata and hashes only; report must remain private outside Git.",
                            "Allocated bytes are nonadditive; no physical reclaim estimate."]}


def write_report(path: str, report: dict[str, Any], inputs: Sequence[str]) -> None:
    absolute(path)
    local_roots = ["/private/tmp", "/private/var/folders", str(Path.home() / ".codex" / "visualizations")]
    if sys.platform != "darwin":
        local_roots.append("/tmp")
    if not any(path.startswith(base + "/") for base in local_roots):
        raise BoundaryError("report_root_not_supported_local")
    if protected(path):
        raise BoundaryError("unsafe_report_location")
    # Never publish reports in a checkout, cloud sync store, or known protected store.
    for ancestor in Path(path).parents:
        if protected(str(ancestor)) and str(ancestor) not in {"/", "/Users", str(Path.home())}:
            raise BoundaryError("unsafe_report_location")
        with parent_fd(str(ancestor / ".git")) as (fd, name):
            try:
                os.stat(name, dir_fd=fd, follow_symlinks=False)
            except FileNotFoundError:
                continue
            raise BoundaryError("report_inside_checkout")
    data = encoded(report) + b"\n"
    if len(data) > 16 * 1024 * 1024:
        raise BoundaryError("report_budget_exceeded")
    if len(inputs) > MAX_SCOPE_PATHS:
        raise BoundaryError("scope_path_budget_exceeded")
    anchors: list[ScopeAnchor] = []
    with parent_fd(path, anchors=anchors) as (fd, name):
        parent = os.fstat(fd)
        if parent.st_uid != os.getuid() or stat.S_IMODE(parent.st_mode) & 0o077:
            raise BoundaryError("report_parent_not_private")
        for source in inputs:
            if (prefix_overlap(scope_parts(path), scope_parts(source))
                    or scopes_overlap(anchors, path_scope(source))):
                raise BoundaryError("unsafe_report_location")
        if not same_path(str(Path(path).parent), identity(parent)):
            raise BoundaryError("report_parent_identity_drift")
        out = os.open(name, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600, dir_fd=fd)
        try:
            with os.fdopen(out, "wb", closefd=False) as stream:
                stream.write(data)
                stream.flush()
                os.fsync(out)
        finally:
            os.close(out)


def main(arguments: Sequence[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--candidate", action="append", required=True)
    parser.add_argument("--context")
    parser.add_argument("--report", required=True)
    args = parser.parse_args(arguments)
    try:
        context, context_hash, error = None, None, None
        exclusions: list[str] = []
        try:
            context, context_hash = load_context(args.context)
        except (BoundaryError, OSError) as problem:
            if isinstance(problem, BoundaryError) and problem.scope_unknown:
                raise
            error = str(problem) if isinstance(problem, BoundaryError) else "context_inaccessible"
            if isinstance(problem, BoundaryError):
                exclusions = problem.exclusions
        report = inspect_candidates(args.candidate, context, error)
        report["context_sha256"] = context_hash
        inputs = list(args.candidate) + ([args.context] if args.context else [])
        inputs += exclusions + (context_exclusions(context) if context is not None else [])
        write_report(args.report, report, inputs)
        print("storage-inventory: private report created; removal authority: none")
        return 0
    except (BoundaryError, OSError, ValueError) as error:
        code = str(error) if isinstance(error, BoundaryError) else "inspection_or_report_failed"
        print("storage-inventory: " + code, file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
