package org.eclipse.kura.management.backend.api.dto;

import java.util.Map;

import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.NotNull;

public record CreateCommandRequest(
        @NotBlank String gatewayId,
        @NotBlank String type,
        @NotNull Map<String, Object> payload) {
}
