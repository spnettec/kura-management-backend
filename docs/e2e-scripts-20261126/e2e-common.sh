#!/bin/bash
# YOFC Kura E2E 部署：公共配置与辅助函数。
# 用法：在其余脚本里  . "$(dirname "$0")/e2e-common.sh"
#
# 拓扑：管理中心跑在 Docker（compose，18443/18444）；3 台 OrbStack 虚拟机 + 1 台 Parallels
# 虚拟机作为 IoT 网关，每台只装 kura-core 与 kura-management-agent 两个 deb。
# 两类虚拟机都通过宿主上的 httpd 取 deb、脚本和证书，网关内的步骤脚本把输出写到
# /tmp/e2e/<名称>.log，宿主读到结束哨兵后再判定成败。

E2E_ROOT="${E2E_ROOT:-$HOME/yofc-e2e}"                       # CA、证书、日志落盘目录
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# 必填：两个 arm64 deb（build-all.sh 与 build-agent.py 的产物）
: "${CORE_DEB:?set CORE_DEB to the arm64 kura-core deb}"
: "${AGENT_DEB:?set AGENT_DEB to the arm64 kura-management-agent deb}"
export CORE_DEB AGENT_DEB

CONSOLE_REPO="${CONSOLE_REPO:-$HOME/iot-kura-develop/git/kura-management-console}"
CONSOLE_NAME="${CONSOLE_NAME:-localhost}"                    # 浏览器访问中心用的名字
CONSOLE_ORIGIN="https://${CONSOLE_NAME}:18443"
ADMIN_USER="${ADMIN_USER:-admin}"
ADMIN_PASSWORD="${ADMIN_PASSWORD:-YofcE2e!2026}"             # E2E 测试口令
PG_PASSWORD="${PG_PASSWORD:-YofcE2ePg2026}"
export CONSOLE_ORIGIN ADMIN_USER ADMIN_PASSWORD PG_PASSWORD

HOST_IP_ORB="${HOST_IP_ORB:-198.18.0.1}"                     # OrbStack 虚拟机看到的宿主地址
HOST_IP_PARALLELS="${HOST_IP_PARALLELS:-10.211.55.2}"        # Parallels 虚拟机看到的宿主地址
ENDPOINT_ORB="https://${HOST_IP_ORB}:18444"                  # 网关连中心 gRPC 的地址
ENDPOINT_PARALLELS="https://${HOST_IP_PARALLELS}:18444"
PARALLELS_VM="${PARALLELS_VM:-YOFC Review ARM64 20261124}"
PARALLELS_SNAPSHOT="${PARALLELS_SNAPSHOT:-{ce60c087-5614-477b-92d1-8c003317d150}}"
HTTP_PORT="${HTTP_PORT:-18999}"                              # 宿主 httpd 端口
FILE_URL_ORB="http://host.orb.internal:${HTTP_PORT}"         # 虚拟机取文件的地址
FILE_URL_PARALLELS="http://${HOST_IP_PARALLELS}:${HTTP_PORT}"

GW_ORB=(yofc-gw-1 yofc-gw-2 yofc-gw-3)
GW_PARALLELS=yofc-gw-4
GW_ALL=("${GW_ORB[@]}" "$GW_PARALLELS")

PKI_DIR="$E2E_ROOT/pki"
LOG_DIR="$E2E_ROOT/logs"
SHARE_DIR="$E2E_ROOT/share"                                  # httpd 根目录
SHOTS="${SHOTS:-$E2E_ROOT/screenshots}"
CHROME="${CHROME:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"
PKG_IMAGE="${PKG_IMAGE:-yofc-agent-packager:latest}"         # 自带 OpenSSL 3.5 的打包镜像，用于签证书

for d in "$PKI_DIR" "$LOG_DIR" "$SHARE_DIR" "$SHOTS" "$E2E_ROOT/gw"; do mkdir -p "$d"; done

step() { printf '\n\033[1;32m==> %s\033[0m\n' "$*"; }
note() { printf '    %s\n' "$*"; }
die() { printf '\033[1;31mERROR: %s\033[0m\n' "$*" >&2; exit 1; }

