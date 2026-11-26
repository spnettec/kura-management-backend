#!/bin/bash
# 06 终验：管理中心里 4 台网关都是 online=true 且 kuraAvailable=true。
set -uo pipefail
. "$(dirname "$0")/e2e-common.sh"

step "06 终验"
api login || die "管理员登录失败"
api list | tee "$E2E_ROOT/verify.txt"
missing=0
for gw in "${GW_ALL[@]}"; do
    if grep -qE "^${gw} +online=True +kuraAvailable=True" "$E2E_ROOT/verify.txt"; then
        note "${gw} online + kuraAvailable"
    else
        note "${gw} 未达标"; missing=$((missing + 1))
    fi
done
if [ "$missing" = 0 ]; then
    record OK verify "${#GW_ALL[@]}/${#GW_ALL[@]} online + kuraAvailable"
else
    record FAIL verify "${missing} 台未达标"
    exit 1
fi
