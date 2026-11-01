#!/usr/bin/env python3
"""Modify then roll back an already transferred link in the disposable VM."""
import importlib.util
import argparse
import json
from pathlib import Path
import sys
import re

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('network_smoke', ROOT / 'scripts/phase2-network-smoke.py')
n = importlib.util.module_from_spec(spec)
spec.loader.exec_module(n)
n.IFACE = 'yofc-adopt0'
checks = []


def check(name, value):
    if not value:
        raise AssertionError(name)
    checks.append(name)


def runtime():
    row = json.loads(n.orb('ip', '-j', 'address', 'show', 'dev', n.IFACE))[0]
    return {'mtu': row['mtu'], 'addresses': sorted(a['local'] for a in row['addr_info']),
            'routes': json.loads(n.orb('ip', '-j', '-4', 'route', 'show', 'dev', n.IFACE))}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('vm', choices=[n.VM])
    parser.add_argument('--evidence-prefix', default='takeover-adopted-link-regression')
    args = parser.parse_args()
    if not re.fullmatch(r'[a-z0-9-]+', args.evidence_prefix):
        parser.error('invalid evidence prefix')
    before = runtime()
    source = n.orb('sha256sum', '/etc/netplan/40-yofc-existing-fixture.yaml', '/etc/netplan/90-yofc-managed.yaml')
    view = n.local('network.inspect')
    check('already transferred fixture has revision 2', view['revision'] == 2 and '2001:db8::90' in before['addresses'])
    c = n.m.Client()
    c.login('admin')
    j = n.prepare(c, '192.0.2.91/24')
    current = runtime()
    check('ordinary remote prepare updates adopted IPv4 and MTU', j['state'] == 'WAIT_CONFIRM' and '192.0.2.91' in current['addresses'] and current['mtu'] == 1500)
    check('ordinary update preserves adopted IPv6', '2001:db8::90' in current['addresses'])
    check('unconfirmed update leaves persistent files unchanged', n.orb('sha256sum', '/etc/netplan/40-yofc-existing-fixture.yaml', '/etc/netplan/90-yofc-managed.yaml') == source)
    j = n.remote(c, 'network.rollback', networkAction=n.action(j))
    check('rollback restores adopted addresses MTU and routes', j['state'] == 'ROLLED_BACK' and runtime() == before)
    check('confirmed revision remains 2', n.local('network.inspect')['revision'] == 2)
    check('other managed interface preserved', '192.0.2.2' in n.orb('ip', '-j', '-4', 'address', 'show', 'dev', 'yofc-test0'))
    output = {'passed': len(checks), 'checks': checks, 'vm': n.VM, 'revision': 2,
              'limitations': ['static dummy link only; DHCP resolver and physical matrix pending']}
    (ROOT / f'target/{args.evidence_prefix}.json').write_text(json.dumps(output, indent=2) + '\n')
    print(json.dumps(output, indent=2))


if __name__ == '__main__':
    main()
