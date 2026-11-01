#!/usr/bin/env python3
"""One-shot reversible profile for real Kura REST -> public API -> Rust UDS validation."""
import hashlib
import importlib.util
import json
from pathlib import Path
import shutil
import sys
import zipfile

ROOT = Path(__file__).resolve().parents[1]
DEV = ROOT / '.phase1-dev'
RUNTIME = DEV / 'kura-runtime'
STATE = DEV / 'network-rest-profile-state.json'
BACKUP = DEV / 'network-rest-profile-backup'
SNAPSHOTS = DEV / 'kura-home/user/snapshots'
PROBE_OUTPUT = ROOT / 'target/phase2-network-rest-runtime.json'
PROBE_FINISH = ROOT / 'target/phase2-network-rest-probe.finish'


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def stopped():
    spec = importlib.util.spec_from_file_location('netplan_probe', ROOT / 'scripts/phase2-netplan-runtime-probe.py')
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    module.stopped()


def prepare():
    stopped()
    if STATE.exists() or BACKUP.exists():
        raise RuntimeError('Existing one-shot profile; inspect it instead of repeating prepare')
    previous = json.loads((DEV / 'netplan-adapter-probe-state.json').read_text())
    if previous['status'] != 'restored':
        raise RuntimeError('Restore the previous adapter probe before starting this increment')
    config = RUNTIME / 'configuration/config.ini'
    args = RUNTIME / 'jvm.args'
    text = config.read_text()
    if 'org.eclipse.kura.nm-' in text or 'org.eclipse.kura.nm_' in text:
        raise RuntimeError('Old NM writer must be omitted')
    provider = 'reference:file:plugins/org.eclipse.kura.netplan-1.0.0-SNAPSHOT.jar@5:start,'
    if provider in text or 'osgi.bundles=' not in text:
        raise RuntimeError('Unexpected active provider or missing bundle list in original profile')
    artifacts = []
    for repo, module, name in [
        ('kura', 'kura/org.eclipse.kura.api', 'org.eclipse.kura.api-3.0.0-SNAPSHOT.jar'),
        ('kura-networking', 'bundles/org.eclipse.kura.netplan', 'org.eclipse.kura.netplan-1.0.0-SNAPSHOT.jar'),
        ('kura-networking', 'bundles/org.eclipse.kura.rest.network.configuration.provider',
         'org.eclipse.kura.rest.network.configuration.provider-3.0.0-SNAPSHOT.jar'),
    ]:
        source = ROOT.parent / repo / module / 'target' / name
        matches = [p for p in (RUNTIME / 'plugins').iterdir() if p.name.endswith(name)]
        if not source.is_file() or len(matches) != 1:
            raise RuntimeError(f'Missing or ambiguous reviewed artifact: {name}')
        artifacts.append({'path': str(matches[0]), 'source': str(source), 'sha256': digest(matches[0]),
                          'preparedSha256': digest(source)})
    BACKUP.mkdir(mode=0o700)
    originals = []
    for path in [config, args]:
        saved = BACKUP / path.name
        shutil.copy2(path, saved)
        originals.append({'path': str(path), 'backup': str(saved), 'sha256': digest(path)})
    for row in artifacts:
        saved = BACKUP / Path(row['path']).name
        shutil.copy2(row['path'], saved)
        row['backup'] = str(saved)
    state = {'status': 'preparing', 'originals': originals, 'artifacts': artifacts,
             'snapshots': {str(p): digest(p) for p in SNAPSHOTS.glob('snapshot_*.xml')}}
    STATE.write_text(json.dumps(state, indent=2) + '\n')
    for row in artifacts:
        shutil.copy2(row['source'], row['path'])
    config.write_text(text.replace('osgi.bundles=', 'osgi.bundles=' + provider, 1))
    with args.open('a') as stream:
        stream.write('\n"-Dkura.networking.provider=netplan"\n')
        stream.write('"-Dkura.configuration.external.pids=org.eclipse.kura.net.admin.NetworkConfigurationService"\n')
    for row in originals:
        row['preparedSha256'] = digest(Path(row['path']))
    state['status'] = 'prepared'
    STATE.write_text(json.dumps(state, indent=2) + '\n')
    print(json.dumps({'status': 'prepared', 'artifacts': len(artifacts), 'runtime': str(RUNTIME)}))


