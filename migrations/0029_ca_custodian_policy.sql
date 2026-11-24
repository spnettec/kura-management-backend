ALTER TABLE ca_quorum_policy
    ADD COLUMN revision bigint NOT NULL DEFAULT 0 CHECK (revision >= 0),
    ADD COLUMN custodian_ids uuid[],
    ADD CONSTRAINT ca_quorum_configured_roster CHECK (
        custodian_ids IS NULL OR
        (revision > 0 AND quorum_size >= 2 AND cardinality(custodian_ids) = total_approvers)
    );

ALTER TABLE ca_activation_proposals
    ADD COLUMN policy_revision bigint NOT NULL DEFAULT 0 CHECK (policy_revision >= 0);

CREATE TABLE ca_policy_proposals (
    operation_id uuid PRIMARY KEY,
    tenant_id text NOT NULL,
    expected_revision bigint NOT NULL CHECK (expected_revision >= 0),
    required_count smallint NOT NULL CHECK (required_count >= 2),
    custodian_ids uuid[] NOT NULL CHECK (cardinality(custodian_ids) >= required_count),
    approval_required_count smallint NOT NULL CHECK (approval_required_count >= 2),
    applied_approver_ids uuid[] CHECK (cardinality(applied_approver_ids) >= approval_required_count)
);

CREATE TABLE ca_policy_approvals (
    operation_id uuid NOT NULL REFERENCES ca_policy_proposals(operation_id),
    approver_id uuid NOT NULL REFERENCES mgmt_users(id),
    approved_at timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (operation_id, approver_id)
);
