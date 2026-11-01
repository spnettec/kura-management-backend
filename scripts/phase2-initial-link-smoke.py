#!/usr/bin/env python3
"""Regression of initial-link compensation in the fixed disposable VM only."""
import importlib.util
import argparse
import json
from pathlib import Path
import sys
import time
import re

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location("network_smoke", ROOT / "scripts/phase2-network-smoke.py")
n = importlib.util.module_from_spec(spec)
spec.loader.exec_module(n)
IFACE = "yofc-first0"
ENV = "/etc/yofc-management-phase2/agent.env"
SERVICE = "yofc-management-phase2-agent.service"
checks = []


def check(name, value):
    if not value:
        raise AssertionError(name)
    checks.append(name)


def link():
    return json.loads(n.orb("ip", "-j", "address", "show", "dev", IFACE))[0]


def wait_local():
    for _ in range(50):
        try:
            return n.local("network.inspect")
        except Exception:
            time.sleep(.1)
    raise AssertionError("local service did not restart")


def change():
    view = n.local("network.inspect")
    return {
        "expectedRevision": view["revision"], "sourceDigest": view["sourceDigest"],
        "confirmSeconds": 120,
        "config": {"interface": IFACE, "dhcp4": False, "addresses": ["192.0.2.90/24"],
                   "routes": [], "nameservers": [], "searchDomains": [], "mtu": 1400,
                   "optional": True, "dhcpMetric": 100},
    }


