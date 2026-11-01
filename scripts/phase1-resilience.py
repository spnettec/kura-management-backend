#!/usr/bin/env python3
"""Destructive only to isolated phase1 processes: reconnect, session, cert and task durability."""
import importlib.util
import json
from pathlib import Path
import socket
import struct
import time

ROOT=Path(__file__).resolve().parents[1]
def module(name,file):
    spec=importlib.util.spec_from_file_location(name,ROOT/'scripts'/file)
    result=importlib.util.module_from_spec(spec);spec.loader.exec_module(result);return result
dev=module('phase1_dev','phase1-dev.py');api=module('phase1_smoke','phase1-smoke.py')
checks=[]
def check(name,condition):
    assert condition,name
    checks.append(name);print(name,flush=True)
def sql(query):
    return dev.run([dev.PG/'psql','-h',dev.DEV/'pgsock','-p',dev.PG_PORT,'-d','management','-Atc',query],capture_output=True,text=True).stdout.strip()
def poll(predicate,timeout=45):
    until=time.monotonic()+timeout
    while time.monotonic()<until:
        if predicate():return True
        time.sleep(.25)
    return False
def local_status():
    with socket.socket(socket.AF_UNIX) as stream:
        stream.settimeout(3);stream.connect(str(dev.DEV/'run/control.sock'))
        body=json.dumps({'version':1,'kind':'request','requestId':'resilience','method':'agent.status'}).encode()
        stream.sendall(struct.pack('>I',len(body))+body)
        length=struct.unpack('>I',stream.recv(4))[0]
        return json.loads(stream.recv(length))['result']
def cli(*args):dev.run([ROOT/'target/debug/yofc-management-console',*args],env=dev.env(),capture_output=True)
def main():
    dev.up();client=api.Client();client.login('admin')
    epoch=sql("SELECT connection_epoch FROM mgmt_gateways WHERE id='dev-gateway'")
    dev.down('agent')
    check('gateway becomes offline after stopped agent',poll(lambda:not client.request('/api/v1/gateways/dev-gateway')[1]['online'],25))
    task_id,key=api.task(client,'agent.status')
    dev.launch('agent',dev.AGENT/'target/debug/yofc-management-agent')
    result=api.wait_task(client,task_id)
    check('queued task survives agent restart',result['state']=='SUCCEEDED')
    check('connection epoch changes on agent restart',sql("SELECT connection_epoch FROM mgmt_gateways WHERE id='dev-gateway'")!=epoch)
    dev.down('console')
    check('UDS status works while console is stopped',local_status()['agentAvailable'])
    dev.launch('console',ROOT/'target/debug/yofc-management-console');dev.wait_ready()
    check('server session survives console restart',client.request('/api/v1/session')[0]==200)
    check('agent reconnects after console restart',poll(lambda:client.request('/api/v1/gateways/dev-gateway')[1]['online']))
    repeated,_=api.task(client,'agent.status',key)
    check('same idempotency receipt survives both restarts',repeated==task_id)
    check('persisted terminal result is unchanged',api.wait_task(client,task_id)['result']==result['result'])
    try:
        cli('set-gateway-enabled','dev-gateway','false')
        check('disabled gateway becomes offline',not client.request('/api/v1/gateways/dev-gateway')[1]['online'])
        time.sleep(2)
        check('disabled certificate cannot update last_seen',sql("SELECT last_seen IS NULL FROM mgmt_gateways WHERE id='dev-gateway'")=='t')
        cli('set-gateway-enabled','dev-gateway','true')
        check('enabled agent reconnects',poll(lambda:client.request('/api/v1/gateways/dev-gateway')[1]['online']))
        cli('rotate-gateway-certificate','dev-gateway',str(dev.DEV/'unregistered.pem'))
        time.sleep(2)
        check('old active certificate stream revoked on rotation',sql("SELECT last_seen IS NULL FROM mgmt_gateways WHERE id='dev-gateway'")=='t')
        check('UDS remains available during identity rejection',local_status()['agentAvailable'])
    finally:
        cli('rotate-gateway-certificate','dev-gateway',str(dev.DEV/'agent.pem'))
        cli('set-gateway-enabled','dev-gateway','true')
    check('original isolated identity restored',poll(lambda:client.request('/api/v1/gateways/dev-gateway')[1]['online']))
    check('task terminal event and audit committed',int(sql(f"SELECT count(*) FROM mgmt_audit WHERE target='{task_id}' AND operation='task.result'"))==1 and int(sql(f"SELECT count(*) FROM mgmt_events WHERE target='{task_id}' AND data->>'state'='SUCCEEDED'"))==1)
    output={'passed':len(checks),'checks':checks,'scope':'isolated PostgreSQL and phase1 processes; Kura stopped'}
    (ROOT/'target/phase1-resilience.json').write_text(json.dumps(output,ensure_ascii=False,indent=2)+'\n')
    print(json.dumps(output,ensure_ascii=False))
if __name__=='__main__':main()
