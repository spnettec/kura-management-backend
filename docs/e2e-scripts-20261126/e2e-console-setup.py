#!/usr/bin/env python3
"""First-run setup of the management center: pair, then submit the operations setup.

pair        用数据卷里的安装凭据（installation-credential.json）换取设置会话
operations  提交数据库地址、管理员、中心服务端证书和客户端 CA；中心据此完成初始化

初始化成功后 /api/setup/* 路由会下线，所以轮询收到 404 不算失败，以
/api/setup/status 的 configured=true（或随后能登录）为准。

Usage: e2e-console-setup.py --root <E2E_ROOT> --db-url <postgres URL>
Environment: CONSOLE_ORIGIN, ADMIN_USER, ADMIN_PASSWORD
"""
import argparse
import http.cookiejar
import json
import os
import ssl
import sys
import time
import urllib.error
import urllib.request
import uuid
from pathlib import Path


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument('--root', required=True)
    parser.add_argument('--db-url', required=True)
    options = parser.parse_args()
    root = Path(options.root)
    origin = os.environ['CONSOLE_ORIGIN']
    # The fresh listener presents a self-signed bootstrap certificate; it is the thing being replaced here.
    context = ssl._create_unverified_context()
    opener = urllib.request.build_opener(urllib.request.HTTPCookieProcessor(http.cookiejar.CookieJar()),
                                          urllib.request.HTTPSHandler(context=context))

    def call(method, path, payload=None, csrf=None):
        request = urllib.request.Request(origin + path, method=method)
        request.add_header('Origin', origin)
        request.add_header('Content-Type', 'application/json')
        if csrf:
            request.add_header('X-Setup-CSRF', csrf)
        data = json.dumps(payload).encode() if payload is not None else None
        try:
            with opener.open(request, data=data, timeout=60) as response:
                return response.status, response.read()
        except urllib.error.HTTPError as error:
            return error.code, error.read()

    credential = json.loads((root / 'console' / 'credential.json').read_text())
    status, body = call('POST', '/api/setup/pair',
                        {'installationId': credential['installationId'], 'token': credential['token']})
    if status != 200:
        sys.exit(f'pair failed: HTTP {status} {body[:300]!r}')
    csrf = json.loads(body)['csrf']
    print('pair ok, installation', credential['installationId'])

    operation = str(uuid.uuid4())
    payload = {
        'operationId': operation,
        'origin': origin,
        'grpcAddress': '0.0.0.0:18444',
        'databaseUrl': options.db_url,
        'username': os.environ['ADMIN_USER'],
        'tenantId': 'yofc',
        'password': os.environ['ADMIN_PASSWORD'],
        'serverPem': (root / 'pki' / 'server.pem').read_text(),
        'serverKey': (root / 'pki' / 'server.key').read_text(),
        'clientCaPem': (root / 'pki' / 'client-ca.pem').read_text(),
    }
    status, body = call('POST', '/api/setup/operations', payload, csrf)
    if status not in (200, 201, 202):
        sys.exit(f'operations submit failed: HTTP {status} {body[:300]!r}')
    print('operations submitted', operation, json.loads(body).get('state'))

    deadline = time.time() + 180
    outcome = 'UNKNOWN'
    while time.time() < deadline:
        time.sleep(2)
        status, body = call('GET', f'/api/setup/operations/{operation}', None, csrf)
        if status == 404:
            outcome = 'SETUP-ROUTES-RETIRED'  # the setup listener is gone: initialization finished
            break
        state = json.loads(body).get('state') or json.loads(body).get('status')
        if state in ('SUCCEEDED', 'FAILED', 'REJECTED'):
            outcome = state
            break
    if outcome not in ('SUCCEEDED', 'SETUP-ROUTES-RETIRED'):
        sys.exit(f'setup did not succeed: {outcome} {body[:300]!r}')
    (root / 'console' / 'setup-operation.json').write_text(json.dumps({'operationId': operation, 'outcome': outcome}, indent=2) + '\n')
    print('setup outcome', outcome)


if __name__ == '__main__':
    main()
