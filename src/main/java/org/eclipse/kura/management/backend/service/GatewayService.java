package org.eclipse.kura.management.backend.service;

import java.time.Instant;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Optional;

import org.eclipse.kura.management.backend.domain.GatewayEntity;
import org.eclipse.kura.management.backend.domain.GatewayHeartbeatEntity;
import org.eclipse.kura.management.backend.domain.GatewayStatus;
import org.eclipse.kura.management.backend.domain.TenantEntity;
import org.eclipse.kura.management.backend.api.dto.CreateGatewayRequest;
import org.eclipse.kura.management.backend.repository.GatewayHeartbeatRepository;
import org.eclipse.kura.management.backend.repository.GatewayRepository;
import org.eclipse.kura.management.backend.repository.TenantRepository;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

@Service
public class GatewayService {

    private final GatewayRepository gatewayRepository;
    private final GatewayHeartbeatRepository heartbeatRepository;
    private final TenantRepository tenantRepository;

    public GatewayService(
            GatewayRepository gatewayRepository,
            GatewayHeartbeatRepository heartbeatRepository,
            TenantRepository tenantRepository) {
        this.gatewayRepository = gatewayRepository;
        this.heartbeatRepository = heartbeatRepository;
        this.tenantRepository = tenantRepository;
    }

    public List<GatewayEntity> listGateways() {
        return gatewayRepository.findAll();
    }

    public List<GatewayEntity> listGateways(String status) {
        GatewayStatus gatewayStatus = parseStatus(status);
        if (gatewayStatus == null) {
            return listGateways();
        }
        return gatewayRepository.findByStatusOrderByGatewayIdAsc(gatewayStatus);
    }

    public Optional<GatewayEntity> findGateway(String gatewayId) {
        return gatewayRepository.findById(gatewayId);
    }

    public GatewayEntity requireEnabledGateway(String gatewayId) {
        GatewayEntity gateway = gatewayRepository.findById(gatewayId)
                .orElseThrow(() -> new IllegalArgumentException("Gateway not found: " + gatewayId));
        if (!gateway.isEnabled()) {
            throw new IllegalArgumentException("Gateway is disabled: " + gatewayId);
        }
        return gateway;
    }

    @Transactional
    public GatewayEntity createGateway(CreateGatewayRequest request) {
        String tenantId = request.tenantId() == null || request.tenantId().isBlank() ? "t-001" : request.tenantId();
        ensureTenant(tenantId);
        if (gatewayRepository.existsById(request.gatewayId())) {
            throw new IllegalArgumentException("Gateway already exists: " + request.gatewayId());
        }

        GatewayEntity gateway = new GatewayEntity();
        gateway.setGatewayId(request.gatewayId());
        gateway.setTenantId(tenantId);
        gateway.setName(request.name() == null || request.name().isBlank() ? request.gatewayId() : request.name());
        gateway.setGroupId(request.groupId());
        gateway.setStatus(GatewayStatus.UNKNOWN);
        gateway.setEnabled(true);
        return gatewayRepository.save(gateway);
    }

    @Transactional
    public GatewayEntity setEnabled(String gatewayId, boolean enabled) {
        GatewayEntity gateway = gatewayRepository.findById(gatewayId)
                .orElseThrow(() -> new IllegalArgumentException("Gateway not found: " + gatewayId));
        gateway.setEnabled(enabled);
        if (!enabled) {
            gateway.setStatus(GatewayStatus.OFFLINE);
        }
        return gatewayRepository.save(gateway);
    }

    public Map<String, Long> getGatewaySummary() {
        return Map.of(
                "total", gatewayRepository.count(),
                "online", gatewayRepository.countByStatus(GatewayStatus.ONLINE),
                "offline", gatewayRepository.countByStatus(GatewayStatus.OFFLINE),
                "unknown", gatewayRepository.countByStatus(GatewayStatus.UNKNOWN));
    }

    public Optional<GatewayHeartbeatEntity> findLatestHeartbeat(String gatewayId) {
        return heartbeatRepository.findFirstByGatewayIdOrderByTsDesc(gatewayId);
    }

    public List<GatewayHeartbeatEntity> listRecentHeartbeats(String gatewayId) {
        return heartbeatRepository.findTop20ByGatewayIdOrderByTsDesc(gatewayId);
    }

