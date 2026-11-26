#!/bin/bash
# 07 登录管理中心后的页面截图。00-run-all.sh 成功后单独执行：网关总览一屏看下四台网关。
#    依赖宿主的 Node 22+ 与 Google Chrome。只截图，不往 steps.tsv 里写（那是 00-run-all.sh 的记录）。
set -uo pipefail
. "$(dirname "$0")/e2e-common.sh"

step "07 登录后的页面截图"
api login >/dev/null || die "管理员登录失败：先确认 00-run-all.sh 已经成功"
node "$SCRIPT_DIR/console-shots.mjs" --origin "$CONSOLE_ORIGIN" --out "$SHOTS" --user "$ADMIN_USER" --password "$ADMIN_PASSWORD" \
    || die "截图失败"
