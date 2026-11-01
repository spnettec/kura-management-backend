<template>
  <main class="page-shell">
    <RefreshNotice :message="refreshError" />

    <a-card size="small" class="table-card">
      <template #title>{{ t('command.title') }}</template>
      <template #extra>
        <a-space>
          <a-segmented v-model:value="timeRange" :options="timeRangeOptions" size="small" />
          <a-input-search
            v-model:value="gatewayFilter"
            :placeholder="t('command.filters.anyGateway')"
            style="width: 220px"
            allow-clear
            @search="loadCommands()"
          />
          <a-select
            v-model:value="statusFilter"
            :placeholder="t('command.filters.allStatuses')"
            allow-clear
            style="width: 150px"
            :options="statusOptions"
            @change="loadCommands()"
          />
          <a-button type="primary" @click="createOpen = true">
            <template #icon><PlusOutlined /></template>
            {{ t('command.submit') }}
          </a-button>
          <a-button :loading="loading" @click="loadCommands()">
            <template #icon><ReloadOutlined /></template>
          </a-button>
        </a-space>
      </template>

      <a-alert
        v-if="createMessage"
        :message="createMessage"
        :type="createMessage === t('command.created') ? 'success' : 'error'"
        show-icon
        closable
        style="margin-bottom: 12px"
      />

      <a-table
        :data-source="filteredCommands"
        :columns="columns"
        :loading="loading"
        :pagination="{ pageSize: 15, hideOnSinglePage: true }"
        row-key="commandId"
        size="middle"
      >
        <template #bodyCell="{ column, record }">
          <template v-if="column.key === 'commandId'">
            <a @click="openDetail(record.commandId)">{{ record.commandId }}</a>
          </template>
          <template v-else-if="column.key === 'gatewayId'">
            <RouterLink :to="`/gateways/${record.gatewayId}`">{{ record.gatewayId }}</RouterLink>
          </template>
          <template v-else-if="column.key === 'type'">
            <a-tag>{{ record.type }}</a-tag>
          </template>
          <template v-else-if="column.key === 'status'">
            <a-tag :color="statusColor(record.status)">{{ t(`status.${record.status}`) }}</a-tag>
          </template>
          <template v-else-if="column.key === 'requestedAt'">
            {{ formatTime(record.requestedAt) }}
          </template>
          <template v-else-if="column.key === 'finishedAt'">
            {{ formatTime(record.finishedAt) }}
          </template>
          <template v-else-if="column.key === 'action'">
            <a-button size="small" type="link" @click="openDetail(record.commandId)">
              {{ t('command.details') }}
            </a-button>
          </template>
        </template>
      </a-table>
    </a-card>

    <a-drawer
      v-model:open="detailOpen"
      :title="selectedCommand ? `${t('command.details')} · ${selectedCommand.commandId}` : t('command.details')"
      width="720"
      destroy-on-close
    >
      <div class="drawer-toolbar">
        <a-space :size="8">
          <span class="toolbar-label">{{ t('command.autoRefresh') }}</span>
          <a-switch v-model:checked="autoRefresh" size="small" />
          <a-button size="small" :loading="detailLoading" @click="refreshSelected">
            <template #icon><ReloadOutlined /></template>
          </a-button>
        </a-space>
        <a-tag v-if="selectedCommand" :color="statusColor(selectedCommand.status)">
          {{ t(`status.${selectedCommand.status}`) }}
        </a-tag>
      </div>
      <a-spin v-if="detailLoading" style="display: block; padding: 48px" />
      <CommandDetailPanel v-else-if="selectedCommand" :command="selectedCommand" />
    </a-drawer>

    <a-modal
      v-model:open="createOpen"
      :title="t('command.submit')"
      :confirm-loading="creating"
      @ok="submitCommand"
    >
      <a-form layout="vertical">
        <a-form-item :label="t('command.fields.gatewayId')" required>
          <a-input v-model:value="form.gatewayId" placeholder="gw-001" />
        </a-form-item>
        <a-form-item :label="t('command.fields.type')">
          <a-select v-model:value="form.type" :options="typeOptions" />
        </a-form-item>
        <a-form-item v-if="form.type === 'config.get'" :label="t('command.fields.pid')">
          <a-input v-model:value="form.pid" />
        </a-form-item>
      </a-form>
    </a-modal>
  </main>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, watch } from 'vue'
import { RouterLink } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { PlusOutlined, ReloadOutlined } from '@ant-design/icons-vue'
import CommandDetailPanel from '@/components/CommandDetailPanel.vue'
import RefreshNotice from '@/components/RefreshNotice.vue'
import { createCommand, fetchCommand, fetchCommands, type CommandDetailItem, type CommandListItem } from '@/api/commands'
import type { CommandStatus } from '@/types/common'

