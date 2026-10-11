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

    def success_bytes(self):
        return ('a'*40+'\n'+probe.PARENT+'\n').encode()

    def result(self,outcome='metadata_read',status=0,output=None,cleanup=None,close=False,category='stderr_empty'):
        return (outcome,status,self.success_bytes() if output is None else output,cleanup,close,category)

    def test_exact_source_and_each_malformed_mismatch(self):
        self.assertEqual(probe.source_bound(self.success_bytes(),'a'*40),'source_bound')
        for output in [b'PRIVATE',self.success_bytes()+b'\n',self.success_bytes().upper(),b'\xff',b'a'*83]:
            self.assertEqual(probe.source_bound(output,'a'*40),'metadata_malformed')
        for output in [('b'*40+'\n'+probe.PARENT+'\n').encode(),('a'*40+'\n'+'b'*40+'\n').encode()]:
            self.assertEqual(probe.source_bound(output,'a'*40),'head_parent_mismatch')

    def test_fixture_binding_remains_strict(self):
        self.assertTrue(probe.fixture_bound(ROOT))
        with patch.object(probe,'FIXTURE_HASH','0'*64):self.assertFalse(probe.fixture_bound(ROOT))
        with patch.object(probe.Path,'is_symlink',return_value=True):self.assertFalse(probe.fixture_bound(ROOT))
        with patch.object(probe.Path,'read_bytes',side_effect=OSError('PRIVATE')):self.assertFalse(probe.fixture_bound(ROOT))

    def metadata_case(self,events,status=0,wait=None,close=None,select_error=None,clock=None):
        child=Mock();child.stdout.fileno.return_value=11;child.stderr.fileno.return_value=12
        child.wait.return_value=status;child.poll.return_value=status
        if wait is not None:child.wait.side_effect=wait
        if close is not None:getattr(child,close).close.side_effect=OSError('PRIVATE')
        selector=Mock();queue=[]
        def selected(timeout):
            if select_error is not None:raise select_error
            if not events:return []
            batch=events.pop(0)
            queue.extend(batch)
            return [(Mock(data=name),1)for name,chunk in batch]
        selector.select.side_effect=selected
        def read(fd,count):
            name,chunk=queue.pop(0)
            self.assertEqual(fd,11 if name=='stdout' else 12)
            self.assertLessEqual(count,83 if name=='stdout' else probe.STDERR_LIMIT+1)
            if isinstance(chunk,Exception):raise chunk
            self.assertLessEqual(len(chunk),count)
            return chunk
        context=Mock();context.__enter__=Mock(return_value=selector);context.__exit__=Mock(return_value=False)
        with patch.object(probe.subprocess,'Popen',return_value=child)as spawn,patch.object(probe.selectors,'DefaultSelector',return_value=context),patch.object(probe.os,'read',side_effect=read)as reader,patch.object(probe.time,'monotonic',side_effect=clock,return_value=1):
            result=probe.metadata(ROOT)
        self.assertEqual(spawn.call_args.args[0],[probe.GIT,*probe.GIT_OPTIONS,'rev-parse','HEAD','HEAD^'])
        self.assertEqual(spawn.call_args.kwargs['env'],probe.BASE_ENV)
        self.assertEqual(spawn.call_args.kwargs['cwd'],ROOT)
        self.assertEqual(spawn.call_args.kwargs['stdin'],subprocess.DEVNULL)
        for name in ['stdout','stderr']:
            self.assertEqual(spawn.call_args.kwargs[name],subprocess.PIPE)
            getattr(child,name).close.assert_called_once_with()
        child.kill.assert_not_called()
        return result,child,reader,selector

    def events(self,stdout=None,stderr=b'',order=('stdout','stderr')):
        data={'stdout':self.success_bytes()if stdout is None else stdout,'stderr':stderr}
        return [[(n,data[n])]for n in order if data[n]]+[[(n,b'')]for n in order]

    def test_dual_stream_both_eof_orders_and_simultaneous_ready(self):
        for order in [('stdout','stderr'),('stderr','stdout')]:
            result,child,read,selector=self.metadata_case(self.events(stderr=b'PRIVATE',order=order))
            self.assertEqual(result,('metadata_read',0,self.success_bytes(),None,False,'stderr_unknown'))
            self.assertEqual([x.args[1]for x in read.call_args_list], [83,2049,1,2042]if order[0]=='stdout'else[2049,83,2042,1])
            self.assertEqual(selector.register.call_count,2);self.assertEqual(selector.unregister.call_count,2);child.terminate.assert_not_called()
        result,_,_,_=self.metadata_case([[('stdout',self.success_bytes()),('stderr',b'PRIVATE')],[('stderr',b''),('stdout',b'')]])
        self.assertEqual(result[-1],'stderr_unknown')

    def test_split_reads_preserve_metadata_shape_and_bound_caps(self):
        data=self.success_bytes()
        result,_,read,_=self.metadata_case([[('stdout',data[:40])],[('stderr',b'x'*1024)],[('stdout',data[40:])],[('stderr',b'x'*1024)],[('stdout',b''),('stderr',b'')]])
        self.assertEqual(result[:3],('metadata_read',0,data));self.assertEqual(result[-1],'stderr_unknown')
        self.assertEqual([x.args[1]for x in read.call_args_list],[83,2049,43,1025,1,1])

    def test_reviewed_signatures_empty_unknown_and_near_misses(self):
        positive=[(probe.REPOSITORY_SIGNATURE,'stderr_git_repository_signature'),(probe.OWNERSHIP_PREFIX+b"/PRIVATE/path'\nPRIVATE trailing advice",'stderr_git_ownership_signature')]
        for raw,category in positive:
            self.assertEqual(probe.classify_stderr(bytearray(raw)),category)
        for raw in [b'PRIVATE',b'\xff',probe.REPOSITORY_SIGNATURE[:-1],b'x'+probe.REPOSITORY_SIGNATURE,probe.REPOSITORY_SIGNATURE+b'PRIVATE',probe.REPOSITORY_SIGNATURE.upper(),probe.OWNERSHIP_PREFIX+b"/PRIVATE/path'",probe.OWNERSHIP_PREFIX+b"/PRIVATE\x1b/path'\n",b"warning: "+probe.OWNERSHIP_PREFIX]:
            self.assertEqual(probe.classify_stderr(bytearray(raw)),'stderr_unknown')
        self.assertEqual(probe.classify_stderr(bytearray()),'stderr_empty')

    def test_normal_nonzero_signature_observation_remains_failure(self):
        for status in [1,128,-15]:
            result,child,_,_=self.metadata_case(self.events(stdout=b'PRIVATE',stderr=probe.REPOSITORY_SIGNATURE),status=status)
            self.assertEqual(result,('metadata_nonzero',status,b'',None,False,'stderr_git_repository_signature'));child.terminate.assert_not_called()
            code,count,rows=self.experiment([result]);self.assertEqual((code,count),(1,1));self.assertEqual(rows[-1]['outcome'],'comparison_stopped')

    def test_each_stream_overflow_stops_with_independent_cleanup(self):
        for name,chunk,primary,category in [('stdout',b'P'*83,'metadata_malformed','stderr_capture_failed'),('stderr',b'P'*2049,'metadata_stderr_limit','stderr_truncated')]:
            result,child,read,_=self.metadata_case([[(name,chunk)]],status=1)
            self.assertEqual(result,(primary,None,b'',('metadata_cleanup_reaped',1),False,category))
            self.assertEqual(read.call_count,1);child.terminate.assert_not_called()

    def test_truncated_signature_never_classifies_or_exports_bytes(self):
        raw=probe.OWNERSHIP_PREFIX+b"/PRIVATE/path'\n"+b'P'*(2049-len(probe.OWNERSHIP_PREFIX)-len(b"/PRIVATE/path'\n"))
        with patch.object(probe,'classify_stderr')as classify:
            result,_,_,_=self.metadata_case([[('stderr',raw)]],status=1)
        classify.assert_not_called();self.assertEqual(result[-1],'stderr_truncated')
        code,count,rows=self.experiment([result]);self.assertEqual((code,count),(1,1));self.assertNotIn('PRIVATE',json.dumps(rows))

    def test_spawn_capture_select_and_wait_failures_stay_distinct(self):
        with patch.object(probe.subprocess,'Popen',side_effect=OSError('PRIVATE')):
            self.assertEqual(probe.metadata(ROOT),('metadata_spawn_failed',None,b'',None,False,'stderr_capture_failed'))
        for stream in ['stdout','stderr']:
            result,_,_,_=self.metadata_case([[(stream,OSError('PRIVATE'))]],status=1)
            self.assertEqual(result,('metadata_capture_failed',None,b'',('metadata_cleanup_reaped',1),False,'stderr_capture_failed'))
        result,_,_,_=self.metadata_case([],status=1,select_error=ValueError('PRIVATE'))
        self.assertEqual(result[0],'metadata_capture_failed')
        result,_,_,_=self.metadata_case(self.events(),wait=[OSError('PRIVATE'),1])
        self.assertEqual(result,('metadata_wait_failed',None,b'',('metadata_cleanup_reaped',1),False,'stderr_capture_failed'))

    def test_shared_deadline_includes_both_streams_and_reaping(self):
        for events,clock in [([],None),([[('stdout',self.success_bytes())]],None),([[('stderr',probe.REPOSITORY_SIGNATURE)]],[1,12])]:
            with patch.object(probe,'classify_stderr')as classify:
                result,_,_,_=self.metadata_case(events,status=1,clock=clock)
            self.assertEqual(result[0],'metadata_timed_out');self.assertEqual(result[-1],'stderr_capture_failed');classify.assert_not_called()
        result,child,_,_=self.metadata_case(self.events(),wait=[subprocess.TimeoutExpired('PRIVATE',10),1])
        self.assertEqual(result[0],'metadata_timed_out');self.assertEqual(result[-1],'stderr_capture_failed')
        self.assertEqual(child.wait.call_args_list[0].kwargs,{'timeout':10});self.assertEqual(child.wait.call_args_list[1].kwargs,{'timeout':5})

    def test_either_close_failure_preserves_primary_and_denies_classification(self):
        for name in ['stdout','stderr']:
            with patch.object(probe,'classify_stderr')as classify:
                result,_,_,_=self.metadata_case(self.events(stderr=probe.REPOSITORY_SIGNATURE),status=1,close=name)
            self.assertEqual(result,('metadata_nonzero',1,b'',None,True,'stderr_capture_failed'));classify.assert_not_called()
            code,count,rows=self.experiment([result]);self.assertEqual((code,count),(1,1))
            self.assertEqual([r['outcome']for r in rows],['metadata_nonzero','stderr_capture_failed','metadata_stream_close_failed','comparison_stopped'])

    def test_transient_stderr_buffer_is_cleared_and_only_label_returned(self):
        buffers=[];real=probe.classify_stderr
        def classify(captured):buffers.append(captured);return real(captured)
        with patch.object(probe,'classify_stderr',side_effect=classify):
            result,_,_,_=self.metadata_case(self.events(stderr=b'PRIVATE key/path/environment'),status=1)
        self.assertEqual(buffers,[bytearray()]);self.assertEqual(result[-1],'stderr_unknown')
        self.assertNotIn('PRIVATE',repr(result))
        code,count,rows=self.experiment([result]);self.assertNotIn('PRIVATE',json.dumps(rows))
        self.assertTrue(all(set(r)=={'stage','outcome','exit_status'}for r in rows))

    def test_cleanup_owned_identity_race_and_unknown_status(self):
        child=Mock();child.poll.return_value=None;child.wait.return_value=-15
        self.assertEqual(probe.stop_metadata_child(child),('metadata_cleanup_reaped',-15));child.terminate.assert_called_once_with();child.kill.assert_not_called()
        for error in [OSError('PRIVATE'),ValueError('PRIVATE'),subprocess.TimeoutExpired('PRIVATE',5)]:
            child=Mock();child.poll.return_value=0;child.wait.side_effect=error
            self.assertEqual(probe.stop_metadata_child(child),('metadata_cleanup_unresolved',None));child.terminate.assert_not_called();child.kill.assert_not_called()
        child=Mock();child.poll.side_effect=OSError('PRIVATE')
        self.assertEqual(probe.stop_metadata_child(child),('metadata_cleanup_unresolved',None));child.terminate.assert_not_called()

    def experiment(self,results,hashes=None):
        buf=io.StringIO()
        with patch.object(probe,'metadata',side_effect=results)as query,patch.object(probe,'executable_hash',side_effect=hashes,return_value='f'*64),redirect_stdout(buf):
            status=probe.metadata_only(ROOT,'a'*40,'f'*64)
        return status,query.call_count,[json.loads(x)for x in buf.getvalue().splitlines()]

    def test_baseline_success_strictly_bound_stops_without_variant(self):
        code,count,rows=self.experiment([self.result()]);self.assertEqual((code,count),(0,1));self.assertEqual(rows[-1]['outcome'],'metadata_baseline_verified')
        for output in [b'PRIVATE',('b'*40+'\n'+probe.PARENT+'\n').encode()]:
            code,count,rows=self.experiment([self.result(output=output)]);self.assertEqual((code,count),(1,1));self.assertNotIn('PRIVATE',json.dumps(rows))

    def test_every_failure_including_normal_exit_one_stops_after_one_query(self):
        for outcome,status in [('metadata_spawn_failed',None),('metadata_capture_failed',None),('metadata_wait_failed',None),('metadata_timed_out',None),('metadata_malformed',None),('metadata_stderr_limit',None),('metadata_nonzero',1),('metadata_nonzero',2),('metadata_nonzero',-15)]:
            code,count,rows=self.experiment([self.result(outcome,status,b'',category='stderr_unknown')]);self.assertEqual((code,count),(1,1));self.assertEqual(rows[-1]['outcome'],'comparison_stopped')

    def test_primary_failure_survives_cleanup_one_or_unavailable_status(self):
        for cleanup in [('metadata_cleanup_reaped',1),('metadata_cleanup_unresolved',None)]:
            code,count,rows=self.experiment([self.result('metadata_capture_failed',None,b'',cleanup,category='stderr_capture_failed')])
            self.assertEqual((code,count),(1,1));self.assertEqual([r['outcome']for r in rows],['metadata_capture_failed','stderr_capture_failed',cleanup[0],'comparison_stopped'])

    def test_executable_drift_blocks_acceptance_without_second_query(self):
        for result in [self.result(),self.result('metadata_nonzero',1,b'')]:
            code,count,rows=self.experiment([result],['e'*64]);self.assertEqual((code,count),(1,1));self.assertEqual(rows[-1]['outcome'],'executable_drift')

    def test_metadata_private_root_claim_and_replay(self):
        with tempfile.TemporaryDirectory() as t:
            home=probe.private_metadata_home(Path(t),'123')
            for p in [home.parent,home]:self.assertEqual(p.stat().st_mode &0o777,0o700)
            self.assertEqual(list(home.iterdir()),[])
            with self.assertRaises(FileExistsError):probe.private_metadata_home(Path(t),'123')
            link=Path(t)/'link';link.symlink_to(Path(t),target_is_directory=True)
            with self.assertRaises(ValueError):probe.private_metadata_home(link,'124')
            for bad in ['0','../outside']:
                with self.assertRaises(ValueError):probe.private_metadata_home(Path(t),bad)
        with self.assertRaises(ValueError):probe.private_metadata_home(Path('.'),'123')

    def test_metadata_claim_wrong_owner_or_mode_stops(self):
        with tempfile.TemporaryDirectory() as t:
            uid=os.getuid()
            with patch.object(probe.os,'getuid',return_value=uid+1):
                with self.assertRaises(ValueError):probe.private_metadata_home(Path(t),'123')
            mkdir=Path.mkdir
            def wrong_mode(path,mode=0o777,parents=False,exist_ok=False):
                return mkdir(path,mode=0o755,parents=parents,exist_ok=exist_ok)
            with patch.object(probe.Path,'mkdir',wrong_mode):
                with self.assertRaises(ValueError):probe.private_metadata_home(Path(t),'124')

    def test_main_denies_context_and_fixture_before_claim_or_query(self):
        for context,binding in [({},True),(self.context(),False)]:
            with patch.object(probe.sys,'argv',['probe']),patch.dict(os.environ,context,clear=True),patch.object(probe,'fixture_bound',return_value=binding),patch.object(probe,'private_metadata_home') as home,patch.object(probe,'metadata') as query,redirect_stdout(io.StringIO()):
                self.assertEqual(probe.main(),1);home.assert_not_called();query.assert_not_called()

    def test_main_claim_errors_are_fixed_and_never_query(self):
        for error,outcome in [(FileExistsError('PRIVATE'),'already_used'),(OSError('PRIVATE'),'preflight_failed')]:
            buf=io.StringIO()
            with patch.object(probe.sys,'argv',['probe']),patch.dict(os.environ,self.context(),clear=True),patch.object(probe,'fixture_bound',return_value=True),patch.object(probe,'executable_hash',return_value='f'*64),patch.object(probe,'private_metadata_home',side_effect=error),patch.object(probe,'metadata_only') as query,redirect_stdout(buf):
                self.assertEqual(probe.main(),1);query.assert_not_called()
            self.assertNotIn('PRIVATE',buf.getvalue());self.assertEqual(json.loads(buf.getvalue().splitlines()[-1])['outcome'],outcome)

    def test_main_can_never_execute_git_init_or_fixtures(self):
        with patch.object(probe.sys,'argv',['probe']),patch.dict(os.environ,self.context(),clear=True),patch.object(probe,'fixture_bound',return_value=True),patch.object(probe,'executable_hash',return_value='f'*64),patch.object(probe,'private_metadata_home',return_value=Path('/synthetic/home')),patch.object(probe,'metadata_only',return_value=0) as query,patch.object(probe,'private_fixtures') as fixtures,patch.object(probe,'git_init') as init,patch.object(probe,'compare') as compare,redirect_stdout(io.StringIO()):
            self.assertEqual(probe.main(),0);query.assert_called_once();fixtures.assert_not_called();init.assert_not_called();compare.assert_not_called()

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
