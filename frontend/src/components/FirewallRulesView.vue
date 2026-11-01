<template>
  <section class="firewall-view">
    <a-descriptions :column="3" size="small" bordered class="firewall-summary">
      <a-descriptions-item :label="t('firewall.enabled')">
        <a-badge :status="config.enabled ? 'success' : 'default'" :text="config.enabled ? t('common.yes') : t('common.no')" />
      </a-descriptions-item>
      <a-descriptions-item :label="t('firewall.running')">
        <a-badge :status="config.running ? 'processing' : 'default'" :text="config.running ? t('status.running') : t('status.inactive')" />
      </a-descriptions-item>
      <a-descriptions-item :label="t('firewall.health')">
        <a-tag :color="config.healthy ? 'success' : 'error'">{{ config.healthy ? t('status.normal') : t('status.abnormal') }}</a-tag>
      </a-descriptions-item>
      <a-descriptions-item :label="t('firewall.packageInstalled')">
        {{ config.packageInstalled ? t('common.yes') : t('common.no') }}
      </a-descriptions-item>
      <a-descriptions-item :label="t('firewall.serviceState')">{{ translateState(config.serviceState) }}</a-descriptions-item>
      <a-descriptions-item :label="t('firewall.systemdState')">{{ translateState(config.systemdState) }}</a-descriptions-item>
      <a-descriptions-item :label="t('firewall.ruleTotals')">
        <a-tag color="blue">{{ totalRuleCount }}</a-tag>
      </a-descriptions-item>
      <a-descriptions-item :label="t('firewall.source')">{{ config.source || '-' }}</a-descriptions-item>
      <a-descriptions-item :label="t('firewall.collectedAt')">{{ formatTimestamp(config.collectedAt) }}</a-descriptions-item>
    </a-descriptions>

    <a-alert v-if="config.errorMessage" :message="config.errorMessage" type="error" show-icon style="margin-top: 12px" />
    <a-alert v-if="config.healthMessage && !config.healthy" :message="config.healthMessage" type="warning" show-icon style="margin-top: 12px" />

    <div class="firewall-controls">
      <a-segmented v-model:value="ipVersionFilter" :options="ipVersionOptions" size="small" />
      <a-select
        v-model:value="tableFilter"
        :placeholder="t('firewall.filters.allTables')"
        allow-clear
        size="small"
        style="min-width: 160px"
        :options="tableOptions.map((name) => ({ value: name, label: name }))"
      />
      <a-input-search
        v-model:value="searchText"
        :placeholder="t('firewall.filters.searchPlaceholder')"
        size="small"
        allow-clear
      />
    </div>

    <a-empty v-if="visibleTableCount === 0" :description="t('firewall.noRules')" style="padding: 24px" />
    <div v-for="ruleSet in visibleRuleSets" :key="ruleSet.ipVersion" class="ruleset">
      <a-divider content-position="left">
        {{ t(`firewall.${ruleSet.ipVersion}`, ruleSet.ipVersion) }}
      </a-divider>
      <section v-for="table in ruleSet.tables" :key="`${ruleSet.ipVersion}-${table.name}`" class="table-block">
        <div class="table-heading">
          <a-tag color="geekblue">{{ table.name }}</a-tag>
          <span class="table-meta">{{ t('firewall.ruleCount', { count: table.rules.length }) }}</span>
        </div>
        <a-table
          v-if="table.chains.length > 0"
          :data-source="table.chains"
          :columns="chainColumns"
          :pagination="false"
          row-key="name"
          size="small"
          style="margin-bottom: 12px"
        >
          <template #bodyCell="{ column, record }">
            <template v-if="column.key === 'name'"><code>{{ record.name }}</code></template>
            <template v-else-if="column.key === 'policy'">{{ record.policy || '-' }}</template>
            <template v-else-if="column.key === 'counters'"><code>{{ record.counters || '-' }}</code></template>
          </template>
        </a-table>
        <a-table
          :data-source="table.rules"
          :columns="ruleColumns"
          :pagination="false"
          row-key="expression"
          size="small"
        >
          <template #bodyCell="{ column, record }">
            <template v-if="column.key === 'action'">
              <a-tag :color="record.action === 'ACCEPT' ? 'success' : (record.action === 'DROP' ? 'error' : 'default')">
                {{ record.action }}
              </a-tag>
            </template>
            <template v-else-if="column.key === 'chain'"><code>{{ record.chain }}</code></template>
            <template v-else-if="column.key === 'expression'"><code>{{ record.expression || '-' }}</code></template>
          </template>
          <template #emptyText>
            <a-empty :description="t('firewall.noRules')" :image-style="{ height: '40px' }" />
          </template>
        </a-table>
      </section>
    </div>
  </section>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { useI18n } from 'vue-i18n'

export interface FirewallRule {
  action: string
  chain: string
  expression: string
}

