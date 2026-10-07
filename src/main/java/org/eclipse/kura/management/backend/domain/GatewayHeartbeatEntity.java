package org.eclipse.kura.management.backend.domain;

import java.time.Instant;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

import org.hibernate.annotations.JdbcTypeCode;
import org.hibernate.type.SqlTypes;
import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;
import jakarta.persistence.Table;

@Entity
@Table(name = "gateway_heartbeats")
public class GatewayHeartbeatEntity {

    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(name = "heartbeat_id")
    private Long heartbeatId;

    @Column(name = "tenant_id", nullable = false, length = 64)
    private String tenantId;

    @Column(name = "gateway_id", nullable = false, length = 64)
    private String gatewayId;

    @Column(name = "ts", nullable = false)
    private Instant ts;

    @Column(name = "online", nullable = false)
    private boolean online;

    @Column(name = "kura_version")
    private String kuraVersion;

    @Column(name = "os_version")
    private String osVersion;

    @JdbcTypeCode(SqlTypes.JSON)
    @Column(name = "ip_addrs", columnDefinition = "jsonb", nullable = false)
    private List<String> ipAddrs = new ArrayList<>();

    @Column(name = "uptime_sec")
    private Long uptimeSec;

    @Column(name = "cpu_pct")
    private Double cpuPct;

    @Column(name = "mem_pct")
    private Double memPct;

    @Column(name = "disk_pct")
    private Double diskPct;

    @JdbcTypeCode(SqlTypes.JSON)
    @Column(name = "services", columnDefinition = "jsonb", nullable = false)
    private Map<String, Object> services = new HashMap<>();

    @JdbcTypeCode(SqlTypes.JSON)
    @Column(name = "raw_payload", columnDefinition = "jsonb", nullable = false)
    private Map<String, Object> rawPayload = new HashMap<>();

    public Long getHeartbeatId() {
        return heartbeatId;
    }

    public void setHeartbeatId(Long heartbeatId) {
        this.heartbeatId = heartbeatId;
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

    public Instant getTs() {
        return ts;
    }

    public void setTs(Instant ts) {
        this.ts = ts;
    }

    public boolean isOnline() {
        return online;
    }

    public void setOnline(boolean online) {
        this.online = online;
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

    public Long getUptimeSec() {
        return uptimeSec;
    }

    public void setUptimeSec(Long uptimeSec) {
        this.uptimeSec = uptimeSec;
    }

    public Double getCpuPct() {
        return cpuPct;
    }

    public void setCpuPct(Double cpuPct) {
        this.cpuPct = cpuPct;
    }

    public Double getMemPct() {
        return memPct;
    }

    public void setMemPct(Double memPct) {
        this.memPct = memPct;
    }

    public Double getDiskPct() {
        return diskPct;
    }

    public void setDiskPct(Double diskPct) {
        this.diskPct = diskPct;
    }

    public Map<String, Object> getServices() {
        return services;
    }

    public void setServices(Map<String, Object> services) {
        this.services = services;
    }

    public Map<String, Object> getRawPayload() {
        return rawPayload;
    }

    public void setRawPayload(Map<String, Object> rawPayload) {
        this.rawPayload = rawPayload;
    }
}
