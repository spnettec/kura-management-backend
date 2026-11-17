#!/usr/bin/python3
"""Fixed host-admin actions for the authenticated Cockpit installer page.

No arbitrary paths, shell commands, restart/reset or private-key download API.
Credential stdout belongs exclusively to the authorized private host channel.
"""
import argparse
import json
import hashlib
import http.client
import ipaddress
import os
from pathlib import Path
import re
import socket
import ssl
import stat
import subprocess
import sys
from setup import Setup, valid_instance
from host_control import HostControl
from docker_control import DockerControl
from docker_maintenance import MaintenanceControl
from native_packages import NativePackages
from agent_control import AgentControl

DOCKER = '/usr/bin/docker'
LABEL = 'com.yofc.console.installation'
INSTALLER_FILES={
    '/usr/lib/kura-management-console/setup.py','/usr/lib/kura-management-console/installer.py',
    '/usr/lib/kura-management-console/host_control.py','/usr/sbin/kura-console-setup',
    '/usr/lib/kura-management-console/docker_control.py','/usr/lib/kura-management-console/docker_maintenance.py','/usr/share/kura-management-console-installer/images.json',
    '/usr/lib/kura-management-console/native_packages.py','/usr/share/kura-management-console-installer/packages.json',
    '/usr/lib/kura-management-console/agent_control.py',
    *('/usr/share/cockpit/yofc-console-setup/'+name for name in ['index.html','installer.js','native-packages.js','agent-control.js','installer.css','manifest.json'])}
DESKTOP_FILES={'/usr/share/applications/com.yofc.KuraConsoleSetup.desktop'}
HOST_UI_FILES={'/usr/lib/kura-management-console/host_ui.py','/usr/sbin/kura-console-host-ui',
               '/usr/share/polkit-1/actions/com.yofc.console.host-ui.policy'}


def verify_installer_payload():
    """Independently bind the root host tools; the server has its own provenance."""
    setup=Setup()
    def protected(name):
        path=setup.path(name)
        descriptor=os.open(path,os.O_RDONLY|os.O_NOFOLLOW|os.O_NONBLOCK)
        with os.fdopen(descriptor,'rb') as stream:
            info=os.fstat(stream.fileno())
            if not stat.S_ISREG(info.st_mode) or info.st_uid!=0 or info.st_mode&0o022 or info.st_size>4194304:
                raise RuntimeError('Host installer provenance and payload must be protected by root')
            value=stream.read(4194305)
            if len(value)>4194304:raise RuntimeError('Host installer payload is too large')
            return value
    root='/usr/share/kura-management-console-installer/'
    record=json.loads(protected(root+'provenance-v1.json'))
    desktop=record.get('desktopIntegration',False)
    if type(desktop) is not bool:
        raise RuntimeError('Invalid host desktop integration provenance')
    expected=INSTALLER_FILES | DESKTOP_FILES if desktop else INSTALLER_FILES
    bootstrap=record.get('hostUiBootstrapProtocolVersion',0)
    if type(bootstrap) is not int or bootstrap not in [0,1] or bootstrap and not desktop:
        raise RuntimeError('Invalid host desktop bootstrap provenance')
    if bootstrap:expected=expected | HOST_UI_FILES
    rows=record.get('files',[])
    if record.get('schemaVersion')!=1 or record.get('maintenanceProtocolVersion')!=1 \
            or len(rows)!=len(expected) or {row.get('path') for row in rows}!=expected:
        raise RuntimeError('Verified host installer provenance is required')
    if hashlib.sha256(protected(root+'source-manifest.json')).hexdigest()!=record.get('sourceManifestSha256'):
        raise RuntimeError('Host installer source provenance mismatch')
    for row in rows:
        if hashlib.sha256(protected(row['path'])).hexdigest()!=row.get('sha256'):
            raise RuntimeError('Host installer payload differs from its package provenance')
    return record


def docker(*arguments):
    result = subprocess.run([DOCKER, '--host', 'unix:///var/run/docker.sock', '--config',
        '/etc/kura-management-console/docker-client', *arguments], env={'PATH':'/usr/sbin:/usr/bin:/sbin:/bin'},
                            capture_output=True, text=True, timeout=15)
    if result.returncode:
        raise RuntimeError('Container operation cannot be verified; inspect the original container')
    return result.stdout


