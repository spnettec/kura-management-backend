#!/usr/bin/env python3
"""Live network task contracts. Observation only on the development host."""
import importlib.util
import json
from pathlib import Path
import time
import uuid

ROOT=Path(__file__).resolve().parents[1]
spec=importlib.util.spec_from_file_location('phase1_smoke',ROOT/'scripts/phase1-smoke.py')
m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
checks=[]
def check(name,value):
    if not value:raise AssertionError(name)
    checks.append(name)
def submit(client,operation,**extra):
    return client.request('/api/v1/gateways/dev-gateway/tasks','POST',{'operation':operation,'idempotencyKey':str(uuid.uuid4()),**extra})
def main():
    c=m.Client();c.login('admin')
    for _ in range(60):
        gateways=c.request('/api/v1/gateways')[1]['items']
        if any(g['id']=='dev-gateway' and 'network.inspect' in g['capabilities'] for g in gateways):break
        time.sleep(.25)
    check('agent advertises network capability',any(g['id']=='dev-gateway' and 'network.inspect' in g['capabilities'] for g in gateways))
    status,v=submit(c,'network.inspect');check('inspection accepted',status==202)
    result=m.wait_task(c,v['id']);check('inspection traverses mTLS gRPC',result['state']=='SUCCEEDED')
    inspect=result['result'];check('host remains observation-only',inspect['writeEnabled'] is False)
    check('inspection separates source digest and revision',len(inspect['sourceDigest'])==64 and inspect['revision']==0)
    config={'interface':'not-delegated','dhcp4':True,'addresses':[],'routes':[],'nameservers':[],'searchDomains':[],'mtu':1500,'optional':True,'dhcpMetric':100}
    change={'expectedRevision':0,'config':config,'confirmSeconds':120,'sourceDigest':inspect['sourceDigest']}
    check('network.prepare requires typed payload',submit(c,'network.prepare')[0]==400)
    check('inspection rejects mutation payload',submit(c,'network.inspect',networkChange=change)[0]==400)
    check('unknown nested payload field rejected',submit(c,'network.prepare',networkChange={**change,'shell':'forbidden'})[0]==422)
    key=str(uuid.uuid4());body={'operation':'network.prepare','idempotencyKey':key,'networkChange':change}
    status,v=c.request('/api/v1/gateways/dev-gateway/tasks','POST',body);check('typed network change admitted',status==202)
    repeat=c.request('/api/v1/gateways/dev-gateway/tasks','POST',body);check('identical typed replay returns same task',repeat[0]==202 and repeat[1]['id']==v['id'])
    body['networkChange']={**change,'confirmSeconds':121}
    check('same key cannot change payload',c.request('/api/v1/gateways/dev-gateway/tasks','POST',body)[0]==409)
    result=m.wait_task(c,v['id']);check('undelegated interface fails closed',result['state']=='FAILED' and result['error']=='FORBIDDEN')
    check('failed prepare leaves no candidate transaction',m.wait_task(c,submit(c,'network.inspect')[1]['id'])['result']['transaction'] is None)
    viewer=m.Client();viewer.login('viewer');check('viewer cannot enqueue network task',submit(viewer,'network.inspect')[0]==403)
    other=m.Client();other.login('other');check('operator cannot mutate network',submit(other,'network.prepare',networkChange=change)[0]==403)
    check('logout',c.request('/api/v1/auth/logout','POST')[0]==204)
    output={'passed':len(checks),'checks':checks,'writes':'none: unowned development-host interface rejected'}
    (ROOT/'target/phase2-contract.json').write_text(json.dumps(output,ensure_ascii=False,indent=2)+'\n')
    print(json.dumps(output,ensure_ascii=False,indent=2))
if __name__=='__main__':main()
