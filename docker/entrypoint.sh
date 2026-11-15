#!/bin/sh
# Root entrypoint: prepare state, then drop privileges for the console itself.
#   - serve (default): prepare private bootstrap TLS, init
#     the task key if missing, migrate (+ seed bootstrap admin/admin when no
#     admin exists), then exec the server as `console`.
#   - anything else: forward to the binary as `console` (migrate, create-user, ...).
set -e
STATE=/var/lib/kura-management-console
BINARY=/usr/local/bin/yofc-management-console
CONSOLE_USER=console
run() {
  setpriv --reuid "$CONSOLE_USER" --regid "$CONSOLE_USER" --clear-groups \
    env DATABASE_URL="$DATABASE_URL" \
        CONSOLE_DATA_KEY_FILE="$CONSOLE_DATA_KEY_FILE" \
        CONSOLE_ORIGIN="$CONSOLE_ORIGIN" \
        CONSOLE_HTTP_ADDR="$CONSOLE_HTTP_ADDR" \
        CONSOLE_GRPC_ADDR="$CONSOLE_GRPC_ADDR" \
        CONSOLE_TLS_CERT="$CONSOLE_TLS_CERT" \
        CONSOLE_TLS_KEY="$CONSOLE_TLS_KEY" \
        CONSOLE_CLIENT_CA="$CONSOLE_CLIENT_CA" \
        CONSOLE_STATE_DIR="$STATE" \
        CONSOLE_WEB_DIR="$CONSOLE_WEB_DIR" \
        "$BINARY" "$@"
}
prepare_tls() {
  # This root-owned runtime parent prevents the console user from redirecting
  # privileged copies. Every destination is new, private and unpredictable.
  install -d -o root -g root -m 0755 /run/kura-management-console
  BOOTSTRAP_TLS=$(mktemp -d /run/kura-management-console/tls.XXXXXX)
  cp -- "$CONSOLE_TLS_CERT" "$BOOTSTRAP_TLS/server.pem"
  cp -- "$CONSOLE_TLS_KEY" "$BOOTSTRAP_TLS/server.key"
  cp -- "$CONSOLE_CLIENT_CA" "$BOOTSTRAP_TLS/client-ca.pem"
  chmod 0600 "$BOOTSTRAP_TLS/server.pem" "$BOOTSTRAP_TLS/server.key" "$BOOTSTRAP_TLS/client-ca.pem"
  chown -R "$CONSOLE_USER:$CONSOLE_USER" "$BOOTSTRAP_TLS"
  CONSOLE_TLS_CERT="$BOOTSTRAP_TLS/server.pem"
  CONSOLE_TLS_KEY="$BOOTSTRAP_TLS/server.key"
  CONSOLE_CLIENT_CA="$BOOTSTRAP_TLS/client-ca.pem"
  # Pending TLS/CA generations and DB-bound keys are resolved by the server
  # after dropping privileges. Root never copies console-writable pending data.
}
case "${1:-serve}" in
  serve)
    mkdir -p "$STATE"
    chown "$CONSOLE_USER:$CONSOLE_USER" "$STATE"
    chmod 0700 "$STATE"
    if [ ! -f "$CONSOLE_DATA_KEY_FILE" ]; then
      run task-key-init "$CONSOLE_DATA_KEY_FILE"
    fi
    run migrate
    prepare_tls
    exec setpriv --reuid "$CONSOLE_USER" --regid "$CONSOLE_USER" --clear-groups \
      env DATABASE_URL="$DATABASE_URL" \
          CONSOLE_DATA_KEY_FILE="$CONSOLE_DATA_KEY_FILE" \
          CONSOLE_ORIGIN="$CONSOLE_ORIGIN" \
          CONSOLE_HTTP_ADDR="$CONSOLE_HTTP_ADDR" \
          CONSOLE_GRPC_ADDR="$CONSOLE_GRPC_ADDR" \
          CONSOLE_TLS_CERT="$CONSOLE_TLS_CERT" \
          CONSOLE_TLS_KEY="$CONSOLE_TLS_KEY" \
          CONSOLE_CLIENT_CA="$CONSOLE_CLIENT_CA" \
          CONSOLE_STATE_DIR="$STATE" \
          CONSOLE_WEB_DIR="$CONSOLE_WEB_DIR" \
          "$BINARY" serve
    ;;
  *)
    run "$@"
    ;;
esac
