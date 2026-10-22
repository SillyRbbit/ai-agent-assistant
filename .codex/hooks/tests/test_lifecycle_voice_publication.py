"""Real disposable Git histories for the fixed voice repair admission."""
from __future__ import annotations

import copy
import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock

HOOKS = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(HOOKS))
import test_post_increment_gate as legacy

gate = legacy.gate
import lifecycle_closure as closure
import lifecycle_voice_publication as voice


class VoicePublicationTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name).resolve()
        self.git("init", "-q")
        self.git("config", "user.email", "fixture@example.invalid")
        self.git("config", "user.name", "Voice fixture")
        self.git("checkout", "-qb", voice.BRANCH)
        self.put(".gitignore", b".codex/state/\n")
        self.put("DECISIONS.md", b"explicit fixture owner approval\n")
        self.put("protected.txt", b"immutable unrelated work\n")
        self.original = b'#[cfg(not(target_os = "macos"))]\ntype HelperIdentity = ();\n    #[test]\n    fn ipc_unknown_fields_and_unknown_operations_reject() {}\n'
        self.put(voice.PRODUCT, self.original)
        self.put(voice.FAILED_REPORT_PATH, b"original FAIL remains FAIL\n")
        self.put("accepted.md", b"original accepted Mac review\n")
        self.put("readiness.md", b"frozen original readiness\n")
        for name in voice.MAINTENANCE:
            self.put(name, b"verified fixture validator\n")
        self.git("add", ".")
        self.git("commit", "-qm", "original fixture")
        self.head = self.git("rev-parse", "HEAD").strip().decode()
        self.tree = self.git("rev-parse", "HEAD^{tree}").strip().decode()
        self.inputs = ".codex/state/voice-fixture-inputs/"
        admission = {"request": {"increment_id": voice.PREDECESSOR,
                                 "readiness_report": "readiness.md", "schema_version": 2},
                     "closures": [], "raw_readiness_report": "frozen original readiness\n",
                     "readiness_report_sha256": self.hash(b"frozen original readiness\n")}
        a = closure.encoded(admission)
        ad = self.hash(a)
        self.put(str(closure.DIRECTORY / ("admission-" + ad + ".json")), a)
        self.old = {"schema_version": 3, "status": "failed", "increment_id": voice.PREDECESSOR,
                    "quality_gate": "FAIL", "next_increment_readiness": "Blocked",
                    "head_commit": self.head, "report_path": voice.FAILED_REPORT_PATH,
                    "report_sha256": self.hash((self.root / voice.FAILED_REPORT_PATH).read_bytes()),
                    "baseline_fingerprint": gate.workspace_fingerprint(self.root),
                    "workspace_fingerprint": gate.workspace_fingerprint(self.root),
                    "closure_lineage": {"admission": ad, "closures": []}}
        gate.write_state(self.root, self.old)
        raw = gate.state_path(self.root).read_bytes()
        patches = {"ROOT": str(self.root), "HEAD": self.head, "TREE": self.tree,
                   "FAILURE": self.hash(raw), "REPORT": self.old["report_sha256"],
                   "AUTHORITY": self.hash(b"approved exact fixture\n"),
                   "ACCEPTED_REPORT": self.hash(b"original accepted Mac review\n"), "ACCEPTED_ADMISSION": ad}
        for name, value in patches.items():
            p = mock.patch.object(voice, name, value)
            p.start()
            self.addCleanup(p.stop)
        self.req = {"profile": voice.SUCCESSOR, "head": self.head, "tree": self.tree,
                    "fingerprint": gate.workspace_fingerprint(self.root),
                    "index_sha256": self.hash(closure.read_bytes(closure.git_index_path(self.root))),
                    "owner": {"approved": True,
                              "authorization": self.ref("authority", b"approved exact fixture\n"),
                              "decision": self.ref("decision", (self.root / "DECISIONS.md").read_bytes())},
                    "failure": {"state": self.ref("failure", raw),
                                "report": self.existing(voice.FAILED_REPORT_PATH),
                                "admission": self.existing(str(closure.DIRECTORY / ("admission-" + ad + ".json")))},
                    "accepted": {"report": self.existing("accepted.md"),
                                 "admission": self.existing(str(closure.DIRECTORY / ("admission-" + ad + ".json")))},
                    "product_before": self.ref("product-before", self.original),
                    "maintenance": {"files": {n: self.existing(n) for n in voice.MAINTENANCE}}}
        receipt = {"exit": 0, "boundary": None, "owned_group_absent": True,
                   "source_before": "same", "source_after": "same", "argv": ["actual", "focused", "fixture", "command"]}
        validation = {"result": "Passed", "input_hashes": {n: self.hash((self.root/n).read_bytes()) for n in voice.MAINTENANCE},
                      "command": "actual focused fixture command", "receipt": self.ref("receipt", closure.encoded(receipt))}
        self.req["maintenance"]["validation"] = self.ref("validation", closure.encoded(validation))
        self.save()

    def hash(self, data):
        return closure.digest(data)

    def put(self, name, data):
        p = self.root / name
        p.parent.mkdir(parents=True, exist_ok=True)
        p.write_bytes(data)

    def existing(self, name):
        return {"path": name, "sha256": self.hash((self.root/name).read_bytes())}

    def ref(self, role, data):
        name = self.inputs + role
        self.put(name, data)
        return self.existing(name)

    def git(self, *args):
        return subprocess.check_output(["git", *args], cwd=self.root, stderr=subprocess.PIPE)

    def save(self):
        self.path = self.inputs + "request"
        self.put(self.path, closure.encoded(self.req))

    def begin(self):
        gate.begin_gate(self.root, voice.SUCCESSOR, dependent_request=self.path)
        state = gate.read_state(self.root)
        self.assertEqual(state["status"], "active")
        self.assertEqual(state["schema_version"], 5)
        return state

    def test_exact_route_preserves_failure_and_allows_only_repair(self):
        raw = gate.state_path(self.root).read_bytes()
        state = self.begin()
        voice.record(self.root, state)
        self.put(voice.PRODUCT, voice.corrected_product(self.original))
        voice.record(self.root, state)
        self.assertEqual((self.root/self.req["failure"]["state"]["path"]).read_bytes(), raw)
        self.assertEqual((self.root/voice.FAILED_REPORT_PATH).read_bytes(), b"original FAIL remains FAIL\n")

    def test_replay_rejected(self):
        self.begin()
        with self.assertRaises(gate.GateError): self.begin()

    def test_ambiguous_request_rejected(self):
        self.req["force"] = True
        self.save()
        with self.assertRaises(gate.GateError): self.begin()

    def test_mixed_lineage_rejected(self):
        state = self.begin()
        state["closure_lineage"] = self.old["closure_lineage"]
        with self.assertRaises(gate.GateError): gate.validate_state(state)

    def test_wrong_root_rejected(self):
        with mock.patch.object(voice, "ROOT", "/not-this-checkout"):
            with self.assertRaises(gate.GateError): self.begin()

    def test_unapproved_owner_rejected(self):
        self.req["owner"]["approved"] = False
        self.save()
        with self.assertRaises(gate.GateError): self.begin()

    def test_changed_failure_rejected(self):
        self.put(voice.FAILED_REPORT_PATH, b"rewritten PASS\n")
        with self.assertRaises(gate.GateError): self.begin()

    def test_failed_maintenance_rejected(self):
        val = self.req["maintenance"]["validation"]
        value = json.loads((self.root/val["path"]).read_text())
        receipt = {"exit": 1, "boundary": None, "owned_group_absent": True,
                   "source_before": "same", "source_after": "same", "argv": ["actual", "focused", "fixture", "command"]}
        value["receipt"] = self.ref("failed-receipt", closure.encoded(receipt))
        self.req["maintenance"]["validation"] = self.ref("failed-validation", closure.encoded(value))
        self.save()
        with self.assertRaises(gate.GateError): self.begin()

    def test_admission_race_preserves_original(self):
        raw = gate.state_path(self.root).read_bytes()
        original_exclusive = voice.exclusive
        def raced(root, name, data):
            original_exclusive(root, name, data)
            if name != voice.POINTER:
                self.put("protected.txt", b"racing unrelated writer\n")
        with mock.patch.object(voice, "exclusive", side_effect=raced):
            with self.assertRaises(gate.GateError): self.begin()
        self.assertEqual(gate.state_path(self.root).read_bytes(), raw)
        self.assertFalse((self.root/voice.POINTER).exists())

    def test_arbitrary_product_edit_rejected(self):
        state = self.begin()
        self.put(voice.PRODUCT, self.original + b"unreviewed behavior\n")
        with self.assertRaises(gate.GateError): voice.record(self.root, state)

    def test_unrelated_change_and_deletion_rejected(self):
        state = self.begin()
        self.put("protected.txt", b"unrelated\n")
        with self.assertRaises(gate.GateError): voice.record(self.root, state)
        (self.root/"protected.txt").unlink()
        with self.assertRaises(gate.GateError): voice.record(self.root, state)

    def test_maintenance_edit_after_validation_rejected(self):
        state = self.begin()
        self.put(".codex/hooks/post_increment_gate.py", b"changed validator\n")
        with self.assertRaises(gate.GateError): voice.record(self.root, state)

    def test_metadata_change_rejected(self):
        state = self.begin()
        (self.root/"DECISIONS.md").chmod(0o600)
        with self.assertRaises(gate.GateError): voice.record(self.root, state)

    def test_hidden_index_flag_rejected(self):
        state = self.begin()
        self.git("update-index", "--assume-unchanged", "protected.txt")
        with self.assertRaises(gate.GateError): voice.record(self.root, state)

    def test_source_commit_preserves_gate_validity(self):
        state = self.begin()
        self.put(voice.PRODUCT, voice.corrected_product(self.original))
        self.git("add", voice.PRODUCT)
        self.git("commit", "-qm", "fix: exact voice correction")
        voice.record(self.root, state)

    def test_unrelated_intermediate_commit_cannot_be_hidden_by_revert(self):
        state = self.begin()
        self.put("protected.txt", b"unrelated intermediate\n")
        self.git("add", "protected.txt")
        self.git("commit", "-qm", "unrelated")
        self.git("revert", "--no-edit", "HEAD")
        with self.assertRaises(gate.GateError): voice.record(self.root, state)

    def test_symlink_evidence_rejected(self):
        state = self.begin()
        p = self.root/self.req["product_before"]["path"]
        p.unlink()
        p.symlink_to(self.root/voice.PRODUCT)
        with self.assertRaises(gate.GateError): voice.record(self.root, state)

    def test_pointer_tamper_rejected(self):
        state = self.begin()
        self.put(str(voice.POINTER), closure.encoded({"admission": "0"*64}))
        with self.assertRaises(gate.GateError): voice.record(self.root, state)

    def test_report_omitted_obligation_and_unknown_status_rejected(self):
        self.begin()
        report = {"milestone": {"criteria": {}, "paths": {}}}
        with self.assertRaises(gate.GateError): voice.milestone_report(self.root, voice.SUCCESSOR, report)
        report["milestone"]["criteria"] = {k: {"status": "unknown", "evidence": "not proof"} for k in voice.CRITERIA}
        with self.assertRaises(gate.GateError): voice.milestone_report(self.root, voice.SUCCESSOR, report)

    def test_same_content_replacement_does_not_authorize_other_scope(self):
        # Workspace custody intentionally uses content/full-mode, not incidental inode continuity.
        state = self.begin()
        replacement = self.root/"replacement"
        replacement.write_bytes(self.original)
        replacement.replace(self.root/voice.PRODUCT)
        voice.record(self.root, state)

    def test_partial_staged_content_rejected(self):
        state = self.begin()
        self.put(voice.PRODUCT, self.original + b"staged unreviewed behavior\n")
        self.git("add", voice.PRODUCT)
        self.put(voice.PRODUCT, voice.corrected_product(self.original))
        with self.assertRaises(gate.GateError): voice.record(self.root, state)

    def test_unreviewed_product_intermediate_commit_rejected(self):
        state = self.begin()
        self.put(voice.PRODUCT, self.original + b"intermediate behavior\n")
        self.git("add", voice.PRODUCT)
        self.git("commit", "-qm", "unreviewed intermediate")
        self.put(voice.PRODUCT, voice.corrected_product(self.original))
        self.git("add", voice.PRODUCT)
        self.git("commit", "-qm", "exact current correction")
        with self.assertRaises(gate.GateError): voice.record(self.root, state)

    def test_duplicate_json_rejected(self):
        with self.assertRaises(gate.GateError): voice.parse(b'{"a":1,"a":2}')

    def test_partial_admission_is_not_replayed(self):
        (self.root/voice.STORE).mkdir(parents=True)
        self.put(str(voice.STORE/"interrupted.json"), b"{}")
        with self.assertRaises(gate.GateError): self.begin()


if __name__ == "__main__":
    unittest.main()
