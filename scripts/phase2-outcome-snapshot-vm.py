#!/usr/bin/env python3
"""One-use opt-in snapshot and configuration outcome fixture in the existing Ubuntu 24.04 VM."""
import argparse
import importlib.util
import json
import os
from pathlib import Path
import pwd
import shutil
import socket
import subprocess
import time

ROOT = Path(__file__).resolve().parents[1]
DEST = Path('/opt/yofc-outcome-snapshot-20261101')
MODULE = DEST / 'kura-dev-runtime'
RUNTIME = MODULE / 'target/runtime'
DATA = DEST / 'kura-home'
STATE = DEST / 'state.json'
SOURCE = ROOT / '.phase1-dev/configuration-outcome-snapshot-idea-20261101/runtime'
IDENTITY = ROOT / '.phase1-dev/outcome-snapshot-linux-20261101'
BINARY = Path('/tmp/yofc-agent-src-20261101/kura-management-agent/target/release/yofc-management-agent')
ACCOUNT = 'yofc-outcome-snapshot'
AGENT = 'yofc-outcome-snapshot-agent.service'
KURA = 'yofc-outcome-snapshot-kura.service'
GATEWAY = 'outcome-snapshot-linux-20261101'
spec = importlib.util.spec_from_file_location('outcome_vm_baseline', ROOT / 'scripts/phase2-configuration-revision-vm.py')
previous = importlib.util.module_from_spec(spec); spec.loader.exec_module(previous)
run, sha, baseline = previous.run, previous.sha, previous.baseline


def save(state):
    STATE.write_text(json.dumps(state, indent=2) + '\n'); STATE.chmod(0o600)


def wait_proof(stage):
    for _ in range(240):
        if (DEST / 'observer-error.json').exists(): raise AssertionError('observer failed; retain fixture')
        path = DEST / (stage + '-proof.json')
        if path.exists():
            value = json.loads(path.read_text()); assert value['status'] == 'passed'; return value
        time.sleep(.25)
    raise AssertionError('observer deadline; retain fixture')


def service_snapshot(unit):
    pid = int(run('systemctl', 'show', unit, '--property=MainPID', '--value'))
    result = {unit.replace('.service', '') + 'Pid': pid}
    if pid:
        text = Path(f'/proc/{pid}/status').read_text()
        result[unit.replace('.service', '') + 'Uid'] = int(text.split('Uid:\t')[1].split()[0])
        result[unit.replace('.service', '') + 'CapEff'] = text.split('CapEff:\t')[1].split()[0]
        result[unit.replace('.service', '') + 'NoNewPrivileges'] = text.split('NoNewPrivs:\t')[1].split()[0] == '1'
    return result


def status():
    state = json.loads(STATE.read_text()); assert baseline() == state['baseline']
    assert run('systemctl', 'is-active', 'yofc-management-phase2-agent.service').strip() == 'active'
    inventory = json.loads((RUNTIME / 'inventory.json').read_text())
    for item in inventory: assert sha(RUNTIME / item['path']) == item['sha256']
    result = {'phase': state['phase'], 'gateway': GATEWAY, 'baselineUnchanged': True,
              'baselineFiles': len(state['baseline']), 'inventoryChecked': len(inventory),
              'inventoryAllMatch': True, 'fixtureOnly': True, 'networkWrites': 0,
              'agentSha256': sha(DEST / 'agent')}
    result.update(service_snapshot(AGENT)); result.update(service_snapshot(KURA))
    agent_pid = result['yofc-outcome-snapshot-agentPid']; kura_pid = result['yofc-outcome-snapshot-kuraPid']
    if agent_pid:
        assert result['yofc-outcome-snapshot-agentCapEff'] == '0000000000000000'
        assert result['yofc-outcome-snapshot-agentNoNewPrivileges'] is True
    if kura_pid:
        assert result['yofc-outcome-snapshot-kuraUid'] != 0
        assert result['yofc-outcome-snapshot-kuraCapEff'] == '0000000000000000'
    result['fullSystemdSandbox'] = 'deferred: existing OrbStack service override'
    return result


