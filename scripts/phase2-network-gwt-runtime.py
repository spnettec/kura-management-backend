#!/usr/bin/env python3
"""One-shot reversible IDEA profile for GWT RPC/UI with a test-only network owner.

No host or guest network commands are run. The original isolated profile is restored exactly.
"""
import argparse
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import shutil
import zipfile

ROOT = Path(__file__).resolve().parents[1]
DEV = ROOT / '.phase1-dev'
RUNTIME = DEV / 'kura-runtime'
STATE = DEV / 'network-gwt-profile-state.json'
BACKUP = DEV / 'network-gwt-profile-backup'
SNAPSHOTS = DEV / 'kura-home/user/snapshots'
OUTPUT = ROOT / 'target/gwt-network-fixture.txt'
RESTORATION = ROOT / 'target/gwt-network-restoration.json'
RUN_ID = None


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def stopped():
    spec = importlib.util.spec_from_file_location('gwt_runtime_guard', ROOT / 'scripts/phase2-netplan-runtime-probe.py')
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    module.stopped()


def prepare():
    stopped()
    if RUN_ID:
        previous = json.loads((DEV / 'network-gwt-profile-state.json').read_text())
        if previous['status'] != 'restored':
            raise RuntimeError('The previous GWT profile must remain restored before a new named run')
        for row in previous['originals'] + previous['artifacts']:
            if digest(Path(row['path'])) != row['sha256']:
                raise RuntimeError('Previous restored baseline changed; preserve it for review')
    if STATE.exists() or BACKUP.exists() or OUTPUT.exists():
        raise RuntimeError('Keep existing one-shot state/evidence; do not repeat prepare')
    if json.loads((DEV / 'network-rest-profile-state.json').read_text())['status'] != 'restored':
        raise RuntimeError('Previous REST profile must be restored first')
    config = RUNTIME / 'configuration/config.ini'
    args = RUNTIME / 'jvm.args'
    if 'network-rest-runtime-probe.jar' in config.read_text() or 'network-gwt-fixture.jar' in config.read_text():
        raise RuntimeError('Unexpected existing test fixture')
    artifacts = []
    for repo, module, name in [
        ('kura', 'kura/org.eclipse.kura.api', 'org.eclipse.kura.api-3.0.0-SNAPSHOT.jar'),
        ('kura-management-ui', 'bundles/org.eclipse.kura.web2', 'org.eclipse.kura.web2-3.0.0-SNAPSHOT.jar'),
    ]:
        source = ROOT.parent / repo / module / 'target' / name
        targets = [p for p in (RUNTIME / 'plugins').iterdir() if p.name.endswith(name)]
        if not source.is_file() or len(targets) != 1:
            raise RuntimeError('Missing or ambiguous reviewed artifact: ' + name)
        artifacts.append({'path': str(targets[0]), 'source': str(source), 'sha256': digest(targets[0]),
                          'preparedSha256': digest(source)})
    clazz = ROOT.parent / ('kura-management-ui/bundles/org.eclipse.kura.web2.test/target/test-classes/'
                          'org/eclipse/kura/web/server/net2/configuration/probe/NetworkTransactionFixture.class')
    probe = RUNTIME / 'plugins/network-gwt-fixture.jar'
    if not clazz.is_file() or probe.exists():
        raise RuntimeError('Build the test-only fixture first; preserve any existing runtime fixture')
    BACKUP.mkdir(mode=0o700)
    originals = []
    for path in (config, args):
        saved = BACKUP / path.name
        shutil.copy2(path, saved)
        originals.append({'path': str(path), 'backup': str(saved), 'sha256': digest(path)})
    for row in artifacts:
        saved = BACKUP / Path(row['path']).name
        shutil.copy2(row['path'], saved); row['backup'] = str(saved)
    state = {'status': 'preparing', 'originals': originals, 'artifacts': artifacts,
             'snapshots': {str(p): digest(p) for p in SNAPSHOTS.glob('snapshot_*.xml')}}
    STATE.write_text(json.dumps(state, indent=2) + '\n')
    for row in artifacts:
        shutil.copy2(row['source'], row['path'])
    manifest = ('Manifest-Version: 1.0\nBundle-ManifestVersion: 2\n'
                'Bundle-SymbolicName: yofc.test.network.gwt.fixture\nBundle-Version: 1.0.0\n'
                'Bundle-Activator: org.eclipse.kura.web.server.net2.configuration.probe.NetworkTransactionFixture\n'
                'Import-Package: org.osgi.framework,org.eclipse.kura.net.transaction\n\n')
    with zipfile.ZipFile(probe, 'w') as jar:
        jar.writestr('META-INF/MANIFEST.MF', manifest)
        jar.write(clazz, 'org/eclipse/kura/web/server/net2/configuration/probe/NetworkTransactionFixture.class')
    config.write_text(config.read_text().replace('osgi.bundles=',
                     'osgi.bundles=reference:file:plugins/network-gwt-fixture.jar@5:start,', 1))
    with args.open('a') as stream:
        stream.write('\n"-Dkura.networking.provider=netplan"\n"-Dkura.have.net.admin=true"\n')
        stream.write(f'"-Dkura.network.gwt.fixture.output={OUTPUT}"\n')
    for row in originals:
        row['preparedSha256'] = digest(Path(row['path']))
    state.update(status='prepared', probe=str(probe), probeSha256=digest(probe))
    STATE.write_text(json.dumps(state, indent=2) + '\n')
    print(json.dumps({'status': 'prepared', 'testOnlyNetworkOwner': True, 'systemNetworkWrites': 0}))


