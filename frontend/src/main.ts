import { createApp } from 'vue'
import Antd from 'ant-design-vue'
import 'ant-design-vue/dist/reset.css'
import App from './App.vue'
import router from './router'
import { store } from './store'
import { i18n } from './i18n'
import './styles/index.css'

createApp(App).use(store).use(router).use(i18n).use(Antd).mount('#app')
