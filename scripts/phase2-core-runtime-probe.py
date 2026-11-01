#!/usr/bin/env python3
"""Stage/restore a test-only external-configuration probe in the stopped isolated IDEA runtime."""
import argparse
import hashlib
import json
from pathlib import Path
import shutil
import socket
import subprocess
import zipfile

ROOT = Path(__file__).resolve().parents[1]
WORKSPACE = ROOT.parent
DEV = ROOT / '.phase1-dev'
RUNTIME = DEV / 'kura-runtime'
BACKUP = DEV / 'provider-core-probe-backup'
STATE = DEV / 'provider-core-probe-state.json'
SNAPSHOTS = DEV / 'kura-home/user/snapshots'
OUTPUT = ROOT / 'target/phase2-provider-runtime-probe.json'
PROBE_NAME = 'yofc-external-configuration-test-probe.jar'
CLASS = 'org/eclipse/kura/core/configuration/probe/ExternalConfigurationRuntimeProbe'
MODULE = WORKSPACE / 'kura/kura/org.eclipse.kura.core.configuration'
ARTIFACTS = {
    'org.eclipse.kura.api': (
        WORKSPACE / 'kura/kura/org.eclipse.kura.api/target/org.eclipse.kura.api-3.0.0-SNAPSHOT.jar',
        'org.eclipse.kura-org.eclipse.kura.api-3.0.0-SNAPSHOT.jar'),
    'org.eclipse.kura.core.configuration': (
        MODULE / 'target/org.eclipse.kura.core.configuration-3.0.0-SNAPSHOT.jar',
        'org.eclipse.kura-org.eclipse.kura.core.configuration-3.0.0-SNAPSHOT.jar'),
    'org.eclipse.kura.management.bridge': (
        WORKSPACE / 'kura-networking/bundles/org.eclipse.kura.management.bridge/target/org.eclipse.kura.management.bridge-1.0.0-SNAPSHOT.jar',
        'org.eclipse.kura.management.bridge-1.0.0-SNAPSHOT.jar'),
}


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def stopped():
    processes = subprocess.check_output(['ps', '-axo', 'pid,command'], text=True)
    if any('/bin/java ' in line and str(RUNTIME / 'jvm.args') in line for line in processes.splitlines()):
        raise SystemExit('Stop the isolated Kura in IDEA before preparing/restoring it')
    try:
        with socket.create_connection(('127.0.0.1', 18080), timeout=1):
            raise SystemExit('Port 18080 is active; do not replace runtime files while it is running')
    except (ConnectionRefusedError, TimeoutError):
        pass


def save(state):
    STATE.write_text(json.dumps(state, indent=2) + '\n')
    STATE.chmod(0o600)


def manifest(headers):
    lines = []
    for key, value in headers.items():
        line = f'{key}: {value}'
        while len(line.encode()) > 70:
            lines.append(line[:70])
            line = ' ' + line[70:]
        lines.append(line)
    return ('\r\n'.join(lines) + '\r\n\r\n').encode()