def refresh():
    stopped()
    state = json.loads(STATE.read_text())
    if state['status'] != 'prepared' or not OUTPUT.exists() or 'prepares=0\n' not in OUTPUT.read_text():
        raise RuntimeError('Refresh only a stopped fixture that has never accepted a write')
    for row in state['originals'] + state['artifacts']:
        if digest(Path(row['path'])) != row['preparedSha256']:
            raise RuntimeError('Prepared profile changed externally')
    if digest(Path(state['probe'])) != state['probeSha256']:
        raise RuntimeError('Test fixture changed externally')
    revisions = state.setdefault('artifactRevisions', [])
    archive = BACKUP / ('artifact-revision-' + str(len(revisions) + 1)); archive.mkdir()
    for row in state['artifacts']:
        shutil.copy2(row['path'], archive / Path(row['path']).name)
        shutil.copy2(row['source'], row['path'])
        row['preparedSha256'] = digest(Path(row['path']))
    shutil.move(OUTPUT, archive / OUTPUT.name)
    revisions.append({'archive': str(archive), 'reason': 'pre-write status-page session regression'})
    STATE.write_text(json.dumps(state, indent=2) + '\n')
    print(json.dumps({'status': 'prepared', 'archivedRevision': str(archive)}))


def restore():
    stopped()
    state = json.loads(STATE.read_text())
    if state['status'] != 'prepared':
        raise RuntimeError('Restore requires the exact prepared/stopped profile')
    for row in state['originals'] + state['artifacts']:
        if digest(Path(row['path'])) != row['preparedSha256'] or digest(Path(row['backup'])) != row['sha256']:
            raise RuntimeError('Profile/artifact/backup changed externally; preserve it for review')
    if digest(Path(state['probe'])) != state['probeSha256']:
        raise RuntimeError('Test fixture changed externally')
    for path, sha in state['snapshots'].items():
        if digest(Path(path)) != sha:
            raise RuntimeError('Original snapshot changed; refusing overwrite')
    created = BACKUP / 'created-snapshots'
    for path in SNAPSHOTS.glob('snapshot_*.xml'):
        if str(path) not in state['snapshots']:
            created.mkdir(exist_ok=True); shutil.move(path, created / path.name)
    reviewed = BACKUP / 'reviewed-artifacts'; reviewed.mkdir()
    for row in state['artifacts']:
        shutil.copy2(row['path'], reviewed / Path(row['path']).name)
    shutil.move(state['probe'], reviewed / Path(state['probe']).name)
    for row in state['originals'] + state['artifacts']:
        shutil.copy2(row['backup'], row['path'])
        if digest(Path(row['path'])) != row['sha256']:
            raise RuntimeError('Restore verification failed')
    state['status'] = 'restored'; STATE.write_text(json.dumps(state, indent=2) + '\n')
    result = {'status': 'restored', 'originalArtifactsAndSnapshotsVerified': True, 'testOnlyNetworkOwner': True,
              'systemNetworkWrites': 0}
    RESTORATION.write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps(result))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action', choices=['prepare', 'refresh', 'restore', 'status'])
    parser.add_argument('--run-id', help='New one-shot run; previous state and evidence are preserved')
    arguments = parser.parse_args()
    RUN_ID = arguments.run_id
    if RUN_ID:
        if not re.fullmatch(r'[a-z0-9][a-z0-9-]{0,47}', RUN_ID):
            parser.error('run-id must contain 1-48 lowercase letters, digits or hyphens')
        STATE = DEV / f'network-gwt-{RUN_ID}-state.json'
        BACKUP = DEV / f'network-gwt-{RUN_ID}-backup'
        OUTPUT = ROOT / f'target/gwt-network-{RUN_ID}-fixture.txt'
        RESTORATION = ROOT / f'target/gwt-network-{RUN_ID}-restoration.json'
    action = arguments.action
    if action == 'prepare': prepare()
    elif action == 'refresh': refresh()
    elif action == 'restore': restore()
    elif action == 'status': print(STATE.read_text() if STATE.exists() else '{"status":"not-prepared"}')
