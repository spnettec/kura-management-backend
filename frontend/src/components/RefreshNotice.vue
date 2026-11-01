<template>
  <a-alert
    v-if="message"
    :message="`${t('common.refreshFailed')} — ${message}`"
    type="warning"
    show-icon
    closable
    style="margin-bottom: 12px"
  />
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'

defineProps<{
  message?: string | null
}>()

const { t } = useI18n()
</script>
