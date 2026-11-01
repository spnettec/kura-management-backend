#!/usr/bin/env python3
"""Read-only live gateway tasks through encrypted center storage; no values exported."""
import argparse
import importlib.util
import json
from pathlib import Path
import subprocess
import time
import uuid

ROOT = Path(__file__).resolve().parents[1]


def module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    result = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(result)
    return result


api = module('storage_api', ROOT / 'scripts/phase1-smoke.py')
dev = module('storage_dev', ROOT / 'scripts/phase1-dev.py')


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--gateway', required=True)
    parser.add_argument('--evidence', required=True)
    args = parser.parse_args()
    output = ROOT / args.evidence
    assert not output.exists(), 'retain previous evidence'
    checks = []

    def check(name, passed):
        assert passed, name
        checks.append(name)

    def pg(query):
        return subprocess.run([str(dev.PG / 'psql'), '-X', '-qAt', '-v', 'ON_ERROR_STOP=1',
                               '-h', str(dev.DEV / 'pgsock'), '-p', str(dev.PG_PORT), '-d', 'management'],
                              input=query, text=True, capture_output=True, check=True).stdout.strip()

    client = api.Client()
    client.login('admin')
    task_ids = []
    try:
        status, gateway = client.request('/api/v1/gateways/' + args.gateway)
        check('gateway reports a real live heartbeat', status == 200 and gateway['online'])
        for operation, code in [('agent.status', 1), ('network.inspect', 4)]:
            key = str(uuid.uuid4())
            status, accepted = client.request('/api/v1/gateways/' + args.gateway + '/tasks', 'POST',
                                             {'operation': operation, 'idempotencyKey': key})
            check(operation + ' admitted', status == 202)
            task_id = accepted['id']
            task_ids.append(task_id)
            for _ in range(100):
                status, task = client.request('/api/v1/tasks/' + task_id)
                assert status == 200
                if task['state'] not in ['QUEUED', 'RUNNING']:
                    break
                time.sleep(.2)
            check(operation + ' returns decrypted typed result', task['state'] == 'SUCCEEDED'
                  and task['operation'] == code and isinstance(task['result'], dict)
                  and '$yofcTask' not in task['result'])
            check(operation + ' remains ciphertext in PostgreSQL', pg(
                "SELECT (input ? '$yofcTask') AND (result ? '$yofcTask') FROM mgmt_tasks WHERE id='" + task_id + "'") == 't')
            status, replay = client.request('/api/v1/gateways/' + args.gateway + '/tasks', 'POST',
                                           {'operation': operation, 'idempotencyKey': key})
            check(operation + ' preserves encrypted idempotency', status == 202 and replay['id'] == task_id)
            if operation == 'agent.status':
                check('independent agent remains available', task['result']['agentAvailable'])
                check('Kura absence is reported truthfully', not task['result']['kuraAvailable'])
        viewer = api.Client()
        viewer.login('viewer')
        check('viewer can read allowed tenant receipt', viewer.request('/api/v1/tasks/' + task_ids[0])[0] == 200)
        other = api.Client()
        other.login('other')
        check('other tenant cannot read by UUID', other.request('/api/v1/tasks/' + task_ids[0])[0] == 404)
        for session in [viewer, other]:
            check('additional session logout', session.request('/api/v1/auth/logout', 'POST')[0] == 204)
    finally:
        check('admin logout', client.request('/api/v1/auth/logout', 'POST')[0] == 204)
    report = {'status': 'passed', 'passed': len(checks), 'checks': checks, 'gateway': args.gateway,
              'taskIds': task_ids, 'transport': 'real HTTPS/PostgreSQL/mTLS gRPC',
              'gatewayWrites': 0, 'configurationValues': 'not exported'}
    with output.open('x') as stream:
        json.dump(report, stream, indent=2)
        stream.write('\n')
    print(json.dumps({'passed': len(checks), 'evidence': str(output)}))


if __name__ == '__main__':
    main()
