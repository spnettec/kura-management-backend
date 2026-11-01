#!/usr/bin/env python3
"""Real HTTPS/Postgres/mTLS/UDS fault contract with a clearly test-only bridge.

Creates one isolated, read-only agent identity. Changes only its own gateway/task
rows to inject lost receipts; never changes system networking or an old fixture.
This is protocol evidence, not evidence of Java/Kura or Linux write execution.
"""
import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import socket
import sqlite3
import struct
import subprocess
import tempfile
import threading
import time
import uuid

ROOT = Path(__file__).resolve().parents[1]


def module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    result = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(result)
    return result


api = module('journal_api', ROOT / 'scripts/phase1-smoke.py')
dev = module('journal_dev', ROOT / 'scripts/phase1-dev.py')


def until(predicate, seconds=12):
    deadline = time.monotonic() + seconds
    while time.monotonic() < deadline:
        result = predicate()
        if result:
            return result
        time.sleep(.04)
    raise AssertionError('bounded condition timed out')


def send(peer, frame):
    data = json.dumps(frame).encode()
    peer.sendall(struct.pack('!I', len(data)) + data)


def receive(peer):
    def exact(size):
        data = b''
        while len(data) < size:
            part = peer.recv(size - len(data))
            if not part:
                raise EOFError('agent closed bridge')
            data += part
        return data
    size = struct.unpack('!I', exact(4))[0]
    assert 0 < size <= 1024 * 1024
    return json.loads(exact(size))


