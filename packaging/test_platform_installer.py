import contextlib
import io
import json
import unittest
from unittest.mock import Mock, patch
import installer


class HostInstallerBoundaryTests(unittest.TestCase):
    def setUp(self):
        # This suite isolates host action routing. Real shared/exclusive locks,
        # durable fences and stop receipts are exercised in test_host_control.
        control=patch.object(installer,'HostControl');self.addCleanup(control.stop)
        self.control=control.start().return_value
        self.control.starting.side_effect=lambda:contextlib.nullcontext()
        provenance=patch.object(installer,'verify_installer_payload');self.addCleanup(provenance.stop);self.provenance=provenance.start()
    def test_unprivileged_host_request_fails_without_docker_or_service_fallback(self):
        with patch.object(installer.os,'geteuid',return_value=1000), patch.object(installer.sys,'argv',['installer.py','containers']), patch.object(installer,'containers') as docker:
            with self.assertRaisesRegex(RuntimeError,'authorization'):
                installer.main()
            docker.assert_not_called()

    def test_container_credential_requires_label_running_state_and_fixed_unprivileged_program(self):
        identity='a'*64
        def result(*args):
            if args[0]=='inspect':
                return json.dumps({installer.LABEL:'1'}) if 'Labels' in args[2] else 'true\n'
            if args[4]=='stat':
                return '0 644\n0 755\n'
            if args[4]=='cat':
                return json.dumps({'schemaVersion':1,'binarySha256':'b'*64,'localReadCommands':['installation-credential']})
            if args[4]=='sha256sum':
                return 'b'*64+'  /usr/local/bin/yofc-management-console\n'
            return '{"schemaVersion":1}'
        with patch.object(installer,'docker',side_effect=result) as run:
            self.assertEqual(installer.container_credential(identity),{'schemaVersion':1})
            self.assertEqual(run.call_args.args,('exec','--user','console',identity,'/usr/local/bin/yofc-management-console','installation-credential'))
        with patch.object(installer,'docker',return_value='{}') as run:
            with self.assertRaisesRegex(RuntimeError,'running YOFC'):
                installer.container_credential(identity)
            self.assertEqual(run.call_count,1)
        with patch.object(installer,'docker') as run:
            with self.assertRaisesRegex(RuntimeError,'exact container'):
                installer.container_credential('../another')
            run.assert_not_called()

    def test_container_missing_provenance_never_probes_an_unknown_console_command(self):
        def result(*args):
            if args[0]=='inspect':
                return json.dumps({installer.LABEL:'1'}) if 'Labels' in args[2] else 'true\n'
            raise RuntimeError('missing protected provenance')
        with patch.object(installer,'docker',side_effect=result) as run:
            with self.assertRaisesRegex(RuntimeError,'provenance'):
                installer.container_credential('a'*64)
            self.assertFalse(any('installation-credential' in call.args for call in run.call_args_list))

    def test_in_progress_service_operation_is_observed_without_start_or_directory_changes(self):
        service=Mock()
        with patch.object(installer,'Setup',return_value=service),patch.object(installer.os,'geteuid',return_value=0),patch.object(installer.sys,'argv',['installer.py','start']),patch.object(installer,'status',return_value={'state':'activating','job':'present'}):
            with self.assertRaisesRegex(RuntimeError,'Observe'):
                installer.main()
        service.activate.assert_not_called();service.init_instance.assert_not_called();service.initial_listener.assert_not_called()

    def test_active_service_is_not_restarted_or_reconfigured(self):
        service=Mock()
        state={'state':'active','job':'','pid':'123','unit':'test-only-center'}
        output=io.StringIO()
        with patch.object(installer,'Setup',return_value=service),patch.object(installer.os,'geteuid',return_value=0),patch.object(installer.sys,'argv',['installer.py','start','--port','28443']),patch.object(installer,'status',return_value=state),contextlib.redirect_stdout(output):
            installer.main()
        self.assertEqual(json.loads(output.getvalue()),state)
        service.activate.assert_not_called();service.init_instance.assert_not_called();service.initial_listener.assert_not_called()

    def test_existing_inactive_instance_checks_chosen_port_before_activation(self):
        service=Mock();service.path.return_value.exists.return_value=True
        before={'state':'inactive','job':'','pid':'0','unit':'test-only-center'}
        after={'state':'active','job':'','pid':'123','unit':'test-only-center'}
        with patch.object(installer,'Setup',return_value=service),patch.object(installer.os,'geteuid',return_value=0),patch.object(installer.sys,'argv',['installer.py','start','--instance','center2','--port','28443']),patch.object(installer,'status',side_effect=[before,after]),contextlib.redirect_stdout(io.StringIO()):
            installer.main()
        service.init_instance.assert_not_called()
        service.initial_listener.assert_called_once_with(28443,allow_unpaired=True)
        service.activate.assert_called_once_with()
        calls=[call[0] for call in service.method_calls]
        self.assertLess(calls.index('initial_listener'),calls.index('activate'))


if __name__=='__main__':
    unittest.main()
