#!/usr/bin/env python3
# Copyright (c) 2026 YOFC IOT and others
# SPDX-License-Identifier: EPL-2.0
"""One new real systemd agent-first fixture; preserve any failed original case.

No Core Main start, key RPC, backup, publication, restore or lease request.
Only package-driven fresh key provisioning may occur when Core is installed.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import time
import uuid


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ('agent', 'core', 'results'):
        parser.add_argument('--' + name, type=Path, required=True)
    for name in ('agent-sha256', 'core-sha256'):
        parser.add_argument('--' + name, required=True)
    parser.add_argument('--disposable-fixture', action='store_true', required=True)
    args = parser.parse_args()
    output = args.results.resolve()
    output.mkdir(mode=0o700, parents=True, exist_ok=False)

    def save(name, value):
        fd = os.open(output / name, os.O_CREAT | os.O_EXCL | os.O_WRONLY, 0o600)
        with os.fdopen(fd, 'w') as stream:
            json.dump(value, stream, indent=2)
            stream.write('\n')

    def command(arguments, timeout=45):
        return subprocess.run(arguments, capture_output=True, text=True, check=True, timeout=timeout).stdout

    packages = []
    for name in ('agent', 'core'):
        path = getattr(args, name).resolve(strict=True)
        expected = getattr(args, name + '_sha256')
        assert hashlib.sha256(path.read_bytes()).hexdigest() == expected
        packages.append(dict(role=name, path=str(path), sha256=expected, target='/candidate-' + name + '.deb'))
    image = 'yofc-kura-main-archive-tests:ubuntu24.04'
    image_id = json.loads(command(['docker', 'image', 'inspect', image]))[0]['Id']
    create = ['docker', 'run', '--detach', '--pull=never', '--name', 'yofc-agent-first-' + uuid.uuid4().hex[:10],
              '--label', 'yofc.purpose=component-install-order', '--privileged', '--cgroupns=private',
              '--memory=1536m', '--cpus=2', '--tmpfs', '/run:rw,nosuid,nodev,mode=755,size=96m',
              '--tmpfs', '/run/lock:rw,nosuid,nodev,mode=755,size=2m', '--env', 'PYTHONDONTWRITEBYTECODE=1']
    for package in packages:
        create += ['--mount', 'type=bind,src=' + package['path'] + ',dst=' + package['target'] + ',readonly']
    create += ['--entrypoint', '/bin/sh', image, '-c', 'mount --make-rshared /run && exec /sbin/init']
    cid = command(create).strip()
    save('owned-fixture.json', dict(containerId=cid, imageId=image_id, packages=packages,
                                   adoptExistingForbidden=True, automaticReplayForbidden=True))
    print('OWNED_AGENT_FIRST_FIXTURE ' + cid, flush=True)
    for _ in range(40):
        ready = subprocess.run(['docker', 'exec', cid, 'systemctl', 'show-environment'], capture_output=True)
        if ready.returncode == 0:
            break
        time.sleep(0.25)
    else:
        raise RuntimeError('New systemd fixture did not become ready; retain it')
    command(['docker', 'exec', cid, 'python3', '-B', '-c', """from pathlib import Path
import os
assert Path('/.dockerenv').exists()
for name in ('/opt/eclipse', '/etc/kura', '/etc/kura-management-agent', '/var/lib/kura-key-service'):
 assert not os.path.lexists(name), 'Never adopt a previous environment'
