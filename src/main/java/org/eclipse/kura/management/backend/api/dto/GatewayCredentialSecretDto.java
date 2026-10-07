package org.eclipse.kura.management.backend.api.dto;

import java.time.Instant;

public record GatewayCredentialSecretDto(
        Long credentialId,
        String gatewayId,
        String tenantId,
        String username,
        String secret,
        boolean enabled,
        Instant createdAt,
        Instant rotatedAt) {
}
