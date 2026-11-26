#!/bin/bash
# 在网关内以 root 执行：完成登记并连通中心。
#   启用两个 relay socket → 停 kura 释放桥登记位 → 以 kurad 身份 register+bind
#   → 启用远程通道 → 启动 kura → 轮询 remote.status 直到 CONNECTED
# 变量：RID 登记请求的 UUID  SHA stage 槽位的 SHA-256   FILES 宿主 httpd 地址   TTY 可选
set -euo pipefail
say() { printf '%s  %s\n' "$(date '+%H:%M:%S')" "$*" | { if [ -n "${TTY:-}" ]; then tee -a "$TTY"; else cat; fi; } || true; }
install -d -m 0700 /tmp/e2e
curl -fsS -m 60 -o /tmp/e2e/gw-enroll-final.py "$FILES/gw-enroll-final.py"
say "[3/4] relay sockets, kurad register + bind"
say "[4/4] enable remote channel, start kura, wait for CONNECTED"
python3 /tmp/e2e/gw-enroll-final.py "$RID" "$SHA"
say "== enrollment done: $(python3 -c 'import json; print(json.load(open("/var/lib/kura-management-agent-runtime/enrollment/identity.json"))["actor"])' 2>/dev/null || echo "?") =="
