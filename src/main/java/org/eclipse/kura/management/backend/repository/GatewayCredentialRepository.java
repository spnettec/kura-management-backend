package org.eclipse.kura.management.backend.repository;

import java.util.Optional;

import org.eclipse.kura.management.backend.domain.GatewayCredentialEntity;
import org.springframework.data.jpa.repository.JpaRepository;

public interface GatewayCredentialRepository extends JpaRepository<GatewayCredentialEntity, Long> {

    Optional<GatewayCredentialEntity> findFirstByGatewayIdOrderByCredentialIdDesc(String gatewayId);
}
