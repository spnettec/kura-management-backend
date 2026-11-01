#!/usr/bin/env python3
"""HTTPS/gRPC domain-error contract; the host remains observation-only."""
import importlib.util
import argparse
import json
from pathlib import Path
import re
import uuid

ROOT=Path(__file__).resolve().parents[1]
spec=importlib.util.spec_from_file_location('network_smoke',ROOT/'scripts/phase2-network-smoke.py')
n=importlib.util.module_from_spec(spec);spec.loader.exec_module(n)
n.GATEWAY='dev-gateway'
checks=[]
def check(name,value):
 if not value:raise AssertionError(name)
 checks.append(name)
def main():
 parser=argparse.ArgumentParser(description=__doc__)
 parser.add_argument('--evidence-prefix',default='writers-contract')
 args=parser.parse_args()
 if not re.fullmatch(r'[a-z0-9-]+',args.evidence_prefix):parser.error('invalid evidence prefix')
 c=n.m.Client();c.login('admin')
 view=n.remote(c,'network.inspect')
 check('host remains observation only',not view['writeEnabled'])
 key=uuid.uuid4().hex
 body={'operation':'network.prepare','idempotencyKey':key,'networkChange':{'expectedRevision':view['revision'],'sourceDigest':view['sourceDigest'],'confirmSeconds':120,'config':{'interface':'no-such-iface','dhcp4':False,'addresses':['192.0.2.9/24'],'routes':[],'nameservers':[],'searchDomains':[],'mtu':1500,'optional':True,'dhcpMetric':100}}}
 status,receipt=c.request('/api/v1/gateways/dev-gateway/tasks','POST',body)
 check('typed domain denial admitted as task',status==202)
 task=n.m.wait_task(c,receipt['id'])
 check('gRPC persists typed domain error',task['state']=='FAILED' and task['error']=='FORBIDDEN' and task['result']['code']=='FORBIDDEN')
 check('authored diagnostic traverses gRPC and HTTPS',task['result']['diagnostic']=='FORBIDDEN: interface not explicitly delegated')
 status,replay=c.request('/api/v1/gateways/dev-gateway/tasks','POST',body)
 check('failed task replay remains idempotent',status==202 and replay['id']==receipt['id'])
 check('denial leaves network journal unchanged',n.remote(c,'network.inspect')['transaction']==view['transaction'])
 status,_=c.request('/api/v1/auth/logout','POST',{})
 check('logout',status==204)
 output={'passed':len(checks),'checks':checks,'writes':'none; host interface is undelegated'}
 (ROOT/f'target/{args.evidence_prefix}.json').write_text(json.dumps(output,indent=2)+'\n');print(json.dumps(output,indent=2))
if __name__=='__main__':main()
