#!/bin/bash
# 05 把一台已安装的网关登记到管理中心：
#    keygen（私钥不出网关）→ 中心侧用 client CA 签发 leaf → 登记网关 → stage → enroll（bind + 连通）
#    用法：05-gateway-enroll.sh <网关ID>
set -uo pipefail
. "$(dirname "$0")/e2e-common.sh"
gw="${1:?usage: 05-gateway-enroll.sh <gateway-id>}"
files="$(gw_file_url "$gw")"
gw_files="$files/gw/$gw"
endpoint="$(gw_endpoint "$gw")"
tty=""
[ "$(gw_kind "$gw")" = orb ] || tty=/dev/tty3
work="$E2E_ROOT/gw/$gw"
install -d -m 0700 "$work" "$SHARE_DIR/gw/$gw"
stop() { record FAIL "enroll-$gw" "$1"; exit 1; }

step "05 网关 ${gw}：登记到管理中心"

gw_run "$gw" "keygen-$gw" "$SCRIPT_DIR/vm/vm-keygen.sh" GW_ID="$gw" FILES="$files" TTY="$tty" >/dev/null \
    || stop "keygen 失败，见 $LOG_DIR/keygen-$gw.log"
awk '/-----CSR-BEGIN-----/{on=1; next} /-----CSR-END-----/{on=0} on' "$LOG_DIR/keygen-$gw.log" | tr -d '\r' > "$work/$gw.csr"
grep -q "BEGIN CERTIFICATE REQUEST" "$work/$gw.csr" || stop "没有取到 CSR"
note "CSR $(openssl req -in "$work/$gw.csr" -noout -subject 2>/dev/null)"

cp "$work/$gw.csr" "$PKI_DIR/gw-$gw.csr"
sed "s|@@ID@@|$gw|g" "$SCRIPT_DIR/pki/pki-sign-leaf.sh" > "$PKI_DIR/sign-$gw.sh"
openssl_box "sign-$gw.sh" > "$work/sign.log" 2>&1 || stop "签发失败，见 $work/sign.log"
cp "$PKI_DIR/gw-$gw-leaf.pem" "$work/leaf.pem"
note "leaf $(grep '^sha256' "$work/sign.log" | cut -c1-80)"

# 登记是必做步骤：未登记的网关即使 bind 成功，中心也会拒绝它的连接（FORBIDDEN）。
api register "$gw" "YOFC Gateway ${gw##*-}" "$work/leaf.pem" || stop "登记失败"

cp "$work/leaf.pem" "$SHARE_DIR/gw/$gw/leaf.pem"
cp "$PKI_DIR/client-ca.pem" "$SHARE_DIR/gw/$gw/issuer-ca.pem"
cp "$PKI_DIR/server-ca.pem" "$SHARE_DIR/gw/$gw/console-ca.pem"
rid=$(uuidgen | tr 'A-Z' 'a-z')            # agent 要求规范写法的小写 UUID
echo "$rid" > "$work/request-id"

gw_run "$gw" "stage-$gw" "$SCRIPT_DIR/vm/vm-stage.sh" \
        GW_ID="$gw" RID="$rid" ENDPOINT="$endpoint" FILES="$files" GW_FILES="$gw_files" TTY="$tty" >/dev/null \
    || stop "stage 失败，见 $LOG_DIR/stage-$gw.log"
sha=$(sed -n 's/^STAGE-SHA //p' "$LOG_DIR/stage-$gw.log" | tr -d '\r')
[ -n "$sha" ] || stop "没有取到 stage SHA"
echo "$sha" > "$work/stage-sha"
note "stage request $rid  sha ${sha:0:16}…"

gw_run "$gw" "enroll-$gw" "$SCRIPT_DIR/vm/vm-enroll.sh" RID="$rid" SHA="$sha" FILES="$files" TTY="$tty" \
    || stop "enroll 失败，见 $LOG_DIR/enroll-$gw.log"
record OK "enroll-$gw" "$(grep '^FINAL:' "$LOG_DIR/enroll-$gw.log" | tr -d '\r') $(grep '^receipt actor' "$LOG_DIR/enroll-$gw.log" | tr -d '\r')"
# 必须用 if：写成 [ … ] && { … } 时，OrbStack 网关（没有 tty）这一行以 1 结束，会变成脚本最终的退出码
if [ "$tty" = /dev/tty3 ]; then sleep 2; vmshot "05-enroll-$gw-console" || true; fi
