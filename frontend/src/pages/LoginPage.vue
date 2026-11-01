<template>
  <main class="page-shell login-shell">
    <a-card class="login-card" :bordered="false">
      <div class="login-brand">
        <span class="brand-dot"></span>
        <h1>{{ t('app.brand') }}</h1>
      </div>
      <a-form layout="vertical" @finish="submit">
        <a-form-item :label="t('auth.username')" name="username">
          <a-input v-model:value="username" size="large" :autocomplete="['username']">
            <template #prefix><UserOutlined style="color: rgba(0, 0, 0, 0.35)" /></template>
          </a-input>
        </a-form-item>
        <a-form-item :label="t('auth.password')" name="password">
          <a-input-password v-model:value="password" size="large" autocomplete="current-password" />
        </a-form-item>
        <a-alert
          v-if="error"
          :message="error"
          type="error"
          show-icon
          style="margin-bottom: 16px"
        />
        <a-button type="primary" size="large" block :loading="submitting" @click="submit">
          {{ t('auth.submit') }}
        </a-button>
      </a-form>
    </a-card>
  </main>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { UserOutlined } from '@ant-design/icons-vue'
import { useAuthStore } from '@/store/auth'

const { t } = useI18n()
const route = useRoute()
const router = useRouter()
const auth = useAuthStore()

const username = ref('')
const password = ref('')
const error = ref('')
const submitting = ref(false)

async function submit() {
  if (!username.value || !password.value || submitting.value) return
  submitting.value = true
  error.value = ''
  try {
    await auth.login(username.value, password.value)
    const redirect = typeof route.query.redirect === 'string' ? route.query.redirect : '/gateways'
    router.push(redirect)
  } catch (e) {
    error.value = t('auth.loginFailed')
    password.value = ''
  } finally {
    submitting.value = false
  }
}
</script>

<style scoped>
.login-shell {
  align-items: center;
  background: linear-gradient(160deg, #f5f7fa 0%, #e4ecf7 100%);
  display: grid;
  min-height: 100vh;
  place-items: center;
}

.login-card {
  border-radius: 12px;
  box-shadow: 0 10px 40px rgba(17, 24, 39, 0.08);
  min-width: 360px;
}

.login-brand {
  align-items: center;
  display: flex;
  gap: 10px;
  margin-bottom: 20px;
}

.login-brand h1 {
  font-size: 20px;
  margin: 0;
}

.brand-dot {
  background: #1677ff;
  border-radius: 4px;
  display: inline-block;
  height: 18px;
  width: 18px;
}
</style>
