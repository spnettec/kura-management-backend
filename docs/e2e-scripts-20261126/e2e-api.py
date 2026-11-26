#!/usr/bin/env python3
"""Console REST helper for the E2E deployment.

login                              建立管理员会话（cookie + CSRF 保存在 <root>/console/session.json）
register <网关ID> <名称> <leaf.pem> 以 cert-only 方式登记网关
list                               每台网关一行：id online kuraAvailable agentVersion
get <网关ID>                       原始 JSON

The server certificate is verified against the E2E server CA (pki/server-ca.pem);
only the first-run setup (e2e-console-setup.py) talks to the bootstrap listener.

Usage: e2e-api.py --root <E2E_ROOT> <action> [args...]
Environment: CONSOLE_ORIGIN, ADMIN_USER, ADMIN_PASSWORD
"""
import argparse
import http.cookiejar
import json
import os
import ssl
import sys
import urllib.error
import urllib.request
from pathlib import Path


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument('--root', required=True)
    parser.add_argument('action', choices=['login', 'register', 'list', 'get'])
    parser.add_argument('args', nargs='*')
    options = parser.parse_args()
    root = Path(options.root)
    state = root / 'console'
    state.mkdir(parents=True, exist_ok=True)
    origin = os.environ['CONSOLE_ORIGIN']
    context = ssl.create_default_context(cafile=str(root / 'pki' / 'server-ca.pem'))
    opener = urllib.request.build_opener(urllib.request.HTTPCookieProcessor(http.cookiejar.CookieJar()),
                                          urllib.request.HTTPSHandler(context=context))

    def call(method, path, payload=None, csrf=None):
        request = urllib.request.Request(origin + path, method=method)
        request.add_header('Origin', origin)
        request.add_header('Content-Type', 'application/json')
        if csrf:
            request.add_header('X-CSRF-Token', csrf)
        data = json.dumps(payload).encode() if payload is not None else None
        try:
            with opener.open(request, data=data, timeout=30) as response:
                return response.status, response.read()
        except urllib.error.HTTPError as error:
            return error.code, error.read()

    status, body = call('POST', '/api/v1/auth/login',
                        {'username': os.environ['ADMIN_USER'], 'password': os.environ['ADMIN_PASSWORD']})
    if status != 200:
        sys.exit(f'login failed: HTTP {status} {body[:200]!r}')
    csrf = json.loads(body)['csrf']
    if options.action == 'login':
        (state / 'session.json').write_text(json.dumps({'origin': origin, 'csrf': csrf}, indent=2) + '\n')
        changed = json.loads(body).get('mustChangePassword')
        print(f'login ok: {os.environ["ADMIN_USER"]} at {origin} mustChangePassword={changed}')
    elif options.action == 'register':
        gateway, name, leaf = options.args
        status, body = call('POST', '/api/v1/gateways',
                            {'id': gateway, 'name': name, 'clientCertificatePem': Path(leaf).read_text()}, csrf)
        if status != 201:
            sys.exit(f'register {gateway} failed: HTTP {status} {body[:300]!r}')
        record = json.loads(body)
        print(f"registered {record['id']} generation={record['bindingGeneration']} certificateSha256={record['certificateSha256']}")
    else:
        status, body = call('GET', '/api/v1/gateways')
        if status != 200:
            sys.exit(f'list failed: HTTP {status} {body[:200]!r}')
        items = json.loads(body)['items']
        if options.action == 'get':
            wanted = options.args[0]
            print(json.dumps(next((item for item in items if item['id'] == wanted), {}), indent=2))
        else:
            for item in items:
                print(f"{item['id']:<12} online={str(item.get('online')):<5} kuraAvailable={str(item.get('kuraAvailable')):<5} "
                      f"agent={item.get('agentVersion')}")
            online = sum(1 for item in items if item.get('online'))
            available = sum(1 for item in items if item.get('kuraAvailable'))
            print(f'total={len(items)} online={online} kura_available={available}')


if __name__ == '__main__':
    main()
