package org.eclipse.kura.management.backend.api.dto;

import java.time.Instant;
import java.util.List;
import java.util.Map;

public record CommandDetailDto(
        String commandId,
        String tenantId,
        String gatewayId,
        String type,
        String status,
        Map<String, Object> requestPayload,
        Map<String, Object> replyPayload,
        String initiator,
        String correlationId,
        Instant requestedAt,
        Instant sentAt,
        Instant startedAt,
        Instant finishedAt,
        Instant timeoutAt,
        String errorCode,
        String errorMessage,
        List<CommandEventDto> events) {
}
