-- CA operation quorum: multi-person approval before trust-anchor generation activation.
-- Scheme A: external CA signing, the center never stores a CA private key.
-- Each CA generation activation requires QUORUM_SIZE distinct admin approvals.
CREATE TABLE IF NOT EXISTS ca_operation_approvals (
    id              uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id       text NOT NULL,
    operation_id    uuid NOT NULL,
    operation_kind  text NOT NULL CHECK (operation_kind IN ('generation-activate','generation-revoke','batch-import')),
    generation_id   uuid,
    approver        text NOT NULL,
    approved_at     timestamptz NOT NULL DEFAULT now(),
    status          text NOT NULL DEFAULT 'pending' CHECK (status IN ('pending','approved','rejected','expired')),
    UNIQUE (tenant_id, operation_id, approver, operation_kind)
);
CREATE INDEX IF NOT EXISTS idx_ca_approvals_operation
    ON ca_operation_approvals(tenant_id, operation_id, status);

-- Quorum policy: the required approval count per operation kind.
CREATE TABLE IF NOT EXISTS ca_quorum_policy (
    tenant_id       text PRIMARY KEY,
    quorum_size     smallint NOT NULL DEFAULT 2 CHECK (quorum_size >= 1),
    total_approvers smallint NOT NULL DEFAULT 3 CHECK (total_approvers >= quorum_size)
);
