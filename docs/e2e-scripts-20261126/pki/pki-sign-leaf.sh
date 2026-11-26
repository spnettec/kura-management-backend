#!/bin/bash
# 在 OpenSSL 容器内执行（/work 即 E2E_ROOT/pki）：用 client CA 给网关提交的 CSR 签发 leaf 证书。
# 宿主把 @@ID@@ 替换成网关标识后调用；输入 gw-<标识>.csr，输出 gw-<标识>-leaf.pem。
# 网关证书只能是客户端用途的叶子证书（CA:FALSE），中心拒绝把 CA 证书登记为网关身份。
set -euo pipefail
export OPENSSL_CONF=/dev/null
cat > "gw-@@ID@@.ext" <<EOT
basicConstraints=critical,CA:FALSE
keyUsage=critical,digitalSignature
extendedKeyUsage=clientAuth
subjectKeyIdentifier=hash
authorityKeyIdentifier=keyid,issuer
EOT
openssl x509 -req -in "gw-@@ID@@.csr" -CA client-ca.pem -CAkey client-ca.key -CAcreateserial \
    -out "gw-@@ID@@-leaf.pem" -days 3650 -extfile "gw-@@ID@@.ext" -sha256
openssl x509 -in "gw-@@ID@@-leaf.pem" -noout -subject -fingerprint -sha256
