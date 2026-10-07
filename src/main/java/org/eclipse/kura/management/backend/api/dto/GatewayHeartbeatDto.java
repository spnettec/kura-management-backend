package org.eclipse.kura.management.backend.api.dto;

import java.time.Instant;
import java.util.List;
import java.util.Map;

public record GatewayHeartbeatDto(
        Long heartbeatId,
        String tenantId,
        String gatewayId,
        Instant ts,
        boolean online,
        String kuraVersion,
        String osVersion,
        List<String> ipAddrs,
        Long uptimeSec,
        Double cpuPct,
        Double memPct,
        Double diskPct,
        Map<String, Object> services,
        Map<String, Object> rawPayload) {
}
