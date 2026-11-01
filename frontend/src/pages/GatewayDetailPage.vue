<template>
  <main class="page-shell">
    <RefreshNotice :message="refreshError" />

    <a-card size="small" class="head-card">
      <a-skeleton v-if="loading && !gateway" active :paragraph="{ rows: 2 }" />
      <a-result v-else-if="error && !gateway" status="warning" :title="error" />
      <template v-else-if="gateway">
        <div class="head-row">
          <div>
            <a-space :size="10" align="center">
              <span class="head-title">{{ gateway.name }}</span>
              <StatusPill :status="gateway.status" />
              <a-tag>{{ gateway.gatewayId }}</a-tag>
            </a-space>
            <div class="head-sub">{{ gateway.kuraVersion ?? '-' }} · {{ gateway.ipAddrs.join(', ') || '-' }}</div>
          </div>
          <a-space>
            <a-switch
              :checked="gateway.enabled"
              :checked-children="t('common.enabled')"
              :un-checked-children="t('common.disabled')"
              :loading="savingGateway"
              @change="toggleGateway"
            />
            <a-divider type="vertical" />
            <a-button size="small" :loading="commandRunning" @click="runGatewayCommand('system.info')">
              {{ t('command.types.systemInfo') }}
            </a-button>
            <a-button size="small" :loading="commandRunning" @click="readFirewallRules">
              {{ t('command.types.firewallGet') }}
            </a-button>
            <a-button size="small" :loading="commandRunning" @click="readNetworkStatus">
              {{ t('command.types.networkGet') }}
            </a-button>
          </a-space>
        </div>
        <a-alert
          v-if="commandMessage"
          :message="commandMessage"
          type="info"
          show-icon
          closable
          style="margin-top: 12px"
        />
      </template>
    </a-card>

    <a-tabs v-if="gateway" v-model:active-key="activeTab" class="detail-tabs" size="small">
      <a-tab-pane key="overview" :tab="t('gateway.summary')">
        <div class="stat-row">
          <a-card size="small">
            <a-statistic title="CPU" :value="gateway.resources.cpu" suffix="%" :precision="1" :value-style="{ color: '#1677ff' }" />
          </a-card>
          <a-card size="small">
            <a-statistic title="MEM" :value="gateway.resources.mem" suffix="%" :precision="1" :value-style="{ color: '#722ed1' }" />
          </a-card>
          <a-card size="small">
            <a-statistic title="DISK" :value="gateway.resources.disk" suffix="%" :precision="1" :value-style="{ color: '#fa8c16' }" />
          </a-card>
          <a-card size="small">
            <a-statistic :title="t('gateway.fields.uptime')" :value="gateway.resources.uptimeSec ?? 0" suffix="s" />
          </a-card>
        </div>

        <a-card size="small" style="margin-top: 12px">
          <a-descriptions :column="2" size="small" bordered>
            <a-descriptions-item :label="t('gateway.fields.name')">{{ gateway.name }}</a-descriptions-item>
            <a-descriptions-item :label="t('gateway.fields.lastSeen')">{{ formatTime(gateway.lastSeenAt) }}</a-descriptions-item>
            <a-descriptions-item :label="t('gateway.fields.kura')">{{ gateway.kuraVersion ?? '-' }}</a-descriptions-item>
            <a-descriptions-item :label="t('gateway.fields.os')">{{ gateway.osVersion ?? '-' }}</a-descriptions-item>
            <a-descriptions-item :label="t('gateway.fields.ip')">{{ gateway.ipAddrs.join(', ') || '-' }}</a-descriptions-item>
            <a-descriptions-item :label="t('gateway.fields.mac')">{{ gateway.macAddrs.join(', ') || '-' }}</a-descriptions-item>
            <a-descriptions-item :label="t('gateway.fields.firewall')" :span="2">
              {{ translateStatus(gateway.firewallStatus.running ? 'running' : (gateway.firewallStatus.serviceState || (gateway.firewallStatus.enabled ? 'enabled' : 'disabled'))) }}
              <a-tag :color="gateway.firewallStatus.healthy ? 'success' : 'error'">
                {{ gateway.firewallStatus.healthy ? t('status.normal') : t('status.abnormal') }}
              </a-tag>
            </a-descriptions-item>
          </a-descriptions>
        </a-card>

        <a-card size="small" :title="t('gateway.services')" style="margin-top: 12px">
          <a-space wrap :size="8">
            <a-tag v-for="(value, key) in gateway.services" :key="key">
              {{ key }}:
              <template v-if="value === 'active' || value === 'running'"><a-badge status="success" :text="translateStatus(value)" /></template>
              <template v-else-if="value === 'failed'"><a-badge status="error" :text="translateStatus(value)" /></template>
              <template v-else><a-badge status="default" :text="translateStatus(value)" /></template>
            </a-tag>
          </a-space>
        </a-card>
      </a-tab-pane>

      <a-tab-pane key="config" :tab="t('gateway.config')">
        <ConfigPanel :gateway-id="gateway.gatewayId" />
      </a-tab-pane>

      <a-tab-pane key="snapshots" :tab="t('gateway.snapshots')">
        <SnapshotPanel :gateway-id="gateway.gatewayId" />
      </a-tab-pane>

      <a-tab-pane key="firewall" :tab="t('gateway.firewall')">
        <div class="panel-heading">
          <span class="panel-meta">
            <a-tag color="default">v4: {{ gateway.firewallStatus.ipv4RuleCount ?? 0 }}</a-tag>
            <a-tag color="default">v6: {{ gateway.firewallStatus.ipv6RuleCount ?? 0 }}</a-tag>
            <a-tag :color="gateway.firewallStatus.healthy ? 'success' : 'error'">
              {{ t('firewall.health') }}: {{ gateway.firewallStatus.healthy ? t('status.normal') : t('status.abnormal') }}
            </a-tag>
            <span v-if="firewallUpdatedAt">{{ t('gateway.firewallFromCommand', { time: formatTime(firewallUpdatedAt) }) }}</span>
            <RouterLink v-if="firewallCommandId" :to="`/commands/${firewallCommandId}`">
              {{ t('gateway.viewCommand') }}
            </RouterLink>
          </span>
          <a-button size="small" :loading="firewallLoading" @click="readFirewallRules">
            <template #icon><ReloadOutlined /></template>
            {{ t('command.types.firewallGet') }}
          </a-button>
        </div>
        <a-spin v-if="firewallLoading" style="display: block; padding: 32px" />
        <a-alert v-else-if="firewallError" :message="firewallError" type="error" show-icon />
        <FirewallRulesView v-else-if="firewallConfig" :config="firewallConfig" />
        <a-empty v-else :description="t('gateway.noFirewallData')" style="padding: 32px" />
      </a-tab-pane>

      <a-tab-pane key="network" :tab="t('gateway.network')">
        <div class="panel-heading">
          <span class="panel-meta">
            <span v-if="networkUpdatedAt">{{ t('gateway.networkFromCommand', { time: formatTime(networkUpdatedAt) }) }}</span>
            <RouterLink v-if="networkCommandId" :to="`/commands/${networkCommandId}`">
              {{ t('gateway.viewCommand') }}
            </RouterLink>
          </span>
          <a-button size="small" :loading="networkLoading" @click="readNetworkStatus">
            <template #icon><ReloadOutlined /></template>
            {{ t('command.types.networkGet') }}
          </a-button>
        </div>
        <a-spin v-if="networkLoading" style="display: block; padding: 32px" />
        <a-alert v-else-if="networkError" :message="networkError" type="error" show-icon />
        <NetworkStatusView v-else-if="networkConfig" :config="networkConfig" />
        <a-empty v-else :description="t('gateway.noNetworkData')" style="padding: 32px" />
      </a-tab-pane>

      <a-tab-pane key="credentials" :tab="t('gateway.credentials')">
        <a-alert v-if="credentialError" :message="credentialError" type="warning" show-icon style="margin-bottom: 12px" />
        <a-descriptions v-else-if="credential" :column="2" size="small" bordered>
          <a-descriptions-item :label="t('gateway.fields.username')">
            <a-tag>{{ credential.gatewayId }}</a-tag>
          </a-descriptions-item>
          <a-descriptions-item :label="t('gateway.fields.enabled')">
            {{ credential.enabled ? t('common.yes') : t('common.no') }}
          </a-descriptions-item>
          <a-descriptions-item :label="t('gateway.fields.createdAt')">{{ formatTime(credential.createdAt) }}</a-descriptions-item>
          <a-descriptions-item :label="t('gateway.fields.rotatedAt')">{{ formatTime(credential.rotatedAt) }}</a-descriptions-item>
          <a-descriptions-item :label="t('gateway.fields.lastUsedAt')" :span="2">
            {{ formatTime(credential.lastUsedAt) }}
          </a-descriptions-item>
        </a-descriptions>
        <a-space style="margin-top: 12px">
          <a-button type="primary" :loading="credentialRunning" @click="rotateCredential">
            {{ t('gateway.rotateCredential') }}
          </a-button>
          <a-button v-if="credential" :loading="credentialRunning" @click="toggleCredential">
            {{ credential.enabled ? t('gateway.disableCredential') : t('gateway.enableCredential') }}
          </a-button>
        </a-space>
        <a-alert
          v-if="credentialSecret"
          type="success"
          show-icon
          style="margin-top: 12px"
          :message="`${t('gateway.fields.secret')}: ${credentialSecret.secret}`"
          :description="t('gateway.secretHint')"
        />
      </a-tab-pane>

      <a-tab-pane key="heartbeats" :tab="t('gateway.heartbeats')">
        <a-empty v-if="heartbeats.length === 0" :description="t('gateway.noHeartbeats')" />
        <template v-else>
          <ResourceTrend :heartbeats="heartbeats" />
          <a-descriptions :column="4" size="small" bordered>
            <a-descriptions-item :label="t('gateway.fields.reportedAt')">{{ formatTime(heartbeats[0].ts) }}</a-descriptions-item>
            <a-descriptions-item :label="t('gateway.fields.status')">
              <StatusPill :status="heartbeats[0].online ? 'online' : 'offline'" />
            </a-descriptions-item>
            <a-descriptions-item :label="t('gateway.fields.ip')">{{ heartbeats[0].ipAddrs.join(', ') || '-' }}</a-descriptions-item>
            <a-descriptions-item :label="t('gateway.fields.uptime')">{{ heartbeats[0].uptimeSec ?? 0 }}s</a-descriptions-item>
          </a-descriptions>
          <pre>{{ formatJson(heartbeats[0].rawPayload) }}</pre>
        </template>
      </a-tab-pane>

      <a-tab-pane key="commands" :tab="t('gateway.commands')">
        <a-empty v-if="commands.length === 0" :description="t('gateway.noCommands')" />
        <a-table
          v-else
          :data-source="commands"
          :columns="commandColumns"
          :pagination="{ pageSize: 10, hideOnSinglePage: true }"
          row-key="commandId"
          size="small"
        >
          <template #bodyCell="{ column, record }">
            <template v-if="column.key === 'commandId'">
              <RouterLink :to="`/commands/${record.commandId}`">{{ record.commandId }}</RouterLink>
            </template>
            <template v-else-if="column.key === 'status'">
              <a-tag :color="commandStatusColor(record.status)">{{ t(`status.${record.status}`) }}</a-tag>
            </template>
            <template v-else-if="column.key === 'requestedAt'">
              {{ formatTime(record.requestedAt) }}
            </template>
          </template>
        </a-table>
      </a-tab-pane>
    </a-tabs>
  </main>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, watch } from 'vue'
