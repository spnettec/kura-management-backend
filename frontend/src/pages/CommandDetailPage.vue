<template>
  <main class="page-shell">
    <RefreshNotice :message="refreshError" />
    <a-card size="small">
      <template #title>{{ t('command.detailTitle') }}</template>
      <a-skeleton v-if="loading" active :paragraph="{ rows: 4 }" />
      <a-result v-else-if="error" status="warning" :title="error" />
      <CommandDetailPanel v-else-if="command" :command="command" />
    </a-card>
  </main>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, watch } from 'vue'
import { useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'
import CommandDetailPanel from '@/components/CommandDetailPanel.vue'
import RefreshNotice from '@/components/RefreshNotice.vue'
import { fetchCommand, type CommandDetailItem } from '@/api/commands'

const props = defineProps<{
  commandId?: string
}>()

const route = useRoute()
const { t } = useI18n()
const command = ref<CommandDetailItem | null>(null)
const loading = ref(false)
const error = ref<string | null>(null)
const refreshError = ref<string | null>(null)
let refreshTimer: number | undefined

const resolvedCommandId = computed(() => String(props.commandId ?? route.params.commandId ?? ''))

async function loadCommand(options: { silent?: boolean } = {}) {
  const commandId = resolvedCommandId.value
  if (!commandId) {
    error.value = t('command.missingId')
    return
  }

  if (!options.silent) {
    loading.value = true
    error.value = null
  }
  try {
    command.value = await fetchCommand(commandId)
    refreshError.value = null
  } catch (err) {
    const message = err instanceof Error ? err.message : t('command.failedDetail')
    if (!options.silent || !command.value) {
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
  void loadCommand()
  refreshTimer = window.setInterval(() => {
    void loadCommand({ silent: true })
  }, 5000)
})

onUnmounted(() => {
  if (refreshTimer !== undefined) {
    window.clearInterval(refreshTimer)
  }
})

watch(resolvedCommandId, () => {
  void loadCommand()
})
</script>
