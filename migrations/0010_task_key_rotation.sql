-- Version 2 means retained payloads were atomically rekeyed by task-key-rotate.
ALTER TABLE mgmt_task_crypto DROP CONSTRAINT mgmt_task_crypto_version_check;
ALTER TABLE mgmt_task_crypto ADD CONSTRAINT mgmt_task_crypto_version_check CHECK(version IN (1,2));
