import 'virtual:uno.css'
import '@unocss/reset/tailwind.css'
import 'floating-vue/dist/style.css'
import './assets/main.scss'
import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import i18n from './i18n'
import FloatingVue from 'floating-vue'

createApp(App).use(createPinia()).use(i18n).use(FloatingVue).mount('#app')
