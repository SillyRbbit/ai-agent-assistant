"""Fixed pinned-publication effects are confined to disposable synthetic Git."""
import copy
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import lifecycle_dashboard_pinned_publication as p
import lifecycle_closure as c
import post_increment_gate as g


class PinnedPublicationTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(prefix="pinned-fixture-", dir=os.environ.get("TMPDIR"))
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name).resolve()
        self.home = self.root.parent / (self.root.name + "-home")
        # HOME stays inside the disposable fixture, excluded from Git inventory.
        self.home = self.root / ".git" / "private-home"
        self.env = {"PATH": "/usr/bin:/bin", "HOME": str(self.home), "LC_ALL": "C", "LANG": "C",
                    "GIT_CONFIG_NOSYSTEM": "1", "GIT_CONFIG_GLOBAL": "/dev/null",
                    "GIT_TERMINAL_PROMPT": "0", "GIT_CONFIG_SYSTEM": "/dev/null",
                    "GIT_AUTHOR_NAME": "Synthetic", "GIT_AUTHOR_EMAIL": "synthetic@example.invalid",
                    "GIT_COMMITTER_NAME": "Synthetic", "GIT_COMMITTER_EMAIL": "synthetic@example.invalid"}
        self.git("init", "-q", "-b", p.BRANCH)
        self.home.mkdir(mode=0o700)
        self.write(".gitignore", ".codex/state/\n")
        for name in ["product.txt", "DECISIONS.md", p.PLAN, "historical.md", "upstream.txt"]:
            self.write(name, "Synthetic baseline " + name + "\n")
        self.git("add", "."); self.git("commit", "-qm", "test: original")
        original = self.git("rev-parse", "HEAD").strip()
        self.write("upstream.txt", "Published memory/Tasks/security fixture\n")
        self.write("package-lock.json", '{"vitest":"4.1.12"}\n')
        storage_original = b'Synthetic published storage test\n'
        storage_current = b'Synthetic explicit-mode storage test\n'
        self.write(p.STORAGE_TEST_PATH, storage_original)
        storage_oid = hashlib.sha1(
            b'blob ' + str(len(storage_original)).encode() + b'\0' + storage_original).hexdigest()
        self.git("add", "."); self.git("commit", "-qm", "test: pinned upstream")
        upstream = self.git("rev-parse", "HEAD").strip()
        upstream_tree = self.git("rev-parse", "HEAD^{tree}").strip()
        self.git("reset", "--hard", original)  # Disposable fixture only.
        self.raw_index = c.git_index_path(self.root).read_bytes()
        baseline = c.manifest(self.root)
        publication = {"status": "Synthetic historical only", "groups": {name: [] for name in sorted(p.prior.GROUPS)}}
        publication["groups"]["dashboard_product"] = [{"path": "product.txt", "sha256": baseline["product.txt"]["sha256"]}]
        publication["groups"]["shared_project_memory"] = [{"path": "DECISIONS.md", "sha256": baseline["DECISIONS.md"]["sha256"]}]
        issued_raw = (json.dumps(publication, indent=2) + "\n").encode()
        self.assertNotEqual(issued_raw, c.encoded(publication))
        self.assertNotEqual(c.digest(issued_raw), c.digest(c.encoded(publication)))
        self.state = {"schema_version": 8, "status": "complete", "increment_id": p.repair.SUCCESSOR,
                      "workspace_fingerprint": c.fingerprint(baseline), "report_path": "historical.md",
                      "report_sha256": baseline["historical.md"]["sha256"]}
        self.write_json(".codex/state/post_increment_gate.json", self.state)
        self.write(".codex/state/dashboard-history/failure.json", "Immutable synthetic FAIL/Blocked\n")
        history = ".codex/state/dashboard-history/failure.json"
        self.base = {"schema": 1, "root": str(self.root), "branch": p.BRANCH, "head": original,
                     "raw_state": c.encoded(self.state).decode(), "index_hex": self.raw_index.hex(),
                     "manifest": baseline, "protected_records": {history: {
                         "sha256": c.digest((self.root/history).read_bytes()), "mode": 0o644}},
                     "publication_manifest": publication,
                     "publication_sha256": c.digest(issued_raw),
                     "retained_acceptance": {"four": 4, "nine": 9, "ten": 10}}
        candidate = {name: baseline[name] for name in ["product.txt", "DECISIONS.md", p.PLAN]}
        for name, value in {"ORIGINAL": original, "UPSTREAM": upstream, "UPSTREAM_TREE": upstream_tree,
                            "UPSTREAM_PARENT": original, "ROOT_SHA256": c.digest(str(self.root).encode()),
                            "SNAPSHOT_SHA256": c.digest(c.encoded(self.base)), "STATE_SHA256": c.digest(c.encoded(self.state)),
                            "INDEX_SHA256": c.digest(self.raw_index), "HISTORY_COUNT": 1,
                            "RAW_MANIFEST_SHA256": self.base["publication_sha256"],
                            "CANONICAL_MANIFEST_SHA256": c.digest(c.encoded(publication)),
                            "CANDIDATE": candidate, "OVERLAPS": set(),
                            "EDITABLE": {"DECISIONS.md", p.PLAN} | p.NEW,
                            "STORAGE_TEST_UPSTREAM_SHA256": c.digest(storage_original),
                            "STORAGE_TEST_UPSTREAM_OID": storage_oid,
                            "STORAGE_TEST_CURRENT_SHA256": c.digest(storage_current),
                            "STORAGE_TEST_CURRENT_OID": hashlib.sha1(b'blob ' + str(len(storage_current)).encode() + b'\0' + storage_current).hexdigest()}.items():
            patch = mock.patch.object(p, name, value); patch.start(); self.addCleanup(patch.stop)
        # Full historical schema validation belongs to unchanged legacy coverage.
        # This fixture keeps the real new reader, pins, history and index checks.
        patch = mock.patch.object(g, "validate_state", lambda state: None)
        patch.start(); self.addCleanup(patch.stop)
        patch = mock.patch.object(p.prior, "maintenance_history", lambda *args: None)
        patch.start(); self.addCleanup(patch.stop)
        self.write_json(p.BASE, self.base)
        self.write(p.STORAGE_TEST_PATH, storage_current)
        self.write("upstream.txt", "Published memory/Tasks/security fixture\n")
        self.write("package-lock.json", '{"vitest":"4.1.12"}\n')
        for name in p.NEW - {p.REVIEW}: self.write(name, "# Synthetic maintenance\n")
        self.write("DECISIONS.md", "Owner synthetic pinned authorization\n")
        report = {"increment_id": p.REVIEW_ID, "quality_gate": "PASS", "findings": [],
                  "manual_verification": [{"check": name, "required": True, "status": "Passed"} for name in
                      ["Preservation and scope review", "Architecture, security, code-health and readiness review"]]}
        self.write(p.REVIEW, "# Synthetic readiness — not real verification\n\n" + g.MANIFEST_START + json.dumps(report) + g.MANIFEST_END + "\n")
        manifest = c.manifest(self.root)
        groups = {group: {row["path"]: manifest[row["path"]]["sha256"] for row in rows}
                  for group, rows in publication["groups"].items()}
        groups["governance"] = {n: manifest[n]["sha256"] for n in (set(candidate) - {"product.txt", "DECISIONS.md"}) | p.NEW | {p.STORAGE_TEST_PATH}}
        original_tree = {n: (m, oid) for n, m, oid in p.prior.tree(self.root, original)}
        inherited = {n: {"mode": m, "oid": oid} for n, m, oid in p.prior.tree(self.root, upstream)
                     if n not in candidate and n != p.STORAGE_TEST_PATH and original_tree.get(n) != (m, oid)}
        self.req = {"schema": 1, "profile": p.PROFILE, "increment_id": p.INCREMENT,
                    "original_parent": original, "upstream_parent": upstream, "upstream_tree": upstream_tree,
                    "accepted_sha256": p.SNAPSHOT_SHA256, "original_raw_manifest_sha256": p.RAW_MANIFEST_SHA256,
                    "original_canonical_manifest_sha256": p.CANONICAL_MANIFEST_SHA256,
                    "baseline_bindings": {n: v for n, v in manifest.items() if n != p.REVIEW},
                    "manifest": manifest, "publication_manifest": {"original_sha256": p.RAW_MANIFEST_SHA256,
                        "groups": groups, "upstream_inheritance": inherited,
                        "upstream_test_maintenance": {p.STORAGE_TEST_PATH: p.storage_test_maintenance(
                            {p.STORAGE_TEST_PATH: ('100644', storage_oid)}, manifest,
                            {p.STORAGE_TEST_PATH: ('100644', p.STORAGE_TEST_CURRENT_OID)})}},
                    "expected_tree": [list(row) for row in p.prior.expected_tree(self.root, manifest)],
                    "owner_authorization": {"approved": True, "source": "Synthetic pinned owner",
                        "decision_path": "DECISIONS.md", "decision_sha256": manifest["DECISIONS.md"]["sha256"]},
                    "plan_sha256": manifest[p.PLAN]["sha256"], "review_sha256": manifest[p.REVIEW]["sha256"], "checks": {}}
        self.make_proofs()
        self.assert_baseline()

    def git(self, *args):
        result = subprocess.run(["/usr/bin/git", "-c", "core.hooksPath=/dev/null", "-c", "gc.auto=0",
                                 "-c", "maintenance.auto=false", *args], cwd=self.root, env=self.env,
                                stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=5)
        self.assertLess(len(result.stdout) + len(result.stderr), 131072)
        self.assertEqual(result.returncode, 0, result.stderr.decode(errors="replace"))
        return result.stdout.decode()

    def write(self, name, raw):
        path = self.root / name; path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(raw.encode() if isinstance(raw, str) else raw); path.chmod(0o644)

    def write_json(self, name, value): self.write(name, c.encoded(value))

    def ref(self, name): return {"path": str(name), "sha256": c.digest((self.root/name).read_bytes())}

    def make_proofs(self):
        for label, argv in p.CHECKS.items():
            log = p.STORE / "evidence" / (label + ".log")
            self.write(log, "Synthetic proof only; no actual V invocation\n")
            item = {"label": label, "argv": argv, "cwd": str(self.root), "input_bindings": self.req["baseline_bindings"],
                    "exit_status": 0, "timed_out": False, "complete_capture": True,
                    "cleanup": {"root_reaped": True, "group_absent": True}, "elapsed_seconds": 1, "log": self.ref(log)}
            name = p.STORE / "evidence" / (label + ".json"); self.write_json(name, item)
            self.req["checks"][label] = self.ref(name)

    def assert_baseline(self):
        self.assertEqual(p.accepted(self.root, self.state), self.base)
        self.assertTrue(p.contract(self.root, self.req, self.base))
        self.assertEqual(c.git_index_path(self.root).read_bytes(), self.raw_index)

    def seal(self):
        self.write_json(p.REQUEST, self.req); p.seal(self.root, p.REQUEST.as_posix())

    def reject(self, reason):
        with self.assertRaisesRegex(g.GateError, reason): self.seal()
        self.assertFalse((self.root/p.phase_path("prepared", "intent")).exists())

    def change_proof(self, label, **changes):
        name = p.STORE / "evidence" / (label + ".json")
        item = json.loads((self.root/name).read_bytes()); item.update(changes)
        self.write_json(name, item); self.req["checks"][label] = self.ref(name)

    def test_distinct_historical_provenance_and_carry(self):
        self.assertNotEqual(p.RAW_MANIFEST_SHA256, p.CANONICAL_MANIFEST_SHA256)
        self.assertEqual(self.base["retained_acceptance"], {"four": 4, "nine": 9, "ten": 10})
        self.assert_baseline()

    def test_issued_manifest_order_is_explicit(self):
        publication = self.base['publication_manifest']
        self.assertEqual(list(publication), ['status', 'groups'])
        self.assertEqual(list(publication['groups']), sorted(p.prior.GROUPS))
        raw = (json.dumps(publication, indent=2) + "\n").encode()
        self.assertEqual(c.digest(raw), p.RAW_MANIFEST_SHA256)
        self.assertNotEqual(raw, c.encoded(publication))
        self.assertNotEqual(p.RAW_MANIFEST_SHA256, p.CANONICAL_MANIFEST_SHA256)

    def test_exact_fixture_maintenance_binding(self):
        name = p.FIXTURE_MAINTENANCE_PATH
        original = {'exists': True, 'mode': 0o644, 'sha256': p.FIXTURE_MAINTENANCE_ORIGINAL_SHA256}
        with mock.patch.dict(p.CANDIDATE, {name: original}):
            self.assertEqual(p.candidate_binding(name),
                             {'exists': True, 'mode': 0o644, 'sha256': p.FIXTURE_MAINTENANCE_SHA256})
            self.assertEqual(p.CANDIDATE[name], original)
            self.assertEqual(p.candidate_binding('product.txt'), p.CANDIDATE['product.txt'])
            for wrong in [dict(original, sha256='0' * 64), dict(original, mode=0o755),
                          dict(original, exists=1), dict(original, extra=True),
                          {'mode': 0o644, 'sha256': original['sha256']}]:
                with self.subTest(wrong=wrong), mock.patch.dict(p.CANDIDATE, {name: wrong}):
                    with self.assertRaisesRegex(g.GateError, 'original fixture maintenance binding'):
                        p.candidate_binding(name)

    def test_fixture_maintenance_current_identity_rejects_drift(self):
        name = p.FIXTURE_MAINTENANCE_PATH
        original = {'exists': True, 'mode': 0o644, 'sha256': p.FIXTURE_MAINTENANCE_ORIGINAL_SHA256}
        raw = Path(__file__).with_name('test_lifecycle_dashboard_publication.py').read_bytes()
        self.assertEqual(c.digest(raw), p.FIXTURE_MAINTENANCE_SHA256)
        self.write(name, raw)
        with mock.patch.dict(p.CANDIDATE, {name: original}):
            manifest = c.manifest(self.root)
            self.req['manifest'] = manifest
            self.req['baseline_bindings'] = {n: v for n, v in manifest.items() if n != p.REVIEW}
            self.req['expected_tree'] = [list(row) for row in p.prior.expected_tree(self.root, manifest)]
            self.req['publication_manifest']['groups']['governance'][name] = manifest[name]['sha256']
            self.make_proofs()
            self.assert_baseline()
            self.assertNotIn(name, p.EDITABLE)
            for kind in ('content', 'mode', 'omission'):
                with self.subTest(kind=kind):
                    if kind == 'content': self.write(name, raw + b'\n# drift\n')
                    elif kind == 'mode': (self.root / name).chmod(0o600)
                    else: (self.root / name).unlink()
                    self.req['manifest'] = c.manifest(self.root)
                    expected = 'complete combined scope' if kind == 'omission' else 'protected candidate drift'
                    with self.assertRaisesRegex(g.GateError, expected):
                        p.inventories(self.root, self.req, self.base)
                    self.write(name, raw)

    def test_exact_upstream_test_maintenance_provenance(self):
        name = p.STORAGE_TEST_PATH
        row = self.req['publication_manifest']['upstream_test_maintenance'][name]
        self.assertEqual(row['group'], 'governance')
        self.assertEqual(row['upstream']['oid'], p.STORAGE_TEST_UPSTREAM_OID)
        self.assertEqual(row['current']['sha256'], p.STORAGE_TEST_CURRENT_SHA256)
        self.assertIn(name, self.req['publication_manifest']['groups']['governance'])
        self.assertNotIn(name, self.req['publication_manifest']['upstream_inheritance'])
        self.assertNotIn(name, p.CANDIDATE)
        self.assertNotIn(name, p.EDITABLE)
        self.assert_baseline()

    def test_upstream_test_maintenance_rejects_current_drift(self):
        name = p.STORAGE_TEST_PATH
        raw = (self.root / name).read_bytes()
        saved = copy.deepcopy(self.req)
        for kind in ('content', 'mode', 'omission'):
            self.assert_baseline()
            with self.subTest(kind=kind):
                if kind == 'content': self.write(name, raw + b'drift')
                elif kind == 'mode': (self.root / name).chmod(0o600)
                else: (self.root / name).unlink()
                self.req['manifest'] = c.manifest(self.root)
                expected = 'complete combined scope' if kind == 'omission' else 'test maintenance current'
                with self.assertRaisesRegex(g.GateError, expected):
                    p.inventories(self.root, self.req, self.base)
                self.write(name, raw)
                self.req = copy.deepcopy(saved)

    def test_upstream_test_maintenance_rejects_rebound_provenance(self):
        name = p.STORAGE_TEST_PATH
        saved = copy.deepcopy(self.req['publication_manifest']['upstream_test_maintenance'])
        for wrong in ({}, {name: {}}, {name: saved[name], 'other.py': saved[name]}):
            self.assert_baseline()
            with self.subTest(wrong=wrong):
                self.req['publication_manifest']['upstream_test_maintenance'] = wrong
                with self.assertRaisesRegex(g.GateError, 'test maintenance provenance'):
                    p.inventories(self.root, self.req, self.base)
                self.req['publication_manifest']['upstream_test_maintenance'] = copy.deepcopy(saved)
        with mock.patch.object(p, 'STORAGE_TEST_UPSTREAM_OID', '0' * 40):
            with self.assertRaisesRegex(g.GateError, 'test maintenance origin'):
                p.inventories(self.root, self.req, self.base)

    def test_fixed_profile_and_parent(self):
        for key in ["profile", "original_parent", "upstream_parent", "upstream_tree", "accepted_sha256",
                    "original_raw_manifest_sha256", "original_canonical_manifest_sha256"]:
            before = self.req[key]; self.req[key] = "wrong"
            self.reject("pinned request identity"); self.req[key] = before

    def test_closed_schema_and_bool_schema(self):
        self.req["schema"] = True; self.reject("pinned request identity")
        self.req["schema"] = 1; self.req["override"] = True; self.reject("fields")

    def test_missing_history_and_mode_drift(self):
        name = next(iter(self.base["protected_records"]))
        (self.root/name).chmod(0o600)
        with self.assertRaisesRegex(g.GateError, "historical record drift"): p.accepted(self.root, self.state)

    def test_historical_bytes_drift(self):
        name = next(iter(self.base["protected_records"])); self.write(name, "PASS")
        with self.assertRaisesRegex(g.GateError, "historical record drift"): p.accepted(self.root, self.state)

    def test_missing_reference_does_not_fall_back_to_old_route(self):
        (self.root/p.BASE).unlink()
        with self.assertRaises(g.GateError): p.accepted(self.root, self.state)

    def test_bad_snapshot_and_duplicate_json(self):
        with self.assertRaises(g.GateError): p.parse(b'{"a":1,"a":2}')
        with self.assertRaises(g.GateError): p.parse(b'{"a":NaN}')
        self.write(p.BASE, "{}"); self.reject("pinned snapshot drift")

    def test_upstream_package_and_memory_preservation(self):
        self.write("package-lock.json", '{"vitest":"old"}\n')
        self.reject("pinned current inventory")

    def test_rebound_upstream_drift_still_rejected(self):
        self.write("upstream.txt", "unpublished")
        self.req["manifest"] = c.manifest(self.root)
        self.reject("pinned upstream inheritance drift")

    def test_protected_product_and_mode(self):
        self.write("product.txt", "new product")
        self.req["manifest"] = c.manifest(self.root)
        self.reject("pinned protected candidate drift")

    def test_unrelated_and_missing_paths(self):
        self.write("unrelated.txt", "not authorized")
        self.req["manifest"] = c.manifest(self.root)
        self.reject("pinned complete combined scope")

    def test_group_omission_duplicate_and_misclassification(self):
        groups = self.req["publication_manifest"]["groups"]; baseline = copy.deepcopy(groups)
        for wrong in ["omit", "duplicate", "move"]:
            groups.clear(); groups.update(copy.deepcopy(baseline)); value = groups["dashboard_product"]["product.txt"]
            if wrong != "duplicate": del groups["dashboard_product"]["product.txt"]
            if wrong != "omit": groups["governance"]["product.txt"] = value
            self.reject("pinned omitted/duplicate/misgrouped")

    def test_inheritance_not_dashboard_group(self):
        self.req["publication_manifest"]["upstream_inheritance"] = {}
        self.reject("pinned upstream attribution")

    def test_seven_proofs_required(self):
        del self.req["checks"]["hooks"]; self.reject("fields")

    def test_exact_argv_no_alias(self):
        self.change_proof("whitespace", argv=["/usr/bin/git", "diff", "--check"])
        self.reject("pinned incomplete/stale/failed proof")

    def test_complete_input_baseline_no_generic_omission(self):
        item = copy.deepcopy(self.req["baseline_bindings"]); del item["DECISIONS.md"]
        self.change_proof("docs", input_bindings=item); self.reject("pinned incomplete/stale/failed proof")

    def test_bool_exit_cleanup_and_deadline(self):
        for changes in [{"exit_status": True}, {"cleanup": {"root_reaped": 1, "group_absent": True}},
                        {"elapsed_seconds": 601}, {"timed_out": True}, {"complete_capture": False}]:
            self.make_proofs(); self.change_proof("docs", **changes)
            self.reject("pinned incomplete/stale/failed proof")

    def test_proof_hash_drift(self):
        self.write(p.STORE/"evidence/docs.log", "changed")
        self.reject("pinned proof drift")

    def test_fixed_proof_and_capture_roles(self):
        before = copy.deepcopy(self.req["checks"]["docs"])
        self.req["checks"]["docs"] = copy.deepcopy(self.req["checks"]["security"])
        self.reject("pinned fixed proof role")
        self.req["checks"]["docs"] = before
        self.change_proof("docs", log=self.ref(p.STORE/"evidence/security.log"))
        self.reject("pinned fixed capture role")

    def test_required_named_review_and_blocking_finding(self):
        baseline = (self.root/p.REVIEW).read_bytes()
        for kind in ["missing", "failed", "blocking"]:
            report = g._extract_manifest(baseline.decode())
            if kind == "missing": report["manual_verification"].pop()
            elif kind == "failed": report["quality_gate"] = "FAIL"
            else: report["findings"] = [{"blocks_completion": True}]
            self.write(p.REVIEW, g.MANIFEST_START + json.dumps(report) + g.MANIFEST_END)
            self.req["manifest"] = c.manifest(self.root)
            self.req["review_sha256"] = self.req["manifest"][p.REVIEW]["sha256"]
            self.req["expected_tree"] = [list(row) for row in p.prior.expected_tree(self.root, self.req["manifest"])]
            with self.assertRaises(g.GateError): self.seal()

    def test_wrong_review_or_missing_named_review(self):
        self.write(p.REVIEW, "not a passing report")
        self.req["manifest"] = c.manifest(self.root); self.req["review_sha256"] = c.digest((self.root/p.REVIEW).read_bytes())
        with self.assertRaises(g.GateError): self.seal()

    def test_prepared_and_replay(self):
        self.seal(); self.assertEqual(p.validate(self.root, self.state)["phase"], "prepared")
        with self.assertRaisesRegex(g.GateError, "pinned replay"): p.seal(self.root, p.REQUEST.as_posix())

    def test_all_four_phases(self):
        self.seal()
        token = p.prepare_effect(self.root, "staged"); self.git("add", "--all"); p.finish_effect(self.root, "staged", token)
        self.assertEqual(p.validate(self.root, self.state)["phase"], "staged")
        token = p.prepare_effect(self.root, "parent-aligned")
        self.git("update-ref", "refs/heads/" + p.BRANCH, p.UPSTREAM, p.ORIGINAL)
        p.finish_effect(self.root, "parent-aligned", token)
        self.assertEqual(p.validate(self.root, self.state)["phase"], "parent-aligned")
        token = p.prepare_effect(self.root, "committed"); self.git("commit", "-qm", "feat: synthetic dashboard")
        p.finish_effect(self.root, "committed", token)
        self.assertEqual(p.validate(self.root, self.state)["phase"], "committed")

    def test_partial_staging(self):
        self.seal(); self.git("add", "product.txt", "DECISIONS.md")
        with self.assertRaisesRegex(g.GateError, "pinned partial/hidden/unrelated index"): p.validate(self.root, self.state)

    def test_hidden_flag_on_tracked_baseline(self):
        self.seal(); self.assertIn("historical.md", self.git("ls-files").splitlines())
        index = c.git_index_path(self.root)
        original = index.read_bytes()
        baseline = p.prior.index_semantics(original)
        self.assertEqual(original, self.raw_index)
        self.assertEqual(p.validate(self.root, self.state)["phase"], "prepared")
        self.git("update-index", "--assume-unchanged", "historical.md")
        with self.assertRaisesRegex(g.GateError, "index flags/stage"): p.validate(self.root, self.state)
        self.git("update-index", "--no-assume-unchanged", "historical.md")
        self.assertEqual(p.prior.index_semantics(index.read_bytes()), baseline)
        # Git can rewrite a semantically identical index encoding. Restore only
        # this disposable fixture's original bytes; production remains strict.
        if index.read_bytes() != original:
            index.write_bytes(original)
        self.assertEqual(index.read_bytes(), original)
        self.assertEqual(p.validate(self.root, self.state)["phase"], "prepared")

    def test_raw_prepared_index_drift_rejected(self):
        self.seal()
        index = c.git_index_path(self.root)
        original = index.read_bytes()
        baseline = p.prior.index_semantics(original)
        self.assertEqual(p.validate(self.root, self.state)["phase"], "prepared")
        self.assertEqual(original[:8], b"DIRC\x00\x00\x00\x02")
        self.assertGreater(int.from_bytes(original[8:12], "big"), 0)
        drift = bytearray(original[:-20])
        # Change only the first entry's cached ctime, not its path, object,
        # stage, flags or mode. Keep a valid Git index checksum.
        drift[12:16] = ((int.from_bytes(drift[12:16], "big") + 1) % (1 << 32)).to_bytes(4, "big")
        drift = bytes(drift) + hashlib.sha1(drift).digest()
        self.assertNotEqual(drift, original)
        try:
            index.write_bytes(drift)
            self.assertEqual(p.prior.index_semantics(index.read_bytes()), baseline)
            with self.assertRaisesRegex(g.GateError, "pinned partial/hidden/unrelated index"):
                p.validate(self.root, self.state)
        finally:
            index.write_bytes(original)
        self.assertEqual(index.read_bytes(), original)
        self.assertEqual(p.validate(self.root, self.state)["phase"], "prepared")

    def test_wrong_branch_independent_baseline(self):
        self.seal(); self.git("checkout", "-qb", "codex/wrong")
        with self.assertRaisesRegex(g.GateError, "pinned checkout identity"): p.validate(self.root, self.state)

    def test_interrupted_effect_is_not_completion(self):
        self.seal(); p.prepare_effect(self.root, "staged")
        with self.assertRaisesRegex(g.GateError, "pinned interrupted/unexpected effect"): p.validate(self.root, self.state)
        with self.assertRaises(g.GateError): p.prepare_effect(self.root, "staged")

    def test_foreign_token_and_receipt_survival(self):
        self.seal(); original = (self.root/p.phase_path("prepared", "receipt")).read_bytes()
        token = p.prepare_effect(self.root, "staged")
        with self.assertRaisesRegex(g.GateError, "pinned consumed/foreign effect"): p.finish_effect(self.root, "staged", "0"*64)
        self.assertEqual((self.root/p.phase_path("prepared", "receipt")).read_bytes(), original)
        self.assertEqual(len(token), 64)

    def test_post_effect_receipt_retained_and_replay_rejected(self):
        self.write_json(p.REQUEST, self.req)
        with mock.patch.object(p, "validate", side_effect=g.GateError("synthetic post-effect")):
            with self.assertRaisesRegex(g.GateError, "synthetic post-effect"): p.seal(self.root, p.REQUEST.as_posix())
        issued = (self.root/p.phase_path("prepared", "receipt")).read_bytes()
        with self.assertRaises(g.GateError): p.seal(self.root, p.REQUEST.as_posix())
        self.assertEqual((self.root/p.phase_path("prepared", "receipt")).read_bytes(), issued)

    def test_ancestry_change_without_journal_rejected(self):
        self.seal(); self.git("add", "--all"); self.git("update-ref", "refs/heads/"+p.BRANCH, p.UPSTREAM, p.ORIGINAL)
        with self.assertRaises(g.GateError): p.validate(self.root, self.state)

    def test_false_completion_preserves_state_index(self):
        before = (self.root/".codex/state/post_increment_gate.json").read_bytes()
        self.write_json(p.REQUEST, self.req)
        with self.assertRaises(g.GateError): p.finish_effect(self.root, "committed", "0"*64)
        self.assertEqual((self.root/".codex/state/post_increment_gate.json").read_bytes(), before)
        self.assertEqual(c.git_index_path(self.root).read_bytes(), self.raw_index)


if __name__ == "__main__": unittest.main()
