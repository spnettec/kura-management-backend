package org.eclipse.kura.management.backend.api;

import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

import org.eclipse.kura.management.backend.api.dto.GatewayDetailDto;
import org.eclipse.kura.management.backend.api.dto.CreateGatewayRequest;
import org.eclipse.kura.management.backend.api.dto.GatewayCredentialDto;
import org.eclipse.kura.management.backend.api.dto.GatewayCredentialSecretDto;
import org.eclipse.kura.management.backend.api.dto.GatewayHeartbeatDto;
import org.eclipse.kura.management.backend.api.dto.GatewaySummaryDto;
import org.eclipse.kura.management.backend.api.dto.SetEnabledRequest;
import org.eclipse.kura.management.backend.domain.GatewayCredentialEntity;
import org.eclipse.kura.management.backend.domain.GatewayEntity;
import org.eclipse.kura.management.backend.domain.GatewayHeartbeatEntity;
import org.eclipse.kura.management.backend.service.GatewayCredentialService;
import org.eclipse.kura.management.backend.service.GatewayService;
import jakarta.validation.Valid;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PatchMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/api/gateways")
public class GatewayController {

    private final GatewayService gatewayService;
    private final GatewayCredentialService gatewayCredentialService;

    public GatewayController(GatewayService gatewayService, GatewayCredentialService gatewayCredentialService) {
        this.gatewayService = gatewayService;
        this.gatewayCredentialService = gatewayCredentialService;
    }

    @GetMapping
    public Map<String, Object> listGateways(@RequestParam(required = false) String status) {
        List<GatewaySummaryDto> items = gatewayService.listGateways(status).stream()
                .map(this::toSummaryDto)
                .collect(Collectors.toList());
        return Map.of("items", items, "total", items.size());
    }

    @PostMapping
    public GatewaySummaryDto createGateway(@Valid @RequestBody CreateGatewayRequest request) {
        return toSummaryDto(gatewayService.createGateway(request));
    }

    @GetMapping("/summary")
    public Map<String, Long> getGatewaySummary() {
        return gatewayService.getGatewaySummary();
    }

    @GetMapping("/{gatewayId}")
    public GatewayDetailDto getGateway(@PathVariable String gatewayId) {
        return gatewayService.findGateway(gatewayId)
                .map(this::toDetailDto)
                .orElseThrow(() -> new IllegalArgumentException("Gateway not found: " + gatewayId));
    }

    @PatchMapping("/{gatewayId}")
    public GatewayDetailDto setGatewayEnabled(
            @PathVariable String gatewayId,
            @RequestBody SetEnabledRequest request) {
        return toDetailDto(gatewayService.setEnabled(gatewayId, request.enabled()));
    }

    @GetMapping("/{gatewayId}/credentials")
    public GatewayCredentialDto getCredential(@PathVariable String gatewayId) {
        return gatewayCredentialService.findLatest(gatewayId)
                .map(this::toCredentialDto)
                .orElseThrow(() -> new IllegalArgumentException("Gateway credential not found: " + gatewayId));
    }

    @PostMapping("/{gatewayId}/credentials")
    public GatewayCredentialSecretDto rotateCredential(@PathVariable String gatewayId) {
        return gatewayCredentialService.rotate(gatewayId);
    }

    @PatchMapping("/{gatewayId}/credentials")
    public GatewayCredentialDto setCredentialEnabled(
            @PathVariable String gatewayId,
            @RequestBody SetEnabledRequest request) {
        return toCredentialDto(gatewayCredentialService.setEnabled(gatewayId, request.enabled()));
    }

    @GetMapping("/{gatewayId}/heartbeats")
    public Map<String, Object> listHeartbeats(@PathVariable String gatewayId) {
        List<GatewayHeartbeatDto> items = gatewayService.listRecentHeartbeats(gatewayId).stream()
                .map(this::toHeartbeatDto)
                .collect(Collectors.toList());
        return Map.of("items", items, "total", items.size());
    }

