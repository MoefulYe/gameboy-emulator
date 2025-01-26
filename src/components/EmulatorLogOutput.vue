<template>
  <div class="flex-(~ col) max-h-full text-gray-8">
    <div class="border-b flex items-center justify-between text-lg lg:text-xl p-2">
      <span> Title </span>
      <VTooltip>
        <span
          class="i-solar:trash-bin-minimalistic-2-outline transition-colors duration-200 ease-in-out hover:text-blue-3 text-xl lg:text-2xl"
          @click="logs.splice(0)"
        />
        <template #popper>
          <span class="font-mono"> Clear </span>
        </template>
      </VTooltip>
    </div>
    <div
      class="overflow-y-auto whitespace-pre-wrap break-words p-2 text-lg lg:text-xl"
      ref="logContainer"
    >
      <p v-for="{ id, level, msg } of logs" :key="id" class="my-2">
        <span :class="['rounded-lg me-0.5 p-1 text-white', LOG_STYLE[level]]">{{
          LOG_STR[level]
        }}</span
        ><span class="p-1">{{ msg }}</span>
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useEmulator } from '@/emulator'
import { LogLevel } from '@/emulator/worker/event'
import { nextTick, onUpdated, shallowRef, reactive } from 'vue'
let id = 0
const emu = useEmulator()
const logs = reactive<Log[]>([])
const logContainer = shallowRef<HTMLDivElement>()

emu.useListener('log', (level, msg) => logs.push({ level, msg, id: id++ }))
onUpdated(async () => {
  if (logContainer.value) {
    await nextTick()
    logContainer.value.scrollTo(0, logContainer.value.scrollHeight)
  }
})
</script>
<script lang="ts">
type Log = {
  level: LogLevel
  msg: string
  id: number
}

const LOG_STYLE = [
  'bg-gray-4', // OFF
  'bg-red-4', // ERROR,
  'bg-yellow-4', // WARN,
  'bg-green-4', // INFO
  'bg-blue-4' // DEBUG
] as const

export const LOG_STR = ['off', 'error', 'warn', 'info', 'debug'] as const
</script>
