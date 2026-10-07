package org.eclipse.kura.management.backend.api.dto;

import java.time.Instant;

public record CommandSummaryDto(
        String commandId,
        String gatewayId,
        String type,
        String status,
        String initiator,
        Instant requestedAt,
        Instant finishedAt,
        String errorCode,
        String errorMessage) {
}