    private GatewaySummaryDto toSummaryDto(GatewayEntity gateway) {
        return new GatewaySummaryDto(
                gateway.getGatewayId(),
                gateway.getTenantId(),
                gateway.getName(),
                gateway.getGroupId(),
                gateway.getStatus().name().toLowerCase(),
                gateway.isEnabled(),
                gateway.getLastSeenAt(),
                gateway.getKuraVersion(),
                gateway.getOsVersion(),
                gateway.getIpAddrs(),
                gateway.getConfigRevision(),
                gateway.getSnapshotId(),
                gateway.getAlertCount(),
                List.of());
    }

    private GatewayDetailDto toDetailDto(GatewayEntity gateway) {
        Map<String, String> services = new LinkedHashMap<>();
        services.put("kura", "unknown");
        services.put("firewall", "unknown");
        services.put("cloud", "unknown");
        Map<String, Object> firewallStatus = new LinkedHashMap<>();
        firewallStatus.put("enabled", false);
        firewallStatus.put("serviceState", "unknown");
        firewallStatus.put("ipv4RuleCount", 0);
        firewallStatus.put("ipv6RuleCount", 0);
        Map<String, Object> resources = new LinkedHashMap<>();
        resources.put("cpu", 0.0);
        resources.put("mem", 0.0);
        resources.put("disk", 0.0);
        resources.put("uptimeSec", 0L);

        gatewayService.findLatestHeartbeat(gateway.getGatewayId()).ifPresent(heartbeat -> {
            resources.put("cpu", valueOrZero(heartbeat.getCpuPct()));
            resources.put("mem", valueOrZero(heartbeat.getMemPct()));
            resources.put("disk", valueOrZero(heartbeat.getDiskPct()));
            resources.put("uptimeSec", heartbeat.getUptimeSec() == null ? 0L : heartbeat.getUptimeSec());
            heartbeat.getServices().forEach((key, value) -> services.put(key, String.valueOf(value)));
            Object firewall = heartbeat.getRawPayload().get("firewall");
            if (firewall instanceof Map<?, ?> values) {
                firewallStatus.clear();
                values.forEach((key, value) -> firewallStatus.put(String.valueOf(key), value));
            }
        });

        return new GatewayDetailDto(
                gateway.getGatewayId(),
                gateway.getTenantId(),
                gateway.getName(),
                gateway.getGroupId(),
                gateway.getStatus().name().toLowerCase(),
                gateway.isEnabled(),
                gateway.getLastSeenAt(),
                gateway.getKuraVersion(),
                gateway.getOsVersion(),
                gateway.getIpAddrs(),
                gateway.getMacAddrs(),
                services,
                firewallStatus,
                resources,
                gateway.getConfigRevision(),
                gateway.getSnapshotId(),
                List.of());
    }

    private GatewayHeartbeatDto toHeartbeatDto(GatewayHeartbeatEntity heartbeat) {
        return new GatewayHeartbeatDto(
                heartbeat.getHeartbeatId(),
                heartbeat.getTenantId(),
                heartbeat.getGatewayId(),
                heartbeat.getTs(),
                heartbeat.isOnline(),
                heartbeat.getKuraVersion(),
                heartbeat.getOsVersion(),
                heartbeat.getIpAddrs(),
                heartbeat.getUptimeSec(),
                heartbeat.getCpuPct(),
                heartbeat.getMemPct(),
                heartbeat.getDiskPct(),
                heartbeat.getServices(),
                heartbeat.getRawPayload());
    }

    private double valueOrZero(Double value) {
        return value == null ? 0.0 : value;
    }

    private GatewayCredentialDto toCredentialDto(GatewayCredentialEntity credential) {
        return new GatewayCredentialDto(
                credential.getCredentialId(),
                credential.getGatewayId(),
                credential.getTenantId(),
                credential.isEnabled(),
                credential.getCreatedAt(),
                credential.getRotatedAt(),
                credential.getLastUsedAt());
    }
}
