<template>
  <a-config-provider :locale="antdLocale">
    <RouterView />
  </a-config-provider>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { RouterView } from 'vue-router'
import { useI18n } from 'vue-i18n'
import zhCN from 'ant-design-vue/es/locale/zh_CN'
import enUS from 'ant-design-vue/es/locale/en_US'

const { locale } = useI18n()
const antdLocale = computed(() => (locale.value === 'zh-CN' ? zhCN : enUS))
</script>
