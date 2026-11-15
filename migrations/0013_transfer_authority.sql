-- Cross-center ownership transfer is authorized by a separately recorded signer and
-- a separate approver. Ordinary registration and gateway-local recovery do not use this path.
CREATE TABLE mgmt_transfer_signers (
    key_id text PRIMARY KEY,
    tenant_id text NOT NULL,
    public_key_sha256 text NOT NULL UNIQUE,
    status text NOT NULL DEFAULT 'ACTIVE'
        CHECK (status IN ('ACTIVE','REVOKED')),
    created_at timestamptz NOT NULL DEFAULT now(),
    revoked_at timestamptz
);
CREATE INDEX mgmt_transfer_signers_tenant_status
    ON mgmt_transfer_signers(tenant_id,status);

-- A request records the exact token claims. Only an approval by another admin allows issuance.
CREATE TABLE mgmt_transfer_approvals (
    id uuid PRIMARY KEY,
    tenant_id text NOT NULL,
    gateway_id text NOT NULL REFERENCES mgmt_gateways(id),
    expected_generation bigint NOT NULL CHECK (expected_generation >= 1),
    to_generation bigint NOT NULL CHECK (to_generation = expected_generation + 1),
    new_gateway_id text NOT NULL,
    new_endpoint text NOT NULL,
    new_client_ca_sha256 text NOT NULL,
    new_client_certificate_sha256 text NOT NULL,
    requested_by uuid NOT NULL REFERENCES mgmt_users(id),
    approved_by uuid REFERENCES mgmt_users(id),
    status text NOT NULL DEFAULT 'PENDING'
        CHECK (status IN ('PENDING','APPROVED','REJECTED','ISSUED','REVOKED')),
    created_at timestamptz NOT NULL DEFAULT now(),
    approved_at timestamptz,
    closed_at timestamptz
);
CREATE UNIQUE INDEX mgmt_transfer_approvals_one_active_gateway
    ON mgmt_transfer_approvals(gateway_id)
    WHERE status IN ('PENDING','APPROVED');
CREATE INDEX mgmt_transfer_approvals_tenant_status
    ON mgmt_transfer_approvals(tenant_id,status);

-- Issuance itself is the durable consumption point. A token can therefore be issued only once,
-- even if token delivery fails and the approval must be requested again.
CREATE TABLE mgmt_transfer_issuances (
    transfer_id uuid PRIMARY KEY,
    approval_id uuid NOT NULL UNIQUE REFERENCES mgmt_transfer_approvals(id),
    signer_key_id text NOT NULL REFERENCES mgmt_transfer_signers(key_id),
    issued_by uuid NOT NULL REFERENCES mgmt_users(id),
    issued_at timestamptz NOT NULL DEFAULT now()
);
