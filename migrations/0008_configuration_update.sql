-- Typed, set-only conditional update; existing operation numbers remain intact.
ALTER TABLE mgmt_tasks DROP CONSTRAINT mgmt_tasks_operation_check;
ALTER TABLE mgmt_tasks ADD CONSTRAINT mgmt_tasks_operation_check CHECK(operation BETWEEN 1 AND 12);
