#!/usr/bin/env python3
"""Backend ownership guards on the named disposable Netplan/networkd VM.

Only fixture files are injected; foreign network profiles are never applied.
The NetworkManager service fixture runs /usr/bin/true, not NetworkManager.
"""
import argparse
import importlib.util
import json
from pathlib import Path
import re
import subprocess
import time
import uuid

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('network_smoke', ROOT / 'scripts/phase2-network-smoke.py')
n = importlib.util.module_from_spec(spec)
spec.loader.exec_module(n)
checks = []
GENERATED = '/run/systemd/network/10-netplan-yofc-test0.network'
UNIT = '/run/systemd/system/NetworkManager.service'
injected = set()
backups = {}
service_fixture = False
initial_transaction = None
evidence_prefix = 'writers-linux'


def check(name, value):
    if not value:
        raise AssertionError(name)
    checks.append(name)
    (ROOT / f'target/{evidence_prefix}-progress.json').write_text(json.dumps({'passed': len(checks), 'checks': checks}, indent=2) + '\n')


def read(path):
    return n.orb('python3', '-c', 'import pathlib,sys;print(pathlib.Path(sys.argv[1]).read_bytes().hex())', path).strip()


def write(path, content, new=False):
    n.orb('python3', '-c', '''import os,pathlib,sys
p=pathlib.Path(sys.argv[1]);new=sys.argv[3]=='1'
if new:assert not p.exists(),str(p)
p.parent.mkdir(parents=True,exist_ok=True)
with p.open('xb' if new else 'wb') as f:f.write(bytes.fromhex(sys.argv[2]))
os.chmod(p,0o600)
''', path, content, '1' if new else '0')
    if new:
        injected.add(path)


def remove_fixture(path):
    assert path in injected, path
    n.orb('python3', '-c', 'import pathlib,sys;pathlib.Path(sys.argv[1]).unlink(missing_ok=True)', path)
    injected.remove(path)


def change(c):
    view = n.remote(c, 'network.inspect')
    return {'expectedRevision': view['revision'], 'sourceDigest': view['sourceDigest'], 'confirmSeconds': 120,
            'config': {'interface': n.IFACE, 'dhcp4': False, 'addresses': ['192.0.2.77/24'],
                       'routes': [], 'nameservers': [], 'searchDomains': [], 'mtu': 1500,
                       'optional': True, 'dhcpMetric': 100}}


def task(c, operation, **extra):
    status, receipt = c.request(f'/api/v1/gateways/{n.GATEWAY}/tasks', 'POST',
                                {'operation': operation, 'idempotencyKey': str(uuid.uuid4()), **extra})
    assert status == 202, (status, receipt)
    return n.m.wait_task(c, receipt['id'])


def denied(c, body, message):
    before = n.local('network.inspect')['transaction']
    result = task(c, 'network.prepare', networkChange=body)
    if result['state'] != 'FAILED' or result['error'] != 'CONFLICT':
        raise AssertionError((message, result))
    check(f'{message}: typed CONFLICT', True)
    check(f'{message}: safe diagnostic', result['result']['diagnostic'].startswith('CONFLICT:'))
    check(f'{message}: no journal mutation', n.local('network.inspect')['transaction'] == before)


def start_service_fixture():
    global service_fixture
    assert 'LoadState=not-found' in n.orb('systemctl', 'show', 'NetworkManager.service', '--property=LoadState')
    content = '[Unit]\nDescription=YOFC writer policy fixture (not NetworkManager)\n[Service]\nType=oneshot\nRemainAfterExit=yes\nExecStart=/usr/bin/true\n'
    write(UNIT, content.encode().hex(), new=True)
    service_fixture = True
    n.orb('systemctl', 'daemon-reload')
    n.orb('systemctl', 'start', 'NetworkManager.service')
    assert n.orb('systemctl', 'is-active', 'NetworkManager.service').strip() == 'active'


