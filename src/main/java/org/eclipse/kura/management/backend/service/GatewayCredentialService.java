package org.eclipse.kura.management.backend.service;

import java.security.SecureRandom;
import java.time.Instant;
import java.util.Base64;
import java.util.Optional;

import org.eclipse.kura.management.backend.api.dto.GatewayCredentialSecretDto;
import org.eclipse.kura.management.backend.domain.GatewayCredentialEntity;
import org.eclipse.kura.management.backend.domain.GatewayEntity;
import org.eclipse.kura.management.backend.repository.GatewayCredentialRepository;
import org.eclipse.kura.management.backend.repository.GatewayRepository;
import org.springframework.security.crypto.bcrypt.BCryptPasswordEncoder;
import org.springframework.security.crypto.password.PasswordEncoder;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

@Service
public class GatewayCredentialService {

    private final GatewayCredentialRepository credentialRepository;
    private final GatewayRepository gatewayRepository;
    private final PasswordEncoder passwordEncoder = new BCryptPasswordEncoder();
    private final SecureRandom secureRandom = new SecureRandom();

    public GatewayCredentialService(
            GatewayCredentialRepository credentialRepository,
            GatewayRepository gatewayRepository) {
        this.credentialRepository = credentialRepository;
        this.gatewayRepository = gatewayRepository;
    }

    public Optional<GatewayCredentialEntity> findLatest(String gatewayId) {
        return credentialRepository.findFirstByGatewayIdOrderByCredentialIdDesc(gatewayId);
    }

    public boolean canAcceptGateway(String tenantId, String gatewayId) {
        return findLatest(gatewayId)
                .filter(GatewayCredentialEntity::isEnabled)
                .filter(credential -> credential.getTenantId().equals(tenantId))
                .isPresent();
    }

    public boolean verifySecret(String tenantId, String gatewayId, String secret) {
        if (secret == null || secret.isBlank()) {
            return false;
        }
        return findLatest(gatewayId)
                .filter(GatewayCredentialEntity::isEnabled)
                .filter(credential -> credential.getTenantId().equals(tenantId))
                .filter(credential -> passwordEncoder.matches(secret, credential.getSecretHash()))
                .isPresent();
    }

    @Transactional
    public void markUsed(String gatewayId) {
        findLatest(gatewayId).ifPresent(credential -> {
            credential.setLastUsedAt(Instant.now());
            credentialRepository.save(credential);
        });
    }

    @Transactional
    public GatewayCredentialSecretDto rotate(String gatewayId) {
        GatewayEntity gateway = gatewayRepository.findById(gatewayId)
                .orElseThrow(() -> new IllegalArgumentException("Gateway not found: " + gatewayId));

        String secret = generateSecret();
        GatewayCredentialEntity credential = findLatest(gatewayId).orElseGet(GatewayCredentialEntity::new);
        Instant now = Instant.now();
        if (credential.getCredentialId() == null) {
            credential.setCreatedAt(now);
        } else {
            credential.setRotatedAt(now);
        }
        credential.setGatewayId(gateway.getGatewayId());
        credential.setTenantId(gateway.getTenantId());
        credential.setSecretHash(passwordEncoder.encode(secret));
        credential.setEnabled(true);
        GatewayCredentialEntity saved = credentialRepository.save(credential);

        return new GatewayCredentialSecretDto(
                saved.getCredentialId(),
                saved.getGatewayId(),
                saved.getTenantId(),
                saved.getGatewayId(),
                secret,
                saved.isEnabled(),
                saved.getCreatedAt(),
                saved.getRotatedAt());
    }

    @Transactional
    public GatewayCredentialEntity setEnabled(String gatewayId, boolean enabled) {
        GatewayCredentialEntity credential = findLatest(gatewayId)
                .orElseThrow(() -> new IllegalArgumentException("Gateway credential not found: " + gatewayId));
        credential.setEnabled(enabled);
        return credentialRepository.save(credential);
    }

    private String generateSecret() {
        byte[] bytes = new byte[32];
        secureRandom.nextBytes(bytes);
        return Base64.getUrlEncoder().withoutPadding().encodeToString(bytes);
    }
}
