import { createRouter, createWebHistory } from 'vue-router'
import AppLayout from '@/layouts/AppLayout.vue'
import LoginPage from '@/pages/LoginPage.vue'
import GatewayListPage from '@/pages/GatewayListPage.vue'
import GatewayDetailPage from '@/pages/GatewayDetailPage.vue'
import CommandListPage from '@/pages/CommandListPage.vue'
import CommandDetailPage from '@/pages/CommandDetailPage.vue'
import { useAuthStore } from '@/store/auth'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', redirect: '/gateways' },
    { path: '/login', component: LoginPage, meta: { titleKey: 'auth.signIn' } },
    {
      path: '/',
      component: AppLayout,
      children: [
        { path: 'gateways', component: GatewayListPage, meta: { titleKey: 'gateway.listTitle' } },
        { path: 'gateways/:gatewayId', component: GatewayDetailPage, props: true, meta: { titleKey: 'gateway.detailTitle' } },
        { path: 'commands', component: CommandListPage, meta: { titleKey: 'command.title' } },
        { path: 'commands/:commandId', component: CommandDetailPage, props: true, meta: { titleKey: 'command.detailTitle' } },
      ],
    },
  ],
})

router.beforeEach((to) => {
  const auth = useAuthStore()
  if (to.path === '/login') {
    return auth.isAuthenticated ? '/gateways' : true
  }
  if (!auth.isAuthenticated) {
    return { path: '/login', query: { redirect: to.fullPath } }
  }
  return true
})

export default router
