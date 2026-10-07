package org.eclipse.kura.management.backend.api.dto;

import java.time.Instant;

public record GatewayCredentialDto(
        Long credentialId,
        String gatewayId,
        String tenantId,
        boolean enabled,
        Instant createdAt,
        Instant rotatedAt,
        Instant lastUsedAt) {
}
