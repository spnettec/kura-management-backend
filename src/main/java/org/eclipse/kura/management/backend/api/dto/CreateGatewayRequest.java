package org.eclipse.kura.management.backend.api.dto;

import jakarta.validation.constraints.NotBlank;

public record CreateGatewayRequest(
        @NotBlank String gatewayId,
        String tenantId,
        String name,
        String groupId) {
}