def stop_service_fixture():
    global service_fixture
    if service_fixture:
        n.orb('systemctl', 'stop', 'NetworkManager.service')
        remove_fixture(UNIT)
        n.orb('systemctl', 'daemon-reload')
        service_fixture = False


def restore_backend():
    if GENERATED in backups:
        write(GENERATED, backups.pop(GENERATED))


def rollback(c, journal):
    return n.remote(c, 'network.rollback', networkAction=n.action(journal))


def conflict_during_candidate(c, fixture):
    journal = n.prepare(c, '192.0.2.78/24')
    assert journal['state'] == 'WAIT_CONFIRM', journal
    time.sleep(2)  # agent reconnects to the required fresh authenticated stream
    if fixture == 'service':
        start_service_fixture()
    else:
        backups[GENERATED] = read(GENERATED)
        write(GENERATED, backups[GENERATED] + b'\n# yofc writer drift fixture\n'.hex())
    result = task(c, 'network.confirm', networkAction=n.action(journal))
    check(f'{fixture} drift blocks remote confirmation', result['state'] == 'FAILED' and result['error'] == 'CONFLICT')
    blocked = rollback(c, journal)
    check(f'{fixture} drift requires operator recovery', blocked['state'] == 'RECOVERY_REQUIRED')
    if fixture == 'service':
        check('competing service left intact', n.orb('systemctl', 'is-active', 'NetworkManager.service').strip() == 'active')
        stop_service_fixture()
    else:
        check('unknown runtime bytes left intact', read(GENERATED).endswith(b'\n# yofc writer drift fixture\n'.hex()))
        restore_backend()
    recovered = rollback(c, journal)
    check(f'{fixture} correction permits explicit rollback', recovered['state'] == 'ROLLED_BACK' and n.current_ips() == ['192.0.2.2/24'])


