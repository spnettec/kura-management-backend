#!/bin/sh
# One-shot demo PKI: self-signed CA, server certificate for localhost, and one
# client certificate per gateway name passed as arguments (default gw-001..003).
# Replace with a real CA for production; the console only needs server.pem,
# server.key and ca.pem.
set -e
umask 077
cd "$(dirname "$0")"
mkdir -p tls
[ ! -e tls/server.key ] && [ ! -e tls/server.pem ] || {
  echo "existing server identity retained; choose a new fixture directory" >&2; exit 1;
}
for name in ${*:-gw-001 gw-002 gw-003}; do
  case "$name" in ''|*[!a-zA-Z0-9_-]*) echo "invalid gateway name" >&2; exit 1;; esac
  [ ! -e "tls/$name.key" ] && [ ! -e "tls/$name.pem" ] || {
    echo "existing client identity retained: $name" >&2; exit 1;
  }
done
[ -f tls/ca.pem ] || openssl req -x509 -sha256 -newkey ec -pkeyopt ec_paramgen_curve:P-256 -pkeyopt ec_param_enc:named_curve \
  -nodes -days 825 -subj "/CN=yofc-console-demo-ca" -addext "basicConstraints=critical,CA:TRUE" \
  -addext "keyUsage=critical,keyCertSign,cRLSign" -addext "subjectKeyIdentifier=hash" \
  -keyout tls/ca.key -out tls/ca.pem 2>/dev/null
openssl req -sha256 -newkey ec -pkeyopt ec_paramgen_curve:P-256 -pkeyopt ec_param_enc:named_curve -nodes \
  -subj "/CN=localhost" -addext "subjectAltName=DNS:localhost,IP:127.0.0.1,DNS:host.orb.internal" \
  -keyout tls/server.key -out tls/server.csr 2>/dev/null
printf 'basicConstraints=critical,CA:FALSE\nkeyUsage=critical,digitalSignature\nextendedKeyUsage=serverAuth\nsubjectAltName=DNS:localhost,IP:127.0.0.1,DNS:host.orb.internal\nsubjectKeyIdentifier=hash\nauthorityKeyIdentifier=keyid,issuer\n' > tls/server.ext
openssl x509 -req -sha256 -in tls/server.csr -CA tls/ca.pem -CAkey tls/ca.key \
  -CAcreateserial -days 825 -extfile tls/server.ext -out tls/server.pem 2>/dev/null
chmod 600 tls/ca.key tls/server.key
printf 'basicConstraints=critical,CA:FALSE\nkeyUsage=critical,digitalSignature\nextendedKeyUsage=clientAuth\nsubjectKeyIdentifier=hash\nauthorityKeyIdentifier=keyid,issuer\n' > tls/client.ext
for name in ${*:-gw-001 gw-002 gw-003}; do
  openssl req -sha256 -newkey ec -pkeyopt ec_paramgen_curve:P-256 -pkeyopt ec_param_enc:named_curve -nodes \
    -subj "/CN=$name" -keyout "tls/$name.key" -out "tls/$name.csr" 2>/dev/null
  openssl x509 -req -sha256 -in "tls/$name.csr" -CA tls/ca.pem -CAkey tls/ca.key \
    -CAcreateserial -days 825 \
    -extfile tls/client.ext \
    -out "tls/$name.pem" 2>/dev/null
  chmod 600 "tls/$name.key"
  echo "client identity: tls/$name.pem tls/$name.key (CA: tls/ca.pem)"
done
echo "server: tls/server.pem tls/server.key (CA: tls/ca.pem)"
