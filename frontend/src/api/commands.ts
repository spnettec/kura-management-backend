import { request } from './http'
import type { CommandStatus, ListResponse } from '@/types/common'

export type ApiTimestamp = string | number

export interface CommandListItem {
  commandId: string
  gatewayId: string
  type: string
  status: CommandStatus
  initiator?: string | null
  requestedAt: ApiTimestamp
  finishedAt?: ApiTimestamp | null
  errorCode?: string | null
  errorMessage?: string | null
}

export interface CommandDetailItem extends CommandListItem {
  tenantId: string
  requestPayload: Record<string, unknown>
  replyPayload?: Record<string, unknown> | null
  correlationId?: string | null
  sentAt?: ApiTimestamp | null
  startedAt?: ApiTimestamp | null
  timeoutAt?: ApiTimestamp | null
  events: CommandEventItem[]
}

export interface CommandEventItem {
  eventId: number
  eventType: string
  status?: CommandStatus | null
  message?: string | null
  createdAt: ApiTimestamp
}

export function fetchCommands(params?: { gatewayId?: string; status?: CommandStatus | '' }) {
  const searchParams = new URLSearchParams()
  if (params?.gatewayId) {
    searchParams.set('gatewayId', params.gatewayId)
  }
  if (params?.status) {
    searchParams.set('status', params.status)
  }
  const query = searchParams.toString()
  return request<ListResponse<CommandListItem>>(`/api/commands${query ? `?${query}` : ''}`)
}

export function fetchCommand(commandId: string) {
  return request<CommandDetailItem>(`/api/commands/${commandId}`)
}

export function createCommand(payload: {
  gatewayId: string
  type: string
  payload: Record<string, unknown>
}) {
  return request<{ commandId: string; status: string }>('/api/commands', {
    method: 'POST',
    body: JSON.stringify(payload),
  })
}

const PENDING_STATUSES = new Set(['queued', 'sent', 'running'])

/**
 * Sends a command and polls until the gateway reply settles the status
 * (success/failed/timeout/rejected) or the timeout elapses.
 */
export async function createCommandAndWait(
  payload: { gatewayId: string; type: string; payload: Record<string, unknown> },
  timeoutMs = 15000,
): Promise<CommandDetailItem> {
  const created = await createCommand(payload)
  const deadline = Date.now() + timeoutMs
  while (Date.now() < deadline) {
    await new Promise((resolve) => setTimeout(resolve, 500))
    const detail = await fetchCommand(created.commandId)
    if (!PENDING_STATUSES.has(detail.status)) {
      return detail
    }
  }
  throw new Error(`Command ${created.commandId} did not finish in time`)
}
