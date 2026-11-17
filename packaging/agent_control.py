#!/usr/bin/python3
"""Fixed host-admin delegation to the separately paired native agent controller.

No service mutation, arbitrary path, environment editing, key access or channel
activation is exposed. The agent owns the durable original grant and recovery.
"""
import hashlib
import json
import os
from pathlib import Path
import re
import stat
import subprocess
import uuid
from setup import Setup

PROGRAM='/usr/lib/kura-management-agent/runtime_setup.py'
SHARE='/usr/share/kura-management-agent/'
FILES={PROGRAM:'runtime_setup.py','/usr/lib/kura-management-agent/setup.py':'setup.py',
       '/usr/lib/kura-management-agent/enrollment_relay.py':'enrollment_relay.py',
       '/usr/lib/kura-management-agent/enrollment_reader.py':'enrollment_reader.py',
       **{'/usr/lib/systemd/system/'+name:name for name in [
           'kura-management-agent-v2.service','kura-management-agent-enrollment.socket',
           'kura-management-agent-enrollment@.service','kura-management-enrollment-reader.socket',
           'kura-management-enrollment-reader@.service']}}
UNITS=('kura.service','kura-management-agent-v2.service','kura-management-agent-enrollment.socket',
       'kura-management-enrollment-reader.socket','kura-management-agent.service')
ENV={'PATH':'/usr/sbin:/usr/bin:/sbin:/bin','LC_ALL':'C'}


