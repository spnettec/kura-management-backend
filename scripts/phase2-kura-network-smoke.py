#!/usr/bin/env python3
"""Explicit registered-Kura role against the existing disposable Linux dummy link.

This is not a Linux Kura JVM/provider test. It never changes eth0 or provisions identity.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import socket
import struct
import subprocess
import time
import uuid

ROOT = Path(__file__).resolve().parents[1]
VM = 'yofc-management-network-20261101'
UNIT = 'yofc-management-phase2-agent.service'
IFACE = 'yofc-test0'
SOCKET = '/run/yofc-management-phase2/control.sock'
EVIDENCE = ROOT / 'target/phase2-kura-network-linux-role.json'


def run(*args):
    return subprocess.run(args, check=True, capture_output=True, text=True).stdout


class Peer:
    def __init__(self, registered=False):
        self.stream = socket.socket(socket.AF_UNIX)
        self.stream.settimeout(40)
        self.stream.connect(SOCKET)
        if registered:
            assert self.call('kura.register', {'methods': ['kura.configuration.list', 'kura.snapshot.list']}, kind='register')['result']['registered']

    def close(self):
        self.stream.close()

    def read(self, length):
        result = b''
        while len(result) < length:
            value = self.stream.recv(length - len(result))
            if not value:
                raise EOFError('local peer closed')
            result += value
        return result

    def call(self, method, params=None, kind='request'):
        request_id = str(uuid.uuid4())
        frame = {'version': 1, 'kind': kind, 'requestId': request_id, 'method': method}
        if params is not None:
            frame['params'] = params
        payload = json.dumps(frame).encode()
        self.stream.sendall(struct.pack('>I', len(payload)) + payload)
        length = struct.unpack('>I', self.read(4))[0]
        assert length <= 1024 * 1024
        response = json.loads(self.read(length))
        assert response['requestId'] == request_id and response['method'] == method
        return response

    def result(self, method, params=None):
        reply = self.call(method, params)
        assert not reply.get('error'), reply
        return reply['result']


def ready():
    deadline = time.monotonic() + 30
    while time.monotonic() < deadline:
        try:
            peer = Peer()
            peer.result('agent.status')
            peer.close()
            return
        except (OSError, EOFError):
            time.sleep(.2)
    raise RuntimeError('agent socket did not become ready')


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def inside():
    assert os.geteuid() == 0 and '24.04' in Path('/etc/os-release').read_text()
    assert not EVIDENCE.exists(), 'retain previous evidence instead of replaying this fixture'
    env_path = Path('/etc/yofc-management-phase2/agent.env')
    previous_env = env_path.read_bytes()
    lines = previous_env.decode().splitlines()
    allowed = next(x.split('=', 1)[1] for x in lines if x.startswith('AGENT_NETWORK_INTERFACES='))
    assert set(allowed.split(',')) == {'yofc-test0', 'yofc-adopt0'}
    assert 'AGENT_KURA_UID=65534' in lines
    assert not any(x.startswith('AGENT_KURA_NETWORK_INTERFACES=') for x in lines)
    binary = Path('/opt/yofc-management-phase2/agent')
    source = Path('/mnt/machines/kura-package-validation-20261029/home/heyoulin/.cache/yofc-management-phase1/target-agent/debug/yofc-management-agent')
    old_sha = sha(binary)
    backup = binary.with_name('agent-backup-' + old_sha[:16])
    if backup.exists():
        assert sha(backup) == old_sha
    else:
        shutil.copy2(binary, backup)
    env_backup = Path('/opt/yofc-management-phase2/kura-role-original.env')
    assert not env_backup.exists(), 'inspect previous role fixture before restarting it'
    env_backup.write_bytes(previous_env);env_backup.chmod(0o600)
    original_eth0 = sha('/etc/netplan/10-lxc.yaml')
    original_route = run('ip', '-j', '-4', 'route', 'show', 'default')
    original_source = sha('/etc/netplan/90-yofc-managed.yaml')
    checks = []
    output = {'vm': VM, 'syntheticRegisteredKuraRole': True, 'javaProviderLinuxValidated': False,
              'oldAgentSha256': old_sha, 'agentBackup': str(backup), 'newAgentSha256': sha(source)}
    peer = admin = None
    pending = None
    actor = None

    def check(name, condition):
        if not condition:
            raise AssertionError(name)
        checks.append(name)

    def ips():
        rows = json.loads(run('ip', '-j', '-4', 'address', 'show', 'dev', IFACE))
        return [f"{a['local']}/{a['prefixlen']}" for row in rows for a in row['addr_info']]

    def change(address):
        view = admin.result('network.inspect')
        return {'expectedRevision': view['revision'], 'sourceDigest': view['sourceDigest'], 'confirmSeconds': 120,
                'config': {'interface': IFACE, 'dhcp4': False, 'addresses': [address], 'routes': [],
                           'nameservers': [], 'searchDomains': [], 'mtu': 1500, 'optional': True, 'dhcpMetric': 100}}

    def action(transaction):
        return {'transactionId': transaction['id'], 'confirmationToken': transaction['confirmationToken']}

    try:
        run('systemctl', 'stop', UNIT)
        shutil.copy2(source, binary)
        env_path.write_text(previous_env.decode().replace('AGENT_KURA_UID=65534', 'AGENT_KURA_UID=0')
                            + '\nAGENT_KURA_NETWORK_INTERFACES=yofc-test0\n')
        run('systemctl', 'start', UNIT);ready()
        peer = Peer(True);admin = Peer()
        view = peer.result('network.kura.view')
        baseline = view['revision'];output['baselineRevision'] = baseline
        check('registered role sees actual confirmed state', baseline >= 2 and ips() == ['192.0.2.2/24'])
        check('same root admin connection cannot impersonate registered API', admin.call('network.kura.view')['error'] == 'FORBIDDEN')
        check('registered role cannot inherit generic admin methods', peer.call('network.inspect')['error'] == 'FORBIDDEN')
        check('registered role has no source takeover method', peer.call('network.kura.takeover.prepare')['error'] == 'UNSUPPORTED')
        unauthorized = change('192.0.2.7/24');unauthorized['config']['interface'] = 'yofc-adopt0'
        check('nondelegated adopted interface cannot be modified', peer.call('network.kura.prepare', unauthorized)['error'] == 'FORBIDDEN')
        pending = peer.result('network.kura.prepare', change('192.0.2.7/24'));actor = peer
        check('real local candidate is awaiting confirmation', pending['state'] == 'WAIT_CONFIRM' and ips() == ['192.0.2.7/24'])
        view = peer.result('network.kura.view')
        config = next(x['config'] for x in view['configurations'] if x.get('config', {}).get('interface') == IFACE)
        check('candidate is separate from confirmed mirror', view['revision'] == baseline and config['addresses'] == ['192.0.2.2/24'])
        check('own transaction can be queried without a confirmation token', peer.result('network.kura.transaction', {'transactionId': pending['id']})['state'] == 'WAIT_CONFIRM')
        wrong = action(pending);wrong['confirmationToken'] = 'wrong'
        check('invalid token cannot roll back', peer.call('network.kura.rollback', wrong)['error'] == 'FORBIDDEN')
        rolled = peer.result('network.kura.rollback', action(pending));pending = None
        check('explicit local rollback restores actual network', rolled['state'] == 'ROLLED_BACK' and ips() == ['192.0.2.2/24'])
        pending = admin.result('network.prepare', change('192.0.2.9/24'));actor = admin
        other = peer.result('network.kura.view')['transaction']
        check('other actor token is hidden', not other['owned'] and 'confirmationToken' not in other)
        check('other actor transaction cannot be rolled back even with known token', peer.call('network.kura.rollback', action(pending))['error'] == 'FORBIDDEN')
        admin.result('network.rollback', action(pending));pending = None
        # Confirm the identical baseline: prove the real local confirmation path
        # without leaving a changed address or a modified persistent YAML source.
        pending = peer.result('network.kura.prepare', change('192.0.2.2/24'));actor = peer
        committed = peer.result('network.kura.confirm', action(pending));pending = None
        check('explicit local confirmation commits one revision', committed['state'] == 'COMMITTED' and committed['revision'] == baseline + 1)
        check('baseline address and persistent source remain identical', ips() == ['192.0.2.2/24'] and sha('/etc/netplan/90-yofc-managed.yaml') == original_source)
        check('eth0 source and management route remain identical', sha('/etc/netplan/10-lxc.yaml') == original_eth0 and run('ip', '-j', '-4', 'route', 'show', 'default') == original_route)
        output['finalRevision'] = committed['revision']
        output['status'] = 'passed'
    except BaseException as error:
        output.update(status='failed', error=str(error))
        raise
    finally:
        if pending is not None and actor is not None:
            method = 'network.kura.rollback' if actor is peer else 'network.rollback'
            try:
                actor.result(method, action(pending))
            except Exception as error:
                output['rollbackError'] = str(error)
        if peer is not None:
            peer.close()
        if admin is not None:
            admin.close()
        run('systemctl', 'stop', UNIT)
        env_path.write_bytes(previous_env)
        run('systemctl', 'start', UNIT);ready()
        check('original UID and disabled Kura delegation are restored', env_path.read_bytes() == previous_env)
        observer = Peer()
        check('agent remains available after synthetic Kura leaves', observer.result('agent.status')['kuraAvailable'] is False)
        observer.close()
        output.update(checks=checks, passed=len(checks), environmentRestored=env_path.read_bytes()==previous_env)
        EVIDENCE.write_text(json.dumps(output, ensure_ascii=False, indent=2) + '\n')
    print(json.dumps(output, ensure_ascii=False, indent=2))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('vm', choices=[VM], nargs='?')
    parser.add_argument('--inside-fixture', action='store_true')
    arguments = parser.parse_args()
    if arguments.inside_fixture:
        inside()
    elif arguments.vm:
        subprocess.run(['orb', 'run', '-m', VM, '-u', 'root', 'python3',
                        '/mnt/mac' + str(Path(__file__).resolve()), '--inside-fixture'], check=True)
    else:
        parser.error('the explicit disposable VM name is required')
