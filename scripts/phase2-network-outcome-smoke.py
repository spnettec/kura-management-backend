#!/usr/bin/env python3
"""Real dummy-link actions followed by receipt-loss injection, not power failure."""
import hashlib
import argparse
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import time
import uuid

ROOT = Path(__file__).resolve().parents[1]
VM = 'yofc-management-network-20261101'
GATEWAY = 'network-outcome-gateway-20261101'
NAME = 'network-outcome-20261101'
OUTPUT = ROOT / 'target/network-outcome-linux-live-20261101.json'


def module(name, filename):
    spec = importlib.util.spec_from_file_location(name, ROOT / 'scripts' / filename)
    value = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(value)
    return value


api = module('outcome_api', 'phase1-smoke.py')
dev = module('outcome_dev', 'phase1-dev.py')


def until(action, seconds=30):
    deadline = time.monotonic() + seconds
    while time.monotonic() < deadline:
        value = action()
        if value:
            return value
        time.sleep(.2)
    raise AssertionError('condition timed out')


def vm(step, *args):
    result = subprocess.run(['orb', 'run', '-m', VM, '-u', 'root', 'python3',
                             '/mnt/mac' + str(ROOT / 'scripts/phase2-network-outcome-vm.py'), step, *map(str, args)],
                            capture_output=True, text=True)
    if result.returncode:
        raise RuntimeError('fixture step ' + step + ': ' + result.stderr[-3000:])
    return json.loads(result.stdout)


def pg(query):
    return subprocess.run([str(dev.PG / 'psql'), '-X', '-qAt', '-v', 'ON_ERROR_STOP=1',
                           '-h', str(dev.DEV / 'pgsock'), '-p', str(dev.PG_PORT), '-d', 'management'],
                          input=query, text=True, capture_output=True, check=True).stdout.strip()


