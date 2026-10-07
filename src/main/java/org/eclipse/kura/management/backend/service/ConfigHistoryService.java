package org.eclipse.kura.management.backend.service;

import java.util.Map;
import java.util.Optional;

import org.eclipse.kura.management.backend.domain.CommandEntity;
import org.eclipse.kura.management.backend.domain.ConfigSnapshotEntity;
import org.eclipse.kura.management.backend.repository.CommandRepository;
import org.eclipse.kura.management.backend.repository.ConfigSnapshotRepository;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.stereotype.Service;

/**
 * Persists gateway-side snapshot results reported through successful command replies,
 * giving the console a backend-side history without an extra gateway round trip.
 */
@Service
public class ConfigHistoryService {

    private static final Logger logger = LoggerFactory.getLogger(ConfigHistoryService.class);

    private final CommandRepository commandRepository;
    private final ConfigSnapshotRepository snapshotRepository;

    public ConfigHistoryService(CommandRepository commandRepository, ConfigSnapshotRepository snapshotRepository) {
        this.commandRepository = commandRepository;
        this.snapshotRepository = snapshotRepository;
    }

    public void recordFromReply(String commandId) {
        try {
            Optional<CommandEntity> found = this.commandRepository.findById(commandId);
            if (found.isEmpty()) {
                return;
            }
            CommandEntity command = found.get();
            if (!"snapshot.take".equals(command.getType()) && !"snapshot.rollback".equals(command.getType())) {
                return;
            }
            Long gatewaySnapshotId = extractSnapshotId(command.getReplyPayload());
            if (gatewaySnapshotId == null) {
                logger.debug("Snapshot command {} succeeded without a snapshot id", commandId);
                return;
            }
            if (this.snapshotRepository.existsByGatewayIdAndGatewaySnapshotId(command.getGatewayId(),
                    gatewaySnapshotId)) {
                return;
            }
            ConfigSnapshotEntity snapshot = new ConfigSnapshotEntity();
            snapshot.setTenantId(command.getTenantId());
            snapshot.setGatewayId(command.getGatewayId());
            snapshot.setGatewaySnapshotId(gatewaySnapshotId);
            snapshot.setRevision(0);
            snapshot.setPayload(command.getReplyPayload() == null ? Map.of() : command.getReplyPayload());
            snapshot.setCreatedAt(command.getFinishedAt());
            snapshot.setCreatedBy(command.getInitiator());
            snapshot.setSourceCommandId(command.getCommandId());
            this.snapshotRepository.save(snapshot);
        } catch (Exception e) {
            logger.warn("Unable to record config snapshot for command {}: {}", commandId, e.getMessage());
        }
    }

    private Long extractSnapshotId(Object replyPayload) {
        if (replyPayload instanceof Map<?, ?> payload) {
            Object id = payload.get("snapshotId");
            if (id instanceof Number number) {
                return number.longValue();
            }
        }
        return null;
    }
}
