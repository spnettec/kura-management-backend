#!/usr/bin/env python3
"""Live HTTPS/SSE + persisted read-task contracts against phase1-dev.py (real PostgreSQL)."""
import http.cookiejar
import json
from pathlib import Path
import ssl
import sys
import time
import urllib.request
import urllib.error
import uuid

ROOT=Path(__file__).resolve().parents[1]
DEV=ROOT/'.phase1-dev'
BASE='https://localhost:18443'
checks=[]
def check(name,condition):
    if not condition:raise AssertionError(name)
    checks.append(name)

class Client:
    def __init__(self):
        self.cookies=http.cookiejar.CookieJar()
        self.opener=urllib.request.build_opener(urllib.request.HTTPCookieProcessor(self.cookies),urllib.request.HTTPSHandler(context=ssl.create_default_context(cafile=str(DEV/'ca.pem'))))
        self.csrf=''
    def request(self,path,method='GET',body=None,origin=BASE,csrf=True):
        headers={}
        if method!='GET':
            if origin:headers['Origin']=origin
            if csrf:headers['X-CSRF-Token']=self.csrf
        if body is not None:headers['Content-Type']='application/json'
        request=urllib.request.Request(BASE+path,headers=headers,method=method,data=None if body is None else body if isinstance(body,bytes) else json.dumps(body).encode())
        try:
            with self.opener.open(request,timeout=8) as response:
                raw=response.read();return response.status,json.loads(raw) if raw and response.headers.get_content_type()=='application/json' else raw
        except urllib.error.HTTPError as e:
            raw=e.read()
            try:value=json.loads(raw)
            except ValueError:value=raw
            return e.code,value
    def login(self,user):
        status,value=self.request('/api/v1/auth/login','POST',{'username':user,'password':(DEV/f'{user}.password').read_text()})
        check(f'{user} login',status==200)
        self.csrf=value['csrf']

def task(client,operation,key=None):
    key=key or str(uuid.uuid4())
    status,result=client.request('/api/v1/gateways/dev-gateway/tasks','POST',{'operation':operation,'idempotencyKey':key})
    check(f'{operation} accepted',status==202)
    return result['id'],key

def wait_task(client,task_id):
    for _ in range(60):
        status,body=client.request('/api/v1/tasks')
        found=next((t for t in body['items'] if t['id']==task_id),None)
        if found and found['state'] in ['SUCCEEDED','FAILED','EXPIRED']:return found
        time.sleep(.25)
    raise AssertionError('task timed out')

