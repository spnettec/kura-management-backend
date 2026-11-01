<template>
  <a-layout class="app-shell">
    <a-layout-sider theme="dark" :width="216" class="app-sider">
      <div class="brand">
        <span class="brand-dot"></span>
        {{ t('app.brand') }}
      </div>
      <a-menu
        theme="dark"
        mode="inline"
        :selected-keys="selectedKeys"
        :items="menuItems"
        @click="onMenuClick"
      />
      <div class="sider-footer">tenant / {{ auth.tenantId }}</div>
    </a-layout-sider>
    <a-layout>
      <a-layout-header class="app-header">
        <a-breadcrumb>
          <a-breadcrumb-item>{{ t('app.brand') }}</a-breadcrumb-item>
          <a-breadcrumb-item v-if="route.meta.titleKey">{{ t(String(route.meta.titleKey)) }}</a-breadcrumb-item>
        </a-breadcrumb>
        <a-space :size="12">
          <a-segmented
            :value="locale"
            :options="localeOptions"
            size="small"
            @change="onLocaleChange"
          />
          <a-dropdown>
            <a class="user-entry" @click.prevent>
              <a-avatar :size="28" style="background-color: #1677ff">
                {{ (auth.displayName || auth.username || '?').slice(0, 1).toUpperCase() }}
              </a-avatar>
              <span class="user-name">{{ auth.displayName || auth.username }}</span>
            </a>
            <template #overlay>
              <a-menu @click="onUserMenuClick">
                <a-menu-item key="logout">
                  {{ t('nav.logout') }}
                </a-menu-item>
              </a-menu>
            </template>
          </a-dropdown>
        </a-space>
      </a-layout-header>
      <a-layout-content class="app-content">
        <RouterView />
      </a-layout-content>
    </a-layout>
  </a-layout>
</template>

<script setup lang="ts">
import { computed, h } from 'vue'
import { RouterLink, RouterView, useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import {
  CloudServerOutlined,
  CodeSandboxOutlined,
} from '@ant-design/icons-vue'
import { setLocale, supportedLocales } from '@/i18n'
import { useAuthStore } from '@/store/auth'

const { t, locale } = useI18n()
const route = useRoute()
const router = useRouter()
const auth = useAuthStore()

const menuItems = computed(() => [
  {
    key: '/gateways',
    icon: () => h(CloudServerOutlined),
    label: h(RouterLink, { to: '/gateways' }, () => t('nav.gateways')),
  },
  {
    key: '/commands',
    icon: () => h(CodeSandboxOutlined),
    label: h(RouterLink, { to: '/commands' }, () => t('nav.commands')),
  },
])

const selectedKeys = computed(() => {
  const path = route.path
  if (path.startsWith('/gateways')) return ['/gateways']
  if (path.startsWith('/commands')) return ['/commands']
  return []
})

const localeOptions = supportedLocales.map((item) => ({
  label: item === 'zh-CN' ? '中文' : 'EN',
  value: item,
}))

function onMenuClick({ key }: { key: string }) {
  void router.push(String(key))
}

function onLocaleChange(value: string | number) {
  const next = String(value)
  if (next === 'zh-CN' || next === 'en-US') {
    setLocale(next)
  }
}

function onUserMenuClick({ key }: { key: string }) {
  if (key === 'logout') {
    void auth.logout().finally(() => router.push('/login'))
  }
}
</script>

<style scoped>
.app-shell {
  min-height: 100vh;
}

.app-sider {
  display: flex;
  flex-direction: column;
}

.brand {
  align-items: center;
  color: #fff;
  display: flex;
  font-size: 16px;
  font-weight: 700;
  gap: 8px;
  height: 56px;
  padding: 0 20px;
}

.brand-dot {
  background: #1677ff;
  border-radius: 3px;
  display: inline-block;
  height: 14px;
  width: 14px;
}

.app-sider :deep(.ant-layout-sider-children) {
  display: flex;
  flex-direction: column;
}

.sider-footer {
  color: #6b7280;
  font-size: 12px;
  padding: 14px 24px;
}

.app-header {
  align-items: center;
  background: #fff;
  border-bottom: 1px solid #f0f0f0;
  display: flex;
  justify-content: space-between;
  line-height: 56px;
  padding: 0 24px;
}

.header-title {
  color: #1f2328;
  font-size: 15px;
  font-weight: 600;
}

.app-content {
  padding: 24px;
}

.user-entry {
  align-items: center;
  color: #1f2328;
  display: inline-flex;
  gap: 8px;
}

.user-name {
  font-weight: 500;
}
</style>
