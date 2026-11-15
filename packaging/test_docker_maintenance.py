"""Fault contracts for Docker maintenance, separate from real engine acceptance."""
import json
import os
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch
import uuid
from docker_control import atomic
from docker_maintenance import MaintenanceControl
from setup import Setup


@unittest.skipUnless(os.geteuid()==0,'private root records')
class MaintenanceTests(unittest.TestCase):
    def setUp(self):
        self.temporary=tempfile.TemporaryDirectory();self.addCleanup(self.temporary.cleanup)
        self.control=MaintenanceControl(Setup(Path(self.temporary.name)));self.control.prepare()
        self.operation=str(uuid.uuid4());self.deployment=str(uuid.uuid4())
        self.old={'operationId':self.deployment,'kind':'DEPLOY','phase':'APPLIED','name':'owned',
            'containerId':'a'*64,'volume':'owned-volume','image':{'maintenanceProtocolVersion':1,'imageId':'sha256:'+'1'*64}}
        self.record={'operationId':self.operation,'deploymentId':self.deployment,'kind':'MAINTENANCE','phase':'REQUESTED',
            'oldGeneration':self.old,'stageIntents':{'freeze':{'dispatchIssued':True}},'stageClaims':{},
            'intents':{},'receipts':{},'liveStage':'freeze'}
        self.control.save(self.record)

    def test_unsettled_installation_is_refused_before_restart_policy_or_freeze(self):
        with patch.object(self.control,'verified_read',side_effect=RuntimeError('unsettled')),patch.object(self.control,'mutate') as mutate:
            self.control.worker_stage(self.operation,'freeze');mutate.assert_not_called()
        self.assertEqual(self.control.record(self.operation)['phase'],'REFUSED')

    def test_unconfigured_installation_is_refused_without_policy_or_fence_changes(self):
        with patch.object(self.control,'verified_read',return_value={'configured':False}) as read,patch.object(self.control,'mutate') as mutate:
            self.control.worker_stage(self.operation,'freeze');mutate.assert_not_called()
            self.assertEqual(read.call_count,1)
        self.assertEqual(self.control.record(self.operation)['phase'],'REFUSED')

    def test_missing_docker_stop_receipt_is_retained_and_never_dispatched_again(self):
        row=self.record;row.update(phase='REQUESTED',liveStage='stop',stageIntents={'stop':{'dispatchIssued':True}});self.control.save(row)
        container={'State':{'Running':True},'HostConfig':{'RestartPolicy':{'Name':'no'}}}
        process={'pid':42,'startTime':'123','uid':999,'exeSha256':'a'*64};descriptor=os.open(os.devnull,os.O_RDONLY);self.addCleanup(os.close,descriptor)
        original_read=Path.read_text
        def read(path,*args,**kwargs):return '42 (approved) S '+'0 '*18+'123' if str(path)=='/proc/42/stat' else original_read(path,*args,**kwargs)
        with patch.object(self.control,'frozen'),patch.object(self.control,'managed_container',return_value=container),patch.object(self.control,'runtime_process',return_value=process),patch('docker_maintenance.os.pidfd_open',side_effect=lambda *_:os.dup(descriptor)),patch('docker_maintenance.signal.pidfd_send_signal',side_effect=OSError('lost signal return')) as send,patch.object(Path,'read_text',read):
            self.control.worker_stage(self.operation,'stop');self.assertEqual(send.call_count,1)
        saved=self.control.record(self.operation);self.assertEqual(saved['phase'],'UNKNOWN');self.assertIn('stop',saved['intents']);self.assertNotIn('stop',saved['receipts'])
        with patch.object(self.control,'launch_stage') as dispatch:
            self.assertEqual(self.control.request(self.operation,'stop')['phase'],'UNKNOWN');dispatch.assert_not_called()
        with self.assertRaises(RuntimeError):self.control.worker_stage(self.operation,'stop')

    def test_signal_observation_timeout_never_escalates_or_invents_a_stop_receipt(self):
        row=self.record;row.update(phase='REQUESTED',liveStage='stop',stageIntents={'stop':{'dispatchIssued':True}});self.control.save(row)
        running={'State':{'Running':True,'Pid':42,'Restarting':False},'HostConfig':{'RestartPolicy':{'Name':'no'}}}
        def delivered(record,process):
            record['intents']['stop']={'issued':True};record['receipts']['stop']={'signalDelivered':True};self.control.save(record)
        with patch.object(self.control,'frozen'),patch.object(self.control,'managed_container',return_value=running),patch.object(self.control,'runtime_process',return_value={'pid':42}),patch.object(self.control,'signal_once',side_effect=delivered) as send,patch('docker_maintenance.time.sleep'):
            self.control.worker_stage(self.operation,'stop');self.assertEqual(send.call_count,1)
        saved=self.control.record(self.operation);self.assertEqual(saved['phase'],'UNKNOWN');self.assertNotIn('stopReceipt',saved)
        self.assertIn('stop',saved['receipts']) # Signal delivery is not process-stop proof.

    def test_a_stopped_process_without_original_stop_receipt_cannot_start_a_generation(self):
        row=self.record;row.update(phase='RUNNING',liveStage='upgrade')
        with patch.object(self.control,'managed_container',return_value={'State':{'Running':False,'Pid':0},'HostConfig':{'RestartPolicy':{'Name':'no'}}}),patch.object(self.control,'create_generation') as create:
            with self.assertRaisesRegex(RuntimeError,'Original stop'):self.control.transition(row)
            create.assert_not_called()

    def test_protocol_one_cannot_resume_by_unfreezing_before_start(self):
        row=self.record;row.update(phase='STOPPED',stageIntents={'freeze':{},'stop':{}});self.control.save(row)
        with patch.object(self.control,'launch_stage') as dispatch:
            with self.assertRaisesRegex(RuntimeError,'fenced startup'):self.control.request(self.operation,'resume')
            dispatch.assert_not_called()

    def test_upgrade_rejects_same_image_before_any_dispatch(self):
        row=self.record;row.update(phase='STOPPED',stageIntents={});self.control.save(row)
        image={**self.old['image'],'maintenanceProtocolVersion':2,'fencedStartupCommand':'serve-maintenance'}
        with patch.object(self.control,'image',return_value=image),patch.object(self.control,'installed_image',return_value=True),patch.object(self.control,'launch_stage') as dispatch:
            with self.assertRaisesRegex(RuntimeError,'different approved'):self.control.request(self.operation,'upgrade','same')
            dispatch.assert_not_called()

    def test_wrong_deployment_cannot_select_another_generations_volume(self):
        self.control.save(self.old);generation=str(uuid.uuid4())
        atomic(self.control.directory/'deployments/owned.json',{'operationId':self.deployment,'generationId':generation})
        atomic(self.control.directory/'generations'/(generation+'.json'),{'operationId':generation,'deploymentId':str(uuid.uuid4()),'volume':'someone-else'})
        with self.assertRaisesRegex(RuntimeError,'Generation binding'):self.control.generation(self.deployment)

    def test_started_review_rejects_missing_start_or_unacknowledged_mutations(self):
        for receipts in [{},{'start-generation':{'commandReturnedSuccess':True}}]:
            row={**self.record,'phase':'UNKNOWN','newGeneration':{'containerId':'b'*64},'stopReceipt':{'successfulReturn':True},
                'intents':{'start-generation':{},'complete':{}},'receipts':receipts}
            with patch.object(self.control,'frozen') as check:
                with self.assertRaises(RuntimeError):self.control.reviewable_start(row)
                check.assert_not_called()

    def test_started_review_only_accepts_original_selected_stopped_and_frozen_generation(self):
        row={**self.record,'phase':'UNKNOWN','newGeneration':{'containerId':'b'*64},'stopReceipt':{'successfulReturn':True},
             'intents':{'start-generation':{}},'receipts':{'start-generation':{'commandReturnedSuccess':True}}}
        with patch.object(self.control,'generation',return_value=self.old),patch.object(self.control,'managed_container',return_value={'State':{'Running':False,'Pid':0}}),patch.object(self.control,'frozen') as check,patch.object(self.control,'mutate') as mutate:
            self.control.reviewable_start(row);check.assert_called_once_with(row,row['newGeneration']);mutate.assert_not_called()


if __name__=='__main__':unittest.main()
