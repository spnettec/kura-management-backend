package org.eclipse.kura.management.backend.service;

import java.util.List;
import java.time.Instant;
import java.util.Map;
import java.util.Optional;

import org.eclipse.kura.management.backend.domain.CommandEntity;
import org.eclipse.kura.management.backend.domain.CommandEventEntity;
import org.eclipse.kura.management.backend.domain.CommandStatus;
import org.eclipse.kura.management.backend.repository.CommandEventRepository;
import org.eclipse.kura.management.backend.repository.CommandRepository;
import org.springframework.stereotype.Service;

@Service
public class CommandService {

    private final CommandRepository commandRepository;
    private final CommandEventRepository commandEventRepository;

    public CommandService(CommandRepository commandRepository, CommandEventRepository commandEventRepository) {
        this.commandRepository = commandRepository;
        this.commandEventRepository = commandEventRepository;
    }

    public CommandEntity createCommand(CommandEntity command) {
        CommandEntity saved = commandRepository.save(command);
        recordEvent(saved.getCommandId(), "created", saved.getStatus(), "Command created");
        return saved;
    }

    public List<CommandEntity> listCommands() {
        return commandRepository.findAllByOrderByRequestedAtDesc();
    }

    public List<CommandEntity> listCommands(String gatewayId) {
        if (gatewayId == null || gatewayId.isBlank()) {
            return listCommands();
        }
        return commandRepository.findByGatewayIdOrderByRequestedAtDesc(gatewayId);
    }

    public List<CommandEntity> listCommands(String gatewayId, String status) {
        CommandStatus commandStatus = parseStatus(status);
        boolean hasGateway = gatewayId != null && !gatewayId.isBlank();
        if (hasGateway && commandStatus != null) {
            return commandRepository.findByGatewayIdAndStatusOrderByRequestedAtDesc(gatewayId, commandStatus);
        }
        if (hasGateway) {
            return commandRepository.findByGatewayIdOrderByRequestedAtDesc(gatewayId);
        }
        if (commandStatus != null) {
            return commandRepository.findByStatusOrderByRequestedAtDesc(commandStatus);
        }
        return listCommands();
    }

    public Optional<CommandEntity> findCommand(String commandId) {
        return commandRepository.findById(commandId);
    }

    public List<CommandEventEntity> listEvents(String commandId) {
        return commandEventRepository.findByCommandIdOrderByCreatedAtAsc(commandId);
    }

    public void markSent(String commandId) {
        commandRepository.findById(commandId).ifPresent(command -> {
            if (isTerminal(command.getStatus())) {
                return;
            }
            command.setStatus(CommandStatus.SENT);
            command.setSentAt(Instant.now());
            commandRepository.save(command);
            recordEvent(commandId, "sent", CommandStatus.SENT, "Command published to MQTT");
        });
    }

    public void markSuccess(String commandId, Map<String, Object> replyPayload) {
        commandRepository.findById(commandId).ifPresent(command -> {
            if (isTerminal(command.getStatus())) {
                return;
            }
            command.setStatus(CommandStatus.SUCCESS);
            command.setReplyPayload(replyPayload);
            command.setFinishedAt(Instant.now());
            commandRepository.save(command);
            recordEvent(commandId, "success", CommandStatus.SUCCESS, "Command completed successfully");
        });
    }

    public void markFailed(String commandId, String errorCode, String errorMessage, Map<String, Object> replyPayload) {
        commandRepository.findById(commandId).ifPresent(command -> {
            if (isTerminal(command.getStatus())) {
                return;
            }
            command.setStatus(CommandStatus.FAILED);
            command.setReplyPayload(replyPayload);
            command.setErrorCode(errorCode);
            command.setErrorMessage(errorMessage);
            command.setFinishedAt(Instant.now());
            commandRepository.save(command);
            recordEvent(commandId, "failed", CommandStatus.FAILED, errorMessage);
        });
    }

    public int markTimedOutCommands() {
        List<CommandEntity> commands = commandRepository.findByStatusInAndTimeoutAtBefore(
                List.of(CommandStatus.QUEUED, CommandStatus.SENT, CommandStatus.RUNNING),
                Instant.now());
        commands.forEach(command -> {
            command.setStatus(CommandStatus.TIMEOUT);
            command.setFinishedAt(Instant.now());
            command.setErrorCode("TIMEOUT");
            command.setErrorMessage("Command timed out waiting for reply");
            commandRepository.save(command);
            recordEvent(command.getCommandId(), "timeout", CommandStatus.TIMEOUT, command.getErrorMessage());
        });
        return commands.size();
    }

    private void recordEvent(String commandId, String eventType, CommandStatus status, String message) {
        CommandEventEntity event = new CommandEventEntity();
        event.setCommandId(commandId);
        event.setEventType(eventType);
        event.setStatus(status.name().toLowerCase());
        event.setMessage(message);
        event.setCreatedAt(Instant.now());
        commandEventRepository.save(event);
    }

    private boolean isTerminal(CommandStatus status) {
        return status == CommandStatus.SUCCESS
                || status == CommandStatus.FAILED
                || status == CommandStatus.TIMEOUT
                || status == CommandStatus.REJECTED;
    }

    private CommandStatus parseStatus(String status) {
        if (status == null || status.isBlank()) {
            return null;
        }
        return CommandStatus.valueOf(status.toUpperCase());
    }
}
