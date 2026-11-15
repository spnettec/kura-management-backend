"""Contract/fault tests; real Docker and Cockpit acceptance is recorded separately."""
import hashlib
import json
import io
import os
from pathlib import Path
import subprocess
import tempfile
import tarfile
import unittest
from unittest.mock import patch
import uuid
from docker_control import DockerControl, CATALOG, atomic
from setup import Setup


@unittest.skipUnless(os.geteuid()==0,'root-owned deployment fixtures')
class DockerControlTests(unittest.TestCase):
    def setUp(self):
        self.temporary=tempfile.TemporaryDirectory();self.addCleanup(self.temporary.cleanup)
        self.control=DockerControl(Setup(Path(self.temporary.name)));self.control.prepare()
        self.archive=b'approved fixture archive'
        self.image={'key':'unit-arm64','title':'Unit fixture','imageId':'sha256:'+'a'*64,'manifestId':'sha256:'+'c'*64,'architecture':'arm64',
            'archiveSha256':hashlib.sha256(self.archive).hexdigest(),'archiveSize':len(self.archive),
            'binarySha256':'b'*64,'maintenanceProtocolVersion':1}
        path=self.control.setup.path(CATALOG);path.parent.mkdir(parents=True);atomic(path,{'schemaVersion':1,'images':[self.image]})
        self.machine=patch('docker_control.platform.machine',return_value='aarch64');self.machine.start();self.addCleanup(self.machine.stop)

    def operation(self):return str(uuid.uuid4())

    def upload(self):
        operation=self.operation();self.control.prepare_upload(operation,self.image['key'])
        path=self.control.upload_path(operation);path.write_bytes(self.archive);path.chmod(0o600)
        return operation

    def test_exact_archive_hash_owner_and_symlink_are_checked_before_import(self):
        operation=self.upload();self.assertTrue(self.control.status(operation)['uploadReady'])
        path=self.control.upload_path(operation);path.write_bytes(b'x'*len(self.archive))
        with patch.object(self.control,'launch') as launch:
            with self.assertRaisesRegex(RuntimeError,'exact approved'):self.control.import_image(operation)
            launch.assert_not_called()
        path.unlink();other=self.control.directory/'other';other.write_bytes(self.archive);path.symlink_to(other)
        with self.assertRaises(OSError):self.control.status(operation)

    def test_catalog_refuses_mutable_image_tag_or_unbounded_archive(self):
        for key,value in [('imageId','latest'),('archiveSize',True),('archiveSize',257*1024*1024),('maintenanceProtocolVersion',0)]:
            changed={**self.image,key:value};atomic(self.control.setup.path(CATALOG),{'schemaVersion':1,'images':[changed]})
            with self.assertRaises(RuntimeError):self.control.catalog()

    def test_unknown_operation_fences_new_ids_across_browser_profiles(self):
        first=self.operation();self.control.save({'operationId':first,'kind':'DEPLOY','phase':'UNKNOWN'})
        with patch.object(self.control,'installed_image',return_value=True):
            with self.assertRaisesRegex(RuntimeError,'original deployment'):self.control.deploy(self.operation(),self.image['key'],'new-name',48443,48444)
        self.assertEqual(len(list((self.control.directory/'operations').glob('*.json'))),1)

    def test_lost_systemd_dispatch_is_not_resent_after_observation_or_duplicate_import(self):
        operation=self.upload()
        with patch('docker_control.subprocess.run',side_effect=subprocess.TimeoutExpired('systemd-run',8)) as run:
            self.assertEqual(self.control.import_image(operation)['phase'],'UNKNOWN')
            self.assertEqual(run.call_count,1)
        with patch.object(self.control,'live',return_value=False),patch.object(self.control,'launch') as launch:
            self.assertEqual(self.control.status(operation)['phase'],'UNKNOWN')
            self.assertEqual(self.control.import_image(operation)['phase'],'UNKNOWN')
            launch.assert_not_called()

    def test_pending_requires_a_live_matching_worker_and_missing_journal_is_unobserved(self):
        operation=self.operation();self.control.save({'operationId':operation,'kind':'DEPLOY','phase':'RUNNING'})
        with patch.object(self.control,'live',return_value=False):self.assertEqual(self.control.status(operation)['phase'],'UNKNOWN')
        with patch.object(self.control,'live',return_value=True):self.assertEqual(self.control.status(operation)['phase'],'PENDING')
        unseen=self.operation();self.assertEqual(self.control.status(unseen),{'operationId':unseen,'phase':'UNOBSERVED','transportNonDeliveryCertified':False})

    def test_lost_daemon_receipt_retains_original_intent_and_cannot_reenter_worker(self):
        operation=self.upload();record=self.control.record(operation);record['phase']='ACCEPTED';self.control.save(record)
        self.control.upload_path(operation).rename(self.control.directory/'uploads'/(operation+'.accepted.tar.gz'))
        with patch.object(self.control,'verify_archive_manifest'),patch.object(self.control,'docker',side_effect=subprocess.TimeoutExpired('docker load',120)) as docker:
            self.control.worker(operation);self.assertEqual(docker.call_count,1)
        saved=self.control.record(operation);self.assertEqual(saved['phase'],'UNKNOWN');self.assertIn('load',saved['intents']);self.assertNotIn('load',saved['receipts'])
        with patch.object(self.control,'docker') as docker:
            with self.assertRaisesRegex(RuntimeError,'already entered'):self.control.worker(operation)
            docker.assert_not_called()

    def test_invalid_name_ports_and_architecture_do_not_create_or_dispatch(self):
        for name,http,grpc in [('a;touch /tmp/b',48443,48444),('valid',48443,48443),('valid',True,48444),('valid',80,48444)]:
            with patch.object(self.control,'launch') as launch:
                with self.assertRaises(RuntimeError):self.control.deploy(self.operation(),self.image['key'],name,http,grpc)
                launch.assert_not_called()
        self.assertFalse(list((self.control.directory/'operations').glob('*.json')))
        with patch('docker_control.platform.machine',return_value='x86_64'):
            with self.assertRaises(RuntimeError):self.control.image(self.image['key'])

    def test_changed_parameters_cannot_reuse_an_existing_accepted_id(self):
        operation=self.operation()
        with patch.object(self.control,'installed_image',return_value=True),patch.object(self.control,'resolved_image',return_value=self.image['imageId']),patch.object(self.control,'launch',side_effect=lambda row:self.control.save(row) or row):
            self.control.deploy(operation,self.image['key'],'chosen',48443,48444)
        with patch.object(self.control,'launch') as launch:
            with self.assertRaisesRegex(RuntimeError,'original deployment parameters'):self.control.deploy(operation,self.image['key'],'other',48443,48444)
            launch.assert_not_called()

    def test_both_classic_config_and_containerd_manifest_addresses_are_immutable(self):
        for selected in ['imageId','manifestId']:
            def inspect(kind,digest):
                if digest!=self.image[selected]:raise RuntimeError('not in this store')
                return {'Id':digest,'Architecture':'arm64','Os':'linux'}
            with patch.object(self.control,'inspect',side_effect=inspect):self.assertEqual(self.control.resolved_image(self.image),self.image[selected])

    def test_missing_load_receipt_cannot_be_reconciled_from_image_presence(self):
        operation=self.operation();self.control.save({'operationId':operation,'kind':'IMPORT','phase':'UNKNOWN','receipts':{},'image':self.image})
        with patch.object(self.control,'resolved_image',return_value=self.image['manifestId']) as inspect:
            with self.assertRaisesRegex(RuntimeError,'successful load receipt'):self.control.verify_import(operation)
            inspect.assert_not_called()

    def test_original_successful_load_can_be_verified_without_reloading_or_changing_id(self):
        digest=lambda data:'sha256:'+hashlib.sha256(data).hexdigest()
        config=json.dumps({'architecture':'arm64','os':'linux'}).encode();self.image['imageId']=digest(config)
        manifest=json.dumps({'schemaVersion':2,'config':{'digest':digest(config)}}).encode();self.image['manifestId']=digest(manifest)
        index=json.dumps({'schemaVersion':2,'manifests':[{'digest':digest(manifest)}]}).encode();output=io.BytesIO()
        with tarfile.open(fileobj=output,mode='w:gz') as archive:
            for name,data in [('index.json',index),('blobs/sha256/'+digest(manifest)[7:],manifest),('blobs/sha256/'+digest(config)[7:],config)]:
                member=tarfile.TarInfo(name);member.size=len(data);archive.addfile(member,io.BytesIO(data))
        data=output.getvalue();self.image.update(archiveSha256=hashlib.sha256(data).hexdigest(),archiveSize=len(data))
        atomic(self.control.setup.path(CATALOG),{'schemaVersion':1,'images':[self.image]})
        operation=self.operation();path=self.control.directory/'uploads'/(operation+'.accepted.tar.gz');path.write_bytes(data);path.chmod(0o600)
        original={key:value for key,value in self.image.items() if key!='manifestId'}
        self.control.save({'operationId':operation,'kind':'IMPORT','phase':'UNKNOWN','image':original,
            'intents':{'load':{'issued':True}},'receipts':{'load':{'commandReturnedSuccess':True}}})
        with patch.object(self.control,'resolved_image',return_value=self.image['manifestId']),patch.object(self.control,'mutate') as mutate:
            record=self.control.verify_import(operation);self.assertEqual(record['phase'],'APPLIED');self.assertEqual(record['operationId'],operation)
            self.assertEqual(record['postLoadVerification']['dockerMutationsIssued'],0);self.assertEqual(path.read_bytes(),data)
            self.assertEqual(self.control.verify_import(operation)['phase'],'APPLIED');mutate.assert_not_called()


if __name__=='__main__':unittest.main()
