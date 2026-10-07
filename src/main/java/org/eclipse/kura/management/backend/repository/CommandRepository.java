package org.eclipse.kura.management.backend.repository;

import java.time.Instant;
import java.util.List;

import org.eclipse.kura.management.backend.domain.CommandEntity;
import org.eclipse.kura.management.backend.domain.CommandStatus;
import org.springframework.data.jpa.repository.JpaRepository;

public interface CommandRepository extends JpaRepository<CommandEntity, String> {
    List<CommandEntity> findAllByOrderByRequestedAtDesc();

    List<CommandEntity> findByGatewayIdOrderByRequestedAtDesc(String gatewayId);

    List<CommandEntity> findByStatusOrderByRequestedAtDesc(CommandStatus status);

    List<CommandEntity> findByGatewayIdAndStatusOrderByRequestedAtDesc(String gatewayId, CommandStatus status);

    List<CommandEntity> findByStatusInAndTimeoutAtBefore(List<CommandStatus> statuses, Instant timeoutAt);
}
