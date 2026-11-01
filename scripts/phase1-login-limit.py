#!/usr/bin/env python3
"""Exercise the isolated console IP throttle, including spoofed forwarding headers."""
import importlib.util
import json
from pathlib import Path
import urllib.request
import urllib.error
ROOT=Path(__file__).resolve().parents[1]
s=importlib.util.spec_from_file_location('api',ROOT/'scripts/phase1-smoke.py')
api=importlib.util.module_from_spec(s);s.loader.exec_module(api)
c=api.Client();statuses=[]
for i in range(12):
    request=urllib.request.Request(api.BASE+'/api/v1/auth/login',method='POST',data=json.dumps({'username':f'unknown-{i}','password':'incorrect'}).encode(),headers={'Origin':api.BASE,'Content-Type':'application/json','X-Forwarded-For':f'198.51.100.{i+1}'})
    try:
        with c.opener.open(request,timeout=8) as r:status,body=r.status,json.loads(r.read())
    except urllib.error.HTTPError as r:status,body=r.code,json.loads(r.read())
    assert status in [401,429],(status,body)
    assert body['error']['code']==('UNAUTHENTICATED' if status==401 else 'RESOURCE_EXHAUSTED')
    statuses.append(status)
assert 429 in statuses and statuses[-1]==429 and statuses.count(401)<=10
output={'passed':2,'checks':['IP throttle returns typed 429','spoofed X-Forwarded-For and rotated usernames cannot bypass IP throttle'],'statuses':statuses}
(ROOT/'target/phase1-login-limit.json').write_text(json.dumps(output,indent=2)+'\n')
print(json.dumps(output))
