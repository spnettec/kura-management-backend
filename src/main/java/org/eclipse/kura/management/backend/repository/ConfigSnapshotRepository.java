package org.eclipse.kura.management.backend.repository;

import java.util.List;

import org.eclipse.kura.management.backend.domain.ConfigSnapshotEntity;
import org.springframework.data.jpa.repository.JpaRepository;

public interface ConfigSnapshotRepository extends JpaRepository<ConfigSnapshotEntity, Long> {

    List<ConfigSnapshotEntity> findTop50ByGatewayIdOrderByGatewaySnapshotIdDesc(String gatewayId);

    boolean existsByGatewayIdAndGatewaySnapshotId(String gatewayId, long gatewaySnapshotId);
}
