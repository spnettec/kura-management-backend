package org.eclipse.kura.management.backend.repository;

import java.util.List;

import org.eclipse.kura.management.backend.domain.CommandEventEntity;
import org.springframework.data.jpa.repository.JpaRepository;

public interface CommandEventRepository extends JpaRepository<CommandEventEntity, Long> {

    List<CommandEventEntity> findByCommandIdOrderByCreatedAtAsc(String commandId);
}
