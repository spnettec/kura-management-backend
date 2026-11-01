#!/usr/bin/env python3
"""Stage a new one-use complete IDEA runtime for real configuration CAS/snapshot proof."""
import argparse
import getpass
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import zipfile
from xml.etree import ElementTree as E

ROOT = Path(__file__).resolve().parents[1]
WORKSPACE = ROOT.parent
DEV = ROOT / '.phase1-dev'
FIXTURE = DEV / 'configuration-revision-final-20261101'
RUNTIME = FIXTURE / 'runtime'
DATA = FIXTURE / 'kura-home'
STATE = FIXTURE / 'state.json'
SOURCE = DEV / 'kura-runtime'
CORE = WORKSPACE / 'kura/kura/org.eclipse.kura.core.configuration'
CLASS = 'org/eclipse/kura/core/configuration/probe/ConfigurationRevisionRuntimeProbe'
PROBE = 'yofc-configuration-revision-probe.jar'
ARTIFACTS = {
    'org.eclipse.kura.api': WORKSPACE / 'kura/kura/org.eclipse.kura.api/target/org.eclipse.kura.api-3.0.0-SNAPSHOT.jar',
    'org.eclipse.kura.core.configuration': CORE / 'target/org.eclipse.kura.core.configuration-3.0.0-SNAPSHOT.jar',
    'org.eclipse.kura.management.bridge': WORKSPACE / 'kura-networking/bundles/org.eclipse.kura.management.bridge/target/org.eclipse.kura.management.bridge-1.0.0-SNAPSHOT.jar',
    'org.eclipse.kura.container.provider': WORKSPACE / 'kura-container/bundles/org.eclipse.kura.container.provider/target/org.eclipse.kura.container.provider-3.0.0-SNAPSHOT.jar',
}


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def stopped():
    processes = subprocess.check_output(['ps', '-axo', 'pid,command'], text=True)
    for line in processes.splitlines():
        if '/bin/java ' in line and any(str(p / 'jvm.args') in line for p in [RUNTIME, SOURCE]):
            raise SystemExit('Stop the isolated IDEA JVM before staging/reconfiguring it')


def manifest(headers):
    lines = []
    for key, value in headers.items():
        remaining = f'{key}: {value}'
        while len(remaining.encode()) > 70:
            lines.append(remaining[:70]); remaining = ' ' + remaining[70:]
        lines.append(remaining)
    return ('\r\n'.join(lines) + '\r\n\r\n').encode()


def save(state):
    STATE.write_text(json.dumps(state, indent=2) + '\n'); STATE.chmod(0o600)


