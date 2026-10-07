package org.eclipse.kura.management.backend.api.dto;

import java.time.Instant;

public record GatewaySnapshotDto(
        Long snapshotPk,
        String gatewayId,
        long gatewaySnapshotId,
        Object payload,
        Instant createdAt,
        String createdBy,
        String sourceCommandId) {
}
