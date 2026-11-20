import fcntl
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch
import uuid
from host_control import HostControl
from setup import Setup


@unittest.skipUnless(os.geteuid()==0,'root-owned native control fixtures')
class MaintenanceTests(unittest.TestCase):
    def setUp(self):
        self.temporary=tempfile.TemporaryDirectory();self.addCleanup(self.temporary.cleanup)
        self.root=Path(self.temporary.name);(self.root/'var/lib').mkdir(parents=True)
        (self.root/'run/systemd/system').mkdir(parents=True)
        self.calls=[];self.active=True;self.fail_stop=False
        self.setup=Setup(self.root,self.run_command)
        self.control=HostControl(self.setup)
        self.id=str(uuid.uuid4())
        self.guard=patch.object(Setup,'guard_all_instances');self.addCleanup(self.guard.stop);self.check=self.guard.start()
        self.inventory=patch.object(Setup,'template_units',return_value=[])
        self.addCleanup(self.inventory.stop);self.listed=self.inventory.start()
        self.protocol=patch.object(HostControl,'supported',return_value=True);self.addCleanup(self.protocol.stop);self.protocol.start()
        self.active_proof=patch.object(HostControl,'verify_active_participants')
        self.addCleanup(self.active_proof.stop);self.active_proof.start()

    def run_command(self,argv,**kwargs):
        self.calls.append(argv)
        if argv[1]=='show':
            return SimpleNamespace(stdout='ActiveState='+('active' if self.active else 'inactive')+'\nMainPID='+('123' if self.active else '0')+'\nJob=\n',returncode=0)
        if argv[1]=='stop':
            self.active=False
            if self.fail_stop: raise subprocess.TimeoutExpired(argv,1)
        return SimpleNamespace(stdout='',returncode=0)

    def stops(self):return [call for call in self.calls if call[1]=='stop']

    def test_real_cross_process_writer_refuses_before_fence_or_stop(self):
        self.control.provision()
        script='import fcntl,sys; f=open(sys.argv[1]); fcntl.flock(f,fcntl.LOCK_SH); print("ready",flush=True); sys.stdin.read()'
        child=subprocess.Popen([sys.executable,'-c',script,str(self.control.directory/'writers.lock')],stdin=subprocess.PIPE,stdout=subprocess.PIPE,text=True)
        try:
            self.assertEqual(child.stdout.readline().strip(),'ready')
            with self.assertRaisesRegex(RuntimeError,'busy'):self.control.begin(self.id)
            self.assertIsNone(self.control.active());self.assertEqual(self.stops(),[]);self.check.assert_not_called()
        finally:
            child.communicate('done',timeout=5)
        self.assertEqual(child.returncode,0)
        self.control.begin(self.id)
        with self.assertRaisesRegex(RuntimeError,'frozen'):
            with self.control.starting():raise AssertionError('must not enter')

    def test_unsettled_guard_never_accepts_or_stops(self):
        self.check.side_effect=RuntimeError('unsettled UNKNOWN task')
        with self.assertRaisesRegex(RuntimeError,'unsettled'):self.control.begin(self.id)
        self.assertIsNone(self.control.active());self.assertEqual(self.stops(),[])

    def test_live_maintenance_requires_the_original_executable_digest(self):
        self.active_proof.stop()
        with patch.object(Setup, 'active_process_digest', return_value='b' * 64) as digest, \
                patch.object(Setup, 'verify_local_command', return_value={'binarySha256': 'a' * 64}):
            with self.assertRaisesRegex(RuntimeError, 'running participant'):
                self.control.begin(self.id)
            self.check.assert_not_called()
            self.assertFalse(self.control.status()['activeFencePresent'])
            self.assertEqual(self.stops(), [])
            digest.assert_called_once_with('123')
            digest.return_value = 'a' * 64
            self.assertEqual(self.control.begin(self.id)['phase'], 'FROZEN')
            self.assertTrue(self.control.status()['activeFencePresent'])
            self.assertEqual(self.stops(), [])

    def test_unknown_stop_is_not_reissued_even_when_service_is_now_inactive(self):
        self.control.begin(self.id);self.fail_stop=True
        with self.assertRaises(subprocess.TimeoutExpired):self.control.stop(self.id)
        self.assertFalse(self.active);self.assertEqual(len(self.stops()),1)
        reopened=HostControl(self.setup)
        with self.assertRaisesRegex(RuntimeError,'UNKNOWN'):reopened.stop(self.id)
        with self.assertRaisesRegex(RuntimeError,'uncertain'):reopened.complete(self.id)
        self.assertEqual(len(self.stops()),1);self.assertIsNotNone(reopened.active())
        with self.assertRaisesRegex(RuntimeError,'Another original'):reopened.begin(str(uuid.uuid4()))

    def test_completed_stop_is_observed_and_completion_never_starts(self):
        self.control.begin(self.id);self.control.stop(self.id);self.control.stop(self.id)
        self.assertEqual(len(self.stops()),1)
        with patch.object(Setup,'verify_local_command',return_value={'maintenanceProtocolVersion':1,'binarySha256':'a'*64,'sourceManifestSha256':'b'*64}):
            value=self.control.complete(self.id);again=self.control.complete(self.id)
        self.assertEqual(value,again);self.assertEqual(value['phase'],'COMPLETE');self.assertIsNone(self.control.active())
        self.assertEqual(value['verifiedPayload']['binarySha256'],'a'*64)
        self.assertTrue(value['stopReceipts'][self.setup.unit]['commandIssued'])
        self.assertEqual(value['stopReceipts'][self.setup.unit]['returncode'],0)
        self.assertFalse(any(call[1] in ['start','restart','enable'] for call in self.calls))
        with self.assertRaisesRegex(RuntimeError,'reused'):self.control.begin(self.id)

    def test_debian_configure_recovers_completed_fence_release_without_rewriting(self):
        self.control.begin(self.id)
        stopped = self.control.stop(self.id)
        context = {'version':'1.2', 'previousVersion':'1.1', 'bootId':str(uuid.uuid4()),
                   'rootDevice':1, 'rootInode':2, 'scriptInode':3, 'scriptSha256':'e'*64}
        provenance = {'maintenanceProtocolVersion':1, 'binarySha256':'a'*64, 'sourceManifestSha256':'b'*64}
        with patch.object(Setup,'verify_local_command',return_value=provenance), \
                patch.object(Setup,'debian_package_state',return_value=('1.2','half-configured')) as state:
            complete = self.control.complete(self.id, 'a'*64, 'b'*64, debian=context)
            self.assertEqual(complete['stopReceipts'], stopped['stopReceipts'])
            self.assertIsNone(self.control.active())
            journal = self.control.directory/'operations'/f'{self.id}.json'
            old = journal.stat().st_ino
            recovered = self.control.complete(None, 'a'*64, 'b'*64, debian=context)
            self.assertEqual(recovered, complete)
            self.assertEqual(journal.stat().st_ino, old)
            for mutation in ({'rootDevice':99}, {'previousVersion':'1.0'}, {'scriptInode':9}):
                with self.assertRaisesRegex(RuntimeError,'Exact original'):
                    self.control.complete(None, 'a'*64, 'b'*64, debian={**context,**mutation})
            with self.assertRaisesRegex(RuntimeError,'target payload'):
                self.control.complete(None, 'c'*64, 'b'*64, debian=context)
            with self.assertRaisesRegex(RuntimeError,'finish the original'):
                with self.control.starting():
                    raise AssertionError('must not enter before dpkg settles')
            with self.assertRaisesRegex(RuntimeError,'Finish the original'):
                self.control.begin(str(uuid.uuid4()))
            state.return_value = ('1.2','installed')
            with self.control.starting():
                pass
        self.assertEqual(len(self.stops()),1)

    def test_debian_configure_intent_without_terminal_receipt_does_not_acknowledge_missing_fence(self):
        self.control.begin(self.id)
        self.control.stop(self.id)
        context = {'version':'1.2'}
        self.control.write('debian-configure.json', {
            'schemaVersion':1, 'operationId':self.id, 'context':context},new=True)
        (self.control.directory/'active.json').unlink()
        provenance = {'maintenanceProtocolVersion':1, 'binarySha256':'a'*64, 'sourceManifestSha256':'b'*64}
        with patch.object(Setup,'verify_local_command',return_value=provenance):
            with self.assertRaisesRegex(RuntimeError,'completion differs'):
                self.control.complete(None, 'a'*64, 'b'*64, debian=context)
        self.assertEqual(self.control.journal(self.id)['phase'],'STOPPED')
        self.assertEqual(len(self.stops()),1)

    def test_exact_dormant_enablement_remains_in_the_original_removal_fence(self):
        name = 'kura-management-console@later.service'
        self.listed.return_value = [name]
        self.control.begin(self.id)
        stopped = self.control.stop(self.id)
        self.assertEqual(stopped['units'], [self.setup.unit, name])

        def enablement(item):
            if item.unit==self.setup.unit:
                return 'disabled'
            if ['/bin/systemctl','disable','--runtime',name] in self.calls:
                self.listed.return_value = []
                return 'disabled'
            return 'enabled-runtime'
        with patch.object(Setup, 'removal_enablement', enablement):
            self.control.disable_for_removal(self.id)
            self.control.disable_for_removal(self.id)
        disables = [call for call in self.calls if call[1]=='disable']
        self.assertEqual(disables, [['/bin/systemctl','disable','--runtime',name]])
        value = self.control.journal(self.id)
        self.assertEqual(value['disabledDormantUnits'], [name])
        self.assertEqual(value['stopReceipts'], stopped['stopReceipts'])
        with patch.object(Setup,'verify_local_command',return_value={
                'maintenanceProtocolVersion':1,'binarySha256':'a'*64,'sourceManifestSha256':'b'*64}):
            new_state = self.root/'var/lib/kura-management-console-later'
            new_state.mkdir()
            with self.assertRaisesRegex(RuntimeError,'dormant console participant acquired state'):
                self.control.complete(self.id)
            new_state.rmdir()
            self.assertEqual(self.control.complete(self.id)['phase'],'COMPLETE')

    def test_wrong_identity_malformed_or_dangling_fence_cannot_release_writes(self):
        self.control.begin(self.id)
        with self.assertRaisesRegex(RuntimeError,'original|Original|uncertain|No such'):
            try:self.control.complete(str(uuid.uuid4()))
            except FileNotFoundError:raise RuntimeError('Original receipt missing')
        path=self.control.directory/'active.json';path.unlink();path.symlink_to('absent')
        with self.assertRaises(OSError):self.control.active()

    def test_cancel_retires_original_frozen_id_without_claiming_transport_delivery_or_stopping(self):
        self.control.begin(self.id)
        value=self.control.cancel(self.id)
        self.assertTrue(self.active);self.assertEqual(self.stops(),[])
        self.assertEqual(value['phase'],'COMPLETE')
        self.assertEqual(value['outcome'],'CANCELLED_BEFORE_STOP_INTENT')
        self.assertTrue(value['lateActionsFenced']);self.assertFalse(value['transportNonDeliveryCertified'])
        self.assertIsNone(self.control.active())
        # A late original request is rejected, not executed or retried.
        with self.assertRaisesRegex(RuntimeError,'Original'):self.control.stop(self.id)
        with self.assertRaisesRegex(RuntimeError,'reused'):self.control.begin(self.id)
        self.assertEqual(self.control.cancel(self.id),value);self.assertEqual(self.stops(),[])

    def test_cancel_unobserved_id_tombstone_blocks_late_begin_and_preserves_other_fence(self):
        self.control.begin(self.id);other=str(uuid.uuid4())
        value=self.control.cancel(other)
        self.assertEqual(value['outcome'],'CANCELLED_BEFORE_ACCEPTANCE')
        self.assertEqual(self.control.active()['operationId'],self.id)
        self.control.cancel(self.id)
        with self.assertRaisesRegex(RuntimeError,'reused'):self.control.begin(other)
        self.assertEqual(self.stops(),[])

    def test_cancel_refuses_after_unknown_native_stop_without_issuing_another_stop(self):
        self.control.begin(self.id);self.fail_stop=True
        with self.assertRaises(subprocess.TimeoutExpired):self.control.stop(self.id)
        with self.assertRaisesRegex(RuntimeError,'native action'):self.control.cancel(self.id)
        self.assertEqual(len(self.stops()),1);self.assertIsNotNone(self.control.active())

    def test_verified_cancel_receipt_fences_stop_before_explicit_remaining_pointer_release(self):
        self.control.begin(self.id)
        original=Path.unlink
        def fail_active(path,*args,**kwargs):
            if path==self.control.directory/'active.json':raise OSError('unit-only pointer release failure')
            return original(path,*args,**kwargs)
        with patch.object(Path,'unlink',fail_active):
            with self.assertRaises(OSError):self.control.cancel(self.id)
        observation=self.control.status()
        self.assertTrue(observation['activeFencePresent'])
        self.assertEqual(observation['active']['outcome'],'CANCELLED_BEFORE_STOP_INTENT')
        with self.assertRaisesRegex(RuntimeError,'UNKNOWN'):self.control.stop(self.id)
        value=self.control.cancel(self.id,finalize_only=True)
        self.assertEqual(value['phase'],'COMPLETE');self.assertIsNone(self.control.active())
        self.assertEqual(self.stops(),[])
        with self.assertRaisesRegex(RuntimeError,'verified cancellation'):
            self.control.cancel(str(uuid.uuid4()),finalize_only=True)

    def test_unsupported_live_legacy_is_not_stopped_or_frozen(self):
        binary=self.setup.path('/usr/bin/yofc-management-console');binary.parent.mkdir(parents=True);binary.write_bytes(b'unit legacy')
        with patch.object(HostControl,'supported',return_value=False):
            with self.assertRaisesRegex(RuntimeError,'offline adoption'):self.control.begin(self.id,offline_legacy=True)
        self.assertIsNone(self.control.active());self.assertEqual(self.stops(),[])

if __name__=='__main__':unittest.main()
