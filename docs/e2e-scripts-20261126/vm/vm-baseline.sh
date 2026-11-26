#!/bin/bash
# 在虚拟机内以 root 执行：核对它处于未安装任何 Kura 组件的干净基线。
# 第四台网关用的 Parallels 虚拟机靠快照还原回到基线；每次还原后都先跑这一步。
set -euo pipefail
kura=$(dpkg -l 2>/dev/null | grep -cE '^ii  kura' || true)
echo "hostname=$(hostname)  arch=$(dpkg --print-architecture)  uptime=$(cut -d. -f1 /proc/uptime)s"
echo "now=$(date -u '+%Y-%m-%dT%H:%M:%SZ')"
echo "kura packages installed: $kura"
echo "/opt/eclipse/kura present: $([ -e /opt/eclipse/kura ] && echo yes || echo no)"
echo "agent state entries: $(ls /var/lib/kura-management-agent-runtime 2>/dev/null | wc -l)"
[ "$kura" = 0 ]
[ ! -e /opt/eclipse/kura ]
[ ! -e /var/lib/kura-management-agent-runtime ]
echo "BASELINE-CLEAN"
