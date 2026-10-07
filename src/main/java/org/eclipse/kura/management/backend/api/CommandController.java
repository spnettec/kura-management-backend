package org.eclipse.kura.management.backend.api;

import java.time.Instant;
import java.time.temporal.ChronoUnit;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.UUID;
import java.util.stream.Collectors;

import org.eclipse.kura.management.backend.api.dto.CommandEventDto;
import org.eclipse.kura.management.backend.api.dto.CommandSummaryDto;
import org.eclipse.kura.management.backend.api.dto.CommandDetailDto;
import org.eclipse.kura.management.backend.api.dto.CreateCommandRequest;
import org.eclipse.kura.management.backend.domain.CommandEntity;
import org.eclipse.kura.management.backend.domain.CommandStatus;
import org.eclipse.kura.management.backend.service.CommandService;
import org.eclipse.kura.management.backend.service.GatewayService;
import org.eclipse.kura.management.backend.service.MqttDispatchService;
import org.eclipse.kura.management.backend.service.TokenService;
import jakarta.validation.Valid;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/api/commands")
public class CommandController {

    /**
     * Types the gateway agent is able to execute; keep in sync with the agent's
     * DefaultCommandExecutor dispatch table.
     */
    private static final Set<String> SUPPORTED_TYPES = Set.of(
            "system.info",
            "config.list", "config.get", "config.put",
            "firewall.get", "network.get",
            "snapshot.list", "snapshot.take", "snapshot.rollback");

    private final CommandService commandService;
    private final GatewayService gatewayService;
    private final MqttDispatchService mqttDispatchService;
    private final long commandTimeoutSec;

    public CommandController(
            CommandService commandService,
            GatewayService gatewayService,
            MqttDispatchService mqttDispatchService,
            @Value("${management.command-timeout-sec:30}") long commandTimeoutSec) {
        this.commandService = commandService;
        this.gatewayService = gatewayService;
        this.mqttDispatchService = mqttDispatchService;
        this.commandTimeoutSec = commandTimeoutSec;
    }

    @GetMapping
    public Map<String, Object> listCommands(
            @RequestParam(required = false) String gatewayId,
            @RequestParam(required = false) String status) {
        List<CommandSummaryDto> items = commandService.listCommands(gatewayId, status).stream()
                .map(this::toSummaryDto)
                .collect(Collectors.toList());
        return Map.of("items", items, "total", items.size());
    }

    @PostMapping
    public Map<String, Object> createCommand(@Valid @RequestBody CreateCommandRequest request) {
        if (!SUPPORTED_TYPES.contains(request.type())) {
            throw new IllegalArgumentException("Unsupported command type: " + request.type());
        }
        TokenService.AuthContext auth = currentAuth();
        gatewayService.requireEnabledGateway(request.gatewayId());
        CommandEntity command = new CommandEntity();
        command.setCommandId("cmd-" + UUID.randomUUID().toString().substring(0, 8));
        command.setCorrelationId("corr-" + UUID.randomUUID().toString().substring(0, 8));
        command.setTenantId(auth.tenantId());
        command.setGatewayId(request.gatewayId());
        command.setType(request.type());
        command.setStatus(CommandStatus.QUEUED);
        command.setRequestPayload(request.payload());
        command.setRequestedAt(Instant.now());
        command.setTimeoutAt(Instant.now().plus(commandTimeoutSec, ChronoUnit.SECONDS));
        command.setInitiator(auth.username());
        CommandEntity saved = commandService.createCommand(command);
        try {
            mqttDispatchService.publishCommand(saved);
            return Map.of("commandId", saved.getCommandId(), "status", "sent");
        } catch (IllegalStateException e) {
            return Map.of("commandId", saved.getCommandId(), "status", "queued");
        }
    }

    private TokenService.AuthContext currentAuth() {
        var authentication = org.springframework.security.core.context.SecurityContextHolder.getContext()
                .getAuthentication();
        if (authentication != null && authentication.getPrincipal() instanceof TokenService.AuthContext context) {
            return context;
        }
        throw new IllegalStateException("No authenticated console user in request context");
    }

    @GetMapping("/{commandId}")
    public CommandDetailDto getCommand(@PathVariable String commandId) {
        return commandService.findCommand(commandId)
                .map(this::toDetailDto)
                .orElseThrow(() -> new IllegalArgumentException("Command not found: " + commandId));
    }

    private CommandSummaryDto toSummaryDto(CommandEntity command) {
        return new CommandSummaryDto(
                command.getCommandId(),
                command.getGatewayId(),
                command.getType(),
                command.getStatus().name().toLowerCase(),
                command.getInitiator(),
                command.getRequestedAt(),
                command.getFinishedAt(),
                command.getErrorCode(),
                command.getErrorMessage());
    }

    private CommandDetailDto toDetailDto(CommandEntity command) {
        return new CommandDetailDto(
                command.getCommandId(),
                command.getTenantId(),
                command.getGatewayId(),
                command.getType(),
                command.getStatus().name().toLowerCase(),
                command.getRequestPayload(),
                command.getReplyPayload(),
                command.getInitiator(),
                command.getCorrelationId(),
                command.getRequestedAt(),
                command.getSentAt(),
                command.getStartedAt(),
                command.getFinishedAt(),
                command.getTimeoutAt(),
                command.getErrorCode(),
                command.getErrorMessage(),
                commandService.listEvents(command.getCommandId()).stream()
                        .map(event -> new CommandEventDto(
                                event.getEventId(),
                                event.getEventType(),
                                event.getStatus(),
                                event.getMessage(),
                                event.getCreatedAt()))
                        .collect(Collectors.toList()));
    }
}