# 记录一条步骤的成败：record OK|FAIL 名称 说明
record() { printf '%s\t%s\t%s\t%s\n' "$(date '+%H:%M:%S')" "$1" "$2" "$3" >> "$E2E_ROOT/steps.tsv"; printf '    [%s] %s  %s\n' "$1" "$2" "$3"; }
failures() { grep -c "^.*FAIL" "$E2E_ROOT/steps.tsv" 2>/dev/null || echo 0; }

gw_kind() { case "$1" in yofc-gw-[123]) echo orb;; *) echo parallels;; esac; }
gw_endpoint() { if [ "$(gw_kind "$1")" = orb ]; then echo "$ENDPOINT_ORB"; else echo "$ENDPOINT_PARALLELS"; fi; }
gw_file_url() { if [ "$(gw_kind "$1")" = orb ]; then echo "$FILE_URL_ORB"; else echo "$FILE_URL_PARALLELS"; fi; }

# ---------------------------------------------------------------- 宿主 httpd（虚拟机取文件的唯一通道）
ensure_httpd() {
    # 端口开着还不够：必须确认它服务的正是本轮的 $SHARE_DIR。
    echo "$E2E_ROOT" > "$SHARE_DIR/.e2e-root"
    local served
    served=$(curl -s -m 3 "http://127.0.0.1:${HTTP_PORT}/.e2e-root" 2>/dev/null)
    [ "$served" = "$E2E_ROOT" ] && return 0
    if curl -s -m 2 -o /dev/null "http://127.0.0.1:${HTTP_PORT}/"; then
        die "端口 ${HTTP_PORT} 被其它 httpd 占用（服务目录不是 $SHARE_DIR）。先运行 01-wipe.sh，或换一个 HTTP_PORT。"
    fi
    (cd "$SHARE_DIR" && nohup python3 -m http.server "$HTTP_PORT" --bind 0.0.0.0 > "$LOG_DIR/httpd.log" 2>&1 &)
    sleep 2
    served=$(curl -s -m 3 "http://127.0.0.1:${HTTP_PORT}/.e2e-root" 2>/dev/null)
    [ "$served" = "$E2E_ROOT" ] || die "httpd 没有在端口 ${HTTP_PORT} 上服务 $SHARE_DIR（见 $LOG_DIR/httpd.log）"
}

# ---------------------------------------------------------------- Parallels 访问
# prlctl exec 按空格重切参数、偶发 PrlJob_GetResult 失败：只传单个命令，读取类命令带重试。
parallels_exec_retry() { local out="" i; for i in 1 2 3 4 5 6 7 8; do out=$(prlctl exec "$PARALLELS_VM" "$@" 2>/dev/null) && [ -n "$out" ] && { printf '%s\n' "$out"; return 0; }; sleep 3; done; return 1; }
parallels_state() { prlctl list -a -o status,name --no-header 2>/dev/null | awk -v n="$PARALLELS_VM" '{state=$1; $1=""; sub(/^[ \t]+/, ""); sub(/[ \t]+$/, ""); if ($0 == n) print tolower(state)}'; }

# ---------------------------------------------------------------- OrbStack 虚拟机清单
# orbctl list 在刚删除虚拟机后可能瞬时失败：以它的退出码为准并重试，失败不等于"不存在"。
orb_list_names() {
    local i
    for i in 1 2 3 4 5 6; do
        if orbctl list > "$E2E_ROOT/.orb-list" 2>/dev/null; then awk '{print $1}' "$E2E_ROOT/.orb-list"; return 0; fi
        sleep 2
    done
    return 1
}
orb_exists() { orb_list_names | grep -qx "$1"; }

