# Multi-stage build for the YOFC management console (Rust + React).
# Result: a self-contained ubuntu:24.04 runtime image; PostgreSQL runs as a
# separate service (see compose.yml).
FROM node:24-alpine AS web
WORKDIR /build
COPY frontend/react/package.json frontend/react/package-lock.json ./frontend/react/
COPY docs/openapi-v1.json ./docs/openapi-v1.json
COPY frontend/react/index.html frontend/react/tsconfig.json frontend/react/vite.config.ts ./frontend/react/
COPY frontend/react/src ./frontend/react/src
RUN cd frontend/react && npm ci --no-audit --no-fund && npm run build

FROM rust:1.95 AS rust
WORKDIR /build
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY crates ./crates
COPY migrations ./migrations
RUN cargo build --release --workspace --locked

FROM ubuntu:24.04
RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates curl openssl \
 && rm -rf /var/lib/apt/lists/*
RUN groupadd -r console && useradd -r -g console -d /var/lib/kura-management-console -s /usr/sbin/nologin console
COPY --from=rust /build/target/release/yofc-management-console /usr/local/bin/yofc-management-console
COPY --from=web /build/frontend/react/dist /usr/share/kura-management-console/react
COPY docker/entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod 0755 /usr/local/bin/entrypoint.sh /usr/local/bin/yofc-management-console \
 && mkdir -p /var/lib/kura-management-console /etc/kura-management-console/tls \
 && chown -R console:console /var/lib/kura-management-console /etc/kura-management-console \
 && chmod 0750 /etc/kura-management-console
ENV CONSOLE_HTTP_ADDR=0.0.0.0:18443 \
    CONSOLE_GRPC_ADDR=0.0.0.0:18444 \
    CONSOLE_WEB_DIR=/usr/share/kura-management-console/react \
    CONSOLE_STATE_DIR=/var/lib/kura-management-console \
    CONSOLE_DATA_KEY_FILE=/var/lib/kura-management-console/task-data.key \
    CONSOLE_TLS_CERT=/etc/kura-management-console/tls/server.pem \
    CONSOLE_TLS_KEY=/etc/kura-management-console/tls/server.key \
    CONSOLE_CLIENT_CA=/etc/kura-management-console/tls/ca.pem
VOLUME ["/var/lib/kura-management-console", "/etc/kura-management-console/tls"]
EXPOSE 18443 18444
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
HEALTHCHECK --interval=10s --timeout=5s --start-period=20s --retries=5 \
  CMD curl -sk -f https://127.0.0.1:18443/health/ready || exit 1
