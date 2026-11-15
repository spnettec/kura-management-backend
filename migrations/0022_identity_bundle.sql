ALTER TABLE mgmt_gateways ADD COLUMN identity_bundle_token_hash TEXT;
ALTER TABLE mgmt_gateways ADD COLUMN identity_bundle_ciphertext TEXT;
ALTER TABLE mgmt_gateways ADD COLUMN identity_bundle_issued_at TIMESTAMPTZ;
ALTER TABLE mgmt_gateways ADD COLUMN identity_bundle_delivered_at TIMESTAMPTZ;
