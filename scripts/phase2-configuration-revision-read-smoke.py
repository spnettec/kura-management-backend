#!/usr/bin/env python3
"""Verify the new read projection through real HTTPS/mTLS/UDS; remote writes remain closed."""
import argparse
import importlib.util
import json
from pathlib import Path
import re
import time
import uuid

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('revision_smoke', ROOT / 'scripts/phase1-smoke.py')
module = importlib.util.module_from_spec(spec); spec.loader.exec_module(module)

if __name__ == '__main__':
    parser = argparse.ArgumentParser(); parser.add_argument('--evidence', required=True); args = parser.parse_args()
    output = ROOT / args.evidence; assert not output.exists()
    client = module.Client(); client.login('admin')
    checks = []

    def check(name, valid):
        assert valid, name
        checks.append(name)

    body = {'operation': 'kura.configuration.get', 'idempotencyKey': str(uuid.uuid4()),
            'kuraConfiguration': {'pid': 'org.eclipse.kura.management.revisionFixture', 'locale': 'en'}}
    status, task = client.request('/api/v1/gateways/dev-gateway/tasks', 'POST', body)
    check('typed read admitted', status == 202)
    for _ in range(100):
        status, task = client.request('/api/v1/tasks/' + task['id'])
        assert status == 200
        if task['state'] not in ['QUEUED', 'RUNNING']:
            break
        time.sleep(.1)
    check('real bridge read succeeded', task['state'] == 'SUCCEEDED')
    view = task['result']
    check('core revision exposed as opaque token', bool(re.fullmatch('[0-9a-f]{64}', view.get('revision', ''))))
    check('core conditional capability reported', view['writable'] is True)
    props = {item['id']: item for item in view['properties']}
    check('long precision survives remote projection', props['count']['value'] == '9223372036854775807')
    check('password stays redacted without value', props['password']['redacted'] and 'value' not in props['password'])
    check('neither old nor new fixture secret is returned', all(s not in json.dumps(task) for s in ['fixture-initial', 'fixture-replaced']))
    status, _ = client.request('/api/v1/gateways/dev-gateway/tasks', 'POST',
                               {'operation': 'kura.configuration.update', 'idempotencyKey': str(uuid.uuid4())})
    check('unfinished remote writer is not admitted', status == 422)
    client.request('/api/v1/auth/logout', 'POST')
    output.write_text(json.dumps({'status': 'passed', 'checks': checks, 'passed': len(checks),
                                 'transport': 'real HTTPS/PostgreSQL/mTLS/UDS/OSGi',
                                 'remoteWrites': 0, 'valuesIncluded': False}, indent=2) + '\n')
