#!/usr/bin/env python3
"""One ownership-transfer acceptance, confined to the named disposable VM.

--prepare installs only the two dedicated dummy links/source for browser checks.
The default run also reboots the VM and finally commits ownership, preserving IPs.
"""
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location("network_smoke", ROOT / "scripts/phase2-network-smoke.py")
n = importlib.util.module_from_spec(spec)
spec.loader.exec_module(n)
SOURCE = "/etc/netplan/40-yofc-existing-fixture.yaml"
IFACE = "yofc-adopt0"
checks = []


def check(name, value):
    if not value:
        raise AssertionError(name)
    checks.append(name)
    (ROOT / "target/takeover-runtime-progress.json").write_text(json.dumps({"passed":len(checks),"checks":checks},indent=2)+"\n")


def plan():
    return n.local("network.takeover.plan", {"interface": IFACE})


def request(p):
    return {"interface": IFACE, "expectedRevision": p["revision"], "sourceDigest": p["sourceDigest"], "planDigest": p["planDigest"], "confirmSeconds": 120}


def values(interface):
    return json.loads(n.orb("ip", "-j", "address", "show", "dev", interface))[0]


def reboot():
    before = n.orb("cat", "/proc/sys/kernel/random/boot_id").strip()
    subprocess.run(["orb","restart",n.VM],check=True,capture_output=True,text=True)
    deadline = time.monotonic() + 45
    while time.monotonic() < deadline:
        try:
            after = n.orb("cat", "/proc/sys/kernel/random/boot_id").strip()
            if after != before and n.orb("systemctl","is-active","yofc-management-phase2-agent.service").strip()=="active":
                return after
        except subprocess.CalledProcessError:
            pass
        time.sleep(.5)
    raise AssertionError("VM did not reboot and recover")


def setup():
    assert n.local("network.inspect")["revision"] == 1, "fixture already advanced; read its saved takeover evidence instead of reinitializing it"
    n.orb("python3", "-c", '''from pathlib import Path
import subprocess
source=Path('/etc/netplan/40-yofc-existing-fixture.yaml')
if not source.exists():
 source.write_text("""network:
  version: 2
  renderer: networkd
  ethernets:
    yofc-adopt0:
      dhcp4: false
      dhcp6: false
      accept-ra: false
      addresses: [192.0.2.90/24, '2001:db8::90/64']
      nameservers: {addresses: [192.0.2.53], search: [example.test]}
      routes: [{to: 198.18.0.0/24, via: 192.0.2.1, metric: 101}]
      mtu: 1400
      optional: true
    yofc-preserve0:
      dhcp4: false
      addresses: [198.51.100.50/24]
      mtu: 1450
      optional: true
""")
 source.chmod(0o640)
unit=Path('/etc/systemd/system/yofc-management-takeover-links.service')
unit.write_text("""[Unit]
Description=Disposable ownership transfer links
Before=yofc-management-phase2-agent.service
After=yofc-management-phase2-link.service
[Service]
Type=oneshot
RemainAfterExit=yes
ExecStart=/usr/sbin/ip link add yofc-adopt0 type dummy
ExecStart=/usr/sbin/ip link set yofc-adopt0 up
ExecStart=/usr/sbin/ip link add yofc-preserve0 type dummy
ExecStart=/usr/sbin/ip link set yofc-preserve0 up
[Install]
WantedBy=multi-user.target
""")
p=Path('/etc/yofc-management-phase2/agent.env')
p.write_text(p.read_text().replace('AGENT_NETWORK_INTERFACES=yofc-test0\\n','AGENT_NETWORK_INTERFACES=yofc-test0,yofc-adopt0\\n'))
subprocess.run(['systemctl','daemon-reload'],check=True)
subprocess.run(['systemctl','enable','--now','yofc-management-takeover-links.service'],check=True)
subprocess.run(['netplan','apply'],check=True,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
subprocess.run(['systemctl','restart','yofc-management-phase2-agent.service'],check=True)
''')
    time.sleep(1)
    p = plan()
    (ROOT/"target/takeover-plan.json").write_text(json.dumps(p,indent=2)+"\n")
    assert p["eligible"],p


