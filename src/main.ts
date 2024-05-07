import 'virtual:uno.css'
import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import { getEmulator } from './emulator'

createApp(App).use(createPinia()).mount('#app')

const emu = await getEmulator()
emu.step()
