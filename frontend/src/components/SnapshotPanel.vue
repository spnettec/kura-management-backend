<template>
  <div class="snapshot-panel">
    <div class="panel-heading">
      <a-space>
        <a-button size="small" :loading="busy" @click="writeSnapshot">
          <template #icon><SaveOutlined /></template>
          {{ t('snapshot.write') }}
        </a-button>
        <a-button size="small" :loading="busy" @click="refresh">
          <template #icon><ReloadOutlined /></template>
          {{ t('snapshot.refresh') }}
        </a-button>
      </a-space>
    </div>

    <a-alert v-if="message" :message="message" type="info" show-icon closable style="margin-bottom: 12px" />

    <a-table
      v-if="rows.length"
      :data-source="rows"
      :columns="columns"
      :loading="busy"
      :pagination="{ pageSize: 10, hideOnSinglePage: true }"
      row-key="id"
      size="small"
    >
      <template #bodyCell="{ column, record }">
        <template v-if="column.key === 'id'">
          <a-tag color="blue">{{ record.id }}</a-tag>
        </template>
        <template v-else-if="column.key === 'archivedAt'">
          {{ record.archivedAt ? formatTime(record.archivedAt) : t('snapshot.notArchived') }}
        </template>
        <template v-else-if="column.key === 'by'">
          {{ record.by || '-' }}
        </template>
        <template v-else-if="column.key === 'sourceCommandId'">
          <RouterLink v-if="record.sourceCommandId" :to="`/commands/${record.sourceCommandId}`">
            {{ record.sourceCommandId }}
          </RouterLink>
          <template v-else>-</template>
        </template>
        <template v-else-if="column.key === 'action'">
          <a-popconfirm :title="t('snapshot.rollback') + ' ?'" @confirm="rollback(record.id)">
            <a-button size="small" danger :disabled="busy">{{ t('snapshot.rollback') }}</a-button>
          </a-popconfirm>
        </template>
      </template>
    </a-table>
    <a-empty v-else-if="!busy" :description="t('snapshot.empty')" />
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { RouterLink } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { ReloadOutlined, SaveOutlined } from '@ant-design/icons-vue'
import { createCommandAndWait } from '@/api/commands'
import { request } from '@/api/http'
import type { ListResponse } from '@/types/common'

interface ArchivedSnapshot {
  gatewaySnapshotId: number
  createdAt: string | number
  createdBy?: string | null
  sourceCommandId?: string | null
}

interface SnapshotRow {
  id: number
  archivedAt: string | number | null
  by: string | null
  sourceCommandId: string | null
}

const props = defineProps<{ gatewayId: string }>()
const { t, locale } = useI18n()

const busy = ref(false)
const message = ref('')
const rows = ref<SnapshotRow[]>([])

const columns = computed(() => [
  { title: t('snapshot.id'), key: 'id', width: 160 },
  { title: t('snapshot.archivedAt'), key: 'archivedAt', width: 180 },
  { title: t('snapshot.by'), key: 'by', width: 90 },
  { title: t('snapshot.source'), key: 'sourceCommandId', width: 130 },
  { title: '', key: 'action', width: 130 },
])

function formatTime(value: string | number | null): string {
  if (!value) return '-'
  return new Date(value).toLocaleString(locale.value)
}

async function run(type: string, payload: Record<string, unknown>): Promise<Record<string, unknown>> {
  const detail = await createCommandAndWait({ gatewayId: props.gatewayId, type, payload })
  if (detail.status === 'success') {
    // replies are stored as full envelopes; the command payload lives one level down
    return (detail.replyPayload?.payload as Record<string, unknown> | undefined) ?? {}
  }
  throw new Error(detail.errorMessage || t('snapshot.commandFailed'))
}

async function refresh() {
  busy.value = true
  message.value = ''
  try {
    const [reply, archived] = await Promise.all([
      run('snapshot.list', {}),
      request<ListResponse<ArchivedSnapshot>>(`/api/gateways/${props.gatewayId}/snapshots`),
    ])
    const archiveById = new Map<number, ArchivedSnapshot>()
    for (const item of archived.items ?? []) {
      archiveById.set(item.gatewaySnapshotId, item)
    }
    const gatewayIds = ((reply.snapshots as number[]) ?? []).slice().sort((a, b) => b - a)
    for (const archivedItem of archived.items ?? []) {
      if (!gatewayIds.includes(archivedItem.gatewaySnapshotId)) {
        gatewayIds.push(archivedItem.gatewaySnapshotId)
      }
    }
    rows.value = gatewayIds.map((id) => {
      const match = archiveById.get(id)
      return {
        id,
        archivedAt: match?.createdAt ?? null,
        by: match?.createdBy ?? null,
        sourceCommandId: match?.sourceCommandId ?? null,
      }
    })
    message.value = t('snapshot.listLoaded', { count: rows.value.length })
  } catch (e) {
    message.value = String((e as Error).message || e)
  } finally {
    busy.value = false
  }
}

async function writeSnapshot() {
  busy.value = true
  message.value = ''
  try {
    const reply = await run('snapshot.take', {})
    const id = (reply as { snapshotId?: number }).snapshotId
    message.value = t('snapshot.written', { id: id ?? '?' })
    await refresh()
  } catch (e) {
    message.value = String((e as Error).message || e)
  } finally {
    busy.value = false
  }
}

async function rollback(snapshotId: number) {
  busy.value = true
  message.value = ''
  try {
    const reply = await run('snapshot.rollback', { snapshotId })
    const id = (reply as { snapshotId?: number }).snapshotId
    message.value = t('snapshot.rolledBack', { id: snapshotId, newId: id ?? '?' })
  } catch (e) {
    message.value = String((e as Error).message || e)
  } finally {
    busy.value = false
  }
}
</script>

<style scoped>
.snapshot-panel {
  display: grid;
  gap: 12px;
}

.panel-heading {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
}
</style>
