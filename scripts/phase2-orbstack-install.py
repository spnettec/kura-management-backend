#!/usr/bin/env python3
"""Install an explicitly named, disposable Linux network fixture (not a release package)."""
import importlib.util
import os
from pathlib import Path
import subprocess
import sys

ROOT=Path(__file__).resolve().parents[1]
DEV=ROOT/'.phase1-dev'
VM='yofc-management-network-20261101'
GATEWAY='network-gateway'
def run(args,**kw):return subprocess.run([str(x) for x in args],check=True,**kw)
def host():
    if len(sys.argv)!=2 or sys.argv[1]!=VM:raise SystemExit(f'usage: {sys.argv[0]} {VM}')
    spec=importlib.util.spec_from_file_location('dev',ROOT/'scripts/phase1-dev.py');dev=importlib.util.module_from_spec(spec);spec.loader.exec_module(dev)
    key=DEV/'network-agent.key';cert=DEV/'network-agent.pem'
    if not cert.exists():
        run(['openssl','req','-newkey','rsa:2048','-nodes','-subj','/CN=network-fixture','-keyout',key,'-out',DEV/'network-agent.csr'],stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
        ext=DEV/'network-agent.ext';ext.write_text('basicConstraints=critical,CA:FALSE\nkeyUsage=critical,digitalSignature\nextendedKeyUsage=clientAuth\n')
        run(['openssl','x509','-req','-in',DEV/'network-agent.csr','-CA',DEV/'ca.pem','-CAkey',DEV/'ca.key','-CAserial',DEV/'ca-serial','-days','7','-extfile',ext,'-out',cert],stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL);key.chmod(0o600)
        run([ROOT/'target/debug/yofc-management-console','register-gateway',GATEWAY,'dev','网络事务隔离验证',cert],env=dev.env())
    run(['orb','run','-m',VM,'-u','root','python3',Path('/mnt/mac'+str(Path(__file__).resolve())),'--inside-fixture'])
def inside():
    assert os.geteuid()==0
    assert Path('/etc/os-release').read_text().find('24.04')>=0
    # Reject an unexpected delegation before stopping/replacing the running agent.
    env_path=Path('/etc/yofc-management-phase2/agent.env')
    interfaces='yofc-test0'
    if env_path.exists():
        line=next((s for s in env_path.read_text().splitlines() if s.startswith('AGENT_NETWORK_INTERFACES=')),None)
        assert line is not None, 'fixture delegation missing; inspect before reinstalling'
        interfaces=line.split('=',1)[1]
        assert set(interfaces.split(',')) in [{'yofc-test0'},{'yofc-test0','yofc-adopt0'}], 'unexpected fixture delegation; inspect before reinstalling'
    # A separate link carries documentation-only addresses. eth0 is never rewritten.
    for d in ['/opt/yofc-management-phase2','/etc/yofc-management-phase2','/var/lib/yofc-management-phase2','/run/yofc-management-phase2']:
        Path(d).mkdir(exist_ok=True,mode=0o700)
    source=Path('/mnt/machines/kura-package-validation-20261029/home/heyoulin/.cache/yofc-management-phase1/target-agent/debug/yofc-management-agent')
    run(['systemctl','stop','yofc-management-phase2-agent.service'],stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL) if Path('/etc/systemd/system/yofc-management-phase2-agent.service').exists() else None
    run(['install','-m','755',source,'/opt/yofc-management-phase2/agent'])
    for src,dst in [('ca.pem','ca.pem'),('network-agent.pem','agent.pem'),('network-agent.key','agent.key')]:
        run(['install','-m','600',DEV/src,Path('/etc/yofc-management-phase2')/dst])
    env_path.write_text('''AGENT_CONSOLE_ENDPOINT=https://host.orb.internal:18444
AGENT_GATEWAY_ID=network-gateway
AGENT_CA=/etc/yofc-management-phase2/ca.pem
AGENT_CERT=/etc/yofc-management-phase2/agent.pem
AGENT_KEY=/etc/yofc-management-phase2/agent.key
AGENT_STATE_DB=/var/lib/yofc-management-phase2/results.sqlite
AGENT_SOCKET=/run/yofc-management-phase2/control.sock
AGENT_KURA_UID=65534
AGENT_ADMIN_UID=0
AGENT_NETWORK_INTERFACES=yofc-test0
AGENT_NETWORK_ALLOW_VIRTUAL_FIXTURE=1
RUST_LOG=info
'''.replace('AGENT_NETWORK_INTERFACES=yofc-test0\n',f'AGENT_NETWORK_INTERFACES={interfaces}\n'))
    Path('/etc/yofc-management-phase2/agent.env').chmod(0o600)
    Path('/etc/systemd/system/yofc-management-phase2-link.service').write_text('''[Unit]
Description=Disposable management test link
Before=yofc-management-phase2-agent.service
[Service]
Type=oneshot
RemainAfterExit=yes
ExecStart=/usr/sbin/ip link add yofc-test0 type dummy
ExecStart=/usr/sbin/ip link set yofc-test0 up
[Install]
WantedBy=multi-user.target
''')
    Path('/etc/systemd/system/yofc-management-phase2-agent.service').write_text('''[Unit]
Description=YOFC network transaction disposable fixture
After=network.target yofc-management-phase2-link.service
Wants=yofc-management-phase2-link.service
[Service]
Type=simple
EnvironmentFile=/etc/yofc-management-phase2/agent.env
ExecStart=/opt/yofc-management-phase2/agent
Restart=always
RestartSec=2
UMask=0077
RuntimeDirectory=yofc-management-phase2
RuntimeDirectoryMode=0700
StateDirectory=yofc-management-phase2
StateDirectoryMode=0700
[Install]
WantedBy=multi-user.target
''')
    run(['systemctl','daemon-reload'])
    run(['systemctl','enable','--now','yofc-management-phase2-link.service','yofc-management-phase2-agent.service'])
    print('Disposable network fixture ready; eth0 configuration retained.')
if __name__=='__main__':inside() if sys.argv[1:]==['--inside-fixture'] else host()
