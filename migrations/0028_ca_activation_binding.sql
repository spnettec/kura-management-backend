CREATE TABLE ca_activation_proposals (
    operation_id uuid PRIMARY KEY,
    tenant_id text NOT NULL,
    generation_id uuid NOT NULL,
    expected_generation uuid NOT NULL,
    pending_revision text NOT NULL CHECK (pending_revision ~ '^[0-9a-f]{64}$')
);

-- Rows written before this binding was introduced cannot authorize activation.
ALTER TABLE ca_operation_approvals
    ADD COLUMN approver_id uuid REFERENCES mgmt_users(id);
CREATE UNIQUE INDEX ca_activation_distinct_admin
    ON ca_operation_approvals(operation_id, approver_id)
    WHERE operation_kind = 'generation-activate' AND approver_id IS NOT NULL;
