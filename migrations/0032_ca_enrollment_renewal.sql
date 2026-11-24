ALTER TABLE ca_enrollment_batches
    DROP CONSTRAINT ca_enrollment_batches_rejection_code_check,
    ADD CONSTRAINT ca_enrollment_batches_rejection_code_check
        CHECK (rejection_code IN ('GATEWAY_CONFLICT', 'CERTIFICATE_INVALID', 'RENEWAL_FENCED'));
