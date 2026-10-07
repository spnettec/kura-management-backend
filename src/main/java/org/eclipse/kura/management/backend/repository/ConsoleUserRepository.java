package org.eclipse.kura.management.backend.repository;

import java.util.Optional;

import org.eclipse.kura.management.backend.domain.ConsoleUserEntity;
import org.springframework.data.jpa.repository.JpaRepository;

public interface ConsoleUserRepository extends JpaRepository<ConsoleUserEntity, Long> {

    Optional<ConsoleUserEntity> findByUsernameAndEnabledTrue(String username);

    boolean existsByUsername(String username);
}
