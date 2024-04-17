import './assets/main.css'
import 'virtual:uno.css'
import { createApp } from 'vue'
import { createPinia } from 'pinia'
import init, { greet } from 'gameboy/pkg'

import App from './App.vue'
import router from './router'

createApp(App).use(createPinia()).use(router).mount('#app')

await init()
greet()