import { RouterLink, useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { ReloadOutlined } from '@ant-design/icons-vue'
import StatusPill from '@/components/StatusPill.vue'
import FirewallRulesView, { type FirewallConfigPayload } from '@/components/FirewallRulesView.vue'
import RefreshNotice from '@/components/RefreshNotice.vue'
import NetworkStatusView, { type NetworkConfigPayload } from '@/components/NetworkStatusView.vue'
import ConfigPanel from '@/components/ConfigPanel.vue'
import SnapshotPanel from '@/components/SnapshotPanel.vue'
import ResourceTrend from '@/components/ResourceTrend.vue'
import { createCommand, fetchCommand, fetchCommands, type CommandListItem } from '@/api/commands'
import {
  fetchGateway,
  fetchGatewayCredential,
  fetchGatewayHeartbeats,
  rotateGatewayCredential,
  setGatewayCredentialEnabled,
  setGatewayEnabled,
  type GatewayCredential,
  type GatewayCredentialSecret,
  type GatewayDetailItem,
  type GatewayHeartbeatItem,
} from '@/api/gateways'

const props = defineProps<{
  gatewayId?: string
}>()

const route = useRoute()
const { t, locale } = useI18n()
const gateway = ref<GatewayDetailItem | null>(null)
const heartbeats = ref<GatewayHeartbeatItem[]>([])
const credential = ref<GatewayCredential | null>(null)
const credentialSecret = ref<GatewayCredentialSecret | null>(null)
const commands = ref<CommandListItem[]>([])
const loading = ref(false)
const error = ref<string | null>(null)
const refreshError = ref<string | null>(null)
const credentialError = ref<string | null>(null)
const commandRunning = ref(false)
const commandMessage = ref<string | null>(null)
const firewallLoading = ref(false)
const firewallError = ref<string | null>(null)
const firewallConfig = ref<FirewallConfigPayload | null>(null)
const firewallCommandId = ref<string | null>(null)
const firewallUpdatedAt = ref<string | number | null>(null)
const networkLoading = ref(false)
const networkError = ref<string | null>(null)
const networkConfig = ref<NetworkConfigPayload | null>(null)
const networkCommandId = ref<string | null>(null)
const networkUpdatedAt = ref<string | number | null>(null)
const credentialRunning = ref(false)
const savingGateway = ref(false)
const activeTab = ref('overview')
let refreshTimer: number | undefined
let loadedFirewallCommandId: string | null = null
let loadedNetworkCommandId: string | null = null

const resolvedGatewayId = computed(() => String(props.gatewayId ?? route.params.gatewayId ?? ''))

const commandColumns = computed(() => [
  { title: t('command.fields.commandId'), key: 'commandId' },
  { title: t('command.fields.type'), dataIndex: 'type', key: 'type' },
  { title: t('command.fields.status'), key: 'status', width: 110 },
  { title: t('command.fields.requested'), key: 'requestedAt', width: 170 },
])

function commandStatusColor(status: string) {
  if (status === 'success') return 'success'
  if (status === 'failed' || status === 'timeout' || status === 'rejected') return 'error'
  if (status === 'running' || status === 'sent') return 'processing'
  return 'default'
}

function formatTime(value?: string | number | null) {
  if (!value) {
    return '-'
  }
  return new Date(value).toLocaleString(locale.value)
}

function translateStatus(value: string) {
  return t(`status.${value}`, value)
}

function formatJson(value: unknown) {
  return JSON.stringify(value, null, 2)
}

async function runGatewayCommand(type: 'system.info' | 'firewall.get' | 'network.get') {
  const gatewayId = resolvedGatewayId.value
  if (!gatewayId) {
    commandMessage.value = t('gateway.missingId')
    return
  }

  commandRunning.value = true
  commandMessage.value = null
  try {
    await createCommand({
      gatewayId,
      type,
      payload: {},
    })
    commandMessage.value = t('command.created')
    await loadGateway({ silent: true })
    window.setTimeout(() => {
      void loadGateway({ silent: true })
    }, 1000)
  } catch (err) {
    commandMessage.value = err instanceof Error ? err.message : t('command.createFailed')
  } finally {
    commandRunning.value = false
  }
}

async function readNetworkStatus() {
  const gatewayId = resolvedGatewayId.value
  if (!gatewayId) {
    networkError.value = t('gateway.missingId')
    return
  }

  commandRunning.value = true
  networkLoading.value = true
  networkError.value = null
  commandMessage.value = null
  try {
    const created = await createCommand({
      gatewayId,
      type: 'network.get',
      payload: {},
    })
    networkCommandId.value = created.commandId
    commandMessage.value = t('command.created')
    const detail = await waitForCommandReply(created.commandId)
    if (detail.status !== 'success') {
      throw new Error(detail.errorMessage || t('gateway.networkFailed'))
    }
    const payload = detail.replyPayload?.payload
    if (!isNetworkConfigPayload(payload)) {
      throw new Error(t('gateway.invalidNetworkReply'))
    }
    applyNetworkConfig(payload, detail.commandId, detail.finishedAt ?? detail.requestedAt)
    await loadGateway({ silent: true })
  } catch (err) {
    networkError.value = err instanceof Error ? err.message : t('gateway.networkFailed')
  } finally {
    commandRunning.value = false
    networkLoading.value = false
  }
}

async function readFirewallRules() {
  const gatewayId = resolvedGatewayId.value
  if (!gatewayId) {
    firewallError.value = t('gateway.missingId')
    return
  }

  commandRunning.value = true
  firewallLoading.value = true
  firewallError.value = null
  commandMessage.value = null
  try {
    const created = await createCommand({
      gatewayId,
      type: 'firewall.get',
      payload: {},
    })
    firewallCommandId.value = created.commandId
    commandMessage.value = t('command.created')
    const detail = await waitForCommandReply(created.commandId)
    if (detail.status !== 'success') {
      throw new Error(detail.errorMessage || t('gateway.firewallFailed'))
    }
    const payload = detail.replyPayload?.payload
    if (!isFirewallConfigPayload(payload)) {
      throw new Error(t('gateway.invalidFirewallReply'))
    }
    applyFirewallConfig(payload, detail.commandId, detail.finishedAt ?? detail.requestedAt)
    await loadGateway({ silent: true })
  } catch (err) {
    firewallError.value = err instanceof Error ? err.message : t('gateway.firewallFailed')
  } finally {
    commandRunning.value = false
    firewallLoading.value = false
  }
}

function applyFirewallConfig(payload: FirewallConfigPayload, commandId: string, updatedAt?: string | number | null) {
  firewallConfig.value = payload
  firewallCommandId.value = commandId
  firewallUpdatedAt.value = updatedAt ?? null
  loadedFirewallCommandId = commandId
}

function resetFirewallState() {
  firewallLoading.value = false
  firewallError.value = null
  firewallConfig.value = null
  firewallCommandId.value = null
  firewallUpdatedAt.value = null
  loadedFirewallCommandId = null
}

function applyNetworkConfig(payload: NetworkConfigPayload, commandId: string, updatedAt?: string | number | null) {
  networkConfig.value = payload
  networkCommandId.value = commandId
  networkUpdatedAt.value = updatedAt ?? null
  loadedNetworkCommandId = commandId
}

function resetNetworkState() {
  networkLoading.value = false
  networkError.value = null
  networkConfig.value = null
  networkCommandId.value = null
  networkUpdatedAt.value = null
  loadedNetworkCommandId = null
}

async function loadLatestFirewallFromCommands(commandItems: CommandListItem[]) {
  const latest = commandItems.find((command) => command.type === 'firewall.get' && command.status === 'success')
  if (!latest || latest.commandId === loadedFirewallCommandId) {
    return
  }

  const detail = await fetchCommand(latest.commandId)
  const payload = detail.replyPayload?.payload
  if (detail.status === 'success' && isFirewallConfigPayload(payload)) {
    applyFirewallConfig(payload, detail.commandId, detail.finishedAt ?? detail.requestedAt)
  }
}

async function loadLatestNetworkFromCommands(commandItems: CommandListItem[]) {
  const latest = commandItems.find((command) => command.type === 'network.get' && command.status === 'success')
  if (!latest || latest.commandId === loadedNetworkCommandId) {
    return
  }

  const detail = await fetchCommand(latest.commandId)
  const payload = detail.replyPayload?.payload
  if (detail.status === 'success' && isNetworkConfigPayload(payload)) {
    applyNetworkConfig(payload, detail.commandId, detail.finishedAt ?? detail.requestedAt)
  }
}

async function waitForCommandReply(commandId: string) {
  for (let attempt = 0; attempt < 8; attempt += 1) {
    const detail = await fetchCommand(commandId)
    if (detail.status === 'success' || detail.status === 'failed' || detail.status === 'timeout') {
      return detail
    }
    await delay(500)
  }
  throw new Error(t('gateway.firewallTimeout'))
}

function delay(ms: number) {
  return new Promise((resolve) => window.setTimeout(resolve, ms))
}

function isFirewallConfigPayload(value: unknown): value is FirewallConfigPayload {
  if (!value || typeof value !== 'object') {
    return false
  }
  const candidate = value as Partial<FirewallConfigPayload>
  return typeof candidate.enabled === 'boolean'
    && typeof candidate.packageInstalled === 'boolean'
    && Array.isArray(candidate.parsedRules)
}

function isNetworkConfigPayload(value: unknown): value is NetworkConfigPayload {
  if (!value || typeof value !== 'object') {
    return false
  }
  const candidate = value as Partial<NetworkConfigPayload>
  return Array.isArray(candidate.interfaces)
    && Array.isArray(candidate.routes)
    && Array.isArray(candidate.dnsServers)
}

async function loadCredential() {
  const gatewayId = resolvedGatewayId.value
  if (!gatewayId) {
    return
  }
  credentialError.value = null
  try {
    credential.value = await fetchGatewayCredential(gatewayId)
  } catch (err) {
    credential.value = null
    credentialError.value = err instanceof Error ? err.message : t('gateway.noCredential')
  }
}

async function rotateCredential() {
  credentialRunning.value = true
  credentialError.value = null
  try {
    credentialSecret.value = await rotateGatewayCredential(resolvedGatewayId.value)
    credential.value = credentialSecret.value
  } catch (err) {
    credentialError.value = err instanceof Error ? err.message : t('gateway.credentialFailed')
  } finally {
    credentialRunning.value = false
  }
}

async function toggleCredential() {
  if (!credential.value) {
    return
  }
  credentialRunning.value = true
  credentialError.value = null
  try {
    credential.value = await setGatewayCredentialEnabled(resolvedGatewayId.value, !credential.value.enabled)
  } catch (err) {
    credentialError.value = err instanceof Error ? err.message : t('gateway.credentialFailed')
  } finally {
    credentialRunning.value = false
  }
}

async function toggleGateway() {
  if (!gateway.value) {
    return
  }
  savingGateway.value = true
  try {
    gateway.value = await setGatewayEnabled(gateway.value.gatewayId, !gateway.value.enabled)
  } catch (err) {
    error.value = err instanceof Error ? err.message : t('gateway.failedDetail')
  } finally {
    savingGateway.value = false
  }
}

async function loadGateway(options: { silent?: boolean } = {}) {
  const gatewayId = resolvedGatewayId.value
  if (!gatewayId) {
    error.value = t('gateway.missingId')
    return
  }

  if (!options.silent) {
    loading.value = true
    error.value = null
  }
  try {
    const [gatewayResponse, heartbeatResponse, commandResponse] = await Promise.all([
      fetchGateway(gatewayId),
      fetchGatewayHeartbeats(gatewayId),
      fetchCommands({ gatewayId }),
    ])
    gateway.value = gatewayResponse
    heartbeats.value = heartbeatResponse.items
    commands.value = commandResponse.items
    refreshError.value = null
    await Promise.all([
      loadCredential(),
      loadLatestFirewallFromCommands(commandResponse.items),
      loadLatestNetworkFromCommands(commandResponse.items),
    ])
  } catch (err) {
    const message = err instanceof Error ? err.message : t('gateway.failedDetail')
    if (!options.silent || !gateway.value) {
      error.value = message
    } else {
      refreshError.value = message
    }
  } finally {
    if (!options.silent) {
      loading.value = false
    }
  }
}

onMounted(() => {
  void loadGateway()
  refreshTimer = window.setInterval(() => {
    void loadGateway({ silent: true })
  }, 15000)
})

onUnmounted(() => {
  if (refreshTimer !== undefined) {
    window.clearInterval(refreshTimer)
  }
})

watch(resolvedGatewayId, () => {
  resetFirewallState()
  resetNetworkState()
  void loadGateway()
})
</script>

<style scoped>
.head-card {
  border-radius: 8px;
}

.head-row {
  align-items: center;
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
  justify-content: space-between;
}

.head-title {
  font-size: 18px;
  font-weight: 600;
}

.head-sub {
  color: #6b7280;
  font-size: 13px;
  margin-top: 6px;
}

.detail-tabs {
  background: transparent;
  margin-top: 16px;
}

.detail-tabs :deep(.ant-tabs-content-holder) {
  background: #fff;
  border: 1px solid #f0f0f0;
  border-radius: 8px;
  padding: 16px;
}

.stat-row {
  display: grid;
  gap: 12px;
  grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
}

.panel-heading {
  align-items: center;
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
  justify-content: space-between;
  margin-bottom: 12px;
}

.panel-meta {
  align-items: center;
  color: #6b7280;
  display: flex;
  flex-wrap: wrap;
  font-size: 13px;
  gap: 8px;
}

pre {
  background: #0f172a;
  border-radius: 6px;
  color: #e5e7eb;
  margin: 16px 0 0;
  max-height: 320px;
  overflow: auto;
  padding: 12px;
  white-space: pre-wrap;
  word-break: break-word;
}
</style>
