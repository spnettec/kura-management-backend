#!/usr/bin/env python3
"""Isolated real-JVM fixture for the existing disposable OrbStack dummy links.

Not a package installer. Retains data and artifacts; restore only reverses the
explicit agent delegation and socket group introduced by this fixture.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import pwd
import shutil
import socket
import struct
import subprocess
import time
import urllib.request
import uuid

ROOT = Path(__file__).resolve().parents[1]
VM = 'yofc-management-network-20261101'
DEST = Path('/opt/yofc-kura-netplan-profile')
MODULE = DEST / 'kura-dev-runtime'
RUNTIME = MODULE / 'target/runtime-netplan'
HOME = Path('/var/lib/yofc-kura-netplan-profile')
STATE = DEST / 'state.json'
ACCOUNT = 'yofc-kura-profile'
UNIT = 'yofc-kura-netplan-profile.service'
AGENT_UNIT = 'yofc-management-phase2-agent.service'
ENV = Path('/etc/yofc-management-phase2/agent.env')
DROPIN = Path('/etc/systemd/system') / (AGENT_UNIT + '.d') / 'kura-netplan-profile.conf'
SOCKET = '/run/yofc-management-phase2/control.sock'


def run(*args):
    return subprocess.run(args, check=True, capture_output=True, text=True).stdout


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def save(path, value):
    with path.open('x') as stream:
        stream.write(json.dumps(value, ensure_ascii=False, indent=2) + '\n')


def admin(method):
    request_id = str(uuid.uuid4())
    frame = json.dumps({'version': 1, 'kind': 'request', 'requestId': request_id, 'method': method, 'params': {}}).encode()
    with socket.socket(socket.AF_UNIX) as stream:
        stream.settimeout(30)
        stream.connect(SOCKET)
        stream.sendall(struct.pack('>I', len(frame)) + frame)

        def read(size):
            value = b''
            while len(value) < size:
                chunk = stream.recv(size - len(value))
                if not chunk:
                    raise EOFError('local peer closed')
                value += chunk
            return value

        size = struct.unpack('>I', read(4))[0]
        assert size <= 1024 * 1024
        reply = json.loads(read(size))
        assert reply['requestId'] == request_id and reply['method'] == method
        assert not reply.get('error'), reply.get('error')
        return reply['result']


def wait_agent(available):
    deadline = time.monotonic() + 40
    while time.monotonic() < deadline:
        try:
            result = admin('agent.status')
            if result['kuraAvailable'] == available:
                return result
        except (OSError, EOFError, AssertionError):
            pass
        time.sleep(.2)
    raise RuntimeError('agent did not reach the expected bridge state')


def network_baseline():
    return {'eth0SourceSha256': sha('/etc/netplan/10-lxc.yaml'),
            'managedSourceSha256': sha('/etc/netplan/90-yofc-managed.yaml'),
            'defaultRoute': json.loads(run('ip', '-j', '-4', 'route', 'show', 'default')),
            'dummyAddress': json.loads(run('ip', '-j', '-4', 'address', 'show', 'dev', 'yofc-test0'))[0]['addr_info'],
            'dummyMtu': json.loads(run('ip', '-j', 'link', 'show', 'dev', 'yofc-test0'))[0]['mtu']}


def prepare():
    assert not STATE.exists() and not DEST.exists() and not HOME.exists(), 'inspect the retained fixture; do not replay prepare'
    assert not DROPIN.exists()
    assert '24.04' in Path('/etc/os-release').read_text()
    assert not shutil.which('java') is None, 'install the official JRE before preparing this fixture'
    assert admin('agent.status')['kuraAvailable'] is False
    network = admin('network.inspect')
    assert not network.get('transaction') or network['transaction']['state'] not in ('PREPARING', 'APPLYING', 'WAIT_CONFIRM', 'ROLLING_BACK')
    original = ENV.read_bytes()
    assert b'AGENT_KURA_UID=65534\n' in original
    assert b'AGENT_KURA_NETWORK_INTERFACES=' not in original
    original_network = network_baseline()
    source_module = ROOT.parent / 'kura/kura-dev-runtime'
    source_runtime = source_module / 'target/runtime-netplan'
    for name in ['runtime.json', 'network-providers.json']:
        assert (source_module / name).is_file(), name
    inventory = json.loads((source_runtime / 'inventory.json').read_text())
    for entry in inventory:
        assert sha(source_runtime / entry['path']) == entry['sha256'], entry['symbolicName']
    try:
        pwd.getpwnam(ACCOUNT)
    except KeyError:
        run('useradd', '--system', '--user-group', '--no-create-home', '--home-dir', str(HOME),
            '--shell', '/usr/sbin/nologin', ACCOUNT)
    account = pwd.getpwnam(ACCOUNT)
    assert account.pw_uid != 0
    DEST.mkdir(mode=0o755)
    backup = DEST / 'agent-original.env'
    backup.touch(mode=0o600)
    backup.write_bytes(original)
    state = {'vm': VM, 'status': 'preparing', 'uid': account.pw_uid, 'gid': account.pw_gid,
             'originalAgentEnvSha256': sha(ENV), 'networkBefore': original_network,
             'baselineRevision': network['revision'], 'developmentRuntime': True,
             'productionPackageValidated': False}
    save(STATE, state)
    STATE.chmod(0o600)
    (RUNTIME / 'configuration').mkdir(parents=True)
    for name in ['plugins']:
        shutil.copytree(source_runtime / name, RUNTIME / name)
    for name in ['launcher.jar', 'framework.jar', 'inventory.json', 'profile.json']:
        shutil.copy2(source_runtime / name, RUNTIME / name)
    shutil.copy2(source_runtime / 'configuration/config.ini', RUNTIME / 'configuration/config.ini')
    shutil.copytree(source_module / 'tools', MODULE / 'tools', ignore=shutil.ignore_patterns('__pycache__'))
    shutil.copytree(source_module / 'src/main/resources', MODULE / 'src/main/resources')
    for name in ['runtime.json', 'network-providers.json']:
        shutil.copy2(source_module / name, MODULE / name)
    return start_fixture(state)


def resume():
    # The first preparation stopped before delegation because its manifest name
    # was wrong. Resume only that exact retained state, never reinitialize data.
    state = json.loads(STATE.read_text())
    assert state['status'] == 'preparing' and not HOME.exists() and not DROPIN.exists()
    assert not (Path('/etc/systemd/system') / UNIT).exists()
    assert sha(ENV) == state['originalAgentEnvSha256']
    assert network_baseline() == state['networkBefore']
    inventory = json.loads((RUNTIME / 'inventory.json').read_text())
    for entry in inventory:
        assert sha(RUNTIME / entry['path']) == entry['sha256'], entry['symbolicName']
    for name in ['runtime.json', 'network-providers.json']:
        assert not (MODULE / name).exists()
        shutil.copy2(ROOT.parent / 'kura/kura-dev-runtime' / name, MODULE / name)
    return start_fixture(state)


def start_fixture(state):
    account = pwd.getpwnam(ACCOUNT)
    original = (DEST / 'agent-original.env').read_bytes()
    HOME.mkdir(mode=0o700)
    for directory in [MODULE, HOME]:
        run('chown', '-R', f'{ACCOUNT}:{ACCOUNT}', str(directory))
    run('runuser', '-u', ACCOUNT, '--', 'env', f'KURA_DEV_HOME={HOME}', 'KURA_HTTP_PORT=19081',
        'KURA_HTTPS_PORT=19447', 'KURA_CLIENT_AUTH_PORT=19448', f'KURA_MANAGEMENT_SOCKET={SOCKET}',
        'KURA_MANAGEMENT_AGENT_USER=root', 'python3', str(MODULE / 'tools/runtime.py'), 'prepare',
        '--profile', 'linux', '--network-provider', 'netplan')
    assert '/Users/' not in (RUNTIME / 'jvm.args').read_text()
    state['originalSnapshotSha256'] = sha(HOME / 'user/snapshots/snapshot_0.xml')
    state['bundleInventorySha256'] = sha(RUNTIME / 'inventory.json')
    unit = (f'[Unit]\nDescription=Isolated real Kura JVM Netplan development profile\nAfter={AGENT_UNIT}\n'
            f'[Service]\nType=simple\nUser={ACCOUNT}\nGroup={ACCOUNT}\nUMask=0077\n'
            f'WorkingDirectory={RUNTIME}\nExecStart=/usr/bin/java @{RUNTIME}/jvm.args -jar {RUNTIME}/launcher.jar '
            f'-configuration {RUNTIME}/configuration -consoleLog\nTimeoutStopSec=30\n')
    unit_path = Path('/etc/systemd/system') / UNIT
    assert not unit_path.exists()
    unit_path.write_text(unit)
    DROPIN.parent.mkdir(exist_ok=True)
    DROPIN.write_text(f'[Service]\nGroup={ACCOUNT}\nRuntimeDirectoryMode=0750\n')
    delegated = original.decode().replace('AGENT_KURA_UID=65534', f'AGENT_KURA_UID={account.pw_uid}')
    ENV.write_text(delegated + '\nAGENT_KURA_NETWORK_INTERFACES=yofc-test0\n')
    state['delegatedAgentEnvSha256'] = sha(ENV)
    state['dropinSha256'] = sha(DROPIN)
    STATE.write_text(json.dumps(state, indent=2) + '\n')
    run('systemctl', 'daemon-reload')
    run('systemctl', 'restart', AGENT_UNIT)
    wait_agent(False)
    run('systemctl', 'start', UNIT)
    wait_agent(True)
    deadline = time.monotonic() + 45
    while time.monotonic() < deadline:
        try:
            with urllib.request.urlopen('http://127.0.0.1:19081/admin/auth', timeout=2) as response:
                if response.status == 200:
                    break
        except OSError:
            pass
        time.sleep(.2)
    else:
        raise RuntimeError('real Kura console did not become ready')
    state['status'] = 'ready'
    STATE.write_text(json.dumps(state, indent=2) + '\n')
    return status()


def status():
    state = json.loads(STATE.read_text())
    pid = int(run('systemctl', 'show', UNIT, '--property=MainPID', '--value').strip())
    socket_stat = Path(SOCKET).stat()
    return {'status': state['status'], 'vm': VM, 'kuraPid': pid,
            'kuraUid': int(Path(f'/proc/{pid}/status').read_text().split('Uid:\t')[1].split()[0]) if pid else None,
            'agentStatus': admin('agent.status'), 'networkNow': network_baseline(),
            'networkBefore': state['networkBefore'], 'baselineRevision': state['baselineRevision'],
            'socketUid': socket_stat.st_uid, 'socketGid': socket_stat.st_gid,
            'socketMode': oct(socket_stat.st_mode & 0o777),
            'originalSnapshotPreserved': sha(HOME / 'user/snapshots/snapshot_0.xml') == state['originalSnapshotSha256'],
            'developmentRuntime': True, 'productionPackageValidated': False}


def update_web2():
    """Replace one verified development bundle while the JVM is stopped."""
    state = json.loads(STATE.read_text())
    assert state['status'] == 'ready'
    assert network_baseline() == state['networkBefore']
    transaction = admin('network.inspect').get('transaction')
    assert not transaction or transaction['state'] in ('COMMITTED', 'ROLLED_BACK', 'FAILED')
    source = ROOT.parent / 'kura/kura-dev-runtime/target/runtime-netplan'
    inventory_path = RUNTIME / 'inventory.json'
    inventory = json.loads(inventory_path.read_text())
    incoming = json.loads((source / 'inventory.json').read_text())
    entry = next(item for item in inventory if item['symbolicName'] == 'org.eclipse.kura.web2')
    replacement = next(item for item in incoming if item['symbolicName'] == 'org.eclipse.kura.web2')
    assert entry['path'] == replacement['path'] and entry['sha256'] != replacement['sha256']
    assert sha(inventory_path) == state['bundleInventorySha256']
    for item in inventory:
        assert sha(RUNTIME / item['path']) == item['sha256'], item['symbolicName']
    assert sha(source / replacement['path']) == replacement['sha256']
    backup = DEST / ('web2-before-' + entry['sha256'])
    assert not backup.exists(), 'inspect a previous bundle replacement before replaying'
    run('systemctl', 'stop', UNIT)
    assert int(run('systemctl', 'show', UNIT, '--property=MainPID', '--value')) == 0
    wait_agent(False)
    backup.mkdir(mode=0o700)
    shutil.copy2(inventory_path, backup / 'inventory.json')
    shutil.copy2(RUNTIME / entry['path'], backup / Path(entry['path']).name)
    previous = dict(entry)
    shutil.copy2(source / replacement['path'], RUNTIME / entry['path'])
    entry.update(replacement)
    inventory_path.write_text(json.dumps(inventory, indent=2) + '\n')
    state['bundleInventorySha256'] = sha(inventory_path)
    state.setdefault('web2Updates', []).append({'previous': previous, 'replacement': replacement,
                                               'backup': str(backup)})
    STATE.write_text(json.dumps(state, indent=2) + '\n')
    run('systemctl', 'start', UNIT)
    wait_agent(True)
    result = status()
    result['web2Update'] = state['web2Updates'][-1]
    assert result['originalSnapshotPreserved'] and result['networkNow'] == state['networkBefore']
    return result


def observe_network():
    """Capture actual dummy-link state and the agent's confirmed configuration."""
    result = status()
    assert result['status'] == 'ready'
    network = admin('network.inspect')
    transaction = network.get('transaction')
    result['network'] = {key: network[key] for key in ('revision', 'sourceDigest', 'confirmedYaml')}
    result['network']['transaction'] = ({key: transaction.get(key) for key in
                                          ('id', 'state', 'revision', 'deadlineUnixMs', 'diagnostic')}
                                         if transaction else None)
    result['network']['observedInterface'] = next(item for item in network['interfaces']
                                                if item['name'] == 'yofc-test0')
    result['networkWritesByObserver'] = 0
    return result


