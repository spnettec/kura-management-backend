package org.eclipse.kura.management.backend.domain;

public enum CommandStatus {
    QUEUED,
    SENT,
    RUNNING,
    SUCCESS,
    FAILED,
    TIMEOUT,
    REJECTED
}