def containers():
    rows = []
    for line in docker('ps', '--all', '--no-trunc', '--filter', 'label=' + LABEL + '=1', '--format', '{{json .}}').splitlines():
        entry = json.loads(line)
        if re.fullmatch(r'[0-9a-f]{64}', entry['ID']):
            labels=json.loads(docker('inspect','--format','{{json .Config.Labels}}',entry['ID'])) or {}
            if labels.get('com.yofc.console.role')=='maintenance-helper':continue
            owner=labels.get('com.yofc.console.deployment')
            display_name=entry['Names']
            if owner:
                selected=DockerControl().generation(owner)
                if selected['containerId']!=entry['ID']:continue # Retained history is never presented as the active center.
                display_name=selected['name']
            rows.append({'id':entry['ID'], 'name':display_name[:128], 'state':entry['State']})
    return rows


def container_credential(identity):
    if not re.fullmatch(r'[0-9a-f]{64}', identity):
        raise RuntimeError('Select an exact container ID')
    labels = json.loads(docker('inspect', '--format', '{{json .Config.Labels}}', identity)) or {}
    if labels.get(LABEL) != '1' or docker('inspect', '--format', '{{.State.Running}}', identity).strip() != 'true':
        raise RuntimeError('Select a running YOFC console container')
    metadata=docker('exec','--user','console',identity,'stat','-c','%u %a',
        '/usr/share/kura-management-console/local-command-provenance.json','/usr/local/bin/yofc-management-console')
    owners=[line.split() for line in metadata.splitlines()]
    if len(owners)!=2 or any(owner!='0' or int(mode,8)&0o022 for owner,mode in owners):
        raise RuntimeError('Container executable and provenance must be protected by root')
    record=json.loads(docker('exec','--user','console',identity,'cat',
        '/usr/share/kura-management-console/local-command-provenance.json'))
    binary_hash=docker('exec','--user','console',identity,'sha256sum','/usr/local/bin/yofc-management-console').split()[0]
    if record.get('schemaVersion')!=1 or 'installation-credential' not in record.get('localReadCommands',[]) \
            or record.get('binarySha256')!=binary_hash:
        raise RuntimeError('Container has no verified support for credential export')
    # Never execute a host shell or an arbitrary program/container user.
    output = docker('exec', '--user', 'console', identity,
                    '/usr/local/bin/yofc-management-console', 'installation-credential')
    return json.loads(output)


def active_process_digest(pid):
    if not re.fullmatch(r'[1-9][0-9]*',pid):
        raise RuntimeError('Invalid service process identity')
    # This is a fixed kernel-managed magic link, not a console-owned path.
    # Keep Setup.path's symlink refusal for all configuration/state files.
    return hashlib.sha256((Path('/proc')/pid/'exe').read_bytes()).hexdigest()


def status(setup):
    result = setup.systemctl('show', setup.unit, '-p', 'ActiveState', '-p', 'MainPID', '-p', 'Job')
    if result is None:
        raise RuntimeError('A systemd host is required')
    fields = dict(line.split('=',1) for line in result.stdout.splitlines() if '=' in line)
    overrides=setup.overrides(missing_ok=True)
    value={'unit':setup.unit,'state':fields.get('ActiveState','unknown'),
            'pid':fields.get('MainPID','0'),'job':fields.get('Job',''),
            'httpAddress':overrides.get('CONSOLE_HTTP_ADDR'),
            'httpPort':int(overrides.get('CONSOLE_SETUP_PORT','18443')),
            'ready':False,'readiness':'NOT_READY'}
    maintenance=HostControl(setup).status()
    if maintenance['active'] is not None:
        value['readiness']='MAINTENANCE';value['maintenance']=maintenance['active'];return value
    if value['state']!='active' or not re.fullmatch(r'[1-9][0-9]*',value['pid']):
        return value
    try:
        record=setup.verify_local_command('installation-status')
        if active_process_digest(value['pid'])!=record['binarySha256']:
            raise RuntimeError('Active executable differs from installed provenance')
    except (OSError,RuntimeError,KeyError,TypeError,ValueError):
        value['readiness']='UNVERIFIED'
        return value
    try:
        description=setup.bound_command('installation-status',include_overrides=True)
        value['httpAddress']=description.get('httpAddress')
        value['httpPort']=description['httpPort']
        value['ready']=probe_listener(description)
        value['readiness']='READY' if value['ready'] else 'NOT_READY'
    except (OSError,RuntimeError,KeyError,TypeError,ValueError,subprocess.TimeoutExpired):
        pass  # A partial bootstrap or expired observation is never a new start.
    return value


