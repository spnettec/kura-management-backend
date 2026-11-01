#!/usr/bin/env python3
"""Stage one new IDEA runtime for real configuration outcomes and explicit snapshots."""
import argparse
import getpass
import importlib.util
import json
import os
from pathlib import Path
import shutil
import signal
import subprocess
import sys
import tempfile
import time
import uuid
import zipfile
from xml.etree import ElementTree as E

ROOT = Path(__file__).resolve().parents[1]
WORKSPACE = ROOT.parent
FIXTURE = ROOT / '.phase1-dev/configuration-outcome-snapshot-idea-20261101'
RUNTIME = FIXTURE / 'runtime'
DATA = FIXTURE / 'kura-home'
STATE = FIXTURE / 'state.json'
SOURCE = ROOT / '.phase1-dev/configuration-revision-final-20261101/runtime'
GATEWAY = 'configuration-outcome-snapshot-idea-20261101'
PID = 'org.eclipse.kura.management.revisionFixture'
CORE = WORKSPACE / 'kura/kura/org.eclipse.kura.core.configuration'


def module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    value = importlib.util.module_from_spec(spec); spec.loader.exec_module(value); return value


dev = module('outcome_snapshot_dev', ROOT / 'scripts/phase1-dev.py')
revision = module('outcome_snapshot_source', ROOT / 'scripts/phase2-configuration-revision-runtime.py')
sha = revision.sha


def save(state):
    STATE.write_text(json.dumps(state, indent=2) + '\n'); STATE.chmod(0o600)


def console(*arguments):
    subprocess.run([str(ROOT / 'target/debug/yofc-management-console'), *map(str, arguments)],
                   check=True, env={**os.environ, **dev.env()}, stdout=subprocess.DEVNULL)


def java_pid():
    lines = subprocess.check_output(['ps', '-axo', 'pid,command'], text=True).splitlines()
    pids = [int(line.split()[0]) for line in lines if '/bin/java ' in line and str(RUNTIME / 'jvm.args') in line]
    assert len(pids) <= 1
    return pids[0] if pids else 0


