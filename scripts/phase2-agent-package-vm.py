#!/usr/bin/env python3
"""One-way package/UID validation in the named, non-production OrbStack gateway.

No network mutations; existing fixture identities and configuration are hash-guarded.
Secrets and before-purge archives remain root-only in the VM.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import pwd
import shutil
import subprocess
import tarfile
import time

FIXTURE = Path('/opt/yofc-agent-package-20261101')
AGENT_CONFIG = Path('/etc/kura-management-agent')
AGENT_STATE = Path('/var/lib/kura-management-agent')
KURA_CONFIG = Path('/etc/kura-netplan')
KURA_STATE = Path('/var/lib/kura-netplan')
UNIT = 'kura-management-agent.service'
GATEWAY = 'package-gateway-20261101'


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def run(args, name=None, check=True):
    path = FIXTURE / (name + '.log') if name else None
    if path and path.exists():
        raise RuntimeError('Receipt already exists; inspect the previous attempt')
    result = subprocess.run([str(x) for x in args], capture_output=True, text=True)
    if path:
        path.write_text(result.stdout + result.stderr)
        path.chmod(0o600)
    if check and result.returncode:
        raise RuntimeError(f'{args[0]} failed ({result.returncode}); inspect receipt {name}')
    return result


def baseline():
    paths = [Path('/opt/yofc-kura-netplan-profile/state.json'),
             Path('/opt/yofc-kura-netplan-package-20261101/state.json'),
             Path('/etc/yofc-management-phase2/agent.env')]
    for directory in ['/etc/netplan', '/etc/systemd/network', '/etc/NetworkManager/conf.d']:
        paths.extend(path for path in Path(directory).glob('*') if path.is_file())
    return {str(path): digest(path) for path in sorted(paths) if path.is_file()}


def retained():
    return {str(path): digest(path) for base in [AGENT_CONFIG, AGENT_STATE, KURA_CONFIG, KURA_STATE]
            for path in base.rglob('*') if path.is_file()}


def inactive(unit):
    assert run(['systemctl', 'show', unit, '-p', 'MainPID', '--value']).stdout.strip() == '0'


def save(state):
    path = FIXTURE / 'state.json'
    path.write_text(json.dumps(state, indent=2) + '\n')
    path.chmod(0o600)


def main(args):
    assert os.getuid() == 0 and 'VERSION_ID="24.04"' in Path('/etc/os-release').read_text()
    if args.action == 'inventory':
        if FIXTURE.exists():
            raise RuntimeError('Keep previous validation; never delete state to replay')
        for path in [AGENT_CONFIG, AGENT_STATE, KURA_CONFIG, KURA_STATE, Path('/opt/eclipse/kura')]:
            assert not path.exists() and not path.is_symlink(), 'Existing installation needs explicit migration'
        FIXTURE.mkdir(mode=0o700)
        state = {'phase': 'inventoried', 'baseline': baseline(), 'receipts': []}
    else:
        state = json.loads((FIXTURE / 'state.json').read_text())
    required = {'install': 'inventoried', 'identity': 'installed', 'recover-activation': 'installed', 'authorize': 'identity-imported',
                'start-kura': 'authorized', 'stop-kura': 'kura-started', 'upgrade': 'kura-stopped',
                'remove': 'upgraded', 'reinstall': 'removed', 'resume': 'reinstalled',
                'settle': 'reinstalled-running', 'purge': 'reinstalled'}
    if args.action != 'inventory' and state['phase'] != required[args.action]:
        raise RuntimeError(f'Expected {required[args.action]}, found {state["phase"]}; inspect previous attempt')
    receipt = {'action': args.action}
    if args.action in ('install', 'upgrade', 'reinstall'):
        packages = [args.agent, args.core, args.network] if args.action != 'upgrade' else [args.agent]
        assert all(packages), 'Explicit package paths required'
        receipt['artifacts'] = {str(path): digest(path) for path in packages}
        if args.action == 'install':
            simulation = run(['apt-get', '--simulate', 'install', *packages], 'apt-simulation').stdout
            assert not any('Inst ' + name + ' ' in simulation for name in ['network-manager', 'modemmanager', 'iptables'])
            run(['apt-get', 'install', '-y', *packages], 'install')
            assert run(['systemctl', 'is-enabled', UNIT], check=False).stdout.strip() == 'disabled'
            assert not (AGENT_CONFIG / 'agent.env').exists()
        else:
            if args.action == 'upgrade':
                run(['systemctl', 'stop', UNIT])
                state['retained'] = retained()
            before = state['retained']
            run(['dpkg', '--install', *packages], args.action)
            assert retained() == before, 'Package operation altered retained identity or data'
        inactive(UNIT)
        inactive('kura-netplan.service')
        state['phase'] = {'install': 'installed', 'upgrade': 'upgraded', 'reinstall': 'reinstalled'}[args.action]
    elif args.action == 'identity':
        source = args.identity
        assert source and source.is_dir()
        destination = FIXTURE / 'identity'
        destination.mkdir(mode=0o700)
        for name in ['ca.pem', 'agent.pem', 'agent.key']:
            target = destination / name
            shutil.copyfile(source / name, target)
            target.chmod(0o600)
        command = ['kura-agent-setup', 'import-identity', '--endpoint', 'https://host.orb.internal:18444',
                   '--gateway', GATEWAY, '--ca', destination / 'ca.pem', '--cert', destination / 'agent.pem',
                   '--key', destination / 'agent.key']
        run(command, 'identity-import')
        before = retained()
        run(command, 'identity-repeat')
        assert before == retained()
        refused = list(command)
        refused[refused.index('https://host.orb.internal:18444')] = 'https://other-console.example.invalid:18444'
        result = run(refused, 'second-console-refused', check=False)
        assert result.returncode != 0 and retained() == before
        env = (AGENT_CONFIG / 'agent.env').read_text()
        assert 'AGENT_KURA_UID=4294967295\n' in env and 'AGENT_NETWORK_INTERFACES=\n' in env
        run(['kura-agent-setup', 'activate'], 'activate')
        assert run(['systemctl', 'is-active', UNIT]).stdout.strip() == 'active'
        state['phase'] = 'identity-imported'
        receipt['secondConsoleOverwrite'] = 'refused; retained identity unchanged'
    elif args.action == 'recover-activation':
        # Preserve the imported identity and failed receipt; do not replay enrollment.
        assert (FIXTURE / 'activate.log').is_file() and args.agent
        inactive(UNIT)
        before = retained()
        run(['journalctl', '-u', UNIT, '--no-pager'], 'failed-activation-journal')
        run(['dpkg', '--install', args.agent], 'activation-fix-upgrade')
        assert retained() == before
        run(['systemctl', 'reset-failed', UNIT])
        run(['kura-agent-setup', 'activate'], 'activate-fixed')
        assert run(['systemctl', 'is-active', UNIT]).stdout.strip() == 'active'
        receipt['artifacts'] = {str(args.agent): digest(args.agent)}
        receipt['failedCandidate'] = 'revision 2 activation deadlock; identity retained unchanged'
        state['phase'] = 'identity-imported'
    elif args.action == 'authorize':
        run(['systemctl', 'stop', UNIT])
        run(['kura-agent-setup', 'authorize-kura', '--user', 'kura-netplan'], 'authorize-kura')
        run(['kura-agent-setup', 'check'], 'check-authorized')
        uid = pwd.getpwnam('kura-netplan').pw_uid
        env = (AGENT_CONFIG / 'agent.env').read_text()
        assert f'AGENT_KURA_UID={uid}\n' in env and 'AGENT_KURA_NETWORK_INTERFACES=\n' in env
        run(['kura-agent-setup', 'activate'], 'activate-authorized')
        inactive('kura-netplan.service')
        receipt.update(kuraUid=uid, networkWriteDelegation=False)
        state['phase'] = 'authorized'
    elif args.action == 'start-kura':
        run(['systemctl', 'start', 'kura-netplan.service'], 'start-kura')
        for _ in range(100):
            if Path('/run/kura-netplan/configuration/config.ini').exists() and Path('/run/kura-management-agent/control.sock').is_socket():
                pid = int(run(['systemctl', 'show', 'kura-netplan.service', '-p', 'MainPID', '--value']).stdout.strip())
                if pid:
                    break
            time.sleep(.2)
        else:
            raise RuntimeError('Kura/socket not ready')
        fields = dict(line.split(':', 1) for line in Path(f'/proc/{pid}/status').read_text().splitlines() if ':' in line)
        assert int(fields['Uid'].split()[0]) == pwd.getpwnam('kura-netplan').pw_uid
        assert int(fields['CapEff'].strip(), 16) == 0
        receipt.update(kuraPid=pid, kuraUid=int(fields['Uid'].split()[0]), kuraEffectiveCapabilities='0')
        receipt['systemHardening'] = 'deferred: OrbStack LXC global systemd sandbox override'
        state['phase'] = 'kura-started'
    elif args.action == 'stop-kura':
        run(['systemctl', 'stop', 'kura-netplan.service'], 'stop-kura')
        inactive('kura-netplan.service')
        assert run(['systemctl', 'show', 'kura-netplan.service', '-p', 'Result', '--value']).stdout.strip() == 'success'
        assert run(['systemctl', 'is-active', UNIT]).stdout.strip() == 'active'
        state['retained'] = retained()
        state['phase'] = 'kura-stopped'
    elif args.action == 'remove':
        run(['dpkg', '--remove', 'kura-management-agent', 'kura-networking-netplan', 'kura-core-netplan'], 'remove')
        assert retained() == state['retained']
        state['phase'] = 'removed'
    elif args.action == 'resume':
        run(['kura-agent-setup', 'check'], 'reinstalled-check')
        run(['kura-agent-setup', 'activate'], 'reinstalled-activate')
        run(['systemctl', 'start', 'kura-netplan.service'], 'reinstalled-start-kura')
        assert run(['systemctl', 'is-active', UNIT]).stdout.strip() == 'active'
        state['phase'] = 'reinstalled-running'
    elif args.action == 'settle':
        run(['systemctl', 'stop', 'kura-netplan.service'], 'reinstalled-stop-kura')
        run(['systemctl', 'stop', UNIT], 'reinstalled-stop-agent')
        inactive(UNIT)
        inactive('kura-netplan.service')
        state['retained'] = retained()
        state['phase'] = 'reinstalled'
    elif args.action == 'purge':
        archive = FIXTURE / 'retained-before-purge.tar.gz'
        with tarfile.open(archive, 'x:gz') as output:
            for path in [AGENT_CONFIG, AGENT_STATE, KURA_CONFIG, KURA_STATE]:
                output.add(path, arcname=str(path).lstrip('/'))
        archive.chmod(0o600)
        run(['dpkg', '--purge', 'kura-management-agent', 'kura-networking-netplan', 'kura-core-netplan'], 'purge')
        assert all(not path.exists() for path in [AGENT_CONFIG, AGENT_STATE, KURA_CONFIG, KURA_STATE])
        assert not Path('/etc/systemd/system/kura-netplan.service.d/agent-authorization.conf').exists()
        receipt['archive'] = str(archive)
        state['phase'] = 'purged'
    assert baseline() == state['baseline'], 'Existing owner or prior fixture changed'
    assert run(['systemctl', 'is-active', 'yofc-management-phase2-agent.service']).stdout.strip() == 'active'
    receipt['result'] = 'passed'
    state['receipts'].append(receipt)
    save(state)
    print(json.dumps({'phase': state['phase'], 'receipt': receipt, 'existingOwnerUnchanged': True}, indent=2))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action', choices=['inventory', 'install', 'identity', 'recover-activation', 'authorize', 'start-kura',
                                         'stop-kura', 'upgrade', 'remove', 'reinstall', 'resume', 'settle', 'purge'])
    for name in ['agent', 'core', 'network', 'identity']:
        parser.add_argument('--' + name, type=Path)
    main(parser.parse_args())
