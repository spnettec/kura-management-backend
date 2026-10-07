package org.eclipse.kura.management.backend.repository;

import org.eclipse.kura.management.backend.domain.TenantEntity;
import org.springframework.data.jpa.repository.JpaRepository;

public interface TenantRepository extends JpaRepository<TenantEntity, String> {
}
