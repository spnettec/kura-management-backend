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

FROM rust:1.95-bookworm AS rust
WORKDIR /build
ENV CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY crates ./crates
COPY migrations ./migrations
RUN cargo build --release --workspace --locked --message-format=json

FROM ubuntu:24.04
RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates curl openssl util-linux \
 && rm -rf /var/lib/apt/lists/*
RUN groupadd -r console && useradd -r -g console -d /var/lib/kura-management-console -s /usr/sbin/nologin console
COPY --from=rust /build/target/release/yofc-management-console /usr/local/bin/yofc-management-console
COPY --from=web /build/frontend/react/dist /usr/share/kura-management-console/react
COPY docker/entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod 0755 /usr/local/bin/entrypoint.sh /usr/local/bin/yofc-management-console \
 && mkdir -p /var/lib/kura-management-console \
 && chown console:console /var/lib/kura-management-console \
 && chmod 0700 /var/lib/kura-management-console
ENV CONSOLE_SETUP_PORT=18443 \
    CONSOLE_WEB_DIR=/usr/share/kura-management-console/react \
    CONSOLE_STATE_DIR=/var/lib/kura-management-console
LABEL com.yofc.console.installation="1"
RUN binary_sha=$(sha256sum /usr/local/bin/yofc-management-console | cut -d' ' -f1) \
 && printf '{"schemaVersion":1,"binarySha256":"%s","maintenanceProtocolVersion":2,"fencedStartupCommand":"serve-maintenance","maintenanceCommands":["maintenance-status","maintenance-begin","maintenance-complete"],"localReadCommands":["installation-credential","installation-maintenance-check","installation-status"]}\n' "$binary_sha" > /usr/share/kura-management-console/local-command-provenance.json \
 && chmod 0644 /usr/share/kura-management-console/local-command-provenance.json
VOLUME ["/var/lib/kura-management-console"]
EXPOSE 18443 18444
STOPSIGNAL SIGINT
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
HEALTHCHECK --interval=10s --timeout=5s --start-period=20s --retries=5 \
  CMD curl -sk -f https://127.0.0.1:18443/health/live || exit 1
