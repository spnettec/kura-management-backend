#!/usr/bin/env python3
"""Gateway enrollment completion (runs on the gateway as root).
Sequence: relay sockets on -> stop kura (frees the bridge register slot) ->
kurad register+bind (single UDS connection, kura-uid receipt) -> set-enabled
(root local controller) -> start kura -> poll remote.status until CONNECTED.
Usage: gw-enroll-final.py <slot-rid> <stage-sha256>"""
import hashlib
import json
import glob
import socket
import struct
import subprocess
import sys
import time
import uuid

RID, SHA = sys.argv[1], sys.argv[2]
STATE = "/var/lib/kura-management-agent-runtime"
SOCK = "/run/kura-management-agent-runtime/control.sock"
OPERATOR = "yofc-enroll"
RELAY_SOCKETS = ["kura-management-agent-enrollment.socket",
                 "kura-management-enrollment-reader.socket"]
METHODS = ["kura.configuration.list", "kura.snapshot.list", "kura.configuration.get",
           "kura.network.roles", "kura.firewall.source", "kura.firewall.origin",
           "kura.network.fence", "kura.network.owner", "kura.network.startup.profile",
           "kura.network.template.export", "kura.configuration.outcome",
           "kura.configuration.update", "kura.snapshot.save", "kura.snapshot.rollback",
           "kura.recovery.verify", "kura.recovery.complete", "kura.ordinary.verify"]


def frame(kind, method=None, params=None):
    b = dict(version=1, kind=kind, requestId=str(uuid.uuid4()))
    if method:
        b["method"] = method
    if params is not None:
        b["params"] = params
    wire = json.dumps(b, sort_keys=True, separators=(",", ":")).encode() + b"\n"
    return struct.pack("!I", len(wire)) + wire


def rf(c):
    h = b""
    while len(h) < 4:
        ch = c.recv(4 - len(h))
        if not ch:
            raise RuntimeError("connection closed by agent (handshake rejected?)")
        h += ch
    (n,) = struct.unpack("!I", h)
    d = b""
    while len(d) < n:
        ch = c.recv(n - len(d))
        if not ch:
            raise RuntimeError("closed mid-frame")
        d += ch
    return json.loads(d)


# 0. Verify the staged slot matches the arguments before touching any unit.
manifests = glob.glob("/opt/eclipse/kura_*_linux/data/management-enrollment/"
                      + RID + "/certificates/manifest.json")
if len(manifests) != 1:
    sys.exit("staged slot not found for " + RID)
actual = hashlib.sha256(open(manifests[0], "rb").read()).hexdigest()
if actual != SHA:
    sys.exit("stage sha mismatch: slot=%s arg=%s" % (actual, SHA))
print("slot verified:", RID)

# 1. Relay sockets must be enabled+active (contract; idempotent).
subprocess.run(["systemctl", "enable", "--now", *RELAY_SOCKETS], check=True)

# 2. Free the bridge register slot: stop kura, wait for the agent to reap EOF.
was_active = subprocess.run(["systemctl", "is-active", "--quiet", "kura.service"]).returncode == 0
if was_active:
    subprocess.run(["systemctl", "stop", "kura.service"], check=True)
    while subprocess.run(["systemctl", "is-active", "--quiet", "kura.service"]).returncode == 0:
        time.sleep(1)
    time.sleep(3)
print("kura stopped, bridge slot free")

# 3. Kura peer (kurad) register handshake + bind: single connection, kura-uid receipt.
bind_program = r'''
import json, socket, struct, sys, uuid
revision, rid, sha, operator = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[5]
METHODS = json.loads(sys.argv[4])
def frame(kind, method=None, params=None):
    b = dict(version=1, kind=kind, requestId=str(uuid.uuid4()))
    if method: b["method"] = method
    if params is not None: b["params"] = params
    w = json.dumps(b, sort_keys=True, separators=(",", ":")).encode() + b"\n"
    return struct.pack("!I", len(w)) + w
def rf(c):
    h = b""
    while len(h) < 4:
        ch = c.recv(4-len(h))
        if not ch: raise RuntimeError("connection closed by agent (handshake rejected?)")
        h += ch
    (n,) = struct.unpack("!I", h)
    d = b""
    while len(d) < n:
        ch = c.recv(n-len(d))
        if not ch: raise RuntimeError("closed mid-frame")
        d += ch
    return json.loads(d)
with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as c:
    c.settimeout(90); c.connect(sys.argv[6])
    c.sendall(frame("register", "kura.register", {"methods": METHODS}))
    ack = rf(c)
    if "error" in ack: print("REGISTER-ERROR", ack["error"]); sys.exit(2)
    c.sendall(frame("request", "remote.bind", {"expectedRevision": revision, "requestId": rid,
        "expectedStageSha256": sha, "operator": operator}))
    print(json.dumps(rf(c)))
'''
rev = json.load(open(STATE + "/remote-management.json"))["revision"]
proc = subprocess.run(["setpriv", "--reuid", "kurad", "--regid", "kurad", "--init-groups",
                       "python3", "-I", "-c", bind_program,
                       rev, RID, SHA, json.dumps(METHODS), OPERATOR, SOCK],
                      capture_output=True, text=True, timeout=120, cwd="/")
print("kurad rc:", proc.returncode)
print("kurad stdout:", proc.stdout.strip())
if proc.stderr.strip():
    print("kurad stderr:", proc.stderr.strip())
if proc.returncode != 0:
    sys.exit("kurad bind failed")

# 4. Enable the channel (root local controller) — the bind rewrote the decision.
with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as c:
    c.settimeout(60)
    c.connect(SOCK)
    c.sendall(frame("request", "remote.status", {}))
    st = rf(c)["result"]
    if st.get("enabled"):
        print("already enabled:", st.get("connectionState"))
    else:
        c.sendall(frame("request", "remote.set-enabled",
                        {"expectedRevision": st["revision"], "enabled": True}))
        res = rf(c).get("result", {})
        print("enabled:", res.get("connectionState"), res.get("enabled"))

# 5. Start kura; it re-registers over the bound channel. Poll until CONNECTED.
subprocess.run(["systemctl", "start", "kura.service"], check=True)
deadline = time.time() + 240
final = {}
last = None
while time.time() < deadline:
    time.sleep(5)
    try:
        with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as c:
            c.settimeout(20)
            c.connect(SOCK)
            c.sendall(frame("request", "remote.status", {}))
            final = rf(c)["result"]
    except Exception as e:
        print("status poll error:", e)
        continue
    cur = final.get("connectionState")
    if cur != last:
        print("state:", cur, "| bound:", final.get("bound"), "| enabled:", final.get("enabled"))
        last = cur
    if cur == "CONNECTED" and final.get("bound"):
        break
print("FINAL:", final.get("connectionState"), "| bound:", final.get("bound"),
      "| enabled:", final.get("enabled"))
if not (final.get("connectionState") == "CONNECTED" and final.get("bound")):
    sys.exit(3)
receipt = json.load(open(STATE + "/enrollment/identity.json"))
print("receipt actor:", receipt["actor"])