def prepare():
    resuming = DEST.exists()
    assert '24.04' in Path('/etc/os-release').read_text()
    if not resuming:
        for unit in [AGENT, KURA]: assert not (Path('/etc/systemd/system') / unit).exists()
        try:
            pwd.getpwnam(ACCOUNT); raise AssertionError('account exists')
        except KeyError: pass
    assert BINARY.read_bytes()[:4] == b'\x7fELF'
    inventory = json.loads((SOURCE / 'inventory.json').read_text())
    for item in inventory: assert sha(SOURCE / item['path']) == item['sha256'], item['path']
    resuming = DEST.exists()
    if resuming:
        state = json.loads(STATE.read_text()); assert state['phase'] == 'preparing'
        assert (DEST / '.outcome-snapshot-owned').exists() and pwd.getpwnam(ACCOUNT)
        assert (RUNTIME / 'jvm.args').exists() and (RUNTIME / 'launcher.jar').exists()
        old_agent = state['oldAgentPid']
    else:
        initial = baseline(); DEST.mkdir(mode=0o755); DEST.chmod(0o755)
        old_agent = int(run('systemctl', 'show', 'yofc-management-phase2-agent.service', '--property=MainPID', '--value'))
        save({'phase': 'preparing', 'baseline': initial, 'oldAgentPid': old_agent,
              'sourceInventorySha256': sha(SOURCE / 'inventory.json')})
        run('useradd', '--system', '--user-group', '--no-create-home', '--home-dir', str(DATA), '--shell', '/usr/sbin/nologin', ACCOUNT)
        (DEST / '.outcome-snapshot-owned').write_text('new one-use fixture\n'); (DEST / 'stage').write_text('initial\n')
        RUNTIME.mkdir(parents=True)
        shutil.copytree(SOURCE / 'plugins', RUNTIME / 'plugins')
        (RUNTIME / 'configuration').mkdir()
        for name in ['launcher.jar', 'framework.jar', 'inventory.json']: shutil.copy2(SOURCE / name, RUNTIME / name)
        shutil.copy2(SOURCE / 'configuration/config.ini', RUNTIME / 'configuration/config.ini')
        core = ROOT.parent / 'kura/kura-dev-runtime'
        shutil.copytree(core / 'tools', MODULE / 'tools', ignore=shutil.ignore_patterns('__pycache__'))
        shutil.copytree(core / 'src/main/resources', MODULE / 'src/main/resources')
        for name in ['runtime.json', 'network-providers.json']: shutil.copy2(core / name, MODULE / name)
        DATA.mkdir(mode=0o700)
        run('chown', '-R', f'{ACCOUNT}:{ACCOUNT}', str(DEST))
        run('runuser', '-u', ACCOUNT, '--', 'env', f'KURA_DEV_HOME={DATA}', 'KURA_HTTP_PORT=19585', 'KURA_HTTPS_PORT=19657',
            'KURA_CLIENT_AUTH_PORT=19658', 'python3', str(MODULE / 'tools/runtime.py'), 'prepare', '--profile', 'linux', '--network-provider', 'none')
    args = RUNTIME / 'jvm.args'; text = args.read_text().replace('"-Xmx2g"', '"-Xmx768m"')
    control = '/run/yofc-outcome-snapshot/control.sock'
    text += f'"-Dkura.management.socket={control}"\n"-Dkura.management.agent.user=root"\n'
    text += f'"-Dkura.management.configuration.update.pids=org.eclipse.kura.management.revisionFixture"\n'
    text += '"-Dkura.management.snapshot.enabled=true"\n' + f'"-Dkura.configuration.remote.probe.root={DEST}"\n'
    assert '/Users/' not in text; args.write_text(text)
    with (RUNTIME / 'configuration/kura.properties').open('a') as output: output.write('\nkura.need.password.change=false\n')
    identity = DEST / 'identity'; identity.mkdir(mode=0o700, exist_ok=True); (DEST / 'agent-state').mkdir(mode=0o700, exist_ok=True)
    shutil.copy2(BINARY, DEST / 'agent'); (DEST / 'agent').chmod(0o700)
    for name in ['ca.pem', 'agent.pem', 'agent.key']:
        shutil.copy2(IDENTITY / name, identity / name); (identity / name).chmod(0o600)
    for path in [identity, DEST / 'agent', DEST / 'agent-state']: run('chown', '-R', 'root:root', str(path))
    env = {'AGENT_CONSOLE_ENDPOINT': 'https://host.orb.internal:18444', 'AGENT_GATEWAY_ID': GATEWAY,
           'AGENT_CA': str(identity / 'ca.pem'), 'AGENT_CERT': str(identity / 'agent.pem'), 'AGENT_KEY': str(identity / 'agent.key'),
           'AGENT_STATE_DB': str(DEST / 'agent-state/results.sqlite'), 'AGENT_SOCKET': control,
           'AGENT_KURA_UID': str(pwd.getpwnam(ACCOUNT).pw_uid), 'AGENT_ADMIN_UID': '0',
           'AGENT_NETWORK_INTERFACES': '', 'AGENT_KURA_NETWORK_INTERFACES': '',
           'AGENT_KURA_CONFIGURATION_UPDATE': '1', 'AGENT_KURA_SNAPSHOT': '1', 'RUST_LOG': 'info'}
    env_path = DEST / 'agent.env'; env_path.write_text(''.join(key + '=' + value + '\n' for key, value in env.items())); env_path.chmod(0o600)
    run('chown', 'root:root', str(env_path))
    agent_unit = f'[Unit]\nDescription=Outcome snapshot isolated agent\n[Service]\nType=simple\nUser=root\nGroup={ACCOUNT}\nUMask=0077\nRuntimeDirectory=yofc-outcome-snapshot\nRuntimeDirectoryMode=0750\nEnvironmentFile={env_path}\nExecStart={DEST}/agent\nNoNewPrivileges=true\nCapabilityBoundingSet=\nTimeoutStopSec=20\n'
    kura_unit = f'[Unit]\nDescription=Outcome snapshot isolated Kura\nAfter={AGENT}\n[Service]\nType=simple\nUser={ACCOUNT}\nGroup={ACCOUNT}\nUMask=0077\nWorkingDirectory={RUNTIME}\nExecStart=/usr/bin/java @{args} -jar {RUNTIME}/launcher.jar -configuration {RUNTIME}/configuration -consoleLog\nNoNewPrivileges=true\nCapabilityBoundingSet=\nTimeoutStopSec=30\n'
    for unit, content in [(AGENT, agent_unit), (KURA, kura_unit)]: (Path('/etc/systemd/system') / unit).write_text(content)
    state = json.loads(STATE.read_text()); state['phase'] = 'prepared'; save(state)
    run('systemctl', 'daemon-reload'); return status()


def change(action):
    state = json.loads(STATE.read_text()); assert baseline() == state['baseline']
    if action == 'start':
        assert state['phase'] == 'prepared' and not (DEST / 'initial-proof.json').exists()
        run('systemctl', 'start', AGENT); run('systemctl', 'start', KURA); proof = wait_proof('initial')
        state.update(phase='started', initialProof=proof)
    elif action == 'finish':
        assert state['phase'] in {'started', 'validated'}
        run('systemctl', 'stop', KURA); run('systemctl', 'stop', AGENT); state['phase'] = 'stopped-data-retained'
    else: raise AssertionError(action)
    save(state); return status()


if __name__ == '__main__':
    parser = argparse.ArgumentParser(); parser.add_argument('action', choices=['prepare', 'start', 'finish', 'status']); action = parser.parse_args().action
    assert os.geteuid() == 0 and socket.gethostname() == 'yofc-management-network-20261101'; os.umask(0o077)
    result = prepare() if action == 'prepare' else status() if action == 'status' else change(action)
    if action != 'status':
        marker = DEST / (action + '.json')
        with marker.open('x') as output: json.dump(result, output, indent=2); output.write('\n')
    print(json.dumps(result, indent=2))