    @Transactional
    public void recordStatusReport(String tenantId, String gatewayId, Instant ts, boolean online,
            Map<String, Object> payload) {
        GatewayEntity gateway = gatewayRepository.findById(gatewayId)
                .orElseThrow(() -> new IllegalArgumentException("Gateway is not registered: " + gatewayId));
        if (!gateway.isEnabled()) {
            throw new IllegalArgumentException("Gateway is disabled: " + gatewayId);
        }

        gateway.setTenantId(tenantId);
        gateway.setStatus(online ? GatewayStatus.ONLINE : GatewayStatus.OFFLINE);
        gateway.setLastSeenAt(ts);
        applySystemPayload(gateway, payload.get("system"));
        gatewayRepository.save(gateway);

        GatewayHeartbeatEntity heartbeat = new GatewayHeartbeatEntity();
        heartbeat.setTenantId(tenantId);
        heartbeat.setGatewayId(gatewayId);
        heartbeat.setTs(ts);
        heartbeat.setOnline(online);
        heartbeat.setRawPayload(payload);
        applyHeartbeatPayload(heartbeat, payload);
        heartbeatRepository.save(heartbeat);
    }

    private void ensureTenant(String tenantId) {
        if (tenantRepository.existsById(tenantId)) {
            return;
        }
        TenantEntity tenant = new TenantEntity();
        tenant.setTenantId(tenantId);
        tenant.setName(tenantId);
        tenant.setStatus("active");
        tenantRepository.save(tenant);
    }

    private void applySystemPayload(GatewayEntity gateway, Object systemPayload) {
        if (!(systemPayload instanceof Map<?, ?> system)) {
            return;
        }

        gateway.setKuraVersion(asString(system.get("kuraVersion")));
        String osName = asString(system.get("osVersion"));
        String kernelVersion = asString(system.get("kernelVersion"));
        if (kernelVersion != null && !kernelVersion.isBlank() && osName != null && !osName.isBlank()) {
            gateway.setOsVersion(osName + " " + kernelVersion);
        } else {
            gateway.setOsVersion(osName);
        }

        Object ipAddrs = system.get("ipAddrs");
        if (ipAddrs instanceof List<?> values) {
            gateway.setIpAddrs(values.stream()
                    .map(String::valueOf)
                    .toList());
        } else {
            gateway.setIpAddrs(new ArrayList<>());
        }
    }

    private void applyHeartbeatPayload(GatewayHeartbeatEntity heartbeat, Map<String, Object> payload) {
        Object systemPayload = payload.get("system");
        if (systemPayload instanceof Map<?, ?> system) {
            heartbeat.setKuraVersion(asString(system.get("kuraVersion")));
            String osName = asString(system.get("osVersion"));
            String kernelVersion = asString(system.get("kernelVersion"));
            heartbeat.setOsVersion(kernelVersion == null || kernelVersion.isBlank() ? osName : osName + " " + kernelVersion);
            heartbeat.setUptimeSec(asLong(system.get("uptimeSec")));
            Object ipAddrs = system.get("ipAddrs");
            if (ipAddrs instanceof List<?> values) {
                heartbeat.setIpAddrs(values.stream().map(String::valueOf).toList());
            }
        }

        Object resources = payload.get("resources");
        if (resources instanceof Map<?, ?> values) {
            heartbeat.setCpuPct(asDouble(values.get("cpu")));
            heartbeat.setMemPct(asDouble(values.get("mem")));
            heartbeat.setDiskPct(asDouble(values.get("disk")));
        }

        Object services = payload.get("services");
        if (services instanceof Map<?, ?> values) {
            heartbeat.setServices(values.entrySet().stream()
                    .collect(HashMap::new, (map, entry) -> map.put(String.valueOf(entry.getKey()), entry.getValue()),
                            HashMap::putAll));
        }
    }

    private String asString(Object value) {
        if (value == null) {
            return null;
        }
        return String.valueOf(value);
    }

    private Long asLong(Object value) {
        if (value instanceof Number number) {
            return number.longValue();
        }
        if (value == null) {
            return null;
        }
        return Long.parseLong(String.valueOf(value));
    }

    private Double asDouble(Object value) {
        if (value instanceof Number number) {
            return number.doubleValue();
        }
        if (value == null) {
            return null;
        }
        return Double.parseDouble(String.valueOf(value));
    }

    private GatewayStatus parseStatus(String status) {
        if (status == null || status.isBlank()) {
            return null;
        }
        return GatewayStatus.valueOf(status.toUpperCase());
    }
}
