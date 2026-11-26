#!/bin/bash
# 在 OpenSSL 容器内执行（/work 即 E2E_ROOT/pki）：生成 E2E 证书体系。
#   client-ca  签发各网关的 leaf 证书（网关和中心用它做双向 TLS 的客户端 CA）
#   server-ca  签发中心服务端证书（网关用它锚定中心）
#   server.*   中心服务端证书，SAN 必须覆盖浏览器和两类虚拟机访问中心的全部地址
# 宿主把 @@SAN@@ 替换成实际的 subjectAltName 列表后调用。
set -euo pipefail
export OPENSSL_CONF=/dev/null

openssl genpkey -algorithm EC -pkeyopt ec_paramgen_curve:P-256 -out client-ca.key 2>/dev/null
openssl req -x509 -new -key client-ca.key -out client-ca.pem -days 3650 -subj "/CN=yofc-e2e-client-ca" \
    -addext "basicConstraints=critical,CA:TRUE,pathlen:0" \
    -addext "keyUsage=critical,keyCertSign,cRLSign" -addext "subjectKeyIdentifier=hash"

openssl genpkey -algorithm EC -pkeyopt ec_paramgen_curve:P-256 -out server-ca.key 2>/dev/null
openssl req -x509 -new -key server-ca.key -out server-ca.pem -days 3650 -subj "/CN=yofc-e2e-server-issuer" \
    -addext "basicConstraints=critical,CA:TRUE,pathlen:0" \
    -addext "keyUsage=critical,keyCertSign,cRLSign" -addext "subjectKeyIdentifier=hash"

openssl genpkey -algorithm EC -pkeyopt ec_paramgen_curve:P-256 -out server.key 2>/dev/null
cat > server.ext <<EOT
basicConstraints=critical,CA:FALSE
keyUsage=critical,digitalSignature,keyEncipherment
extendedKeyUsage=serverAuth
subjectKeyIdentifier=hash
authorityKeyIdentifier=keyid,issuer
subjectAltName=@@SAN@@
EOT
openssl req -new -key server.key -subj "/CN=yofc-e2e-console" -out server.csr 2>/dev/null
openssl x509 -req -in server.csr -CA server-ca.pem -CAkey server-ca.key -CAcreateserial \
    -out server.pem -days 3650 -extfile server.ext -sha256 2>/dev/null

chmod 644 ./*.pem
chmod 600 ./*.key
openssl x509 -in server.pem -noout -subject -ext subjectAltName
