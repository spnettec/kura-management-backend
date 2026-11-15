#!/usr/bin/python3
"""Root-owned Docker deployment jobs. No shell, registry login or user environment.

Accepted jobs outlive the Cockpit channel. A missing receipt is never replayed,
even when the daemon happens to contain an object with the expected name.
"""
import contextlib
import fcntl
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import stat
import subprocess
import sys
import tarfile
import tempfile
import uuid
from setup import Setup

BASE='/var/lib/yofc-console-installations'
CATALOG='/usr/share/kura-management-console-installer/images.json'
PROGRAM='/usr/lib/kura-management-console/docker_control.py'
DOCKER=['/usr/bin/docker','--host','unix:///var/run/docker.sock','--config','/etc/kura-management-console/docker-client']
ENV={'PATH':'/usr/sbin:/usr/bin:/sbin:/bin'}
LABEL='com.yofc.console.installation'
OWNER='com.yofc.console.deployment'
ROLE='com.yofc.console.role'
MAX_ARCHIVE=256*1024*1024
TERMINAL={'APPLIED','REFUSED','COMPLETE'}


def identity(value):
    if str(uuid.UUID(value))!=value:raise ValueError('Canonical operation ID required')
    return value


def protected(path,maximum=4*1024*1024):
    with os.fdopen(os.open(path,os.O_RDONLY|os.O_NOFOLLOW|os.O_NONBLOCK),'rb') as stream:
        info=os.fstat(stream.fileno())
        if not stat.S_ISREG(info.st_mode) or info.st_uid!=0 or info.st_mode&0o022 or info.st_size>maximum:
            raise RuntimeError('Root-protected regular file required')
        data=stream.read(maximum+1)
        if len(data)>maximum:raise RuntimeError('File too large')
        return data


def atomic(path,value):
    data=(json.dumps(value,sort_keys=True,indent=2)+'\n').encode()
    descriptor,name=tempfile.mkstemp(prefix='.pending-',dir=path.parent)
    try:
        with os.fdopen(descriptor,'wb') as output:
            output.write(data);output.flush();os.fsync(output.fileno())
        os.replace(name,path)
        directory=os.open(path.parent,os.O_RDONLY|os.O_DIRECTORY)
        try:os.fsync(directory)
        finally:os.close(directory)
    finally:
        if os.path.exists(name):os.unlink(name)


