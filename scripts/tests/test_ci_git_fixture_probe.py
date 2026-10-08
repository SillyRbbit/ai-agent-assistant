"""Offline fixed-probe validation: subprocesses are mocked, never run Git init."""
from __future__ import annotations
import hashlib
import importlib.util
import io
import json
import os
import subprocess
import sys
import tempfile
import unittest
from contextlib import redirect_stdout
from pathlib import Path
from unittest.mock import Mock, patch

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location("ci_git_fixture_probe", ROOT / "scripts/ci_git_fixture_probe.py")
probe = importlib.util.module_from_spec(spec)
spec.loader.exec_module(probe)


class ProbeTests(unittest.TestCase):
    def context(self):
        return {"GITHUB_ACTIONS":"true", "GITHUB_EVENT_NAME":"push",
                "GITHUB_REPOSITORY":"SillyRbbit/ai-agent-assistant", "GITHUB_REF":probe.BRANCH,
                "GITHUB_RUN_ATTEMPT":"1", "PROBE_BEFORE":probe.PARENT,
                "RUNNER_NAME":"Henrys-MacBook-Pro", "RUNNER_OS":"macOS", "RUNNER_ARCH":"X64",
                "GITHUB_RUN_ID":"123", "GITHUB_SHA":"a"*40}

    def test_exact_context_and_each_mismatch(self):
        self.assertTrue(probe.valid_context(self.context()))
        for key in self.context():
            bad=self.context();bad[key]='untrusted/private'
            self.assertFalse(probe.valid_context(bad),key)
        for bad_id in ['', '0', '../outside', '1'*21]:
            values=self.context();values['GITHUB_RUN_ID']=bad_id
            self.assertFalse(probe.valid_context(values))
        values=self.context();values['GITHUB_SHA']=probe.PARENT
        self.assertFalse(probe.valid_context(values))

    def test_exact_invocation_only_home_differs(self):
        child=Mock();child.wait.return_value=1
        with patch.object(probe.subprocess,'Popen',return_value=child) as popen:
            probe.git_init(Path('/synthetic/baseline'))
            probe.git_init(Path('/synthetic/variant'),Path('/synthetic/private-home'))
        a,b=popen.call_args_list
        self.assertEqual(a.args[0],[probe.GIT,*probe.GIT_OPTIONS,'init','-q'])
        self.assertEqual(a.args,b.args)
        self.assertEqual(a.kwargs['env'],{'PATH':'/usr/bin:/bin','GIT_CONFIG_NOSYSTEM':'1','GIT_CONFIG_GLOBAL':'/dev/null'})
        self.assertEqual(b.kwargs['env'],dict(a.kwargs['env'],HOME='/synthetic/private-home'))
        for call in [a,b]:
            for stream in ['stdin','stdout','stderr']:
                self.assertEqual(call.kwargs[stream],subprocess.DEVNULL)
        self.assertNotIn('HOME',probe.BASE_ENV)

    def comparison(self,results):
        buf=io.StringIO()
        with patch.object(probe,'git_init',side_effect=results) as run,redirect_stdout(buf):
            status=probe.compare(Path('/b'),Path('/v'),Path('/h'))
        return status,run.call_count,[json.loads(x)for x in buf.getvalue().splitlines()]

    def test_nonreproduction_stops_without_variant(self):
        status,count,rows=self.comparison([('exited',0)])
        self.assertEqual((status,count),(0,1))
        self.assertEqual(rows[-1]['outcome'],'baseline_not_reproduced')

    def test_only_exit_one_allows_one_variant(self):
        for exit_status in [0,1]:
            status,count,rows=self.comparison([('exited',1),('exited',exit_status)])
            self.assertEqual((status,count),(0,2))
            self.assertEqual(rows[0]['exit_status'],1)
            self.assertEqual(rows[1]['exit_status'],exit_status)
            self.assertEqual(rows[-1]['outcome'],'comparison_complete')

    def test_unexpected_baseline_never_retries(self):
        for result in [('exited',2),('exited',-15),('spawn_failed',None),('timed_out',-15),('cleanup_unresolved',None)]:
            status,count,rows=self.comparison([result])
            self.assertEqual((status,count),(1,1))
            self.assertEqual(rows[0]['outcome'],result[0])

    def test_unexpected_variant_stops(self):
        for result in [('exited',128),('spawn_failed',None),('timed_out',-15),('cleanup_unresolved',None)]:
            status,count,_=self.comparison([('exited',1),result])
            self.assertEqual((status,count),(1,2))

    def test_timeout_terminates_only_owned_child_no_force_kill(self):
        child=Mock();child.wait.side_effect=[subprocess.TimeoutExpired('private sentinel',10),-15];child.poll.return_value=None
        with patch.object(probe.subprocess,'Popen',return_value=child):
            self.assertEqual(probe.git_init(Path('/b')),('timed_out',-15))
        child.terminate.assert_called_once_with();child.kill.assert_not_called()
        self.assertEqual(child.wait.call_args_list[0].kwargs,{'timeout':10})
        self.assertEqual(child.wait.call_args_list[1].kwargs,{'timeout':5})

    def test_exited_child_is_not_signalled_after_timeout_race(self):
        child=Mock();child.wait.side_effect=[subprocess.TimeoutExpired('private',10),0];child.poll.return_value=0
        with patch.object(probe.subprocess,'Popen',return_value=child):
            self.assertEqual(probe.git_init(Path('/b')),('timed_out',0))
        child.terminate.assert_not_called();child.kill.assert_not_called()

    def test_cleanup_failure_stays_closed(self):
        child=Mock();child.wait.side_effect=subprocess.TimeoutExpired('private',10);child.poll.return_value=None
        with patch.object(probe.subprocess,'Popen',return_value=child):
            self.assertEqual(probe.git_init(Path('/b')),('cleanup_unresolved',None))
        child.kill.assert_not_called()
        child=Mock();child.wait.side_effect=subprocess.TimeoutExpired('private',10);child.poll.return_value=None;child.terminate.side_effect=OSError('private')
        with patch.object(probe.subprocess,'Popen',return_value=child):
            self.assertEqual(probe.git_init(Path('/b')),('cleanup_unresolved',None))

    def test_spawn_error_private_text_not_emitted(self):
        buf=io.StringIO()
        with patch.object(probe.subprocess,'Popen',side_effect=OSError('PRIVATE SENTINEL')),redirect_stdout(buf):
            self.assertEqual(probe.git_init(Path('/b')),('spawn_failed',None))
        self.assertEqual(buf.getvalue(),'')

    def test_closed_output_rejects_arbitrary_content(self):
        for args in [('private','exited'),('result','PRIVATE'),('result','exited','PRIVATE'),('result','exited',True),('result','exited',999)]:
            with self.assertRaises(ValueError):probe.emit(*args)
        with self.assertRaises(ValueError):probe.emit('executable','executable_bound',executable_sha256='PRIVATE')
        buf=io.StringIO()
        with redirect_stdout(buf):probe.emit('executable','executable_bound',executable_sha256='a'*64)
        self.assertEqual(set(json.loads(buf.getvalue())),{'stage','outcome','exit_status','executable_sha256'})

    def test_fresh_private_directories_and_replay_rejection(self):
        with tempfile.TemporaryDirectory() as t:
            base,variant,home=probe.private_fixtures(Path(t),'123')
            for p in [base.parent,base,variant,home]:
                self.assertEqual(p.stat().st_mode & 0o777,0o700)
            self.assertEqual(list(home.iterdir()),[])
            with self.assertRaises(FileExistsError):probe.private_fixtures(Path(t),'123')
            for value in ['../outside','0']:
                with self.assertRaises(ValueError):probe.private_fixtures(Path(t),value)

    def test_symlink_or_relative_fixture_parent_rejected(self):
        with tempfile.TemporaryDirectory() as t:
            link=Path(t)/'link';link.symlink_to(Path(t),target_is_directory=True)
            with self.assertRaises(ValueError):probe.private_fixtures(link,'123')
        with self.assertRaises(ValueError):probe.private_fixtures(Path('.'),'123')

    def test_bound_source_and_wrong_parent_head_hash(self):
        with patch.object(probe.subprocess,'run',return_value=subprocess.CompletedProcess([],0,('a'*40+'\n'+probe.PARENT+'\n').encode())) as run:
            self.assertTrue(probe.source_bound(ROOT,'a'*40))
            self.assertEqual(run.call_args.args[0],[probe.GIT,*probe.GIT_OPTIONS,'rev-parse','HEAD','HEAD^'])
        for output in [b'private',('b'*40+'\n'+probe.PARENT+'\n').encode()]:
            with patch.object(probe.subprocess,'run',return_value=subprocess.CompletedProcess([],0,output)):
                self.assertFalse(probe.source_bound(ROOT,'a'*40))
        with patch.object(probe,'FIXTURE_HASH','0'*64),patch.object(probe.subprocess,'run') as run:
            self.assertFalse(probe.source_bound(ROOT,'a'*40));run.assert_not_called()

    def test_main_rejects_context_before_any_io(self):
        with patch.object(probe.sys,'argv',['probe']),patch.dict(os.environ,{},clear=True),patch.object(probe,'source_bound') as bound,redirect_stdout(io.StringIO()):
            self.assertEqual(probe.main(),1);bound.assert_not_called()

    def test_main_routes_preflight_failure_without_private_output(self):
        for effect in [OSError('PRIVATE'),subprocess.TimeoutExpired('PRIVATE',10)]:
            buf=io.StringIO()
            with patch.object(probe.sys,'argv',['probe']),patch.dict(os.environ,self.context(),clear=True),patch.object(probe,'source_bound',side_effect=effect),patch.object(probe,'compare') as compare,redirect_stdout(buf):
                self.assertEqual(probe.main(),1);compare.assert_not_called()
            self.assertNotIn('PRIVATE',buf.getvalue())
            self.assertEqual(json.loads(buf.getvalue())['outcome'],'preflight_failed')

    def test_workflow_preserves_all_normal_jobs_and_controls(self):
        text=(ROOT/'.github/workflows/ci.yml').read_text()
        prefix,job=text.split('\n  pr139-git-fixture-diagnostic:\n')
        self.assertEqual(hashlib.sha256(prefix.encode()).hexdigest(),'de11476018d8fc8ce9bb2d983471db5a64ac967134c8698d3e55f05bc945bf07')
        for guard in ["github.event_name == 'push'", "github.ref == '"+probe.BRANCH+"'", "github.event.before == '"+probe.PARENT+"'", 'github.run_attempt == 1']:
            self.assertIn(guard,job)
        for guard in ['runs-on: [self-hosted, macOS, X64, cortexa-ci]','timeout-minutes: 5','persist-credentials: false','fetch-depth: 2','actions/checkout@9c091bb21b7c1c1d1991bb908d89e4e9dddfe3e0','run: python3 -B scripts/ci_git_fixture_probe.py']:
            self.assertIn(guard,job)
        self.assertNotIn('needs:',job);self.assertNotIn('inputs.',job);self.assertNotIn('secrets.',job)
        self.assertNotIn('continue-on-error',job)


if __name__=='__main__':unittest.main()
