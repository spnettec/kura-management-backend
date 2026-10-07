package org.eclipse.kura.management.backend.repository;

import java.util.List;
import java.util.Optional;

import org.eclipse.kura.management.backend.domain.GatewayHeartbeatEntity;
import org.springframework.data.jpa.repository.JpaRepository;

public interface GatewayHeartbeatRepository extends JpaRepository<GatewayHeartbeatEntity, Long> {

    Optional<GatewayHeartbeatEntity> findFirstByGatewayIdOrderByTsDesc(String gatewayId);

    List<GatewayHeartbeatEntity> findTop20ByGatewayIdOrderByTsDesc(String gatewayId);
}
