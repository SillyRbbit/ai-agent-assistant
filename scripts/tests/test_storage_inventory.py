from __future__ import annotations

import importlib.util
import io
import json
import os
from pathlib import Path
import socket
import stat
import sys
import tempfile
import unittest
from contextlib import redirect_stderr, redirect_stdout
from unittest.mock import patch

spec = importlib.util.spec_from_file_location("storage_inventory", Path(__file__).parents[1] / "storage_inventory.py")
assert spec and spec.loader
inventory = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = inventory
spec.loader.exec_module(inventory)


class StorageInventoryTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="cortexa-inventory-test-", dir="/private/tmp" if sys.platform == "darwin" else None)
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name).resolve()
        self.candidate = self.root / "artifact"
        self.candidate.mkdir()
        self.file = self.candidate / "payload.bin"
        self.file.write_bytes(b"synthetic bytes only")
        self.output = self.root / "reports"
        self.output.mkdir(mode=0o700)

    def result(self, path=None, **kwargs):
        return inventory.inspect_candidates([str(path or self.candidate)], **kwargs)["candidates"][0]

    def test_exact_members_hash_identity_and_no_mutation(self):
        before = self.file.stat()
        r = self.result()
        self.assertEqual(r["disposition"], "defer")
        self.assertFalse(r["removal_eligible"])
        i = r["inventory"]
        self.assertTrue(i["complete"])
        self.assertEqual([v["member"] for v in i["entries"]], [".", "payload.bin"])
        self.assertEqual(i["entries"][1]["sha256"], inventory.digest(b"synthetic bytes only"))
        self.assertEqual(i["entries"][1]["identity"]["inode"], before.st_ino)
        self.assertEqual(inventory.identity(self.file.stat()), inventory.identity(before))
        self.assertIn("task_process_runner_consumers_unknown", r["reasons"])
        self.assertFalse(i["snapshot_or_quiescence_proof"])

    def test_empty_directory_complete_but_never_eligible(self):
        p = self.root / "empty"
        p.mkdir()
        r = self.result(p)
        self.assertTrue(r["inventory"]["complete"])
        self.assertFalse(r["removal_eligible"])
        self.assertEqual(r["inventory"]["entries"][0]["children"], [])

    def test_root_and_known_sensitive_paths_not_traversed(self):
        for p in ["/", str(Path.home()), str(self.root / ".env"), str(self.root / "Application Support/private")]:
            with self.subTest(path=p), patch.object(inventory, "Inventory") as walker:
                r = self.result(p)
                walker.assert_not_called()
                self.assertEqual(r["disposition"], "retain")

    def test_relative_traversal_and_noncanonical_inputs_defer(self):
        for p in ["relative", "/private/tmp/../secret", "/private//tmp", "//private/tmp", "bad\0path"]:
            with self.subTest(path=p):
                r = self.result(p)
                self.assertIn("invalid_absolute_path", r["reasons"])
                self.assertNotIn("inventory", r)

    def test_symlink_leaf_recorded_never_followed(self):
        target = self.root / "outside"
        target.write_bytes(b"never read this")
        link = self.candidate / "link"
        link.symlink_to(target)
        r = self.result()
        e = next(v for v in r["inventory"]["entries"] if v["member"] == "link")
        self.assertEqual(e["kind"], "symlink")
        self.assertEqual(e["target"], str(target))
        self.assertNotIn("sha256", e)
        self.assertIn("symlink_dependency_unresolved", r["reasons"])

    def test_symlink_ancestor_rejected(self):
        link = self.root / "shortcut"
        link.symlink_to(self.candidate, target_is_directory=True)
        r = self.result(link / "payload.bin")
        self.assertFalse(r["inventory"]["complete"])
        self.assertEqual(r["inventory"]["entries"], [])

    def test_hardlink_dependency_stops_content_read(self):
        os.link(self.file, self.root / "external-link")
        r = self.result()
        self.assertIn("hardlink_dependency_unresolved", r["reasons"])
        self.assertNotIn("sha256", r["inventory"]["entries"][1])
        self.assertFalse(r["inventory"]["complete"])

    def test_fifo_and_socket_never_opened(self):
        os.mkfifo(self.candidate / "pipe")
        s = socket.socket(socket.AF_UNIX)
        self.addCleanup(s.close)
        s.bind(str(self.candidate / "sock"))
        r = self.result()
        self.assertIn("unsupported_file_kind", r["reasons"])
        self.assertFalse(r["inventory"]["complete"])

    def test_mount_boundary_stops_subtree(self):
        original = inventory.os.stat
        from types import SimpleNamespace
        def other_device(path, *args, **kwargs):
            observed = original(path, *args, **kwargs)
            if path == "artifact":
                fields = {n: getattr(observed, n) for n in dir(observed) if n.startswith("st_")}
                fields["st_dev"] += 1
                return SimpleNamespace(**fields)
            return observed
        with patch.object(inventory.os, "stat", side_effect=other_device):
            r = self.result()
        self.assertEqual(len(r["inventory"]["entries"]), 1)
        self.assertIn("mount_boundary", r["reasons"])

    def test_protected_child_and_checkout_retained(self):
        (self.candidate / "node_modules").mkdir()
        (self.candidate / "node_modules" / "hidden").write_text("unread")
        r = self.result()
        self.assertEqual(r["disposition"], "retain")
        self.assertNotIn("node_modules/hidden", [v["member"] for v in r["inventory"]["entries"]])
        (self.candidate / ".git").write_text("gitdir: /unrelated")
        r = self.result()
        self.assertIn("checkout_or_git_directory", r["reasons"])
        self.assertEqual(len(r["inventory"]["entries"]), 1)

    def test_source_and_sealed_evidence_not_hashed(self):
        for name in ["source.rs", "VERIFIED.json", "FINAL-SEAL.json", ".env.private"]:
            (self.candidate / name).write_text("private content")
        r = self.result()
        for e in r["inventory"]["entries"]:
            if e["member"] not in {".", "payload.bin"}:
                self.assertEqual(e["kind"], "protected")
                self.assertNotIn("sha256", e)

    def test_required_local_hold_overrides_optional_flag(self):
        c = self.root / "context.json"
        c.write_text(json.dumps({"schema_version": 1, "required_local_sealed_desktop_inputs": [
            {"path": str(self.file), "optional_large_manifest": True}]}))
        context, _ = inventory.load_context(str(c))
        with patch.object(inventory, "Inventory") as walker:
            r = self.result(context=context)
        walker.assert_not_called()
        self.assertEqual(r["disposition"], "retain")
        self.assertIn("explicit_local_hold", r["reasons"])

    def test_ancestor_hold_and_missing_held_path_still_hold(self):
        for hold in [self.root, self.candidate / "not-present"]:
            r = self.result(context={"candidates": [], "holds": [str(hold)]})
            self.assertEqual(r["disposition"], "retain")

    def test_entry_hash_depth_and_time_budgets_are_incomplete(self):
        (self.candidate / "child").mkdir()
        (self.candidate / "child" / "x").write_bytes(b"x")
        limits = [inventory.Limits(entries=1), inventory.Limits(hashed_bytes=1),
                  inventory.Limits(depth=0), inventory.Limits(seconds=-1)]
        for limit in limits:
            with self.subTest(limit=limit):
                r = self.result(limits=limit)
                self.assertFalse(r["inventory"]["complete"])
                self.assertFalse(r["removal_eligible"])

    def test_file_changes_while_hashing_fail_closed(self):
        original = inventory.os.read
        changed = False
        def mutate(fd, count):
            nonlocal changed
            chunk = original(fd, count)
            if chunk and not changed:
                changed = True
                self.file.write_bytes(b"changed")
            return chunk
        with patch.object(inventory.os, "read", side_effect=mutate):
            r = self.result()
        self.assertFalse(r["inventory"]["complete"])
        self.assertIn("entry_identity_drift", r["reasons"])
        self.assertNotIn("sha256", r["inventory"]["entries"][1])

    def test_file_replaced_by_link_before_open_never_reads_target(self):
        original = inventory.os.open
        outside = self.root / "outside"
        outside.write_bytes(b"secret outside")
        def swap(path, flags, *args, **kwargs):
            if path == "payload.bin":
                self.file.unlink()
                self.file.symlink_to(outside)
            return original(path, flags, *args, **kwargs)
        with patch.object(inventory.os, "open", side_effect=swap):
            r = self.result()
        self.assertFalse(r["inventory"]["complete"])
        self.assertEqual(r["inventory"]["hashed_bytes"], 0)

    def test_directory_membership_changes_and_siblings_continue(self):
        original = inventory.Inventory.names
        calls = 0
        def names(obj, fd):
            nonlocal calls
            calls += 1
            if calls == 2:
                (self.candidate / "new.bin").write_bytes(b"new")
            return original(obj, fd)
        with patch.object(inventory.Inventory, "names", new=names):
            r = self.result()
        self.assertFalse(r["inventory"]["complete"])
        self.assertIn("directory_membership_or_identity_drift", r["reasons"])

    def test_inaccessible_member_does_not_block_independent_candidate(self):
        real = inventory.os.stat
        def denied(path, *a, **k):
            if path == "payload.bin":
                raise PermissionError("PRIVATE OWNER TEXT")
            return real(path, *a, **k)
        second = self.root / "second.bin"
        second.write_bytes(b"second")
        with patch.object(inventory.os, "stat", side_effect=denied):
            report = inventory.inspect_candidates([str(self.candidate), str(second)])
        self.assertFalse(report["candidates"][0]["inventory"]["complete"])
        self.assertTrue(report["candidates"][1]["inventory"]["complete"])
        self.assertNotIn("PRIVATE OWNER TEXT", json.dumps(report))

    def test_missing_is_not_removed_and_overlaps_defer(self):
        r = self.result(self.root / "absent")
        self.assertEqual(r["disposition"], "defer")
        self.assertFalse(r["inventory"]["complete"])
        for paths in [[str(self.candidate)] * 2, [str(self.candidate), str(self.file)]]:
            r = inventory.inspect_candidates(paths)
            for c in r["candidates"]:
                self.assertIn("duplicate_or_overlapping_candidates", c["reasons"])
                self.assertNotIn("inventory", c)

    def receipt(self, role="payload", **changes):
        value = {"status": "PASS", "archive_sha256": "a" * 64, "archive_bytes": 42,
                 "content_modes_links_match": True, "source_stable": True,
                 "executed_restored_files": 0, "restored_entries": 2,
                 "independent_download": "/isolated/download/archive.tar.gz", "restore": "/isolated/restore"}
        value.update(changes)
        p = self.root / (role + ".json")
        p.write_text(json.dumps(value))
        return {"role": role, "path": str(p), "sha256": inventory.digest(p.read_bytes()),
                "archive_filename": "archive.tar.gz", "archive_sha256": "a" * 64, "archive_bytes": 42}

    def test_payload_and_recovery_records_reported_without_elevation(self):
        refs = [self.receipt(), self.receipt("recovery-records")]
        context = {"holds": [], "candidates": [{"path": str(self.candidate),
                   "owner": "OWNER PRIVATE", "task": "TASK PRIVATE", "evidence": refs}]}
        r = self.result(context=context)
        self.assertEqual(r["ownership"]["status"], "asserted_not_verified")
        self.assertEqual([x["status"] for x in r["recovery_evidence"]], ["recorded_pass_identity_matches"] * 2)
        self.assertFalse(r["removal_eligible"])
        self.assertIn("private_cloud_availability_not_checked", r["reasons"])
        self.assertNotIn("OWNER PRIVATE", json.dumps(r))
        self.assertNotIn("independent_download", json.dumps(r))

    def test_receipt_missing_mismatch_executed_or_forged_bool_fails(self):
        ref = self.receipt()
        Path(ref["path"]).write_text("CHANGED")
        self.assertEqual(inventory.evidence_status(ref)["reason"], "receipt_identity_mismatch")
        ref["path"] += "missing"
        self.assertEqual(inventory.evidence_status(ref)["reason"], "receipt_inaccessible")
        for changes in [{"status": "FAIL"}, {"executed_restored_files": 1},
                        {"executed_restored_files": False}, {"archive_bytes": True},
                        {"source_stable": False}, {"content_modes_links_match": "true"},
                        {"archive_sha256": "b" * 64}, {"restore": ""},
                        {"independent_download": "/other/file.tar.gz"}, {"restore": "/isolated/download"}]:
            with self.subTest(changes=changes):
                status = inventory.evidence_status(self.receipt(**changes))
                self.assertEqual(status["status"], "unverified")

    def test_context_errors_do_not_discard_holds_or_inspect_candidates(self):
        p = self.root / "context.json"
        for data in ['{"schema_version":1,"schema_version":1}', '{"schema_version":1,"release_all":true}',
                     '{"schema_version":1,"holds":[7]}', '{"schema_version":NaN}', '[[]]']:
            p.write_text(data)
            with self.subTest(data=data), redirect_stdout(io.StringIO()), redirect_stderr(io.StringIO()):
                report = self.output / (str(len(data)) + inventory.digest(data.encode())[:8] + ".json")
                result = inventory.main(["--candidate", str(self.candidate), "--context", str(p), "--report", str(report)])
                if "NaN" in data or data.count('"schema_version"') > 1 or '"holds":[7]' in data or data == '[[]]':
                    self.assertEqual(result, 2)
                    self.assertFalse(report.exists())
                else:
                    self.assertEqual(result, 0)
                    value = json.loads(report.read_text())["candidates"][0]
                    self.assertNotIn("inventory", value)
                    self.assertFalse(value["removal_eligible"])

    def test_evidence_input_boundaries(self):
        p = self.root / "input"
        p.write_bytes(b"x" * 8)
        with self.assertRaises(inventory.BoundaryError):
            inventory.read_regular(str(p), limit=4)
        link = self.root / "input-link"
        link.symlink_to(p)
        with self.assertRaises(inventory.BoundaryError):
            inventory.read_regular(str(link))
        os.link(p, self.root / "other")
        with self.assertRaises(inventory.BoundaryError):
            inventory.read_regular(str(p))

    def test_report_private_exclusive_and_content_free_stdout(self):
        report = self.output / "result.json"
        out = io.StringIO()
        with redirect_stdout(out):
            exit_code = inventory.main(["--candidate", str(self.candidate), "--report", str(report)])
        self.assertEqual(exit_code, 0)
        self.assertEqual(stat.S_IMODE(report.stat().st_mode), 0o600)
        self.assertNotIn(str(self.root), out.getvalue())
        self.assertNotIn("synthetic bytes only", report.read_text())
        before = report.read_bytes()
        with redirect_stderr(io.StringIO()):
            self.assertEqual(inventory.main(["--candidate", str(self.candidate), "--report", str(report)]), 2)
        self.assertEqual(before, report.read_bytes())

    def test_report_refuses_shared_parent_links_input_overlap_and_checkout(self):
        shared = self.root / "shared"
        shared.mkdir(mode=0o755)
        shared.chmod(0o755)  # Exercise a genuinely nonprivate parent under any umask.
        self.assertEqual(stat.S_IMODE(shared.stat().st_mode), 0o755)
        link = self.root / "report-link"
        link.symlink_to(self.output, target_is_directory=True)
        checkout = self.root / "checkout"
        checkout.mkdir(mode=0o700)
        (checkout / ".git").write_text("gitdir: /not-followed")
        locations = [shared / "out.json", link / "out.json", self.candidate / "out.json", checkout / "out.json"]
        for path in locations:
            with self.subTest(path=path), self.assertRaises((inventory.BoundaryError, OSError)):
                inventory.write_report(str(path), {}, [str(self.candidate)])
        self.assertEqual(list(shared.iterdir()), [])
        self.assertEqual(list(self.output.iterdir()), [])

    def test_report_final_symlink_not_overwritten(self):
        path = self.output / "out.json"
        path.symlink_to(self.file)
        with self.assertRaises(OSError):
            inventory.write_report(str(path), {}, [str(self.candidate)])
        self.assertEqual(self.file.read_bytes(), b"synthetic bytes only")

    def test_expected_inventory_drift_is_not_accepted(self):
        context = {"holds": [], "candidates": [{"path": str(self.candidate), "expected_manifest_sha256": "a" * 64}]}
        r = self.result(context=context)
        self.assertIn("expected_inventory_identity_mismatch", r["reasons"])
        self.assertFalse(r["inventory"]["complete"])

    def test_known_credential_store_is_not_opened_as_evidence(self):
        with patch.object(inventory.os, "open") as opening:
            with self.assertRaises(inventory.BoundaryError):
                inventory.read_regular(str(self.root / ".env"))
            opening.assert_not_called()

    def test_budget_is_shared_across_candidates(self):
        second = self.root / "second.bin"
        second.write_bytes(b"second")
        r = inventory.inspect_candidates([str(self.file), str(second)], limits=inventory.Limits(entries=1))
        self.assertTrue(r["candidates"][0]["inventory"]["complete"])
        self.assertIn("run_entry_budget_exceeded", r["candidates"][1]["reasons"])

    def test_evidence_parser_schema_binds_exact_reference(self):
        ref = self.receipt()
        p = self.root / "context.json"
        p.write_text(json.dumps({"schema_version": 1, "candidates": [{"path": str(self.candidate), "evidence": [ref]}]}))
        context, hashed = inventory.load_context(str(p))
        self.assertEqual(hashed, inventory.digest(p.read_bytes()))
        self.assertEqual(context["candidates"][0]["evidence"], [ref])
        ref["unexpected"] = "not accepted"
        p.write_text(json.dumps({"schema_version": 1, "candidates": [{"path": str(self.candidate), "evidence": [ref]}]}))
        with self.assertRaises(inventory.BoundaryError):
            inventory.load_context(str(p))

    def test_evidence_changes_during_read_are_rejected(self):
        ref = self.receipt()
        real = inventory.os.read
        changed = False
        def mutate(fd, count):
            nonlocal changed
            data = real(fd, count)
            if data and not changed:
                changed = True
                Path(ref["path"]).write_text("changed")
            return data
        with patch.object(inventory.os, "read", side_effect=mutate):
            status = inventory.evidence_status(ref)
        self.assertEqual(status["status"], "unverified")
        self.assertEqual(status["reason"], "input_identity_drift")

    def test_final_ancestor_replacement_is_detected(self):
        original = inventory.same_path
        def replace(path, expected):
            self.candidate.rename(self.root / "moved-artifact")
            self.candidate.mkdir()
            return original(path, expected)
        with patch.object(inventory, "same_path", side_effect=replace):
            r = self.result()
        self.assertFalse(r["inventory"]["complete"])
        self.assertIn("candidate_ancestry_drift", r["reasons"])

    def test_failed_report_write_keeps_private_file_without_cleanup(self):
        path = self.output / "failed.json"
        with patch.object(inventory.os, "fsync", side_effect=OSError("private failure")):
            with self.assertRaises(OSError):
                inventory.write_report(str(path), {"partial": True}, [])
        self.assertTrue(path.is_file())
        self.assertEqual(stat.S_IMODE(path.stat().st_mode), 0o600)

    def dataless_stat(self, path, *args, **kwargs):
        observed = self.real_stat(path, *args, **kwargs)
        if path == "payload.bin":
            from types import SimpleNamespace
            fields = {n: getattr(observed, n) for n in dir(observed) if n.startswith("st_")}
            fields["st_flags"] = inventory.DATALESS
            return SimpleNamespace(**fields)
        return observed

    def test_dataless_member_is_not_hydrated(self):
        self.real_stat = inventory.os.stat
        real_open = inventory.os.open
        opened = []
        def track(path, *args, **kwargs):
            opened.append(path)
            return real_open(path, *args, **kwargs)
        with patch.object(inventory.os, "stat", side_effect=self.dataless_stat), patch.object(inventory.os, "open", side_effect=track):
            r = self.result()
        self.assertIn("dataless_member_not_opened", r["reasons"])
        self.assertFalse(r["inventory"]["complete"])
        self.assertNotIn("payload.bin", opened)

    def test_dataless_evidence_input_is_not_opened(self):
        self.real_stat = inventory.os.stat
        with patch.object(inventory.os, "stat", side_effect=self.dataless_stat):
            with self.assertRaisesRegex(inventory.BoundaryError, "dataless_input_not_opened"):
                inventory.read_regular(str(self.file))

    def test_unsupported_report_root_is_refused_before_open(self):
        with patch.object(inventory.os, "open") as opened:
            with self.assertRaisesRegex(inventory.BoundaryError, "report_root_not_supported_local"):
                inventory.write_report("/arbitrary-private-root/report.json", {}, [])
            opened.assert_not_called()

    def test_explicit_candidate_count_limits(self):
        for paths in [[], [str(self.candidate)] * 65]:
            with self.assertRaises(inventory.BoundaryError):
                inventory.inspect_candidates(paths)

    def test_case_alias_cannot_place_report_inside_candidate(self):
        alias = self.candidate.with_name("ARTIFACT")
        report = alias / "case-alias-report.json"
        self.assertTrue(inventory.overlaps(str(self.candidate), str(alias)))
        with self.assertRaises((inventory.BoundaryError, OSError)):
            inventory.write_report(str(report), {}, [str(self.candidate)])
        self.assertFalse((self.candidate / report.name).exists())

    def native_alias(self, path):
        if sys.platform != "darwin":
            self.skipTest("macOS Data-volume alias probe")
        alias = Path("/System/Volumes/Data") / path.relative_to("/")
        try:
            alias_stat, source_stat = alias.stat(), path.stat()
            equivalent = (alias_stat.st_dev, alias_stat.st_ino) == (source_stat.st_dev, source_stat.st_ino)
        except OSError:
            equivalent = False
        if not equivalent:
            self.skipTest("No native Data-volume alias for the synthetic fixture")
        return alias

    def test_physical_alias_cannot_place_report_inside_candidate(self):
        alias = self.native_alias(self.candidate)
        report = self.candidate / "physical-alias-report.json"
        self.assertTrue(inventory.overlaps(str(alias), str(self.candidate)))
        with self.assertRaises(inventory.BoundaryError):
            inventory.write_report(str(report), {}, [str(alias)])
        self.assertFalse(report.exists())

    def test_missing_hold_under_physical_alias_still_excludes_report(self):
        alias = self.native_alias(self.output)
        missing = alias / "future-caf\u00e9.json"
        report = self.output / "FUTURE-CAFE\u0301.JSON"
        self.assertFalse(missing.exists())
        self.assertTrue(inventory.overlaps(str(missing), str(report)))
        with self.assertRaises(inventory.BoundaryError):
            inventory.write_report(str(report), {}, [str(missing)])
        self.assertFalse(report.exists())

    def test_hardlink_aliases_are_not_independent_candidates(self):
        other = self.root / "alias.bin"
        os.link(self.file, other)
        self.assertTrue(inventory.overlaps(str(self.file), str(other)))
        report = inventory.inspect_candidates([str(self.file), str(other)])
        for result in report["candidates"]:
            self.assertIn("duplicate_or_overlapping_candidates", result["reasons"])
            self.assertNotIn("inventory", result)
            self.assertFalse(result["removal_eligible"])

    def test_alias_hold_overrides_inventory_without_opening_content(self):
        context = {"candidates": [], "holds": [str(self.candidate.with_name("ARTIFACT"))]}
        with patch.object(inventory, "Inventory") as scanner:
            result = self.result(context=context)
        scanner.assert_not_called()
        self.assertEqual(result["disposition"], "retain")
        self.assertIn("explicit_local_hold", result["reasons"])

    def test_report_can_use_unrelated_similar_prefix_directory(self):
        neighbor = self.root / "artifact-extra"
        neighbor.mkdir(mode=0o700)
        report = neighbor / "safe.json"
        inventory.write_report(str(report), {"synthetic": True}, [str(self.candidate)])
        self.assertEqual(json.loads(report.read_text()), {"synthetic": True})
        self.assertEqual(stat.S_IMODE(report.stat().st_mode), 0o600)

    def test_explicit_and_required_sealed_holds_exclude_report_destinations(self):
        sealed = self.root / "sealed"
        sealed.mkdir(mode=0o700)
        for index, control in enumerate([
            {"holds": [str(sealed)]},
            {"required_local_sealed_desktop_inputs": [{"path": str(sealed), "optional_large_manifest": True}]},
        ]):
            context = self.root / ("held-context-" + str(index) + ".json")
            context.write_text(json.dumps({"schema_version": 1, **control}))
            report = sealed / ("report-" + str(index) + ".json")
            with redirect_stderr(io.StringIO()):
                result = inventory.main(["--candidate", str(self.file), "--context", str(context), "--report", str(report)])
            self.assertEqual(result, 2)
            self.assertFalse(report.exists())
        self.assertEqual(list(sealed.iterdir()), [])

    def test_context_role_types_defer_safely_without_traceback(self):
        for index, role in enumerate([[], {}, None, True, 7]):
            ref = {"role": role, "path": str(self.root / "receipt.json"), "sha256": "a" * 64,
                   "archive_filename": "archive.tar.gz", "archive_sha256": "b" * 64, "archive_bytes": 1}
            context = self.root / ("role-context-" + str(index) + ".json")
            context.write_text(json.dumps({"schema_version": 1, "candidates": [{"path": str(self.candidate), "evidence": [ref]}]}))
            report = self.output / ("role-result-" + str(index) + ".json")
            errors, output = io.StringIO(), io.StringIO()
            with redirect_stderr(errors), redirect_stdout(output):
                result = inventory.main(["--candidate", str(self.candidate), "--context", str(context), "--report", str(report)])
            self.assertEqual(result, 0)
            self.assertNotIn("Traceback", errors.getvalue())
            candidate = json.loads(report.read_text())["candidates"][0]
            self.assertEqual(candidate["disposition"], "defer")
            self.assertIn("invalid_evidence_reference", candidate["reasons"])
            self.assertFalse(candidate["removal_eligible"])
            self.assertNotIn("inventory", candidate)

    def test_parsed_invalid_context_cannot_discard_valid_holds(self):
        sealed = self.root / "sealed"
        sealed.mkdir(mode=0o700)
        bad_ref = {"role": [], "path": str(self.root / "receipt.json"), "sha256": "a" * 64,
                   "archive_filename": "archive.tar.gz", "archive_sha256": "b" * 64, "archive_bytes": 1}
        variants = [
            {"schema_version": 1, "holds": [str(sealed)], "candidates": [{"path": str(self.file), "evidence": [bad_ref]}]},
            {"schema_version": 1, "required_local_sealed_desktop_inputs": [{"path": str(sealed), "optional_large_manifest": True}], "unknown": True},
        ]
        for index, context_value in enumerate(variants):
            context = self.root / ("malformed-held-context-" + str(index) + ".json")
            context.write_text(json.dumps(context_value))
            with redirect_stderr(io.StringIO()):
                status = inventory.main(["--candidate", str(self.file), "--context", str(context), "--report", str(sealed / "not-created.json")])
            self.assertEqual(status, 2)
            self.assertEqual(list(sealed.iterdir()), [])
            safe_report = self.output / ("malformed-held-result-" + str(index) + ".json")
            with redirect_stdout(io.StringIO()):
                status = inventory.main(["--candidate", str(self.file), "--context", str(context), "--report", str(safe_report)])
            self.assertEqual(status, 0)
            self.assertFalse(json.loads(safe_report.read_text())["candidates"][0]["removal_eligible"])

    def test_inaccessible_scope_stops_report_creation(self):
        report = self.output / "not-created.json"
        with patch.object(inventory, "path_scope", side_effect=PermissionError("private external error")):
            with self.assertRaises(OSError):
                inventory.write_report(str(report), {}, [str(self.file)])
        self.assertFalse(report.exists())

    def test_output_parent_drift_stops_report_creation(self):
        report = self.output / "not-created.json"
        with patch.object(inventory, "same_path", return_value=False):
            with self.assertRaisesRegex(inventory.BoundaryError, "report_parent_identity_drift"):
                inventory.write_report(str(report), {}, [str(self.file)])
        self.assertFalse(report.exists())

    def test_scope_depth_and_count_are_bounded_before_creation(self):
        with patch.object(inventory.os, "open") as opening:
            with self.assertRaisesRegex(inventory.BoundaryError, "scope_depth_exceeded"):
                inventory.path_scope("/" + "/".join(["deep"] * 129))
            opening.assert_not_called()
        report = self.output / "not-created.json"
        with self.assertRaisesRegex(inventory.BoundaryError, "scope_path_budget_exceeded"):
            inventory.write_report(str(report), {}, [str(self.file)] * 4097)
        self.assertFalse(report.exists())

    def test_protected_store_case_aliases_are_not_opened(self):
        secret = self.root / ".CODEX" / "AUTH.JSON"
        self.assertTrue(inventory.protected(str(secret)))
        with patch.object(inventory.os, "open") as opening:
            with self.assertRaisesRegex(inventory.BoundaryError, "protected_input_store"):
                inventory.read_regular(str(secret))
            opening.assert_not_called()

    def test_ambiguous_or_unreadable_context_cannot_write_inside_a_hidden_hold(self):
        context = self.root / "ambiguous.json"
        hidden_hold = str(self.candidate)
        context.write_text('{"schema_version":1,"holds":[' + json.dumps(hidden_hold) + '],"holds":[]}')
        report = self.candidate / "not-created.json"
        for source in [context, self.root / "not-present-context.json"]:
            errors = io.StringIO()
            with redirect_stderr(errors):
                result = inventory.main(["--candidate", str(self.file), "--context", str(source), "--report", str(report)])
            self.assertEqual(result, 2)
            self.assertIn("context_scope_unavailable", errors.getvalue())
            self.assertNotIn("Traceback", errors.getvalue())
            self.assertFalse(report.exists())

    def test_noncanonical_scopes_cannot_be_dropped_to_allow_a_report(self):
        report = self.candidate / "not-created.json"
        with redirect_stderr(io.StringIO()):
            status = inventory.main(["--candidate", str(self.candidate) + "/", "--report", str(report)])
        self.assertEqual(status, 2)
        self.assertFalse(report.exists())
        context = self.root / "bad-held-path.json"
        context.write_text(json.dumps({"schema_version": 1, "holds": [str(self.candidate) + "/"]}))
        errors = io.StringIO()
        with redirect_stderr(errors):
            status = inventory.main(["--candidate", str(self.file), "--context", str(context), "--report", str(report)])
        self.assertEqual(status, 2)
        self.assertIn("context_scope_unavailable", errors.getvalue())
        self.assertFalse(report.exists())

    def test_invalid_scope_container_shapes_cannot_hide_held_paths(self):
        context = self.root / "bad-scope-shape.json"
        report = self.candidate / "not-created.json"
        for fields in [{"holds": str(self.candidate)},
                       {"holds": {"path": str(self.candidate)}},
                       {"required_local_sealed_desktop_inputs": [str(self.candidate)]},
                       {"candidates": {"path": str(self.candidate)}}]:
            context.write_text(json.dumps({"schema_version": 1, **fields}))
            with redirect_stderr(io.StringIO()):
                result = inventory.main(["--candidate", str(self.file), "--context", str(context), "--report", str(report)])
            self.assertEqual(result, 2)
            self.assertFalse(report.exists())


if __name__ == "__main__":
    unittest.main()
