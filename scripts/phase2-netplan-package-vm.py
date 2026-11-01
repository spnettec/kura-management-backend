#!/usr/bin/env python3
"""One-way, named package validation on the existing OrbStack test gateway.

Run inside the VM as root. This never replays the restored development-runtime fixture.
Receipts and any retained archive stay local to the VM; secrets are never printed.
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
import urllib.error
import urllib.request

FIXTURE = Path('/opt/yofc-kura-netplan-package-20261101')
STATE = Path('/var/lib/kura-netplan')
CONFIG = Path('/etc/kura-netplan')


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def run(args, name=None, check=True):
    result = subprocess.run(args, capture_output=True, text=True)
    if name:
        (FIXTURE / (name + '.log')).write_text(result.stdout + result.stderr)
    if check and result.returncode:
        raise RuntimeError(f'{args[0]} failed ({result.returncode}); inspect VM receipt {name}')
    return result.stdout


def ownership():
    result = {}
    for base, patterns in [('/etc/netplan', ('*.yaml', '*.yml')), ('/etc/systemd/network', ('*',)),
                           ('/etc/NetworkManager/conf.d', ('*',))]:
        for pattern in patterns:
            for path in Path(base).glob(pattern):
                if path.is_file():
                    result[str(path)] = digest(path)
    for path in [Path('/opt/yofc-kura-netplan-profile/state.json'),
                 Path('/etc/yofc-management-phase2/agent.env')]:
        if path.is_file():
            result[str(path)] = digest(path)
    return result


def retained():
    return {str(path): digest(path) for base in [STATE, CONFIG] for path in base.rglob('*') if path.is_file()}


def save(state):
    (FIXTURE / 'state.json').write_text(json.dumps(state, indent=2) + '\n')


def main(args):
    if os.getuid() != 0:
        raise RuntimeError('VM root required')
    if args.action == 'inventory':
        if FIXTURE.exists():
            raise RuntimeError('Validation already exists; do not remove its state or replay inventory')
        for path in [Path('/opt/eclipse/kura'), STATE, CONFIG]:
            if path.exists() or path.is_symlink():
                raise RuntimeError('Existing installation/state; explicit migration required')
        FIXTURE.mkdir(mode=0o700)
        state = {'phase': 'inventoried', 'baseline': ownership(),
                 'os': Path('/etc/os-release').read_text(),
                 'agentBaseline': run(['systemctl', 'is-active', 'yofc-management-phase2-agent.service']).strip(),
                 'receipts': []}
        save(state)
    else:
        state = json.loads((FIXTURE / 'state.json').read_text())
    expected = {'install': 'inventoried', 'install-fixed': 'bootstrap-failure-archived',
                'verify-install': 'bootstrap-failure-archived',
                'archive-bootstrap-failure': 'installed', 'start': 'installed', 'verify-start': 'installed', 'repair-prefix': 'installed', 'upgrade': 'started',
                'remove': 'upgraded', 'reinstall': 'removed', 'service-stop-check': 'reinstalled', 'purge': 'reinstalled'}
    if args.action != 'inventory' and state['phase'] != expected[args.action]:
        raise RuntimeError(f'Expected {expected[args.action]}, found {state["phase"]}; do not replay')
    receipt = {'action': args.action}
    if args.action == 'service-stop-check':
        if state.get('serviceStopChecked'):
            raise RuntimeError('Service stop check already completed')
        before = retained()
        run(['dpkg', '--install', str(args.core.resolve()), str(args.network.resolve())], 'stop-fix-upgrade')
        assert retained() == before
        run(['systemctl', 'reset-failed', 'kura-netplan.service'])
        run(['systemctl', 'start', 'kura-netplan.service'])
        for _ in range(60):
            import socket
            try:
                with socket.create_connection(('127.0.0.1', 8080), timeout=1):
                    break
            except OSError:
                time.sleep(.5)
        else:
            raise RuntimeError('Listener did not return for stop verification')
        run(['systemctl', 'stop', 'kura-netplan.service'])
        assert run(['systemctl', 'is-active', 'kura-netplan.service'], check=False).strip() == 'inactive'
        assert run(['systemctl', 'show', 'kura-netplan.service', '-p', 'Result', '--value']).strip() == 'success'
        receipt['normalJavaExit143'] = 'accepted by systemd'
        receipt['artifacts'] = {str(p): digest(p) for p in (args.core, args.network)}
        state['serviceStopChecked'] = True
        state['retained'] = retained()
    elif args.action == 'archive-bootstrap-failure':
        run(['systemctl', 'stop', 'kura-netplan.service'])
        destination = FIXTURE / 'failed-bootstrap'
        if destination.exists():
            raise RuntimeError('Failed bootstrap was already archived')
        destination.mkdir(mode=0o700)
        run(['journalctl', '-u', 'kura-netplan.service', '--no-pager'], 'failed-bootstrap-journal')
        shutil.copytree('/var/log/kura-netplan', destination / 'logs')
        run(['dpkg', '--remove', 'kura-networking-netplan', 'kura-core-netplan'], 'failed-bootstrap-remove')
        shutil.move(str(STATE), destination / 'state')
        shutil.move(str(CONFIG), destination / 'config')
        run(['dpkg', '--purge', 'kura-networking-netplan', 'kura-core-netplan'], 'failed-bootstrap-purge')
        restored = json.loads(Path('/opt/yofc-kura-netplan-profile/state.json').read_text())
        env_path = Path('/etc/yofc-management-phase2/agent.env')
        assert restored['status'] == 'restored' and digest(env_path) == restored['originalAgentEnvSha256']
        state['baseline'][str(env_path)] = digest(env_path)
        state['phase'] = 'bootstrap-failure-archived'
        receipt['failedBootstrap'] = str(destination)
    elif args.action == 'repair-prefix':
        # Repair only this failed, new bootstrap; preserve its before-image and all keys.
        if state.get('prefixRepaired'):
            raise RuntimeError('Prefix repair has already run')
        run(['systemctl', 'stop', 'kura-netplan.service'])
        seed = STATE / 'user/snapshots/snapshot_0.xml'
        if list(seed.parent.glob('snapshot_*.xml')) != [seed]:
            raise RuntimeError('Unexpected newer snapshots; explicit recovery required')
        old = seed.read_bytes()
        if b'<ns0:configurations' not in old or b'xmlns:ns0="http://eurotech.com/esf/2.0"' not in old:
            raise RuntimeError('Seed is not the known failed bootstrap')
        backup = FIXTURE / 'snapshot_0-before-prefix-repair.xml'
        backup.write_bytes(old)
        backup.chmod(0o600)
        updated = old.replace(b'xmlns:ns0=', b'xmlns:esf=').replace(b'<ns0:', b'<esf:').replace(b'</ns0:', b'</esf:')
        from xml.dom import minidom
        assert minidom.parseString(updated).getElementsByTagName('esf:configuration')
        seed.write_bytes(updated)
        state['prefixRepaired'] = True
        # Correct the initially mistyped env path against the already restored fixture's recorded SHA.
        restored = json.loads(Path('/opt/yofc-kura-netplan-profile/state.json').read_text())
        env_path = Path('/etc/yofc-management-phase2/agent.env')
        assert restored['status'] == 'restored' and digest(env_path) == restored['originalAgentEnvSha256']
        state['baseline'][str(env_path)] = digest(env_path)
        receipt['seedBeforeSha256'] = hashlib.sha256(old).hexdigest()
        receipt['seedAfterSha256'] = digest(seed)
    elif args.action in ('install', 'install-fixed', 'verify-install', 'upgrade', 'reinstall'):
        if not args.core or not args.network:
            raise RuntimeError('Explicit package paths required')
        paths = [str(args.core.resolve()), str(args.network.resolve())]
        receipt['artifacts'] = {str(path): digest(path) for path in (args.core, args.network)}
        if args.action in ('install', 'install-fixed'):
            simulated = run(['apt-get', '--simulate', 'install', *paths], 'apt-simulation')
            assert not any(('Inst ' + name + ' ') in simulated for name in ['network-manager', 'modemmanager', 'iptables'])
            run(['apt-get', 'install', '-y', *paths], 'install')
        elif args.action == 'verify-install':
            for path in paths:
                name = run(['dpkg-deb', '--field', path, 'Package']).strip()
                version = run(['dpkg-deb', '--field', path, 'Version']).strip()
                assert run(['dpkg-query', '-W', '-f=${Version}', name]).strip() == version
        else:
            if args.action == 'upgrade':
                run(['systemctl', 'stop', 'kura-netplan.service'])
                state['retained'] = retained()
            before = state['retained']
            run(['dpkg', '--install', *paths], args.action)
            assert retained() == before, 'Upgrade/reinstall altered retained state'
        active = run(['systemctl', 'is-active', 'kura-netplan.service'], check=False).strip()
        assert active in ('inactive', 'failed')
        assert run(['systemctl', 'show', 'kura-netplan.service', '--property=MainPID', '--value']).strip() == '0'
        assert run(['systemctl', 'is-enabled', 'kura-netplan.service'], check=False).strip() == 'disabled'
        receipt['unit'] = active + ', MainPID=0 and disabled; explicit start required'
        state['phase'] = {'install': 'installed', 'install-fixed': 'installed', 'verify-install': 'installed', 'upgrade': 'upgraded', 'reinstall': 'reinstalled'}[args.action]
    elif args.action in ('start', 'verify-start'):
        if args.action == 'start':
            run(['systemctl', 'start', 'kura-netplan.service'])
        for _ in range(80):
            try:
                with urllib.request.urlopen('http://127.0.0.1:8080/', timeout=1) as response:
                    receipt['http'] = response.status
                    break
            except urllib.error.HTTPError as error:
                receipt['http'] = error.code
                break
            except (OSError, urllib.error.URLError):
                time.sleep(0.5)
        else:
            run(['journalctl', '-u', 'kura-netplan.service', '--no-pager', '-n', '160'], 'startup-failure')
            raise RuntimeError('Kura HTTP listener did not start; preserve state and inspect startup-failure.log')
        pid = int(run(['systemctl', 'show', 'kura-netplan.service', '--property=MainPID', '--value']).strip())
        status = Path(f'/proc/{pid}/status').read_text()
        fields = dict(line.split(':', 1) for line in status.splitlines() if ':' in line)
        uid = pwd.getpwnam('kura-netplan').pw_uid
        assert int(fields['Uid'].split()[0]) == uid and uid != 0
        assert int(fields['CapEff'].strip(), 16) == 0
        argv = Path(f'/proc/{pid}/cmdline').read_bytes()
        assert CONFIG.joinpath('crypto.key').read_bytes().strip() not in argv
        hardened = fields['NoNewPrivs'].strip() == '1'
        receipt.update(pid=pid, uid=uid, effectiveCapabilities='0', noNewPrivileges=hardened)
        unit = run(['systemctl', 'cat', 'kura-netplan.service'], 'effective-unit')
        if not hardened:
            assert '/run/systemd/system/service.d/zzz-lxc-service.conf' in unit
            receipt['systemHardening'] = 'deferred: OrbStack global LXC override disables NoNewPrivileges and filesystem sandbox'
        run(['journalctl', '-u', 'kura-netplan.service', '--no-pager'], 'started-journal')
        receipt['networkWriterInConfig'] = any(name in Path('/run/kura-netplan/configuration/config.ini').read_text()
                   for name in ['org.eclipse.kura.nm-', 'org.eclipse.kura.linux.net-', 'org.eclipse.kura.net.admin.firewall-'])
        assert not receipt['networkWriterInConfig']
        (STATE / 'data/package-validation-user-data').write_text('retain through ordinary uninstall\n')
        state['phase'] = 'started'
    elif args.action == 'remove':
        run(['dpkg', '--remove', 'kura-networking-netplan', 'kura-core-netplan'], 'remove')
        assert retained() == state['retained'], 'Ordinary remove changed retained data'
        state['phase'] = 'removed'
    elif args.action == 'purge':
        # Archive this test's generated data before verifying the declared purge scope.
        archive = FIXTURE / 'retained-before-purge.tar.gz'
        with tarfile.open(archive, 'w:gz') as output:
            for path in [STATE, CONFIG]:
                output.add(path, arcname=str(path).lstrip('/'))
        archive.chmod(0o600)
        run(['dpkg', '--purge', 'kura-networking-netplan', 'kura-core-netplan'], 'purge')
        assert not STATE.exists() and not CONFIG.exists()
        assert not Path('/var/log/kura-netplan').exists()
        receipt['archive'] = str(archive)
        state['phase'] = 'purged'
    assert ownership() == state['baseline'], 'Another owner state changed'
    assert run(['systemctl', 'is-active', 'yofc-management-phase2-agent.service']).strip() == state['agentBaseline']
    receipt['result'] = 'passed'
    state['receipts'].append(receipt)
    save(state)
    # Never expose bootstrap credentials, key material or retained payload content.
    print(json.dumps({'phase': state['phase'], 'receipt': receipt, 'baselineUnchanged': True}, indent=2))


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('action', choices=['inventory', 'install', 'archive-bootstrap-failure', 'install-fixed', 'verify-install', 'repair-prefix', 'start', 'verify-start', 'upgrade', 'remove', 'reinstall', 'service-stop-check', 'purge'])
    parser.add_argument('--core', type=Path)
    parser.add_argument('--network', type=Path)
    main(parser.parse_args())
