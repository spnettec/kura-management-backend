package org.eclipse.kura.management.backend.api.dto;

import java.time.Instant;
import java.util.List;

public record GatewaySummaryDto(
        String gatewayId,
        String tenantId,
        String name,
        String groupId,
        String status,
        boolean enabled,
        Instant lastSeenAt,
        String kuraVersion,
        String osVersion,
        List<String> ipAddrs,
        Long configRevision,
        Long snapshotId,
        int alertCount,
        List<String> tags) {
}