class ControlledBridge:
    def __init__(self, path):
        self.peer = socket.socket(socket.AF_UNIX)
        self.peer.settimeout(8)
        self.peer.connect(str(path))
        send(self.peer, {'version': 1, 'kind': 'register', 'requestId': 'fixture-register',
                         'method': 'kura.register', 'params': {'methods': [
                             'kura.configuration.list', 'kura.snapshot.list', 'kura.configuration.get']}})
        assert receive(self.peer).get('result', {}).get('registered') is True
        self.received = threading.Event()
        self.release = threading.Event()
        self.done = threading.Event()
        self.calls = 0
        self.error = None
        self.thread = threading.Thread(target=self.run, daemon=True)
        self.thread.start()

    def run(self):
        try:
            request = receive(self.peer)
            assert request['method'] == 'kura.configuration.get'
            self.calls += 1
            self.received.set()
            assert self.release.wait(4), 'contract failed to release delayed response'
            send(self.peer, {'version': 1, 'kind': 'response', 'requestId': request['requestId'],
                             'method': request['method'], 'result': {
                                 'fixture': 'controlled-test-only-bridge', 'pid': request['params']['pid'],
                                 'locale': request['params']['locale'], 'properties': [],
                                 'privateValue': 'TaskJournalPrivateValue'}})
            self.done.set()
            # Detect an accidental second execution without responding to it.
            while True:
                receive(self.peer)
                self.calls += 1
        except (EOFError, OSError):
            pass
        except Exception as error:
            self.error = repr(error)
            self.done.set()

    def close(self):
        self.release.set()
        self.peer.close()
        self.thread.join(timeout=1)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--name', default='task-journal-20261101')
    parser.add_argument('--evidence', required=True)
    args = parser.parse_args()
    assert re.fullmatch(r'[a-z0-9-]{1,64}', args.name)
    fixture = dev.DEV / args.name
    output = ROOT / args.evidence
    assert not fixture.exists() and not output.exists(), 'preserve previous identities/evidence'
    os.umask(0o077)
    fixture.mkdir(mode=0o700)
    socket_dir = Path(tempfile.mkdtemp(prefix='yofc-task-journal-'))
    socket_path = socket_dir / 'control.sock'
    (fixture / 'socket-path').write_text(str(socket_path))
    gateway = args.name + '-gateway'
    checks = []
    agent = bridge = None
    registered = False
    client = api.Client()

    def check(name, condition):
        assert condition, name
        checks.append(name)

    def pg(query):
        result = subprocess.run([str(dev.PG / 'psql'), '-X', '-qAt', '-v', 'ON_ERROR_STOP=1',
                                 '-h', str(dev.DEV / 'pgsock'), '-p', str(dev.PG_PORT),
                                 '-d', 'management'], input=query, text=True, capture_output=True, check=True)
        return result.stdout.strip()

    def console(*arguments):
        subprocess.run([str(ROOT / 'target/debug/yofc-management-console'), *map(str, arguments)],
                       check=True, env={**os.environ, **dev.env()}, stdout=subprocess.DEVNULL)

    def journal(query, params=()):
        with sqlite3.connect('file:' + str(fixture / 'results.sqlite') + '?mode=ro', uri=True) as db:
            return db.execute(query, params).fetchall()

    def receipt(task_id):
        status, value = client.request('/api/v1/tasks/' + task_id)
        assert status == 200
        return value

    def submit(operation, key=None, **payload):
        return client.request('/api/v1/gateways/' + gateway + '/tasks', 'POST', {
            'operation': operation, 'idempotencyKey': key or str(uuid.uuid4()), **payload})

    try:
        ext = fixture / 'client.ext'
        ext.write_text('basicConstraints=critical,CA:FALSE\nkeyUsage=critical,digitalSignature\n'
                       'extendedKeyUsage=clientAuth\nsubjectKeyIdentifier=hash\nauthorityKeyIdentifier=keyid,issuer\n')
        for command in [
            ['openssl', 'req', '-newkey', 'rsa:2048', '-nodes', '-subj', '/CN=' + gateway,
             '-keyout', str(fixture / 'agent.key'), '-out', str(fixture / 'agent.csr')],
            ['openssl', 'x509', '-req', '-in', str(fixture / 'agent.csr'), '-CA', str(dev.DEV / 'ca.pem'),
             '-CAkey', str(dev.DEV / 'ca.key'), '-set_serial', '0x' + uuid.uuid4().hex,
             '-days', '3', '-extfile', str(ext), '-out', str(fixture / 'agent.pem')],
        ]:
            subprocess.run(command, check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        console('register-gateway', gateway, 'dev', 'Task journal protocol fixture', fixture / 'agent.pem')
        registered = True
        environment = {**os.environ, **dev.env(), 'AGENT_GATEWAY_ID': gateway,
                       'AGENT_CERT': str(fixture / 'agent.pem'), 'AGENT_KEY': str(fixture / 'agent.key'),
                       'AGENT_SOCKET': str(socket_path),
                       'AGENT_STATE_DB': str(fixture / 'results.sqlite'),
                       'AGENT_NETWORK_INTERFACES': '', 'AGENT_KURA_NETWORK_INTERFACES': ''}
        with (fixture / 'agent.log').open('ab') as log:
            agent = subprocess.Popen([str(dev.AGENT / 'target/debug/yofc-management-agent')],
                                     env=environment, stdout=log, stderr=log)
        until(lambda: socket_path.exists())
        bridge = ControlledBridge(socket_path)
        client.login('admin')
        epoch = until(lambda: pg("SELECT connection_epoch FROM mgmt_gateways WHERE id='" + gateway + "'"))
        check('new agent advertises receipt reconciliation', pg(
            "SELECT capabilities ? 'task.reconcile.v1' FROM mgmt_gateways WHERE id='" + gateway + "'") == 't')
        status, accepted = submit('kura.configuration.get', kuraConfiguration={'pid': 'test.fixture.pid', 'locale': 'en'})
        check('controlled read admitted through real HTTPS', status == 202)
        task_id = accepted['id']
        check('execution reached controlled UDS bridge', bridge.received.wait(4))
        check('RUNNING admission is durable before bridge responds', journal(
            'SELECT state,result FROM executions WHERE command_id=?', (task_id,)) == [('RUNNING', None)])
        # Invalidate only this fixture's connection. The real server closes its gRPC stream.
        pg("UPDATE mgmt_gateways SET connection_epoch='" + str(uuid.uuid4()) + "' WHERE id='" + gateway + "';")
        until(lambda: 'control disconnected; reconnect scheduled' in (fixture / 'agent.log').read_text(), 1.7)
        check('real gRPC connection closed before UDS response', not bridge.done.is_set())
        bridge.release.set()
        until(lambda: journal('SELECT state FROM executions WHERE command_id=?', (task_id,)) == [('SUCCEEDED',)], 4)
        check('accepted worker completed after disconnect', bridge.done.is_set() and bridge.error is None)
        before = journal('SELECT state,updated_at,plan,result FROM executions WHERE command_id=?', (task_id,))[0]
        check('execution plan and receipt are ciphertext', before[2][0] == 1 and before[3][0] == 1
              and b'TaskJournalPrivateValue' not in before[3] and b'test.fixture.pid' not in before[2])
        # Deliberately inject a lost/expired receipt on this fixture's single read task.
        pg("UPDATE mgmt_tasks SET state='UNKNOWN',error='OUTCOME_UNKNOWN',result=NULL,lease_until=NULL,"
           "expires_at=now()-interval '1 hour' WHERE id='" + task_id + "' AND gateway_id='" + gateway + "';")
        until(lambda: receipt(task_id)['state'] == 'SUCCEEDED', 15)
        after = journal('SELECT state,updated_at,plan,result FROM executions WHERE command_id=?', (task_id,))[0]
        new_epoch = pg("SELECT connection_epoch FROM mgmt_gateways WHERE id='" + gateway + "'")
        check('fresh connection resolves UNKNOWN after original deadline', new_epoch != epoch
              and receipt(task_id)['result']['fixture'] == 'controlled-test-only-bridge')
        check('query leaves execution record and timestamps untouched', before == after and bridge.calls == 1)
        check('center stores encrypted input and result', pg(
            "SELECT (input ? '$yofcTask') AND (result ? '$yofcTask') "
            "AND strpos(input::text,'test.fixture.pid')=0 "
            "AND strpos(result::text,'TaskJournalPrivateValue')=0 FROM mgmt_tasks WHERE id='" + task_id + "'") == 't')
        check('authorized receipt decrypts original result',
              receipt(task_id)['result']['privateValue'] == 'TaskJournalPrivateValue')
        original_ciphertext = pg("SELECT result::text FROM mgmt_tasks WHERE id='" + task_id + "'")
        try:
            # Same row, different field: shape is valid but AEAD context is not.
            pg("UPDATE mgmt_tasks SET result=input WHERE id='" + task_id + "'")
            status, corrupt = client.request('/api/v1/tasks/' + task_id)
            check('corrupted receipt fails closed with a typed error', status == 503
                  and corrupt['error']['code'] == 'INTERNAL'
                  and 'TaskJournalPrivateValue' not in json.dumps(corrupt))
        finally:
            pg("UPDATE mgmt_tasks SET result='" + original_ciphertext.replace("'", "''")
               + "'::jsonb WHERE id='" + task_id + "'")

        missing_key = 'injected-missing-write'
        change = {'expectedRevision': 0, 'confirmSeconds': 30, 'sourceDigest': '', 'config': {
            'interface': 'fixture0', 'dhcp4': True, 'addresses': [], 'routes': [], 'nameservers': [],
            'searchDomains': [], 'mtu': 1500, 'optional': True, 'dhcpMetric': 0}}
        # Stop this fixture's agent before admission. The real HTTP path seals
        # its input; no Command frame can reach an executor during injection.
        bridge.close()
        agent.terminate()
        agent.wait(timeout=5)
        status, missing_task = submit('network.prepare', key=missing_key, networkChange=change)
        assert status == 202
        missing = missing_task['id']
        pg("UPDATE mgmt_tasks SET state='UNKNOWN',error='OUTCOME_UNKNOWN',expires_at=now()-interval '1 hour' "
           "WHERE id='" + missing + "' AND gateway_id='" + gateway + "' AND state='QUEUED';")
        assert receipt(missing)['state'] == 'UNKNOWN'
        with (fixture / 'agent.log').open('ab') as log:
            agent = subprocess.Popen([str(dev.AGENT / 'target/debug/yofc-management-agent')],
                                     env=environment, stdout=log, stderr=log)
        until(lambda: (receipt(missing).get('result') or {}).get('reason') == 'RECEIPT_NOT_FOUND', 12)
        check('missing receipt remains UNKNOWN with explicit reason', receipt(missing)['state'] == 'UNKNOWN'
              and receipt(missing)['error'] == 'OUTCOME_UNKNOWN')
        check('query did not create a missing execution', journal(
            'SELECT count(*) FROM executions WHERE command_id=?', (missing,))[0][0] == 0 and bridge.calls == 1)
        status, conflict = submit('network.prepare', networkChange=change)
        check('unresolved write blocks a new write', status == 409
              and conflict['error']['code'] == 'OUTCOME_UNKNOWN')
        status, replay = submit('network.prepare', key=missing_key, networkChange=change)
        check('idempotent retry returns the same unknown task', status == 202 and replay['id'] == missing
              and replay['state'] == 'UNKNOWN')
        status, rollback = submit('network.rollback', networkAction={
            'transactionId': str(uuid.uuid4()), 'confirmationToken': ''})
        check('explicit rollback remains available', status == 202)
        rollback = until(lambda: (value if value['state'] not in ['QUEUED', 'RUNNING'] else None)
                         if (value := receipt(rollback['id'])) else None)
        check('rollback of absent transaction is a typed failure without system writes',
              rollback['state'] == 'FAILED' and rollback['error'] == 'NOT_FOUND')
        status, healthy = submit('agent.status')
        check('reads remain available with unresolved write', status == 202)
        healthy = until(lambda: (value if value['state'] == 'SUCCEEDED' else None)
                        if (value := receipt(healthy['id'])) else None)
        check('independent agent stays manageable', healthy['result']['agentAvailable'] and agent.poll() is None)
        check('local encryption key is private', (fixture / 'results.sqlite.plan-key').stat().st_mode & 0o777 == 0o600)
        check('session logout', client.request('/api/v1/auth/logout', 'POST')[0] == 204)
        result = {'status': 'passed', 'passed': len(checks), 'checks': checks, 'gateway': gateway,
                  'taskIds': [task_id, missing, rollback['id'], healthy['id']],
                  'bridge': 'controlled-test-only; delayed configuration read, not Java/Kura',
                  'transport': 'real HTTPS/PostgreSQL/mTLS gRPC/UDS', 'bridgeExecutions': bridge.calls,
                  'faults': ['fixture connection epoch invalidated', 'fixture task receipt/TTL lost',
                             'fixture UNKNOWN write with no local admission',
                             'fixture encrypted result replaced by input then exactly restored'],
                  'systemNetworkWrites': 0, 'identity': 'retained locally; not archived',
                  'agentSha256': hashlib.sha256((dev.AGENT / 'target/debug/yofc-management-agent').read_bytes()).hexdigest()}
        with output.open('x') as stream:
            json.dump(result, stream, ensure_ascii=False, indent=2)
            stream.write('\n')
        print(json.dumps({'passed': len(checks), 'evidence': str(output)}))
    finally:
        if bridge:
            bridge.close()
        if agent and agent.poll() is None:
            agent.terminate()
            agent.wait(timeout=5)
        if registered:
            console('set-gateway-enabled', gateway, 'false')
        (fixture / 'final-state.json').write_text(json.dumps({
            'phase': 'stopped-data-retained', 'gatewayDisabled': registered,
            'agentStopped': agent is None or agent.poll() is not None,
            'evidenceExists': output.exists(), 'checksCompleted': len(checks)}) + '\n')


if __name__ == '__main__':
    main()
