CREATE TABLE ca_signing_requests (
    operation_id uuid PRIMARY KEY,
    tenant_id text NOT NULL,
    manifest_sha256 text NOT NULL CHECK (manifest_sha256 ~ '^[0-9a-f]{64}$'),
    entries jsonb NOT NULL CHECK (jsonb_typeof(entries) = 'array' AND jsonb_array_length(entries) BETWEEN 2 AND 32),
    UNIQUE (operation_id, tenant_id, manifest_sha256)
);

ALTER TABLE ca_enrollment_batches
    ADD COLUMN signing_manifest_sha256 text,
    ADD CONSTRAINT ca_enrollment_signing_manifest
        FOREIGN KEY (operation_id, tenant_id, signing_manifest_sha256)
        REFERENCES ca_signing_requests(operation_id, tenant_id, manifest_sha256);
