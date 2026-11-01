#!/usr/bin/env python3
"""One-way validation of the new agent deb in the named non-production VM.

Only this new package fixture owns the standard agent paths. No Kura/network
package is installed; network writers stay disabled. Synthetic unresolved rows
exercise shipped maintainer scripts and are removed only by their exact test ID.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import socket
import sqlite3
import subprocess
import tarfile
import uuid

ROOT = Path(__file__).resolve().parents[1]
FIXTURE = Path('/opt/yofc-task-journal-package-20261101')
STATE = FIXTURE / 'state.json'
CONFIG = Path('/etc/kura-management-agent')
DATA = Path('/var/lib/kura-management-agent')
UNIT = 'kura-management-agent.service'
GATEWAY = 'task-journal-package-gateway-20261101'
IDENTITY = ROOT / '.phase1-dev/task-journal-package-20261101'
BASELINE_PATHS = [
    '/etc/yofc-management-phase2/agent.env', '/opt/yofc-kura-netplan-profile/state.json',
    '/opt/yofc-kura-netplan-package-20261101/state.json', '/opt/yofc-agent-package-20261101/state.json',
    '/opt/yofc-config-read-20261101/state.json',
    '/etc/netplan/10-lxc.yaml', '/etc/netplan/90-yofc-managed.yaml',
]


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def baseline():
    return {p: sha(p) for p in BASELINE_PATHS}


def run(arguments, receipt=None, successful=True):
    if receipt:
        assert not (FIXTURE / (receipt + '.log')).exists(), 'preserve the earlier attempt'
    result = subprocess.run(list(map(str, arguments)), text=True, capture_output=True)
    if receipt:
        (FIXTURE / (receipt + '.log')).write_text(result.stdout + result.stderr)
    if successful and result.returncode:
        raise RuntimeError('command failed; inspect private fixture log: ' + str(receipt))
    return result


def retained():
    files = [p for directory in [CONFIG, DATA] for p in directory.rglob('*') if p.is_file()
             and not p.name.endswith(('.sqlite', '.sqlite-wal', '.sqlite-shm'))]
    with sqlite3.connect(DATA / 'results.sqlite') as db:
        rows = db.execute('SELECT command_id,state,request_hash,hex(plan),hex(result) FROM executions ORDER BY command_id').fetchall()
    return {'files': {str(p): sha(p) for p in sorted(files)},
            'executionsSha256': hashlib.sha256(json.dumps(rows).encode()).hexdigest(), 'executionCount': len(rows)}


def stopped():
    assert run(['systemctl', 'show', UNIT, '-p', 'MainPID', '--value']).stdout.strip() == '0'


def status(state):
    assert baseline() == state['baseline'], 'prior fixture or system network input changed'
    assert run(['systemctl', 'is-active', 'yofc-management-phase2-agent.service']).stdout.strip() == 'active'
    return {'phase': state['phase'], 'gateway': GATEWAY, 'baselineUnchanged': True, 'oldAgentActive': True,
            'packageSha256': state.get('packageSha256'), 'systemNetworkWrites': 0,
            'kuraPackageInstalled': False, 'systemdFilesystemSandbox': 'deferred: OrbStack global override'}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('action', choices=['inventory', 'install', 'identity', 'stop', 'guard', 'upgrade',
                                         'remove', 'reinstall', 'resume', 'settle', 'purge', 'status'])
    parser.add_argument('--package', type=Path)
    args = parser.parse_args()
    assert os.geteuid() == 0 and socket.gethostname() == 'yofc-management-network-20261101'
    assert 'VERSION_ID="24.04"' in Path('/etc/os-release').read_text()
    os.umask(0o077)
    if args.action == 'inventory':
        assert not FIXTURE.exists(), 'never replay an existing fixture'
        for path in [CONFIG, DATA, Path('/usr/bin/yofc-management-agent'), Path('/etc/kura-netplan'),
                     Path('/opt/eclipse/kura')]:
            assert not path.exists() and not path.is_symlink(), 'explicit migration required'
        FIXTURE.mkdir(mode=0o700)
        state = {'phase': 'inventoried', 'baseline': baseline()}
    else:
        state = json.loads(STATE.read_text())
    transitions = {'install': ('inventoried', 'installed'), 'identity': ('installed', 'active'),
                   'stop': ('active', 'stopped'), 'guard': ('stopped', 'guarded'),
                   'upgrade': ('guarded', 'upgraded'), 'remove': ('upgraded', 'removed'),
                   'reinstall': ('removed', 'reinstalled'), 'resume': ('reinstalled', 'resumed'),
                   'settle': ('resumed', 'settled'), 'purge': ('settled', 'purged')}
    if args.action in transitions:
        assert state['phase'] == transitions[args.action][0], 'inspect retained fixture before continuing'
    assert not (FIXTURE / (args.action + '.json')).exists() or args.action == 'status'
    result = {}
    if args.action in ['install', 'upgrade', 'reinstall']:
        assert args.package and args.package.is_file()
        digest = sha(args.package)
        if args.action == 'install':
            state['packageSha256'] = digest
            simulation = run(['apt-get', '--simulate', 'install', args.package], 'apt-simulation').stdout
            assert not any('Inst ' + name + ' ' in simulation for name in ['network-manager', 'modemmanager', 'iptables'])
            run(['apt-get', 'install', '-y', args.package], 'install')
            stopped()
            assert not (CONFIG / 'agent.env').exists()
        else:
            assert digest == state['packageSha256']
            run(['dpkg', '--install', args.package], args.action)
            stopped()
            assert retained() == state['retained'], 'identity, key, user data or logical executions changed'
        result['sameCandidate'] = True
    elif args.action == 'identity':
        run(['kura-agent-setup', 'import-identity', '--endpoint', 'https://host.orb.internal:18444',
             '--gateway', GATEWAY, '--ca', IDENTITY / 'ca.pem', '--cert', IDENTITY / 'agent.pem',
             '--key', IDENTITY / 'agent.key'], 'identity-import')
        settings = (CONFIG / 'agent.env').read_text()
        assert 'AGENT_KURA_UID=4294967295\n' in settings and 'AGENT_NETWORK_INTERFACES=\n' in settings
        run(['kura-agent-setup', 'activate'], 'activate')
        assert run(['systemctl', 'is-active', UNIT]).stdout.strip() == 'active'
        result['networkWriteDelegation'] = False
    elif args.action in ['stop', 'settle']:
        run(['systemctl', 'stop', UNIT], args.action)
        stopped()
        # An explicit agent-owned user marker tests ordinary removal preservation.
        marker = DATA / 'user-retention-fixture'
        if args.action == 'stop':
            with marker.open('x') as stream:
                stream.write('task-journal-package-test-only\n')
        state['retained'] = retained()
        result['retained'] = state['retained']
    elif args.action == 'guard':
        stopped()
        assert retained() == state['retained']
        synthetic = str(uuid.uuid4())
        # Metadata injection only, while stopped. No system operation was admitted
        # for this ID and no center task exists. Keep its exact identity in evidence.
        with sqlite3.connect(DATA / 'results.sqlite') as db:
            assert db.execute('SELECT count(*) FROM executions').fetchone()[0] > 0
            db.execute("INSERT INTO executions(command_id,operation,idempotency_key,request_hash,plan,state,result,created_at,updated_at,expires_at) SELECT ?,6,'synthetic-package-guard',request_hash,plan,'UNKNOWN',NULL,created_at,updated_at,expires_at FROM executions LIMIT 1", (synthetic,))
        state['syntheticId'] = synthetic
        STATE.write_text(json.dumps(state, indent=2) + '\n')
        before = retained()
        blocked = []
        for action in ['remove', 'purge']:
            attempt = run(['dpkg', '--' + action, 'kura-management-agent'], 'uncertain-' + action, successful=False)
            assert attempt.returncode != 0 and 'Settle uncertain executions' in attempt.stderr
            assert retained() == before
            blocked.append(action)
        # Remove only the deliberately inserted, unexecuted test metadata. This
        # is not an operator/API path for resolving a real UNKNOWN outcome.
        with sqlite3.connect(DATA / 'results.sqlite') as db:
            deleted = db.execute("DELETE FROM executions WHERE command_id=? AND idempotency_key='synthetic-package-guard' AND state='UNKNOWN'", (synthetic,)).rowcount
            assert deleted == 1
        assert retained() == state['retained']
        result.update(blockedActions=blocked, syntheticId=synthetic, restoredOnlySyntheticMetadata=True)
    elif args.action == 'remove':
        run(['dpkg', '--remove', 'kura-management-agent'], 'remove')
        assert retained() == state['retained']
        assert not Path('/usr/bin/yofc-management-agent').exists()
        result['identityKeyAndUserDataRetained'] = True
    elif args.action == 'resume':
        run(['kura-agent-setup', 'check'], 'reinstalled-check')
        run(['kura-agent-setup', 'activate'], 'reinstalled-activate')
        assert run(['systemctl', 'is-active', UNIT]).stdout.strip() == 'active'
    elif args.action == 'purge':
        stopped()
        with tarfile.open(FIXTURE / 'retained-before-purge.tar.gz', 'x:gz') as archive:
            archive.add(CONFIG, arcname=str(CONFIG).lstrip('/'))
            archive.add(DATA, arcname=str(DATA).lstrip('/'))
        run(['dpkg', '--purge', 'kura-management-agent'], 'purge')
        assert not CONFIG.exists() and not DATA.exists()
        result['agentOwnedDataPurged'] = True
    if args.action in transitions:
        state['phase'] = transitions[args.action][1]
    value = {**status(state), **result}
    if args.action != 'status':
        STATE.write_text(json.dumps(state, indent=2) + '\n')
        with (FIXTURE / (args.action + '.json')).open('x') as stream:
            json.dump(value, stream, indent=2)
            stream.write('\n')
    print(json.dumps(value, indent=2))


if __name__ == '__main__':
    main()
