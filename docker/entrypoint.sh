#!/bin/sh
# Root entrypoint: prepare state, then drop privileges for the console itself.
#   - serve (default): start the HTTPS setup wizard as `console`.
#     The server owns configuration, key creation and database initialization.
#   - anything else: forward to the binary as `console` (migrate, create-user, ...).
set -e
STATE=/var/lib/kura-management-console
BINARY=/usr/local/bin/yofc-management-console
CONSOLE_USER=console
run() {
  exec setpriv --reuid "$CONSOLE_USER" --regid "$CONSOLE_USER" --clear-groups \
    env CONSOLE_STATE_DIR="$STATE" \
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
  export CONSOLE_TLS_CERT CONSOLE_TLS_KEY CONSOLE_CLIENT_CA
  # Pending TLS/CA generations and DB-bound keys are resolved by the server
  # after dropping privileges. Root never copies console-writable pending data.
}
case "${1:-serve}" in
  serve)
    [ "$#" -le 1 ] || { echo 'Unexpected serve arguments' >&2; exit 2; }
    [ ! -L "$STATE" ] || { echo 'State directory must not be a symlink' >&2; exit 1; }
    mkdir -p "$STATE"
    OWNER=$(stat -c '%u' "$STATE")
    EXPECTED=$(id -u "$CONSOLE_USER")
    [ "$OWNER" = "$EXPECTED" ] || [ "$OWNER" = 0 ] || { echo 'Unexpected state directory owner' >&2; exit 1; }
    # A new named volume is root-owned. Never take over retained files or
    # recursively rewrite ownership in a prior installation.
    if [ "$OWNER" = 0 ] && [ -n "$(find "$STATE" -mindepth 1 -maxdepth 1 -print -quit)" ]; then
      echo 'Nonempty root-owned state requires explicit recovery' >&2; exit 1
    fi
    chown "$CONSOLE_USER:$CONSOLE_USER" "$STATE"
    chmod 0700 "$STATE"
    if [ -n "${CONSOLE_TLS_CERT:-}" ] && [ -n "${CONSOLE_TLS_KEY:-}" ] && [ -n "${CONSOLE_CLIENT_CA:-}" ]; then
      prepare_tls
    fi
    exec setpriv --reuid "$CONSOLE_USER" --regid "$CONSOLE_USER" --clear-groups \
      env CONSOLE_STATE_DIR="$STATE" \
          "$BINARY" serve
    ;;
  *)
    run "$@"
    ;;
esac
