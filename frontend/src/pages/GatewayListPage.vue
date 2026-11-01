<template>
  <main class="page-shell">
    <RefreshNotice :message="refreshError" />

    <div class="summary-grid">
      <a-card v-for="card in summaryCards" :key="card.key" size="small" hoverable @click="setStatusFilter(card.filter)">
        <a-statistic :title="card.title" :value="card.value" :value-style="{ color: card.color }" />
      </a-card>
    </div>

    <a-card size="small" class="table-card">
      <template #title>
        {{ t('gateway.listTitle') }}
        <a-tag v-if="statusFilter" closable @close="setStatusFilter('')">{{ statusLabel(statusFilter) }}</a-tag>
      </template>
      <template #extra>
        <a-space>
          <a-input-search
            v-model:value="searchKeyword"
            :placeholder="t('gateway.searchPlaceholder')"
            allow-clear
            style="width: 220px"
          />
          <a-select
            v-model:value="statusFilter"
            :placeholder="t('gateway.filters.allStatuses')"
            allow-clear
            style="width: 160px"
            :options="statusOptions"
            @change="loadGateways()"
          />
          <a-button type="primary" @click="createOpen = true">
            <template #icon><PlusOutlined /></template>
            {{ t('gateway.create') }}
          </a-button>
          <a-button :loading="loading" @click="loadGateways()">
            <template #icon><ReloadOutlined /></template>
          </a-button>
        </a-space>
      </template>

      <a-table
        :data-source="filteredGateways"
        :columns="columns"
        :loading="loading"
        :pagination="{ pageSize: 15, hideOnSinglePage: true }"
        row-key="gatewayId"
        size="middle"
      >
        <template #bodyCell="{ column, record }">
          <template v-if="column.key === 'name'">
            <RouterLink :to="`/gateways/${record.gatewayId}`">{{ record.name }}</RouterLink>
          </template>
          <template v-else-if="column.key === 'status'">
            <StatusPill :status="record.status" />
          </template>
          <template v-else-if="column.key === 'enabled'">
            <a-switch :checked="record.enabled" size="small" disabled />
          </template>
          <template v-else-if="column.key === 'lastSeenAt'">
            {{ formatTime(record.lastSeenAt) }}
          </template>
          <template v-else-if="column.key === 'ipAddrs'">
            {{ record.ipAddrs.join(', ') || '-' }}
          </template>
        </template>
      </a-table>
    </a-card>

    <a-modal
      v-model:open="createOpen"
      :title="t('gateway.create')"
      :confirm-loading="creatingGateway"
      @ok="submitGateway"
    >
      <a-form layout="vertical">
        <a-form-item :label="t('gateway.fields.gatewayId')" required>
          <a-input v-model:value="newGateway.gatewayId" placeholder="gw-002" />
        </a-form-item>
        <a-form-item :label="t('gateway.fields.name')">
          <a-input v-model:value="newGateway.name" />
        </a-form-item>
      </a-form>
      <a-alert v-if="createMessage" :message="createMessage" type="info" show-icon />
    </a-modal>
  </main>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { RouterLink } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { PlusOutlined, ReloadOutlined } from '@ant-design/icons-vue'
import StatusPill from '@/components/StatusPill.vue'
import RefreshNotice from '@/components/RefreshNotice.vue'
import {
  createGateway,
  fetchGateways,
  fetchGatewaySummary,
  type GatewayListItem,
  type GatewaySummary,
} from '@/api/gateways'
import type { GatewayStatus } from '@/types/common'

const { t, locale } = useI18n()
const gateways = ref<GatewayListItem[]>([])
const summary = ref<GatewaySummary>({
  total: 0,
  online: 0,
  offline: 0,
  unknown: 0,
})
const loading = ref(false)
const error = ref<string | null>(null)
const refreshError = ref<string | null>(null)
const statusFilter = ref<GatewayStatus | undefined>(undefined)
const searchKeyword = ref('')
const createOpen = ref(false)
const creatingGateway = ref(false)
const createMessage = ref<string | null>(null)
const newGateway = ref({
  gatewayId: '',
  name: '',
})
let refreshTimer: number | undefined

