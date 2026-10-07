CREATE TABLE console_users (
    user_id BIGSERIAL PRIMARY KEY,
    username VARCHAR(64) NOT NULL UNIQUE,
    password_hash VARCHAR(100) NOT NULL,
    tenant_id VARCHAR(64) NOT NULL,
    display_name VARCHAR(128),
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_console_users_tenant ON console_users (tenant_id);
