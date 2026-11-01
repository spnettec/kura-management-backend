<template>
  <section class="network-view">
    <a-descriptions :column="3" size="small" bordered class="network-summary">
      <a-descriptions-item :label="t('network.hostname')">
        <a-tag>{{ config.hostname || '-' }}</a-tag>
      </a-descriptions-item>
      <a-descriptions-item :label="t('network.interfaces')">
        <a-tag color="blue">{{ config.interfaces.length }}</a-tag>
      </a-descriptions-item>
      <a-descriptions-item :label="t('network.routes')">
        <a-tag color="blue">{{ config.routes.length }}</a-tag>
      </a-descriptions-item>
      <a-descriptions-item :label="t('network.dns')" :span="2">
        {{ config.dnsServers.join(', ') || '-' }}
      </a-descriptions-item>
      <a-descriptions-item :label="t('network.collectedAt')">{{ formatTimestamp(config.collectedAt) }}</a-descriptions-item>
    </a-descriptions>

    <a-alert v-if="config.errorMessage" :message="config.errorMessage" type="error" show-icon style="margin-top: 12px" />

    <a-card size="small" :title="t('network.interfaces')" style="margin-top: 12px">
      <a-table
        :data-source="config.interfaces"
        :columns="interfaceColumns"
        :pagination="false"
        row-key="name"
        size="small"
      >
        <template #bodyCell="{ column, record }">
          <template v-if="column.key === 'name'"><code>{{ record.name }}</code></template>
          <template v-else-if="column.key === 'up'">
            <a-badge :status="record.up ? 'success' : 'default'" :text="record.up ? t('status.up') : t('status.down')" />
          </template>
          <template v-else-if="column.key === 'macAddress'"><code>{{ record.macAddress || '-' }}</code></template>
          <template v-else-if="column.key === 'mtu'">{{ record.mtu || '-' }}</template>
          <template v-else-if="column.key === 'addresses'">
            <div v-if="record.addresses.length > 0" class="address-list">
              <code v-for="address in record.addresses" :key="`${record.name}-${address.address}`">
                {{ address.address }} <span class="addr-family">{{ address.family }}</span>
              </code>
            </div>
            <span v-else>-</span>
          </template>
        </template>
      </a-table>
    </a-card>

    <a-card size="small" :title="t('network.routes')" style="margin-top: 12px">
      <a-table
        :data-source="config.routes"
        :columns="routeColumns"
        :pagination="{ pageSize: 10, hideOnSinglePage: true }"
        :row-key="(_record: NetworkRoutePayload, index: number) => index"
        size="small"
      >
        <template #bodyCell="{ column, record }">
          <template v-if="column.key === 'destination'"><code>{{ record.destination || '-' }}</code></template>
          <template v-else-if="column.key === 'gateway'"><code>{{ record.gateway || '-' }}</code></template>
          <template v-else-if="column.key === 'device'"><code>{{ record.device || '-' }}</code></template>
          <template v-else-if="column.key === 'source'"><code>{{ record.source || '-' }}</code></template>
        </template>
        <template #emptyText>
          <a-empty :description="t('common.noData')" :image-style="{ height: '40px' }" />
        </template>
      </a-table>
    </a-card>
  </section>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'

export interface NetworkAddressPayload {
  address: string
  family: string
  scope: string
}

export interface NetworkInterfacePayload {
  name: string
  displayName: string
  macAddress: string
  up: boolean
  loopback: boolean
  virtual: boolean
  pointToPoint: boolean
  mtu: number
  addresses: NetworkAddressPayload[]
}

export interface NetworkRoutePayload {
  destination: string
  gateway: string
  device: string
  protocol: string
  scope: string
  source: string
  raw: string
}

export interface NetworkConfigPayload {
  collectedAt?: string | number | null
  hostname: string
  interfaces: NetworkInterfacePayload[]
  routes: NetworkRoutePayload[]
  dnsServers: string[]
  source: string
  errorMessage: string
}

defineProps<{
  config: NetworkConfigPayload
}>()

const { t } = useI18n()

const interfaceColumns = computed(() => [
  { title: t('network.fields.name'), key: 'name', width: 110 },
  { title: t('network.fields.state'), key: 'up', width: 90 },
  { title: t('network.fields.mac'), key: 'macAddress', width: 150 },
  { title: t('network.fields.mtu'), dataIndex: 'mtu', key: 'mtu', width: 80 },
  { title: t('network.fields.addresses'), key: 'addresses' },
])

const routeColumns = computed(() => [
  { title: t('network.fields.destination'), key: 'destination', width: 180 },
  { title: t('network.fields.gateway'), key: 'gateway', width: 150 },
  { title: t('network.fields.device'), key: 'device', width: 110 },
  { title: t('network.fields.source'), key: 'source', width: 150 },
])

function formatTimestamp(value?: string | number | null) {
  if (!value) {
    return '-'
  }
  return new Date(value).toLocaleString()
}
</script>

<style scoped>
.network-view {
  margin-top: 4px;
}

.address-list {
  display: grid;
  gap: 4px;
}

code {
  font-size: 12px;
  overflow-wrap: anywhere;
  white-space: pre-wrap;
}

.addr-family {
  color: #8c8c8c;
}
</style>
