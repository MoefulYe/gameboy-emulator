<template>
  <div class="w-full h-full flex flex-col min-h-screen">
    <HeaderBar />
    <main class="flex-grow p-4"></main>
  </div>
</template>

<script setup lang="ts">
import { getEmulator, emuKey } from '@/emulator'
import sleep from '@/utils/sleep'
import { provide } from 'vue'
import HeaderBar from './HeaderBar.vue'

const props = defineProps<{
  delay: number
}>()

const delay = sleep(props.delay)
const emulator = await getEmulator()
await delay
provide(emuKey, emulator)
</script>
