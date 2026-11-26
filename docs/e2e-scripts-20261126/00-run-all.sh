#!/bin/bash
# 00 一键完整部署：3 台 OrbStack + 1 台 Parallels 作为网关，管理中心跑在 Docker。
#    任一步失败立即停止并以非 0 退出；每步结果追加到 $E2E_ROOT/steps.tsv。
#
#    用法：
#      CORE_DEB=<arm64 kura-core deb>  AGENT_DEB=<arm64 agent deb>  E2E_ROOT=<工作目录> \
#      ./00-run-all.sh
set -uo pipefail
cd "$(dirname "$0")"
export CORE_DEB="${CORE_DEB:-}" AGENT_DEB="${AGENT_DEB:-}" E2E_ROOT="${E2E_ROOT:-}"
. ./e2e-common.sh
: > "$E2E_ROOT/steps.tsv"

step "YOFC Kura E2E 部署开始   $(date '+%Y-%m-%d %H:%M:%S')"
note "E2E_ROOT = $E2E_ROOT"
failed=""
for part in 01-wipe 02-ca 03-console; do
    bash "./$part.sh" || { failed="$part"; break; }
done
if [ -z "$failed" ]; then
    for gw in "${GW_ALL[@]}"; do
        for part in 04-gateway-install 05-gateway-enroll; do
            bash "./$part.sh" "$gw" || { failed="$part $gw"; break; }
        done
        [ -z "$failed" ] || break
    done
fi
[ -n "$failed" ] || { bash ./06-verify.sh || failed="06-verify"; }

echo
step "结果"
column -t -s "$(printf '\t')" "$E2E_ROOT/steps.tsv"
if [ -n "$failed" ]; then echo "FAILED at: $failed"; exit 1; fi
if [ "$(grep -c FAIL "$E2E_ROOT/steps.tsv")" != 0 ]; then echo "有失败的步骤"; exit 1; fi
echo "ALL STEPS PASSED"
