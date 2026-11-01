#!/usr/bin/env python3
"""Real Linux systemd/reboot fixture acceptance. Requires explicit VM name and installed fixture."""
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import time
import uuid
ROOT=Path(__file__).resolve().parents[1]
if len(sys.argv)!=2:raise SystemExit('usage: phase1-orbstack-smoke.py VM_NAME')
VM=sys.argv[1]
spec=importlib.util.spec_from_file_location('api',ROOT/'scripts/phase1-smoke.py')
api=importlib.util.module_from_spec(spec);spec.loader.exec_module(api)
checks=[]
def check(name,ok):
    assert ok,name
    checks.append(name);print(name,flush=True)
def orb(*args,root=False):
    cmd=['orb','run','-m',VM]+(['-u','root'] if root else [])+list(args)
    return subprocess.run(cmd,check=True,capture_output=True,text=True).stdout.strip()
def poll(predicate,timeout=40):
    deadline=time.monotonic()+timeout
    while time.monotonic()<deadline:
        try:
            if predicate():return True
        except (subprocess.SubprocessError,OSError):pass
        time.sleep(.5)
    return False
def task(c,operation):
    status,value=c.request('/api/v1/gateways/orb-gateway/tasks','POST',{'operation':operation,'idempotencyKey':str(uuid.uuid4())})
    assert status==202,(status,value)
    return api.wait_task(c,value['id'])
def main():
    c=api.Client();c.login('admin')
    check('actual Ubuntu 24.04 arm64',orb('uname','-m')=='aarch64' and 'VERSION_ID="24.04"' in orb('cat','/etc/os-release'))
    check('fixture service is enabled',orb('systemctl','is-enabled','yofc-management-phase1-agent.service')=='enabled')
    check('Linux mTLS heartbeat reaches console',poll(lambda:c.request('/api/v1/gateways/orb-gateway')[1]['online']))
    result=task(c,'agent.status')
    check('Linux agent is manageable without Kura',result['state']=='SUCCEEDED' and result['result']['agentAvailable'] and not result['result']['kuraAvailable'])
    result=task(c,'kura.configuration.list')
    check('Linux missing Kura has typed failure',result['state']=='FAILED' and result['error']=='KURA_UNAVAILABLE')
    local_code="import socket,struct,json;s=socket.socket(socket.AF_UNIX);s.settimeout(3);s.connect('/run/yofc-management-phase1/control.sock');v=json.dumps({'version':1,'kind':'request','requestId':'linux','method':'agent.status'}).encode();s.sendall(struct.pack('>I',len(v))+v);n=struct.unpack('>I',s.recv(4))[0];print(s.recv(n).decode())"
    check('Linux credential-checked UDS status works',json.loads(orb('python3','-c',local_code,root=True))['result']['agentAvailable'])
    before=c.request('/api/v1/gateways/orb-gateway')[1]['lastSeen']
    pid=orb('systemctl','show','yofc-management-phase1-agent.service','-p','MainPID','--value')
    orb('systemctl','restart','yofc-management-phase1-agent.service',root=True)
    check('systemd restarts independent agent',poll(lambda:orb('systemctl','show','yofc-management-phase1-agent.service','-p','MainPID','--value') not in ['0',pid]))
    check('restarted Linux agent reconnects',poll(lambda:c.request('/api/v1/gateways/orb-gateway')[1]['lastSeen']>before))
    boot=orb('cat','/proc/sys/kernel/random/boot_id')
    subprocess.run(['orb','restart',VM],check=True,capture_output=True)
    check('VM reboot changes boot ID',poll(lambda:orb('cat','/proc/sys/kernel/random/boot_id')!=boot))
    check('agent starts automatically after VM reboot',poll(lambda:orb('systemctl','is-active','yofc-management-phase1-agent.service')=='active'))
    check('auto-started agent reaches console',poll(lambda:c.request('/api/v1/gateways/orb-gateway')[1]['online']))
    check('post-reboot read task succeeds',task(c,'agent.status')['state']=='SUCCEEDED')
    stats=orb('systemctl','show','yofc-management-phase1-agent.service','-p','MemoryCurrent','-p','TasksCurrent','-p','CPUUsageNSec','-p','NoNewPrivileges','-p','CapabilityBoundingSet')
    check('service applies empty capability set','CapabilityBoundingSet=\n' in stats+'\n')
    pid=orb('systemctl','show','yofc-management-phase1-agent.service','-p','MainPID','--value')
    thread_code="from pathlib import Path;import sys;rows=[next(l for l in p.read_text().splitlines() if l.startswith('NoNewPrivs:')) for p in Path('/proc/'+sys.argv[1]+'/task').glob('*/status')];assert rows and all(l.split()[1]=='1' for l in rows);print(len(rows))"
    check('all agent threads enforce no_new_privs',int(orb('python3','-c',thread_code,pid,root=True))>0)
    output={'passed':len(checks),'checks':checks,'vm':VM,'platform':'Ubuntu 24.04.5 arm64','serviceStats':stats,'limitations':['OrbStack LXC drop-in disables systemd NoNewPrivileges/ProtectSystem/ProtectHome/PrivateTmp; agent enforces process no_new_privs itself; remaining sandbox acceptance deferred to non-container Ubuntu.'],'scope':'Rust read-only agent fixture; Kura not installed; no Netplan/firewall mutations; no deb lifecycle claim'}
    (ROOT/'target/phase1-orbstack.json').write_text(json.dumps(output,ensure_ascii=False,indent=2)+'\n')
    (ROOT/'target/phase1-orbstack-journal.log').write_text(orb('journalctl','-u','yofc-management-phase1-agent.service','--no-pager','-n','40',root=True)+'\n')
    print(json.dumps(output,ensure_ascii=False))
if __name__=='__main__':main()