def main():
    global initial_transaction, evidence_prefix
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('vm', choices=[n.VM])
    parser.add_argument('--evidence-prefix', default=evidence_prefix)
    args = parser.parse_args()
    if not re.fullmatch(r'[a-z0-9-]+', args.evidence_prefix):
        parser.error('invalid evidence prefix')
    evidence_prefix = args.evidence_prefix
    c = n.m.Client()
    c.login('admin')
    view = n.remote(c, 'network.inspect')
    assert view['revision'] == 2 and n.current_ips() == ['192.0.2.2/24'], view
    initial_transaction = view['transaction']['id'] if view['transaction'] else None
    baseline_files = n.orb('sha256sum', '/etc/netplan/10-lxc.yaml', '/etc/netplan/40-yofc-existing-fixture.yaml', '/etc/netplan/90-yofc-managed.yaml')
    baseline_default = n.orb('ip', '-j', '-4', 'route', 'show', 'default')

    def adopted():
        row = json.loads(n.orb('ip', '-j', 'address', 'show', 'dev', 'yofc-adopt0'))[0]
        return {'mtu': row['mtu'], 'addresses': sorted(f"{a['local']}/{a['prefixlen']}" for a in row['addr_info'] if a['scope'] == 'global')}

    baseline_adopted = adopted()
    journal = n.prepare(c, '192.0.2.77/24')
    check('real generated backend passes ordinary prepare', journal['state'] == 'WAIT_CONFIRM' and n.current_ips() == ['192.0.2.77/24'])
    check('baseline explicit rollback', rollback(c, journal)['state'] == 'ROLLED_BACK')

    body = change(c)
    path = '/run/systemd/network/01-yofc-writer-fixture.network'
    content = b'[Match]\nName=yofc-unused-writer-fixture\n[Network]\nDHCP=no\n'.hex()
    write(path, content, new=True)
    denied(c, body, 'foreign runtime backend')
    check('foreign runtime backend not overwritten', read(path) == content)
    remove_fixture(path)

    backups[GENERATED] = read(GENERATED)
    write(GENERATED, backups[GENERATED] + b'\n# yofc writer drift fixture\n'.hex())
    denied(c, body, 'forged legitimate generated filename')
    check('generated filename drift not overwritten', read(GENERATED).endswith(b'\n# yofc writer drift fixture\n'.hex()))
    restore_backend()

    for path, content, label in [
        ('/run/systemd/network/10-netplan-yofc-test0.network.d/50-yofc-fixture.conf', b'[Network]\n# fixture\n', 'runtime drop-in'),
        ('/etc/systemd/network/01-yofc-writer-fixture.network', b'[Match]\nName=yofc-unused-writer-fixture\n', 'local backend'),
        ('/run/NetworkManager/system-connections/yofc-writer-fixture.nmconnection', b'# not loaded; writer guard fixture\n', 'runtime NM connection'),
    ]:
        write(path, content.hex(), new=True)
        denied(c, body, label)
        check(f'{label}: fixture intact', read(path) == content.hex())
        remove_fixture(path)
        if '.network.d/' in path:
            n.orb('rmdir', str(Path(path).parent))

    start_service_fixture()
    denied(c, body, 'active competing service fixture')
    stop_service_fixture()
    conflict_during_candidate(c, 'service')
    conflict_during_candidate(c, 'runtime')

    journal = n.prepare(c, '192.0.2.79/24')
    n.orb('systemctl', 'stop', 'yofc-management-phase2-agent.service')
    candidate = '/run/netplan/90-yofc-managed.yaml'
    n.orb('python3', '-c', 'import pathlib,sys; p=pathlib.Path(sys.argv[1]);assert p.exists();p.unlink()', candidate)
    n.orb('systemctl', 'start', 'yofc-management-phase2-agent.service')
    recovered = n.wait_state('ROLLED_BACK')
    check('journal attributable candidate backend recovers after source disappearance', recovered['revision'] == 2 and n.current_ips() == ['192.0.2.2/24'])
    check('persistent sources unchanged', n.orb('sha256sum', '/etc/netplan/10-lxc.yaml', '/etc/netplan/40-yofc-existing-fixture.yaml', '/etc/netplan/90-yofc-managed.yaml') == baseline_files)
    check('adopted IPv4 IPv6 and MTU unchanged', adopted() == baseline_adopted)
    check('management default route unchanged', n.orb('ip', '-j', '-4', 'route', 'show', 'default') == baseline_default)
    check('runtime candidate removed', n.orb('test', '!', '-e', candidate) == '')
    check('fixture Kura absence does not block management', n.local('agent.status')['kuraAvailable'] is False)
    c.request('/api/v1/auth/logout', 'POST', {})
    output = {'passed': len(checks), 'checks': checks, 'vm': n.VM, 'revision': 2,
              'agentSha256': n.orb('sha256sum', '/opt/yofc-management-phase2/agent').split()[0],
              'limitations': ['NetworkManager policy fixture uses /usr/bin/true; not real NM handover',
                              'no foreign network profile was applied', 'dummy links, not physical hardware']}
    (ROOT / f'target/{evidence_prefix}.json').write_text(json.dumps(output, indent=2) + '\n')
    print(json.dumps(output, indent=2))


if __name__ == '__main__':
    try:
        main()
    finally:
        stop_service_fixture()
        restore_backend()
        for path in list(injected):
            remove_fixture(path)
        # Empty fixture drop-in directory is ours; never remove other directories.
        subprocess.run(['orb', 'run', '-m', n.VM, '-u', 'root', 'rmdir', '/run/systemd/network/10-netplan-yofc-test0.network.d'], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        if initial_transaction is not None:
            current = n.local('network.inspect')['transaction']
            if current and current['id'] != initial_transaction and current['state'] not in ['ROLLED_BACK', 'COMMITTED']:
                recovered = n.local('network.rollback', n.action(current))
                assert recovered['state'] == 'ROLLED_BACK', recovered
