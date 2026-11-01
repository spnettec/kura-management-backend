#!/usr/bin/env python3
"""Continue the exact retained six-command fixture after its first proven outcome."""
import importlib.util
import json
from pathlib import Path
import time
import uuid

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('outcome', ROOT / 'scripts/phase2-network-outcome-smoke.py')
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)


def main():
    assert not m.OUTPUT.exists()
    prior = m.vm('settled-after-proof')
    assert m.pg("SELECT enabled FROM mgmt_gateways WHERE id='" + m.GATEWAY + "'") == 'f'
    checks, results, task_ids = [], {'retainedPrepare': prior}, []
    c = m.api.Client()

    def check(name, condition):
        assert condition, name
        checks.append(name)

    def task(task_id):
        code, value = c.request('/api/v1/tasks/' + task_id)
        assert code == 200
        return value

    def submit(operation, **payload):
        body = {'operation': operation, 'idempotencyKey': str(uuid.uuid4()), **payload}
        code, value = c.request('/api/v1/gateways/' + m.GATEWAY + '/tasks', 'POST', body)
        check(operation + ' admitted', code == 202)
        task_ids.append(value['id'])
        return m.until(lambda: (v if v['state'] in ['SUCCEEDED', 'FAILED', 'EXPIRED'] else None)
                       if (v := task(value['id'])) else None)

    def epoch():
        return m.pg("SELECT connection_epoch FROM mgmt_gateways WHERE id='" + m.GATEWAY + "'")

    enabled = False
    try:
        m.console('set-gateway-enabled', m.GATEWAY, 'true')
        enabled = True
        old_epoch = epoch()
        m.vm('start')
        m.until(lambda: epoch() and epoch() != old_epoch)
        c.login('admin')
        first = task(prior['reconciledPrepareId'])
        p = first['result']['reconciliation']
        check('real prepare recovery is retained as FAILED/CONFLICT', first['state'] == 'FAILED' and first['error'] == 'CONFLICT')
        check('prepare proof remains bound to task and transaction', p['commandId'] == first['id']
              and p['operation'] == 5 and p['source'] == 'network.journal.v1' and p['state'] == 'ROLLED_BACK'
              and p['transactionId'] == first['result']['id'])
        check('first query attempted no child execution', prior['priorQuery']['queryExecveCount'] == 0)
        check('first query retained business journal and restored full initial baseline',
              prior['priorQuery']['networkJournalUnchanged'] and prior['priorQuery']['initialBaselineRestored'])
        view = submit('network.inspect')
        config = prior['prepare']['config']
        candidate = {**config, 'mtu': 1468}
        pending = submit('network.prepare', networkChange={
            'expectedRevision': view['result']['revision'], 'sourceDigest': view['result']['sourceDigest'],
            'confirmSeconds': 300, 'config': candidate})
        check('resolved uncertainty permits a new candidate', pending['state'] == 'SUCCEEDED'
              and pending['result']['state'] == 'WAIT_CONFIRM')
        actual = m.vm('status')
        check('actual dummy MTU changed for second candidate', actual['mtu'] == 1468 and actual['candidateExists'])
        rolled = submit('network.rollback', networkAction={'transactionId': pending['result']['id'], 'confirmationToken': ''})
        check('real remote rollback completes', rolled['state'] == 'SUCCEEDED' and rolled['result']['state'] == 'ROLLED_BACK')
        m.vm('stop')
        results['rollbackInjection'] = m.vm('inject', '--task', rolled['id'])
        old_epoch = epoch()
        m.vm('start')
        m.until(lambda: epoch() and epoch() != old_epoch)
        current = m.until(lambda: (v if [r for r in v['executions'] if r[0] == rolled['id'] and r[2] == 'UNKNOWN']
                                   else None) if (v := m.vm('status')) else None)
        check('restart retains uncertain rollback receipt', current['state'] == 'ROLLED_BACK' and current['mtu'] == config['mtu'])
        results['rollbackMark'] = m.vm('mark', '--label', 'rollback-receipt')
        changed = m.pg("UPDATE mgmt_tasks SET state='UNKNOWN',error='OUTCOME_UNKNOWN',result=NULL,lease_until=NULL,"
                       "expires_at=now()-interval '1 hour' WHERE id='" + rolled['id']
                       + "' AND gateway_id='" + m.GATEWAY + "' AND state='SUCCEEDED' RETURNING id")
        check('only this fixture rollback receipt is removed at center', changed.splitlines()[0] == rolled['id'])
        resolved = m.until(lambda: (v if v['state'] == 'SUCCEEDED' else None) if (v := task(rolled['id'])) else None)
        proof = resolved['result']['reconciliation']
        check('rollback outcome proof binds original accepting command', proof['source'] == 'network.journal.v1'
              and proof['commandId'] == rolled['id'] and proof['operation'] == 7
              and proof['transactionId'] == rolled['result']['id'] and proof['state'] == 'ROLLED_BACK'
              and resolved['error'] is None and len(proof['journalSha256']) == 64)
        results['rollbackQuery'] = m.vm('verify', '--label', 'rollback-receipt')
        check('rollback query attempts no child and preserves complete baseline',
              results['rollbackQuery']['queryExecveCount'] == 0
              and results['rollbackQuery']['systemBaselineUnchangedDuringObservation'])
        # Reconnect after settlement: query/cached receipts must not add another audit or resolution.
        before = m.vm('status')
        old_epoch = epoch()
        m.vm('stop')
        m.vm('start')
        m.until(lambda: epoch() and epoch() != old_epoch)
        after = m.vm('status')
        check('both resolutions and original executions survive reconnect without additions',
              before['executions'] == after['executions'] and before['resolutionCount'] == after['resolutionCount'] == 2)
        for original in [first['id'], rolled['id']]:
            check('center network proof audit exactly once: ' + original, m.pg(
                "SELECT count(*) FROM mgmt_audit WHERE target='" + original + "' AND operation='task.reconcile.network'") == '1')
            check('center result and input remain encrypted: ' + original, m.pg(
                "SELECT (input ? '$yofcTask') AND (result ? '$yofcTask') FROM mgmt_tasks WHERE id='" + original + "'") == 't')
        check('no new execution for read-only outcome queries', len(after['executions']) == 9)
        healthy = submit('agent.status')
        check('Kura absent and independent management remains available', healthy['state'] == 'SUCCEEDED'
              and healthy['result']['agentAvailable'] and not healthy['result']['kuraAvailable'])
        check('session logout', c.request('/api/v1/auth/logout', 'POST')[0] == 204)
        results['prepareProof'] = p
        results['rollbackProof'] = proof
    finally:
        try:
            results['final'] = m.vm('finish')
        finally:
            if enabled:
                m.console('set-gateway-enabled', m.GATEWAY, 'false')
    report = {'status': 'passed', 'passed': len(checks), 'checks': checks, 'newTaskIds': task_ids,
              'gateway': m.GATEWAY, 'gatewayDisabled': True, 'results': results,
              'fault': 'receipt-loss injection after real dummy-link actions; not actual power loss',
              'priorSettledRecordsPreserved': 6, 'permanentNetworkConfirmation': False,
              'priorFailedAttemptsPreserved': True, 'kuraAbsent': True}
    with m.OUTPUT.open('x') as f:
        json.dump(report, f, indent=2)
        f.write('\n')
    print(json.dumps({'passed': len(checks), 'evidence': str(m.OUTPUT)}))


if __name__ == '__main__':
    main()
