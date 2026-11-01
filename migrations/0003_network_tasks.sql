-- Existing read-only tasks retain an empty typed input.
ALTER TABLE mgmt_tasks DROP CONSTRAINT mgmt_tasks_operation_check;
ALTER TABLE mgmt_tasks ADD CONSTRAINT mgmt_tasks_operation_check CHECK(operation BETWEEN 1 AND 8);
ALTER TABLE mgmt_tasks ADD COLUMN input jsonb NOT NULL DEFAULT '{}';
