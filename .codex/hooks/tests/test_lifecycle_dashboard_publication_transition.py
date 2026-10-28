"""Publication effects occur only in disposable synthetic Git repositories."""
import copy
import hashlib
import json
import os
from pathlib import Path
import sys
import unittest
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
sys.path.insert(0, str(Path(__file__).resolve().parent))
import lifecycle_closure as c
import lifecycle_dashboard_publication_transition as t
import lifecycle_dashboard_verification_repair as repair
import post_increment_gate as g
import test_lifecycle_dashboard_verification_repair as fixture


class DashboardPublicationTransitionTests(unittest.TestCase):
    def setUp(self):
        self.previous = fixture.DashboardRepairTests(methodName="runTest")
        self.addCleanup(self.previous.doCleanups)
        self.previous.setUp(); self.previous.admit(); self.previous.final()
        self.root, self.f = self.previous.root, self.previous.f
        g.finalize_gate(self.root, repair.SUCCESSOR, repair.REPORT)
        self.state = g.read_state(self.root)
        self.raw_state = g.state_path(self.root).read_bytes()
        baseline = c.manifest(self.root)
        # Issued raw JSON preserves insertion order; canonical JSON sorts keys.
        # Deliberately exercise both identities instead of making them equal.
        publication = {"status": "Synthetic issued manifest", "groups": {name: [] for name in t.GROUPS}}
        self.ignore_before = (self.root / ".prettierignore").read_bytes()
        product_name = next(iter(repair.previous.previous.PRODUCT))
        publication["groups"]["dashboard_product"] = [{"path": product_name, "sha256": baseline[product_name]["sha256"]}]
        publication["groups"]["shared_project_memory"] = [{"path": "DECISIONS.md", "sha256": baseline["DECISIONS.md"]["sha256"]}]
        # Mirror the retained snapshot's selected lifecycle namespaces. Complete
        # custody backups are retained separately, not passed as repository paths
        # through the shared validator (which intentionally rejects `.git`).
        records = {}
        for selected in sorted((self.root / ".codex/state").iterdir()):
            if selected.name != "closures" and not selected.name.startswith("dashboard-"):
                continue
            sources = sorted(selected.rglob("*")) if selected.is_dir() else [selected]
            for path in sources:
                self.assertFalse(path.is_symlink())
                if path.is_file():
                    name = path.relative_to(self.root).as_posix()
                    g.validate_relative_path(name)
                    records[name] = {"sha256": c.digest(path.read_bytes()), "mode": t.mode(path)}
        self.backup_records = {
            path.relative_to(self.root).as_posix(): (c.digest(path.read_bytes()), t.mode(path))
            for name in ["backup", "second-backup", "repair-backup"]
            for path in (self.root / ".codex/state" / name).rglob("*") if path.is_file()
        }
        self.original_index = c.git_index_path(self.root).read_bytes()
        self.assertEqual(c.digest(self.original_index), repair.INDEX_SHA256)
        self.addCleanup(self.assert_backup_custody)
        self.base = {"schema": 1, "root": str(self.root), "branch": t.original.BRANCH,
            "head": t.original.HEAD, "raw_state": self.raw_state.decode(),
            "index_hex": c.git_index_path(self.root).read_bytes().hex(), "manifest": baseline,
            "protected_records": records, "publication_manifest": publication,
            "publication_sha256": c.digest((json.dumps(publication, indent=2) + "\n").encode()), "retained_acceptance": {}}
        for name, value in {"SNAPSHOT_SHA256": c.digest(c.encoded(self.base)), "STATE_SHA256": c.digest(self.raw_state),
                            "PUBLICATION_SHA256": self.base["publication_sha256"],
                            "PUBLICATION_CANONICAL_SHA256": c.digest(c.encoded(publication)),
                            "IGNORE_BASE_SHA256": c.digest(self.ignore_before)}.items():
            patcher = mock.patch.object(t, name, value); patcher.start(); self.addCleanup(patcher.stop)
        self.f.write_json(t.BASE.as_posix(), self.base)
        for name in t.CODE | {t.PLAN, t.BOUNDARY_ASSESSMENT}:
            self.f.write(name, "# Synthetic transition maintenance\n")
        pins = {}
        for name in t.FROZEN_REPORTS:
            self.f.write(name, "# Synthetic immutable assessment: " + name + "\n")
            pins[name] = c.digest((self.root / name).read_bytes())
        patcher = mock.patch.object(t, "FROZEN_REPORTS", pins)
        patcher.start(); self.addCleanup(patcher.stop)
        self.f.write(".prettierignore", (self.ignore_before + t.IGNORE_SUFFIX).decode())
        self.f.write("DECISIONS.md", "Separate synthetic publication authorization.\n")
        # Build only the fixed new readiness report; the older helper would also
        # leave an out-of-scope October 20 report in the candidate inventory.
        manifest = copy.deepcopy(g._extract_manifest((self.root / repair.REPORT).read_text()))
        manifest.update(increment_id=t.REVIEW_ID, quality_gate="PASS", findings=[])
        manifest["manual_verification"] = [{"check": n, "required": True, "status": "Passed"} for n in
            ["Preservation and scope review", "Architecture, security, code-health and readiness review"]]
        self.f.write_review(t.REVIEW, manifest)
        current = c.manifest(self.root)
        groups = {name: {row["path"]: current[row["path"]]["sha256"] for row in publication["groups"][name]} for name in t.GROUPS}
        shared = t.DOCS - {"AGENTS.md", "ENGINEERING_GUIDE.md", t.PLAN, t.REVIEW, t.ASSESSMENT}
        for name in c.changed(baseline, current):
            group = "shared_project_memory" if name in shared else "governance"
            groups[group][name] = current[name]["sha256"]
        checks = {}
        for label, argv in t.CHECKS.items():
            log = t.STORE / "evidence" / (label + ".log")
            self.f.write(log.as_posix(), "Synthetic output, not an actual full verification.\n")
            proof = {"label": label, "argv": argv, "cwd": str(self.root),
                     "input_bindings": {n: v for n, v in current.items() if n != t.REVIEW},
                     "exit_status": 0, "timed_out": False, "complete_capture": True,
                     "cleanup": {"root_reaped": True, "group_absent": True}, "elapsed_seconds": 1,
                     "log": {"path": log.as_posix(), "sha256": c.digest((self.root/log).read_bytes())}}
            path = t.STORE / "evidence" / (label + ".json")
            self.f.write_json(path.as_posix(), proof)
            checks[label] = {"path": path.as_posix(), "sha256": c.digest((self.root/path).read_bytes())}
        self.req = {"schema": 1, "increment_id": t.MAINTENANCE, "accepted_sha256": t.SNAPSHOT_SHA256,
                    "parent": t.original.HEAD, "manifest": current, "owner_authorization": self.f.auth(),
                    "plan_sha256": c.digest((self.root/t.PLAN).read_bytes()),
                    "review_sha256": c.digest((self.root/t.REVIEW).read_bytes()), "checks": checks,
                    "publication_manifest": {"original_sha256": t.PUBLICATION_SHA256, "groups": groups}}
        self.product = product_name
        self.assert_valid_baseline()

    def assert_backup_custody(self):
        for name, bound in self.backup_records.items():
            path = self.root / name
            self.assertEqual((c.digest(path.read_bytes()), t.mode(path)), bound, name)

    def assert_valid_baseline(self):
        self.assertEqual(t.accepted(self.root, self.state), self.base)
        self.assertTrue(t.contract(self.root, self.req, self.base))
        self.assertEqual(c.git_index_path(self.root).read_bytes(), self.original_index)

    def seal(self):
        self.f.write_json(t.REQUEST.as_posix(), self.req)
        t.seal(self.root, t.REQUEST.as_posix())

    def reject(self, reason):
        with self.assertRaisesRegex(g.GateError, reason): self.seal()
        self.assertFalse((self.root/t.RECEIPT).exists())
        self.assertEqual(g.state_path(self.root).read_bytes(), self.raw_state)

    def test_selected_history_and_separate_index_custody(self):
        self.assertTrue(self.base["protected_records"])
        self.assertTrue(self.backup_records)
        self.assertTrue(any(".git/index" in name for name in self.backup_records))
        self.assertFalse(set(self.backup_records) & set(self.base["protected_records"]))
        self.assertEqual(bytes.fromhex(self.base["index_hex"]), self.original_index)
        for name in self.base["protected_records"]:
            self.assertIn(Path(name).parts[2], {"closures"} | {
                p.name for p in (self.root / ".codex/state").iterdir() if p.name.startswith("dashboard-")})
            g.validate_relative_path(name)

    def test_only_fixed_readiness_report_is_added(self):
        old = "docs/reviews/2026-10-20-" + t.REVIEW_ID + "-post-increment-review.md"
        self.assertFalse((self.root / old).exists())
        delta = c.changed(self.base["manifest"], self.req["manifest"])
        self.assertIn(t.REVIEW, delta)
        self.assertFalse(delta - t.ALLOWED)

    def test_issued_raw_and_canonical_manifest_identities_are_distinct(self):
        self.assertNotEqual(t.PUBLICATION_SHA256, t.PUBLICATION_CANONICAL_SHA256)
        raw = (json.dumps(self.base["publication_manifest"], indent=2) + "\n").encode()
        self.assertEqual(c.digest(raw), self.base["publication_sha256"])
        self.assertEqual(json.loads(raw), self.base["publication_manifest"])
        self.assertEqual(c.digest(c.encoded(json.loads(raw))), t.PUBLICATION_CANONICAL_SHA256)
        self.assert_valid_baseline()
        for name in ["PUBLICATION_SHA256", "PUBLICATION_CANONICAL_SHA256"]:
            with self.subTest(pin=name), mock.patch.object(t, name, "0" * 64):
                with self.assertRaisesRegex(g.GateError, "publication original manifest drift"):
                    t.accepted(self.root, self.state)

    def test_rebound_snapshot_cannot_substitute_manifest_content_or_raw_identity(self):
        original = (self.root / t.BASE).read_bytes()
        for field in ["publication_manifest", "publication_sha256"]:
            changed = copy.deepcopy(self.base)
            if field == "publication_manifest": changed[field]["status"] = "Substituted content"
            else: changed[field] = t.PUBLICATION_CANONICAL_SHA256
            raw = c.encoded(changed)
            try:
                (self.root / t.BASE).write_bytes(raw)
                with mock.patch.object(t, "SNAPSHOT_SHA256", c.digest(raw)):
                    with self.assertRaisesRegex(g.GateError, "publication original manifest drift"):
                        t.accepted(self.root, self.state)
            finally: (self.root / t.BASE).write_bytes(original)
        self.assert_valid_baseline()

    def test_fixed_additions_belong_only_to_governance(self):
        groups = self.req["publication_manifest"]["groups"]
        self.assertEqual(t.GOVERNANCE_ADDITIONS,
                         {".prettierignore", t.BOUNDARY_ASSESSMENT} | (set(t.FROZEN_REPORTS) - {t.ASSESSMENT}))
        self.assertFalse(t.GOVERNANCE_ADDITIONS & t.DOCS)
        for name in t.GOVERNANCE_ADDITIONS:
            self.assertIn(name, groups["governance"])
            self.assertNotIn(name, groups["shared_project_memory"])
        self.assert_valid_baseline()

    def test_omitted_duplicated_or_misgrouped_additions_rejected(self):
        baseline = copy.deepcopy(self.req)
        name = next(iter(t.GOVERNANCE_ADDITIONS))
        for kind in ["omitted", "duplicated", "misgrouped"]:
            self.req = copy.deepcopy(baseline)
            groups = self.req["publication_manifest"]["groups"]
            value = groups["governance"][name]
            if kind != "duplicated": del groups["governance"][name]
            if kind != "omitted": groups["shared_project_memory"][name] = value
            self.reject("publication scope omitted/duplicated")
        self.req = baseline

    def test_exact_ignore_delta_rejects_broader_duplicate_missing_or_prefix_changes(self):
        path = self.root / ".prettierignore"; raw = path.read_bytes()
        variants = [raw + b"\n**/*.md\n", raw + t.IGNORE_SUFFIX,
                    raw.replace(t.ASSESSMENT.encode(), b"other.md"), b"# changed\n" + raw]
        try:
            for candidate in variants:
                with self.subTest(candidate=c.digest(candidate)):
                    path.write_bytes(candidate)
                    self.assertIsInstance(t.accepted(self.root, self.state), dict)
                    with self.assertRaisesRegex(g.GateError, "publication exact formatting amendment"):
                        t.maintenance_history(self.root, self.base, c.manifest(self.root))
        finally: path.write_bytes(raw)
        self.assert_valid_baseline()

    def test_original_ignore_binding_is_not_caller_selected(self):
        changed = copy.deepcopy(self.base)
        changed["manifest"][".prettierignore"]["sha256"] = "0" * 64
        with self.assertRaisesRegex(g.GateError, "publication original ignore binding"):
            t.maintenance_history(self.root, changed, self.req["manifest"])

    def test_frozen_reports_reject_content_mode_and_omission(self):
        for name in t.FROZEN_REPORTS:
            path = self.root / name; raw = path.read_bytes(); old_mode = t.mode(path)
            try:
                path.write_bytes(raw + b"historical drift\n")
                with self.assertRaisesRegex(g.GateError, "publication immutable assessment drift"):
                    t.maintenance_history(self.root, self.base, c.manifest(self.root))
                path.write_bytes(raw); path.chmod(0o755)
                with self.assertRaisesRegex(g.GateError, "publication immutable assessment drift"):
                    t.maintenance_history(self.root, self.base, c.manifest(self.root))
                path.chmod(old_mode)
                manifest = copy.deepcopy(self.req["manifest"]); del manifest[name]
                with self.assertRaisesRegex(g.GateError, "publication immutable assessment drift"):
                    t.maintenance_history(self.root, self.base, manifest)
            finally: path.write_bytes(raw); path.chmod(old_mode)
        self.assert_valid_baseline()

    def test_rebound_request_cannot_admit_unrelated_assessment_path(self):
        name = "docs/reviews/unapproved-assessment.md"
        self.f.write(name, "# Unapproved\n")
        candidate = copy.deepcopy(self.req); candidate["manifest"] = c.manifest(self.root)
        with self.assertRaisesRegex(g.GateError, "publication unrelated/deleted scope"):
            t.contract(self.root, candidate, self.base)

    def test_unsafe_protected_path_is_still_rejected(self):
        raw = (self.root / t.BASE).read_bytes()
        changed = copy.deepcopy(self.base)
        name = ".codex/state/backup/before/.git/index"
        path = self.root / name
        changed["protected_records"][name] = {"sha256": c.digest(path.read_bytes()), "mode": t.mode(path)}
        altered = c.encoded(changed)
        try:
            (self.root / t.BASE).write_bytes(altered)
            with mock.patch.object(t, "SNAPSHOT_SHA256", c.digest(altered)):
                with self.assertRaisesRegex(g.GateError, "repository-relative path is unsafe"):
                    t.accepted(self.root, self.state)
        finally:
            (self.root / t.BASE).write_bytes(raw)

    def test_rebound_snapshot_cannot_change_original_index_binding(self):
        raw = (self.root / t.BASE).read_bytes()
        changed = copy.deepcopy(self.base)
        changed["index_hex"] = (self.original_index + b"drift").hex()
        altered = c.encoded(changed)
        try:
            (self.root / t.BASE).write_bytes(altered)
            with mock.patch.object(t, "SNAPSHOT_SHA256", c.digest(altered)):
                with self.assertRaisesRegex(g.GateError, "publication accepted workspace/index"):
                    t.accepted(self.root, self.state)
        finally:
            (self.root / t.BASE).write_bytes(raw)

    def test_prepared_staged_committed_and_stop(self):
        self.seal()
        self.assertEqual(t.validate(self.root, self.state)["phase"], "prepared")
        self.f.git("add", "--all")
        self.assertEqual(t.validate(self.root, self.state)["phase"], "staged")
        self.f.git("commit", "-m", "feat: synthetic dashboard candidate")
        self.assertEqual(t.validate(self.root, self.state)["phase"], "committed")
        self.assertTrue(g.validate_completed_state(self.root, self.state))
        self.assertFalse(g.evaluate_stop_payload({"cwd": str(self.root), "hook_event_name": "Stop", "stop_hook_active": False}).should_continue)
        self.assertEqual(g.state_path(self.root).read_bytes(), self.raw_state)
        self.assertEqual(g.redacted_status(self.root)["publication"]["phase"], "committed")

    def test_partial_staging_rejected(self):
        self.seal(); self.f.git("add", "DECISIONS.md")
        with self.assertRaisesRegex(g.GateError, "publication partial/unrelated staging"): t.validate(self.root, self.state)
        self.assertFalse(g.validate_completed_state(self.root, self.state))

    def test_empty_extra_commit_rejected(self):
        self.seal(); self.f.git("add", "--all"); self.f.git("commit", "-m", "synthetic")
        self.f.git("commit", "--allow-empty", "-m", "unexpected extra commit")
        with self.assertRaisesRegex(g.GateError, "publication unsupported commit/tree/parents"): t.validate(self.root, self.state)

    def test_product_mode_and_unrelated_scope_rejected(self):
        for name in [self.product, "unrelated.txt"]:
            with self.subTest(name=name):
                candidate = self.root/name; old = candidate.read_bytes() if candidate.exists() else None
                self.f.write(name, "unapproved drift\n")
                self.reject("publication candidate inventory")
                if old is None: candidate.unlink()
                else: candidate.write_bytes(old)
        path = self.root/self.product; old = path.stat().st_mode
        path.chmod(0o755)
        try: self.reject("publication candidate inventory")
        finally: path.chmod(old)

    def test_accepted_state_report_history_and_base_drift_rejected(self):
        for name, reason in [(g.STATE_RELATIVE_PATH.as_posix(), "publication accepted state drift"),
                (repair.REPORT, "publication accepted report drift"),
                (next(iter(self.base["protected_records"])), "publication historical record drift"),
                (t.BASE.as_posix(), "publication accepted snapshot drift")]:
            path = self.root/name; raw = path.read_bytes(); path.write_bytes(raw+b" ")
            try:
                with self.assertRaisesRegex(g.GateError, reason): t.accepted(self.root, g.read_state(self.root))
            finally: path.write_bytes(raw)
            self.assert_valid_baseline()

    def test_historical_mode_and_missing_record_rejected(self):
        path = self.root / next(iter(self.base["protected_records"]))
        old_mode = t.mode(path)
        path.chmod(0o600 if old_mode != 0o600 else 0o400)
        try:
            with self.assertRaisesRegex(g.GateError, "publication historical record drift"):
                t.accepted(self.root, self.state)
        finally: path.chmod(old_mode)
        # Keep bytes/mode in place while simulating an unavailable protected file.
        original_read = t.read
        def unavailable(candidate):
            if candidate == path: raise FileNotFoundError("synthetic protected loss")
            return original_read(candidate)
        with mock.patch.object(t, "read", side_effect=unavailable):
            with self.assertRaisesRegex(g.GateError, "malformed or unavailable evidence"):
                t.accepted(self.root, self.state)

    def test_authority_plan_review_and_request_rejected(self):
        before = copy.deepcopy(self.req)
        for key, value in [("schema", True), ("increment_id", "other"), ("parent", "0"*40),
                           ("accepted_sha256", "0"*64), ("plan_sha256", "0"*64), ("review_sha256", "0"*64)]:
            self.req = copy.deepcopy(before); self.req[key] = value
            reason = "publication frozen plan drift" if key == "plan_sha256" else (
                "publication readiness drift" if key == "review_sha256" else "publication request identity/ancestry")
            self.reject(reason)
        self.req = copy.deepcopy(before); self.req["owner_authorization"]["approved"] = False
        self.reject("recorded owner authorization required")

    def test_checks_cannot_be_omitted_downgraded_or_rebound(self):
        for label in t.CHECKS:
            saved = self.req["checks"].pop(label); self.reject("invalid record fields"); self.req["checks"][label] = saved
        path = self.root/self.req["checks"]["full"]["path"]
        baseline = path.read_bytes(); proof = json.loads(baseline)
        for key, value in [("exit_status", 1), ("complete_capture", False), ("timed_out", True),
                           ("input_bindings", {}), ("cleanup", {"root_reaped":False,"group_absent":True})]:
            changed = dict(proof); changed[key] = value
            path.write_bytes(c.encoded(changed)); old = self.req["checks"]["full"]["sha256"]
            self.req["checks"]["full"]["sha256"] = c.digest(path.read_bytes()); self.reject("publication check incomplete/stale/failed")
            self.req["checks"]["full"]["sha256"] = old; path.write_bytes(baseline)

    def test_replay_and_interrupted_receipt_consumed(self):
        self.seal()
        with self.assertRaisesRegex(g.GateError, "publication replay/request identity"): self.seal()
        (self.root/t.RECEIPT).write_bytes(b"{")
        with self.assertRaisesRegex(g.GateError, "malformed or unavailable evidence"): t.validate(self.root, self.state)
        with self.assertRaisesRegex(g.GateError, "publication replay/request identity"): self.seal()

    def test_post_effect_failure_leaves_receipt_and_blocks_retry(self):
        with mock.patch.object(t, "validate", side_effect=g.GateError("synthetic post-effect failure")) as injected:
            with self.assertRaisesRegex(g.GateError, "synthetic post-effect failure"): self.seal()
            injected.assert_called_once_with(self.root, self.state)
        self.assertTrue((self.root/t.RECEIPT).exists())
        receipt = (self.root/t.RECEIPT).read_bytes()
        request = (self.root/t.REQUEST).read_bytes()
        self.assertEqual(receipt, c.encoded({"request_sha256": c.digest(request), "request": self.req}))
        with mock.patch.object(t, "validate") as not_reached:
            with self.assertRaisesRegex(g.GateError, "publication replay/request identity"): self.seal()
            not_reached.assert_not_called()
        self.assertEqual((self.root/t.RECEIPT).read_bytes(), receipt)

    def test_request_scope_and_log_drift(self):
        self.req["publication_manifest"]["groups"]["dashboard_product"] = {}; self.reject("publication scope omitted/duplicated")
        self.req["publication_manifest"]["groups"]["dashboard_product"][self.product] = self.req["manifest"][self.product]["sha256"]
        path = self.root/t.STORE/"evidence/full.log"; path.write_bytes(b"changed")
        self.reject("publication proof drift")

    def test_hidden_index_flag_on_tracked_baseline_and_restoration(self):
        tracked = "src/protected.txt"
        self.assertEqual(self.f.git("ls-files", "--error-unmatch", "--", tracked).stdout.decode().strip(), tracked)
        self.assertEqual(self.f.git("ls-files", "--", self.product).stdout, b"")
        self.seal()
        self.assertEqual(t.validate(self.root, self.state)["phase"], "prepared")
        self.assertEqual(c.git_index_path(self.root).read_bytes(), self.original_index)
        self.f.git("update-index", "--assume-unchanged", tracked)
        with self.assertRaisesRegex(g.GateError, "publication index flags/stage"): t.validate(self.root, self.state)
        self.f.git("update-index", "--no-assume-unchanged", tracked)
        self.assertEqual(t.validate(self.root, self.state)["phase"], "prepared")
        # Git can invalidate derived TREE cache metadata while clearing a flag.
        # Preserve entry identity/content/mode and strict flag/checksum validation,
        # not the incidental cache encoding. The original raw index stays retained.
        self.assertEqual(t.index_semantics(c.git_index_path(self.root).read_bytes()),
                         t.index_semantics(self.original_index))

    def test_wrong_branch_rejected_independently(self):
        self.seal()
        self.assertEqual(t.validate(self.root, self.state)["phase"], "prepared")
        self.f.git("checkout", "-b", "codex/not-dashboard")
        with self.assertRaisesRegex(g.GateError, "publication checkout identity"): t.validate(self.root, self.state)

    def test_malformed_duplicate_nonfinite_and_oversized(self):
        for raw in [b'{"a":1,"a":2}', b'{"a":NaN}', b'{', b' '* (t.MAX_BYTES+1)]:
            with self.assertRaises((g.GateError, ValueError)): t.parse(raw)

    def test_symlink_and_unsafe_mode(self):
        target = self.root/"private-proof"; target.write_text("{}")
        link = self.root/"proof-link"; link.symlink_to(target)
        with self.assertRaises(g.GateError): t.read(link)
        target.chmod(0o666)
        with self.assertRaises(g.GateError): t.read(target)

    def test_absent_transition_preserves_legacy_validation(self):
        with mock.patch.object(t, "present", return_value=False), mock.patch.object(repair, "record", side_effect=g.GateError("legacy check")):
            self.assertFalse(g.validate_completed_state(self.root, self.state))
        self.assertTrue(g.build_parser().parse_args(["seal-dashboard-publication", "--request", t.REQUEST.as_posix()]))


if __name__ == "__main__": unittest.main()