def prepare():
    stopped()
    assert not FIXTURE.exists(), 'Retained fixture exists; do not replay preparation'
    inventory = json.loads((SOURCE / 'inventory.json').read_text())
    for item in inventory:
        assert sha(SOURCE / item['path']) == item['sha256'], item['path']
    FIXTURE.mkdir(mode=0o700)
    (FIXTURE / '.configuration-revision-owned').write_text('test-only\n')
    RUNTIME.mkdir()
    shutil.copytree(SOURCE / 'plugins', RUNTIME / 'plugins')
    for name in ['launcher.jar', 'framework.jar']:
        shutil.copy2(SOURCE / name, RUNTIME / name)
    (RUNTIME / 'configuration').mkdir()
    shutil.copy2(SOURCE / 'configuration/config.ini', RUNTIME / 'configuration/config.ini')
    state = {'phase': 'preparing', 'fixtureOnly': True, 'sourceInventorySha256': sha(SOURCE / 'inventory.json'),
             'sourceRuntimeUnchanged': True, 'artifacts': {}}
    save(state)
    for symbolic_name, artifact in ARTIFACTS.items():
        entry = next(item for item in inventory if item['symbolicName'] == symbolic_name)
        shutil.copy2(artifact, RUNTIME / entry['path'])
        with zipfile.ZipFile(artifact) as archive:
            assert not any(CLASS in name for name in archive.namelist()), 'test probe in product jar'
            text = archive.read('META-INF/MANIFEST.MF').decode().replace('\r\n ', '')
        version = next(line.split(': ', 1)[1] for line in text.splitlines() if line.startswith('Bundle-Version: '))
        entry.update(sha256=sha(artifact), bundleVersion=version, source='configuration-revision-reviewed-local')
        state['artifacts'][symbolic_name] = {'path': entry['path'], 'sha256': sha(artifact)}
    compiled = CORE / 'target/test-classes'
    with zipfile.ZipFile(RUNTIME / 'plugins' / PROBE, 'x') as archive:
        archive.writestr('META-INF/MANIFEST.MF', manifest({
            'Manifest-Version': '1.0', 'Bundle-ManifestVersion': '2',
            'Bundle-SymbolicName': 'org.eclipse.kura.management.revision.probe', 'Bundle-Version': '1.0.0',
            'Bundle-Activator': CLASS.replace('/', '.'), 'Service-Component': 'OSGI-INF/fixture.xml',
            'Import-Package': 'org.eclipse.kura,org.eclipse.kura.configuration;version="[1.4,2)",org.osgi.framework,org.osgi.service.cm',
        }))
        archive.writestr('OSGI-INF/', b'')
        archive.writestr('OSGI-INF/metatype/', b'')
        for path in sorted(compiled.glob(CLASS + '*.class')):
            source = CORE / 'src/test/java' / (CLASS + '.java')
            assert path.stat().st_mtime_ns >= source.stat().st_mtime_ns, 'compile current probe first'
            archive.write(path, str(path.relative_to(compiled)))
        resources = CORE / 'src/test/resources/configuration-revision'
        archive.write(resources / 'fixture.xml', 'OSGI-INF/fixture.xml')
        archive.write(resources / 'metatype.xml', 'OSGI-INF/metatype/org.eclipse.kura.management.revisionFixture.xml')
    probe_entry = {'symbolicName': 'org.eclipse.kura.management.revision.probe', 'bundleVersion': '1.0.0',
                   'path': 'plugins/' + PROBE, 'startLevel': 5, 'start': True,
                   'source': 'test-only', 'sha256': sha(RUNTIME / 'plugins' / PROBE)}
    inventory.append(probe_entry)
    (RUNTIME / 'inventory.json').write_text(json.dumps(inventory, indent=2) + '\n')
    config = RUNTIME / 'configuration/config.ini'
    config.write_text(config.read_text().replace('osgi.bundles=', f'osgi.bundles=reference:file:plugins/{PROBE}@5:start,', 1))
    state.update(inventorySha256=sha(RUNTIME / 'inventory.json'), probeSha256=probe_entry['sha256'])
    save(state)
    return prepare_runtime(state)


def prepare_runtime(state):
    sys.path.insert(0, str(WORKSPACE / 'kura/kura-dev-runtime/tools'))
    spec = importlib.util.spec_from_file_location('revision_runtime', WORKSPACE / 'kura/kura-dev-runtime/tools/runtime.py')
    module = importlib.util.module_from_spec(spec); spec.loader.exec_module(module); module.RUNTIME = RUNTIME
    os.environ.update(KURA_DEV_HOME=str(DATA), KURA_HTTP_PORT='18280', KURA_HTTPS_PORT='18645', KURA_CLIENT_AUTH_PORT='18646')
    module.prepare('macos')
    properties = RUNTIME / 'configuration/kura.properties'
    with properties.open('a') as out:
        out.write('\nkura.need.password.change=false\n')
    args = RUNTIME / 'jvm.args'
    with args.open('a') as out:
        out.write(f'"-Dkura.management.socket={DEV / "run/control.sock"}"\n')
        out.write(f'"-Dkura.management.agent.user={getpass.getuser()}"\n')
        out.write(f'"-Dkura.revision.probe.root={FIXTURE}"\n"-Dkura.revision.probe.phase=write"\n')
    component = E.Element('component', name='ProjectRunConfigurationManager')
    configuration = E.SubElement(component, 'configuration', default='false', name='Kura configuration revision isolated',
                                 type='JarApplication', factoryName='JAR Application')
    for name, value in [('JAR_PATH', str(RUNTIME / 'launcher.jar')), ('VM_PARAMETERS', '@"' + str(args) + '"'),
                        ('PROGRAM_PARAMETERS', '-configuration "' + str(RUNTIME / 'configuration') + '" -consoleLog -console'),
                        ('WORKING_DIRECTORY', str(RUNTIME)), ('ALTERNATIVE_JRE_PATH', '21'), ('ALTERNATIVE_JRE_PATH_ENABLED', 'true')]:
        E.SubElement(configuration, 'option', name=name, value=value)
    E.SubElement(configuration, 'method', v='2')
    destination = WORKSPACE / 'kura/.idea/runConfigurations/Kura_configuration_revision_isolated.xml'
    E.ElementTree(component).write(destination, encoding='unicode')
    state.update(phase='prepared-write', runtime=str(RUNTIME), data=str(DATA), ideaConfiguration=str(destination),
                 inventorySha256=sha(RUNTIME / 'inventory.json'), probeSha256=sha(RUNTIME / 'plugins' / PROBE))
    save(state); return state


