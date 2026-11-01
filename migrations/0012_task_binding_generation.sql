-- Tasks belong to the observed cross-center binding generation, not only to a
-- per-center connection epoch. Existing pre-handshake tasks stay at generation 0
-- and are never leased by a generation-aware agent.
ALTER TABLE mgmt_tasks
    ADD COLUMN binding_generation BIGINT NOT NULL DEFAULT 0,
    ADD CONSTRAINT mgmt_tasks_binding_generation_check CHECK (binding_generation >= 0);
CREATE INDEX mgmt_tasks_generation_dispatch
    ON mgmt_tasks(gateway_id,binding_generation,created_at)
    WHERE state IN ('QUEUED','RUNNING');
