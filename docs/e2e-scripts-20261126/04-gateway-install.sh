#!/bin/bash
# 04 准备一台网关虚拟机并完成安装：依赖 → 两个 deb → initialize → 两项本地授权 → 启动两个服务。
#    用法：04-gateway-install.sh <网关ID>      yofc-gw-1..3 为 OrbStack，yofc-gw-4 为 Parallels
set -uo pipefail
. "$(dirname "$0")/e2e-common.sh"
gw="${1:?usage: 04-gateway-install.sh <gateway-id>}"
files="$(gw_file_url "$gw")"
tty=""
[ "$(gw_kind "$gw")" = orb ] || tty=/dev/tty3       # Parallels：步骤摘要同步显示在 3 号文本终端

step "04 网关 ${gw}：准备虚拟机"
ensure_httpd
# deb 与网关侧脚本经 httpd 提供给虚拟机（整轮共用一份）
if [ ! -f "$SHARE_DIR/kura-core.deb" ] || [ "$CORE_DEB" -nt "$SHARE_DIR/kura-core.deb" ]; then
    cp -c "$CORE_DEB" "$SHARE_DIR/kura-core.deb" 2>/dev/null || cp "$CORE_DEB" "$SHARE_DIR/kura-core.deb"
    cp -c "$AGENT_DEB" "$SHARE_DIR/kura-management-agent.deb" 2>/dev/null || cp "$AGENT_DEB" "$SHARE_DIR/kura-management-agent.deb"
fi
cp "$SCRIPT_DIR"/vm/gw-*.py "$SHARE_DIR/"
core_sha=$(shasum -a 256 "$SHARE_DIR/kura-core.deb" | cut -d' ' -f1)
agent_sha=$(shasum -a 256 "$SHARE_DIR/kura-management-agent.deb" | cut -d' ' -f1)
note "kura-core                $(basename "$CORE_DEB")"
note "kura-management-agent    $(basename "$AGENT_DEB")"

if [ "$(gw_kind "$gw")" = orb ]; then
    if orb_exists "$gw"; then die "${gw} 已存在：先运行 01-wipe.sh"; fi
    # 创建时要向 OrbStack 的镜像源取一次索引，偶发网络 EOF：退避后重试，最多 3 次，每次失败都记在输出里。
    for attempt in 1 2 3; do
        if orbctl create --arch arm64 --memory 2G --cpus 2 debian:trixie "$gw" >/dev/null 2>"$LOG_DIR/create-$gw.attempt$attempt.log"; then break; fi
        orbctl delete "$gw" >/dev/null 2>&1 || true
        [ "$attempt" -lt 3 ] || { cat "$LOG_DIR/create-$gw.attempt$attempt.log"; die "创建 ${gw} 失败（已尝试 ${attempt} 次）"; }
        note "创建 ${gw} 第 ${attempt} 次失败（$(tr -d '\n' < "$LOG_DIR/create-$gw.attempt$attempt.log" | cut -c1-120)），$((attempt * 10)) 秒后重试"
        sleep $((attempt * 10))
    done
    for i in $(seq 1 90); do orb run -m "$gw" -u root true >/dev/null 2>&1 && break; sleep 2; done
    orb run -m "$gw" -u root true >/dev/null 2>&1 || die "${gw} 未就绪"
    note "OrbStack 虚拟机 ${gw} 已创建"
else
    # 必须在停机状态下还原快照再冷启动，内存里才不会残留上一轮的任何状态。
    [ "$(parallels_state)" = stopped ] || die "Parallels 虚拟机不是停机状态（先运行 01-wipe.sh）"
    prlctl snapshot-switch "$PARALLELS_VM" --id "$PARALLELS_SNAPSHOT" --skip-resume >/dev/null || die "快照还原失败"
    prlctl start "$PARALLELS_VM" >/dev/null || die "Parallels 虚拟机启动失败"
    for i in $(seq 1 60); do parallels_exec_retry cat /proc/sys/kernel/random/boot_id >/dev/null 2>&1 && break; sleep 3; done
    note "Parallels 虚拟机已还原到基线并启动"
    if ! gw_run "$gw" "baseline-$gw" "$SCRIPT_DIR/vm/vm-baseline.sh" >/dev/null; then cat "$LOG_DIR/baseline-$gw.log"; die "基线不是干净的"; fi
    grep -q BASELINE-CLEAN "$LOG_DIR/baseline-$gw.log" || die "基线核验未通过"
    note "基线核验：$(grep -E 'uptime|kura packages' "$LOG_DIR/baseline-$gw.log" | tr '\n' ';')"
fi

step "网关 ${gw}：安装并初始化"
if gw_run "$gw" "install-$gw" "$SCRIPT_DIR/vm/vm-install.sh" \
        FILES="$files" CORE_SHA="$core_sha" AGENT_SHA="$agent_sha" TTY="$tty"; then
    record OK "install-$gw" "$(grep -E '^kura-(core|management-agent)=' "$LOG_DIR/install-$gw.log" | tr '\n' ' ') $(grep -E '^(active|inactive|failed)' "$LOG_DIR/install-$gw.log" | tr '\n' ' ')"
    # 必须用 if：写成 [ … ] && { … } 时，OrbStack 网关（没有 tty）这一行以 1 结束，会变成脚本最终的退出码
    if [ "$tty" = /dev/tty3 ]; then vmshot "04-install-$gw-console" || true; fi
else
    record FAIL "install-$gw" "见 $LOG_DIR/install-$gw.log"
    exit 1
fi
