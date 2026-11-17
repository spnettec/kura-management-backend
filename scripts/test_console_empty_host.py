#!/usr/bin/env python3
# Copyright (c) 2026 YOFC IOT and others
# SPDX-License-Identifier: EPL-2.0
"""Create one new native-console host; install only the host installer tools.

The center archive must subsequently be delivered through its actual GUI.
This fixture prepares stock OS/PAM/Cockpit prerequisites; it does not accept
the desktop launcher, install/start/init a center, create a database, or use an
existing Kura runtime. Every failed original case remains available for review.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import secrets
import subprocess
import time
import uuid


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--installer', type=Path, required=True)
    parser.add_argument('--installer-sha256', required=True)
    parser.add_argument('--console', type=Path, required=True)
    parser.add_argument('--console-sha256', required=True)
    parser.add_argument('--results', type=Path, required=True)
    parser.add_argument('--disposable-fixture', action='store_true', required=True)
    args = parser.parse_args()
    output = args.results.resolve()
    output.mkdir(mode=0o700, parents=True, exist_ok=False)

    def save(name, value):
        fd = os.open(output / name, os.O_CREAT | os.O_EXCL | os.O_WRONLY, 0o600)
        with os.fdopen(fd, 'w') as stream:
            json.dump(value, stream, indent=2)
            stream.write('\n')

    def command(arguments, timeout=30):
        return subprocess.run(arguments, capture_output=True, text=True,
                              check=True, timeout=timeout).stdout

    packages = {}
    for role in ('installer', 'console'):
        path = getattr(args, role).resolve(strict=True)
        expected = getattr(args, role + '_sha256')
        assert hashlib.sha256(path.read_bytes()).hexdigest() == expected
        packages[role] = dict(path=str(path), sha256=expected)
    image = 'yofc-kura-main-archive-tests:ubuntu24.04'
    image_id = json.loads(command(['docker', 'image', 'inspect', image]))[0]['Id']
    create = ['docker', 'run', '--detach', '--pull=never', '--name', 'yofc-console-empty-' + uuid.uuid4().hex[:10],
              '--label', 'yofc.purpose=native-console-empty-host', '--privileged', '--cgroupns=private',
              '--memory=1536m', '--cpus=2', '--tmpfs', '/run:rw,nosuid,nodev,mode=755,size=96m',
              '--tmpfs', '/run/lock:rw,nosuid,nodev,mode=755,size=2m',
              '--publish', '127.0.0.1::9090', '--publish', '127.0.0.1::18443',
              '--publish', '127.0.0.1::18444', '--env', 'PYTHONDONTWRITEBYTECODE=1',
              '--mount', 'type=bind,src=' + packages['installer']['path'] + ',dst=/candidate-installer.deb,readonly',
              '--entrypoint', '/bin/sh', image, '-c', 'mount --make-rshared /run && exec /sbin/init']
    cid = command(create).strip()
    save('owned-fixture.json', dict(containerId=cid, imageId=image_id, packages=packages,
                                   adoptExistingForbidden=True, automaticReplayForbidden=True,
                                   desktopLauncherAcceptance=False))
    print('OWNED_EMPTY_CONSOLE_FIXTURE ' + cid, flush=True)
    for _ in range(40):
        ready = subprocess.run(['docker', 'exec', cid, 'systemctl', 'show-environment'], capture_output=True)
        if ready.returncode == 0:
            break
        time.sleep(0.25)
    else:
        raise RuntimeError('New systemd fixture did not become ready; retain it')
    command(['docker', 'exec', cid, 'python3', '-I', '-c', """from pathlib import Path
import os
assert Path('/.dockerenv').exists()
for path in ['/opt/eclipse', '/etc/kura', '/var/lib/kura-key-service',
 '/etc/kura-management-console', '/var/lib/kura-management-console',
 '/usr/bin/yofc-management-console', '/var/lib/yofc-console-native-packages']:
 assert not os.path.lexists(path), 'Never adopt a previous environment: '+path
# Let stock OS dependencies follow their normal service installation policy.
# No center program is installed, and the tools archive has no maintainer scripts.
p=Path('/usr/sbin/policy-rc.d');p.write_text('#!/bin/sh\\nexit 0\\n');p.chmod(0o755)
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

    if native('os-package-index', ['apt-get', 'update', '-qq']):
        return 1
    if native('installer-first-install', ['apt-get', 'install', '-y', '--no-install-recommends', '/candidate-installer.deb']):
        return 1
    user = 'yofcfixture'
    password = secrets.token_urlsafe(24)
    save('fixture-access.json', dict(user=user, password=password,
                                    origin='new disposable OS/PAM account; never a personal credential'))
    command(['docker', 'exec', cid, 'useradd', '--create-home', '--groups', 'sudo', user])
    subprocess.run(['docker', 'exec', '-i', cid, 'chpasswd'], input=user + ':' + password + '\n',
                   text=True, check=True, capture_output=True, timeout=20)
    # This is the stock OS service prerequisite, not a console start/init action.
    save('stock-cockpit-prerequisite-intent.json', dict(command=['systemctl', 'start', 'cockpit.socket'],
                                                       productDesktopLauncherAccepted=False))
    command(['docker', 'exec', cid, 'systemctl', 'start', 'cockpit.socket'])
    observation = json.loads(command(['docker', 'exec', cid, 'python3', '-I', '-c', """from pathlib import Path
import json,subprocess
assert not Path('/usr/bin/yofc-management-console').exists()
assert not Path('/var/lib/kura-management-console').exists()
assert not Path('/var/lib/kura-key-service').exists()
print(json.dumps(dict(installerStatus=subprocess.check_output(['dpkg-query','-W','-f=${Status}',
 'kura-management-console-installer'],text=True),cockpit=subprocess.check_output(['systemctl','show',
 'cockpit.socket','-p','ActiveState,SubState,LoadState'],text=True),centerProgramAbsent=True,
 centerStateAbsent=True,rootKeyJournalAbsent=True)))
"""]))
    save('installer-only-observation.json', observation)
    save('host-ports.json', json.loads(command(['docker', 'inspect', '--format', '{{json .NetworkSettings.Ports}}', cid])))
    print('INSTALLER_ONLY_READY; no center software, configuration or database installed', flush=True)
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
