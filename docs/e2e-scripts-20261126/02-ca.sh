#!/bin/bash
# 02 生成 E2E 证书体系：client CA、server CA、中心服务端证书（在 OpenSSL 容器里生成）。
set -uo pipefail
. "$(dirname "$0")/e2e-common.sh"

step "02 生成证书体系"
san="DNS:localhost,DNS:host.orb.internal,IP:127.0.0.1,IP:${HOST_IP_ORB},IP:${HOST_IP_PARALLELS}"
case "$CONSOLE_NAME" in
    localhost) ;;
    *) san="${san},DNS:${CONSOLE_NAME}" ;;     # 浏览器用别的名字访问中心时，该名字必须在 SAN 里
esac
[ -n "${CONSOLE_EXTRA_SAN:-}" ] && san="${san},${CONSOLE_EXTRA_SAN}"
sed "s|@@SAN@@|$san|" "$SCRIPT_DIR/pki/pki-gen.sh" > "$PKI_DIR/gen.sh"
openssl_box gen.sh || die "证书生成失败"
for f in client-ca.pem server-ca.pem server.pem; do
    [ -s "$PKI_DIR/$f" ] || die "缺少 $PKI_DIR/$f"
    note "$(openssl x509 -in "$PKI_DIR/$f" -noout -subject -enddate | tr '\n' ' ')"
done
record OK ca "证书体系已生成"
