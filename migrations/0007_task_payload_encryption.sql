-- Payload backfill and its guard are installed atomically by the keyed startup
-- step, before accepting HTTP or gRPC traffic. No key is stored in PostgreSQL.
CREATE TABLE mgmt_task_crypto (
    singleton boolean PRIMARY KEY CHECK(singleton),
    version integer NOT NULL CHECK(version=1),
    key_check jsonb NOT NULL
);