def restore():
    state = json.loads(STATE.read_text())
    assert state['status'] == 'ready', 'inspect a failed/incomplete fixture before restoring'
    assert sha(ENV) == state['delegatedAgentEnvSha256'] and sha(DROPIN) == state['dropinSha256'], 'environment changed externally'
    assert network_baseline() == state['networkBefore'], 'restore dummy network before releasing its delegation'
    transaction = admin('network.inspect').get('transaction')
    assert not transaction or transaction['state'] in ('COMMITTED', 'ROLLED_BACK', 'FAILED'), 'settle the network transaction first'
    run('systemctl', 'stop', UNIT)
    wait_agent(False)
    assert sha(DEST / 'agent-original.env') == state['originalAgentEnvSha256']
    run('systemctl', 'stop', AGENT_UNIT)
    ENV.write_bytes((DEST / 'agent-original.env').read_bytes())
    DROPIN.rename(DEST / 'agent-socket-group-restored.conf')
    run('systemctl', 'daemon-reload')
    run('systemctl', 'start', AGENT_UNIT)
    wait_agent(False)
    state['status'] = 'restored'
    STATE.write_text(json.dumps(state, indent=2) + '\n')
    result = status()
    result['agentEnvironmentRestored'] = sha(ENV) == state['originalAgentEnvSha256']
    assert result['kuraPid'] == 0 and result['socketGid'] == 0 and result['agentEnvironmentRestored']
    assert result['networkNow'] == state['networkBefore'] and result['originalSnapshotPreserved']
    return result


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action', choices=['prepare', 'resume', 'status', 'update-web2', 'observe-network', 'restore'])
    parser.add_argument('--vm', choices=[VM], required=True)
    parser.add_argument('--inside-fixture', action='store_true')
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    if args.inside_fixture:
        assert os.geteuid() == 0
        assert not args.output.exists(), 'retain existing evidence'
        result = globals()[args.action.replace('-', '_')]()
        save(args.output, result)
        print(json.dumps(result, ensure_ascii=False, indent=2))
    else:
        subprocess.run(['orb', 'run', '-m', VM, '-u', 'root', 'python3', '/mnt/mac' + str(Path(__file__).resolve()),
                        args.action, '--vm', VM, '--inside-fixture', '--output', str(args.output)], check=True)
