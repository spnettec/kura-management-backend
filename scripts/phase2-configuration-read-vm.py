#!/usr/bin/env python3
"""One-use, read-only OrbStack fixture. No production installer or network writer.

Run only as root in yofc-management-network-20261101. Each action retains its
receipt; failed preparation must be inspected, never replayed over existing data.
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
import uuid

ROOT = Path(__file__).resolve().parents[1]
DEST = Path('/opt/yofc-config-read-20261101')
STATE = DEST / 'state.json'
MODULE = DEST / 'kura-dev-runtime'
RUNTIME = MODULE / 'target/runtime'
HOME = DEST / 'kura-home'
ACCOUNT = 'yofc-config-read'
AGENT = 'yofc-config-read-agent.service'
KURA = 'yofc-config-read-kura.service'
SOCKET = '/run/yofc-config-read/control.sock'
IDENTITY = ROOT / '.phase1-dev/configuration-read-20261101'
BRIDGE = ROOT.parent / 'kura-networking/bundles/org.eclipse.kura.management.bridge/target/org.eclipse.kura.management.bridge-1.0.0-SNAPSHOT.jar'
BINARY = ROOT / 'target/configuration-read-linux-agent-20261101'
BASELINE = [
    '/etc/yofc-management-phase2/agent.env',
    '/opt/yofc-kura-netplan-profile/state.json',
    '/opt/yofc-kura-netplan-package-20261101/state.json',
    '/opt/yofc-agent-package-20261101/state.json',
    '/etc/netplan/10-lxc.yaml', '/etc/netplan/90-yofc-managed.yaml',
]


def run(*args):
    return subprocess.run(args, check=True, capture_output=True, text=True).stdout


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def baseline():
    return {p: sha(p) for p in BASELINE}


def receipt(name, value):
    with (DEST / (name + '.json')).open('x') as stream:
        json.dump(value, stream, indent=2)
        stream.write('\n')


def admin(method):
    request_id = str(uuid.uuid4())
    data = json.dumps({'version': 1, 'kind': 'request', 'requestId': request_id,
                       'method': method, 'params': {}}).encode()
    with socket.socket(socket.AF_UNIX) as stream:
        stream.settimeout(5)
        stream.connect(SOCKET)
        stream.sendall(struct.pack('>I', len(data)) + data)

        def read(size):
            value = b''
            while len(value) < size:
                chunk = stream.recv(size - len(value))
                if not chunk:
                    raise EOFError('peer closed')
                value += chunk
            return value

        size = struct.unpack('>I', read(4))[0]
        assert size <= 1024 * 1024
        reply = json.loads(read(size))
        assert reply['requestId'] == request_id and not reply.get('error')
        return reply['result']


def wait_bridge(available):
    deadline = time.monotonic() + 45
    while time.monotonic() < deadline:
        try:
            if admin('agent.status')['kuraAvailable'] is available:
                return
        except (OSError, EOFError, AssertionError):
            pass
        time.sleep(.2)
    raise RuntimeError('expected bridge availability not reached')


def pid(unit):
    return int(run('systemctl', 'show', unit, '--property=MainPID', '--value'))


def status():
    state = json.loads(STATE.read_text())
    assert baseline() == state['baseline'], 'old fixture or network input changed'
    assert run('systemctl', 'is-active', 'yofc-management-phase2-agent.service').strip() == 'active'
    kura_pid = pid(KURA)
    result = {'phase': state['phase'], 'agentPid': pid(AGENT), 'kuraPid': kura_pid,
              'baselineUnchanged': True, 'oldAgentActive': True,
              'bridgeSha256': sha(RUNTIME / state['bridgePath']),
              'agentSha256': sha(DEST / 'agent'), 'networkWrites': 0,
              'developmentRuntime': True, 'packageAcceptance': False}
    if result['agentPid']:
        result['agentStatus'] = admin('agent.status')
        info = Path(SOCKET).stat()
        result['socketMode'] = oct(info.st_mode & 0o777)
        result['socketUid'] = info.st_uid
        result['socketGid'] = info.st_gid
    if kura_pid:
        proc = Path(f'/proc/{kura_pid}/status').read_text()
        result['kuraUid'] = int(proc.split('Uid:\t')[1].split()[0])
        result['kuraCapEff'] = proc.split('CapEff:\t')[1].split()[0]
    return result


def prepare():
    assert not DEST.exists(), 'retained fixture exists; do not replay prepare'
    assert '24.04' in Path('/etc/os-release').read_text()
    assert shutil.which('java') and BINARY.is_file() and BRIDGE.is_file()
    assert not Path(SOCKET).parent.exists()
    for unit in [AGENT, KURA]:
        assert not (Path('/etc/systemd/system') / unit).exists()
    source = ROOT.parent / 'kura/kura-dev-runtime'
    source_runtime = source / 'target/runtime'
    inventory = json.loads((source_runtime / 'inventory.json').read_text())
    for entry in inventory:
        assert sha(source_runtime / entry['path']) == entry['sha256'], entry['path']
    assert not any(e['symbolicName'] == 'org.eclipse.kura.management.bridge' for e in inventory)
    try:
        pwd.getpwnam(ACCOUNT)
    except KeyError:
        run('useradd', '--system', '--user-group', '--no-create-home', '--home-dir', str(HOME),
            '--shell', '/usr/sbin/nologin', ACCOUNT)
    account = pwd.getpwnam(ACCOUNT)
    assert account.pw_uid != 0
    DEST.mkdir(mode=0o755)
    DEST.chmod(0o755)
    (DEST / 'identity').mkdir(mode=0o700)
    (DEST / 'agent-state').mkdir(mode=0o700)
    state = {'phase': 'preparing', 'baseline': baseline(), 'uid': account.pw_uid,
             'gid': account.pw_gid, 'sourceInventorySha256': sha(source_runtime / 'inventory.json')}
    STATE.write_text(json.dumps(state, indent=2) + '\n')
    STATE.chmod(0o600)
    for name in ['ca.pem', 'agent.pem', 'agent.key']:
        shutil.copyfile(IDENTITY / name, DEST / 'identity' / name)
        (DEST / 'identity' / name).chmod(0o600)
    shutil.copyfile(BINARY, DEST / 'agent')
    (DEST / 'agent').chmod(0o755)
    return complete_prepare(state)


def complete_prepare(state, resume=False):
    source = ROOT.parent / 'kura/kura-dev-runtime'
    source_runtime = source / 'target/runtime'
    assert sha(source_runtime / 'inventory.json') == state['sourceInventorySha256']
    inventory = json.loads((source_runtime / 'inventory.json').read_text())
    account = pwd.getpwnam(ACCOUNT)
    (RUNTIME / 'configuration').mkdir(parents=True, exist_ok=True)
    if not resume:
        shutil.copytree(source_runtime / 'plugins', RUNTIME / 'plugins')
    else:
        for entry in inventory:
            assert sha(RUNTIME / entry['path']) == entry['sha256']
    for name in ['launcher.jar', 'framework.jar']:
        shutil.copy2(source_runtime / name, RUNTIME / name)
    shutil.copy2(source_runtime / 'configuration/config.ini', RUNTIME / 'configuration/config.ini')
    shutil.copytree(source / 'tools', MODULE / 'tools', ignore=shutil.ignore_patterns('__pycache__'))
    shutil.copytree(source / 'src/main/resources', MODULE / 'src/main/resources')
    for name in ['runtime.json', 'network-providers.json']:
        shutil.copy2(source / name, MODULE / name)
    bridge_path = 'plugins/org.eclipse.kura.management.bridge-1.0.0-SNAPSHOT.jar'
    shutil.copy2(BRIDGE, RUNTIME / bridge_path)
    inventory.append({'symbolicName': 'org.eclipse.kura.management.bridge', 'path': bridge_path,
                      'startLevel': 5, 'start': True, 'sha256': sha(BRIDGE),
                      'source': 'local-reviewed-configuration-read-artifact'})
    (RUNTIME / 'inventory.json').write_text(json.dumps(inventory, indent=2) + '\n')
    config_path = RUNTIME / 'configuration/config.ini'
    config = config_path.read_text()
    config = config.replace('osgi.bundles=', 'osgi.bundles=reference:file:' + bridge_path + '@5:start,', 1)
    config += '\norg.osgi.framework.system.packages.extra=jdk.net\n'
    config_path.write_text(config)
    HOME.mkdir(mode=0o700)
    run('chown', '-R', f'{ACCOUNT}:{ACCOUNT}', str(MODULE), str(HOME))
    return prepare_runtime_services(state)


def prepare_runtime_services(state):
    account = pwd.getpwnam(ACCOUNT)
    run('runuser', '-u', ACCOUNT, '--', 'env', f'KURA_DEV_HOME={HOME}', 'KURA_HTTP_PORT=19083',
        'KURA_HTTPS_PORT=19453', 'KURA_CLIENT_AUTH_PORT=19454', 'python3', str(MODULE / 'tools/runtime.py'),
        'prepare', '--profile', 'linux', '--network-provider', 'none')
    bridge_path = 'plugins/org.eclipse.kura.management.bridge-1.0.0-SNAPSHOT.jar'
    args_path = RUNTIME / 'jvm.args'
    options = args_path.read_text().replace('"-Xmx2g"', '"-Xmx768m"')
    options += f'"-Dkura.management.socket={SOCKET}"\n"-Dkura.management.agent.user=root"\n'
    assert '/Users/' not in options
    args_path.write_text(options)
    env = {'AGENT_CONSOLE_ENDPOINT': 'https://host.orb.internal:18444',
           'AGENT_GATEWAY_ID': 'config-read-gateway-20261101', 'AGENT_CA': str(DEST / 'identity/ca.pem'),
           'AGENT_CERT': str(DEST / 'identity/agent.pem'), 'AGENT_KEY': str(DEST / 'identity/agent.key'),
           'AGENT_STATE_DB': str(DEST / 'agent-state/results.sqlite'), 'AGENT_SOCKET': SOCKET,
           'AGENT_KURA_UID': str(account.pw_uid), 'AGENT_ADMIN_UID': '0',
           'AGENT_NETWORK_INTERFACES': '', 'AGENT_KURA_NETWORK_INTERFACES': '', 'RUST_LOG': 'info'}
    env_path = DEST / 'agent.env'
    env_path.write_text(''.join(k + '=' + v + '\n' for k, v in env.items()))
    env_path.chmod(0o600)
    agent_unit = (f'[Unit]\nDescription=Read-only configuration fixture agent\n[Service]\nType=simple\n'
                  f'User=root\nGroup={ACCOUNT}\nUMask=0077\nRuntimeDirectory=yofc-config-read\n'
                  f'RuntimeDirectoryMode=0750\nEnvironmentFile={env_path}\nExecStart={DEST}/agent\n'
                  'NoNewPrivileges=true\nCapabilityBoundingSet=\nTimeoutStopSec=20\n')
    kura_unit = (f'[Unit]\nDescription=Read-only configuration fixture Kura\nAfter={AGENT}\n[Service]\n'
                 f'Type=simple\nUser={ACCOUNT}\nGroup={ACCOUNT}\nUMask=0077\nWorkingDirectory={RUNTIME}\n'
                 f'ExecStart=/usr/bin/java @{args_path} -jar {RUNTIME}/launcher.jar '
                 f'-configuration {RUNTIME}/configuration -consoleLog\n'
                 'NoNewPrivileges=true\nCapabilityBoundingSet=\nTimeoutStopSec=30\n')
    for unit, content in [(AGENT, agent_unit), (KURA, kura_unit)]:
        (Path('/etc/systemd/system') / unit).write_text(content)
    state.update(phase='agent-only', bridgePath=bridge_path)
    STATE.write_text(json.dumps(state, indent=2) + '\n')
    run('systemctl', 'daemon-reload')
    run('systemctl', 'start', AGENT)
    wait_bridge(False)
    return status()



def resume_prepare():
    # Only the retained pre-service failure from a missing optional none-profile
    # descriptor is eligible. Never delete/reinitialize identity, state or runtime.
    state = json.loads(STATE.read_text())
    assert state['phase'] == 'preparing' and not HOME.exists()
    assert baseline() == state['baseline']
    assert not (RUNTIME / 'inventory.json').exists()
    for unit in [AGENT, KURA]:
        assert not (Path('/etc/systemd/system') / unit).exists()
    assert not (MODULE / 'tools').exists() and not (MODULE / 'src').exists()
    assert sha(DEST / 'agent') == sha(BINARY)
    for name in ['ca.pem', 'agent.pem', 'agent.key']:
        assert sha(DEST / 'identity' / name) == sha(IDENTITY / name)
    return complete_prepare(state, resume=True)



def resume_runtime():
    # Exact retained failure: private umask made the outer fixture directory
    # inaccessible to the new Kura account. No service has been created yet.
    state = json.loads(STATE.read_text())
    assert state['phase'] == 'preparing' and baseline() == state['baseline']
    assert HOME.exists() and not list(HOME.iterdir())
    assert not (RUNTIME / 'jvm.args').exists()
    for unit in [AGENT, KURA]:
        assert not (Path('/etc/systemd/system') / unit).exists()
    inventory = json.loads((RUNTIME / 'inventory.json').read_text())
    for entry in inventory:
        assert sha(RUNTIME / entry['path']) == entry['sha256']
    DEST.chmod(0o755)
    return prepare_runtime_services(state)


def change(action):
    state = json.loads(STATE.read_text())
    assert baseline() == state['baseline']
    assert not (DEST / (action + '.json')).exists(), 'retained receipt; do not replay'
    if action == 'start-kura':
        assert state['phase'] == 'agent-only'
        run('systemctl', 'start', KURA)
        wait_bridge(True)
        state['phase'] = 'live'
    elif action == 'restart-agent':
        assert state['phase'] == 'live'
        previous = {'agentPid': pid(AGENT), 'kuraPid': pid(KURA)}
        run('systemctl', 'restart', AGENT)
        wait_bridge(True)
        assert pid(AGENT) != previous['agentPid'] and pid(KURA) == previous['kuraPid']
        state['phase'] = 'reconnected'
        state['restartBefore'] = previous
    elif action == 'stop-kura':
        assert state['phase'] == 'reconnected'
        run('systemctl', 'stop', KURA)
        assert pid(KURA) == 0
        wait_bridge(False)
        state['phase'] = 'kura-stopped'
    elif action == 'finish':
        assert state['phase'] == 'kura-stopped'
        run('systemctl', 'stop', AGENT)
        assert pid(AGENT) == 0
        state['phase'] = 'stopped-data-retained'
    STATE.write_text(json.dumps(state, indent=2) + '\n')
    return status()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('action', choices=['prepare', 'resume-prepare', 'resume-runtime', 'start-kura', 'restart-agent', 'stop-kura', 'finish', 'status'])
    args = parser.parse_args()
    assert os.geteuid() == 0 and socket.gethostname() == 'yofc-management-network-20261101'
    os.umask(0o077)
    if args.action == 'prepare':
        result = prepare()
    elif args.action == 'resume-prepare':
        result = resume_prepare()
    elif args.action == 'resume-runtime':
        result = resume_runtime()
    elif args.action == 'status':
        result = status()
    else:
        result = change(args.action)
    if args.action != 'status':
        receipt(args.action, result)
    print(json.dumps(result, indent=2))


if __name__ == '__main__':
    main()
