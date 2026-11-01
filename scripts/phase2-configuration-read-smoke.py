#!/usr/bin/env python3
"""Read-only HTTPS contracts. Use absent first, then live after the IDEA gate."""
import argparse
import importlib.util
import json
from pathlib import Path
import time
import uuid

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('phase1_smoke', ROOT / 'scripts/phase1-smoke.py')
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)
checks = []


def check(name, condition):
    if not condition:
        raise AssertionError(name)
    checks.append(name)


def wait(client, task_id):
    for _ in range(100):
        status, task = client.request('/api/v1/tasks/' + task_id)
        check_once = status == 200 and task['id'] == task_id
        if not check_once:
            raise AssertionError('scoped task read failed')
        if task['state'] not in ['QUEUED', 'RUNNING']:
            return task
        time.sleep(.3)
    raise AssertionError('task pending after 30 seconds')


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('mode', choices=['absent', 'live'])
    parser.add_argument('--gateway', default='dev-gateway')
    parser.add_argument('--evidence', required=True)
    args = parser.parse_args()
    output = ROOT / args.evidence
    if output.exists():
        raise SystemExit('Evidence exists; choose a new name.')
    c = m.Client()
    c.login('admin')
    endpoint = '/api/v1/gateways/' + args.gateway + '/tasks'

    def submit(operation='kura.configuration.get', target=None, key=None):
        body = {'operation': operation, 'idempotencyKey': key or str(uuid.uuid4())}
        if target is not None:
            body['kuraConfiguration'] = target
        return c.request(endpoint, 'POST', body)

    check('get requires typed target', submit()[0] == 400)
    target = {'pid': 'org.eclipse.kura.clock.ClockService', 'locale': 'en'}
    for name, value, status in [
        ('empty pid', {**target, 'pid': ''}, 400),
        ('blank pid', {**target, 'pid': '  '}, 400),
        ('control in pid', {**target, 'pid': 'test\npid'}, 400),
        ('oversize pid', {**target, 'pid': 'x' * 513}, 400),
        ('UTF8 byte bound', {**target, 'pid': '测' * 200}, 400),
        ('unsupported locale', {**target, 'locale': 'zh-CN'}, 400),
        ('unknown nested field', {**target, 'execute': 'forbidden'}, 422),
        ('missing locale', {'pid': 'test.pid'}, 422),
        ('numeric pid', {**target, 'pid': 12}, 422),
    ]:
        check(name + ' rejected', submit(target=value)[0] == status)
    check('target rejected for list', submit('kura.configuration.list', target)[0] == 400)
    check('target rejected for agent status', submit('agent.status', target)[0] == 400)
    key = str(uuid.uuid4())
    status, accepted = submit(target=target, key=key)
    check('get admitted as task', status == 202)
    check('identical replay retains ID', submit(target=target, key=key)[1]['id'] == accepted['id'])
    check('locale changes cannot reuse key', submit(target={**target, 'locale': 'zh'}, key=key)[0] == 409)
    task = wait(c, accepted['id'])
    check('task detail preserves gateway and operation', task['gatewayId'] == args.gateway and task['operation'] == 11)
    if args.mode == 'absent':
        check('absent Kura produces typed failure', task['state'] == 'FAILED' and task['error'] == 'KURA_UNAVAILABLE')
    else:
        check('live OSGi configuration read succeeds', task['state'] == 'SUCCEEDED')
        result = task['result']
        check('typed result binds target', result['pid'] == target['pid'] and result['locale'] == 'en')
        check('properties projected with type and redaction', all(
            isinstance(f['id'], str) and isinstance(f['present'], bool) and isinstance(f['redacted'], bool)
            and 'type' in f and (not f['redacted'] or 'value' not in f) for f in result['properties']))
        check('metadata defaults and options not exported', all('default' not in f and 'options' not in f for f in result['properties']))
        status, sensitive = submit(target={'pid': 'HttpsKeystore', 'locale': 'zh'})
        sensitive = wait(c, sensitive['id']) if status == 202 else {}
        check('live password metadata is redacted without a value', sensitive.get('state') == 'SUCCEEDED'
              and any(f['type'] == 'PASSWORD' and f['redacted'] and f['redaction'] == 'SECRET' and 'value' not in f
                      for f in sensitive['result']['properties']))
        status, unknown = submit(target={'pid': 'does.not.exist.' + str(uuid.uuid4()), 'locale': 'en'})
        check('missing live PID remains NOT_FOUND', status == 202 and wait(c, unknown['id'])['error'] == 'NOT_FOUND')
    viewer = m.Client()
    viewer.login('viewer')
    check('viewer cannot enqueue configuration get', viewer.request(endpoint, 'POST',
          {'operation': 'kura.configuration.get', 'idempotencyKey': str(uuid.uuid4()), 'kuraConfiguration': target})[0] == 403)
    check('viewer can read same tenant receipt', viewer.request('/api/v1/tasks/' + accepted['id'])[0] == 200)
    other = m.Client()
    other.login('other')
    check('other tenant cannot fetch receipt by known UUID', other.request('/api/v1/tasks/' + accepted['id'])[0] == 404)
    check('anonymous receipt read denied', m.Client().request('/api/v1/tasks/' + accepted['id'])[0] == 401)
    check('invalid task ID has typed error', c.request('/api/v1/tasks/invalid')[0] == 400)
    status, healthy = submit('agent.status')
    healthy = wait(c, healthy['id']) if status == 202 else {}
    check('independent agent remains manageable', healthy.get('state') == 'SUCCEEDED' and healthy['result']['agentAvailable'])
    for client in [c, viewer, other]:
        check('session logout', client.request('/api/v1/auth/logout', 'POST')[0] == 204)
    evidence = {'passed': len(checks), 'checks': checks, 'mode': args.mode, 'gateway': args.gateway,
                'transport': 'real HTTPS + PostgreSQL + mTLS gRPC', 'writes': 'none',
                'taskId': accepted['id'], 'configurationValues': 'not included in evidence'}
    output.write_text(json.dumps(evidence, ensure_ascii=False, indent=2) + '\n')
    print(json.dumps(evidence, ensure_ascii=False, indent=2))


if __name__ == '__main__':
    main()