# ---------------------------------------------------------------- 在网关里执行步骤脚本
# gw_run <网关ID> <名称> <脚本文件> [VAR=VALUE ...]
# 生成 包装脚本（重定向输出到 /tmp/e2e/<名称>.log，VAR 成为脚本内变量，末尾追加退出码哨兵），
# 经 httpd 送入虚拟机执行，宿主把日志取回 $LOG_DIR/<名称>.log 并打印；返回脚本的退出码。
gw_run() {
    local gw="$1" name="$2" script="$3"; shift 3
    local body="$SHARE_DIR/$name.sh" log="/tmp/e2e/$name.log" rc=1 line
    {
        echo '#!/bin/bash'
        echo "exec > $log 2>&1"
        echo "echo \"E2E-BEGIN $name \$(date '+%H:%M:%S')\""
        printf '%s\n' "$@"
        # 步骤脚本会开 set -e：用 EXIT trap 保证无论怎样退出，日志最后一行都是退出码哨兵。
        echo "trap 'rc=\$?; echo \"E2E-EXIT \$rc\"' EXIT"
        cat "$script"
    } > "$body"
    ensure_httpd
    local url; url="$(gw_file_url "$gw")"
    if [ "$(gw_kind "$gw")" = orb ]; then
        orb run -m "$gw" -u root mkdir -p /tmp/e2e >/dev/null 2>&1
        orb run -m "$gw" -u root curl -s -m 60 -o "/tmp/e2e/$name.sh" "$url/$name.sh"
        orb run -m "$gw" -u root rm -f "$log"
        orb run -m "$gw" -u root bash "/tmp/e2e/$name.sh" >/dev/null 2>&1
        rc=$?
        orb run -m "$gw" -u root cat "$log" > "$LOG_DIR/$name.log" 2>&1 || true
    else
        prlctl exec "$PARALLELS_VM" mkdir -p /tmp/e2e >/dev/null 2>&1
        parallels_exec_retry curl -s -m 60 -o "/tmp/e2e/$name.sh" "$url/$name.sh" >/dev/null
        # 脚本没送到就不必等日志：直接判失败。
        [ "$(parallels_exec_retry stat -c %s "/tmp/e2e/$name.sh" 2>/dev/null || echo 0)" -gt 200 ] || { echo "script $name.sh was not delivered" | tee "$LOG_DIR/$name.log"; return 1; }
        prlctl exec "$PARALLELS_VM" rm -f "$log" >/dev/null 2>&1
        prlctl exec "$PARALLELS_VM" bash "/tmp/e2e/$name.sh" >/dev/null 2>&1 &
        line=""
        for i in $(seq 1 600); do
            line=$(prlctl exec "$PARALLELS_VM" tail -n 1 "$log" 2>/dev/null)
            case "$line" in E2E-EXIT*) break;; esac
            sleep 2
        done
        parallels_exec_retry cat "$log" > "$LOG_DIR/$name.log" 2>/dev/null || true
        case "$line" in E2E-EXIT\ 0) rc=0;; E2E-EXIT*) rc=1;; *) rc=1; echo "TIMEOUT waiting for $log" >> "$LOG_DIR/$name.log";; esac
    fi
    cat "$LOG_DIR/$name.log"
    return $rc
}

# ---------------------------------------------------------------- OpenSSL（在 Linux 容器里做，避免 macOS LibreSSL 的显式参数编码）
# openssl_box <脚本文件>：脚本在 /work 下执行，/work 即 $PKI_DIR
openssl_box() { docker run --rm --platform linux/arm64 -v "$PKI_DIR:/work" -w /work --entrypoint bash "$PKG_IMAGE" "$1"; }

# ---------------------------------------------------------------- 控制台 REST
api() { python3 -B "$SCRIPT_DIR/e2e-api.py" --root "$E2E_ROOT" "$@"; }

# ---------------------------------------------------------------- 截图
# shot <名称> <URL>：无头 Chrome 截控制台页面（不需要登录的页面）
shot() {
    local name="$1" url="$2"
    local out="$SHOTS/$name.png"      # 不能和上一行合并：local 的参数先整体展开，同一行里 $name 还不存在
    "$CHROME" --headless=new --disable-gpu --ignore-certificate-errors --hide-scrollbars \
        --screenshot="$out" --window-size=1280,800 --virtual-time-budget=9000 "$url" >/dev/null 2>&1
    if [ -s "$out" ]; then note "screenshot $out"; else die "截图失败：$url"; fi
}
# vmshot <名称>：截 Parallels 虚拟机当前屏幕
vmshot() { prlctl capture "$PARALLELS_VM" --file "$SHOTS/$1.png" >/dev/null 2>&1 && note "screenshot $SHOTS/$1.png" || note "VM screenshot FAILED"; }