def main():
    if sys.argv[1:] not in [[n.VM], [n.VM,"--prepare"]]:
        raise SystemExit(f"usage: {sys.argv[0]} {n.VM} [--prepare]")
    setup()
    if sys.argv[-1] == "--prepare":
        print("Known networkd source ready for explicit browser transfer review")
        return
    c=n.m.Client();c.login("admin")
    original=n.orb("sha256sum",SOURCE)
    eth0=n.orb("sha256sum","/etc/netplan/10-lxc.yaml")
    default=n.orb("ip","-j","-4","route","show","default")
    before=values(IFACE)
    p=plan()
    check("single reviewed networkd source eligible",p["eligible"] and p["sourceMode"]=="0640" and p["preservedOtherInterfaces"]==["yofc-preserve0"])
    stale={**request(p),"planDigest":"0"*64}
    status,receipt=c.request(f"/api/v1/gateways/{n.GATEWAY}/tasks","POST",{"operation":"network.takeover.prepare","idempotencyKey":__import__('uuid').uuid4().hex,"networkTakeover":stale})
    check("stale plan admitted as task",status==202)
    failed=n.m.wait_task(c,receipt["id"])
    check("stale plan rejected before mutation",failed["state"]=="FAILED" and failed["error"]=="CONFLICT" and n.orb("sha256sum",SOURCE)==original)
    j=n.remote(c,"network.takeover.prepare",networkTakeover=request(p))
    check("real staged takeover awaits confirmation",j["state"]=="WAIT_CONFIRM")
    check("persistent original bytes unchanged while pending",n.orb("sha256sum",SOURCE)==original)
    check("volatile normalization shadows original source",n.orb("test","-f","/run/netplan/40-yofc-existing-fixture.yaml")=="")
    check("candidate retains original IPv4 and IPv6",{a['local'] for a in values(IFACE)['addr_info']}=={a['local'] for a in before['addr_info']})
    j=n.remote(c,"network.rollback",networkAction=n.action(j))
    check("explicit rollback returns original ownership",j["state"]=="ROLLED_BACK" and n.orb("sha256sum",SOURCE)==original)
    j=n.remote(c,"network.takeover.prepare",networkTakeover=request(plan()))
    reboot()
    v=n.wait_state("ROLLED_BACK")
    check("unconfirmed real reboot preserves original source",v['revision']==1 and n.orb("sha256sum",SOURCE)==original)
    check("runtime source shadow vanishes on reboot",n.orb("test","!","-e","/run/netplan/40-yofc-existing-fixture.yaml")=="")
    p=plan()
    j=n.remote(c,"network.takeover.prepare",networkTakeover=request(p))
    j=n.remote(c,"network.confirm",networkAction=n.action(j))
    check("fresh mTLS stream commits takeover",j['state']=="COMMITTED" and j['revision']==2)
    content=n.orb("cat",SOURCE)
    check("source normalization preserves unrelated interface",'yofc-adopt0' not in content and 'yofc-preserve0' in content)
    check("source permissions preserved",n.orb("stat","-c","%a",SOURCE).strip()=="640")
    check("preserved interface runtime unchanged",values('yofc-preserve0')['mtu']==1450 and any(a['local']=='198.51.100.50' for a in values('yofc-preserve0')['addr_info']))
    reboot()
    check("confirmed transfer survives real reboot",n.local("network.inspect")['revision']==2 and any(a['local']=='192.0.2.90' for a in values(IFACE)['addr_info']) and any(a['local']=='2001:db8::90' for a in values(IFACE)['addr_info']))
    check("eth0 persistent source preserved",n.orb("sha256sum","/etc/netplan/10-lxc.yaml")==eth0)
    check("management default route preserved",n.orb("ip","-j","-4","route","show","default")==default)
    check("Kura absence does not prevent transfer",n.local('agent.status')['kuraAvailable'] is False)
    output={"passed":len(checks),"checks":checks,"vm":n.VM,"revision":2,"interface":IFACE,"limitations":["explicit single networkd source only","NM/cloud-init handover and DHCP matrix pending","dummy links and graceful reboot, not physical/hard-power acceptance"]}
    (ROOT/'target/takeover-runtime.json').write_text(json.dumps(output,indent=2)+'\n')
    print(json.dumps(output,indent=2))


if __name__ == '__main__':main()