def restore():
    stopped()
    state = json.loads(STATE.read_text())
    if state['status'] != 'prepared':
        raise RuntimeError('Only the exact prepared/stopped profile can be restored')
    if state.get('probe'):
        probe_result = json.loads(PROBE_OUTPUT.read_text())
        if probe_result.get('status') != 'restored' or not probe_result.get('restPolicyRestored'):
            raise RuntimeError('The runtime probe has not verified restoration of the temporary REST policy')
        if digest(Path(state['probe'])) != state['probeSha256']:
            raise RuntimeError('Runtime probe changed externally')
    for row in state['originals'] + state['artifacts']:
        if digest(Path(row['path'])) != row['preparedSha256'] or digest(Path(row['backup'])) != row['sha256']:
            raise RuntimeError('Profile/artifact changed externally; preserve it for review')
    for path, value in state['snapshots'].items():
        if digest(Path(path)) != value:
            raise RuntimeError('Original snapshot changed; refusing overwrite')
    created = BACKUP / 'created-snapshots'
    for path in SNAPSHOTS.glob('snapshot_*.xml'):
        if str(path) not in state['snapshots']:
            created.mkdir(exist_ok=True)
            shutil.move(path, created / path.name)
    reviewed = BACKUP / 'reviewed-artifacts'
    reviewed.mkdir()
    for row in state['artifacts']:
        shutil.copy2(row['path'], reviewed / Path(row['path']).name)
    if state.get('probe'):
        shutil.move(state['probe'], reviewed / Path(state['probe']).name)
    for row in state['originals'] + state['artifacts']:
        shutil.copy2(row['backup'], row['path'])
        if digest(Path(row['path'])) != row['sha256']:
            raise RuntimeError('Restore verification failed')
    state['status'] = 'restored'
    STATE.write_text(json.dumps(state, indent=2) + '\n')
    print(json.dumps({'status': 'restored', 'originalArtifactsAndSnapshotsVerified': True}))


def attach_probe():
    stopped()
    state = json.loads(STATE.read_text())
    if state['status'] != 'prepared' or state.get('probe') or PROBE_OUTPUT.exists() or PROBE_FINISH.exists():
        raise RuntimeError('Attach only once to the stopped, prepared profile')
    for row in state['originals'] + state['artifacts']:
        if digest(Path(row['path'])) != row['preparedSha256']:
            raise RuntimeError('Prepared profile changed externally')
    source = ROOT.parent / ('kura-networking/bundles/org.eclipse.kura.rest.network.configuration.provider/'
                            'target/test-classes/org/eclipse/kura/rest/network/configuration/provider/probe/'
                            'NetworkRestRuntimeProbe.class')
    probe = RUNTIME / 'plugins/network-rest-runtime-probe.jar'
    if not source.is_file() or probe.exists():
        raise RuntimeError('Build the test probe first; do not replace an existing runtime artifact')
    manifest = ('Manifest-Version: 1.0\nBundle-ManifestVersion: 2\n'
                'Bundle-SymbolicName: yofc.test.network.rest.probe\nBundle-Version: 1.0.0\n'
                'Bundle-Activator: org.eclipse.kura.rest.network.configuration.provider.probe.NetworkRestRuntimeProbe\n'
                'Import-Package: org.osgi.framework,org.eclipse.kura,org.eclipse.kura.configuration,\n'
                ' org.eclipse.kura.net.transaction,com.google.gson\n\n')
    with zipfile.ZipFile(probe, 'w') as jar:
        jar.writestr('META-INF/MANIFEST.MF', manifest)
        jar.write(source, 'org/eclipse/kura/rest/network/configuration/provider/probe/NetworkRestRuntimeProbe.class')
    config = RUNTIME / 'configuration/config.ini'
    config.write_text(config.read_text().replace('osgi.bundles=',
                      'osgi.bundles=reference:file:plugins/network-rest-runtime-probe.jar@5:start,', 1))
    with (RUNTIME / 'jvm.args').open('a') as stream:
        stream.write(f'\n"-Dkura.network.rest.probe.output={PROBE_OUTPUT}"\n')
        stream.write(f'"-Dkura.network.rest.probe.finish={PROBE_FINISH}"\n')
        stream.write(f'"-Dkura.network.rest.probe.home={DEV / "kura-home"}"\n')
    for row in state['originals']:
        row['preparedSha256'] = digest(Path(row['path']))
    state.update(probe=str(probe), probeSha256=digest(probe))
    STATE.write_text(json.dumps(state, indent=2) + '\n')
    print(json.dumps({'status': 'prepared', 'testOnlyLoopbackProbeAttached': True}))


