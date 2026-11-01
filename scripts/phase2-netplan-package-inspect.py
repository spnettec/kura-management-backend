#!/usr/bin/env python3
"""Inspect real deb archives on the host before any VM installation."""
import argparse
import hashlib
import io
import json
from pathlib import Path
import subprocess
import tarfile
import zipfile


def archive(path, prefix):
    members = subprocess.check_output(['ar', 't', str(path)], text=True).splitlines()
    name, = [name for name in members if name.startswith(prefix + '.tar')]
    return tarfile.open(fileobj=io.BytesIO(subprocess.check_output(['ar', 'p', str(path), name])), mode='r:*')


def inspect(core, network):
    evidence = []
    for path, package in [(core, 'kura-core-netplan'), (network, 'kura-networking-netplan')]:
        with archive(path, 'control') as control, archive(path, 'data') as data:
            controls = {m.name.removeprefix('./').lstrip('/'): m for m in control.getmembers()}
            metadata = control.extractfile(controls['control']).read().decode()
            assert 'Package: ' + package + '\n' in metadata
            depends = '\n'.join(line for line in metadata.splitlines() if line.startswith(('Depends:', 'Pre-Depends:')))
            assert not any(p in depends for p in ['network-manager', 'modemmanager', 'iptables', 'dnsmasq', 'bind9'])
            payload = {m.name.removeprefix('./').lstrip('/'): m for m in data.getmembers()}
            assert all(m.uid == 0 and m.gid == 0 for m in payload.values())
            assert not any(name.startswith(('var/lib/', 'etc/netplan/', 'etc/NetworkManager/')) for name in payload)
            assert not any(name.endswith(('kura_install.sh', 'kura_network_install.sh', 'firewall.service')) for name in payload)
            names = []
            for name, member in payload.items():
                if name.endswith('.jar'):
                    with zipfile.ZipFile(io.BytesIO(data.extractfile(member).read())) as jar:
                        text = jar.read('META-INF/MANIFEST.MF').decode().replace('\r\n', '\n').replace('\n ', '')
                    identity = next(line.split(': ', 1)[1].split(';')[0] for line in text.splitlines() if line.startswith('Bundle-SymbolicName: '))
                    assert identity not in names
                    assert identity not in {'org.eclipse.kura.nm', 'org.eclipse.kura.linux.net', 'org.eclipse.kura.net.admin.firewall', 'org.eclipse.kura.network.threat.manager'}
                    names.append(identity)
            if package == 'kura-core-netplan':
                from xml.dom import minidom
                seed = data.extractfile(payload['usr/share/kura-netplan/defaults/user/snapshots/snapshot_0.xml']).read()
                assert len(minidom.parseString(seed).getElementsByTagName('esf:configuration')) == 13
                for name in ['user', 'data', 'packages', 'load']:
                    link = payload['opt/eclipse/kura/' + name]
                    assert link.issym() and link.linkname == '/var/lib/kura-netplan/' + name
                for name in ['preinst', 'postinst', 'prerm', 'postrm']:
                    assert controls[name].mode & 0o111
                    script = control.extractfile(controls[name]).read().decode()
                    compile(script, name, 'exec')
                    assert 'SCRIPT_NAME = ' + repr(name) in script
                assert control.extractfile(controls['conffiles']).read().decode().strip() == '/etc/kura-netplan/runtime.json'
                unit = data.extractfile(payload['usr/lib/systemd/system/kura-netplan.service']).read().decode()
                assert 'CapabilityBoundingSet=\n' in unit and 'User=kura-netplan\n' in unit
                assert 'SuccessExitStatus=143\n' in unit
                assert 'NetworkManager' not in unit and 'ModemManager' not in unit
                assert 'SECRET_KEY' not in unit
            else:
                assert len(names) == 5
                assert not ({'preinst', 'postinst', 'postrm'} & controls.keys())
            evidence.append({'package': package, 'path': str(path), 'sha256': hashlib.sha256(path.read_bytes()).hexdigest(),
                             'bundleCount': len(names), 'payloadEntries': len(payload), 'metadata': metadata,
                             'result': 'passed'})
    return {'checks': evidence, 'linuxLifecycle': 'pending', 'matrixOtherPlatforms': 'deferred'}


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('core', type=Path)
    parser.add_argument('network', type=Path)
    args = parser.parse_args()
    print(json.dumps(inspect(args.core, args.network), ensure_ascii=False, indent=2))
