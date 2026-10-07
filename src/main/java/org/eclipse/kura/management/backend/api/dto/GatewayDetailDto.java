package org.eclipse.kura.management.backend.api.dto;

import java.time.Instant;
import java.util.List;
import java.util.Map;

public record GatewayDetailDto(
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
        List<String> macAddrs,
        Map<String, String> services,
        Map<String, Object> firewallStatus,
        Map<String, Object> resources,
        Long configRevision,
        Long snapshotId,
        List<String> tags) {
}
