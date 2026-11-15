-- Public operation history only. No certificate PEM, private key or bundle token.
CREATE TABLE mgmt_gateway_certificate_rotation_ids (
    operation_id uuid PRIMARY KEY,
    gateway_id text NOT NULL REFERENCES mgmt_gateways(id),
    tenant_id text NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX mgmt_gateway_certificate_rotation_ids_by_gateway ON mgmt_gateway_certificate_rotation_ids(gateway_id,tenant_id);
CREATE TABLE mgmt_gateway_certificate_rotations (
    operation_id uuid PRIMARY KEY REFERENCES mgmt_gateway_certificate_rotation_ids(operation_id),
    gateway_id text NOT NULL REFERENCES mgmt_gateways(id),
    tenant_id text NOT NULL,
    previous_certificate_sha256 text NOT NULL CHECK(previous_certificate_sha256 ~ '^[0-9a-f]{64}$'),
    certificate_sha256 text CHECK(certificate_sha256 ~ '^[0-9a-f]{64}$'),
    certificate_pem_sha256 text NOT NULL CHECK(certificate_pem_sha256 ~ '^[0-9a-f]{64}$'),
    bundle_requested boolean NOT NULL,
    state text NOT NULL CHECK(state IN ('PENDING','REJECTED','APPLIED')),
    rejection_code text,
    accepted_revision bigint,
    actor text NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    accepted_at timestamptz,
    CHECK ((state='APPLIED' AND certificate_sha256 IS NOT NULL AND accepted_revision IS NOT NULL AND accepted_revision>=1 AND accepted_at IS NOT NULL AND rejection_code IS NULL)
        OR (state='REJECTED' AND rejection_code IS NOT NULL AND accepted_revision IS NULL AND accepted_at IS NULL)
        OR (state='PENDING' AND rejection_code IS NULL AND accepted_revision IS NULL AND accepted_at IS NULL))
);
CREATE UNIQUE INDEX mgmt_gateway_certificate_one_pending ON mgmt_gateway_certificate_rotations(gateway_id) WHERE state='PENDING';
ALTER TABLE mgmt_gateways ADD COLUMN certificate_rotation_operation_id uuid;
ALTER TABLE mgmt_gateways ADD COLUMN certificate_rotation_pending_id uuid;