class AgentControl:
    def __init__(self,setup=None):self.setup=setup or Setup()

    @staticmethod
    def identity(value):
        if not isinstance(value,str) or str(uuid.UUID(value))!=value or not uuid.UUID(value).int:
            raise RuntimeError('Canonical original native identity required')
        return value

    @staticmethod
    def decode(raw):
        def unique(pairs):
            value={}
            for key,item in pairs:
                if key in value:raise RuntimeError('Duplicate native control property')
                value[key]=item
            return value
        return json.loads(raw,object_pairs_hook=unique)

    def protected(self,name):
        path=self.setup.path(name)
        for parent in path.parents:
            info=parent.lstat()
            if not stat.S_ISDIR(info.st_mode) or info.st_uid!=0 or info.st_mode&0o022:
                raise RuntimeError('Native control requires Root-protected parents')
            if parent==self.setup.root:break
        fd=os.open(path,os.O_RDONLY|os.O_NOFOLLOW|os.O_CLOEXEC|os.O_NONBLOCK|os.O_NOATIME)
        with os.fdopen(fd,'rb') as stream:
            info=os.fstat(stream.fileno())
            if (not stat.S_ISREG(info.st_mode) or info.st_uid!=0 or info.st_gid!=0
                    or stat.S_IMODE(info.st_mode)!=0o644 or info.st_nlink!=1 or info.st_size>2*1024*1024):
                raise RuntimeError('Native control requires Root-owned packaged files')
            raw=stream.read(2*1024*1024+1);after=os.fstat(stream.fileno())
            stable=lambda x:(x.st_dev,x.st_ino,x.st_mode,x.st_uid,x.st_gid,x.st_nlink,x.st_size,x.st_mtime_ns,x.st_ctime_ns)
            if len(raw)>2*1024*1024 or stable(info)!=stable(after) or stable(info)!=stable(path.lstat()):
                raise RuntimeError('Native control payload changed during inspection')
            return raw

    def verify(self):
        if self.protected(SHARE+'package-v2')!=b'kura-management-agent-v2\n':
            raise RuntimeError('A separately installed native agent is required')
        meta=self.decode(self.protected(SHARE+'native-profile.json'))
        source=self.decode(self.protected(SHARE+'source-manifest.json'))
        if (not isinstance(meta,dict) or set(meta)!={'schemaVersion','profile','files'}
                or type(meta['schemaVersion']) is not int or meta['schemaVersion']!=1
                or meta['profile']!='native-nonroot-v1' or not isinstance(meta['files'],dict)
                or set(meta['files'])!=set(FILES) or not isinstance(source,dict)
                or type(source.get('schemaVersion')) is not int or source['schemaVersion']!=2):
            raise RuntimeError('Exact installed native source pair required')
        rows=source.get('sourceFiles')
        if not isinstance(rows,list) or not 1<=len(rows)<=4096:raise RuntimeError('Native source pins required')
        pins={row['path']:row['sha256'] for row in rows}
        if len(pins)!=len(rows):raise RuntimeError('Duplicate native source pin')
        for name,relative in FILES.items():
            expected=meta['files'][name]
            if (not isinstance(expected,str) or not re.fullmatch('[a-f0-9]{64}',expected)
                    or hashlib.sha256(self.protected(name)).hexdigest()!=expected
                    or expected!=pins.get('packaging/'+relative)):
                raise RuntimeError('Native code differs from its Root source pair')

    def invoke(self,action,*args):
        self.verify()
        if self.setup.root!=Path('/'):
            raise RuntimeError('Native actions require the installed host controller')
        result=subprocess.run(['/usr/bin/python3','-I',PROGRAM,action,*args],env=ENV,
                              capture_output=True,text=True,timeout=45)
        if result.returncode or len(result.stdout)>65536:
            raise RuntimeError('Native operation is unverified; retain its original ID and observe')
        return self.decode(result.stdout)

    def inspect(self):
        value=self.invoke('inspect')
        fields={'schemaVersion','profile','initializationId','sourceManifestSha256','nativeUid','controlGid',
                'permission','requestId','readiness'}
        if (not isinstance(value,dict) or set(value)!=fields or type(value['schemaVersion']) is not int
                or value['schemaVersion']!=1 or value['profile']!='native-nonroot-v1'
                or not isinstance(value['sourceManifestSha256'],str)
                or not re.fullmatch('[a-f0-9]{64}',value['sourceManifestSha256'])
                or any(type(value[key]) is not int or not 0<value[key]<4294967295 for key in ['nativeUid','controlGid'])
                or value['permission'] not in ['NONE','PENDING','GRANTED'] or value['readiness'] not in ['READY','REFUSED']):
            raise RuntimeError('Unsupported native host-control inspection')
        self.identity(value['initializationId'])
        if value['permission']=='NONE':
            if value['requestId'] is not None:raise RuntimeError('Unexpected native grant identity')
        else:self.identity(value['requestId'])
        return value

    def services(self):
        result=subprocess.run(['/usr/bin/systemctl','show',*UNITS,'--property=Id,MainPID,ActiveState,Job,NConnections'],
                              env=ENV,capture_output=True,text=True,timeout=15)
        if result.returncode:raise RuntimeError('Installed systemd service observation required')
        rows=[]
        for block in result.stdout.strip().split('\n\n'):
            fields=dict(line.split('=',1) for line in block.splitlines() if '=' in line)
            counter='NConnections' if fields.get('Id','').endswith('.socket') else 'MainPID'
            if (set(fields)!={'Id',counter,'ActiveState','Job'} or fields['Id'] not in UNITS
                    or not re.fullmatch('[0-9]{1,10}',fields[counter])
                    or fields['ActiveState'] not in ['active','reloading','inactive','failed','activating','deactivating','maintenance','refreshing']):
                raise RuntimeError('Unverified native service observation')
            rows.append({'unit':fields['Id'],'state':fields['ActiveState'],
                         'pid':int(fields['MainPID']) if counter=='MainPID' else None,
                         'connections':int(fields['NConnections']) if counter=='NConnections' else None,
                         'jobPending':bool(fields['Job'])})
        if len(rows)!=len(UNITS) or {row['unit'] for row in rows}!=set(UNITS):
            raise RuntimeError('Complete native service observation required')
        return rows

    def overview(self):
        value=self.inspect();rows=self.services()
        return {**value,'services':rows,'allStopped':self.stopped(rows)}

    @staticmethod
    def stopped(rows):
        return all(row['pid'] in [None,0] and row['connections'] in [None,0] and not row['jobPending']
                   and row['state'] in ['inactive','failed'] for row in rows)

    def request(self,action,operation,initialization):
        self.identity(initialization);self.identity(operation)
        current=self.inspect()
        if current['initializationId']!=initialization:
            raise RuntimeError('Different native installation; retain the saved request without querying it here')
        commands={'authorize':'authorize-kura-remote-switch','complete':'complete-kura-remote-switch',
                  'status':'kura-remote-switch-status'}
        if action not in commands:raise RuntimeError('Fixed native agent action required')
        if action=='authorize':
            if current['permission']!='NONE' or current['readiness']!='READY' or not self.stopped(self.services()):
                raise RuntimeError('Explicit stopped original services and an ungranted native profile required')
        elif action=='complete' and (current['permission']!='PENDING' or current['requestId']!=operation):
            raise RuntimeError('Only the original pending native grant may be completed')
        value=self.invoke(commands[action],'--request-id',operation,'--initialization-id',initialization)
        common={'schemaVersion','requestId','state','unitStarted','remoteEnabled','replayed'}
        fields=common|{'currentReadiness','canControl'} if isinstance(value,dict) and value.get('state')=='APPLIED' else common
        if (not isinstance(value,dict) or set(value)!=fields or type(value.get('schemaVersion')) is not int
                or value['schemaVersion']!=1 or value.get('requestId')!=operation
                or value.get('state') not in ['UNKNOWN','APPLIED'] or value.get('replayed') is not False
                or value.get('unitStarted') is not False or value.get('remoteEnabled') is not False
                or value['state']=='APPLIED' and (value['currentReadiness'] not in ['READY','REFUSED']
                    or type(value['canControl']) is not bool)):
            raise RuntimeError('Original native grant outcome is unverified')
        return value