const summaryCards = computed(() => [
  { key: 'total', title: t('gateway.summaryFields.total'), value: summary.value.total, color: '#1f2328', filter: '' as const },
  { key: 'online', title: t('status.online'), value: summary.value.online, color: '#389e0d', filter: 'online' as const },
  { key: 'offline', title: t('status.offline'), value: summary.value.offline, color: '#cf1322', filter: 'offline' as const },
  { key: 'unknown', title: t('status.unknown'), value: summary.value.unknown, color: '#8c8c8c', filter: 'unknown' as const },
])

const statusOptions = [
  { value: 'online', label: t('status.online') },
  { value: 'offline', label: t('status.offline') },
  { value: 'unknown', label: t('status.unknown') },
]

const columns = computed(() => [
  { title: t('gateway.fields.name'), key: 'name' },
  { title: t('gateway.fields.gatewayId'), dataIndex: 'gatewayId', key: 'gatewayId' },
  { title: t('gateway.fields.status'), key: 'status', width: 110 },
  { title: t('gateway.fields.enabled'), key: 'enabled', width: 90 },
  { title: t('gateway.fields.lastSeen'), key: 'lastSeenAt', width: 170 },
  { title: t('gateway.fields.kura'), dataIndex: 'kuraVersion', key: 'kuraVersion', ellipsis: true },
  { title: t('gateway.fields.ip'), key: 'ipAddrs', ellipsis: true },
])

const filteredGateways = computed(() => {
  const keyword = searchKeyword.value.trim().toLowerCase()
  if (!keyword) {
    return gateways.value
  }
  return gateways.value.filter((item) =>
    item.name.toLowerCase().includes(keyword)
    || item.gatewayId.toLowerCase().includes(keyword)
    || item.ipAddrs.some((ip) => ip.toLowerCase().includes(keyword)))
})

function statusLabel(status: GatewayStatus) {
  return t(`status.${status}`)
}

function formatTime(value?: string | number | null) {
  if (!value) {
    return '-'
  }
  return new Date(value).toLocaleString(locale.value)
}

async function loadGateways(options: { silent?: boolean } = {}) {
  if (!options.silent) {
    loading.value = true
    error.value = null
  }
  try {
    const [response, summaryResponse] = await Promise.all([
      fetchGateways({ status: statusFilter.value ?? '' }),
      fetchGatewaySummary(),
    ])
    gateways.value = response.items
    summary.value = summaryResponse
    refreshError.value = null
  } catch (err) {
    const message = err instanceof Error ? err.message : t('gateway.failedList')
    if (!options.silent || gateways.value.length === 0) {
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

function setStatusFilter(status: GatewayStatus | '') {
  statusFilter.value = status || undefined
  void loadGateways()
}

async function submitGateway() {
  if (!newGateway.value.gatewayId) return
  creatingGateway.value = true
  createMessage.value = null
  try {
    await createGateway({
      gatewayId: newGateway.value.gatewayId,
      name: newGateway.value.name,
    })
    createMessage.value = t('gateway.created')
    newGateway.value = { gatewayId: '', name: '' }
    createOpen.value = false
    await loadGateways()
  } catch (err) {
    createMessage.value = err instanceof Error ? err.message : t('gateway.createFailed')
  } finally {
    creatingGateway.value = false
  }
}

onMounted(() => {
  void loadGateways()
  refreshTimer = window.setInterval(() => {
    void loadGateways({ silent: true })
  }, 15000)
})

onUnmounted(() => {
  if (refreshTimer !== undefined) {
    window.clearInterval(refreshTimer)
  }
})
</script>

<style scoped>
.summary-grid {
  display: grid;
  gap: 12px;
  grid-template-columns: repeat(auto-fit, minmax(160px, 1fr));
  margin-bottom: 16px;
}

.table-card {
  border-radius: 8px;
}
</style>