def finish_probe():
    result = json.loads(PROBE_OUTPUT.read_text())
    if result.get('status') == 'ready':
        PROBE_FINISH.write_text('finish\n')
        print(json.dumps({'status': 'finish-requested'}))
    elif result.get('status') == 'restored':
        print(json.dumps(result))
    else:
        raise RuntimeError('Probe did not reach the ready state; inspect its failure before restoring')


def refresh_probe():
    """Archive a failed test attempt, then replace only its checked test JAR while stopped."""
    stopped()
    state = json.loads(STATE.read_text())
    if state['status'] != 'prepared' or not state.get('probe') or PROBE_FINISH.exists():
        raise RuntimeError('Refresh requires an attached, stopped, unfinished profile')
    result = json.loads(PROBE_OUTPUT.read_text())
    if result.get('status') != 'failed':
        raise RuntimeError('Only a failed attempt can be refreshed')
    for row in state['originals'] + state['artifacts']:
        if digest(Path(row['path'])) != row['preparedSha256']:
            raise RuntimeError('Prepared profile changed externally')
    probe = Path(state['probe'])
    if digest(probe) != state['probeSha256']:
        raise RuntimeError('Probe changed externally')
    source = ROOT.parent / ('kura-networking/bundles/org.eclipse.kura.rest.network.configuration.provider/'
                            'target/test-classes/org/eclipse/kura/rest/network/configuration/provider/probe/'
                            'NetworkRestRuntimeProbe.class')
    if not source.is_file():
        raise RuntimeError('Compile the revised test probe first')
    attempt = len(state.get('failedAttempts', [])) + 1
    archive = BACKUP / f'failed-probe-{attempt}'
    archive.mkdir()
    shutil.copy2(probe, archive / probe.name)
    shutil.move(PROBE_OUTPUT, archive / PROBE_OUTPUT.name)
    with zipfile.ZipFile(probe) as old:
        manifest = old.read('META-INF/MANIFEST.MF')
    with zipfile.ZipFile(probe, 'w') as jar:
        jar.writestr('META-INF/MANIFEST.MF', manifest)
        jar.write(source, 'org/eclipse/kura/rest/network/configuration/provider/probe/NetworkRestRuntimeProbe.class')
    state.setdefault('failedAttempts', []).append({'archive': str(archive), 'probeSha256': state['probeSha256'],
                                                 'result': result})
    state['probeSha256'] = digest(probe)
    STATE.write_text(json.dumps(state, indent=2) + '\n')
    print(json.dumps({'status': 'prepared', 'failedAttemptArchived': attempt, 'probeSha256': state['probeSha256']}))


if __name__ == '__main__':
    action = sys.argv[1] if len(sys.argv) == 2 else ''
    if action == 'prepare':
        prepare()
    elif action == 'restore':
        restore()
    elif action == 'attach-probe':
        attach_probe()
    elif action == 'finish-probe':
        finish_probe()
    elif action == 'refresh-probe':
        refresh_probe()
    elif action == 'status':
        print(STATE.read_text() if STATE.exists() else '{"status":"not-prepared"}')
    else:
        raise SystemExit('usage: phase2-network-rest-runtime.py prepare|attach-probe|refresh-probe|finish-probe|restore|status')
