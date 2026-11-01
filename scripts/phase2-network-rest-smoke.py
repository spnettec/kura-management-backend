#!/usr/bin/env python3
"""Validate the isolated original Kura HTTP resource against its real Rust UDS owner.

The runtime must be prepared with phase2-network-rest-runtime.py. The test-only
probe restricts port 18080 to loopback before temporarily allowing REST there.
The gateway's Kura delegation remains observation-only; no system candidate is applied.
"""
import base64
import json
import os
from pathlib import Path
import time
import urllib.error
import urllib.request
import uuid

ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / 'target/phase2-network-rest-http.json'
READY = ROOT / 'target/phase2-network-rest-runtime.json'
BASE = 'http://127.0.0.1:18080/services/networkConfiguration/v1'
checks = []


def check(name, condition):
    if not condition:
        raise AssertionError(name)
    checks.append(name)


def request(path, method='GET', body=None, authenticated=True):
    headers = {}
    if authenticated:
        # Existing disposable Kura emulator identity, independent of console and VM accounts.
        credentials = 'admin:' + os.environ.get('KURA_ISOLATED_ADMIN_PASSWORD', 'admin')
        headers['Authorization'] = 'Basic ' + base64.b64encode(credentials.encode()).decode()
    data = None
    if body is not None:
        headers['Content-Type'] = 'application/json'
        data = json.dumps(body).encode()
    query = urllib.request.Request(BASE + path, headers=headers, method=method, data=data)
    try:
        response = urllib.request.urlopen(query, timeout=8)
    except urllib.error.HTTPError as error:
        response = error
    with response:
        raw = response.read()
        try:
            value = json.loads(raw)
        except ValueError:
            value = None
        return response.status, value, response.headers


def error_is(response, status, code):
    return response[0] == status and response[1]['error']['code'] == code


def main():
    if OUTPUT.exists():
        raise RuntimeError('Evidence exists; do not repeat this one-shot HTTP validation')
    deadline = time.monotonic() + 60
    while time.monotonic() < deadline:
        if READY.exists():
            ready = json.loads(READY.read_text())
            if ready.get('status') == 'ready':
                break
            if ready.get('status') == 'failed':
                raise RuntimeError('Runtime probe failed before HTTP validation')
        time.sleep(.1)
    else:
        raise RuntimeError('Existing IDEA runtime has not reached probe readiness')
    check('test runtime has a loopback-only listener and public API', ready['loopbackOnly'] and ready['publicApiRegistered'])
    (ROOT / 'target/phase2-network-rest-runtime-ready.json').write_text(json.dumps(ready, indent=2) + '\n')
    for path, method, body in [('/transactions/view', 'GET', None),
                               ('/transactions', 'POST', {}),
                               ('/transactions/missing', 'GET', None),
                               ('/transactions/missing/confirm', 'POST', {}),
                               ('/transactions/missing/rollback', 'POST', {})]:
        response = request(path, method, body, False)
        check('original authentication guards ' + method + ' ' + path, response[0] in (401, 403))
    status, pids, _ = request('/configurableComponents')
    check('original configuration route remains available with the original identity', status == 200
          and 'org.eclipse.kura.net.admin.NetworkConfigurationService' in pids['pids'])
    status, view, headers = request('/transactions/view')
    check('real REST to public API to Java provider to Rust UDS returns schema 1', status == 200 and view['schemaVersion'] == 1)
    check('confirmed state is distinct from an unapplied candidate', view['configurations'] == [] and view['revision'] == 0)
    check('view with potential owned tokens is not cacheable', headers.get('Cache-Control') == 'no-store')
    invalid = request('/transactions', 'POST', {'requestId': 'invalid', 'change': {}, 'extra': True})
    check('unknown envelope fields are rejected before delegation', error_is(invalid, 400, 'INVALID_ARGUMENT'))
    change = {'expectedRevision': view['revision'], 'sourceDigest': view['sourceDigest'], 'confirmSeconds': 60,
              'config': {'interface': 'eth0', 'dhcp4': True, 'addresses': [], 'routes': [], 'nameservers': [],
                         'searchDomains': [], 'mtu': 1500, 'optional': False, 'dhcpMetric': 100}}
    denied = request('/transactions', 'POST', {'requestId': uuid.uuid4().hex, 'change': change})
    check('valid original REST request cannot bypass empty Kura interface delegation', error_is(denied, 403, 'FORBIDDEN'))
    change['config']['unexpected'] = True
    malformed = request('/transactions', 'POST', {'requestId': uuid.uuid4().hex, 'change': change})
    check('unknown config fields are rejected by the real agent', error_is(malformed, 400, 'INVALID_ARGUMENT'))
    missing = request('/transactions/missing')
    check('query needs no token and preserves typed missing-transaction response', error_is(missing, 404, 'NOT_FOUND'))
    for operation in ['confirm', 'rollback']:
        empty = request('/transactions/missing/' + operation, 'POST', {})
        check(operation + ' requires an explicit body token', error_is(empty, 400, 'INVALID_ARGUMENT'))
    after = request('/transactions/view')[1]
    check('denied and invalid calls leave confirmed configuration and source unchanged',
          all(after[k] == view[k] for k in ['revision', 'sourceDigest', 'configurations'])
          and after.get('transaction') == view.get('transaction'))
    for response in [invalid, denied, malformed, missing]:
        check('typed error response is not cacheable: ' + response[1]['error']['code'],
              response[2].get('Cache-Control') == 'no-store')
    output = {'status': 'passed', 'checks': checks, 'passed': len(checks), 'realKuraJavaToRustUds': True,
              'actualHttpAuthorization': True, 'candidateApplied': False,
              'nonAdminRoleRuntimeValidated': False, 'linuxJavaProviderValidated': False}
    OUTPUT.write_text(json.dumps(output, indent=2) + '\n')
    print(json.dumps(output, indent=2))


if __name__ == '__main__':
    main()