def console(*args):
    subprocess.run([str(ROOT / 'target/debug/yofc-management-console'), *map(str, args)],
                   env={**os.environ, **dev.env()}, check=True, stdout=subprocess.DEVNULL)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--resume-before-actions', action='store_true',
                        help='Continue the retained identity only after a proven zero-execution setup failure')
    parser.add_argument('--continue-settled-fixture', action='store_true',
                        help='Continue after the three recorded original commands have settled; never replay them')
    args = parser.parse_args()
    os.umask(0o077)
    fixture = dev.DEV / NAME
    assert not OUTPUT.exists(), 'retain previous evidence'
    assert not (args.resume_before_actions and args.continue_settled_fixture)
    continuing = args.resume_before_actions or args.continue_settled_fixture
    retained_count = 0
    if args.continue_settled_fixture:
        settled = vm('settled')
        assert pg("SELECT enabled FROM mgmt_gateways WHERE id='" + GATEWAY + "'") == 'f'
        resumed_prepare = settled['prepare']
        retained_count = len(settled['retainedExecutions'])
        with (fixture / 'settled-continuation.json').open('x') as stream:
            json.dump(settled, stream)
    elif args.resume_before_actions:
        previous = json.loads((fixture / 'final-state.json').read_text())
        assert previous['checksCompleted'] == 2 and previous['gatewayDisabled'] and not previous['evidenceExists']
        assert previous['vmFinal']['fixtureStopped'] and previous['vmFinal']['oldBaselineUnchanged']
        assert not (fixture / 'first-attempt-state.json').exists()
        shutil.copyfile(fixture / 'final-state.json', fixture / 'first-attempt-state.json')
        resumed_prepare = vm('prepared')
    else:
        assert not fixture.exists(), 'retain previous fixture; explicit guarded continuation required'
    assert pg("SELECT count(*) FROM mgmt_tasks WHERE gateway_id='network-gateway' "
              "AND state IN ('QUEUED','RUNNING','UNKNOWN') AND operation IN (5,6,7,10)") == '0'
    if not continuing:
        fixture.mkdir(mode=0o700)
    client = api.Client()
    checks, results, tasks = [], {}, []
    registered = prepared = False

    def check(name, value):
        assert value, name
        checks.append(name)

    def receipt(task_id):
        status, value = client.request('/api/v1/tasks/' + task_id)
        assert status == 200
        return value

    def wait(task_id):
        return until(lambda: (v if v['state'] in ['SUCCEEDED', 'FAILED', 'EXPIRED'] else None)
                     if (v := receipt(task_id)) else None)

    def submit(operation, **payload):
        body = {'operation': operation, 'idempotencyKey': str(uuid.uuid4()), **payload}
        status, task = client.request('/api/v1/gateways/' + GATEWAY + '/tasks', 'POST', body)
        check(operation + ' admitted', status == 202)
        tasks.append(task['id'])
        return wait(task['id']), body

    def epoch():
        return pg("SELECT connection_epoch FROM mgmt_gateways WHERE id='" + GATEWAY + "'")

    def inject_and_reconcile(task, expected, label):
        vm('stop')
        results[label + 'Injection'] = vm('inject', '--task', task['id'])
        original_epoch = epoch()
        vm('start')
        until(lambda: epoch() and epoch() != original_epoch)
        ready = until(lambda: (s if s['state'] == 'ROLLED_BACK'
                              and [r for r in s['executions'] if r[0] == task['id'] and r[2] == 'UNKNOWN']
                              else None) if (s := vm('status')) else None)
        check(label + ' startup preserves UNKNOWN admission', ready['state'] == 'ROLLED_BACK')
        check(label + ' runtime returned to baseline', ready['mtu'] == config['mtu'] and not ready['candidateExists'])
        results[label + 'Mark'] = vm('mark', '--label', label)
        # Only after startup recovery has settled: query itself must execute no child.
        changed = pg("UPDATE mgmt_tasks SET state='UNKNOWN',error='OUTCOME_UNKNOWN',result=NULL,"
                     "lease_until=NULL,expires_at=now()-interval '1 hour' WHERE id='" + task['id']
                     + "' AND gateway_id='" + GATEWAY + "' AND state='SUCCEEDED' RETURNING id;")
        check(label + ' scoped center receipt-loss injection', changed.splitlines()[0] == task['id'])
        resolved = until(lambda: (v if v['state'] == expected else None)
                         if (v := receipt(task['id'])) else None)
        proof = resolved['result']['reconciliation']
        check(label + ' proof bound to original operation and task', proof['source'] == 'network.journal.v1'
              and proof['commandId'] == task['id'] and proof['operation'] == task['operation']
              and proof['transactionId'] == task['result']['id'] and proof['state'] == 'ROLLED_BACK'
              and len(proof['journalSha256']) == 64 and proof['observedAtUnixMs'] > 0)
        check(label + ' typed terminal result', resolved['state'] == expected
              and resolved['error'] == ('CONFLICT' if expected == 'FAILED' else None))
        count = pg("SELECT count(*) FROM mgmt_audit WHERE target='" + task['id']
                   + "' AND operation='task.reconcile.network'")
        check(label + ' center audits reconciliation once', count == '1')
        check(label + ' center retains encrypted input and result', pg(
            "SELECT (input ? '$yofcTask') AND (result ? '$yofcTask') FROM mgmt_tasks WHERE id='" + task['id'] + "'") == 't')
        results[label + 'Query'] = vm('verify', '--label', label)
        results[label + 'Proof'] = {'taskId': task['id'], 'taskState': resolved['state'],
                                  'error': resolved['error'], 'proof': proof}
        return resolved

    try:
        if continuing:
            results['prepare'] = resumed_prepare
            prepared = True
            console('set-gateway-enabled', GATEWAY, 'true')
            registered = True
        else:
            ext = fixture / 'client.ext'
            ext.write_text('basicConstraints=critical,CA:FALSE\nkeyUsage=critical,digitalSignature\n'
                           'extendedKeyUsage=clientAuth\nsubjectKeyIdentifier=hash\nauthorityKeyIdentifier=keyid,issuer\n')
            for cmd in [
            ['openssl', 'req', '-newkey', 'rsa:2048', '-nodes', '-subj', '/CN=' + GATEWAY,
             '-keyout', str(fixture / 'agent.key'), '-out', str(fixture / 'agent.csr')],
            ['openssl', 'x509', '-req', '-in', str(fixture / 'agent.csr'), '-CA', str(dev.DEV / 'ca.pem'),
             '-CAkey', str(dev.DEV / 'ca.key'), '-set_serial', '0x' + uuid.uuid4().hex, '-days', '3',
             '-extfile', str(ext), '-out', str(fixture / 'agent.pem')],
            ]:
                subprocess.run(cmd, check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
            shutil.copyfile(dev.DEV / 'ca.pem', fixture / 'ca.pem')
            console('register-gateway', GATEWAY, 'dev', 'Network outcome receipt-loss fixture', fixture / 'agent.pem')
            registered = True
            results['prepare'] = vm('prepare', '--identity', '/mnt/mac' + str(fixture),
                                    '--binary', '/mnt/mac' + str(ROOT / 'target/network-outcome-linux-agent-20261101'))
            prepared = True
        config = results['prepare']['config']
        check('only approved dummy interface delegated', config['interface'] == 'yofc-test0' and config['mtu'] == 1500)
        vm('start')
        until(epoch)
        check('new agent advertises network outcome proof', pg(
            "SELECT capabilities ? 'task.network-outcome.v1' FROM mgmt_gateways WHERE id='" + GATEWAY + "'") == 't')
        client.login('admin')
        healthy, _ = submit('agent.status')
        check('Kura absent while independent agent available', healthy['state'] == 'SUCCEEDED'
              and healthy['result']['agentAvailable'] and not healthy['result']['kuraAvailable'])
        view, _ = submit('network.inspect')
        revision = view['result']['revision']
        candidate = {**config, 'mtu': 1468}
        change = {'expectedRevision': revision, 'sourceDigest': view['result']['sourceDigest'],
                  'confirmSeconds': 300, 'config': candidate}
        pending, body = submit('network.prepare', networkChange=change)
        check('real candidate waits for confirmation', pending['state'] == 'SUCCEEDED'
              and pending['result']['state'] == 'WAIT_CONFIRM')
        actual = vm('status')
        check('actual dummy MTU changed temporarily', actual['mtu'] == 1468 and actual['candidateExists'])
        resolved = inject_and_reconcile(pending, 'FAILED', 'prepare-recovery')
        status, replay = client.request('/api/v1/gateways/' + GATEWAY + '/tasks', 'POST', body)
        check('idempotent replay retains failed original task', status == 202
              and replay['id'] == resolved['id'] and replay['state'] == 'FAILED')
        view, _ = submit('network.inspect')
        check('rollback preserves confirmed revision', view['result']['revision'] == revision)
        second, _ = submit('network.prepare', networkChange={**change, 'sourceDigest': view['result']['sourceDigest']})
        check('resolved uncertainty permits a new write', second['state'] == 'SUCCEEDED'
              and second['result']['state'] == 'WAIT_CONFIRM')
        rolled, _ = submit('network.rollback', networkAction={
            'transactionId': second['result']['id'], 'confirmationToken': ''})
        check('explicit remote rollback completed', rolled['state'] == 'SUCCEEDED'
              and rolled['result']['state'] == 'ROLLED_BACK')
        inject_and_reconcile(rolled, 'SUCCEEDED', 'rollback-receipt')
        final = vm('status')
        check('each interrupted command resolved locally once', final['resolutionCount'] == 2)
        check('six new original commands executed; prior settled records retained', len(final['executions']) == retained_count + 6)
        healthy, _ = submit('agent.status')
        check('independent management remains available after both resolutions', healthy['state'] == 'SUCCEEDED'
              and healthy['result']['agentAvailable'] and not healthy['result']['kuraAvailable'])
        check('session logout', client.request('/api/v1/auth/logout', 'POST')[0] == 204)
    finally:
        try:
            if prepared:
                results['final'] = vm('finish')
        finally:
            if registered:
                console('set-gateway-enabled', GATEWAY, 'false')
            (fixture / 'final-state.json').write_text(json.dumps({'gatewayDisabled': registered,
                 'checksCompleted': len(checks), 'vmFinal': results.get('final'), 'evidenceExists': OUTPUT.exists()}) + '\n')
    result = {'status': 'passed', 'passed': len(checks), 'checks': checks, 'taskIds': tasks, 'gateway': GATEWAY,
              'transport': 'real HTTPS/PostgreSQL/mTLS; Linux agent; real Netplan and dummy link',
              'fault': 'receipt-loss injection after actual actions; not power loss or exact crash-point test',
              'networkActions': ['temporary MTU candidate twice', 'startup recovery rollback', 'explicit rollback'],
              'permanentNetworkConfirmation': False, 'kuraAbsent': True, 'gatewayDisabled': True,
              'priorSettledExecutionsRetained': retained_count, 'results': results}
    with OUTPUT.open('x') as f:
        json.dump(result, f, indent=2)
        f.write('\n')
    print(json.dumps({'passed': len(checks), 'evidence': str(OUTPUT)}))


if __name__ == '__main__':
    main()