def prepare():
    assert not FIXTURE.exists(), 'This one-use fixture already exists; never replay prepare'
    assert not java_pid()
    inventory = json.loads((SOURCE / 'inventory.json').read_text())
    for entry in inventory:
        assert sha(SOURCE / entry['path']) == entry['sha256'], entry['path']
    FIXTURE.mkdir(mode=0o700)
    (FIXTURE / '.outcome-snapshot-owned').write_text('new one-use fixture\n')
    (FIXTURE / 'stage').write_text('initial\n')
    state = {'phase': 'preparing', 'gateway': GATEWAY, 'sourceInventorySha256': sha(SOURCE / 'inventory.json'),
             'originalHostAgentPid': int((dev.DEV / 'agent.pid').read_text()), 'fixtureOnly': True,
             'networkWrites': 0, 'replays': 0}
    save(state)
    shutil.copytree(SOURCE, RUNTIME, ignore=shutil.ignore_patterns('configuration', 'jvm.args'))
    (RUNTIME / 'configuration').mkdir()
    shutil.copy2(SOURCE / 'configuration/config.ini', RUNTIME / 'configuration/config.ini')
    for symbolic_name in ['org.eclipse.kura.api', 'org.eclipse.kura.core.configuration',
                          'org.eclipse.kura.management.bridge', 'org.eclipse.kura.container.provider']:
        artifact = revision.ARTIFACTS[symbolic_name]
        assert artifact.exists()
        entry = next(item for item in inventory if item['symbolicName'] == symbolic_name)
        shutil.copy2(artifact, RUNTIME / entry['path'])
        entry['sha256'] = sha(RUNTIME / entry['path'])
        entry['source'] = 'outcome-snapshot-reviewed-local'
    probe = next(entry for entry in inventory if entry['symbolicName'] == 'org.eclipse.kura.management.revision.probe')
    compiled = CORE / 'target/test-classes'
    probe_class = 'org/eclipse/kura/core/configuration/probe/ConfigurationRemoteProbe'
    assert (compiled / (probe_class + '.class')).stat().st_mtime_ns >= \
           (CORE / 'src/test/java' / (probe_class + '.java')).stat().st_mtime_ns
    with zipfile.ZipFile(RUNTIME / probe['path'], 'w') as archive:
        archive.writestr('META-INF/MANIFEST.MF', revision.manifest({
            'Manifest-Version': '1.0', 'Bundle-ManifestVersion': '2', 'Bundle-SymbolicName': probe['symbolicName'],
            'Bundle-Version': '1.0.0', 'Bundle-Activator': probe_class.replace('/', '.'),
            'Service-Component': 'OSGI-INF/fixture.xml',
            'Import-Package': 'org.eclipse.kura.configuration;version="[1.4,2)",org.osgi.framework,org.osgi.service.cm'}))
        archive.writestr('OSGI-INF/', b''); archive.writestr('OSGI-INF/metatype/', b'')
        for name in [probe_class + '.class', revision.CLASS + '$Fixture.class']:
            archive.write(compiled / name, name)
        for source, dest in [('fixture.xml', 'OSGI-INF/fixture.xml'),
                             ('metatype.xml', 'OSGI-INF/metatype/' + PID + '.xml')]:
            archive.write(CORE / 'src/test/resources/configuration-revision' / source, dest)
    probe['sha256'] = sha(RUNTIME / probe['path']); probe['source'] = 'test-only-read-observer'
    (RUNTIME / 'inventory.json').write_text(json.dumps(inventory, indent=2) + '\n')
    sys.path.insert(0, str(WORKSPACE / 'kura/kura-dev-runtime/tools'))
    runtime = module('outcome_snapshot_runtime', WORKSPACE / 'kura/kura-dev-runtime/tools/runtime.py')
    runtime.RUNTIME = RUNTIME
    os.environ.update(KURA_DEV_HOME=str(DATA), KURA_HTTP_PORT='18381', KURA_HTTPS_PORT='18755',
                      KURA_CLIENT_AUTH_PORT='18756')
    runtime.prepare('macos')
    with (RUNTIME / 'configuration/kura.properties').open('a') as output:
        output.write('\nkura.need.password.change=false\n')
    socket_path = Path(tempfile.mkdtemp(prefix='yofc-outcome-snapshot-idea-')) / 'control.sock'
    args = RUNTIME / 'jvm.args'
    with args.open('a') as output:
        output.write(f'"-Dkura.management.socket={socket_path}"\n"-Dkura.management.agent.user={getpass.getuser()}"\n')
        output.write(f'"-Dkura.management.configuration.update.pids={PID}"\n')
        output.write('"-Dkura.management.snapshot.enabled=true"\n')
        output.write(f'"-Dkura.configuration.remote.probe.root={FIXTURE}"\n')
    component = E.Element('component', name='ProjectRunConfigurationManager')
    configuration = E.SubElement(component, 'configuration', default='false',
                                 name='Kura outcome snapshot isolated', type='JarApplication', factoryName='JAR Application')
    for key, value in [('JAR_PATH', str(RUNTIME / 'launcher.jar')), ('VM_PARAMETERS', '@"' + str(args) + '"'),
                       ('PROGRAM_PARAMETERS', '-configuration "' + str(RUNTIME / 'configuration') + '" -consoleLog -console'),
                       ('WORKING_DIRECTORY', str(RUNTIME)), ('ALTERNATIVE_JRE_PATH', '21'),
                       ('ALTERNATIVE_JRE_PATH_ENABLED', 'true')]:
        E.SubElement(configuration, 'option', name=key, value=value)
    E.SubElement(configuration, 'method', v='2')
    destination = WORKSPACE / 'kura/.idea/runConfigurations/Kura_outcome_snapshot_isolated.xml'
    E.ElementTree(component).write(destination, encoding='unicode')
    ext = FIXTURE / 'client.ext'
    ext.write_text('basicConstraints=critical,CA:FALSE\nkeyUsage=critical,digitalSignature\n'
                   'extendedKeyUsage=clientAuth\nsubjectKeyIdentifier=hash\nauthorityKeyIdentifier=keyid,issuer\n')
    for command in [
        ['openssl', 'req', '-newkey', 'rsa:2048', '-nodes', '-subj', '/CN=' + GATEWAY,
         '-keyout', str(FIXTURE / 'agent.key'), '-out', str(FIXTURE / 'agent.csr')],
        ['openssl', 'x509', '-req', '-in', str(FIXTURE / 'agent.csr'), '-CA', str(dev.DEV / 'ca.pem'),
         '-CAkey', str(dev.DEV / 'ca.key'), '-set_serial', '0x' + uuid.uuid4().hex, '-days', '3',
         '-extfile', str(ext), '-out', str(FIXTURE / 'agent.pem')],
    ]:
        subprocess.run(command, check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    (FIXTURE / 'agent.key').chmod(0o600)
    console('register-gateway', GATEWAY, 'dev', 'Outcome snapshot IDEA fixture', FIXTURE / 'agent.pem')
    environment = {**os.environ, **dev.env(), 'AGENT_GATEWAY_ID': GATEWAY,
                   'AGENT_CERT': str(FIXTURE / 'agent.pem'), 'AGENT_KEY': str(FIXTURE / 'agent.key'),
                   'AGENT_SOCKET': str(socket_path), 'AGENT_STATE_DB': str(FIXTURE / 'agent.sqlite'),
                   'AGENT_KURA_CONFIGURATION_UPDATE': '1', 'AGENT_KURA_SNAPSHOT': '1',
                   'AGENT_KURA_NETWORK_INTERFACES': '', 'AGENT_NETWORK_INTERFACES': ''}
    with (FIXTURE / 'agent.log').open('ab') as log:
        agent = subprocess.Popen([str(dev.AGENT / 'target/debug/yofc-management-agent')], env=environment,
                                 stdout=log, stderr=log, start_new_session=True)
    state.update(phase='prepared-awaiting-idea-start', agentPid=agent.pid, socketPath=str(socket_path),
                 inventorySha256=sha(RUNTIME / 'inventory.json'), ideaConfiguration=str(destination))
    save(state)
    return status()


def status():
    state = json.loads(STATE.read_text())
    assert sha(SOURCE / 'inventory.json') == state['sourceInventorySha256']
    inventory = json.loads((RUNTIME / 'inventory.json').read_text())
    for entry in inventory:
        assert sha(RUNTIME / entry['path']) == entry['sha256'], entry['path']
    assert int((dev.DEV / 'agent.pid').read_text()) == state['originalHostAgentPid']
    os.kill(state['originalHostAgentPid'], 0)
    return {'phase': state['phase'], 'gateway': GATEWAY, 'kuraPid': java_pid(), 'agentPid': state['agentPid'],
            'inventoryChecked': len(inventory), 'inventoryAllMatch': True,
            'originalHostAgentPreserved': True, 'networkWrites': 0, 'fixtureOnly': True}


def finish():
    assert not java_pid(), 'Stop this Kura in IDEA first'
    state = json.loads(STATE.read_text())
    assert state['phase'] == 'prepared'
    command = subprocess.check_output(['ps', '-p', str(state['agentPid']), '-o', 'command='], text=True).strip()
    assert command == str(dev.AGENT / 'target/debug/yofc-management-agent')
    os.kill(state['agentPid'], signal.SIGTERM)
    deadline = time.monotonic() + 5
    while subprocess.run(['ps', '-p', str(state['agentPid'])], capture_output=True).returncode == 0 \
            and time.monotonic() < deadline:
        time.sleep(.05)
    assert subprocess.run(['ps', '-p', str(state['agentPid'])], capture_output=True).returncode != 0
    console('set-gateway-enabled', GATEWAY, 'false')
    state['phase'] = 'stopped-data-retained'
    save(state)
    return status()


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('action', choices=['prepare', 'status', 'finish'])
    action = parser.parse_args().action
    print(json.dumps({'prepare': prepare, 'status': status, 'finish': finish}[action](), indent=2))
