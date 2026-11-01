#!/usr/bin/env python3
"""Real transport/storage contract with a test-only bridge; no Java/system writes."""
import argparse
import importlib.util
import json
import os
from pathlib import Path
import re
import socket
import sqlite3
import subprocess
import tempfile
import threading
import uuid

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('configuration_contract_base', ROOT / 'scripts/phase2-task-journal-contract.py')
base = importlib.util.module_from_spec(spec); spec.loader.exec_module(base)
dev, api, until, send, receive = base.dev, base.api, base.until, base.send, base.receive


class Bridge:
    def __init__(self, path):
        self.calls = []
        self.drop = False
        self.error = None
        self.peer = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM); self.peer.connect(str(path))
        send(self.peer, {'version': 1, 'kind': 'register', 'requestId': 'register', 'method': 'kura.register',
                         'params': {'methods': ['kura.configuration.list', 'kura.snapshot.list', 'kura.configuration.get', 'kura.configuration.update']}})
        assert receive(self.peer)['result']['registered']
        self.thread = threading.Thread(target=self.run, daemon=True); self.thread.start()

    def run(self):
        try:
            while True:
                request = receive(self.peer); assert request['method'] == 'kura.configuration.update'
                self.calls.append(request['params'])
                if self.drop:
                    self.peer.shutdown(socket.SHUT_RDWR); self.peer.close(); return
                send(self.peer, {'version': 1, 'kind': 'response', 'requestId': request['requestId'], 'method': request['method'],
                                 'result': {'pid': request['params']['pid'], 'submitted': True,
                                            'previousRevision': request['params']['expectedRevision'], 'ignored': 'test-secret-not-for-receipt'}})
        except (EOFError, OSError): pass
        except Exception as error: self.error = type(error).__name__

    def close(self):
        try: self.peer.shutdown(socket.SHUT_RDWR)
        except OSError: pass
        self.peer.close(); self.thread.join(timeout=2)


