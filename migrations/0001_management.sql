-- Separate namespace from the Java/Flyway baseline. No destructive automatic migration.
CREATE TABLE mgmt_users (
    id uuid PRIMARY KEY, tenant_id text NOT NULL, username text UNIQUE NOT NULL,
    password_hash text NOT NULL, role text NOT NULL CHECK(role IN ('viewer','operator','admin','recovery')),
    enabled boolean NOT NULL DEFAULT true
);
CREATE TABLE mgmt_sessions (
    token_hash text PRIMARY KEY, user_id uuid NOT NULL REFERENCES mgmt_users(id),
    csrf text NOT NULL, expires_at timestamptz NOT NULL
);
CREATE INDEX ON mgmt_sessions(expires_at);
CREATE TABLE mgmt_gateways (
    id text PRIMARY KEY, tenant_id text NOT NULL, name text NOT NULL,
    certificate_sha256 text UNIQUE NOT NULL, enabled boolean NOT NULL DEFAULT true,
    connection_epoch uuid, agent_version text, boot_id text,
    capabilities jsonb NOT NULL DEFAULT '[]', last_seen timestamptz,
    kura_available boolean NOT NULL DEFAULT false, uptime_seconds bigint NOT NULL DEFAULT 0,
    sequence bigint NOT NULL DEFAULT 0, revision bigint NOT NULL DEFAULT 0,
    online_reported boolean NOT NULL DEFAULT false
);
CREATE TABLE mgmt_tasks (
    id uuid PRIMARY KEY, gateway_id text NOT NULL REFERENCES mgmt_gateways(id),
    tenant_id text NOT NULL, actor_id uuid NOT NULL REFERENCES mgmt_users(id),
    operation integer NOT NULL CHECK(operation BETWEEN 1 AND 3),
    idempotency_key text NOT NULL, state text NOT NULL DEFAULT 'QUEUED',
    error text, result jsonb, created_at timestamptz NOT NULL DEFAULT now(),
    expires_at timestamptz NOT NULL, lease_until timestamptz, dispatch_epoch uuid,
    UNIQUE(actor_id, gateway_id, idempotency_key)
);
CREATE INDEX ON mgmt_tasks(gateway_id, expires_at);
CREATE TABLE mgmt_events (
    id bigserial PRIMARY KEY, tenant_id text NOT NULL, kind text NOT NULL,
    target text NOT NULL, revision bigint NOT NULL, data jsonb NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX ON mgmt_events(tenant_id, id);
CREATE TABLE mgmt_audit (
    id bigserial PRIMARY KEY, tenant_id text NOT NULL, actor text NOT NULL,
    operation text NOT NULL, target text NOT NULL, outcome text NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now()
);
