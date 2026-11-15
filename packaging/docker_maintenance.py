#!/usr/bin/python3
"""Original-ID Docker maintenance and immutable generation transitions.

An upgraded server starts under the retained private fence, is checked while
writes remain frozen, becomes the selected generation, then completes that exact
fence. Every old container and journal is retained. No uncertain command repeats.
"""
import hashlib
import json
import os
from pathlib import Path
import re
import signal
import subprocess
import sys
import tarfile
import time
from docker_control import DockerControl,DOCKER,ENV,LABEL,OWNER,ROLE,atomic,identity,protected

PROGRAM='/usr/lib/kura-management-console/docker_maintenance.py'
BINARY='/usr/local/bin/yofc-management-console'
STAGES={'freeze','stop','resume','upgrade','verify-start'}


class MaintenanceControl(DockerControl):
    def unit_stage(self,operation,stage):
        if stage not in STAGES:raise RuntimeError('Invalid maintenance stage')
        return 'yofc-console-docker-maint-'+identity(operation)+'-'+stage+'.service'

    def begin(self,operation,deployment):
        identity(operation);identity(deployment)
        with self.lock():
            self.no_other_job(operation)
            if self.path(operation).exists():
                record=self.record(operation)
                if record.get('kind')!='MAINTENANCE' or record.get('deploymentId')!=deployment:raise RuntimeError('Retain original maintenance parameters')
                return self.status(operation)
            generation=self.generation(deployment);container=self.managed_container(generation)
            if not hasattr(os,'pidfd_open') or not hasattr(signal,'pidfd_send_signal'):raise RuntimeError('Verified process signaling is unavailable on this host')
            if generation['image']['maintenanceProtocolVersion']==1 and not any(
                    image['maintenanceProtocolVersion']==2 and self.installed_image(image) for image in self.catalog()):
                raise RuntimeError('Import a supported fenced-startup upgrade image before freezing this version')
            if not container['State']['Running'] or container['State'].get('Restarting'):raise RuntimeError('Select a settled running center')
            if container['HostConfig']['RestartPolicy']['Name']!='unless-stopped':raise RuntimeError('Unexpected original restart policy')
            record={'schemaVersion':1,'kind':'MAINTENANCE','operationId':operation,'deploymentId':deployment,
                'oldGeneration':generation,'phase':'PREPARED','stageIntents':{},'stageClaims':{},'intents':{},'receipts':{}}
            return self.launch_stage(record,'freeze')

    def request(self,operation,stage,image_key=None):
        with self.lock():
            record=self.record(operation);self.no_other_job(operation)
            if record['kind']!='MAINTENANCE':raise RuntimeError('Select original maintenance')
            if stage=='verify-start':
                if stage in record['stageIntents']:return self.status(operation)
                self.reviewable_start(record)
                record['priorStartReview']={'previousPhase':record['phase'],'failureType':record.get('failureType'),
                    'reason':record.get('reason'),'existingStartReceiptRequired':True,'newStartIssued':False}
                return self.launch_stage(record,stage)
            if stage in record['stageIntents'] or record['phase']=='UNKNOWN':return self.status(operation)
            expected='FROZEN' if stage=='stop' else 'STOPPED'
            if stage not in ['stop','resume','upgrade'] or record['phase']!=expected:raise RuntimeError('Observe original maintenance stage')
            if stage in ['resume','upgrade']:
                image=self.image(image_key) if stage=='upgrade' else record['oldGeneration']['image']
                if image['maintenanceProtocolVersion']!=2 or image.get('fencedStartupCommand')!='serve-maintenance' or not self.installed_image(image):
                    raise RuntimeError('Select an imported image with verified fenced startup')
                if stage=='upgrade' and image['imageId']==record['oldGeneration']['image']['imageId']:raise RuntimeError('Upgrade requires a different approved image')
                record['selectedImage']=image
            return self.launch_stage(record,stage)

    def launch_stage(self,record,stage):
        if stage in record['stageIntents']:return self.status(record['operationId'])
        record['stageIntents'][stage]={'dispatchIssued':True};record['liveStage']=stage;record['phase']='REQUESTED';self.save(record)
        try:
            result=subprocess.run(['systemd-run','--quiet','--unit',self.unit_stage(record['operationId'],stage),
                '--property=Type=exec','--property=RuntimeMaxSec=600','--property=MemoryMax=384M','--property=CPUQuota=100%',
                '--property=StandardOutput=null','--property=StandardError=null','/usr/bin/python3',PROGRAM,
                'worker','--operation',record['operationId'],'--stage',stage],env=ENV,capture_output=True,timeout=8)
            if result.returncode:raise RuntimeError('Dispatch requires original-operation review')
        except (OSError,RuntimeError,subprocess.TimeoutExpired):return {'operationId':record['operationId'],'phase':'UNKNOWN'}
        return {'operationId':record['operationId'],'kind':'MAINTENANCE','phase':'ACCEPTED','deploymentId':record['deploymentId']}

    def verified_read(self,generation,command):
        if command not in ['installation-maintenance-check','installation-status','maintenance-status']:raise RuntimeError('Fixed read command required')
        self.managed_container(generation)
        metadata=self.docker('exec','--user','console',generation['containerId'],'stat','-c','%u %a',
            '/usr/share/kura-management-console/local-command-provenance.json',BINARY)
        owners=[line.split() for line in metadata.splitlines()]
        if len(owners)!=2 or any(owner!='0' or int(mode,8)&0o022 for owner,mode in owners):raise RuntimeError('Unprotected container payload')
        provenance=json.loads(self.docker('exec','--user','console',generation['containerId'],'cat','/usr/share/kura-management-console/local-command-provenance.json'))
        digest=self.docker('exec','--user','console',generation['containerId'],'sha256sum',BINARY).split()[0]
        if digest!=generation['image']['binarySha256'] \
                or provenance.get('binarySha256')!=generation['image']['binarySha256'] or provenance.get('schemaVersion')!=1:
            raise RuntimeError('Running program differs from approved image')
        self.runtime_process(generation)
        if command!='maintenance-status' and command not in provenance.get('localReadCommands',[]):raise RuntimeError('Unsupported read capability')
        if command=='maintenance-status' and generation['image']['maintenanceProtocolVersion'] not in [1,2]:raise RuntimeError('Unsupported maintenance capability')
        return json.loads(self.docker('exec','--user','console',generation['containerId'],BINARY,command))

    def worker_stage(self,operation,stage):
        with self.lock():
            record=self.record(operation)
            if record.get('kind')!='MAINTENANCE' or record.get('phase')!='REQUESTED' or record.get('liveStage')!=stage or stage in record['stageClaims']:
                raise RuntimeError('Never re-enter a maintenance stage')
            record['stageClaims'][stage]=True;record['phase']='RUNNING';self.save(record)
        try:
            if stage=='freeze':self.freeze(record)
            elif stage=='stop':self.stop(record)
            elif stage in ['resume','upgrade']:self.transition(record)
            elif stage=='verify-start':self.finish_transition(record,record['newGeneration'])
            else:raise RuntimeError('Unsupported stage')
        except Exception as error:
            record['phase']='UNKNOWN' if record['intents'] else 'REFUSED'
            record['reason']='REVIEW_ORIGINAL_OPERATION';record['failureType']=type(error).__name__;self.save(record)

    def freeze(self,record):
        old=record['oldGeneration']
        if self.verified_read(old,'installation-status').get('configured') is not True:
            raise RuntimeError('Complete installation before entering container maintenance')
        check=self.verified_read(old,'installation-maintenance-check')
        if check!={'ok':True}:raise RuntimeError('Installation is unsettled')
        # Prevent the old command auto-starting after a daemon/host reboot while
        # a private fence is retained. This policy mutation has its own receipt.
        self.mutate(record,'disable-restart','container','update','--restart=no',old['containerId'])
        if self.managed_container(old)['HostConfig']['RestartPolicy']['Name']!='no':raise RuntimeError('Restart policy could not be verified')
        value=json.loads(self.mutate(record,'freeze','exec','--user','console',old['containerId'],BINARY,'maintenance-begin',record['operationId']))
        if value.get('operationId')!=record['operationId'] or value.get('phase')!='FROZEN':raise RuntimeError('Freeze receipt differs')
        record['freezeReceipt']=value;record['phase']='FROZEN';self.save(record)

    def frozen(self,record,generation):
        active=self.verified_read(generation,'maintenance-status').get('active')
        if not active or active.get('operationId')!=record['operationId'] or active.get('phase')!='FROZEN':raise RuntimeError('Original fence is unavailable')

    def stop(self,record):
        old=record['oldGeneration'];self.frozen(record,old);container=self.managed_container(old)
        if not container['State']['Running'] or container['HostConfig']['RestartPolicy']['Name']!='no':raise RuntimeError('Original process or restart policy differs')
        # The server handles SIGINT. PID 1 must not be left to Docker's default
        # SIGTERM timeout followed by SIGKILL. Send once, observe, never escalate.
        process=self.runtime_process(old);self.signal_once(record,process)
        for _ in range(60):
            container=self.managed_container(old)
            if not container['State']['Running'] and not container['State'].get('Restarting') and container['State'].get('Pid',0)==0:break
            time.sleep(0.5)
        if container['State']['Running'] or container['State'].get('Restarting') or container['State'].get('Pid',0)!=0 \
                or container['State'].get('OOMKilled') or container['State'].get('ExitCode')!=0:raise RuntimeError('Stop requires original-operation review')
        record['stopReceipt']={'containerId':old['containerId'],'successfulReturn':True,'running':False,'pid':0,'signal':'SIGINT','exitCode':0,'forcedSignalIssued':False}
        record['phase']='STOPPED';self.save(record)

    def signal_once(self,record,process):
        descriptor=os.pidfd_open(process['pid'],0)
        try:
            current=(Path('/proc')/str(process['pid'])/'stat').read_text().rpartition(')')[2].split()[19]
            if current!=process['startTime']:raise RuntimeError('Original process identity changed')
            if 'stop' in record['intents']:raise RuntimeError('Never repeat a process signal')
            record['intents']['stop']={'issued':True,'action':'pidfd SIGINT','process':process};self.save(record)
            signal.pidfd_send_signal(descriptor,signal.SIGINT)
            record['receipts']['stop']={'signalDelivered':True,'signal':'SIGINT','process':process};self.save(record)
        finally:os.close(descriptor)

    def payload_file(self,container,filename,maximum):
        # Docker cp streams one root-owned regular file from the immutable
        # container, without starting it or extracting a path on the host.
        process=subprocess.Popen([*DOCKER,'cp',container+':'+filename,'-'],env=ENV,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
        try:
            with tarfile.open(fileobj=process.stdout,mode='r|') as archive:
                member=archive.next()
                if not member or not member.isfile() or member.uid!=0 or member.mode&0o022 or member.size>maximum \
                        or member.name.removeprefix('./')!=Path(filename).name:raise RuntimeError('Unprotected image file')
                stream=archive.extractfile(member);digest=hashlib.sha256();content=[]
                while block:=stream.read(1024*1024):
                    digest.update(block)
                    if maximum<=65536:content.append(block)
                if archive.next() is not None:raise RuntimeError('Unexpected image archive entries')
            _,error=process.communicate(timeout=15)
            if process.returncode:raise RuntimeError('Image file could not be read')
            return digest.hexdigest(),b''.join(content)
        finally:
            if process.poll() is None:process.kill();process.wait()

    def verify_candidate(self,generation):
        _,data=self.payload_file(generation['containerId'],'/usr/share/kura-management-console/local-command-provenance.json',65536)
        provenance=json.loads(data);digest,_=self.payload_file(generation['containerId'],BINARY,128*1024*1024)
        if digest!=generation['image']['binarySha256'] or provenance.get('binarySha256')!=digest \
                or provenance.get('maintenanceProtocolVersion')!=2 or provenance.get('fencedStartupCommand')!='serve-maintenance' \
                or set(provenance.get('maintenanceCommands',[]))!={'maintenance-status','maintenance-begin','maintenance-complete'}:
            raise RuntimeError('Candidate fenced-startup capability differs')

    def create_generation(self,record):
        old=record['oldGeneration'];operation=record['operationId'];image=record['selectedImage'];resolved=self.resolved_image(image)
        if not resolved:raise RuntimeError('Approved target image unavailable')
        generation={**{key:old[key] for key in ['name','volume','httpPort','grpcPort']},'schemaVersion':1,'kind':'DEPLOY',
            'deploymentId':record['deploymentId'],'operationId':operation,'image':image,'engineImageId':resolved,
            'containerName':'yofc-console-'+old['name']+'-g-'+operation,'phase':'PREPARED','stopSignal':'SIGINT','command':['serve-maintenance',operation]}
        volume=self.inspect('volume',old['volume'])
        if volume.get('Labels',{}).get(OWNER)!=record['deploymentId']:raise RuntimeError('Retained volume ownership differs')
        arguments=['container','create','--name',generation['containerName'],'--label',LABEL+'=1','--label',OWNER+'='+record['deploymentId'],
            '--label',ROLE+'=center','--restart=no','--stop-signal=SIGINT','--read-only','--security-opt','no-new-privileges','--cap-drop=ALL',
            '--memory=512m','--memory-swap=512m','--cpus=1','--pids-limit=128','--tmpfs','/run:rw,nosuid,nodev,size=16m',
            '--tmpfs','/tmp:rw,nosuid,nodev,noexec,size=16m','--mount','type=volume,src='+old['volume']+',dst=/var/lib/kura-management-console,volume-nocopy',
            '--publish',str(old['httpPort'])+':18443/tcp','--publish',str(old['grpcPort'])+':18444/tcp','--no-healthcheck']
        for capability in ['CHOWN','DAC_OVERRIDE','FOWNER','SETUID','SETGID']:arguments+=['--cap-add',capability]
        container=self.mutate(record,'create-generation',*arguments,resolved,*generation['command']).strip()
        if not re.fullmatch(r'[a-f0-9]{64}',container):raise RuntimeError('Missing exact generation ID')
        generation['containerId']=container;record['newGeneration']=generation;self.save(record)
        self.managed_container(generation);self.verify_candidate(generation)
        return generation

    def transition(self,record):
        old=record['oldGeneration'];container=self.managed_container(old)
        if not record.get('stopReceipt') or container['State']['Running'] or container['State']['Pid']!=0 \
                or container['HostConfig']['RestartPolicy']['Name']!='no':raise RuntimeError('Original stop is unverified')
        generation=self.create_generation(record)
        self.mutate(record,'start-generation','container','start',generation['containerId'])
        self.finish_transition(record,generation)

    def reviewable_start(self,record):
        if record.get('phase')!='UNKNOWN' or not record.get('newGeneration') or not record.get('stopReceipt') \
                or record.get('receipts',{}).get('start-generation')!={'commandReturnedSuccess':True} \
                or set(record.get('intents',{}))!=set(record.get('receipts',{})) or 'complete' in record.get('intents',{}):
            raise RuntimeError('Original successful start and no missing mutation receipt are required')
        selected=self.generation(record['deploymentId'])
        if selected['containerId']!=record['oldGeneration']['containerId']:raise RuntimeError('Selected generation already changed; explicit review required')
        old=self.managed_container(record['oldGeneration'])
        if old['State']['Running'] or old['State']['Pid']!=0:raise RuntimeError('Original stop no longer matches')
        self.frozen(record,record['newGeneration'])

    def finish_transition(self,record,generation):
        old=record['oldGeneration']
        from installer import probe_listener
        ready=False
        for _ in range(30):
            container=self.managed_container(generation)
            if not container['State']['Running']:raise RuntimeError('Candidate stopped before verification')
            try:
                self.frozen(record,generation);description=self.verified_read(generation,'installation-status')
                if not description.get('configured'):raise RuntimeError('Configured installation required')
                description['httpAddress']='127.0.0.1:'+str(generation['httpPort']);description['httpPort']=generation['httpPort']
                ready=probe_listener(description,readiness=True)
            except (OSError,ValueError,RuntimeError,subprocess.SubprocessError):ready=False
            if ready:break
            time.sleep(1)
        if not ready:raise RuntimeError('Candidate readiness requires original-operation review')
        record['fencedStartupVerified']={'containerId':generation['containerId'],'certificateFingerprint':description['certificateFingerprint'],
            'configured':True,'databaseReadiness':True,'originalFenceStillPresent':True};self.save(record)
        # Select the actual verified generation while its APIs remain frozen.
        # A crash cannot silently select the old image after possible migrations.
        generation['phase']='APPLIED';atomic(self.directory/'generations'/(record['operationId']+'.json'),generation)
        atomic(self.directory/'deployments'/(old['name']+'.json'),{'operationId':record['deploymentId'],'name':old['name'],'generationId':record['operationId']})
        value=json.loads(self.mutate(record,'complete','exec','--user','console',generation['containerId'],BINARY,'maintenance-complete',record['operationId']))
        if value.get('operationId')!=record['operationId'] or value.get('state')!='APPLIED':raise RuntimeError('Completion receipt differs')
        if self.verified_read(generation,'maintenance-status').get('active') is not None:raise RuntimeError('Original fence is retained')
        self.mutate(record,'enable-restart','container','update','--restart=unless-stopped',generation['containerId'])
        current=self.managed_container(generation)
        if not current['State']['Running'] or current['HostConfig']['RestartPolicy']['Name']!='unless-stopped':raise RuntimeError('Final process policy differs')
        record['completionReceipt']=value;record['phase']='COMPLETE';self.save(record)


if __name__=='__main__':
    try:
        if os.geteuid()!=0 or len(sys.argv)!=6 or sys.argv[1:3]!=['worker','--operation'] or sys.argv[4]!='--stage' or sys.argv[5] not in STAGES:raise RuntimeError('Invalid fixed worker invocation')
        from installer import verify_installer_payload
        verify_installer_payload();MaintenanceControl().worker_stage(identity(sys.argv[3]),sys.argv[5])
    except Exception:sys.exit(1)
