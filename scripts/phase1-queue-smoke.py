#!/usr/bin/env python3
"""Saturate only the isolated offline dev gateway; prove atomic admission and expiry audit."""
import concurrent.futures
import importlib.util
import json
from pathlib import Path
import time
import uuid
ROOT=Path(__file__).resolve().parents[1]
def module(name,file):
    s=importlib.util.spec_from_file_location(name,ROOT/'scripts'/file)
    m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m
dev=module('dev','phase1-dev.py');api=module('api','phase1-smoke.py')
def sql(query):return dev.run([dev.PG/'psql','-h',dev.DEV/'pgsock','-p',dev.PG_PORT,'-d','management','-Atc',query],capture_output=True,text=True).stdout.strip()
def main():
    c=api.Client();c.login('admin');checks=[]
    def check(name,ok):
        assert ok,name
        checks.append(name);print(name,flush=True)
    prefix='quota-'+str(uuid.uuid4())
    dev.down('agent')
    try:
        assert sql("SELECT count(*) FROM mgmt_tasks WHERE gateway_id='dev-gateway' AND state IN ('QUEUED','RUNNING')")=='0'
        def submit(i):return c.request('/api/v1/gateways/dev-gateway/tasks','POST',{'operation':'agent.status','idempotencyKey':f'{prefix}-{i}'})
        with concurrent.futures.ThreadPoolExecutor(max_workers=12) as pool:
            responses=list(pool.map(submit,range(110)))
        accepted=[body for status,body in responses if status==202]
        rejected=[body for status,body in responses if status==429]
        check('concurrent admission accepts exactly gateway quota',len(accepted)==100)
        check('overflow returns typed capacity error',len(rejected)==10 and all(r['error']['code']=='RESOURCE_EXHAUSTED' for r in rejected))
        index=next(i for i,(status,_) in enumerate(responses) if status==202)
        status,body=submit(index)
        check('idempotent replay succeeds while queue is full',status==202 and body['id']==responses[index][1]['id'])
        deadline=time.monotonic()+40
        while time.monotonic()<deadline:
            expired=int(sql(f"SELECT count(*) FROM mgmt_tasks WHERE idempotency_key LIKE '{prefix}%' AND state='EXPIRED'"))
            if expired==100:break
            time.sleep(.5)
        check('all offline tasks expire at bounded TTL',expired==100)
        rows=int(sql(f"SELECT count(*) FROM mgmt_tasks t JOIN mgmt_audit a ON a.target=t.id::text AND a.operation='task.expire' WHERE t.idempotency_key LIKE '{prefix}%'"))
        check('every expiry has committed audit',rows==100)
    finally:
        dev.launch('agent',dev.AGENT/'target/debug/yofc-management-agent')
    output={'passed':len(checks),'checks':checks,'gatewayQuota':100,'concurrentRequests':110,'workers':12,'scope':'isolated dev gateway only; does not prove 100-gateway throughput'}
    (ROOT/'target/phase1-queue.json').write_text(json.dumps(output,ensure_ascii=False,indent=2)+'\n')
    print(json.dumps(output,ensure_ascii=False))
if __name__=='__main__':main()
