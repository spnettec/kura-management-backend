"""Root-controlled, crash-persistent maintenance for installed console instances.

Host actions serialize separately from application writers. The writer lease is
exclusive while checking settlement and publishing a fence. Stop intents and
receipts belong to the original operation; an uncertain stop is never retried.
"""
from contextlib import contextmanager
import fcntl
import json
import os
import re
from pathlib import Path
import stat
import uuid

CONTROL = '/var/lib/yofc-console-maintenance'
PROVENANCE = '/usr/share/kura-management-console/local-command-provenance.json'


class HostControl:
    def __init__(self, setup):
        self.setup = setup
        self.directory = setup.path(CONTROL)

    def provision(self):
        for path in [self.directory, self.directory/'operations']:
            try:
                path.mkdir(mode=0o755)
                path.chmod(0o755)
            except FileExistsError:
                pass
            info=path.lstat()
            if not stat.S_ISDIR(info.st_mode) or info.st_uid!=0 or info.st_mode & 0o022:
                raise RuntimeError('Maintenance controls must be protected by root')
        for name in ['host.lock','writers.lock']:
            path=self.directory/name
            try:
                descriptor=os.open(path,os.O_WRONLY|os.O_CREAT|os.O_EXCL|os.O_NOFOLLOW,0o644)
            except FileExistsError:
                self.read(name,raw=True)
            else:
                with os.fdopen(descriptor,'wb') as stream:
                    os.fchmod(stream.fileno(),0o644);stream.flush();os.fsync(stream.fileno())
        self.sync()

    def sync(self):
        descriptor=os.open(self.directory,os.O_RDONLY|os.O_DIRECTORY|os.O_NOFOLLOW)
        try: os.fsync(descriptor)
        finally: os.close(descriptor)

    def read(self,name,raw=False,optional=False):
        path=self.directory/name
        try: descriptor=os.open(path,os.O_RDONLY|os.O_NOFOLLOW|os.O_NONBLOCK)
        except FileNotFoundError:
            # A dangling symlink is retained damage, not an absent fence.
            if optional and not path.is_symlink(): return None
            raise
        with os.fdopen(descriptor,'rb') as stream:
            info=os.fstat(stream.fileno())
            if not stat.S_ISREG(info.st_mode) or info.st_uid!=0 or info.st_mode & 0o022 or info.st_size>65536:
                raise RuntimeError('Invalid maintenance control record')
            value=stream.read(65537)
            if len(value)>65536: raise RuntimeError('Maintenance control record too large')
        return value if raw else json.loads(value)

    def write(self,name,value,new=False):
        destination=self.directory/name
        temporary=destination.with_name('.'+destination.name+'.'+uuid.uuid4().hex)
        descriptor=os.open(temporary,os.O_WRONLY|os.O_CREAT|os.O_EXCL|os.O_NOFOLLOW,0o644)
        try:
            with os.fdopen(descriptor,'wb') as stream:
                os.fchmod(stream.fileno(),0o644)
                stream.write((json.dumps(value,sort_keys=True)+'\n').encode());stream.flush();os.fsync(stream.fileno())
            if new: os.link(temporary,destination,follow_symlinks=False)
            else: os.replace(temporary,destination)
            parent=os.open(destination.parent,os.O_RDONLY|os.O_DIRECTORY|os.O_NOFOLLOW)
            try: os.fsync(parent)
            finally: os.close(parent)
        finally:
            temporary.unlink(missing_ok=True)

    @contextmanager
    def locked(self,writers=False,shared=False):
        self.provision()
        name='writers.lock' if writers else 'host.lock'
        self.read(name,raw=True)
        descriptor=os.open(self.directory/name,os.O_RDONLY|os.O_NOFOLLOW)
        try:
            try: fcntl.flock(descriptor,(fcntl.LOCK_SH if shared else fcntl.LOCK_EX)|fcntl.LOCK_NB)
            except BlockingIOError: raise RuntimeError('Maintenance is busy; no stop was accepted') from None
            yield
        finally: os.close(descriptor)

    @staticmethod
    def identity(value):
        parsed=uuid.UUID(value)
        if not parsed.int or str(parsed)!=value: raise RuntimeError('Use the original maintenance UUID')
        return value

    def active(self):
        value=self.read('active.json',optional=True)
        if value is not None:
            self.identity(value['operationId'])
            if value.get('schemaVersion')!=1 or value.get('phase')!='FROZEN':
                raise RuntimeError('Invalid maintenance fence; preserve it')
        return value

    def journal(self,identity):
        identity=self.identity(identity)
        value=self.read('operations/'+identity+'.json')
        if value.get('operationId')!=identity or value.get('schemaVersion')!=1:
            raise RuntimeError('Maintenance journal mismatch')
        return value

    def save(self,value,new=False):
        self.write('operations/'+self.identity(value['operationId'])+'.json',value,new)

    def status(self):
        if not self.directory.exists(): return {'active':None,'activeFencePresent':False}
        active=self.active()
        if active is not None:return {'active':self.journal(active['operationId']),'activeFencePresent':True}
        hold=self.package_hold()
        if hold is not None:
            return {'active':{**self.journal(hold['operationId']),'phase':'PACKAGE_FINALIZATION'},
                    'activeFencePresent':False,'packageOperationId':hold['packageOperationId']}
        unresolved=self.unresolved()
        if unresolved:
            return {'active':{**unresolved[0],'phase':'UNKNOWN'},'activeFencePresent':False,'unresolvedIds':[value['operationId'] for value in unresolved]}
        return {'active':None,'activeFencePresent':False}

    def unresolved(self):
        result=[]
        for path in sorted((self.directory/'operations').glob('*.json')):
            identity=self.identity(path.stem);value=self.journal(identity)
            if value['phase']!='COMPLETE':result.append(value)
        return result

    def participants(self):
        names=[None]+[name.removeprefix('kura-management-console@').removesuffix('.service') for name in self.setup.template_units()]
        return [type(self.setup)(self.setup.root,self.setup.run,name) for name in names]

    def state(self,setup):
        result=setup.systemctl('show',setup.unit,'-p','ActiveState','-p','MainPID','-p','Job')
        if result is None: raise RuntimeError('A running systemd host is required')
        fields=dict(line.split('=',1) for line in result.stdout.splitlines() if '=' in line)
        if not {'ActiveState','MainPID','Job'}<=fields.keys(): raise RuntimeError('Incomplete service observation')
        return fields

    @staticmethod
    def stopped(fields):
        return fields['ActiveState'] in ['inactive','failed'] and fields['MainPID']=='0' and not fields['Job']

    def supported(self):
        path=self.setup.path(PROVENANCE)
        if not path.exists(): return False
        record=self.setup.verify_local_command('installation-maintenance-check')
        return record.get('maintenanceProtocolVersion')==1

    def verify_active_participants(self,participants):
        record=self.setup.verify_local_command('installation-maintenance-check')
        for item in participants:
            fields=self.state(item)
            if self.stopped(fields):continue
            pid=fields['MainPID']
            if fields['ActiveState']!='active' or fields['Job'] or not re.fullmatch(r'[1-9][0-9]*',pid):
                raise RuntimeError('Participant startup or shutdown is unsettled')
            digest=item.active_process_digest(pid)
            if digest!=record['binarySha256']:
                raise RuntimeError('A running participant does not implement the installed maintenance protocol')

    @contextmanager
    def starting(self):
        with self.locked(),self.locked(writers=True,shared=True):
            if self.status()['active'] is not None: raise RuntimeError('Maintenance is frozen; observe the original operation')
            yield

    def begin(self,identity,offline_legacy=False,require_stopped=False):
        identity=self.identity(identity)
        with self.locked(),self.locked(writers=True):
            active=self.active()
            if active is not None:
                if active['operationId']!=identity: raise RuntimeError('Another original maintenance operation is active')
                return self.journal(identity)  # Observation only.
            if self.package_hold() is not None:
                raise RuntimeError('Finish the original verified package release first')
            if self.read('operations/'+identity+'.json',optional=True) is not None:
                raise RuntimeError('A maintenance operation ID cannot be reused')
            if self.unresolved():raise RuntimeError('An original maintenance journal has an uncertain outcome')
            participants=self.participants()
            if require_stopped and not all(self.stopped(self.state(item)) for item in participants):
                raise RuntimeError('Native package installation never implicitly stops a center')
            supported=self.supported()
            if supported:self.verify_active_participants(participants)
            fresh=not self.setup.path('/usr/bin/yofc-management-console').exists() and not any(
                item.path(item.state).exists() for item in participants)
            if not supported and not fresh:
                if not offline_legacy or not all(self.stopped(self.state(item)) for item in participants):
                    raise RuntimeError('Old packages require explicit offline adoption; no stop was accepted')
            # The writer lease is exclusive during all authoritative DB/journal checks.
            self.setup.guard_all_instances()
            value={'schemaVersion':1,'operationId':identity,'phase':'FROZEN',
                   'units':[item.unit for item in participants],'stopIntents':{},'stopReceipts':{},
                   'offlineLegacy':not supported and not fresh,'ownedMasks':[]}
            self.save(value,new=True)
            self.write('active.json',{'schemaVersion':1,'operationId':identity,'phase':'FROZEN','scope':'native-host'},new=True)
            if value['offlineLegacy']:
                # Old binaries do not honor writer leases. Do not stop them. Mask
                # the default and template, then verify no prior job/process survived.
                # An uncertain mask/adoption is retained for explicit review.
                value['phase']='ADOPTION_INTENT';self.save(value)
                for unit in ['kura-management-console.service','kura-management-console@.service']:
                    mask=self.setup.path('/run/systemd/system')/unit
                    if mask.is_symlink() and os.readlink(mask)=='/dev/null': continue
                    if mask.exists() or mask.is_symlink(): raise RuntimeError('Existing runtime unit override requires review')
                    value['ownedMasks'].append(unit);self.save(value)
                    self.setup.systemctl('mask','--runtime',unit)
                if [item.unit for item in self.participants()]!=value['units'] or not all(
                        self.stopped(self.state(item)) for item in self.participants()):
                    raise RuntimeError('Offline adoption cannot be verified; no stop will be issued')
                self.setup.guard_all_instances()
                value['phase']='FROZEN';self.save(value)
            return value

    def stop(self,identity,require_stopped=False):
        with self.locked(),self.locked(writers=True):
            active=self.active()
            if active is None or active['operationId']!=identity: raise RuntimeError('Original maintenance fence required')
            value=self.journal(identity)
            if value['phase']=='STOPPED':
                if require_stopped and (self.participants_units()!=value['units'] or not all(
                        self.stopped(self.state(item)) for item in self.participants())):
                    raise RuntimeError('Original stopped participants changed')
                return value
            if value['phase']!='FROZEN' or value['stopIntents']:
                raise RuntimeError('Original stop outcome is UNKNOWN; do not repeat it')
            if [item.unit for item in self.participants()]!=value['units']: raise RuntimeError('Maintenance participant inventory changed')
            self.setup.guard_all_instances()
            if require_stopped and not all(self.stopped(self.state(item)) for item in self.participants()):
                raise RuntimeError('Native package installation never implicitly stops a center')
            for item in self.participants():
                fields=self.state(item)
                issued=not self.stopped(fields)
                if issued and require_stopped:
                    raise RuntimeError('Native package installation never implicitly stops a center')
                value['stopIntents'][item.unit]={'before':fields,'commandWillBeIssued':issued};value['phase']='STOPPING';self.save(value)
                returned=None
                if issued:
                    returned=item.systemctl('stop',item.unit)
                    if returned is None or returned.returncode!=0:raise RuntimeError('Stop outcome is UNKNOWN; preserve the original intent')
                after=self.state(item)
                if not self.stopped(after): raise RuntimeError('Stop outcome is UNKNOWN; preserve the original intent')
                value['stopReceipts'][item.unit]={'state':'APPLIED','commandIssued':issued,
                    'returncode':returned.returncode if issued else None,'after':after};self.save(value)
            self.setup.guard_all_instances()
            value['phase']='STOPPED';self.save(value)
            return value

    def cancel(self,identity,finalize_only=False):
        """Retire the original UUID, fencing even a delayed GUI request.

        This never retries a stop and never asserts transport non-delivery. A
        native stop intent, offline mask/adoption or uncertain native action
        forbids cancellation. An absent request gets a durable tombstone so a
        delayed begin cannot resurrect it. COMPLETE keeps protocol-1 readers
        compatible; outcome distinguishes cancellation from successful upkeep.
        """
        identity=self.identity(identity)
        with self.locked(),self.locked(writers=True):
            active=self.active()
            value=self.read('operations/'+identity+'.json',optional=True)
            if value is None:
                if finalize_only:raise RuntimeError('A verified cancellation receipt is required')
                if active is not None and active['operationId']==identity:
                    raise RuntimeError('Original journal missing; retain its fence')
                value={'schemaVersion':1,'operationId':identity,'phase':'COMPLETE',
                       'outcome':'CANCELLED_BEFORE_ACCEPTANCE','lateActionsFenced':True,
                       'transportNonDeliveryCertified':False,'units':[],
                       'stopIntents':{},'stopReceipts':{},'ownedMasks':[],'offlineLegacy':False}
                self.save(value,new=True)
                return value  # Another operation's fence is never removed.
            value=self.journal(identity)
            cancelled=value['phase']=='COMPLETE' and value.get('outcome') in [
                'CANCELLED_BEFORE_ACCEPTANCE','CANCELLED_BEFORE_STOP_INTENT']
            if finalize_only and not cancelled:raise RuntimeError('A verified cancellation receipt is required')
            if cancelled and (active is None or active['operationId']!=identity):return value
            if active is not None and active['operationId']!=identity:
                raise RuntimeError('Another original maintenance operation is active')
            if not cancelled and (value['phase']!='FROZEN' or value['stopIntents'] or
                    value['stopReceipts'] or value['offlineLegacy'] or value['ownedMasks']):
                raise RuntimeError('A native action may have entered; retain the original operation')
            participants=self.participants()
            if [item.unit for item in participants]!=value['units']:
                raise RuntimeError('Maintenance participant inventory changed')
            if not self.supported():raise RuntimeError('Verified maintenance protocol is required')
            self.verify_active_participants(participants)
            fields={item.unit:self.state(item) for item in participants}
            if not all(row['ActiveState']=='active' and row['MainPID']!='0' and not row['Job'] for row in fields.values()):
                raise RuntimeError('Cancellation requires all original services to remain active')
            self.setup.guard_all_instances()
            if not cancelled:
                value.update(phase='COMPLETE',outcome='CANCELLED_BEFORE_STOP_INTENT',
                             lateActionsFenced=True,transportNonDeliveryCertified=False,
                             cancellationServiceObservations=fields)
                self.save(value)  # Retire first; a later stop rejects this phase.
            if active is not None:
                (self.directory/'active.json').unlink();self.sync()
            return value  # No stop/start/restart/enable is issued.

    def complete(self,identity,expected_binary=None,expected_source=None):
        with self.locked(),self.locked(writers=True):
            active=self.active();value=self.journal(identity)
            if active is None:
                if value['phase']!='COMPLETE': raise RuntimeError('Original completion receipt missing')
                return value
            if active['operationId']!=identity or value['phase'] not in ['FROZEN','STOPPED','COMPLETE']:
                raise RuntimeError('Maintenance has an uncertain outcome; retain the fence')
            participants=self.participants()
            if [item.unit for item in participants]!=value['units'] or not all(self.stopped(self.state(item)) for item in participants):
                raise RuntimeError('All original participants must be verifiably stopped')
            if value['stopIntents'].keys()!=value['stopReceipts'].keys(): raise RuntimeError('A stop receipt is missing')
            record=self.setup.verify_local_command('installation-maintenance-check')
            hold=self.package_hold()
            if hold is not None and (hold['operationId']!=identity or record.get('hostPackageFenceProtocolVersion')!=1
                    or record['binarySha256']!=hold['binarySha256']
                    or record['sourceManifestSha256']!=hold['sourceManifestSha256']):
                raise RuntimeError('Original approved package completion differs')
            if record.get('maintenanceProtocolVersion')!=1 or (expected_binary and record['binarySha256']!=expected_binary) \
                    or (expected_source and record['sourceManifestSha256']!=expected_source):
                raise RuntimeError('Installed maintenance protocol or target payload cannot be verified')
            self.setup.guard_all_instances()
            if value['ownedMasks'] and value['phase']!='COMPLETE':
                value['phase']='UNMASKING';self.save(value)
            for unit in value['ownedMasks'] if value['phase']!='COMPLETE' else []:
                mask=self.setup.path('/run/systemd/system')/unit
                # Fixed root-owned runtime mask; Setup.path deliberately rejects
                # symlinks, so inspect it through the trusted parent instead.
                if mask.is_symlink() and os.readlink(mask)=='/dev/null': self.setup.systemctl('unmask','--runtime',unit)
                elif mask.exists() or mask.is_symlink(): raise RuntimeError('Maintenance runtime mask changed')
            value['verifiedPayload']={'maintenanceProtocolVersion':1,'binarySha256':record['binarySha256'],
                'sourceManifestSha256':record['sourceManifestSha256']}
            value['phase']='COMPLETE';self.save(value)
            if hold is not None:
                value['packageInstallOperationId']=hold['packageOperationId'];self.save(value)
                return value  # dpkg's own receipt is still required before release.
            (self.directory/'active.json').unlink();self.sync()
            return value  # No enable/start/restart is performed.

    def participants_units(self):
        return [item.unit for item in self.participants()]

    def package_hold(self):
        value=self.read('package-install.json',optional=True)
        if value is None:return None
        if value.get('schemaVersion')!=1 or any(not re.fullmatch(r'[a-f0-9]{64}',value.get(key,''))
                for key in ['packageSha256','binarySha256','sourceManifestSha256']):
            raise RuntimeError('Invalid retained package hold')
        self.identity(value['operationId']);self.identity(value['packageOperationId'])
        return value

    def hold_package(self,identity,package_operation,package):
        """Hold the existing writer fence across dpkg and its final receipt."""
        self.identity(identity);self.identity(package_operation)
        with self.locked(),self.locked(writers=True):
            active=self.active();value=self.journal(identity)
            if active is None or active['operationId']!=identity or value['phase']!='STOPPED' \
                    or self.participants_units()!=value['units'] \
                    or not all(self.stopped(self.state(item)) for item in self.participants()) \
                    or value['stopIntents'].keys()!=value['stopReceipts'].keys():
                raise RuntimeError('All original stopped participants and receipts required')
            self.setup.guard_all_instances()
            hold={'schemaVersion':1,'operationId':identity,'packageOperationId':package_operation,
                  'packageSha256':package['archiveSha256'],'binarySha256':package['binarySha256'],
                  'sourceManifestSha256':package['sourceManifestSha256']}
            old=self.package_hold()
            if old is not None:
                if old!=hold:raise RuntimeError('Another original package hold exists')
                return old
            self.write('package-install.json',hold,new=True)
            return hold

    def release_package(self,identity,package_operation):
        """Release only a verified, successful original native package job.

        No command is inferred from installed files. NativeAPPLIED, the actual
        dpkg return receipt and the matching postinst receipt must all exist.
        This action never installs, stops, starts or repeats an unknown command.
        """
        self.identity(identity);self.identity(package_operation)
        with self.locked(),self.locked(writers=True):
            path=self.setup.path('/var/lib/yofc-console-native-packages/operations/'+package_operation+'.json')
            descriptor=os.open(path,os.O_RDONLY|os.O_NOFOLLOW|os.O_NONBLOCK)
            with os.fdopen(descriptor,'rb') as stream:
                info=os.fstat(stream.fileno())
                if not stat.S_ISREG(info.st_mode) or info.st_uid!=0 or info.st_mode&0o077 or info.st_size>4*1024*1024:
                    raise RuntimeError('Root-private original package receipt required')
                package=json.loads(stream.read(4*1024*1024+1))
            value=self.journal(identity)
            if package.get('schemaVersion')!=1 or package.get('operationId')!=package_operation \
                    or package.get('maintenanceOperationId')!=identity or package.get('phase')!='APPLIED' \
                    or package.get('installReceipt')!={'commandReturnedSuccess':True} \
                    or value['phase']!='COMPLETE' or value.get('packageInstallOperationId')!=package_operation \
                    or value.get('verifiedPayload')!=package.get('verifiedPayload'):
                raise RuntimeError('Exact original package and post-install receipts required')
            expected={'maintenanceProtocolVersion':1,'binarySha256':package['package']['binarySha256'],
                      'sourceManifestSha256':package['package']['sourceManifestSha256']}
            if value.get('verifiedPayload')!=expected:raise RuntimeError('Approved native payload differs')
            active=self.active();hold=self.package_hold()
            if active is None and hold is None:return value
            if hold is None or hold['operationId']!=identity or hold['packageOperationId']!=package_operation \
                    or hold['packageSha256']!=package['package']['archiveSha256'] \
                    or hold['binarySha256']!=expected['binarySha256'] \
                    or hold['sourceManifestSha256']!=expected['sourceManifestSha256']:
                raise RuntimeError('Original package hold differs')
            if active is not None:
                if active['operationId']!=identity or self.participants_units()!=value['units'] \
                        or not all(self.stopped(self.state(item)) for item in self.participants()):
                    raise RuntimeError('Original stopped participants required for package release')
                self.setup.guard_all_instances()
                (self.directory/'active.json').unlink();self.sync()
            (self.directory/'package-install.json').unlink();self.sync()
            return value
