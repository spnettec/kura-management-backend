#!/bin/bash
# 在网关内以 root 执行：把中心签发的 leaf 证书与两个 CA 放入登记目录并生成 stage 槽位。
# 变量：GW_ID 网关标识  RID 本次登记请求的 UUID  ENDPOINT 中心的 gRPC 地址
#       FILES   宿主 httpd 地址   GW_FILES 该网关专属文件的 httpd 地址   TTY 可选
set -euo pipefail
say() { printf '%s  %s\n' "$(date '+%H:%M:%S')" "$*" | { if [ -n "${TTY:-}" ]; then tee -a "$TTY"; else cat; fi; } || true; }
install -d -m 0700 /tmp/e2e /tmp/enroll
curl -fsS -m 60 -o /tmp/e2e/gw-stage-final.py "$FILES/gw-stage-final.py"
curl -fsS -m 60 -o /tmp/enroll/leaf.pem "$GW_FILES/leaf.pem"
curl -fsS -m 60 -o /tmp/enroll/issuer-ca.pem "$GW_FILES/issuer-ca.pem"
curl -fsS -m 60 -o /tmp/enroll/console-ca.pem "$GW_FILES/console-ca.pem"
say "[2/4] stage enrollment material (request $RID)"
python3 /tmp/e2e/gw-stage-final.py "$RID" "$GW_ID" "$ENDPOINT"
