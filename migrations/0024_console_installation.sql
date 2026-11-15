-- New UI installations bind their private generation to exactly one database.
-- Existing legacy installations remain unbound until explicitly migrated.
CREATE TABLE mgmt_console_installation (
    singleton boolean PRIMARY KEY CHECK (singleton),
    installation_id uuid NOT NULL UNIQUE,
    setup_operation_id uuid NOT NULL UNIQUE,
    config_digest text NOT NULL CHECK (config_digest ~ '^[0-9a-f]{64}$'),
    created_at timestamptz NOT NULL DEFAULT now()
);
