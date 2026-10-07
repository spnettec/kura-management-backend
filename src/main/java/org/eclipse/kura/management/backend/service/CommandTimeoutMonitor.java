package org.eclipse.kura.management.backend.service;

import org.springframework.scheduling.annotation.Scheduled;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

@Service
public class CommandTimeoutMonitor {

    private final CommandService commandService;

    public CommandTimeoutMonitor(CommandService commandService) {
        this.commandService = commandService;
    }

    @Scheduled(fixedDelayString = "${management.command-timeout-check-delay-ms:5000}")
    @Transactional
    public void markTimedOutCommands() {
        commandService.markTimedOutCommands();
    }
}
