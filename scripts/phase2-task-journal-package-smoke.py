#!/usr/bin/env python3
"""Reconcile a real packaged Linux read receipt after restart, with no reexecution.

Only this fixture's already completed center task is marked UNKNOWN to inject a
lost reply. This is receipt/protocol evidence, not a real uncertain system write.
"""
import argparse
import importlib.util
import json
from pathlib import Path
import subprocess
import time
import uuid

ROOT = Path(__file__).resolve().parents[1]
GATEWAY = 'task-journal-package-gateway-20261101'
UNIT = 'kura-management-agent.service'
VM = 'yofc-management-network-20261101'


def load(name, relative):
    spec = importlib.util.spec_from_file_location(name, ROOT / relative)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--evidence', type=Path, required=True)
    args = parser.parse_args()
    assert not args.evidence.exists()
    api = load('journal_package_api', 'scripts/phase1-smoke.py')
    dev = load('journal_package_dev', 'scripts/phase1-dev.py')
    client = api.Client()
    client.login('admin')
    checks = []

    def check(name, condition):
        assert condition, name
        checks.append(name)

    def orb(*arguments):
        return subprocess.check_output(['orb', 'run', '-m', VM, '-u', 'root', *arguments], text=True).strip()

    def wait(task_id):
        for _ in range(100):
            code, value = client.request('/api/v1/tasks/' + task_id)
            assert code == 200
            if value['state'] == 'SUCCEEDED':
                return value
            time.sleep(.15)
        raise AssertionError('receipt was not reconciled')

    code, gateway = client.request('/api/v1/gateways/' + GATEWAY)
    check('packaged agent is online without Kura', code == 200 and gateway['online'] and not gateway['kuraAvailable'])
    check('packaged agent advertises receipt lookup', 'task.reconcile.v1' in gateway['capabilities'])
    code, task = client.request('/api/v1/gateways/' + GATEWAY + '/tasks', 'POST', {
        'operation': 'agent.status', 'idempotencyKey': str(uuid.uuid4())})
    check('read admitted through real HTTPS', code == 202)
    task_id = task['id']
    completed = wait(task_id)
    check('independent management read completed', completed['result']['agentAvailable'])
    probe = '''import hashlib,json,pathlib,sqlite3,subprocess,sys
root=pathlib.Path('/var/lib/kura-management-agent')
with sqlite3.connect('file:'+str(root/'results.sqlite')+'?mode=ro',uri=True) as db:
 row=db.execute('SELECT state,updated_at,plan,result FROM executions WHERE command_id=?',(sys.argv[1],)).fetchone()
 count=db.execute('SELECT count(*) FROM executions').fetchone()[0]
assert row and row[0]=='SUCCEEDED'
key=root/'results.sqlite.plan-key'
pid=subprocess.check_output(['systemctl','show','kura-management-agent.service','-p','MainPID','--value'],text=True).strip()
threads=[dict(l.split(':',1) for l in p.read_text().splitlines() if ':' in l) for p in pathlib.Path('/proc/'+pid+'/task').glob('*/status')]
assert threads and all(t['NoNewPrivs'].strip()=='1' and int(t['CapEff'].strip(),16)==0 for t in threads)
print(json.dumps({'state':row[0],'updatedAt':row[1],'planSha256':hashlib.sha256(row[2]).hexdigest(),'resultSha256':hashlib.sha256(row[3]).hexdigest(),'ciphertext':row[2][0]==1 and row[3][0]==1 and b'agentAvailable' not in row[3],'executionCount':count,'keySha256':hashlib.sha256(key.read_bytes()).hexdigest(),'keyMode':oct(key.stat().st_mode&0o777),'noNewPrivs':True,'effectiveCapabilities':0,'threads':len(threads),'pid':pid}))
'''
    before = json.loads(orb('python3', '-c', probe, task_id))
    check('packaged plans and receipts are encrypted with a private key', before['ciphertext'] and before['keyMode'] == '0o600')
    orb('systemctl', 'stop', UNIT)
    query = ("UPDATE mgmt_tasks SET state='UNKNOWN',error='OUTCOME_UNKNOWN',result=NULL,lease_until=NULL,"
             "expires_at=now()-interval '1 hour' WHERE gateway_id='" + GATEWAY + "' AND id='" + task_id + "';")
    subprocess.run([str(dev.PG / 'psql'), '-X', '-qAt', '-v', 'ON_ERROR_STOP=1', '-h', str(dev.DEV / 'pgsock'),
                    '-p', str(dev.PG_PORT), '-d', 'management'], input=query, text=True, check=True, capture_output=True)
    orb('systemctl', 'start', UNIT)
    reconciled = wait(task_id)
    after = json.loads(orb('python3', '-c', probe, task_id))
    check('real packaged agent restarted to a different PID', after['pid'] != before['pid'])
    check('original receipt resolves UNKNOWN after its deadline', completed == reconciled)
    check('receipt query did not execute again or update the local record',
          all(before[key] == after[key] for key in ['updatedAt', 'planSha256', 'resultSha256', 'executionCount']))
    check('restart preserved the exact encryption key', before['keySha256'] == after['keySha256'])
    check('every agent thread has no_new_privs and zero capabilities', after['noNewPrivs'] and after['effectiveCapabilities'] == 0)
    denied = '''import errno,socket
s=socket.socket(socket.AF_UNIX)
try: s.connect('/run/kura-management-agent/control.sock')
except OSError as e:
 assert e.errno==errno.EACCES
 print('EACCES')
else: raise AssertionError('unrelated UID reached socket')
'''
    check('unrelated nobody UID is denied by socket permissions', orb('runuser', '-u', 'nobody', '--', 'python3', '-c', denied) == 'EACCES')
    check('old phase2 agent remains active', orb('systemctl', 'is-active', 'yofc-management-phase2-agent.service') == 'active')
    check('session logout', client.request('/api/v1/auth/logout', 'POST')[0] == 204)
    result = {'status': 'passed', 'passed': len(checks), 'checks': checks, 'gateway': GATEWAY,
              'taskId': task_id, 'before': before, 'after': after,
              'transport': 'real HTTPS/PostgreSQL/mTLS gRPC and packaged agent', 'systemNetworkWrites': 0,
              'fault': 'explicitly injected lost center receipt for an already completed read',
              'fullFilesystemSandbox': 'deferred due to OrbStack global override'}
    with args.evidence.open('x') as stream:
        json.dump(result, stream, indent=2)
        stream.write('\n')
    print(json.dumps({'passed': len(checks), 'evidence': str(args.evidence)}))


if __name__ == '__main__':
    main()
