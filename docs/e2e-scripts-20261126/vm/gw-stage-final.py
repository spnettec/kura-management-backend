#!/usr/bin/env python3
"""Gateway enrollment staging (runs on the gateway as root).
Reads key material from /tmp/enroll and issued certs from /tmp/enroll:
  gw.pk8 gw.csr leaf.pem issuer-ca.pem console-ca.pem
Usage: gw-stage-final.py <request-uuid> <gateway-id> <endpoint>"""
import hashlib
import json
import os
import pwd
import grp
import subprocess
import sys
import uuid

RID, GW_ID, ENDPOINT = sys.argv[1], sys.argv[2], sys.argv[3]
uuid.UUID(RID)
CORE = os.path.realpath("/opt/eclipse/kura")
DATA = CORE + "/data"
SRC = DATA + "/management-enrollment"
SLOT = SRC + "/" + RID
WORK = "/tmp/enroll"
uid, gid = pwd.getpwnam("kurad").pw_uid, grp.getgrnam("kurad").gr_gid
env = dict(os.environ, OPENSSL_CONF="/dev/null")


def ssl(*args, **kw):
    return subprocess.run(["openssl", *args], check=True, capture_output=True, env=env, **kw).stdout


def sha(b):
    return hashlib.sha256(b).hexdigest()


def put(path, data, mode=0o600):
    fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_TRUNC | os.O_NOFOLLOW, mode)
    with os.fdopen(fd, "wb") as out:
        out.write(data)
    os.chown(path, uid, gid)
    os.chmod(path, mode)


def mkdir(path):
    os.makedirs(path, mode=0o700, exist_ok=True)
    os.chown(path, uid, gid)
    os.chmod(path, 0o700)


leaf = open(WORK + "/leaf.pem", "rb").read()
issuer = open(WORK + "/issuer-ca.pem", "rb").read()
console = open(WORK + "/console-ca.pem", "rb").read()
leaf_der = ssl("x509", "-in", WORK + "/leaf.pem", "-outform", "DER")
pub_der = ssl("pkey", "-in", WORK + "/gw.key", "-pubout", "-outform", "DER")
not_after = ssl("x509", "-in", WORK + "/leaf.pem", "-noout", "-enddate", "-dateopt", "iso_8601") \
    .decode().strip().removeprefix("notAfter=").replace(" ", "T")

manifest = {
    "schema": "yofc.kura.remote.certificates.v1", "requestId": RID, "gatewayId": GW_ID,
    "endpoint": ENDPOINT, "publicKeySha256": sha(pub_der), "clientCertificateSha256": sha(leaf_der),
    "issuerCaSha256": sha(issuer), "consoleCaSha256": sha(console), "notAfter": not_after,
}
raw = json.dumps(manifest, sort_keys=True, separators=(",", ":")).encode()
stage_sha = sha(raw)

mkdir(SRC)
mkdir(SLOT)
mkdir(SLOT + "/certificates")
put(SLOT + "/private.pk8", open(WORK + "/gw.pk8", "rb").read())
put(SLOT + "/public.der", pub_der)
put(SLOT + "/request.pem", open(WORK + "/gw.csr", "rb").read())
put(SLOT + "/gateway.id", GW_ID.encode())
put(SLOT + "/complete", b"kura.remote.identity.v1\n")
put(SLOT + "/certificates/client-chain.pem", leaf)
put(SLOT + "/certificates/issuer-ca.pem", issuer)
put(SLOT + "/certificates/console-ca.pem", console)
put(SLOT + "/certificates/manifest.json", raw)
put(SLOT + "/certificates/complete", (stage_sha + "\n").encode())
# Kura-side context marker and lock for the retained-enrollment inspector.
put(DATA + "/management-enrollment-context-v1",
    ("yofc.kura.remote-request-context.v1\n" + str(uuid.uuid4()) + "\n").encode())
put(SRC + "/.enrollment.lock", b"")
print("STAGE-SHA", stage_sha)
