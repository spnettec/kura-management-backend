# Kura Management Backend

Spring Boot service for the lightweight Kura fleet management MVP.

## Development

Run with the in-memory H2 profile:

```bash
mvn spring-boot:run -Dspring-boot.run.profiles=dev
```

Default HTTP port:

```text
8081
```

The `dev` profile seeds one demo gateway:

```text
gw-001
```

Production defaults use PostgreSQL and Flyway.

For local MQTT testing, run Mosquitto on port `1883`:

```bash
docker compose -f docker/docker-compose.yml up -d
```

The development broker uses username/password authentication and ACLs. The
backend connects as `backend`; the demo gateway connects as `gw-001`.

Gateway agents publish status reports to:

```text
mgmt/{tenantId}/{gatewayId}/status
```

Commands are published to:

```text
mgmt/{tenantId}/{gatewayId}/cmd
```

Replies are consumed from:

```text
mgmt/{tenantId}/{gatewayId}/reply
```

Gateway records are marked offline when no heartbeat is received for
`management.gateway-offline-timeout-sec`, which defaults to `90`.
