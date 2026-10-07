package org.eclipse.kura.management.backend.repository;

import org.eclipse.kura.management.backend.domain.AuditEventEntity;
import org.springframework.data.jpa.repository.JpaRepository;

public interface AuditEventRepository extends JpaRepository<AuditEventEntity, Long> {
}
