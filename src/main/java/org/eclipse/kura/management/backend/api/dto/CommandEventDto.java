package org.eclipse.kura.management.backend.api.dto;

import java.time.Instant;

public record CommandEventDto(
        Long eventId,
        String eventType,
        String status,
        String message,
        Instant createdAt) {
}
