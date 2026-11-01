#!/usr/bin/env python3
"""VM-local read-only agent fixture. Invoke as root through orb; no networking changes.

Build first with CARGO_TARGET_DIR=/home/heyoulin/.cache/yofc-management-phase1/target-agent.
This fixture exercises systemd; it is not a distribution package installation test.
"""
from pathlib import Path
import os
import subprocess

assert os.geteuid()==0, 'run inside the selected OrbStack VM as root'
ROOT=Path('/mnt/mac/Users/heyoulin/iot-kura-develop/git')
DEV=ROOT/'kura-management-console/.phase1-dev'
CONFIG=Path('/etc/yofc-management-phase1')
os.umask(0o077)
def run(*args):subprocess.run(args,check=True)
subprocess.run(['systemctl','stop','yofc-management-phase1-agent.service'],check=False,capture_output=True)
run('install','-d','-m','700',str(CONFIG),'/var/lib/yofc-management-phase1')
run('install','-d','-m','755','/opt/yofc-management-phase1')
run('install','-m','755','/home/heyoulin/.cache/yofc-management-phase1/target-agent/debug/yofc-management-agent','/opt/yofc-management-phase1/yofc-management-agent')
for source,target in [('ca.pem','ca.pem'),('orb-agent.pem','agent.pem'),('orb-agent.key','agent.key')]:
    run('install','-m','600',str(DEV/source),str(CONFIG/target))
src=(ROOT/'kura-management-agent/packaging/kura-management-agent.service').read_text()
for old,new in [('Group=kura-management','Group=root'),('/etc/kura-management-agent/agent.env',str(CONFIG/'agent.env')),('/usr/bin/yofc-management-agent','/opt/yofc-management-phase1/yofc-management-agent'),('RuntimeDirectory=kura-management-agent','RuntimeDirectory=yofc-management-phase1'),('StateDirectory=kura-management-agent','StateDirectory=yofc-management-phase1')]:
    src=src.replace(old,new)
Path('/etc/systemd/system/yofc-management-phase1-agent.service').write_text(src)
settings={'AGENT_CONSOLE_ENDPOINT':'https://host.orb.internal:18444','AGENT_GATEWAY_ID':'orb-gateway','AGENT_CA':str(CONFIG/'ca.pem'),'AGENT_CERT':str(CONFIG/'agent.pem'),'AGENT_KEY':str(CONFIG/'agent.key'),'AGENT_SOCKET':'/run/yofc-management-phase1/control.sock','AGENT_STATE_DB':'/var/lib/yofc-management-phase1/results.sqlite','AGENT_KURA_UID':'0','AGENT_ADMIN_UID':'501','RUST_LOG':'info'}
p=CONFIG/'agent.env';p.write_text(''.join(f'{k}={v}\n' for k,v in settings.items()));p.chmod(0o600)
run('systemctl','daemon-reload')
run('systemctl','enable','--now','yofc-management-phase1-agent.service')
run('systemctl','status','yofc-management-phase1-agent.service','--no-pager','-n','8')
