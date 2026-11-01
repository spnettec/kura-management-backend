#!/usr/bin/env python3
"""One-use real HTTPS writes/rejections against isolated IDEA or Linux observers."""
import argparse
import importlib.util
import json
from pathlib import Path
import subprocess
import time
import uuid

ROOT = Path(__file__).resolve().parents[1]
PID = 'org.eclipse.kura.management.revisionFixture'
spec = importlib.util.spec_from_file_location('update_real_api', ROOT / 'scripts/phase1-smoke.py')
api = importlib.util.module_from_spec(spec); spec.loader.exec_module(api)


def main():
    p = argparse.ArgumentParser(); p.add_argument('action', choices=['write', 'rejected', 'restart'])
    p.add_argument('--fixture', required=True); p.add_argument('--gateway', required=True)
    p.add_argument('--vm'); p.add_argument('--evidence', required=True); args = p.parse_args()
    assert args.gateway in ['configuration-update-idea-20261101', 'configuration-update-linux-20261101']
    assert args.fixture in [str(ROOT / '.phase1-dev/configuration-update-idea-20261101'), '/opt/yofc-configuration-update-20261101']
    output = ROOT / args.evidence; assert not output.exists()
    marker = output.with_suffix('.started'); marker.open('x').close()
    client = api.Client(); client.login('admin'); checks = []; task_ids = []

    def check(name, value):
        assert value, name
        checks.append(name)

    def read_file(name):
        path = str(Path(args.fixture) / name)
        return subprocess.check_output(['orb', '-m', args.vm, '-u', 'root', 'cat', path], text=True) if args.vm else Path(path).read_text()

    def stage(name):
        if args.vm:
            subprocess.run(['orb', '-m', args.vm, '-u', 'root', 'python3', '-c',
                            'from pathlib import Path; import sys; Path(sys.argv[1]).write_text(sys.argv[2])',
                            str(Path(args.fixture) / 'stage'), name + '\n'], check=True)
        else: (Path(args.fixture) / 'stage').write_text(name + '\n')
        for _ in range(80):
            try: return json.loads(read_file(name + '-proof.json'))
            except (FileNotFoundError, subprocess.CalledProcessError): time.sleep(.25)
        raise AssertionError('observer deadline')

    def task(operation, payload):
        key = 'kuraConfigurationUpdate' if operation.endswith('.update') else 'kuraConfiguration'
        code, value = client.request('/api/v1/gateways/' + args.gateway + '/tasks', 'POST',
                                     {'operation': operation, 'idempotencyKey': str(uuid.uuid4()), key: payload})
        check('task admitted: ' + operation, code == 202)
        task_ids.append(value['id'])
        for _ in range(150):
            code, receipt = client.request('/api/v1/tasks/' + value['id']); assert code == 200
            if receipt['state'] not in ['QUEUED', 'RUNNING']: return receipt
            time.sleep(.25)
        raise AssertionError('task receipt deadline; do not replay')

    def view():
        receipt = task('kura.configuration.get', {'pid': PID, 'locale': 'en'})
        check('configuration read succeeds', receipt['state'] == 'SUCCEEDED')
        value = receipt['result']
        password = next(x for x in value['properties'] if x['id'] == 'password')
        check('password read is redacted', password['redacted'] and 'value' not in password)
        return value

    def update(revision, properties, pid=PID):
        return task('kura.configuration.update', {'pid': pid, 'expectedRevision': revision, 'properties': properties})

    before = view()
    if args.action == 'write':
        initial = json.loads(read_file('initial-proof.json')); check('initial observer passed', initial['status'] == 'passed')
        result = update(before['revision'], [{'id': 'count', 'value': {'text': '9223372036854775807'}}])
        check('long patch submitted', result['state'] == 'SUCCEEDED' and result['result']['submitted'])
        kept = stage('keep'); check('keep password real CM and snapshot proof', kept['status'] == 'passed' and kept['cmChangeCount'] == initial['cmChangeCount'] + 1)
        current = view(); result = update(current['revision'], [{'id': 'password', 'value': {'text': 'fixture-remote-set'}}])
        check('password patch submitted', result['state'] == 'SUCCEEDED' and result['result']['submitted'])
        changed = stage('set'); check('explicit password real CM and snapshot proof', changed['status'] == 'passed' and changed['cmChangeCount'] == kept['cmChangeCount'] + 1)
    else:
        old = json.loads(read_file('initial-proof.json' if args.action == 'rejected' else 'set-proof.json'))
        result = update(old['revision'], [{'id': 'enabled', 'value': {'boolean': True}}])
        check('stale revision refuses write', result['state'] == 'FAILED' and result['error'] == 'CONFLICT')
        if args.action == 'rejected':
            result = update(before['revision'], [{'id': 'count', 'value': {'boolean': True}}])
            check('invalid metatype refuses write', result['state'] == 'FAILED' and result['error'] == 'INVALID_ARGUMENT')
            result = update(before['revision'], [{'id': 'count', 'value': {'text': '2'}}], 'org.eclipse.kura.not-reviewed')
            check('unreviewed PID refuses write', result['state'] == 'FAILED' and result['error'] == 'FORBIDDEN')
        after = view(); check('rejections preserve revision', before['revision'] == after['revision'])
        proof = stage('rejected' if args.action == 'rejected' else 'restart')
        check('observer verifies preserved value password snapshot', proof['status'] == 'passed' and proof['checks'] == 9)
        if args.action == 'rejected':
            check('rejections cause no CM write', proof['cmChangeCount'] == json.loads(read_file('set-proof.json'))['cmChangeCount'])
        else: check('new provider changes revision', after['revision'] != old['revision'])
    output.write_text(json.dumps({'status': 'passed', 'passed': len(checks), 'checks': checks, 'taskIds': task_ids,
                                  'gateway': args.gateway, 'action': args.action, 'realJava': True, 'networkWrites': 0,
                                  'secretValuesIncluded': False}, indent=2) + '\n')
    print(json.dumps({'status': 'passed', 'passed': len(checks), 'evidence': str(output)}))


if __name__ == '__main__': main()