class DockerControl:
    def __init__(self,setup=None):
        self.setup=setup or Setup()
        self.directory=self.setup.path(BASE)

    def prepare(self):
        self.directory.mkdir(mode=0o700,parents=True,exist_ok=True)
        for directory in [self.directory,self.directory/'operations',self.directory/'deployments',self.directory/'uploads',self.directory/'generations']:
            directory.mkdir(mode=0o700,exist_ok=True)
            info=directory.lstat()
            if not stat.S_ISDIR(info.st_mode) or info.st_uid!=0 or info.st_mode&0o077:
                raise RuntimeError('Private root deployment directory required')

    @contextlib.contextmanager
    def lock(self):
        self.prepare()
        descriptor=os.open(self.directory/'control.lock',os.O_RDWR|os.O_CREAT|os.O_NOFOLLOW,0o600)
        try:
            info=os.fstat(descriptor)
            if not stat.S_ISREG(info.st_mode) or info.st_uid!=0 or info.st_mode&0o077:raise RuntimeError('Invalid deployment lock')
            fcntl.flock(descriptor,fcntl.LOCK_EX)
            yield
        finally:os.close(descriptor)

    def path(self,operation):return self.directory/'operations'/(identity(operation)+'.json')

    def record(self,operation):return json.loads(protected(self.path(operation)))

    def save(self,record):atomic(self.path(record['operationId']),record)

    def catalog(self):
        catalog=json.loads(protected(self.setup.path(CATALOG)))
        if catalog.get('schemaVersion')!=1 or not isinstance(catalog.get('images'),list) or len(catalog['images'])>64:
            raise RuntimeError('Invalid approved image catalog')
        keys=set()
        for image in catalog['images']:
            if not re.fullmatch(r'[a-z0-9][a-z0-9-]{0,63}',image.get('key','')) or image['key'] in keys \
                    or not re.fullmatch(r'sha256:[a-f0-9]{64}',image.get('imageId','')) \
                    or not re.fullmatch(r'sha256:[a-f0-9]{64}',image.get('manifestId','')) \
                    or any(not re.fullmatch(r'[a-f0-9]{64}',image.get(field,'')) for field in ['archiveSha256','binarySha256']) \
                    or type(image.get('archiveSize')) is not int or not 0<image['archiveSize']<=MAX_ARCHIVE \
                    or image.get('architecture') not in ['arm64','amd64'] \
                    or type(image.get('maintenanceProtocolVersion')) is not int or image.get('maintenanceProtocolVersion') not in [1,2] \
                    or image.get('maintenanceProtocolVersion')==2 and image.get('fencedStartupCommand')!='serve-maintenance' \
                    or not isinstance(image.get('title'),str) or not 0<len(image['title'])<=128:
                raise RuntimeError('Invalid approved image')
            keys.add(image['key'])
        return catalog['images']

    def image(self,key):
        image=next((value for value in self.catalog() if value['key']==key),None)
        if not image or image['architecture']!={'aarch64':'arm64','x86_64':'amd64'}.get(platform.machine()):
            raise RuntimeError('Select an approved image for this host')
        return image

    def docker(self,*arguments,stdin=None,timeout=15):
        result=subprocess.run([*DOCKER,*arguments],env=ENV,stdin=stdin,stdout=subprocess.PIPE,stderr=subprocess.PIPE,timeout=timeout)
        if result.returncode:raise RuntimeError('Docker command outcome requires review')
        if len(result.stdout)>4*1024*1024:raise RuntimeError('Unexpected Docker output')
        return result.stdout.decode()

    def inspect(self,kind,key):return json.loads(self.docker(kind,'inspect',key))[0]

    def engine(self):
        try:
            result=json.loads(self.docker('info','--format','{{json .}}'))
            return {'available':True,'version':str(result['ServerVersion'])[:64],'architecture':str(result['Architecture'])[:32]}
        except (OSError,ValueError,RuntimeError,subprocess.TimeoutExpired):return {'available':False}

    def resolved_image(self,image):
        # Classic stores address the configuration digest; containerd-backed
        # stores address the manifest digest. Both are separately catalog-bound.
        for digest in [image.get('manifestId'),image['imageId']]:
            if not digest:continue
            try:
                value=self.inspect('image',digest)
                if value['Id']==digest and value['Architecture']==image['architecture'] and value['Os']=='linux':return digest
            except (OSError,ValueError,RuntimeError,KeyError,subprocess.TimeoutExpired):pass
        return None

    def installed_image(self,image):return self.resolved_image(image) is not None

    def verify_archive_manifest(self,path,image):
        # No extraction, links, registry access or daemon writes. The archive's
        # own manifest must cryptographically bind the approved configuration.
        with os.fdopen(os.open(path,os.O_RDONLY|os.O_NOFOLLOW),'rb') as stream:
            info=os.fstat(stream.fileno())
            if not stat.S_ISREG(info.st_mode) or info.st_uid!=0 or info.st_mode&0o022 or info.st_size!=image['archiveSize'] \
                    or hashlib.file_digest(stream,'sha256').hexdigest()!=image['archiveSha256']:raise RuntimeError('Original archive differs')
            stream.seek(0)
            with tarfile.open(fileobj=stream,mode='r:gz') as archive:
                members=archive.getmembers()
                if len(members)>4096 or sum(member.size for member in members)>512*1024*1024 \
                        or len({member.name for member in members})!=len(members):raise RuntimeError('Invalid archive bounds')
                def metadata(name):
                    member=archive.getmember(name)
                    if not member.isfile() or member.size>65536:raise RuntimeError('Invalid OCI metadata')
                    return archive.extractfile(member).read(65537)
                index=json.loads(metadata('index.json'))
                if len(index.get('manifests',[]))!=1 or index['manifests'][0]['digest']!=image['manifestId']:raise RuntimeError('Archive manifest differs')
                manifest=metadata('blobs/sha256/'+image['manifestId'].removeprefix('sha256:'))
                if 'sha256:'+hashlib.sha256(manifest).hexdigest()!=image['manifestId'] \
                        or json.loads(manifest)['config']['digest']!=image['imageId']:raise RuntimeError('Manifest configuration differs')
                config=metadata('blobs/sha256/'+image['imageId'].removeprefix('sha256:'))
                if 'sha256:'+hashlib.sha256(config).hexdigest()!=image['imageId']:raise RuntimeError('Configuration digest differs')
                parsed=json.loads(config)
                if parsed.get('architecture')!=image['architecture'] or parsed.get('os')!='linux':raise RuntimeError('Configuration architecture differs')

    def verify_import(self,operation):
        with self.lock():
            record=self.record(operation);self.no_other_job(operation)
            if record['kind']!='IMPORT':raise RuntimeError('Select original import')
            if record['phase']=='APPLIED':return self.status(operation)
            if record['phase']!='UNKNOWN' or record.get('receipts',{}).get('load')!={'commandReturnedSuccess':True}:
                raise RuntimeError('Actual successful load receipt required; never repeat an unknown load')
            image=self.image(record['image']['key'])
            if any(image.get(key)!=value for key,value in record['image'].items()):raise RuntimeError('Original approved catalog binding differs')
            self.verify_archive_manifest(self.directory/'uploads'/(operation+'.accepted.tar.gz'),image)
            resolved=self.resolved_image(image)
            if not resolved:raise RuntimeError('Approved loaded image cannot be verified')
            record['postLoadVerification']={'previousPhase':'UNKNOWN','archiveSha256':image['archiveSha256'],
                'configId':image['imageId'],'manifestId':image['manifestId'],'engineImageId':resolved,
                'successfulOriginalLoadReceiptRequired':True,'dockerMutationsIssued':0}
            record['phase']='APPLIED';self.save(record)
            return self.status(operation)

    def unresolved(self):
        if not self.directory.exists():return []
        return [value for file in sorted((self.directory/'operations').glob('*.json'))
                if (value:=json.loads(protected(file))).get('phase') not in TERMINAL]

    def no_other_job(self,operation):
        if any(value['operationId']!=operation for value in self.unresolved()):
            raise RuntimeError('Observe the original deployment operation before creating another')

    def upload_path(self,operation):return self.directory/'uploads'/(identity(operation)+'.tar.gz')

    def upload_ready(self,record):
        image=record['image'];path=self.upload_path(record['operationId'])
        try:
            with os.fdopen(os.open(path,os.O_RDONLY|os.O_NOFOLLOW|os.O_NONBLOCK),'rb') as stream:
                info=os.fstat(stream.fileno())
                if not stat.S_ISREG(info.st_mode) or info.st_uid!=0 or info.st_mode&0o022 or info.st_size!=image['archiveSize']:return False
                digest=hashlib.file_digest(stream,'sha256').hexdigest()
                return digest==image['archiveSha256']
        except FileNotFoundError:return False

    def prepare_upload(self,operation,key):
        identity(operation);image=self.image(key)
        with self.lock():
            self.no_other_job(operation)
            if self.path(operation).exists():
                old=self.record(operation)
                if old['kind']!='IMPORT' or old['image']!=image:raise RuntimeError('Retain original import parameters')
                return self.status(operation)
            record={'schemaVersion':1,'operationId':operation,'kind':'IMPORT','phase':'UPLOADING','image':image,'intents':{},'receipts':{}}
            self.save(record)
        return self.status(operation)

    def unit(self,operation):return 'yofc-console-docker-job-'+identity(operation)+'.service'

    def live(self,operation):
        try:
            record=self.record(operation);stage=record.get('liveStage')
            if stage is not None and stage not in ['freeze','stop','resume','upgrade','verify-start']:return False
            unit=self.unit(operation) if stage is None else 'yofc-console-docker-maint-'+operation+'-'+stage+'.service'
            result=subprocess.run(['systemctl','show',unit,'-p','ActiveState','-p','MainPID'],env=ENV,capture_output=True,text=True,timeout=3,check=True)
            fields=dict(line.split('=',1) for line in result.stdout.splitlines() if '=' in line)
            pid=fields.get('MainPID','0')
            if fields.get('ActiveState') not in ['activating','active'] or not re.fullmatch(r'[1-9][0-9]*',pid):return False
            arguments=(Path('/proc')/pid/'cmdline').read_bytes().rstrip(b'\0').split(b'\0')
            expected=[b'/usr/bin/python3',PROGRAM.encode(),b'worker',b'--operation',operation.encode()] if stage is None else \
                [b'/usr/bin/python3',b'/usr/lib/kura-management-console/docker_maintenance.py',b'worker',b'--operation',operation.encode(),b'--stage',stage.encode()]
            return arguments==expected
        except (OSError,ValueError,subprocess.SubprocessError):return False

    def status(self,operation):
        identity(operation)
        try:record=self.record(operation)
        except FileNotFoundError:return {'operationId':operation,'phase':'UNOBSERVED','transportNonDeliveryCertified':False}
        value=dict(record)
        if value['phase'] in ['ACCEPTED','RUNNING','REQUESTED']:
            value['phase']='PENDING' if self.live(operation) else 'UNKNOWN'
        if value['kind']=='IMPORT' and value['phase']=='UPLOADING':
            value['uploadReady']=self.upload_ready(record)
            value['uploadPath']=str(self.upload_path(operation))
        return value

    def launch(self,record):
        # Write accepted intent before systemd dispatch. Failure is uncertainty,
        # never permission to repeat systemd-run or the eventual Docker action.
        record['phase']='ACCEPTED';record['launchIntent']=True;self.save(record)
        try:
            result=subprocess.run(['systemd-run','--quiet','--unit',self.unit(record['operationId']),
                '--property=Type=exec','--property=RuntimeMaxSec=300','--property=MemoryMax=384M','--property=CPUQuota=100%',
                '--property=StandardOutput=null','--property=StandardError=null',
                '/usr/bin/python3',PROGRAM,'worker','--operation',record['operationId']],
                env=ENV,capture_output=True,timeout=8)
            # Do not rewrite a worker's newer journal with the launch snapshot.
            if result.returncode:raise RuntimeError('Worker dispatch unverified')
        except (OSError,RuntimeError,subprocess.TimeoutExpired):
            return {'operationId':record['operationId'],'phase':'UNKNOWN','transportNonDeliveryCertified':False}
        return {'operationId':record['operationId'],'phase':'ACCEPTED'}

    def import_image(self,operation):
        with self.lock():
            record=self.record(operation);self.no_other_job(operation)
            if record['kind']!='IMPORT':raise RuntimeError('Wrong operation kind')
            if record['phase']!='UPLOADING':return self.status(operation)
            if not self.upload_ready(record):raise RuntimeError('Upload the exact approved archive first')
            sealed=self.directory/'uploads'/(operation+'.accepted.tar.gz')
            if sealed.exists():raise RuntimeError('Retain the accepted archive')
            record['phase']='SEAL_REQUESTED';self.save(record)
            os.rename(self.upload_path(operation),sealed);sealed.chmod(0o600)
            return self.launch(record)

    def deploy(self,operation,key,name,http_port,grpc_port):
        identity(operation);image=self.image(key)
        if not re.fullmatch(r'[a-z0-9][a-z0-9-]{0,31}',name) or any(type(p) is not int or not 1024<=p<=65535 for p in [http_port,grpc_port]) or http_port==grpc_port:
            raise RuntimeError('Choose a valid instance and two different ports')
        with self.lock():
            self.no_other_job(operation)
            if self.path(operation).exists():
                old=self.record(operation)
                if old['kind']!='DEPLOY' or any(old.get(key)!=value for key,value in
                    [('image',image),('name',name),('httpPort',http_port),('grpcPort',grpc_port)]):raise RuntimeError('Retain original deployment parameters')
                return self.status(operation)
            if not self.installed_image(image):raise RuntimeError('Import the approved image first')
            if (self.directory/'deployments'/(name+'.json')).exists():raise RuntimeError('Retain the original deployment')
            record={'schemaVersion':1,'operationId':operation,'kind':'DEPLOY','phase':'RESERVING','image':image,
                'name':name,'containerName':'yofc-console-'+name,'volume':'yofc-console-data-'+name,
                'httpPort':http_port,'grpcPort':grpc_port,'stopSignal':'SIGINT','intents':{},'receipts':{}}
            record['engineImageId']=self.resolved_image(image)
            if not record['engineImageId']:raise RuntimeError('Approved engine image is unavailable')
            # Reserve before dispatch. Even a lost reply cannot reuse this name.
            self.save(record)
            atomic(self.directory/'deployments'/(name+'.json'),{'operationId':operation,'name':name})
            return self.launch(record)

    def mutate(self,record,step,*arguments,stdin=None):
        if step in record['intents']:raise RuntimeError('Never repeat a Docker mutation')
        record['intents'][step]={'issued':True};self.save(record)
        output=self.docker(*arguments,stdin=stdin,timeout=120)
        record['receipts'][step]={'commandReturnedSuccess':True};self.save(record)
        return output

    def worker(self,operation):
        # The controller may still hold control.lock until systemd acknowledges
        # dispatch. The daemon job claims once, under the same lock.
        with self.lock():
            record=self.record(operation)
            if record['phase']!='ACCEPTED' or record.get('workerClaimed'):raise RuntimeError('Job was already entered')
            record['workerClaimed']=True;record['phase']='RUNNING';self.save(record)
        try:
            if record['kind']=='IMPORT':
                path=self.directory/'uploads'/(operation+'.accepted.tar.gz')
                self.verify_archive_manifest(path,record['image'])
                with os.fdopen(os.open(path,os.O_RDONLY|os.O_NOFOLLOW),'rb') as stream:
                    info=os.fstat(stream.fileno())
                    if not stat.S_ISREG(info.st_mode) or info.st_uid!=0 or info.st_mode&0o077 or info.st_size!=record['image']['archiveSize'] \
                            or hashlib.file_digest(stream,'sha256').hexdigest()!=record['image']['archiveSha256']:
                        raise RuntimeError('Accepted archive changed')
                    stream.seek(0);self.mutate(record,'load','load',stdin=stream)
                if not self.installed_image(record['image']):raise RuntimeError('Imported image differs from approved ID')
            elif record['kind']=='DEPLOY':
                self.worker_deploy(record)
            else:raise RuntimeError('Unsupported job')
            record['phase']='APPLIED';self.save(record)
        except Exception:
            # A successful earlier step does not turn a missing later receipt
            # into a negative delivery certificate. All objects are retained.
            record['phase']='UNKNOWN' if record['intents'] else 'REFUSED'
            record['reason']='REVIEW_ORIGINAL_OPERATION';self.save(record)

    def worker_deploy(self,record):
        image=record['image'];operation=record['operationId']
        if self.resolved_image(image)!=record['engineImageId']:raise RuntimeError('Approved image is unavailable')
        # Neither names nor labels alone authorize takeover of an existing object.
        names=self.docker('container','ls','--all','--format','{{.Names}}').splitlines()
        volumes=self.docker('volume','ls','--format','{{.Name}}').splitlines()
        if record['containerName'] in names or record['volume'] in volumes:raise RuntimeError('Existing object requires review')
        volume=self.mutate(record,'volume','volume','create','--label',OWNER+'='+operation,record['volume']).strip()
        if volume!=record['volume'] or self.inspect('volume',volume).get('Labels',{}).get(OWNER)!=operation:raise RuntimeError('Volume ownership mismatch')
        arguments=['container','create','--name',record['containerName'],'--label',LABEL+'=1','--label',OWNER+'='+operation,
            '--label',ROLE+'=center','--restart','unless-stopped','--stop-signal','SIGINT','--read-only','--security-opt','no-new-privileges',
            '--cap-drop','ALL','--memory','512m','--memory-swap','512m','--cpus','1','--pids-limit','128',
            '--tmpfs','/run:rw,nosuid,nodev,size=16m','--tmpfs','/tmp:rw,nosuid,nodev,noexec,size=16m',
            '--mount','type=volume,src='+volume+',dst=/var/lib/kura-management-console,volume-nocopy',
            '--publish',str(record['httpPort'])+':18443/tcp','--publish',str(record['grpcPort'])+':18444/tcp',
            '--no-healthcheck']
        for capability in ['CHOWN','DAC_OVERRIDE','FOWNER','SETUID','SETGID']:arguments+=['--cap-add',capability]
        output=self.mutate(record,'create',*arguments,record['engineImageId']).strip()
        if not re.fullmatch(r'[a-f0-9]{64}',output):raise RuntimeError('No exact container receipt')
        record['containerId']=output;self.save(record)
        container=self.managed_container(record)
        if container['State'].get('Running'):raise RuntimeError('Unexpected pre-start process')
        self.mutate(record,'start','container','start',output)
        container=self.managed_container(record)
        if not container['State'].get('Running') or container['State'].get('Pid',0)<=0:raise RuntimeError('Start outcome requires review')

    def managed_container(self,record):
        value=self.inspect('container',record['containerId']);labels=value['Config'].get('Labels') or {}
        mounts=value.get('Mounts',[]);host=value['HostConfig'];ports=host.get('PortBindings') or {}
        if value['Id']!=record['containerId'] or value['Image'] not in [record['image']['imageId'],record['image']['manifestId']] \
                or value['Config'].get('Image')!=record['engineImageId'] \
                or value['Config'].get('Cmd') not in ([record['command']] if record.get('command') else [None,[]]) \
                or record.get('stopSignal') and value['Config'].get('StopSignal')!=record['stopSignal'] \
                or labels.get(OWNER)!=record.get('deploymentId',record['operationId']) or labels.get(ROLE)!='center' \
                or len(mounts)!=1 or mounts[0].get('Name')!=record['volume'] or mounts[0].get('Destination')!='/var/lib/kura-management-console' \
                or not host.get('ReadonlyRootfs') or set(ports)!={'18443/tcp','18444/tcp'} \
                or any(ports.get(key)!=[{'HostIp':'','HostPort':str(port)}] for key,port in [('18443/tcp',record['httpPort']),('18444/tcp',record['grpcPort'])]):
            raise RuntimeError('Deployment configuration differs from its root record')
        return value

    def generation(self,deployment):
        record=self.record(identity(deployment))
        if record['kind']!='DEPLOY' or record['phase']!='APPLIED':raise RuntimeError('Select an original completed deployment')
        selected=json.loads(protected(self.directory/'deployments'/(record['name']+'.json')))
        if selected.get('operationId')!=deployment:raise RuntimeError('Original deployment binding differs')
        generation=selected.get('generationId')
        if generation is None:return record
        value=json.loads(protected(self.directory/'generations'/(identity(generation)+'.json')))
        if value.get('deploymentId')!=deployment or value.get('operationId')!=generation:raise RuntimeError('Generation binding differs')
        return value

    def runtime_process(self,generation):
        """Locate exactly one expected server inside this exact local container.

        Older approved entrypoints may keep a waiting shell as PID 1. Verify the
        server's actual executable, argv, UID and start time instead of mistaking
        the wrapper for the application. Kernel paths never come from the user.
        """
        container=self.managed_container(generation)
        if not container['State']['Running']:raise RuntimeError('Original container is not running')
        owner=self.docker('exec','--user','console',generation['containerId'],'id','-u').strip()
        if not re.fullmatch(r'[1-9][0-9]*',owner):raise RuntimeError('Expected an unprivileged console user')
        expected=[b'/usr/local/bin/yofc-management-console',*[value.encode() for value in generation.get('command',['serve'])]]
        candidates=[]
        for line in self.docker('top',generation['containerId'],'-eo','pid,ppid,uid,comm').splitlines()[1:]:
            fields=line.split(None,3)
            if len(fields)!=4 or fields[2]!=owner or not re.fullmatch(r'[1-9][0-9]*',fields[0]):continue
            root=Path('/proc')/fields[0]
            try:
                before=root.joinpath('stat').read_text().rpartition(')')[2].split()[19]
                arguments=root.joinpath('cmdline').read_bytes().rstrip(b'\0').split(b'\0')
                if arguments!=expected:continue
                with root.joinpath('exe').open('rb') as stream:digest=hashlib.file_digest(stream,'sha256').hexdigest()
                after=root.joinpath('stat').read_text().rpartition(')')[2].split()[19]
            except (OSError,ValueError,IndexError):continue
            if before==after and digest==generation['image']['binarySha256']:
                candidates.append({'pid':int(fields[0]),'startTime':before,'uid':int(owner),'exeSha256':digest,
                    'containerInitPid':container['State']['Pid'],'bootId':Path('/proc/sys/kernel/random/boot_id').read_text().strip()})
        if len(candidates)!=1:raise RuntimeError('Exactly one approved server process is required')
        return candidates[0]

    def deployment_status(self,operation,probe):
        record=self.generation(operation)
        value={'operationId':operation,'name':record.get('name'),'phase':self.status(operation)['phase'],
               'containerId':record.get('containerId'),'httpPort':record.get('httpPort'),'grpcPort':record.get('grpcPort'),
               'state':'unknown','ready':False,'readiness':'UNVERIFIED'}
        if 'containerId' not in record:return value
        container=self.managed_container(record);value['state']='active' if container['State']['Running'] else 'inactive'
        if any(row.get('kind')=='MAINTENANCE' and row.get('deploymentId')==operation for row in self.unresolved()):
            value['readiness']='MAINTENANCE';return value
        if not container['State']['Running']:
            value['readiness']='NOT_READY';return value
        # Read capabilities are root provenance-bound before invoking a command.
        metadata=self.docker('exec','--user','console',record['containerId'],'stat','-c','%u %a',
            '/usr/share/kura-management-console/local-command-provenance.json','/usr/local/bin/yofc-management-console')
        owners=[line.split() for line in metadata.splitlines()]
        if len(owners)!=2 or any(owner!='0' or int(mode,8)&0o022 for owner,mode in owners):raise RuntimeError('Unprotected container provenance')
        provenance=json.loads(self.docker('exec','--user','console',record['containerId'],'cat','/usr/share/kura-management-console/local-command-provenance.json'))
        digest=self.docker('exec','--user','console',record['containerId'],'sha256sum','/usr/local/bin/yofc-management-console').split()[0]
        if digest!=record['image']['binarySha256'] or provenance.get('binarySha256')!=digest or provenance.get('schemaVersion')!=1 \
                or 'installation-status' not in provenance.get('localReadCommands',[]):raise RuntimeError('Unsupported container status command')
        description=json.loads(self.docker('exec','--user','console',record['containerId'],'/usr/local/bin/yofc-management-console','installation-status'))
        description['httpAddress']='127.0.0.1:'+str(record['httpPort']);description['httpPort']=record['httpPort']
        value['ready']=probe(description);value['readiness']='READY' if value['ready'] else 'NOT_READY'
        return value

    def overview(self):
        engine=self.engine();images=[]
        for row in self.catalog():images.append({**row,'available':engine['available'] and self.installed_image(row)})
        deployments=[]
        if self.directory.exists():
            for path in sorted((self.directory/'deployments').glob('*.json')):
                selected=json.loads(protected(path));record=self.generation(selected['operationId'])
                deployments.append({**{key:record.get(key) for key in ['name','containerId','httpPort','grpcPort','phase']},
                    'operationId':selected['operationId'],'imageKey':record['image']['key'],'fencedStartupSupported':record['image']['maintenanceProtocolVersion']==2})
        return {'engine':engine,'images':images,'deployments':deployments,
                'operations':[self.status(value['operationId']) for value in self.unresolved()]}


if __name__=='__main__':
    # Only systemd's fixed job command uses this entrypoint. Public root actions
    # go through installer.py and verify all independently packaged tool files.
    try:
        if os.geteuid()!=0 or len(sys.argv)!=4 or sys.argv[1:3]!=['worker','--operation']:raise RuntimeError('Invalid worker invocation')
        from installer import verify_installer_payload
        verify_installer_payload();DockerControl().worker(identity(sys.argv[3]))
    except Exception:sys.exit(1)
