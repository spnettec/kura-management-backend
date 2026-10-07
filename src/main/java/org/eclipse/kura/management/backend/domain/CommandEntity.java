package org.eclipse.kura.management.backend.domain;

import java.time.Instant;
import java.util.HashMap;
import java.util.Map;

import org.hibernate.annotations.JdbcTypeCode;
import org.hibernate.type.SqlTypes;
import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.EnumType;
import jakarta.persistence.Enumerated;
import jakarta.persistence.Id;
import jakarta.persistence.Table;

@Entity
@Table(name = "commands")
public class CommandEntity {

    @Id
    @Column(name = "command_id", nullable = false, length = 64)
    private String commandId;

    @Column(name = "tenant_id", nullable = false, length = 64)
    private String tenantId;

    @Column(name = "gateway_id", nullable = false, length = 64)
    private String gatewayId;

    @Column(name = "type", nullable = false, length = 64)
    private String type;

    @Enumerated(EnumType.STRING)
    @Column(name = "status", nullable = false)
    private CommandStatus status = CommandStatus.QUEUED;

    @JdbcTypeCode(SqlTypes.JSON)
    @Column(name = "request_payload", columnDefinition = "jsonb", nullable = false)
    private Map<String, Object> requestPayload = new HashMap<>();

    @JdbcTypeCode(SqlTypes.JSON)
    @Column(name = "reply_payload", columnDefinition = "jsonb")
    private Map<String, Object> replyPayload;

    @Column(name = "initiator")
    private String initiator;

    @Column(name = "correlation_id")
    private String correlationId;

    @Column(name = "requested_at", nullable = false)
    private Instant requestedAt = Instant.now();

    @Column(name = "sent_at")
    private Instant sentAt;

    @Column(name = "started_at")
    private Instant startedAt;

    @Column(name = "finished_at")
    private Instant finishedAt;

    @Column(name = "timeout_at")
    private Instant timeoutAt;

    @Column(name = "error_code")
    private String errorCode;

    @Column(name = "error_message")
    private String errorMessage;

    public String getCommandId() {
        return commandId;
    }

    public void setCommandId(String commandId) {
        this.commandId = commandId;
    }

    public String getTenantId() {
        return tenantId;
    }

    public void setTenantId(String tenantId) {
        this.tenantId = tenantId;
    }

    public String getGatewayId() {
        return gatewayId;
    }

    public void setGatewayId(String gatewayId) {
        this.gatewayId = gatewayId;
    }

    public String getType() {
        return type;
    }

    public void setType(String type) {
        this.type = type;
    }

    public CommandStatus getStatus() {
        return status;
    }

    public void setStatus(CommandStatus status) {
        this.status = status;
    }

    public Map<String, Object> getRequestPayload() {
        return requestPayload;
    }

    public void setRequestPayload(Map<String, Object> requestPayload) {
        this.requestPayload = requestPayload;
    }

    public Map<String, Object> getReplyPayload() {
        return replyPayload;
    }

    public void setReplyPayload(Map<String, Object> replyPayload) {
        this.replyPayload = replyPayload;
    }

    public String getInitiator() {
        return initiator;
    }

    public void setInitiator(String initiator) {
        this.initiator = initiator;
    }

    public String getCorrelationId() {
        return correlationId;
    }

    public void setCorrelationId(String correlationId) {
        this.correlationId = correlationId;
    }

    public Instant getRequestedAt() {
        return requestedAt;
    }

    public void setRequestedAt(Instant requestedAt) {
        this.requestedAt = requestedAt;
    }

    public Instant getSentAt() {
        return sentAt;
    }

    public void setSentAt(Instant sentAt) {
        this.sentAt = sentAt;
    }

    public Instant getStartedAt() {
        return startedAt;
    }

    public void setStartedAt(Instant startedAt) {
        this.startedAt = startedAt;
    }

    public Instant getFinishedAt() {
        return finishedAt;
    }

    public void setFinishedAt(Instant finishedAt) {
        this.finishedAt = finishedAt;
    }

    public Instant getTimeoutAt() {
        return timeoutAt;
    }

    public void setTimeoutAt(Instant timeoutAt) {
        this.timeoutAt = timeoutAt;
    }

    public String getErrorCode() {
        return errorCode;
    }

    public void setErrorCode(String errorCode) {
        this.errorCode = errorCode;
    }

    public String getErrorMessage() {
        return errorMessage;
    }

    public void setErrorMessage(String errorMessage) {
        this.errorMessage = errorMessage;
    }
}
