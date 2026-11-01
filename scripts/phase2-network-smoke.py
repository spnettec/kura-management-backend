#!/usr/bin/env python3
"""Real Netplan/networkd transaction acceptance in the named disposable VM.

Changes only the dedicated yofc-test0 link. Includes service/VM restarts.
"""
import importlib.util
import argparse
import json
from pathlib import Path
import subprocess
import sys
import time
import uuid
import re

ROOT=Path(__file__).resolve().parents[1]
VM='yofc-management-network-20261101'
GATEWAY='network-gateway'
IFACE='yofc-test0'
spec=importlib.util.spec_from_file_location('phase1_smoke',ROOT/'scripts/phase1-smoke.py')
m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
checks=[]
evidence_prefix='phase2-network'
def check(name,value):
    if not value:raise AssertionError(name)
    checks.append(name)
    (ROOT/f'target/{evidence_prefix}-progress.json').write_text(json.dumps({'passed':len(checks),'checks':checks,'vm':VM},ensure_ascii=False,indent=2)+'\n')
def orb(*args):return subprocess.run(['orb','run','-m',VM,'-u','root',*args],check=True,capture_output=True,text=True).stdout
def local(method,params=None):
    program='''import json,socket,struct,sys,uuid
f={'version':1,'kind':'request','requestId':str(uuid.uuid4()),'method':sys.argv[1]}
if len(sys.argv)>2:f['params']=json.loads(sys.argv[2])
s=socket.socket(socket.AF_UNIX);s.settimeout(40);s.connect('/run/yofc-management-phase2/control.sock');b=json.dumps(f).encode();s.sendall(struct.pack('>I',len(b))+b)
def read(n):
 b=b''
 while len(b)<n:
  v=s.recv(n-len(b))
  if not v:raise EOFError()
  b+=v
 return b
n=struct.unpack('>I',read(4))[0];print(read(n).decode())
'''
    args=['python3','-c',program,method]
    if params is not None:args.append(json.dumps(params))
    response=json.loads(orb(*args))
    if response.get('error'):raise AssertionError(response)
    return response['result']
def remote(c,operation,**extra):
    status,result=c.request(f'/api/v1/gateways/{GATEWAY}/tasks','POST',{'operation':operation,'idempotencyKey':str(uuid.uuid4()),**extra})
    assert status==202,(status,result)
    deadline=time.monotonic()+40
    while time.monotonic()<deadline:
        body=c.request('/api/v1/tasks')[1]
        task=next((t for t in body['items'] if t['id']==result['id']),None)
        if task and task['state'] not in ['QUEUED','RUNNING']:
            assert task['state']=='SUCCEEDED',task
            return task['result']
        time.sleep(.2)
    raise AssertionError('remote network task timeout')
def current_ips():
    rows=json.loads(orb('ip','-j','-4','address','show','dev',IFACE))
    return [f"{a['local']}/{a['prefixlen']}" for r in rows for a in r['addr_info']]
def action(j):return {'transactionId':j['id'],'confirmationToken':j['confirmationToken']}
def prepare(c,ip,seconds=120,routes=None):
    view=remote(c,'network.inspect')
    return remote(c,'network.prepare',networkChange={'expectedRevision':view['revision'],'sourceDigest':view['sourceDigest'],'confirmSeconds':seconds,'config':{'interface':IFACE,'dhcp4':False,'addresses':[ip],'routes':routes or [],'nameservers':[],'searchDomains':[],'mtu':1500,'optional':True,'dhcpMetric':100}})
def wait_state(expected,timeout=45):
    deadline=time.monotonic()+timeout
    while time.monotonic()<deadline:
        try:view=local('network.inspect')
        except AssertionError as e:
            if 'CONFLICT' not in str(e):raise
            time.sleep(.2);continue
        state=view['transaction']['state']
        if state==expected:return view
        if state=='RECOVERY_REQUIRED':raise AssertionError(view['transaction'])
        time.sleep(.5)
    raise AssertionError(f'expected {expected}: {view}')