def reject(name):
    before = n.local("network.inspect")["transaction"]
    try:
        n.local("network.prepare", change())
    except AssertionError as error:
        check(name, "'error': 'CONFLICT'" in str(error))
    else:
        raise AssertionError("unowned runtime was accepted")
    check(name + " has no candidate", n.orb("test", "!", "-e", "/run/netplan/90-yofc-managed.yaml") == "")
    check(name + " leaves journal unchanged", n.local("network.inspect")["transaction"] == before)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('vm', choices=[n.VM])
    parser.add_argument('--evidence-prefix', default='phase2-initial-link')
    args = parser.parse_args()
    if not re.fullmatch(r'[a-z0-9-]+', args.evidence_prefix):
        parser.error('invalid evidence prefix')
    baseline = n.local('network.inspect')['revision']
    original_env = n.orb('cat', ENV)
    original_delegation = next(line for line in original_env.splitlines() if line.startswith('AGENT_NETWORK_INTERFACES='))
    allowed = set(original_delegation.split('=', 1)[1].split(','))
    assert allowed == ({'yofc-test0', 'yofc-adopt0'} if baseline >= 2 else {'yofc-test0'}), allowed
    eth0 = n.orb("sha256sum", "/etc/netplan/10-lxc.yaml")
    default = n.orb("ip", "-j", "-4", "route", "show", "default")
    # The script owns this dummy link; no other interface is delegated or changed.
    n.orb("python3", "-c", """from pathlib import Path
import subprocess
p=Path('/etc/yofc-management-phase2/agent.env')
s=p.read_text()
lines=s.splitlines(keepends=True)
s=''.join(line.rstrip('\\n')+',yofc-first0\\n' if line.startswith('AGENT_NETWORK_INTERFACES=') else line for line in lines)
p.write_text(s)
if not Path('/sys/class/net/yofc-first0').exists():
 subprocess.run(['ip','link','add','yofc-first0','type','dummy'],check=True)
subprocess.run(['systemctl','restart','yofc-management-phase2-agent.service'],check=True)
""")
    wait_local()
    # A previous diagnostic run may have exposed the original MTU recovery bug.
    # Retry compensation through the agent rather than editing its journal.
    pending = n.local("network.inspect")["transaction"]
    if pending and pending["state"] == "RECOVERY_REQUIRED":
        check("previous initial-link recovery completes", n.local("network.rollback", n.action(pending))["state"] == "ROLLED_BACK")
    if pending and pending["state"] not in ["COMMITTED", "ROLLED_BACK", "RECOVERY_REQUIRED"]:
        raise AssertionError("finish the existing transaction before reinitializing the fixture link")
    n.orb("ip", "link", "del", IFACE)
    n.orb("ip", "link", "add", IFACE, "type", "dummy")
    n.orb("ip", "link", "set", "dev", IFACE, "mtu", "1500", "up")
    n.orb("ip", "address", "add", "192.0.2.40/24", "dev", IFACE)
    reject("manual IPv4 is not adopted")
    n.orb("ip", "address", "del", "192.0.2.40/24", "dev", IFACE)
    # Keep this manual route distinct from the preserved takeover fixture subnet.
    n.orb("ip", "route", "add", "198.19.100.0/24", "dev", IFACE)
    reject("manual route is not adopted")
    n.orb("ip", "route", "del", "198.19.100.0/24", "dev", IFACE)
    n.orb("ip", "-6", "address", "add", "2001:db8::40/64", "dev", IFACE)
    reject("global IPv6 is not adopted")
    n.orb("ip", "-6", "address", "del", "2001:db8::40/64", "dev", IFACE)
    j = n.local("network.prepare", change())
    current = link()
    check("initial candidate changes actual address and MTU", j["state"] == "WAIT_CONFIRM" and current["mtu"] == 1400 and any(a.get("local") == "192.0.2.90" for a in current["addr_info"]))
    result = n.local("network.rollback", n.action(j))
    current = link()
    check("explicit initial rollback succeeds", result["state"] == "ROLLED_BACK")
    check("initial rollback restores actual MTU", current["mtu"] == 1500)
    check("initial rollback removes candidate address", not any(a["family"] == "inet" for a in current["addr_info"]))
    check("initial rollback preserves administrative up", "UP" in current["flags"])
    j = n.local("network.prepare", change())
    n.orb("ip", "link", "set", "dev", IFACE, "mtu", "1450")
    result = n.local("network.rollback", n.action(j))
    check("external MTU drift requires explicit recovery", result["state"] == "RECOVERY_REQUIRED")
    check("rollback does not overwrite foreign MTU", link()["mtu"] == 1450)
    # The test operator restores the original value before retrying compensation.
    n.orb("ip", "link", "set", "dev", IFACE, "mtu", "1500")
    check("explicit recovery can finish after operator correction", n.local("network.rollback", n.action(j))["state"] == "ROLLED_BACK")
    n.orb("ip", "link", "set", "dev", IFACE, "down")
    j = n.local("network.prepare", change())
    check("initially down link has an applied candidate", j["state"] == "WAIT_CONFIRM" and link()["mtu"] == 1400)
    n.orb("systemctl", "restart", SERVICE)
    wait_local()
    result = n.wait_state("ROLLED_BACK")
    current = link()
    check("restart restores first-link MTU and down state", result["transaction"]["state"] == "ROLLED_BACK" and current["mtu"] == 1500 and "UP" not in current["flags"])
    check("restart leaves no candidate IPv4", not any(a["family"] == "inet" for a in current["addr_info"]))
    check("confirmed baseline revision preserved", result["revision"] == baseline)
    check("eth0 source preserved", n.orb("sha256sum", "/etc/netplan/10-lxc.yaml") == eth0)
    check("management default route preserved", n.orb("ip", "-j", "-4", "route", "show", "default") == default)
    # Restore the installer-owned delegation after successful compensation.
    n.orb("python3", "-c", """from pathlib import Path
import sys
p=Path('/etc/yofc-management-phase2/agent.env')
s=p.read_text()
p.write_text(''.join(sys.argv[1]+'\\n' if line.startswith('AGENT_NETWORK_INTERFACES=') else line for line in s.splitlines(keepends=True)))
""", original_delegation)
    n.orb("systemctl", "restart", SERVICE)
    wait_local()
    n.orb("ip", "link", "del", IFACE)
    check('original delegation preserved', n.orb('cat', ENV) == original_env)
    output = {"passed": len(checks), "checks": checks, "vm": n.VM,
              "baselineRevision": baseline,
              "limitations": ["dummy Ethernet fixture", "no takeover of pre-existing runtime configuration", "DHCP/resolver and physical link matrix still pending"]}
    (ROOT / f"target/{args.evidence_prefix}.json").write_text(json.dumps(output, ensure_ascii=False, indent=2) + "\n")
    print(json.dumps(output, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