def main():
    c=Client()
    check('unauthenticated is 401',c.request('/api/v1/gateways')[0]==401)
    status,error=c.request('/api/v1/auth/login')
    check('unsupported HTTP method uses typed error',status==405 and error['error']['code']=='UNSUPPORTED')
    status,error=c.request('/api/v1/gateways/%FF')
    check('invalid UTF-8 path uses typed error',status==400 and error['error']['code']=='INVALID_ARGUMENT')
    check('login rejects foreign origin',c.request('/api/v1/auth/login','POST',{'username':'admin','password':'wrong'},origin='https://evil.invalid')[0]==403)
    check('bad password is 401',c.request('/api/v1/auth/login','POST',{'username':'admin','password':'wrong'})[0]==401)
    c.login('admin')
    status,error=c.request('/api/v1/tasks?offset=not-a-number')
    check('invalid query uses typed error',status==400 and error['error']['code']=='INVALID_ARGUMENT')
    status,error=c.request('/api/v1/gateways/dev-gateway/tasks','POST',b'{')
    check('malformed JSON uses typed error',status==400 and error['error']['code']=='INVALID_ARGUMENT')
    status,error=c.request('/api/v1/gateways/dev-gateway/tasks','POST',{'operation':'agent.status','idempotencyKey':'extra','extra':True})
    check('unknown JSON field rejected with typed error',status==422 and error['error']['code']=='INVALID_ARGUMENT')
    status,error=c.request('/api/v1/gateways/dev-gateway/tasks','POST',b' '*65537)
    check('oversized HTTP body uses typed error',status==413 and error['error']['code']=='RESOURCE_EXHAUSTED')
    cookie=list(c.cookies)[0]
    check('cookie Secure HttpOnly Strict',cookie.secure and cookie.has_nonstandard_attr('HttpOnly') and cookie.get_nonstandard_attr('SameSite')=='Strict')
    for _ in range(40):
        status,body=c.request('/api/v1/gateways')
        if body['items'] and body['items'][0]['online'] and not body['items'][0]['kuraAvailable']:break
        time.sleep(.25)
    check('certificate bound agent is online',body['items'][0]['online'])
    check('Kura absence truthfully reported',not body['items'][0]['kuraAvailable'])
    check('CSRF required',c.request('/api/v1/gateways/dev-gateway/tasks','POST',{'operation':'agent.status','idempotencyKey':'no-csrf'},csrf=False)[0]==403)
    check('write operation unsupported',c.request('/api/v1/gateways/dev-gateway/tasks','POST',{'operation':'system.reboot','idempotencyKey':'unsupported'})[0]==422)
    task_id,key=task(c,'agent.status')
    repeat,_=task(c,'agent.status',key)
    check('same idempotency key returns same persisted task',repeat==task_id)
    check('idempotency key cannot change operation',c.request('/api/v1/gateways/dev-gateway/tasks','POST',{'operation':'kura.snapshot.list','idempotencyKey':key})[0]==409)
    result=wait_task(c,task_id)
    check('agent remains manageable without Kura',result['state']=='SUCCEEDED' and result['result']['agentAvailable'] and not result['result']['kuraAvailable'])
    kura_id,_=task(c,'kura.configuration.list')
    result=wait_task(c,kura_id)
    check('Kura unavailable is typed error, no offline fallback',result['state']=='FAILED' and result['error']=='KURA_UNAVAILABLE')
    viewer=Client();viewer.login('viewer')
    check('viewer reads gateway',viewer.request('/api/v1/gateways')[0]==200)
    check('viewer cannot submit tasks',viewer.request('/api/v1/gateways/dev-gateway/tasks','POST',{'operation':'agent.status','idempotencyKey':'viewer'})[0]==403)
    other=Client();other.login('other')
    check('cross tenant list filtered',other.request('/api/v1/gateways')[1]['items']==[])
    check('cross tenant gateway hidden',other.request('/api/v1/gateways/dev-gateway')[0]==404)
    check('cross tenant task denied',other.request('/api/v1/gateways/dev-gateway/tasks','POST',{'operation':'agent.status','idempotencyKey':'other'})[0]==404)
    check('cross tenant tasks filtered',other.request('/api/v1/tasks')[1]['items']==[])
    req=urllib.request.Request(BASE+'/api/v1/events')
    with c.opener.open(req,timeout=8) as response:
        check('SSE content type',response.headers.get_content_type()=='text/event-stream')
        lines=[]
        while len(lines)<20:
            line=response.readline().decode();lines.append(line)
            if line.startswith('data:'):break
        check('SSE contains durable event id',any(line.startswith('id:') for line in lines))
    check('unknown API remains 404',c.request('/api/v1/missing')[0]==404)
    check('missing static asset remains 404',c.request('/assets/missing.js')[0]==404)
    check('explicit SPA route supported',c.request('/commands')[0]==200)
    check('logout succeeds',c.request('/api/v1/auth/logout','POST')[0]==204)
    check('logout invalidates server session',c.request('/api/v1/gateways')[0]==401)
    output={'passed':len(checks),'checks':checks,'database':'PostgreSQL','transport':'HTTPS + mTLS gRPC','kura':'absent; isolation verified'}
    (ROOT/'target/phase1-smoke.json').write_text(json.dumps(output,ensure_ascii=False,indent=2)+'\n')
    print(json.dumps(output,ensure_ascii=False,indent=2))
if __name__=='__main__':main()
