export type Id = string

export interface ListResponse<T> {
  items: T[]
  total: number
}

export type GatewayStatus = 'online' | 'offline' | 'unknown'
export type CommandStatus = 'queued' | 'sent' | 'running' | 'success' | 'failed' | 'timeout' | 'rejected'
