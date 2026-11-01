#!/usr/bin/env python3
"""One-shot, reversible Netplan adapter profile for the existing isolated IDEA runtime."""
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import sys
import zipfile

ROOT = Path(__file__).resolve().parents[1]
DEV = ROOT / '.phase1-dev'
RUNTIME = DEV / 'kura-runtime'
BACKUP = DEV / 'netplan-adapter-backup'
STATE = DEV / 'netplan-adapter-probe-state.json'
MODULE = ROOT.parent / 'kura-networking/bundles/org.eclipse.kura.netplan'
SNAPSHOTS = DEV / 'kura-home/user/snapshots'
OUTPUT = ROOT / 'target/phase2-kura-network-runtime.json'


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def stopped():
    rows = subprocess.run(['ps', 'ax', '-o', 'pid=,command='], check=True, text=True, capture_output=True).stdout
    live = [row for row in rows.splitlines() if 'java' in row and str(RUNTIME) in row and 'phase2-netplan-runtime-probe.py' not in row]
    if live:
        raise RuntimeError('Stop the isolated Kura JVM in IDEA before preparing/restoring this profile')


def write_probe(path):
    clazz = MODULE / 'target/test-classes/org/eclipse/kura/netplan/probe/NetplanRuntimeProbe.class'
    manifest = ('Manifest-Version: 1.0\nBundle-ManifestVersion: 2\n'
                'Bundle-SymbolicName: yofc.test.netplan.probe\nBundle-Version: 1.0.0\n'
                'Bundle-Activator: org.eclipse.kura.netplan.probe.NetplanRuntimeProbe\n'
                'Import-Package: org.osgi.framework,org.eclipse.kura,org.eclipse.kura.configuration,\n'
                ' org.eclipse.kura.management.bridge,org.eclipse.kura.netplan,org.eclipse.kura.net.status,com.google.gson\n\n')
    with zipfile.ZipFile(path, 'w') as jar:
        jar.writestr('META-INF/MANIFEST.MF', manifest)
        jar.write(clazz, 'org/eclipse/kura/netplan/probe/NetplanRuntimeProbe.class')


def prepare():
    stopped()
    if STATE.exists() or BACKUP.exists() or OUTPUT.exists():
        raise RuntimeError('Probe state/evidence already exists; do not replay a previous preparation')
    source = MODULE / 'target/org.eclipse.kura.netplan-1.0.0-SNAPSHOT.jar'
    clazz = MODULE / 'target/test-classes/org/eclipse/kura/netplan/probe/NetplanRuntimeProbe.class'
    if not source.is_file() or not clazz.is_file():
        raise RuntimeError('Build the reviewed module and test activator first')
    config = RUNTIME / 'configuration/config.ini'
    args = RUNTIME / 'jvm.args'
    if 'org.eclipse.kura.nm-' in config.read_text() or 'org.eclipse.kura.nm_' in config.read_text():
        raise RuntimeError('Isolated profile still contains the old writer')
    target = RUNTIME / 'plugins' / source.name
    probe = RUNTIME / 'plugins/netplan-runtime-probe.jar'
    if target.exists() or probe.exists():
        raise RuntimeError('Refusing to replace existing probe/provider artifacts')
    BACKUP.mkdir(mode=0o700)
    originals = []
    for path in (config, args):
        saved = BACKUP / path.name
        shutil.copy2(path, saved)
        originals.append({'path': str(path), 'backup': str(saved), 'sha256': digest(path)})
    state = {'status': 'preparing', 'originals': originals,
             'snapshots': {str(p): digest(p) for p in SNAPSHOTS.glob('snapshot_*.xml')},
             'provider': str(target), 'probe': str(probe)}
    STATE.write_text(json.dumps(state, indent=2))
    shutil.copy2(source, target)
    write_probe(probe)
    refs = f'reference:file:plugins/{target.name}@5:start,reference:file:plugins/{probe.name}@5:start,'
    config.write_text(config.read_text().replace('osgi.bundles=', 'osgi.bundles=' + refs, 1))
    with args.open('a') as stream:
        stream.write('\n"-Dkura.networking.provider=netplan"\n')
        stream.write('"-Dkura.configuration.external.pids=org.eclipse.kura.net.admin.NetworkConfigurationService"\n')
        stream.write(f'"-Dkura.netplan.probe.output={OUTPUT}"\n')
    for row in state['originals']:
        row['preparedSha256'] = digest(Path(row['path']))
    state.update(status='prepared', providerSha256=digest(target), probeSha256=digest(probe))
    STATE.write_text(json.dumps(state, indent=2))
    print(json.dumps({'status': 'prepared', 'runtime': str(RUNTIME), 'evidence': str(OUTPUT)}))


