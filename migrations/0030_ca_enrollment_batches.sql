CREATE TABLE ca_enrollment_batches (
    operation_id uuid PRIMARY KEY,
    tenant_id text NOT NULL,
    ca_fingerprint text NOT NULL CHECK (ca_fingerprint ~ '^[0-9a-f]{64}$'),
    ca_generation uuid,
    policy_revision bigint NOT NULL CHECK (policy_revision >= 0),
    required_count smallint NOT NULL CHECK (required_count >= 2),
    entries jsonb NOT NULL CHECK (jsonb_typeof(entries) = 'array' AND jsonb_array_length(entries) BETWEEN 2 AND 32),
    decision_approver_ids uuid[] CHECK (cardinality(decision_approver_ids) >= required_count),
    rejection_code text CHECK (rejection_code IN ('GATEWAY_CONFLICT', 'CERTIFICATE_INVALID')),
    CHECK (rejection_code IS NULL OR decision_approver_ids IS NOT NULL)
);

CREATE TABLE ca_enrollment_batch_approvals (
    operation_id uuid NOT NULL REFERENCES ca_enrollment_batches(operation_id),
    approver_id uuid NOT NULL REFERENCES mgmt_users(id),
    approved_at timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (operation_id, approver_id)
);
