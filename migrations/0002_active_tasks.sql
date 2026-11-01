CREATE INDEX mgmt_tasks_active ON mgmt_tasks(tenant_id,gateway_id)
    WHERE state IN ('QUEUED','RUNNING');