def refresh():
    stopped()
    state = json.loads(STATE.read_text())
    if state['status'] != 'prepared':
        raise RuntimeError('Only a prepared and stopped profile can refresh its reviewed artifacts')
    for row in state['originals']:
        if digest(Path(row['path'])) != row['preparedSha256']:
            raise RuntimeError('Prepared profile changed externally')
    for name in ('provider', 'probe'):
        if digest(Path(state[name])) != state[name + 'Sha256']:
            raise RuntimeError('Prepared artifact changed externally')
    revisions = state.setdefault('artifactRevisions', [])
    archive = BACKUP / f'artifact-revision-{len(revisions) + 1}'
    archive.mkdir()
    for name in ('provider', 'probe'):
        shutil.copy2(state[name], archive / Path(state[name]).name)
    if OUTPUT.exists():
        shutil.move(OUTPUT, archive / OUTPUT.name)
    revisions.append({'providerSha256':state['providerSha256'], 'probeSha256':state['probeSha256'], 'archive':str(archive)})
    # A core bootstrap fix may be required by the external profile. Preserve
    # the previously reviewed isolated JAR and keep the new one after restore.
    core_target = RUNTIME / 'plugins/org.eclipse.kura-org.eclipse.kura.core.configuration-3.0.0-SNAPSHOT.jar'
    core_source = ROOT.parent / 'kura/kura/org.eclipse.kura.core.configuration/target/org.eclipse.kura.core.configuration-3.0.0-SNAPSHOT.jar'
    supporting = state.setdefault('supportingArtifacts', {})
    if str(core_target) in supporting and digest(core_target) != supporting[str(core_target)]:
        raise RuntimeError('Reviewed isolated core artifact changed externally')
    shutil.copy2(core_target, archive / core_target.name)
    shutil.copy2(core_source, core_target)
    supporting[str(core_target)] = digest(core_target)
    shutil.copy2(MODULE / 'target/org.eclipse.kura.netplan-1.0.0-SNAPSHOT.jar', state['provider'])
    write_probe(Path(state['probe']))
    for name in ('provider', 'probe'):
        state[name + 'Sha256'] = digest(Path(state[name]))
    STATE.write_text(json.dumps(state, indent=2))
    print(json.dumps({'status':'prepared', 'artifactRefreshes':len(revisions)}))


def restore():
    stopped()
    state = json.loads(STATE.read_text())
    if state['status'] != 'prepared':
        raise RuntimeError('Only the exact prepared profile can be restored')
    for row in state['originals']:
        if digest(Path(row['path'])) != row['preparedSha256'] or digest(Path(row['backup'])) != row['sha256']:
            raise RuntimeError('Profile/backup changed externally; refusing overwrite')
    for name, value in state['snapshots'].items():
        if digest(Path(name)) != value:
            raise RuntimeError('Original snapshot changed; retain everything for review')
    if digest(Path(state['provider'])) != state['providerSha256'] or digest(Path(state['probe'])) != state['probeSha256']:
        raise RuntimeError('Reviewed probe/provider artifact changed externally')
    for path, value in state.get('supportingArtifacts', {}).items():
        if digest(Path(path)) != value:
            raise RuntimeError('Reviewed supporting artifact changed externally')
    for row in state['originals']:
        shutil.copy2(row['backup'], row['path'])
    created = BACKUP / 'created-snapshots'
    for path in SNAPSHOTS.glob('snapshot_*.xml'):
        if str(path) not in state['snapshots']:
            created.mkdir(exist_ok=True)
            shutil.move(path, created / path.name)
    shutil.move(state['probe'], BACKUP / Path(state['probe']).name)
    # Keep the reviewed provider JAR; the restored bundle list does not load it.
    state['status'] = 'restored'
    STATE.write_text(json.dumps(state, indent=2))
    print(json.dumps({'status': 'restored', 'originalSnapshotsPreserved': len(state['snapshots'])}))


if __name__ == '__main__':
    action = sys.argv[1] if len(sys.argv) == 2 else ''
    if action == 'prepare':
        prepare()
    elif action == 'restore':
        restore()
    elif action == 'refresh':
        refresh()
    elif action == 'status':
        print(STATE.read_text() if STATE.exists() else '{"status":"not-prepared"}')
    else:
        raise SystemExit('usage: phase2-netplan-runtime-probe.py prepare|refresh|restore|status')