def prepare():
    stopped()
    if STATE.exists() or BACKUP.exists() or OUTPUT.exists():
        raise SystemExit('Probe state/evidence already exists; inspect it instead of repeating preparation')
    source = MODULE / f'src/test/java/{CLASS}.java'
    compiled = MODULE / f'target/test-classes/{CLASS}.class'
    if not compiled.exists() or source.stat().st_mtime_ns > compiled.stat().st_mtime_ns:
        raise SystemExit('Compile the current test-only probe before staging it')
    arguments = RUNTIME / 'jvm.args'
    if 'kura.configuration.external.pids' in arguments.read_text():
        raise SystemExit('Existing external PID reservation needs explicit reconciliation')
    BACKUP.mkdir(mode=0o700)
    for path in [arguments, RUNTIME / 'configuration/config.ini', RUNTIME / 'inventory.json']:
        shutil.copy2(path, BACKUP / path.name)
        (BACKUP / path.name).chmod(0o600)
    for _, name in ARTIFACTS.values():
        shutil.copy2(RUNTIME / 'plugins' / name, BACKUP / name)
    state = {'phase': 'preparing', 'runtime': str(RUNTIME), 'fixtureOnly': True,
             'initialSnapshots': sorted(path.name for path in SNAPSHOTS.glob('snapshot_*.xml')),
             'probeOutput': str(OUTPUT), 'artifacts': {}}
    save(state)
    inventory = json.loads((RUNTIME / 'inventory.json').read_text())
    for symbolic_name, (artifact, name) in ARTIFACTS.items():
        destination = RUNTIME / 'plugins' / name
        shutil.copy2(artifact, destination)
        with zipfile.ZipFile(artifact) as archive:
            text = archive.read('META-INF/MANIFEST.MF').decode().replace('\r\n ', '')
            version = next(line.split(': ', 1)[1].strip() for line in text.splitlines()
                           if line.startswith('Bundle-Version: '))
        entry = next((item for item in inventory if item['symbolicName'] == symbolic_name), None)
        if entry is None:
            entry = {'symbolicName': symbolic_name, 'startLevel': 5, 'start': True,
                     'source': 'local-reviewed-artifact'}
            inventory.append(entry)
        entry.update(path=f'plugins/{name}', sha256=digest(destination), bundleVersion=version)
        state['artifacts'][symbolic_name] = {'sha256': digest(destination), 'path': str(destination)}
    (RUNTIME / 'inventory.json').write_text(json.dumps(inventory, indent=2) + '\n')
    probe = RUNTIME / 'plugins' / PROBE_NAME
    with zipfile.ZipFile(probe, 'w') as archive:
        archive.writestr('META-INF/MANIFEST.MF', manifest({
            'Manifest-Version': '1.0', 'Bundle-ManifestVersion': '2',
            'Bundle-SymbolicName': 'org.eclipse.kura.management.external.probe', 'Bundle-Version': '1.0.0',
            'Bundle-Activator': CLASS.replace('/', '.'),
            'Import-Package': 'org.eclipse.kura,org.eclipse.kura.configuration;version="[1.3,2)",'
                              'org.eclipse.kura.core.configuration;version="[2,3)",'
                              'org.eclipse.kura.core.configuration.metatype,org.osgi.framework,org.osgi.service.cm',
        }))
        for suffix in ['.class', '$Fallible.class']:
            archive.write(MODULE / f'target/test-classes/{CLASS}{suffix}', f'{CLASS}{suffix}')
    config = RUNTIME / 'configuration/config.ini'
    config.write_text(config.read_text().replace('osgi.bundles=',
                      f'osgi.bundles=reference:file:plugins/{PROBE_NAME}@5:start,', 1))
    with arguments.open('a') as out:
        out.write('\n"-Dkura.configuration.external.pids=org.eclipse.kura.management.externalProbe"\n')
        out.write(f'"-Dkura.management.probe.output={OUTPUT}"\n')
    state.update(phase='prepared', probeSha256=digest(probe))
    save(state)
    print(json.dumps(state, indent=2))


def restore():
    stopped()
    state = json.loads(STATE.read_text())
    if state['phase'] not in ['prepared', 'preparing']:
        raise SystemExit('Probe already restored; do not repeat mutations')
    shutil.copy2(BACKUP / 'jvm.args', RUNTIME / 'jvm.args')
    shutil.copy2(BACKUP / 'config.ini', RUNTIME / 'configuration/config.ini')
    if state['phase'] == 'preparing':
        shutil.copy2(BACKUP / 'inventory.json', RUNTIME / 'inventory.json')
        for _, name in ARTIFACTS.values():
            shutil.copy2(BACKUP / name, RUNTIME / 'plugins' / name)
    archive = BACKUP / 'created-snapshots'
    archive.mkdir(exist_ok=True)
    moved = []
    for path in SNAPSHOTS.glob('snapshot_*.xml'):
        if path.name not in state['initialSnapshots']:
            destination = archive / path.name
            if destination.exists():
                raise SystemExit('Snapshot archive collision; preserve both before continuing')
            path.rename(destination)
            moved.append(str(destination))
    state.update(phase='restored', archivedSnapshots=moved, oldSnapshotsPreserved=True)
    save(state)
    print(json.dumps(state, indent=2))


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('action', choices=['prepare', 'restore', 'status'])
    action = parser.parse_args().action
    if action == 'prepare':
        prepare()
    elif action == 'restore':
        restore()
    else:
        print(STATE.read_text() if STATE.exists() else 'not prepared')
