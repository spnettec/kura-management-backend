-- Existing original receipts remain version 1 with no inferred generations.
ALTER TABLE mgmt_gateway_certificate_rotations
    ADD COLUMN receipt_version smallint NOT NULL DEFAULT 1,
    ADD COLUMN previous_binding_generation bigint,
    ADD COLUMN binding_generation bigint,
    ADD CONSTRAINT mgmt_gateway_certificate_receipt_generations CHECK (
        (receipt_version = 1 AND previous_binding_generation IS NULL AND binding_generation IS NULL)
        OR (receipt_version = 2 AND previous_binding_generation >= 1
            AND previous_binding_generation IS NOT NULL
            AND ((previous_binding_generation < 9223372036854775807
                AND binding_generation IS NOT NULL
                AND binding_generation::numeric = previous_binding_generation::numeric + 1)
                OR (previous_binding_generation = 9223372036854775807
                    AND binding_generation IS NULL AND state = 'REJECTED')))
    );
