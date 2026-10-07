package org.eclipse.kura.management.backend.repository;

import java.time.Instant;
import java.util.List;

import org.eclipse.kura.management.backend.domain.GatewayEntity;
import org.eclipse.kura.management.backend.domain.GatewayStatus;
import org.springframework.data.jpa.repository.JpaRepository;

public interface GatewayRepository extends JpaRepository<GatewayEntity, String> {

    long countByStatus(GatewayStatus status);

    List<GatewayEntity> findByStatusOrderByGatewayIdAsc(GatewayStatus status);

    List<GatewayEntity> findByStatusAndLastSeenAtBefore(GatewayStatus status, Instant lastSeenAt);
}