def main():
    parser = argparse.ArgumentParser(); parser.add_argument('--name', default='configuration-update-contract-20261101')
    parser.add_argument('--evidence', required=True); args = parser.parse_args()
    assert re.fullmatch('[a-z0-9-]{1,64}', args.name)
    fixture = dev.DEV / args.name; output = ROOT / args.evidence
    assert not fixture.exists() and not output.exists(), 'retain existing fixtures and evidence'
    os.umask(0o077); fixture.mkdir(mode=0o700)
    socket_path = Path(tempfile.mkdtemp(prefix='yofc-config-update-')) / 'control.sock'
    (fixture / 'socket-path').write_text(str(socket_path))
    gateway = args.name + '-gateway'; checks = []; agent = bridge = None; registered = False
    client = api.Client()

    def check(name, valid):
        assert valid, name
        checks.append(name)

    def pg(query):
        return subprocess.run([str(dev.PG / 'psql'), '-X', '-qAt', '-v', 'ON_ERROR_STOP=1', '-h', str(dev.DEV / 'pgsock'),
                               '-p', str(dev.PG_PORT), '-d', 'management'], input=query, text=True, capture_output=True, check=True).stdout.strip()

    def console(*arguments):
        subprocess.run([str(ROOT / 'target/debug/yofc-management-console'), *map(str, arguments)],
                       env={**os.environ, **dev.env()}, check=True, stdout=subprocess.DEVNULL)

    def submit(update, key=None, actor=client, **options):
        return actor.request('/api/v1/gateways/' + gateway + '/tasks', 'POST',
                             {'operation': 'kura.configuration.update', 'idempotencyKey': key or str(uuid.uuid4()),
                              'kuraConfigurationUpdate': update}, **options)

    def receipt(task_id):
        status, result = client.request('/api/v1/tasks/' + task_id); assert status == 200
        return result

    def terminal(task_id):
        return until(lambda: result if (result := receipt(task_id))['state'] not in ['QUEUED', 'RUNNING'] else None, 35)

    try:
        ext = fixture / 'client.ext'
        ext.write_text('basicConstraints=critical,CA:FALSE\nkeyUsage=critical,digitalSignature\nextendedKeyUsage=clientAuth\nsubjectKeyIdentifier=hash\nauthorityKeyIdentifier=keyid,issuer\n')
        for command in [
            ['openssl', 'req', '-newkey', 'rsa:2048', '-nodes', '-subj', '/CN=' + gateway, '-keyout', str(fixture / 'agent.key'), '-out', str(fixture / 'agent.csr')],
            ['openssl', 'x509', '-req', '-in', str(fixture / 'agent.csr'), '-CA', str(dev.DEV / 'ca.pem'), '-CAkey', str(dev.DEV / 'ca.key'),
             '-set_serial', '0x' + uuid.uuid4().hex, '-days', '3', '-extfile', str(ext), '-out', str(fixture / 'agent.pem')],
        ]: subprocess.run(command, check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        console('register-gateway', gateway, 'dev', 'Configuration update protocol fixture', fixture / 'agent.pem'); registered = True
        environment = {**os.environ, **dev.env(), 'AGENT_GATEWAY_ID': gateway, 'AGENT_CERT': str(fixture / 'agent.pem'),
                       'AGENT_KEY': str(fixture / 'agent.key'), 'AGENT_SOCKET': str(socket_path), 'AGENT_STATE_DB': str(fixture / 'results.sqlite'),
                       'AGENT_NETWORK_INTERFACES': '', 'AGENT_KURA_NETWORK_INTERFACES': '', 'AGENT_KURA_CONFIGURATION_UPDATE': '1'}
        def launch():
            with (fixture / 'agent.log').open('ab') as log:
                return subprocess.Popen([str(dev.AGENT / 'target/debug/yofc-management-agent')], env=environment, stdout=log, stderr=log)
        agent = launch(); until(lambda: socket_path.exists()); bridge = Bridge(socket_path)
        until(lambda: pg("SELECT count(*) FROM mgmt_gateways WHERE id='" + gateway + "' AND last_seen>now()-interval '15 seconds' AND capabilities ? 'kura.configuration.update'") == '1')
        client.login('admin')
        patch = {'pid': 'test.pid', 'expectedRevision': 'a' * 64, 'properties': [
            {'id': 'count', 'value': {'text': '9223372036854775807'}},
            {'id': 'password', 'value': {'text': 'contract-private-test-secret'}},
            {'id': 'enabled', 'value': {'boolean': False}}]}
        check('write requires CSRF', submit(patch, csrf=False)[0] == 403)
        check('write rejects foreign origin', submit(patch, origin='https://invalid.example')[0] == 403)
        other = api.Client(); other.login('other')
        check('operator cannot write configuration', submit(patch, actor=other)[0] == 403)
        viewer = api.Client(); viewer.login('viewer')
        check('viewer cannot write configuration', submit(patch, actor=viewer)[0] == 403)
        invalid = dict(patch, properties=[{'id': 'count', 'value': {'text': 42}}])
        check('untyped numeric JSON is rejected', submit(invalid)[0] in [400, 422])
        invalid = dict(patch, properties=patch['properties'] * 2)
        check('duplicate set IDs are rejected', submit(invalid)[0] == 400)
        check('rejected calls create no tasks', pg("SELECT count(*) FROM mgmt_tasks WHERE gateway_id='" + gateway + "'") == '0')
        key = str(uuid.uuid4()); status, accepted = submit(patch, key)
        check('typed update admitted', status == 202); first_id = accepted['id']; first = terminal(first_id)
        check('typed update receipt succeeds', first['state'] == 'SUCCEEDED' and first['result'] == {
            'pid': 'test.pid', 'submitted': True, 'previousRevision': 'a' * 64})
        check('Java parameters preserve long, boolean and explicit password', bridge.calls == [{
            'pid': 'test.pid', 'expectedRevision': 'a' * 64, 'expiresAtUnixMs': bridge.calls[0]['expiresAtUnixMs'],
            'properties': {'count': '9223372036854775807', 'password': 'contract-private-test-secret', 'enabled': False}}])
        check('secret and unapproved bridge result fields are not returned', 'secret' not in json.dumps(first))
        check('center input and result are encrypted', pg("SELECT count(*) FROM mgmt_tasks WHERE id='" + first_id + "' AND input ? 'ciphertext' AND result ? 'ciphertext' AND input::text NOT LIKE '%contract-private-test-secret%'") == '1')
        status, replay = submit(patch, key)
        check('idempotent retry returns original task without executing', status == 202 and replay['id'] == first_id and len(bridge.calls) == 1)
        check('conflicting retry is rejected', submit(dict(patch, expectedRevision='b' * 64), key)[0] == 409)
        check('other tenant cannot read task', other.request('/api/v1/tasks/' + first_id)[0] == 404)
        bridge.drop = True; lost_key = str(uuid.uuid4()); status, lost = submit(patch, lost_key)
        check('fault injection update admitted', status == 202); lost_id = lost['id']; result = terminal(lost_id)
        check('UDS loss after delivery remains UNKNOWN', result['state'] == 'UNKNOWN' and result['error'] == 'OUTCOME_UNKNOWN' and len(bridge.calls) == 2)
        check('new write blocked while outcome is unknown', submit(patch)[0] == 409)
        status, replay = submit(patch, lost_key)
        check('unknown retry returns same task', status == 202 and replay['id'] == lost_id and replay['state'] == 'UNKNOWN')
        previous_epoch = pg("SELECT connection_epoch FROM mgmt_gateways WHERE id='" + gateway + "'")
        agent.terminate(); agent.wait(timeout=5); agent = launch()
        until(lambda: pg("SELECT count(*) FROM mgmt_gateways WHERE id='" + gateway + "' AND connection_epoch<>'" + previous_epoch + "'::uuid AND last_seen>now()-interval '5 seconds'") == '1')
        with sqlite3.connect('file:' + str(fixture / 'results.sqlite') + '?mode=ro', uri=True) as db:
            check('one durable admission per command survives restart', db.execute('SELECT count(*) FROM executions').fetchone()[0] == 2)
            check('local plans do not contain plaintext test password', all(b'contract-private-test-secret' not in row[0] for row in db.execute('SELECT plan FROM executions')))
        check('query after reconnect does not replay write', receipt(lost_id)['state'] == 'UNKNOWN' and len(bridge.calls) == 2 and bridge.error is None)
        result = {'status': 'passed', 'passed': len(checks), 'checks': checks, 'gateway': gateway,
                  'taskIds': [first_id, lost_id], 'testOnlyBridge': True, 'realJavaWrites': 0, 'networkWrites': 0,
                  'transport': 'real HTTPS/PostgreSQL/mTLS/UDS', 'valuesIncluded': False}
        output.write_text(json.dumps(result, indent=2) + '\n')
    finally:
        if bridge: bridge.close()
        if agent and agent.poll() is None: agent.terminate(); agent.wait(timeout=5)
        if registered: console('set-gateway-enabled', gateway, 'false')
        (fixture / 'final-state.json').write_text(json.dumps({'phase': 'stopped-data-retained' if output.exists() else 'stopped-failed-data-retained', 'gatewayDisabled': registered,
                                                           'passed': output.exists(), 'networkWrites': 0}, indent=2) + '\n')
    print(json.dumps({'status': 'passed', 'passed': len(checks), 'evidence': str(output)}))


if __name__ == '__main__': main()
