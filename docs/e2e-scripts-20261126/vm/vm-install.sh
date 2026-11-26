#!/bin/bash
# 在网关内以 root 执行：安装依赖与两个 deb，初始化 agent，完成两项本地授权，启动两个服务。
# 由宿主的 04-gateway-install.sh 经 gw_run 送入；变量由包装脚本提供：
#   FILES      宿主 httpd 地址      CORE_SHA / AGENT_SHA  两个 deb 的 SHA-256
#   TTY        可选，把步骤摘要同步显示到该终端（用于虚拟机控制台截图）
set -euo pipefail
export DEBIAN_FRONTEND=noninteractive
say() { printf '%s  %s\n' "$(date '+%H:%M:%S')" "$*" | { if [ -n "${TTY:-}" ]; then tee -a "$TTY"; else cat; fi; } || true; }
tty_setup() {
    [ -n "${TTY:-}" ] || return 0
    local unit="getty@${TTY#/dev/}" i              # chvt 要终端号（tty3 → 3），systemd 的单元名要 tty3
    chvt "${TTY#/dev/tty}" || true
    # 切到一个还没有 getty 的终端时，systemd 会自动在上面起一个（autovt）：它启动时会对终端做
    # vhangup，登录横幅也会盖掉此刻写上去的内容。等它起来、横幅打完，再清屏。
    for i in $(seq 1 50); do systemctl is-active -q "$unit" 2>/dev/null && break; sleep 0.2; done
    sleep 1
    setterm -term linux -blank 0 < "$TTY" > "$TTY" 2>/dev/null || true
    printf '\033[2J\033[H' > "$TTY" || true
}
SETUP=/usr/lib/kura-management-agent/runtime_setup.py
uuid() { cat /proc/sys/kernel/random/uuid; }

tty_setup
say "== YOFC Kura gateway: install =="

say "[1/5] apt dependencies"
apt-get update -qq
apt-get install -y -qq python3 libssl3 gpsd openjdk-21-jre-headless dnsmasq-base isc-dhcp-client bind9-utils
say "      java $(java -version 2>&1 | head -1 | cut -d'"' -f2)"

say "[2/5] download debs, verify sha256"
install -d -m 0700 /tmp/e2e/debs /tmp/enroll
curl -fsS -m 900 -o /tmp/e2e/debs/kura-management-agent.deb "$FILES/kura-management-agent.deb"
curl -fsS -m 900 -o /tmp/e2e/debs/kura-core.deb "$FILES/kura-core.deb"
echo "$AGENT_SHA  /tmp/e2e/debs/kura-management-agent.deb" | sha256sum -c
echo "$CORE_SHA  /tmp/e2e/debs/kura-core.deb" | sha256sum -c

say "[3/5] install kura-management-agent and kura-core"
# 两个 deb 都交给 apt 安装：apt 先把 deb 声明的依赖配齐，再配置本包，日志里不会出现
# “dpkg -i 先报依赖错误、再用 apt-get install -f 收敛”的那条报错。agent 先于 core。
apt-get install -y -qq /tmp/e2e/debs/kura-management-agent.deb
apt-get install -y -qq /tmp/e2e/debs/kura-core.deb
dpkg-query -W -f='${Package} ${db:Status-Status} ${Version}\n' kura-core kura-management-agent
[ "$(dpkg-query -W -f='${db:Status-Status}' kura-core)" = installed ]
[ "$(dpkg-query -W -f='${db:Status-Status}' kura-management-agent)" = installed ]
say "      $(dpkg-query -W -f='${Package}=${Version}' kura-core)  $(dpkg-query -W -f='${Package}=${Version}' kura-management-agent)"

say "[4/5] initialize, stop units, authorize-kura-peer, authorize-enrollment-reader"
# 两个服务装完不会自启；初始化与授权期间必须处于停止状态。
systemctl stop kura.service kura-management-agent-v2.service 2>/dev/null || true
python3 "$SETUP" initialize --request-id "$(uuid)"
systemctl stop kura.service kura-management-agent-v2.service 2>/dev/null || true
python3 "$SETUP" authorize-kura-peer --request-id "$(uuid)"
python3 "$SETUP" authorize-enrollment-reader --request-id "$(uuid)"

say "[5/5] start kura.service and kura-management-agent-v2.service"
systemctl start kura-management-agent-v2.service kura.service
sleep 3
systemctl is-active kura.service kura-management-agent-v2.service
say "== install done =="
