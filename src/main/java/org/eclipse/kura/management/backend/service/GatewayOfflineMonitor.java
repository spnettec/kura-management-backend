package org.eclipse.kura.management.backend.service;

import java.time.Duration;
import java.time.Instant;

import org.eclipse.kura.management.backend.domain.GatewayEntity;
import org.eclipse.kura.management.backend.domain.GatewayStatus;
import org.eclipse.kura.management.backend.repository.GatewayRepository;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.scheduling.annotation.Scheduled;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

@Service
public class GatewayOfflineMonitor {

    private final GatewayRepository gatewayRepository;
    private final Duration offlineTimeout;

    public GatewayOfflineMonitor(
            GatewayRepository gatewayRepository,
            @Value("${management.gateway-offline-timeout-sec:90}") long offlineTimeoutSec) {
        this.gatewayRepository = gatewayRepository;
        this.offlineTimeout = Duration.ofSeconds(offlineTimeoutSec);
    }

    @Scheduled(fixedDelayString = "${management.gateway-offline-check-delay-ms:15000}")
    @Transactional
    public void markStaleGatewaysOffline() {
        Instant cutoff = Instant.now().minus(offlineTimeout);
        for (GatewayEntity gateway : gatewayRepository.findByStatusAndLastSeenAtBefore(GatewayStatus.ONLINE, cutoff)) {
            gateway.setStatus(GatewayStatus.OFFLINE);
            gatewayRepository.save(gateway);
        }
    }
}
