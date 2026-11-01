import { request } from './http'
import type { GatewayStatus, ListResponse } from '@/types/common'

export type ApiTimestamp = string | number

export interface GatewayListItem {
  gatewayId: string
  tenantId: string
  name: string
  groupId?: string | null
  status: GatewayStatus
  enabled: boolean
  lastSeenAt?: ApiTimestamp | null
  kuraVersion?: string | null
  osVersion?: string | null
  ipAddrs: string[]
  configRevision?: number | null
  snapshotId?: number | null
  alertCount: number
  tags: string[]
}

export interface GatewayDetailItem extends GatewayListItem {
  macAddrs: string[]
  services: Record<string, string>
  firewallStatus: {
    enabled?: boolean
    running?: boolean
    healthy?: boolean
    packageInstalled?: boolean
    serviceState?: string
    systemdState?: string
    healthMessage?: string
    ipv4RuleCount?: number
    ipv6RuleCount?: number
    collectedAt?: ApiTimestamp
    source?: string
    errorMessage?: string
  }
  resources: {
    cpu: number
    mem: number
    disk: number
    uptimeSec: number
  }
}

export interface GatewayHeartbeatItem {
  heartbeatId: number
  tenantId: string
  gatewayId: string
  ts: ApiTimestamp
  online: boolean
  kuraVersion?: string | null
  osVersion?: string | null
  ipAddrs: string[]
  uptimeSec?: number | null
  cpuPct?: number | null
  memPct?: number | null
  diskPct?: number | null
  services: Record<string, unknown>
  rawPayload: Record<string, unknown>
}

export interface GatewaySummary {
  total: number
  online: number
  offline: number
  unknown: number
}

export interface CreateGatewayPayload {
  gatewayId: string
  tenantId?: string
  name?: string
  groupId?: string
}

export interface GatewayCredential {
  credentialId: number
  gatewayId: string
  tenantId: string
  enabled: boolean
  createdAt: ApiTimestamp
  rotatedAt?: ApiTimestamp | null
  lastUsedAt?: ApiTimestamp | null
}

export interface GatewayCredentialSecret extends GatewayCredential {
  username: string
  secret: string
}

export function fetchGateways(params?: { status?: GatewayStatus | '' }) {
  const searchParams = new URLSearchParams()
  if (params?.status) {
    searchParams.set('status', params.status)
  }
  const query = searchParams.toString()
  return request<ListResponse<GatewayListItem>>(`/api/gateways${query ? `?${query}` : ''}`)
}

export function fetchGatewaySummary() {
  return request<GatewaySummary>('/api/gateways/summary')
}

export function fetchGateway(gatewayId: string) {
  return request<GatewayDetailItem>(`/api/gateways/${gatewayId}`)
}

export function createGateway(payload: CreateGatewayPayload) {
  return request<GatewayListItem>('/api/gateways', {
    method: 'POST',
    body: JSON.stringify(payload),
  })
}

export function setGatewayEnabled(gatewayId: string, enabled: boolean) {
  return request<GatewayDetailItem>(`/api/gateways/${gatewayId}`, {
    method: 'PATCH',
    body: JSON.stringify({ enabled }),
  })
}

export function fetchGatewayCredential(gatewayId: string) {
  return request<GatewayCredential>(`/api/gateways/${gatewayId}/credentials`)
}

export function rotateGatewayCredential(gatewayId: string) {
  return request<GatewayCredentialSecret>(`/api/gateways/${gatewayId}/credentials`, {
    method: 'POST',
  })
}

export function setGatewayCredentialEnabled(gatewayId: string, enabled: boolean) {
  return request<GatewayCredential>(`/api/gateways/${gatewayId}/credentials`, {
    method: 'PATCH',
    body: JSON.stringify({ enabled }),
  })
}

export function fetchGatewayHeartbeats(gatewayId: string) {
  return request<ListResponse<GatewayHeartbeatItem>>(`/api/gateways/${gatewayId}/heartbeats`)
}
