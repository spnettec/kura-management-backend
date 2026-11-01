#!/usr/bin/env python3
"""Read the real Kura bridge through the existing HTTPS/mTLS development center.

No identity provisioning or network mutations. Evidence is never overwritten.
"""
import argparse
import importlib.util
import json
from pathlib import Path
import time
import uuid

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('phase1_client', ROOT / 'scripts/phase1-smoke.py')
client_module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(client_module)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--gateway', required=True)
    parser.add_argument('--expect-kura', choices=['available', 'absent'], required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    if args.output.exists():
        parser.error('evidence exists; select a new path instead of overwriting it')
    expected = args.expect_kura == 'available'
    client = client_module.Client()
    client.login('admin')
    checks = []
    tasks = []

    def check(name, condition):
        if not condition:
            raise AssertionError(name)
        checks.append(name)

    for _ in range(40):
        status, gateway = client.request('/api/v1/gateways/' + args.gateway)
        if status == 200 and gateway['online'] and gateway['kuraAvailable'] == expected:
            break
        time.sleep(.25)
    check('gateway is online with the expected real Kura availability',
          status == 200 and gateway['online'] and gateway['kuraAvailable'] == expected)
    for operation in ['agent.status', 'kura.configuration.list', 'kura.snapshot.list', 'network.inspect']:
        status, receipt = client.request('/api/v1/gateways/' + args.gateway + '/tasks', 'POST',
                                         {'operation': operation, 'idempotencyKey': str(uuid.uuid4())})
        check(operation + ' accepted', status == 202)
        result = client_module.wait_task(client, receipt['id'])
        if operation.startswith('kura.') and not expected:
            check(operation + ' reports KURA_UNAVAILABLE', result['state'] == 'FAILED'
                  and result['error'] == 'KURA_UNAVAILABLE')
        else:
            check(operation + ' succeeds', result['state'] == 'SUCCEEDED')
        if operation == 'kura.configuration.list' and expected:
            check('original external network PID is visible',
                  'org.eclipse.kura.net.admin.NetworkConfigurationService' in json.dumps(result['result']))
        if operation == 'agent.status':
            check('agent status agrees with bridge state', result['result']['kuraAvailable'] == expected)
        tasks.append(result)
    check('session logout succeeds', client.request('/api/v1/auth/logout', 'POST')[0] == 204)
    output = {'status': 'passed', 'gateway': args.gateway, 'kuraAvailable': expected,
              'checks': checks, 'passed': len(checks), 'tasks': tasks,
              'transport': 'HTTPS + mTLS gRPC + real Java UDS bridge', 'networkWrites': 0}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    with args.output.open('x') as stream:
        stream.write(json.dumps(output, ensure_ascii=False, indent=2) + '\n')
    print(json.dumps({'status': 'passed', 'passed': len(checks), 'output': str(args.output)}))


if __name__ == '__main__':
    main()
