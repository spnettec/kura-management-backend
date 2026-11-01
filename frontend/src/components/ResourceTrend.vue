<template>
  <div class="trend-card">
    <div class="trend-header">
      <span v-for="series in seriesList" :key="series.label" class="trend-legend">
        <span class="legend-dot" :style="{ background: series.color }"></span>
        {{ series.label }}
        <strong>{{ series.current.toFixed(1) }}%</strong>
      </span>
    </div>
    <svg :viewBox="`0 0 ${width} ${height}`" preserveAspectRatio="none" class="trend-svg">
      <line v-for="grid in gridLines" :key="grid" x1="0" :x2="width" :y1="grid" :y2="grid" class="grid-line" />
      <polyline
        v-for="series in seriesList"
        :key="series.label"
        :points="series.points"
        fill="none"
        :stroke="series.color"
        stroke-width="2"
        stroke-linejoin="round"
        stroke-linecap="round"
      />
    </svg>
    <div class="trend-range">
      <span>{{ rangeLabel.start }}</span>
      <span>{{ rangeLabel.end }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import type { GatewayHeartbeatItem } from '@/api/gateways'

const props = defineProps<{
  heartbeats: GatewayHeartbeatItem[]
}>()

const { locale } = useI18n()

const width = 600
const height = 120
const padding = 6

// heartbeats arrive newest-first; draw oldest -> newest left to right
const ordered = computed(() => props.heartbeats.slice(0, 20).reverse())

const gridLines = computed(() => {
  const lines: number[] = []
  for (let i = 1; i <= 3; i += 1) {
    lines.push(padding + ((height - padding * 2) * i) / 4)
  }
  return lines
})

function buildPoints(pick: (item: GatewayHeartbeatItem) => number | null | undefined) {
  const values = ordered.value.map((item) => pick(item) ?? 0)
  if (values.length === 0) {
    return ''
  }
  const step = values.length > 1 ? (width - padding * 2) / (values.length - 1) : 0
  return values
    .map((value, index) => {
      const x = padding + step * index
      const y = height - padding - (Math.min(Math.max(value, 0), 100) / 100) * (height - padding * 2)
      return `${x.toFixed(1)},${y.toFixed(1)}`
    })
    .join(' ')
}

const seriesList = computed(() => [
  { label: 'CPU', color: '#1677ff', points: buildPoints((item) => item.cpuPct) },
  { label: 'MEM', color: '#722ed1', points: buildPoints((item) => item.memPct) },
  { label: 'DISK', color: '#fa8c16', points: buildPoints((item) => item.diskPct) },
].map((series) => ({
  ...series,
  current: latest(series.label === 'CPU' ? (item: GatewayHeartbeatItem) => item.cpuPct
    : series.label === 'MEM' ? (item: GatewayHeartbeatItem) => item.memPct
      : (item: GatewayHeartbeatItem) => item.diskPct),
})))

const rangeLabel = computed(() => {
  const items = ordered.value
  const format = (value?: string | number | null) =>
    value ? new Date(value).toLocaleTimeString(locale.value) : '-'
  return {
    start: format(items[0]?.ts),
    end: format(items[items.length - 1]?.ts),
  }
})

function latest(pick: (item: GatewayHeartbeatItem) => number | null | undefined) {
  return pick(props.heartbeats[0] ?? ({} as GatewayHeartbeatItem)) ?? 0
}
</script>

<style scoped>
.trend-card {
  background: #fff;
  border: 1px solid #f0f0f0;
  border-radius: 8px;
  margin-bottom: 12px;
  padding: 12px;
}

.trend-header {
  display: flex;
  flex-wrap: wrap;
  gap: 16px;
  margin-bottom: 8px;
}

.trend-legend {
  align-items: center;
  color: #595959;
  display: inline-flex;
  font-size: 13px;
  gap: 6px;
}

.trend-legend strong {
  color: #1f2328;
}

.legend-dot {
  border-radius: 2px;
  display: inline-block;
  height: 10px;
  width: 10px;
}

.trend-svg {
  display: block;
  width: 100%;
}

.grid-line {
  stroke: #f0f0f0;
  stroke-width: 1;
}

.trend-range {
  color: #8c8c8c;
  display: flex;
  font-size: 12px;
  justify-content: space-between;
}
</style>
