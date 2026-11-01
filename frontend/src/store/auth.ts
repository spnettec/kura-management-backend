import { defineStore } from 'pinia'
import { login as loginApi, logout as logoutApi, type LoginResponse } from '@/api/auth'

const STORAGE_KEY = 'kura-mgmt-auth'

interface AuthState {
  token: string
  username: string
  displayName: string
  tenantId: string
}

function loadStoredAuth(): AuthState | null {
  try {
    const raw = localStorage.getItem(STORAGE_KEY)
    if (!raw) return null
    const parsed = JSON.parse(raw) as AuthState
    if (!parsed.token) return null
    return parsed
  } catch {
    return null
  }
}

export const useAuthStore = defineStore('auth', {
  state: (): AuthState => loadStoredAuth() ?? { token: '', username: '', displayName: '', tenantId: '' },
  getters: {
    isAuthenticated: (state) => Boolean(state.token),
  },
  actions: {
    async login(username: string, password: string) {
      const result: LoginResponse = await loginApi({ username, password })
      this.token = result.token
      this.username = result.username
      this.displayName = result.displayName || result.username
      this.tenantId = result.tenantId
      localStorage.setItem(
        STORAGE_KEY,
        JSON.stringify({ token: this.token, username: this.username, displayName: this.displayName, tenantId: this.tenantId }),
      )
    },
    async logout() {
      try {
        await logoutApi()
      } catch {
        // token may already be expired; clearing local state is what matters
      }
      this.clear()
    },
    clear() {
      this.token = ''
      this.username = ''
      this.displayName = ''
      this.tenantId = ''
      localStorage.removeItem(STORAGE_KEY)
    },
  },
})
