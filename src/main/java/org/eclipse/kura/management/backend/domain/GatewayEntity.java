package org.eclipse.kura.management.backend.domain;

import java.time.Instant;
import java.util.ArrayList;
import java.util.List;

import org.hibernate.annotations.JdbcTypeCode;
import org.hibernate.type.SqlTypes;
import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.EnumType;
import jakarta.persistence.Enumerated;
import jakarta.persistence.Id;
import jakarta.persistence.Table;

@Entity
@Table(name = "gateways")
public class GatewayEntity {

    @Id
    @Column(name = "gateway_id", nullable = false, length = 64)
    private String gatewayId;

    @Column(name = "tenant_id", nullable = false, length = 64)
    private String tenantId;

    @Column(name = "group_id", length = 64)
    private String groupId;

    @Column(name = "name", nullable = false)
    private String name;

    @Enumerated(EnumType.STRING)
    @Column(name = "status", nullable = false)
    private GatewayStatus status = GatewayStatus.UNKNOWN;

    @Column(name = "enabled", nullable = false)
    private boolean enabled = true;

    @Column(name = "last_seen_at")
    private Instant lastSeenAt;

    @Column(name = "kura_version")
    private String kuraVersion;

    @Column(name = "os_version")
    private String osVersion;

    @JdbcTypeCode(SqlTypes.JSON)
    @Column(name = "ip_addrs", columnDefinition = "jsonb", nullable = false)
    private List<String> ipAddrs = new ArrayList<>();

    @JdbcTypeCode(SqlTypes.JSON)
    @Column(name = "mac_addrs", columnDefinition = "jsonb", nullable = false)
    private List<String> macAddrs = new ArrayList<>();

    @Column(name = "config_revision")
    private Long configRevision;

    @Column(name = "snapshot_id")
    private Long snapshotId;

    @Column(name = "alert_count", nullable = false)
    private int alertCount;

    public String getGatewayId() {
        return gatewayId;
    }

    public void setGatewayId(String gatewayId) {
        this.gatewayId = gatewayId;
    }

    public String getTenantId() {
        return tenantId;
    }

    public void setTenantId(String tenantId) {
        this.tenantId = tenantId;
    }

    public String getGroupId() {
        return groupId;
    }

    public void setGroupId(String groupId) {
        this.groupId = groupId;
    }

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public GatewayStatus getStatus() {
        return status;
    }

    public void setStatus(GatewayStatus status) {
        this.status = status;
    }

    public boolean isEnabled() {
        return enabled;
    }

    public void setEnabled(boolean enabled) {
        this.enabled = enabled;
    }

    public Instant getLastSeenAt() {
        return lastSeenAt;
    }

    public void setLastSeenAt(Instant lastSeenAt) {
        this.lastSeenAt = lastSeenAt;
    }

    public String getKuraVersion() {
        return kuraVersion;
    }

    public void setKuraVersion(String kuraVersion) {
        this.kuraVersion = kuraVersion;
    }

    public String getOsVersion() {
        return osVersion;
    }

    public void setOsVersion(String osVersion) {
        this.osVersion = osVersion;
    }

    public List<String> getIpAddrs() {
        return ipAddrs;
    }

    public void setIpAddrs(List<String> ipAddrs) {
        this.ipAddrs = ipAddrs;
    }

    public List<String> getMacAddrs() {
        return macAddrs;
    }

    public void setMacAddrs(List<String> macAddrs) {
        this.macAddrs = macAddrs;
    }

    public Long getConfigRevision() {
        return configRevision;
    }

    public void setConfigRevision(Long configRevision) {
        this.configRevision = configRevision;
    }

    public Long getSnapshotId() {
        return snapshotId;
    }

    public void setSnapshotId(Long snapshotId) {
        this.snapshotId = snapshotId;
    }

    public int getAlertCount() {
        return alertCount;
    }

    public void setAlertCount(int alertCount) {
        this.alertCount = alertCount;
    }
}
