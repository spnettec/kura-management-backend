#!/usr/bin/env python3
"""Real HTTPS/mTLS/UDS checks after the named fixture's agent restart.

Stops only this fixture's Kura unit after live reads, leaving its agent running.
"""
import argparse
import importlib.util
import json
from pathlib import Path
import subprocess
import time
import uuid

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('phase1_smoke', ROOT / 'scripts/phase1-smoke.py')
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--evidence', required=True)
    args = parser.parse_args()
    output = ROOT / args.evidence
    assert not output.exists(), 'preserve previous evidence'
    checks = []

    def check(name, condition):
        assert condition, name
        checks.append(name)

    c = m.Client()
    c.login('admin')
    endpoint = '/api/v1/gateways/config-read-gateway-20261101/tasks'

    def task(operation, target=None):
        body = {'operation': operation, 'idempotencyKey': str(uuid.uuid4())}
        if target:
            body['kuraConfiguration'] = target
        status, accepted = c.request(endpoint, 'POST', body)
        check(operation + ' admitted', status == 202)
        for _ in range(100):
            status, value = c.request('/api/v1/tasks/' + accepted['id'])
            assert status == 200
            if value['state'] not in ['QUEUED', 'RUNNING']:
                return value
            time.sleep(.2)
        raise RuntimeError('read task timed out')

    def fixture(action):
        result = subprocess.run(['orb', 'run', '-m', 'yofc-management-network-20261101', '-u', 'root',
                                 'python3', '/mnt/mac' + str(ROOT / 'scripts/phase2-configuration-read-vm.py'),
                                 action], check=True, capture_output=True, text=True)
        return json.loads(result.stdout)

    initial = fixture('status')
    check('fixture agent restarted and Kura automatically re-registered', initial['phase'] == 'reconnected'
          and initial['agentStatus']['kuraAvailable'] and initial['baselineUnchanged'])
    agent = task('agent.status')
    check('agent available over new mTLS connection', agent['state'] == 'SUCCEEDED'
          and agent['result']['kuraAvailable'])
    for locale in ['en', 'zh']:
        configuration = task('kura.configuration.get', {'pid': 'HttpsKeystore', 'locale': locale})
        check('configuration metadata read after reconnect: ' + locale,
              configuration['state'] == 'SUCCEEDED' and configuration['result']['locale'] == locale)
        check('password value absent after reconnect: ' + locale, any(
            f['type'] == 'PASSWORD' and f['redacted'] and 'value' not in f
            for f in configuration['result']['properties']))
    stopped = fixture('stop-kura')
    check('only new Kura stopped, old agent and network inputs preserved', stopped['kuraPid'] == 0
          and stopped['agentPid'] == initial['agentPid'] and stopped['baselineUnchanged'] and stopped['oldAgentActive'])
    configuration = task('kura.configuration.get', {'pid': 'HttpsKeystore', 'locale': 'zh'})
    check('stopped Kura returns typed KURA_UNAVAILABLE', configuration['state'] == 'FAILED'
          and configuration['error'] == 'KURA_UNAVAILABLE')
    agent = task('agent.status')
    check('independent agent stays manageable after Kura stop', agent['state'] == 'SUCCEEDED'
          and agent['result']['agentAvailable'] and not agent['result']['kuraAvailable'])
    check('session logout', c.request('/api/v1/auth/logout', 'POST')[0] == 204)
    evidence = {'passed': len(checks), 'checks': checks, 'gateway': 'config-read-gateway-20261101',
                'before': initial, 'after': stopped, 'transport': 'real HTTPS/PostgreSQL/mTLS/UDS',
                'configurationValues': 'not included', 'networkWrites': 0}
    with output.open('x') as stream:
        json.dump(evidence, stream, indent=2)
        stream.write('\n')
    print(json.dumps({'passed': len(checks), 'evidence': str(output)}))


if __name__ == '__main__':
    main()