p=Path('/usr/sbin/policy-rc.d');p.write_text('#!/bin/sh\\nexit 101\\n');p.chmod(0o755)
"""])

    def native(label, arguments):
        save(label + '-intent.json', dict(containerId=cid, arguments=arguments, automaticReplayForbidden=True))
        fd = os.open(output / (label + '.log'), os.O_CREAT | os.O_EXCL | os.O_WRONLY, 0o600)
        with os.fdopen(fd, 'wb') as log:
            result = subprocess.run(['docker', 'exec', '--env', 'DEBIAN_FRONTEND=noninteractive', cid, *arguments],
                                    stdout=log, stderr=subprocess.STDOUT, timeout=600)
        save(label + '-return.json', dict(returncode=result.returncode))
        print(label + ' RETURNED ' + str(result.returncode), flush=True)
        return result.returncode

    assert native('apt-update', ['apt-get', 'update']) == 0
    assert native('agent-install', ['apt-get', '-y', '--no-install-recommends', 'install', '/candidate-agent.deb']) == 0
    observation = r'''
from pathlib import Path
import hashlib,json,os,stat,subprocess
def meta(path):
 s=path.lstat();return dict(sha256=hashlib.sha256(path.read_bytes()).hexdigest(),uid=s.st_uid,gid=s.st_gid,mode=stat.S_IMODE(s.st_mode),inode=s.st_ino,mtimeNs=s.st_mtime_ns,ctimeNs=s.st_ctime_ns)
value=dict(line.split('=',1) for line in subprocess.check_output(['systemctl','show','kura-management-agent.service','--property=MainPID','--property=ActiveState'],text=True).splitlines())
assert value['ActiveState']=='active' and int(value['MainPID'])>0,value
pid=value['MainPID'];value['startTicks']=Path('/proc',pid,'stat').read_text().rsplit(')',1)[1].split()[19]
value['status']={line.split(':',1)[0]:line.split(':',1)[1].strip() for line in Path('/proc',pid,'status').read_text().splitlines() if line.split(':',1)[0] in ('Uid','Gid','CapEff','CapBnd','CapAmb','NoNewPrivs')}
value['private']={name:meta(Path(name)) for name in ('/etc/kura-management-agent/agent.env','/var/lib/kura-management-agent/remote-management.json')}
assert json.loads(Path('/var/lib/kura-management-agent/remote-management.json').read_text())['enabled'] is False
assert stat.S_ISSOCK(Path('/run/kura-management-agent/control.sock').stat().st_mode)
metadata=json.loads(Path('/usr/share/kura-management-agent/java-bridge.json').read_text());jar=Path(metadata['path'])
value['bridge']={'path':str(jar),'exists':jar.is_file(),'sha256':hashlib.sha256(jar.read_bytes()).hexdigest() if jar.is_file() else None,'expectedSha256':metadata['sha256']}
value['canonicalKeyExists']=Path('/etc/kura/credentials/kura_encryption_key').is_file()
print(json.dumps(value))
'''
    time.sleep(1)
    before = json.loads(command(['docker', 'exec', cid, 'python3', '-B', '-c', observation]))
    assert before['bridge']['sha256'] == before['bridge']['expectedSha256'] and not before['canonicalKeyExists']
    save('agent-before-core.json', before)
    code = native('core-install', ['apt-get', '-y', '--no-install-recommends', 'install',
                                   'openjdk-21-jre-headless', 'locales', '/candidate-core.deb'])
    command(['docker', 'network', 'disconnect', 'bridge', cid])
    after = json.loads(command(['docker', 'exec', cid, 'python3', '-B', '-c', observation]))
    save('agent-after-core.json', after)
    assert before['MainPID'] == after['MainPID'] and before['startTicks'] == after['startTicks']
    assert before['private'] == after['private']
    hidden = json.loads(command(['docker', 'exec', cid, 'python3', '-B', '-c', r'''
from pathlib import Path
import json,sys
root=Path('/proc',sys.argv[1],'root')
paths=['/etc/kura/credentials/kura_encryption_key','/var/lib/kura-key-service/journal.key']
result={p:{'hostExists':Path(p).is_file(),'agentNamespaceExists':(root/p.lstrip('/')).exists()} for p in paths}
own=Path('/var/lib/kura-management-agent/results.sqlite')
result['agentStatePersistent']=own.stat().st_ino==(root/'var/lib/kura-management-agent/results.sqlite').stat().st_ino
result['rootJournalCounts']={name:len(list((Path('/var/lib/kura-key-service')/name).iterdir())) for name in ('operations','startups') if (Path('/var/lib/kura-key-service')/name).exists()}
print(json.dumps(result))
''', after['MainPID']]))
    save('custody-namespace-observation.json', hidden)
    main_state = dict(line.split('=', 1) for line in command(['docker', 'exec', cid,
        'systemctl', 'show', 'kura.service', '--property=MainPID', '--property=ControlPID',
        '--property=ExecMainStartTimestampMonotonic']).splitlines())
    save('main-never-started.json', main_state)
    assert main_state == dict(MainPID='0', ControlPID='0', ExecMainStartTimestampMonotonic='0')
    assert hidden['agentStatePersistent'] and all(not hidden[path]['agentNamespaceExists'] for path in
        ('/etc/kura/credentials/kura_encryption_key', '/var/lib/kura-key-service/journal.key'))
    assert all(value == 0 for value in hidden['rootJournalCounts'].values())
    assert code == 0, 'Core installation failed; keep original scene and intent'
    assert after['bridge']['sha256'] == after['bridge']['expectedSha256'], 'Core installation lost the installed agent adapter'
    save('accepted.json', dict(agentFirstInstall=True, coreAfterAgent=True, agentNeverRestarted=True,
                               privateMetadataExact=True, dynamicCustodyHidden=True, mainStarts=0, keyRpcDispatches=0))


if __name__ == '__main__':
    main()
