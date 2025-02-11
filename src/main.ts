import 'virtual:uno.css'
import '@unocss/reset/tailwind.css'
import 'floating-vue/dist/style.css'
import './assets/main.scss'
import { createApp } from 'vue'
import App from './App.vue'
import FloatingVue from 'floating-vue'

createApp(App).use(FloatingVue).mount('#app')