export interface FirewallTable {
  name: string
  chains: Array<{ name: string; policy: string; counters: string }>
  rules: FirewallRule[]
}

export interface FirewallRuleSet {
  ipVersion: string
  tables: FirewallTable[]
}

export interface FirewallConfigPayload {
  enabled: boolean
  running?: boolean
  healthy?: boolean
  packageInstalled: boolean
  serviceState: string
  systemdState?: string | null
  healthMessage?: string | null
  collectedAt?: string | number | null
  source?: string | null
  errorMessage?: string | null
  ipv4Rules: string
  ipv6Rules: string
  parsedRules: FirewallRuleSet[]
  rules: unknown[]
}

const props = defineProps<{
  config: FirewallConfigPayload
}>()

const { t } = useI18n()
const ipVersionFilter = ref<string>('')
const tableFilter = ref<string | undefined>(undefined)
const searchText = ref('')

const chainColumns = computed(() => [
  { title: t('firewall.fields.chain'), key: 'name', width: 140 },
  { title: t('firewall.fields.policy'), key: 'policy', width: 110 },
  { title: t('firewall.fields.counters'), key: 'counters' },
])

const ruleColumns = computed(() => [
  { title: t('firewall.fields.action'), key: 'action', width: 90 },
  { title: t('firewall.fields.chain'), key: 'chain', width: 130 },
  { title: t('firewall.fields.expression'), key: 'expression' },
])

const ipVersionOptions = computed(() => [
  { value: '', label: t('firewall.filters.allIpVersions') },
  { value: 'ipv4', label: t('firewall.ipv4') },
  { value: 'ipv6', label: t('firewall.ipv6') },
])

const tableOptions = computed(() => {
  const names = new Set<string>()
  props.config.parsedRules.forEach((ruleSet) => {
    ruleSet.tables.forEach((table) => names.add(table.name))
  })
  return Array.from(names).sort()
})

const totalRuleCount = computed(() => props.config.parsedRules.reduce((sum, ruleSet) => {
  return sum + ruleSet.tables.reduce((tableSum, table) => tableSum + table.rules.length, 0)
}, 0))

const visibleRuleSets = computed<FirewallRuleSet[]>(() => {
  const normalizedSearch = searchText.value.toLowerCase()
  const tableSelection = tableFilter.value ?? ''
  return props.config.parsedRules
    .filter((ruleSet) => !ipVersionFilter.value || ruleSet.ipVersion === ipVersionFilter.value)
    .map((ruleSet) => ({
      ...ruleSet,
      tables: ruleSet.tables
        .filter((table) => !tableSelection || table.name === tableSelection)
        .map((table) => {
          if (!normalizedSearch) {
            return table
          }
          const tableMatches = includesSearch(table.name, normalizedSearch)
          const chainMatches = table.chains.some((chain) => includesSearch(chain.name, normalizedSearch)
            || includesSearch(chain.policy, normalizedSearch)
            || includesSearch(chain.counters, normalizedSearch))
          const rules = table.rules.filter((rule) => includesSearch(rule.action, normalizedSearch)
            || includesSearch(rule.chain, normalizedSearch)
            || includesSearch(rule.expression, normalizedSearch))
          return {
            ...table,
            rules: tableMatches || chainMatches ? table.rules : rules,
          }
        })
        .filter((table) => {
          if (!normalizedSearch) {
            return true
          }
          return includesSearch(table.name, normalizedSearch)
            || table.chains.some((chain) => includesSearch(chain.name, normalizedSearch)
              || includesSearch(chain.policy, normalizedSearch)
              || includesSearch(chain.counters, normalizedSearch))
            || table.rules.length > 0
        }),
    }))
    .filter((ruleSet) => ruleSet.tables.length > 0)
})

const visibleTableCount = computed(() => visibleRuleSets.value.reduce((sum, ruleSet) => sum + ruleSet.tables.length, 0))

function includesSearch(value: string | undefined, query: string) {
  return String(value ?? '').toLowerCase().includes(query)
}

function formatTimestamp(value?: string | number | null) {
  if (!value) {
    return '-'
  }
  return new Date(value).toLocaleString()
}

function translateState(value?: string | null) {
  if (!value) {
    return '-'
  }
  return t(`status.${value}`, value)
}
</script>

<style scoped>
.firewall-view {
  margin-top: 4px;
}

.firewall-controls {
  align-items: center;
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
  margin: 12px 0;
}

.ruleset {
  display: grid;
  gap: 12px;
}

.table-heading {
  align-items: center;
  display: flex;
  gap: 10px;
  margin-bottom: 8px;
}

.table-meta {
  color: #6b7280;
  font-size: 12px;
}

code {
  font-size: 12px;
  overflow-wrap: anywhere;
  white-space: pre-wrap;
}
</style>
