#!/usr/bin/env python3
"""One-use real Linux CM/DS/snapshot proof; does not enable a remote configuration writer."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import pwd
import shutil
import socket
import subprocess
import time

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / '.phase1-dev/configuration-revision-final-20261101/runtime'
DEST = Path('/opt/yofc-configuration-revision-20261101')
MODULE = DEST / 'kura-dev-runtime'
RUNTIME = MODULE / 'target/runtime'
DATA = DEST / 'kura-home'
STATE = DEST / 'state.json'
ACCOUNT = 'yofc-revision'
UNIT = 'yofc-configuration-revision-kura.service'
BASELINE = [
    '/etc/yofc-management-phase2/agent.env', '/opt/yofc-kura-netplan-profile/state.json',
    '/opt/yofc-kura-netplan-package-20261101/state.json', '/opt/yofc-agent-package-20261101/state.json',
    '/opt/yofc-config-read-20261101/state.json', '/opt/yofc-network-outcome-20261101/final-state.json',
    '/etc/netplan/10-lxc.yaml', '/etc/netplan/90-yofc-managed.yaml',
]


def run(*args):
    return subprocess.run(args, check=True, capture_output=True, text=True).stdout


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def baseline():
    return {p: sha(p) for p in BASELINE}


def save(state):
    STATE.write_text(json.dumps(state, indent=2) + '\n'); STATE.chmod(0o600)


def status():
    state = json.loads(STATE.read_text())
    assert baseline() == state['baseline'], 'old fixture or network input changed'
    assert run('systemctl', 'is-active', 'yofc-management-phase2-agent.service').strip() == 'active'
    pid = int(run('systemctl', 'show', UNIT, '--property=MainPID', '--value'))
    result = {'phase': state['phase'], 'kuraPid': pid, 'oldAgentActive': True, 'baselineUnchanged': True,
              'fixtureOnly': True, 'networkWrites': 0, 'remoteWriteValidated': False,
              'inventorySha256': sha(RUNTIME / 'inventory.json')}
    if pid:
        text = Path(f'/proc/{pid}/status').read_text()
        result.update(kuraUid=int(text.split('Uid:\t')[1].split()[0]),
                      kuraCapEff=text.split('CapEff:\t')[1].split()[0])
        assert result['kuraUid'] != 0 and int(result['kuraCapEff'], 16) == 0
    return result


def prepare():
    assert not DEST.exists(), 'Retained fixture exists; never replay prepare'
    assert '24.04' in Path('/etc/os-release').read_text()
    inventory = json.loads((SOURCE / 'inventory.json').read_text())
    for item in inventory:
        assert sha(SOURCE / item['path']) == item['sha256'], item['path']
    unit = Path('/etc/systemd/system') / UNIT
    assert not unit.exists()
    try:
        pwd.getpwnam(ACCOUNT)
        raise AssertionError('dedicated account already exists')
    except KeyError:
        pass
    initial = baseline()
    DEST.mkdir(mode=0o755); DEST.chmod(0o755)
    save({'phase': 'preparing', 'baseline': initial, 'sourceInventorySha256': sha(SOURCE / 'inventory.json')})
    run('useradd', '--system', '--user-group', '--no-create-home', '--home-dir', str(DATA), '--shell', '/usr/sbin/nologin', ACCOUNT)
    (DEST / '.configuration-revision-owned').write_text('test-only\n')
    RUNTIME.mkdir(parents=True)
    shutil.copytree(SOURCE / 'plugins', RUNTIME / 'plugins')
    (RUNTIME / 'configuration').mkdir()
    for name in ['launcher.jar', 'framework.jar', 'inventory.json']:
        shutil.copy2(SOURCE / name, RUNTIME / name)
    shutil.copy2(SOURCE / 'configuration/config.ini', RUNTIME / 'configuration/config.ini')
    core_module = ROOT.parent / 'kura/kura-dev-runtime'
    shutil.copytree(core_module / 'tools', MODULE / 'tools', ignore=shutil.ignore_patterns('__pycache__'))
    shutil.copytree(core_module / 'src/main/resources', MODULE / 'src/main/resources')
    for name in ['runtime.json', 'network-providers.json']:
        shutil.copy2(core_module / name, MODULE / name)
    DATA.mkdir(mode=0o700)
    run('chown', '-R', f'{ACCOUNT}:{ACCOUNT}', str(DEST))
    run('runuser', '-u', ACCOUNT, '--', 'env', f'KURA_DEV_HOME={DATA}', 'KURA_HTTP_PORT=19084',
        'KURA_HTTPS_PORT=19455', 'KURA_CLIENT_AUTH_PORT=19456', 'python3', str(MODULE / 'tools/runtime.py'),
        'prepare', '--profile', 'linux', '--network-provider', 'none')
    args = RUNTIME / 'jvm.args'; text = args.read_text().replace('"-Xmx2g"', '"-Xmx768m"')
    text += f'"-Dkura.management.socket=/run/yofc-configuration-revision/no-agent.sock"\n'
    text += f'"-Dkura.revision.probe.root={DEST}"\n"-Dkura.revision.probe.phase=write"\n'
    assert '/Users/' not in text
    args.write_text(text)
    unit.write_text(f'[Unit]\nDescription=Configuration revision test-only Kura\n[Service]\nType=simple\n'
                    f'User={ACCOUNT}\nGroup={ACCOUNT}\nUMask=0077\nWorkingDirectory={RUNTIME}\n'
                    f'ExecStart=/usr/bin/java @{args} -jar {RUNTIME}/launcher.jar -configuration {RUNTIME}/configuration -consoleLog\n'
                    'NoNewPrivileges=true\nCapabilityBoundingSet=\nTimeoutStopSec=30\n')
    state = json.loads(STATE.read_text()); state['phase'] = 'prepared'; save(state)
    run('systemctl', 'daemon-reload')
    return status()


def wait_result(phase):
    output = DEST / (phase + '-result.json'); deadline = time.monotonic() + 60
    while time.monotonic() < deadline:
        if output.exists():
            value = json.loads(output.read_text())
            assert value['status'] == 'passed', value
            return value
        time.sleep(.2)
    raise AssertionError('probe result deadline; retain failed fixture for diagnosis')


def change(action):
    state = json.loads(STATE.read_text()); assert baseline() == state['baseline']
    if action == 'start':
        assert state['phase'] == 'prepared' and not (DEST / 'write-result.json').exists()
        run('systemctl', 'start', UNIT); result = wait_result('write')
        state.update(phase='written', writeResult=result)
    elif action == 'restart':
        assert state['phase'] == 'written' and not (DEST / 'restart-result.json').exists()
        before = int(run('systemctl', 'show', UNIT, '--property=MainPID', '--value'))
        run('systemctl', 'stop', UNIT)
        args = RUNTIME / 'jvm.args'; text = args.read_text(); assert text.count('-Dkura.revision.probe.phase=write') == 1
        args.write_text(text.replace('-Dkura.revision.probe.phase=write', '-Dkura.revision.probe.phase=restart'))
        run('systemctl', 'start', UNIT); result = wait_result('restart')
        after = int(run('systemctl', 'show', UNIT, '--property=MainPID', '--value')); assert before != after
        state.update(phase='restarted', restartResult=result, previousPid=before)
    else:
        assert action == 'finish' and state['phase'] == 'restarted'
        run('systemctl', 'stop', UNIT); state['phase'] = 'stopped-data-retained'
    save(state); return status()


if __name__ == '__main__':
    parser = argparse.ArgumentParser(); parser.add_argument('action', choices=['prepare', 'start', 'restart', 'finish', 'status'])
    action = parser.parse_args().action
    assert os.geteuid() == 0 and socket.gethostname() == 'yofc-management-network-20261101'
    os.umask(0o077)
    result = prepare() if action == 'prepare' else status() if action == 'status' else change(action)
    if action != 'status':
        with (DEST / (action + '.json')).open('x') as out: json.dump(result, out, indent=2); out.write('\n')
    print(json.dumps(result, indent=2))
