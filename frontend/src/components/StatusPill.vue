<template>
  <a-badge :status="badgeStatus" :text="label" />
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import type { GatewayStatus } from '@/types/common'

const props = defineProps<{
  status: GatewayStatus
}>()

const { t } = useI18n()
const label = computed(() => t(`status.${props.status}`))
const badgeStatus = computed(() => {
  if (props.status === 'online') return 'success' as const
  if (props.status === 'offline') return 'error' as const
  return 'default' as const
})
</script>
