-- A recorded issuance can be revoked before its one-time token is consumed.
-- Revocation never resurrects a consumed token and never re-opens the approval;
-- the issuing approval stays ISSUED and a new transfer requires a new approval.
ALTER TABLE mgmt_transfer_issuances ADD COLUMN revoked_at timestamptz;
ALTER TABLE mgmt_transfer_issuances ADD COLUMN revoked_by uuid REFERENCES mgmt_users(id);
