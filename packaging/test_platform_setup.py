import json
import hashlib
import os
from pathlib import Path
from types import SimpleNamespace
import tempfile
import unittest
from unittest.mock import Mock, patch
import lifecycle
import setup


class PlatformInstallerTests(unittest.TestCase):
    def test_missing_provenance_never_invokes_an_old_binary_with_an_unknown_command(self):
        with tempfile.TemporaryDirectory() as directory:
            runner=Mock(side_effect=AssertionError('Old binary must not be invoked'))
            service=setup.Setup(Path(directory),run=runner)
            with self.assertRaisesRegex(RuntimeError,'provenance'):
                service.bound_command('installation-maintenance-check')
            runner.assert_not_called()

    @unittest.skipUnless(os.geteuid()==0,'installed executable provenance is owned by root')
    def test_provenance_requires_capability_source_manifest_and_actual_binary_digest(self):
        with tempfile.TemporaryDirectory() as directory:
            service=setup.Setup(Path(directory),run=Mock(side_effect=AssertionError('Validation only')))
            binary=service.path(setup.BINARY);binary.parent.mkdir(parents=True);binary.write_bytes(b'unit-only executable sentinel');binary.chmod(0o755)
            base=service.path('/usr/share/kura-management-console');base.mkdir(parents=True)
            source=base/'source-manifest.json';source.write_text('{}')
            record={'schemaVersion':1,'binarySha256':hashlib.sha256(binary.read_bytes()).hexdigest(),
                'sourceManifestSha256':hashlib.sha256(source.read_bytes()).hexdigest(),'localReadCommands':['installation-credential']}
            metadata=base/'local-command-provenance.json';metadata.write_text(json.dumps(record))
            service.verify_local_command('installation-credential')
            with self.assertRaisesRegex(RuntimeError,'verified support'):
                service.verify_local_command('installation-maintenance-check')
            binary.write_bytes(b'different unit-only payload')
            with self.assertRaisesRegex(RuntimeError,'verified support'):
                service.verify_local_command('installation-credential')
            service.run.assert_not_called()
    @unittest.skipUnless(os.geteuid()==0,'packaged instance directories require root fixtures')
    def test_host_channel_instance_preparation_has_no_extra_stdout_before_json(self):
        import contextlib
        import io
        with tempfile.TemporaryDirectory() as directory:
            service=setup.Setup(Path(directory),run=Mock(return_value=SimpleNamespace(returncode=0,stdout='',stderr='')),instance='center2')
            output=io.StringIO()
            with patch.object(setup.pwd,'getpwnam',return_value=SimpleNamespace(pw_uid=os.getuid())),patch.object(setup.grp,'getgrnam',return_value=SimpleNamespace(gr_gid=os.getgid())),contextlib.redirect_stdout(output):
                service.init_instance(quiet=True)
            self.assertEqual(output.getvalue(),'')
            self.assertTrue(service.path(service.state+'/.package-v1').is_file())

    @unittest.skipUnless(os.geteuid()==0,'host launch settings must be written by root')
    def test_first_listener_selection_is_private_create_only_and_needs_no_database(self):
        with tempfile.TemporaryDirectory() as directory:
            service=setup.Setup(Path(directory),run=Mock(side_effect=AssertionError('No child process allowed')))
            service.initial_listener(28443)
            path=service.path(service.config+'/console.env')
            self.assertEqual(path.read_text(),'CONSOLE_SETUP_PORT=28443\n')
            self.assertEqual(path.stat().st_mode & 0o777,0o600)
            before=path.read_bytes()
            service.initial_listener(28443)
            with self.assertRaisesRegex(RuntimeError,'cannot be replaced'):
                service.initial_listener(29443)
            self.assertEqual(path.read_bytes(),before)

    def test_first_listener_does_not_reconfigure_an_existing_bootstrap_identity(self):
        with tempfile.TemporaryDirectory() as directory:
            service=setup.Setup(Path(directory))
            (service.path(service.state)/'setup').mkdir(parents=True)
            with self.assertRaisesRegex(RuntimeError,'original listener'):
                service.initial_listener(28443)
            self.assertFalse(service.path(service.config+'/console.env').exists())

    @unittest.skipUnless(os.geteuid()==0,'host launch settings must be written by root')
    def test_explicit_unpaired_port_recovery_preserves_every_identity_file(self):
        with tempfile.TemporaryDirectory() as directory:
            service=setup.Setup(Path(directory),run=Mock(side_effect=AssertionError('No launch in listener selection')))
            state=service.path(service.state)
            for name in ['.package-v1','tls-runtime.lock','setup/bootstrap.key',
                         'setup/bootstrap-request.cnf','setup/bootstrap.pem','setup/installation-credential.json']:
                path=state/name;path.parent.mkdir(parents=True,exist_ok=True)
                path.write_text('unit-only bootstrap fixture '+name)
            before={str(p.relative_to(state)):p.read_bytes() for p in state.rglob('*') if p.is_file()}
            with patch.object(service,'bound_command',return_value={'schemaVersion':1}) as verify:
                service.initial_listener(28443,allow_unpaired=True)
            verify.assert_called_once_with('installation-credential')
            self.assertEqual(service.path(service.config+'/console.env').read_text(),'CONSOLE_SETUP_PORT=28443\n')
            self.assertEqual({str(p.relative_to(state)):p.read_bytes() for p in state.rglob('*') if p.is_file()},before)
            service.run.assert_not_called()

    def test_unpaired_port_recovery_refuses_session_operations_generations_and_unknown_paths(self):
        for extra in ['setup/session.json','setup-operations/original.accepted.json',
                      'installation-generations/original/server.key','unexpected']:
            with self.subTest(extra=extra),tempfile.TemporaryDirectory() as directory:
                service=setup.Setup(Path(directory),run=Mock(side_effect=AssertionError('No child process allowed')))
                state=service.path(service.state)
                for name in ['.package-v1','tls-runtime.lock','setup/bootstrap.key',
                             'setup/bootstrap-request.cnf','setup/bootstrap.pem','setup/installation-credential.json',extra]:
                    path=state/name;path.parent.mkdir(parents=True,exist_ok=True);path.write_text('unit-only fixture')
                with self.assertRaisesRegex(RuntimeError,'original listener'):
                    service.initial_listener(28443,allow_unpaired=True)
                self.assertFalse(service.path(service.config+'/console.env').exists())
                service.run.assert_not_called()

    @unittest.skipUnless(os.geteuid()==0,'native maintenance controls require root fixtures')
    def test_fresh_launch_checks_payload_and_private_state_without_env_or_database(self):
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory)
            service=setup.Setup(root,run=Mock(side_effect=AssertionError('No database or child process allowed')))
            binary=service.path(setup.BINARY)
            binary.parent.mkdir(parents=True)
            binary.write_text('# unit-only executable payload sentinel\n')
            binary.chmod(0o755)
            state=service.path(service.state)
            state.mkdir(parents=True,mode=0o700)
            web=service.path('/usr/share/kura-management-console/react')
            (web/'assets').mkdir(parents=True)
            (web/'index.html').write_text('<html></html>')
            (web/'assets/app.js').write_text('// unit-only web payload sentinel')
            with patch.object(setup.pwd,'getpwnam',return_value=SimpleNamespace(pw_uid=os.getuid())):
                self.assertTrue(service.launch_check()['ok'])
                state.chmod(0o755)
                with self.assertRaisesRegex(RuntimeError,'0700'):
                    service.launch_check()
            self.assertFalse(service.path(service.config+'/console.env').exists())
            service.run.assert_not_called()

    def test_credential_child_gets_only_fixed_identity_and_state_not_host_secrets(self):
        runner=Mock(return_value=SimpleNamespace(returncode=0,stdout='{"schemaVersion":1}',stderr=''))
        service=setup.Setup(run=runner,instance='center2')
        with patch.dict(os.environ,{'DATABASE_URL':'private-test-only','CONSOLE_TLS_KEY':'private-test-only'}),patch.object(service,'verify_local_command') as verify:
            self.assertEqual(service.bound_command('installation-credential'),{'schemaVersion':1})
        verify.assert_called_once_with('installation-credential')
        argv=runner.call_args.args[0]
        self.assertEqual(argv,[setup.RUNUSER,'-u',setup.SERVICE_USER,'--',setup.BINARY,'installation-credential'])
        self.assertEqual(runner.call_args.kwargs['env'],{'PATH':'/usr/sbin:/usr/bin:/sbin:/bin','CONSOLE_STATE_DIR':service.state})
        self.assertNotIn('private-test-only',repr(runner.call_args))

    def test_unsettled_ui_installation_blocks_maintenance_before_any_stop(self):
        service=Mock()
        control=Mock();control.status.return_value={'active':None}
        control.begin.side_effect=RuntimeError('unsettled installation')
        with patch.object(lifecycle,'Setup',return_value=service,create=True),patch.object(lifecycle,'HostControl',return_value=control):
            with self.assertRaisesRegex(RuntimeError,'unsettled'):
                lifecycle.lifecycle('prerm','upgrade')
        control.stop.assert_not_called();service.stop_all_instances.assert_not_called()

    def test_ui_state_guard_uses_readonly_binary_command_without_legacy_env(self):
        with tempfile.TemporaryDirectory() as directory:
            runner=Mock(return_value=SimpleNamespace(returncode=0,stdout='{"ok":true}',stderr=''))
            service=setup.Setup(Path(directory),run=runner)
            (service.path(service.state)/'setup').mkdir(parents=True)
            with patch.object(service,'verify_local_command'):
                service.guard_tasks()
            self.assertEqual(runner.call_args.args[0][-1],'installation-maintenance-check')
            self.assertNotIn('DATABASE_URL',runner.call_args.kwargs['env'])


if __name__=='__main__':
    unittest.main()
