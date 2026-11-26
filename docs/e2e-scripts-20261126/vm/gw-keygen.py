#!/usr/bin/env python3
"""Gateway enrollment step 1 (runs on the gateway as root): generate the P-256
key and CSR; print the CSR. Materials are produced with the gateway's own
OpenSSL so the SPKI carries a named-curve OID (the agent rejects explicit-
parameter encodings produced by some LibreSSL builds)."""
import base64
import os
import secrets
import subprocess

WORK = "/tmp/enroll"
os.makedirs(WORK, mode=0o700, exist_ok=True)
os.chmod(WORK, 0o700)
env = dict(os.environ, OPENSSL_CONF="/dev/null")


def ssl(*args, **kw):
    return subprocess.run(["openssl", *args], check=True, capture_output=True, env=env, **kw).stdout


if not os.path.exists(WORK + "/gw.key"):
    ssl("genpkey", "-algorithm", "EC", "-pkeyopt", "ec_paramgen_curve:P-256", "-out", WORK + "/gw.key")
    # The agent requires a real PKCS#8 DER private key; `pkey -outform DER`
    # emits the legacy SEC1 encoding and is refused.
    ssl("pkcs8", "-topk8", "-nocrypt", "-in", WORK + "/gw.key", "-outform", "DER", "-out", WORK + "/gw.pk8")
    ssl("req", "-new", "-key", WORK + "/gw.key", "-subj", "/CN=" + os.environ["GW_ID"], "-out", WORK + "/gw.csr")
for f in ("gw.key", "gw.pk8"):
    os.chmod(f"{WORK}/{f}", 0o600)
print(open(WORK + "/gw.csr").read())
