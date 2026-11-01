#!/usr/bin/env python3
"""One-shot, explicitly named Linux fixture; never resets a retained fixture."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import sqlite3
import subprocess
import time
import uuid

DEST = Path('/opt/yofc-network-outcome-20261101')
STATE = DEST / 'state'
NETWORK = STATE / 'network'
UNIT = 'yofc-network-outcome-agent.service'
UNIT_FILE = Path('/etc/systemd/system') / UNIT
OLD_UNIT = 'yofc-management-phase2-agent.service'
OLD_STATE = Path('/var/lib/yofc-management-phase2')
MANAGED = Path('/etc/netplan/90-yofc-managed.yaml')
CANDIDATE = Path('/run/netplan/90-yofc-managed.yaml')
TRACE = DEST / 'execve.log'


def run(*args):
    return subprocess.check_output(args, text=True).strip()


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def save(path, value):
    with Path(path).open('x') as stream:
        json.dump(value, stream, indent=2)
        stream.write('\n')


def journal():
    p = NETWORK / 'transaction.json'
    return json.loads(p.read_text()) if p.exists() else None


def sql(query, args=(), write=False):
    with sqlite3.connect('file:' + str(STATE / 'results.sqlite') + ('?mode=rw' if write else '?mode=ro'), uri=True) as db:
        return db.execute(query, args).fetchall()


def baseline():
    paths = list(Path('/etc/netplan').glob('*.yaml'))
    paths += [p for p in Path('/etc/yofc-management-phase2').rglob('*') if p.is_file()]
    paths += [OLD_STATE / 'network/confirmed.json', OLD_STATE / 'network/transaction.json',
              Path('/opt/yofc-management-phase2/agent')]
    for old in ['/opt/yofc-agent-package-20261101', '/opt/yofc-config-read-20261101',
                '/opt/yofc-kura-netplan-package-20261101', '/opt/yofc-task-journal-package-20261101']:
        paths += list(Path(old).glob('*state*.json'))
    links = []
    for name in ['eth0', 'yofc-test0', 'yofc-adopt0']:
        link = json.loads(run('ip', '-j', 'address', 'show', 'dev', name))[0]
        links.append({'name': name, 'mtu': link['mtu'], 'flags': link['flags'],
                      'addresses': sorted([a['family'], a['local'], a['prefixlen'], a['scope']]
                                          for a in link.get('addr_info', [])),
                      'routes': json.loads(run('ip', '-j', 'route', 'show', 'dev', name))})
    return {'files': {str(p): sha(p) for p in sorted(set(paths))}, 'links': links,
            'oldPid': int(run('systemctl', 'show', OLD_UNIT, '--value', '--property=MainPID')),
            'oldActive': run('systemctl', 'is-active', OLD_UNIT), 'candidateExists': CANDIDATE.exists()}


def stopped():
    return run('systemctl', 'show', UNIT, '--value', '--property=MainPID') == '0'


def start():
    subprocess.run(['systemctl', 'start', UNIT], check=True)
    for _ in range(200):
        if (DEST / 'control.sock').exists() and run('systemctl', 'is-active', UNIT) == 'active':
            return {'active': True}
        time.sleep(.1)
    raise AssertionError('fixture agent not ready')


def stop():
    subprocess.run(['systemctl', 'stop', UNIT], check=True)
    assert stopped(), 'fixture process retained'
    return {'stopped': True}


def prepare(identity, binary):
    assert not DEST.exists() and not UNIT_FILE.exists(), 'retain prior fixture; do not replay'
    assert not CANDIDATE.exists(), 'another network candidate exists'
    old_journal = json.loads((OLD_STATE / 'network/transaction.json').read_text())
    assert old_journal['state'] in ['COMMITTED', 'ROLLED_BACK']
    old_confirmed = json.loads((OLD_STATE / 'network/confirmed.json').read_text())
    assert MANAGED.read_text() == old_confirmed['yaml'], 'baseline drift'
    before = baseline()
    assert before['oldActive'] == 'active' and before['oldPid'] > 0
    DEST.mkdir(mode=0o700)
    STATE.mkdir(mode=0o700)
    NETWORK.mkdir(mode=0o700)
    save(DEST / 'baseline.json', before)
    shutil.copy2(OLD_STATE / 'network/confirmed.json', NETWORK / 'confirmed.json')
    shutil.copy2(binary, DEST / 'agent')
    (DEST / 'agent').chmod(0o700)
    tracer = Path(binary).with_name('network-outcome-linux-trace-exec-20261101')
    shutil.copy2(tracer, DEST / 'trace-exec')
    (DEST / 'trace-exec').chmod(0o700)
    for name in ['agent.pem', 'agent.key', 'ca.pem']:
        shutil.copyfile(Path(identity) / name, DEST / name)
        (DEST / name).chmod(0o600)
    old_env = dict(line.split('=', 1) for line in Path('/etc/yofc-management-phase2/agent.env').read_text().splitlines()
                   if '=' in line and not line.startswith('#'))
    env = {'RUST_LOG': 'info', 'AGENT_CONSOLE_ENDPOINT': old_env['AGENT_CONSOLE_ENDPOINT'],
           'AGENT_GATEWAY_ID': 'network-outcome-gateway-20261101', 'AGENT_KURA_UID': '65534',
           'AGENT_ADMIN_UID': '0', 'AGENT_CA': str(DEST / 'ca.pem'), 'AGENT_CERT': str(DEST / 'agent.pem'),
           'AGENT_KEY': str(DEST / 'agent.key'), 'AGENT_SOCKET': str(DEST / 'control.sock'),
           'AGENT_STATE_DB': str(STATE / 'results.sqlite'), 'AGENT_NETWORK_INTERFACES': 'yofc-test0',
           'AGENT_KURA_NETWORK_INTERFACES': '', 'AGENT_NETWORK_ALLOW_VIRTUAL_FIXTURE': '1'}
    (DEST / 'agent.env').write_text(''.join(k + '=' + v + '\n' for k, v in env.items()))
    (DEST / 'agent.env').chmod(0o600)
    UNIT_FILE.write_text('[Unit]\nDescription=Isolated network outcome receipt-loss injection fixture\n'
                         '[Service]\nType=simple\nEnvironmentFile=' + str(DEST / 'agent.env') + '\n'
                         'ExecStart=' + str(DEST / 'trace-exec') + ' ' + str(TRACE) + ' ' + str(DEST / 'agent') + '\n'
                         'KillMode=control-group\nTimeoutStopSec=10\nRestart=no\nUMask=0077\n')
    subprocess.run(['systemctl', 'daemon-reload'], check=True)
    import yaml
    c = yaml.safe_load(old_confirmed['yaml'])['network']['ethernets']['yofc-test0']
    config = {'interface': 'yofc-test0', 'dhcp4': c.get('dhcp4', False), 'addresses': c.get('addresses', []),
              'routes': [{'to': r['to'], 'via': r.get('via', ''), 'metric': r.get('metric', 0)} for r in c.get('routes', [])],
              'nameservers': c.get('nameservers', {}).get('addresses', []),
              'searchDomains': c.get('nameservers', {}).get('search', []), 'mtu': c.get('mtu', 1500),
              'optional': c.get('optional', False), 'dhcpMetric': c.get('dhcp4-overrides', {}).get('route-metric', 0)}
    report = {'phase': 'prepared', 'gateway': env['AGENT_GATEWAY_ID'], 'binarySha256': sha(DEST / 'agent'),
              'config': config, 'baseline': before, 'testOnly': True, 'kuraPresent': False}
    save(DEST / 'prepare.json', report)
    return report


def status():
    j = journal()
    return {'active': run('systemctl', 'show', UNIT, '--value', '--property=ActiveState'),
            'state': j['state'] if j else None, 'transactionId': j['id'] if j else None,
            'journalSha256': sha(NETWORK / 'transaction.json') if j else None,
            'mtu': int(Path('/sys/class/net/yofc-test0/mtu').read_text()),
            'candidateExists': CANDIDATE.exists(),
            'executions': sql('SELECT command_id,operation,state FROM executions ORDER BY command_id')
                          if (STATE / 'results.sqlite').exists() else [],
            'resolutionCount': sql('SELECT count(*) FROM execution_resolutions')[0][0]
                               if (STATE / 'results.sqlite').exists() else 0}


def inject(task):
    assert stopped(), 'stop only this fixture before receipt-loss injection'
    uuid.UUID(task)
    rows = sql('SELECT operation,state,plan,result FROM executions WHERE command_id=?', (task,))
    assert len(rows) == 1 and rows[0][0] in [5, 7] and rows[0][1] == 'SUCCEEDED'
    assert rows[0][2][0] == 1 and rows[0][3][0] == 1
    assert sql('SELECT count(*) FROM execution_resolutions WHERE command_id=?', (task,))[0][0] == 0
    backup = DEST / ('before-injection-' + task + '.sqlite')
    assert not backup.exists()
    with sqlite3.connect(STATE / 'results.sqlite') as db, sqlite3.connect(backup) as target:
        db.backup(target)
    original_hash = hashlib.sha256(rows[0][2]).hexdigest()
    sql("UPDATE executions SET state='RUNNING',result=NULL WHERE command_id=? AND state='SUCCEEDED'", (task,), True)
    assert hashlib.sha256(sql('SELECT plan FROM executions WHERE command_id=?', (task,))[0][0]).hexdigest() == original_hash
    return {'taskId': task, 'injection': 'real completed action receipt removed; durable encrypted plan preserved',
            'planSha256': original_hash, 'businessJournalUnmodified': True}


def mark(label):
    assert label in ['prepare-recovery', 'rollback-receipt']
    j = journal()
    assert j and j['state'] == 'ROLLED_BACK'
    expected = json.loads((DEST / 'baseline.json').read_text())
    for _ in range(300):
        if baseline() == expected:
            time.sleep(.3)
            if baseline() == expected:
                break
        time.sleep(.1)
    else:
        raise AssertionError('wait for normal networkd/SLAAC convergence before query observation')
    report = {'traceOffset': TRACE.stat().st_size, 'networkFiles': {
        p.name: sha(p) for p in NETWORK.iterdir() if p.is_file()}, 'baseline': baseline(), 'status': status()}
    save(DEST / ('query-before-' + label + '.json'), report)
    return {'marked': label, 'state': j['state'], 'execAttemptsBeforeQuery': TRACE.read_text().count('execve(')}


def verify(label):
    original = json.loads((DEST / ('query-before-' + label + '.json')).read_text())
    with TRACE.open('rb') as f:
        f.seek(original['traceOffset'])
        delta = f.read().decode()
    assert 'execve(' not in delta, 'agent executed a child during outcome query'
    assert original['networkFiles'] == {p.name: sha(p) for p in NETWORK.iterdir() if p.is_file()}
    current = baseline()
    stable = original['baseline'] == current
    convergence = None
    if not stable:
        # Preserve the first failed comparison. Accept only the exact observed
        # return of the initial eth0 SLAAC address, and report that difference.
        initial = json.loads((DEST / 'baseline.json').read_text())
        assert label == 'prepare-recovery' and current == initial
        adjusted = json.loads(json.dumps(original['baseline']))
        old = adjusted['links'][0]['addresses']
        missing = [a for a in initial['links'][0]['addresses'] if a not in old]
        assert missing and all(a[0] == 'inet6' and a[3] == 'global' for a in missing)
        adjusted['links'][0]['addresses'] = sorted(old + missing)
        assert adjusted == initial, 'unexplained baseline change'
        convergence = 'first observation preceded return of original eth0 global IPv6 SLAAC address; exact initial baseline now restored'
    report = {'phase': label, 'queryExecveCount': 0, 'networkJournalUnchanged': True,
              'systemBaselineUnchangedDuringObservation': stable, 'initialBaselineRestored':
                  current == json.loads((DEST / 'baseline.json').read_text()),
              'networkdConvergence': convergence, 'status': status()}
    save(DEST / ('query-after-' + label + '.json'), report)
    (DEST / ('query-execve-' + label + '.log')).write_text(delta)
    return report


def finish():
    if journal() and journal()['state'] not in ['COMMITTED', 'ROLLED_BACK']:
        stop()
        start()  # normal startup recovery; never fabricates a terminal business state
        for _ in range(200):
            if journal()['state'] in ['COMMITTED', 'ROLLED_BACK', 'RECOVERY_REQUIRED']:
                break
            time.sleep(.1)
    assert not journal() or journal()['state'] in ['COMMITTED', 'ROLLED_BACK'], 'recovery remains required'
    stop()
    before = json.loads((DEST / 'baseline.json').read_text())
    after = baseline()
    assert before == after, 'fixture requires attention: baseline differs'
    report = {'phase': 'stopped-data-retained', 'oldBaselineUnchanged': True, 'oldPid': after['oldPid'],
              'oldAgentActive': after['oldActive'] == 'active', 'candidateAbsent': not CANDIDATE.exists(),
              'fixtureStopped': True, 'systemdFilesystemSandbox': 'not tested; OrbStack override retained'}
    if (DEST / 'final-state.json').exists():
        shutil.copyfile(DEST / 'final-state.json', DEST / ('final-state-history-' + str(time.time_ns()) + '.json'))
    (DEST / 'final-state.json').write_text(json.dumps(report, indent=2) + '\n')
    return report


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('step', choices=['prepare', 'prepared', 'settled', 'settled-after-proof', 'start', 'stop', 'status', 'inject', 'mark', 'verify', 'finish'])
    parser.add_argument('--identity')
    parser.add_argument('--binary')
    parser.add_argument('--task')
    parser.add_argument('--label')
    args = parser.parse_args()
    assert os.geteuid() == 0
    os.umask(0o077)
    if args.step == 'prepare':
        result = prepare(args.identity, args.binary)
    elif args.step == 'prepared':
        assert stopped() and journal() is None
        assert sql('SELECT count(*) FROM executions')[0][0] == 0
        assert baseline() == json.loads((DEST / 'baseline.json').read_text())
        result = json.loads((DEST / 'prepare.json').read_text())
    elif args.step == 'settled':
        assert stopped() and journal()['state'] == 'ROLLED_BACK'
        rows = sql('SELECT command_id,operation,state FROM executions ORDER BY command_id')
        assert sorted(r[1] for r in rows) == [1, 4, 5] and all(r[2] == 'SUCCEEDED' for r in rows)
        assert sql('SELECT count(*) FROM execution_resolutions')[0][0] == 0
        assert baseline() == json.loads((DEST / 'baseline.json').read_text())
        result = {'prepare': json.loads((DEST / 'prepare.json').read_text()), 'retainedExecutions': rows}
    elif args.step == 'settled-after-proof':
        assert stopped() and journal()['state'] == 'ROLLED_BACK'
        rows = sql('SELECT command_id,operation,state FROM executions ORDER BY command_id')
        assert sorted(r[1] for r in rows) == [1, 1, 4, 4, 5, 5]
        failed = [r for r in rows if r[2] == 'FAILED']
        assert len(failed) == 1 and failed[0][1] == 5
        assert sql('SELECT command_id FROM execution_resolutions') == [(failed[0][0],)]
        assert baseline() == json.loads((DEST / 'baseline.json').read_text())
        result = {'prepare': json.loads((DEST / 'prepare.json').read_text()), 'retainedExecutions': rows,
                  'reconciledPrepareId': failed[0][0], 'priorQuery': verify('prepare-recovery')}
    elif args.step == 'inject':
        result = inject(args.task)
    elif args.step in ['mark', 'verify']:
        result = globals()[args.step](args.label)
    else:
        result = globals()[args.step]()
    print(json.dumps(result))


if __name__ == '__main__':
    main()