def probe_listener(description,readiness=False):
    """Read local health only after matching the exact private-host leaf pin.
    Public PKI/hostname validation at the published URL remains a browser task.
    No credential, Cookie, database password or application write is sent.
    """
    port=description.get('httpPort')
    fingerprint=description.get('certificateFingerprint','')
    if description.get('schemaVersion')!=1 or type(port) is not int or not 1024<=port<=65535 \
            or not re.fullmatch(r'[a-f0-9]{64}',fingerprint):
        return False
    address=description.get('httpAddress')
    hosts=['::1','127.0.0.1']
    if address:
        host,separator,number=address.rpartition(':')
        if not separator or number!=str(port):
            return False
        host=host.removeprefix('[').removesuffix(']')
        ip=ipaddress.ip_address(host)
        if not ip.is_unspecified:
            hosts=[host]
        elif ip.version==4:
            hosts=['127.0.0.1']
    context=ssl.SSLContext(ssl.PROTOCOL_TLS_CLIENT)
    context.check_hostname=False
    context.verify_mode=ssl.CERT_NONE  # Exact out-of-band leaf pin is checked before HTTP.
    for host in hosts:
        connection=None
        try:
            with socket.create_connection((host,port),timeout=0.4) as raw:
                with context.wrap_socket(raw,server_hostname='localhost') as peer:
                    if hashlib.sha256(peer.getpeercert(binary_form=True)).hexdigest()!=fingerprint:
                        return False
                    connection=http.client.HTTPConnection('localhost',port,timeout=0.4)
                    connection.sock=peer
                    connection.request('GET','/health/ready' if readiness else '/health/live',headers={'Connection':'close'})
                    response=connection.getresponse();body=response.read(1025)
                    result=json.loads(body) if len(body)<=1024 else None
                    return response.status==200 and isinstance(result,dict) \
                        and result.get('status') in (['ready'] if readiness else ['setup-required','running'])
        except (OSError,ssl.SSLError,http.client.HTTPException,ValueError):
            continue
        finally:
            if connection:
                connection.close()
    return False


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action', choices=['status','start','credential','containers',
        'agent-overview','agent-authorize','agent-complete','agent-status',
        'native-overview','native-operation','native-upload-prepare','native-install','native-retire','native-verify','native-finalize',
        'docker-overview','docker-operation','docker-upload-prepare','docker-import','docker-verify-import','docker-deploy','docker-status',
        'docker-maintenance-begin','docker-maintenance-stop','docker-maintenance-resume','docker-maintenance-upgrade','docker-maintenance-verify-start',
        'maintenance-status','maintenance-begin','maintenance-stop','maintenance-complete','maintenance-cancel','maintenance-cancel-finalize','maintenance-adopt-offline'])
    parser.add_argument('--instance')
    parser.add_argument('--container')
    parser.add_argument('--port', type=int)
    parser.add_argument('--operation')
    parser.add_argument('--initialization')
    parser.add_argument('--image')
    parser.add_argument('--name')
    parser.add_argument('--http-port',type=int)
    parser.add_argument('--grpc-port',type=int)
    parser.add_argument('--deployment')
    parser.add_argument('--package')
    args=parser.parse_args()
    if os.geteuid()!=0:
        raise RuntimeError('Host administrator authorization is required')
    verify_installer_payload()
    if args.action.startswith('agent-'):
        supplied={key for key in ['operation','initialization','package','instance','container','port','image','name','http_port','grpc_port','deployment'] if getattr(args,key) is not None}
        allowed=set() if args.action=='agent-overview' else {'operation','initialization'}
        if supplied!=allowed:raise RuntimeError('Fixed local native agent action arguments required')
        control=AgentControl()
        value=control.overview() if args.action=='agent-overview' else control.request(
            args.action.removeprefix('agent-'),args.operation,args.initialization)
        print(json.dumps(value));return
    if args.initialization is not None:raise RuntimeError('Unexpected native agent initialization context')
    if args.action.startswith('native-'):
        supplied={key for key in ['operation','package','instance','container','port','image','name','http_port','grpc_port','deployment'] if getattr(args,key) is not None}
        allowed={'native-overview':set(),'native-upload-prepare':{'operation','package'},
                 'native-operation':{'operation'},'native-install':{'operation'},'native-retire':{'operation'},
                 'native-verify':{'operation'},'native-finalize':{'operation'}}[args.action]
        if supplied!=allowed:raise RuntimeError('Fixed native package action arguments required')
        control=NativePackages()
        if args.action=='native-overview':value=control.overview()
        elif args.action=='native-upload-prepare':value=control.prepare_upload(args.operation,args.package)
        elif args.action=='native-operation':value=control.status(args.operation)
        elif args.action=='native-install':value=control.install(args.operation)
        elif args.action=='native-retire':value=control.retire(args.operation)
        elif args.action=='native-verify':value=control.verify(args.operation)
        else:value=control.finalize(args.operation)
        print(json.dumps(value));return
    if args.package is not None:raise RuntimeError('Unexpected native package selection')
    if args.action.startswith('docker-'):
        if args.instance or args.container or args.port is not None:raise RuntimeError('Unexpected Docker target')
        supplied={key for key in ['operation','image','name','http_port','grpc_port','deployment'] if getattr(args,key) is not None}
        allowed={'docker-overview':set(),'docker-operation':{'operation'},'docker-status':{'operation'},
                 'docker-upload-prepare':{'operation','image'},'docker-import':{'operation'},'docker-verify-import':{'operation'},
                 'docker-deploy':{'operation','image','name','http_port','grpc_port'},
                 'docker-maintenance-begin':{'operation','deployment'},'docker-maintenance-stop':{'operation'},
                 'docker-maintenance-verify-start':{'operation'},
                 'docker-maintenance-resume':{'operation'},'docker-maintenance-upgrade':{'operation','image'}}[args.action]
        if supplied!=allowed:raise RuntimeError('Fixed Docker action arguments required')
        control=DockerControl()
        if args.action=='docker-overview':value=control.overview()
        elif args.action=='docker-maintenance-begin':value=MaintenanceControl().begin(args.operation,args.deployment)
        elif args.action.startswith('docker-maintenance-'):value=MaintenanceControl().request(args.operation,args.action.removeprefix('docker-maintenance-'),args.image)
        elif args.action=='docker-upload-prepare':value=control.prepare_upload(args.operation,args.image)
        elif args.action=='docker-import':value=control.import_image(args.operation)
        elif args.action=='docker-verify-import':value=control.verify_import(args.operation)
        elif args.action=='docker-deploy':value=control.deploy(args.operation,args.image,args.name,args.http_port,args.grpc_port)
        elif args.action=='docker-status':value=control.deployment_status(args.operation,probe_listener)
        else:value=control.status(args.operation)
        print(json.dumps(value));return
    if any(getattr(args,key) is not None for key in ['image','name','http_port','grpc_port','deployment']):raise RuntimeError('Unexpected Docker argument')
    if args.action.startswith('maintenance-'):
        if args.instance or args.container or args.port is not None:
            raise RuntimeError('Host maintenance includes every installed native instance')
        control=HostControl(Setup())
        if args.action=='maintenance-status':
            value=control.status()
            if args.operation:
                identity=control.identity(args.operation)
                try: value['operation']=control.journal(identity)
                except FileNotFoundError: value['operation']={'operationId':identity,'phase':'UNOBSERVED','transportNonDeliveryCertified':False}
        else:
            identity=control.identity(args.operation or '')
            if args.action in ['maintenance-begin','maintenance-adopt-offline']:
                value=control.begin(identity,offline_legacy=args.action=='maintenance-adopt-offline')
            elif args.action=='maintenance-stop': value=control.stop(identity)
            elif args.action=='maintenance-cancel': value=control.cancel(identity)
            elif args.action=='maintenance-cancel-finalize': value=control.cancel(identity,finalize_only=True)
            else: value=control.complete(identity)
        print(json.dumps(value));return
    if args.operation: raise RuntimeError('Unexpected maintenance operation')
    if args.instance is not None and not valid_instance(args.instance):
        raise RuntimeError('Invalid console instance')
    if args.container and (args.instance or args.action!='credential'):
        raise RuntimeError('Container setup supports credential delivery only')
    if args.port is not None and args.action!='start':
        raise RuntimeError('A listener port is only selected at first start')
    if args.action=='containers':
        if args.instance or args.container or args.port:
            raise RuntimeError('Unexpected target')
        value=containers()
    elif args.container:
        value=container_credential(args.container)
    else:
        setup=Setup(instance=args.instance)
        if args.action=='credential':
            value=setup.bound_command('installation-credential')
        elif args.action=='status':
            value=status(setup)
        else:
            with HostControl(setup).starting():
                current=status(setup)
                if current['state']!='active':
                    if current['state'] not in ['inactive','failed'] or current['job']:
                        raise RuntimeError('Observe the current service operation before starting')
                    if args.instance and not setup.path(setup.state).exists():
                        setup.init_instance(quiet=True)
                    setup.initial_listener(args.port if args.port is not None else 18443, allow_unpaired=True)
                    setup.activate()
            value=status(setup)
    print(json.dumps(value))


if __name__=='__main__':
    try:
        main()
    except Exception:
        # Do not copy child stderr, private material or arbitrary diagnostics into
        # Cockpit's journal or the browser. Fail closed, preserving original state.
        print('Host installer action failed; inspect host service state and retain the original records.',file=sys.stderr)
        sys.exit(1)
