#!/usr/bin/env python3
"""Actual package-agent restart and Kura bridge re-registration, without network writes."""
import argparse
import importlib.util
import json
from pathlib import Path
import subprocess
import time
import uuid

ROOT = Path(__file__).resolve().parents[1]
VM = 'yofc-management-network-20261101'
GATEWAY = 'package-gateway-20261101'
UNIT = 'kura-management-agent.service'


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    if args.output.exists():
        parser.error('Keep previous evidence; choose a new path')
    spec = importlib.util.spec_from_file_location('api', ROOT / 'scripts/phase1-smoke.py')
    api = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(api)

    def orb(*arguments):
        return subprocess.check_output(['orb', 'run', '-m', VM, '-u', 'root', *arguments], text=True).strip()

    client = api.Client()
    client.login('admin')
    code, before = client.request('/api/v1/gateways/' + GATEWAY)
    assert code == 200 and before['online'] and before['kuraAvailable']
    pid = orb('systemctl', 'show', UNIT, '-p', 'MainPID', '--value')
    orb('systemctl', 'restart', UNIT)
    new_pid = orb('systemctl', 'show', UNIT, '-p', 'MainPID', '--value')
    assert new_pid not in ('0', pid)
    for _ in range(80):
        code, current = client.request('/api/v1/gateways/' + GATEWAY)
        if code == 200 and current['online'] and current['kuraAvailable'] and current['lastSeen'] > before['lastSeen']:
            break
        time.sleep(.25)
    else:
        raise AssertionError('Agent did not reconnect and regain the real Kura bridge')
    tasks = []
    for operation in ['agent.status', 'kura.configuration.list', 'kura.snapshot.list']:
        code, task = client.request('/api/v1/gateways/' + GATEWAY + '/tasks', 'POST',
                                    {'operation': operation, 'idempotencyKey': str(uuid.uuid4())})
        assert code == 202
        result = api.wait_task(client, task['id'])
        assert result['state'] == 'SUCCEEDED'
        tasks.append(result)
    probe = '''import json,pathlib,subprocess
pid=subprocess.check_output(['systemctl','show','kura-management-agent.service','-p','MainPID','--value'],text=True).strip()
rows=[dict(l.split(':',1) for l in p.read_text().splitlines() if ':' in l) for p in pathlib.Path('/proc/'+pid+'/task').glob('*/status')]
assert rows and all(r['NoNewPrivs'].strip()=='1' and int(r['CapEff'].strip(),16)==0 for r in rows)
socket=pathlib.Path('/run/kura-management-agent/control.sock').stat()
print(json.dumps({'threads':len(rows),'allThreadsNoNewPrivs':True,'effectiveCapabilities':'0','socketMode':oct(socket.st_mode&0o777),'socketGid':socket.st_gid,'unit':subprocess.check_output(['systemctl','cat','kura-management-agent.service'],text=True)}))
'''
    permissions = json.loads(orb('python3', '-c', probe))
    denied = '''import errno,socket
s=socket.socket(socket.AF_UNIX)
try:
 s.connect('/run/kura-management-agent/control.sock')
except OSError as e:
 assert e.errno==errno.EACCES
 print('EACCES')
else:
 raise AssertionError('Unrelated UID reached the management socket')
'''
    assert orb('runuser', '-u', 'nobody', '--', 'python3', '-c', denied) == 'EACCES'
    assert client.request('/api/v1/auth/logout', 'POST')[0] == 204
    result = {'status': 'passed', 'vm': VM, 'gateway': GATEWAY, 'oldPid': pid, 'newPid': new_pid,
              'checks': ['systemd restarts packaged agent', 'mTLS heartbeat advances after restart',
                         'running Kura re-registers its actual UID bridge', 'three read tasks succeed after reconnect',
                         'every agent thread has no_new_privs and zero effective capabilities',
                         'unrelated nobody UID is denied by socket permissions'],
              'permissions': permissions, 'tasks': tasks, 'networkWrites': 0,
              'limitations': ['Full systemd filesystem sandbox is deferred due to the OrbStack global LXC override.']}
    with args.output.open('x') as stream:
        stream.write(json.dumps(result, indent=2, ensure_ascii=False) + '\n')
    print(json.dumps({'status': 'passed', 'checks': len(result['checks']), 'output': str(args.output)}))


if __name__ == '__main__':
    main()
