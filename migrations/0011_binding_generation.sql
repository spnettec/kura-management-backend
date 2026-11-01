-- Cross-center binding generation is distinct from per-center connection epoch.
-- Existing single-center registrations retain binding generation 1.
ALTER TABLE mgmt_gateways
    ADD COLUMN binding_generation BIGINT NOT NULL DEFAULT 1,
    ADD CONSTRAINT mgmt_gateways_binding_generation_check CHECK (binding_generation >= 1);