def resume_ports():
    # Exact pre-JVM failure: the previous HTTP port was still unavailable.
    # Retain staged files and seed only the still-absent private data directory.
    stopped(); state = json.loads(STATE.read_text())
    assert state['phase'] == 'preparing' and not DATA.exists() and not (RUNTIME / 'jvm.args').exists()
    inventory = json.loads((RUNTIME / 'inventory.json').read_text())
    for item in inventory:
        assert sha(RUNTIME / item['path']) == item['sha256']
    for name, artifact in ARTIFACTS.items():
        item = next(item for item in inventory if item['symbolicName'] == name)
        assert sha(artifact) == item['sha256']
    assert sha(SOURCE / 'inventory.json') == state['sourceInventorySha256']
    state['resumedPortFailure'] = True
    return prepare_runtime(state)


def change(action):
    stopped(); state = json.loads(STATE.read_text())
    assert sha(SOURCE / 'inventory.json') == state['sourceInventorySha256']
    assert sha(RUNTIME / 'inventory.json') == state['inventorySha256']
    if action == 'restart':
        assert state['phase'] == 'prepared-write'
        result = json.loads((FIXTURE / 'write-result.json').read_text()); assert result['status'] == 'passed'
        # Retain the write-phase artifacts before refreshing reviewed metadata.
        inventory = json.loads((RUNTIME / 'inventory.json').read_text())
        state['writePhaseArtifacts'] = {}
        backup = FIXTURE / 'before-restart-artifacts'; backup.mkdir()
        for name, artifact in ARTIFACTS.items():
            entry = next(item for item in inventory if item['symbolicName'] == name)
            path = RUNTIME / entry['path']
            state['writePhaseArtifacts'][name] = {'path': entry['path'], 'sha256': sha(path)}
            if sha(path) != sha(artifact):
                shutil.copy2(path, backup / path.name)
                shutil.copy2(artifact, path)
                entry['sha256'] = sha(path)
            state['artifacts'][name] = {'path': entry['path'], 'sha256': sha(path)}
        (RUNTIME / 'inventory.json').write_text(json.dumps(inventory, indent=2) + '\n')
        state['inventorySha256'] = sha(RUNTIME / 'inventory.json')
        args = RUNTIME / 'jvm.args'; text = args.read_text()
        assert text.count('-Dkura.revision.probe.phase=write') == 1
        args.write_text(text.replace('-Dkura.revision.probe.phase=write', '-Dkura.revision.probe.phase=restart'))
        state['writeResult'] = result; state['phase'] = 'prepared-restart'
    else:
        assert action == 'finish' and state['phase'] == 'prepared-restart'
        result = json.loads((FIXTURE / 'restart-result.json').read_text()); assert result['status'] == 'passed'
        state.update(phase='stopped-data-retained', restartResult=result)
    save(state); return state


if __name__ == '__main__':
    parser = argparse.ArgumentParser(); parser.add_argument('action', choices=['prepare', 'resume-ports', 'restart', 'finish', 'status'])
    action = parser.parse_args().action
    result = prepare() if action == 'prepare' else resume_ports() if action == 'resume-ports' else json.loads(STATE.read_text()) if action == 'status' else change(action)
    print(json.dumps(result, indent=2))