def main():
    global evidence_prefix
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('vm',choices=[VM])
    parser.add_argument('--evidence-prefix',default=evidence_prefix)
    args=parser.parse_args()
    if not re.fullmatch(r'[a-z0-9-]+',args.evidence_prefix):parser.error('invalid evidence prefix')
    evidence_prefix=args.evidence_prefix
    c=m.Client();c.login('admin')
    original=orb('sha256sum','/etc/netplan/10-lxc.yaml')
    default=orb('ip','-j','-4','route','show','default')
    check('real Ubuntu 24.04', '24.04' in orb('cat','/etc/os-release'))
    check('networkd active',orb('systemctl','is-active','systemd-networkd').strip()=='active')
    view=remote(c,'network.inspect')
    allowed={i['name'] for i in view['interfaces'] if i['allowed']}
    expected={IFACE,'yofc-adopt0'} if view['revision']>=2 else {IFACE}
    check('explicit known fixture delegation only',view['writeEnabled'] and allowed==expected)
    adopted=None
    if 'yofc-adopt0' in allowed:
        def adopted_values():
            link=json.loads(orb('ip','-j','address','show','dev','yofc-adopt0'))[0]
            # Recreated dummy links have a new MAC and link-local IPv6 after reboot.
            # Compare configured global addresses; link-local is kernel-derived.
            return {'mtu':link['mtu'],'addresses':sorted(f"{a['local']}/{a['prefixlen']}" for a in link['addr_info'] if a['scope']=='global'),
                    'routes':json.loads(orb('ip','-j','-4','route','show','dev','yofc-adopt0')),
                    'source':orb('sha256sum','/etc/netplan/40-yofc-existing-fixture.yaml')}
        adopted=adopted_values()
    if view['revision']==0:
        check('fixture link starts with no IPv4',current_ips()==[])
        j=prepare(c,'192.0.2.2/24')
        check('typed remote candidate awaits confirmation',j['state']=='WAIT_CONFIRM')
        check('candidate really applied','192.0.2.2/24' in current_ips())
        check('candidate not persistent',orb('test','!','-e','/etc/netplan/90-yofc-managed.yaml')=='')
        check('candidate is in volatile run',orb('test','-f','/run/netplan/90-yofc-managed.yaml')=='')
        j=remote(c,'network.confirm',networkAction=action(j))
        check('new mTLS stream confirms revision 1',j['state']=='COMMITTED' and j['revision']==1)
        check('confirmed file survives and runtime override removed',orb('test','-f','/etc/netplan/90-yofc-managed.yaml')=='' and orb('test','!','-e','/run/netplan/90-yofc-managed.yaml')=='')
    else:
        check('resume confirmed fixture without reinitialization',view['revision']>=1 and current_ips()==['192.0.2.2/24'])
    baseline=remote(c,'network.inspect')['revision']
    j=prepare(c,'192.0.2.3/24',30)
    check('second address is live before timeout','192.0.2.3/24' in current_ips())
    wait_state('ROLLED_BACK')
    check('timeout restores actual last confirmed address',current_ips()==['192.0.2.2/24'])
    j=prepare(c,'192.0.2.4/24',routes=[{'to':'198.51.100.0/24','via':'192.0.2.1','metric':111}])
    check('basic static route is live','198.51.100.0/24' in orb('ip','-j','-4','route','show','dev',IFACE))
    j=remote(c,'network.rollback',networkAction=action(j))
    check('explicit rollback restores actual address',j['state']=='ROLLED_BACK' and current_ips()==['192.0.2.2/24'])
    check('explicit rollback removes candidate route','198.51.100.0/24' not in orb('ip','-j','-4','route','show','dev',IFACE))
    j=prepare(c,'192.0.2.5/24')
    orb('systemctl','restart','yofc-management-phase2-agent.service')
    wait_state('ROLLED_BACK')
    check('agent process restart rolls back unconfirmed state',current_ips()==['192.0.2.2/24'])
    j=prepare(c,'192.0.2.6/24')
    before=orb('cat','/proc/sys/kernel/random/boot_id').strip()
    subprocess.run(['orb','restart',VM],check=True,capture_output=True,text=True)
    deadline=time.monotonic()+45
    while time.monotonic()<deadline:
        try:
            after=orb('cat','/proc/sys/kernel/random/boot_id').strip()
            if after!=before and orb('systemctl','is-active','yofc-management-phase2-agent.service').strip()=='active':break
        except subprocess.CalledProcessError:pass
        time.sleep(.5)
    check('VM really rebooted',after!=before)
    view=wait_state('ROLLED_BACK')
    check('reboot boots only confirmed baseline revision',view['revision']==baseline and current_ips()==['192.0.2.2/24'])
    check('unconfirmed override vanished on reboot',orb('test','!','-e','/run/netplan/90-yofc-managed.yaml')=='')
    check('original eth0 YAML unchanged',orb('sha256sum','/etc/netplan/10-lxc.yaml')==original)
    check('management default route unchanged',orb('ip','-j','-4','route','show','default')==default)
    check('Kura not required for network service',local('agent.status')['kuraAvailable'] is False)
    if adopted is not None:check('adopted IPv4 IPv6 MTU routes and normalized source preserved',adopted_values()==adopted)
    output={'passed':len(checks),'checks':checks,'vm':VM,'interface':IFACE,'baselineRevision':baseline,'backend':'real Netplan + systemd-networkd','limitations':['virtual dummy link, not physical wired hardware','graceful VM reboot, not hard power loss','original Kura networking provider not yet replaced']}
    (ROOT/f'target/{evidence_prefix}.json').write_text(json.dumps(output,ensure_ascii=False,indent=2)+'\n');print(json.dumps(output,ensure_ascii=False,indent=2))
if __name__=='__main__':main()