const { t, locale } = useI18n()
const commands = ref<CommandListItem[]>([])
const selectedCommand = ref<CommandDetailItem | null>(null)
const detailOpen = ref(false)
const detailLoading = ref(false)
const autoRefresh = ref(true)
let detailTimer: number | undefined
const createOpen = ref(false)
const loading = ref(false)
const creating = ref(false)
const error = ref<string | null>(null)
const refreshError = ref<string | null>(null)
const createMessage = ref<string | null>(null)
const gatewayFilter = ref('')
const statusFilter = ref<CommandStatus | undefined>(undefined)
const timeRange = ref<string>('all')
let refreshTimer: number | undefined
const form = ref({
  gatewayId: 'gw-001',
  type: 'system.info',
  pid: 'org.eclipse.kura.net.admin.FirewallConfigurationService',
})

const statusOptions = [
  'queued', 'sent', 'running', 'success', 'failed', 'timeout', 'rejected',
].map((value) => ({ value, label: t(`status.${value}`) }))

const typeOptions = [
  { value: 'system.info', label: t('command.types.systemInfo') },
  { value: 'config.get', label: t('command.types.configGet') },
  { value: 'firewall.get', label: t('command.types.firewallGet') },
  { value: 'network.get', label: t('command.types.networkGet') },
]

const timeRangeOptions = computed(() => [
  { value: 'all', label: t('command.filters.allTimes') },
  { value: '1h', label: t('command.filters.lastHour') },
  { value: '24h', label: t('command.filters.last24h') },
])

const filteredCommands = computed(() => {
  if (timeRange.value === 'all') {
    return commands.value
  }
  const cutoff = Date.now() - (timeRange.value === '1h' ? 3600_000 : 24 * 3600_000)
  return commands.value.filter((item) => new Date(item.requestedAt).getTime() >= cutoff)
})

const columns = computed(() => [
  { title: t('command.fields.commandId'), key: 'commandId', width: 130 },
  { title: t('command.fields.gateway'), key: 'gatewayId', width: 110 },
  { title: t('command.fields.type'), key: 'type', width: 140 },
  { title: t('command.fields.status'), key: 'status', width: 100 },
  { title: t('command.fields.initiator'), dataIndex: 'initiator', key: 'initiator', width: 90 },
  { title: t('command.fields.requested'), key: 'requestedAt', width: 170 },
  { title: t('command.fields.finished'), key: 'finishedAt', width: 170 },
  { title: '', key: 'action', width: 90 },
])

function statusColor(status: string) {
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

async function openDetail(commandId: string) {
  detailOpen.value = true
  detailLoading.value = true
  try {
    selectedCommand.value = await fetchCommand(commandId)
  } finally {
    detailLoading.value = false
  }
}

async function refreshSelected() {
  if (!selectedCommand.value || detailLoading.value) {
    return
  }
  try {
    selectedCommand.value = await fetchCommand(selectedCommand.value.commandId)
  } catch {
    // transient refresh failure: keep showing the last snapshot
  }
}

watch([detailOpen, autoRefresh], ([open, enabled]) => {
  if (detailTimer !== undefined) {
    window.clearInterval(detailTimer)
    detailTimer = undefined
  }
  if (open && enabled) {
    detailTimer = window.setInterval(() => void refreshSelected(), 5000)
  }
})

onUnmounted(() => {
  if (detailTimer !== undefined) {
    window.clearInterval(detailTimer)
  }
})

async function loadCommands(options: { silent?: boolean } = {}) {
  if (!options.silent) {
    loading.value = true
    error.value = null
  }
  try {
    const response = await fetchCommands({ gatewayId: gatewayFilter.value.trim(), status: statusFilter.value ?? '' })
    commands.value = response.items
    refreshError.value = null
  } catch (err) {
    const message = err instanceof Error ? err.message : t('command.failedList')
    if (!options.silent || commands.value.length === 0) {
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

async function submitCommand() {
  creating.value = true
  createMessage.value = null
  try {
    const payload =
      form.value.type === 'config.get'
        ? { pid: form.value.pid }
        : {}
    await createCommand({
      gatewayId: form.value.gatewayId,
      type: form.value.type,
      payload,
    })
    createMessage.value = t('command.created')
    createOpen.value = false
    await loadCommands({ silent: true })
  } catch (err) {
    createMessage.value = err instanceof Error ? err.message : t('command.createFailed')
  } finally {
    creating.value = false
  }
}

onMounted(() => {
  void loadCommands()
  refreshTimer = window.setInterval(() => {
    void loadCommands({ silent: true })
  }, 5000)
})

onUnmounted(() => {
  if (refreshTimer !== undefined) {
    window.clearInterval(refreshTimer)
  }
})
</script>

<style scoped>
.table-card {
  border-radius: 8px;
}

.drawer-toolbar {
  align-items: center;
  display: flex;
  justify-content: space-between;
  margin-bottom: 12px;
}

.toolbar-label {
  color: #595959;
  font-size: 13px;
}
</style>
