-- Uncertain executions are retained and queried; they are never re-dispatched.
CREATE INDEX mgmt_tasks_reconciliation ON mgmt_tasks(gateway_id,lease_until)
    WHERE state IN ('RUNNING','UNKNOWN');
