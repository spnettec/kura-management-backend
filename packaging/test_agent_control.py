"""Host bridge contracts; synthetic Root fixtures are not installed-host proof."""
import copy
from contextlib import redirect_stdout
import hashlib
import json
import io
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch
import uuid
import agent_control as module
from agent_control import AgentControl,FILES,SHARE,UNITS
from setup import Setup


class NativeAgentBridgeTests(unittest.TestCase):
    def setUp(self):
        self.control=AgentControl();self.initial=str(uuid.uuid4());self.operation=str(uuid.uuid4());self.calls=[]
        self.current={'schemaVersion':1,'profile':'native-nonroot-v1','initializationId':self.initial,
                      'sourceManifestSha256':'a'*64,'nativeUid':997,'controlGid':996,'permission':'NONE',
                      'requestId':None,'readiness':'READY'}
        self.result={'schemaVersion':1,'requestId':self.operation,'state':'APPLIED','currentReadiness':'READY',
                     'canControl':True,'unitStarted':False,'remoteEnabled':False,'replayed':False}
        def invoke(action,*args):
            self.calls.append((action,args))
            return copy.deepcopy(self.current if action=='inspect' else self.result)
        self.mock=patch.object(self.control,'invoke',side_effect=invoke);self.mock.start();self.addCleanup(self.mock.stop)

    @staticmethod
    def observation(active=False,connections=0):
        blocks=[]
        for unit in UNITS:
            socket=unit.endswith('.socket')
            blocks.append('\n'.join(['Id='+unit,'ActiveState='+('active' if active else 'inactive'),'Job=',
                                      ('NConnections='+str(connections)) if socket else 'MainPID='+('42' if active else '0')]))
        return '\n\n'.join(blocks)+'\n'

    def services(self,raw=None):
        return patch.object(module.subprocess,'run',return_value=subprocess.CompletedProcess([],0,raw or self.observation()))

    def test_each_saved_action_verifies_current_root_before_opening_its_request(self):
        for action in ['authorize','complete','status']:
            self.calls.clear()
            with self.assertRaisesRegex(RuntimeError,'Different native installation'):
                self.control.request(action,self.operation,str(uuid.uuid4()))
            self.assertEqual(self.calls,[('inspect',())])

    def test_permission_changes_only_through_fixed_original_context_command(self):
        with self.services():value=self.control.request('authorize',self.operation,self.initial)
        self.assertEqual(value,self.result)
        self.assertEqual(self.calls[-1],('authorize-kura-remote-switch',('--request-id',self.operation,'--initialization-id',self.initial)))

    def test_running_or_connected_or_job_pending_services_refuse_before_authorization(self):
        for raw in [self.observation(active=True),self.observation(connections=1),self.observation().replace('Job=','Job=123')]:
            self.calls.clear()
            with self.services(raw),self.assertRaisesRegex(RuntimeError,'stopped original services'):
                self.control.request('authorize',self.operation,self.initial)
            self.assertEqual(self.calls,[('inspect',())])

    def test_real_socket_observation_has_connections_instead_of_a_service_pid(self):
        with self.services():value=self.control.overview()
        self.assertTrue(value['allStopped']);self.assertEqual(len(value['services']),5)
        sockets=[row for row in value['services'] if row['unit'].endswith('.socket')]
        self.assertEqual([(row['pid'],row['connections']) for row in sockets],[(None,0),(None,0)])

    def test_missing_or_duplicate_unit_or_malformed_kernel_fields_never_mean_stopped(self):
        raw=self.observation()
        for changed in [raw.split('\n\n',1)[1],raw+'\n'+raw,raw.replace('NConnections=0','MainPID=0'),raw.replace('MainPID=0','MainPID=-1')]:
            with self.services(changed),self.assertRaises(RuntimeError):self.control.overview()

    def test_completion_is_only_for_original_pending_request_and_never_regrants(self):
        for permission in ['NONE','GRANTED']:
            self.current.update(permission=permission,requestId=None if permission=='NONE' else self.operation)
            with self.assertRaisesRegex(RuntimeError,'original pending'):self.control.request('complete',self.operation,self.initial)
        self.current.update(permission='PENDING',requestId=str(uuid.uuid4()))
        with self.assertRaisesRegex(RuntimeError,'original pending'):self.control.request('complete',self.operation,self.initial)
        self.current['requestId']=self.operation
        self.assertEqual(self.control.request('complete',self.operation,self.initial),self.result)

    def test_damaged_public_inspection_never_becomes_a_ready_installation(self):
        for key,value in [('schemaVersion',True),('nativeUid',True),('controlGid',0),('sourceManifestSha256','bad'),
                          ('permission','OTHER'),('requestId',self.operation),('privateKey','forbidden')]:
            with self.subTest(key=key):
                original=copy.deepcopy(self.current);self.current[key]=value
                with self.assertRaises((RuntimeError,ValueError)):self.control.inspect()
                self.current=original

    def test_unknown_reply_is_observed_without_a_second_dispatch_and_extra_data_is_rejected(self):
        self.result={key:value for key,value in self.result.items() if key not in ['canControl','currentReadiness']}
        self.result['state']='UNKNOWN'
        self.assertEqual(self.control.request('status',self.operation,self.initial)['state'],'UNKNOWN')
        self.assertEqual(len([row for row in self.calls if row[0]!='inspect']),1)
        self.result['privateKey']='forbidden'
        with self.assertRaisesRegex(RuntimeError,'outcome is unverified'):self.control.request('status',self.operation,self.initial)

    def test_invalid_or_nil_ids_cannot_reach_a_root_request(self):
        for identity in ['',str(uuid.uuid4()).upper(),'00000000-0000-0000-0000-000000000000','../../other']:
            self.calls.clear()
            with self.assertRaises((RuntimeError,ValueError)):self.control.request('status',identity,self.initial)
            self.assertEqual(self.calls,[])

    def test_installer_routes_only_fixed_host_agent_arguments(self):
        import installer
        for action in ['authorize','complete','status']:
            with self.subTest(action=action),patch.object(installer,'verify_installer_payload'),\
                    patch.object(installer,'AgentControl') as factory,patch.object(installer.os,'geteuid',return_value=0),\
                    patch.object(sys,'argv',['installer','agent-'+action,'--operation',self.operation,'--initialization',self.initial]),redirect_stdout(io.StringIO()):
                factory.return_value.request.return_value={}
                installer.main();factory.return_value.request.assert_called_once_with(action,self.operation,self.initial)

    def test_installer_rejects_extra_paths_and_wrong_target_arguments_before_agent_construction(self):
        import installer
        for arguments in [['agent-overview','--operation',self.operation],
                          ['agent-status','--operation',self.operation],
                          ['agent-authorize','--operation',self.operation,'--initialization',self.initial,'--instance','other'],
                          ['agent-authorize','--operation',self.operation,'--initialization',self.initial,'--package','/tmp/other']]:
            with patch.object(installer,'verify_installer_payload'),patch.object(installer,'AgentControl') as factory,\
                    patch.object(installer.os,'geteuid',return_value=0),patch.object(sys,'argv',['installer',*arguments]):
                with self.assertRaisesRegex(RuntimeError,'Fixed local native agent'):installer.main()
                factory.assert_not_called()

    def test_installer_requires_host_admin_before_provenance_or_agent_dispatch(self):
        import installer
        with patch.object(installer,'verify_installer_payload') as verify,patch.object(installer,'AgentControl') as factory,\
                patch.object(installer.os,'geteuid',return_value=997),patch.object(sys,'argv',['installer','agent-overview']):
            with self.assertRaisesRegex(RuntimeError,'Host administrator'):installer.main()
            verify.assert_not_called();factory.assert_not_called()


