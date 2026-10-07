package org.eclipse.kura.management.backend.domain;

import java.time.Instant;

import org.hibernate.annotations.JdbcTypeCode;
import org.hibernate.type.SqlTypes;

import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;
import jakarta.persistence.Table;
import jakarta.persistence.UniqueConstraint;

@Entity
@Table(name = "config_snapshots", uniqueConstraints = @UniqueConstraint(columnNames = { "gatewayId",
        "gatewaySnapshotId" }))
public class ConfigSnapshotEntity {

    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Long snapshotPk;

    private String tenantId;

    private String gatewayId;

    private long gatewaySnapshotId;

    private long revision;

    @JdbcTypeCode(SqlTypes.JSON)
    @Column(name = "payload", columnDefinition = "jsonb", nullable = false)
    private java.util.Map<String, Object> payload = new java.util.HashMap<>();

    private Instant createdAt;

    private String createdBy;

    private String sourceCommandId;

    public Long getSnapshotPk() {
        return snapshotPk;
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

    public long getGatewaySnapshotId() {
        return gatewaySnapshotId;
    }

    public void setGatewaySnapshotId(long gatewaySnapshotId) {
        this.gatewaySnapshotId = gatewaySnapshotId;
    }

    public long getRevision() {
        return revision;
    }

    public void setRevision(long revision) {
        this.revision = revision;
    }

    public java.util.Map<String, Object> getPayload() {
        return payload;
    }

    public void setPayload(java.util.Map<String, Object> payload) {
        this.payload = payload;
    }

    public Instant getCreatedAt() {
        return createdAt;
    }

    public void setCreatedAt(Instant createdAt) {
        this.createdAt = createdAt;
    }

    public String getCreatedBy() {
        return createdBy;
    }

    public void setCreatedBy(String createdBy) {
        this.createdBy = createdBy;
    }

    public String getSourceCommandId() {
        return sourceCommandId;
    }

    public void setSourceCommandId(String sourceCommandId) {
        this.sourceCommandId = sourceCommandId;
    }
}
