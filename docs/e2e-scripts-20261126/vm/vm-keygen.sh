#!/bin/bash
# 在网关内以 root 执行：生成网关自己的 P-256 密钥与 CSR（私钥不出网关）。
# 变量：GW_ID 网关标识（写入 CSR 的 CN）  FILES 宿主 httpd 地址  TTY 可选
set -euo pipefail
say() { printf '%s  %s\n' "$(date '+%H:%M:%S')" "$*" | { if [ -n "${TTY:-}" ]; then tee -a "$TTY"; else cat; fi; } || true; }
install -d -m 0700 /tmp/e2e /tmp/enroll
rm -f /tmp/enroll/gw.key /tmp/enroll/gw.pk8 /tmp/enroll/gw.csr
curl -fsS -m 60 -o /tmp/e2e/gw-keygen.py "$FILES/gw-keygen.py"
say "== YOFC Kura gateway $GW_ID: enrollment =="
say "[1/4] keygen: generate key and CSR"
GW_ID="$GW_ID" python3 /tmp/e2e/gw-keygen.py > /tmp/e2e/gw.csr
echo "-----CSR-BEGIN-----"
cat /tmp/e2e/gw.csr
echo "-----CSR-END-----"
