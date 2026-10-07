package org.eclipse.kura.management.backend.config;

import java.time.Instant;
import java.util.List;

import org.eclipse.kura.management.backend.domain.GatewayCredentialEntity;
import org.eclipse.kura.management.backend.domain.GatewayEntity;
import org.eclipse.kura.management.backend.domain.GatewayStatus;
import org.eclipse.kura.management.backend.domain.TenantEntity;
import org.eclipse.kura.management.backend.repository.GatewayCredentialRepository;
import org.eclipse.kura.management.backend.repository.GatewayRepository;
import org.eclipse.kura.management.backend.repository.TenantRepository;
import org.springframework.boot.CommandLineRunner;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.security.crypto.bcrypt.BCryptPasswordEncoder;

@Configuration
public class DemoDataInitializer {

    @Bean
    CommandLineRunner seedDemoData(
            TenantRepository tenantRepository,
            GatewayRepository gatewayRepository,
            GatewayCredentialRepository credentialRepository) {
        return args -> {
            if (!tenantRepository.existsById("t-001")) {
                TenantEntity tenant = new TenantEntity();
                tenant.setTenantId("t-001");
                tenant.setName("Default Tenant");
                tenant.setStatus("active");
                tenantRepository.save(tenant);
            }

            if (!gatewayRepository.existsById("gw-001")) {
                GatewayEntity gateway = new GatewayEntity();
                gateway.setGatewayId("gw-001");
                gateway.setTenantId("t-001");
                gateway.setName("orb-ubuntu-01");
                gateway.setEnabled(true);
                gateway.setStatus(GatewayStatus.ONLINE);
                gateway.setLastSeenAt(Instant.now());
                gateway.setKuraVersion("6.0.0");
                gateway.setOsVersion("Ubuntu 24.04");
                gateway.setIpAddrs(List.of("192.168.139.101"));
                gateway.setMacAddrs(List.of("02:00:00:00:00:01"));
                gateway.setConfigRevision(1L);
                gateway.setSnapshotId(1778879900000L);
                gateway.setAlertCount(0);
                gatewayRepository.save(gateway);
            }

            if (credentialRepository.findFirstByGatewayIdOrderByCredentialIdDesc("gw-001").isEmpty()) {
                GatewayCredentialEntity credential = new GatewayCredentialEntity();
                credential.setGatewayId("gw-001");
                credential.setTenantId("t-001");
                credential.setSecretHash(new BCryptPasswordEncoder().encode("gw-001-secret"));
                credential.setEnabled(true);
                credential.setCreatedAt(Instant.now());
                credentialRepository.save(credential);
            }
        };
    }
}
