#!/bin/bash
# 01 清空上一轮环境：OrbStack 网关虚拟机、管理中心容器与数据卷、PostgreSQL 容器；
#    Parallels 虚拟机停机（还原到基线由 04 在停机状态下做）。
set -uo pipefail
. "$(dirname "$0")/e2e-common.sh"

step "01 清空上一轮环境"
for gw in "${GW_ORB[@]}"; do
    if orb_exists "$gw"; then
        orbctl delete "$gw" >/dev/null 2>&1 || die "删除 ${gw} 失败"
        note "已删除 OrbStack 虚拟机 ${gw}"
    else
        note "${gw} 不存在"
    fi
done
orb_list_names > "$E2E_ROOT/.orb-after" || die "无法列出 OrbStack 虚拟机"
[ -z "$(grep -E '^yofc-gw-' "$E2E_ROOT/.orb-after")" ] || die "仍有网关虚拟机未删除：$(grep -E '^yofc-gw-' "$E2E_ROOT/.orb-after" | tr '\n' ' ')"
(cd "$CONSOLE_REPO/docker" && docker compose down -v --remove-orphans) >/dev/null 2>&1
note "管理中心容器与数据卷已移除"
if docker rm -f -v yofc-e2e-pg >/dev/null 2>&1; then note "已删除 PostgreSQL 容器"; else note "PostgreSQL 容器不存在"; fi
case "$(parallels_state)" in
    running) prlctl stop "$PARALLELS_VM" >/dev/null 2>&1 && note "Parallels 虚拟机已停机" || die "Parallels 虚拟机停机失败";;
    stopped) note "Parallels 虚拟机已停机";;
    *) die "Parallels 虚拟机状态异常：$(parallels_state)";;
esac
pkill -f "http.server ${HTTP_PORT}" 2>/dev/null && note "已停止上一轮的 httpd（端口 ${HTTP_PORT}）" || note "没有运行中的 httpd"
record OK wipe "上一轮环境已清空"