@unittest.skipUnless(os.geteuid()==0,'disposable Root filesystem fixture required')
class NativeAgentPayloadTests(unittest.TestCase):
    def setUp(self):
        temporary=tempfile.TemporaryDirectory();self.addCleanup(temporary.cleanup)
        self.setup=Setup(Path(temporary.name));self.control=AgentControl(self.setup);self.payload={}
        for name in FILES:self.write(name,b'unit-only fixed native file: '+name.encode())
        self.write(SHARE+'package-v2',b'kura-management-agent-v2\n')
        files={name:hashlib.sha256(self.payload[name]).hexdigest() for name in FILES}
        self.write(SHARE+'native-profile.json',json.dumps({'schemaVersion':1,'profile':'native-nonroot-v1','files':files}).encode())
        self.write(SHARE+'source-manifest.json',json.dumps({'schemaVersion':2,'sourceFiles':[
            {'path':'packaging/'+FILES[name],'sha256':digest} for name,digest in files.items()]}).encode())

    def write(self,name,data):
        path=self.setup.path(name);path.parent.mkdir(parents=True,exist_ok=True);path.write_bytes(data);path.chmod(0o644)
        self.payload[name]=data;return path

    def test_root_paired_payload_is_checked_without_executing_fixture_programs(self):
        with patch.object(module.subprocess,'run') as run:
            self.control.verify();run.assert_not_called()

    def test_changed_script_or_native_file_set_refuses_before_execution(self):
        self.write(module.PROGRAM,b'changed code')
        with patch.object(module.subprocess,'run') as run,self.assertRaisesRegex(RuntimeError,'Root source pair'):
            self.control.invoke('inspect')
        run.assert_not_called()

    def test_nonroot_owner_writable_parent_link_or_hardlink_is_never_imported(self):
        path=self.setup.path(module.PROGRAM)
        for damage,restore in [(lambda:os.chown(path,997,0),lambda:os.chown(path,0,0)),
                               (lambda:path.parent.chmod(0o777),lambda:path.parent.chmod(0o755)),
                               (lambda:os.link(path,path.with_name('other-link')),lambda:path.with_name('other-link').unlink())]:
            damage()
            try:
                with self.assertRaises(RuntimeError):self.control.verify()
            finally:restore()
        other=path.with_name('original');path.rename(other);path.symlink_to(other)
        with self.assertRaises(RuntimeError):self.control.verify()

    def test_duplicate_json_properties_are_rejected_without_last_value_wins(self):
        self.write(SHARE+'native-profile.json',b'{"schemaVersion":0,"schemaVersion":1}')
        with self.assertRaisesRegex(RuntimeError,'Duplicate native control'):self.control.verify()


if __name__=='__main__':unittest.main()
