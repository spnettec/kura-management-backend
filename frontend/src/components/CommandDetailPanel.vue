<template>
  <section class="command-detail">
    <a-descriptions :column="3" size="small" bordered style="margin-bottom: 16px">
      <a-descriptions-item :label="t('command.fields.commandId')">
        <a-tag>{{ command.commandId }}</a-tag>
      </a-descriptions-item>
      <a-descriptions-item :label="t('command.fields.gateway')">
        <RouterLink :to="`/gateways/${command.gatewayId}`">{{ command.gatewayId }}</RouterLink>
      </a-descriptions-item>
      <a-descriptions-item :label="t('command.fields.type')">
        <a-tag color="blue">{{ command.type }}</a-tag>
      </a-descriptions-item>
      <a-descriptions-item :label="t('command.fields.initiator')">{{ command.initiator ?? '-' }}</a-descriptions-item>
      <a-descriptions-item :label="t('command.fields.requested')">{{ formatTime(command.requestedAt) }}</a-descriptions-item>
      <a-descriptions-item :label="t('command.fields.finished')">{{ formatTime(command.finishedAt) }}</a-descriptions-item>
    </a-descriptions>

    <a-card size="small" :title="t('command.events')" style="margin-bottom: 16px">
      <a-timeline v-if="command.events.length" style="margin-top: 8px">
        <a-timeline-item
          v-for="event in command.events"
          :key="event.eventId"
          :color="eventColor(event.eventType)"
        >
          <div class="event-line">
            <strong>{{ t(`command.eventTypes.${event.eventType}`, event.eventType) }}</strong>
            <span class="event-time">{{ formatTime(event.createdAt) }}</span>
          </div>
          <div class="event-message">{{ event.message ?? '-' }}</div>
        </a-timeline-item>
      </a-timeline>
      <a-empty v-else :image-style="{ height: '40px' }" />
    </a-card>

    <div class="json-grid">
      <a-card size="small" :title="t('command.requestPayload')">
        <pre>{{ formatJson(command.requestPayload) }}</pre>
      </a-card>
      <a-card size="small" :title="t('command.replyPayload')">
        <pre>{{ command.replyPayload ? formatJson(command.replyPayload) : t('command.noReply') }}</pre>
      </a-card>
    </div>

    <template v-if="firewallConfig || networkConfig">
      <a-divider />
      <FirewallRulesView v-if="firewallConfig" :config="firewallConfig" />
      <NetworkStatusView v-if="networkConfig" :config="networkConfig" />
    </template>
  </section>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { RouterLink } from 'vue-router'
import { useI18n } from 'vue-i18n'
import type { CommandDetailItem } from '@/api/commands'
import FirewallRulesView, { type FirewallConfigPayload } from '@/components/FirewallRulesView.vue'
import NetworkStatusView, { type NetworkConfigPayload } from '@/components/NetworkStatusView.vue'

const props = defineProps<{
  command: CommandDetailItem
}>()

const { t, locale } = useI18n()

const firewallConfig = computed(() => {
  const payload = props.command.replyPayload?.payload
  if (isFirewallConfigPayload(payload)) {
    return payload
  }
  return null
})

const networkConfig = computed(() => {
  const payload = props.command.replyPayload?.payload
  if (isNetworkConfigPayload(payload)) {
    return payload
  }
  return null
})

function eventColor(eventType: string) {
  if (eventType === 'success') return 'green'
  if (eventType === 'failed' || eventType === 'timeout' || eventType === 'rejected') return 'red'
  return 'blue'
}

function formatTime(value?: string | number | null) {
  if (!value) {
    return '-'
  }
  return new Date(value).toLocaleString(locale.value)
}

function formatJson(value: unknown) {
  return JSON.stringify(value, null, 2)
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
</script>

<style scoped>
.json-grid {
  display: grid;
  gap: 12px;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
}

.event-line {
  align-items: baseline;
  display: flex;
  gap: 10px;
}

.event-time {
  color: #8c8c8c;
  font-size: 12px;
}

.event-message {
  color: #595959;
  font-size: 13px;
}

pre {
  background: #0f172a;
  border-radius: 6px;
  color: #e5e7eb;
  margin: 0;
  max-height: 320px;
  overflow: auto;
  padding: 12px;
  white-space: pre-wrap;
  word-break: break-word;
}
</style>
