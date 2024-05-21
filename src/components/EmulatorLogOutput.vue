<template>
  <div class="flex-(~ col) max-h-full text-gray-8">
    <div class="border-b flex items-center justify-between text-lg lg:text-xl p-4">
      <span>
        {{ t('title') }}
      </span>
      <VTooltip>
        <span
          class="i-mdi:trash transition-colors duration-200 ease-in-out hover:text-blue-3"
          @click="logs.splice(0)"
        />
        <template #popper>
          <span class="font-mono">
            {{ t('clear') }}
          </span>
        </template>
      </VTooltip>
    </div>
    <div class="overflow-y-auto whitespace-pre-wrap break-words p-2" ref="logContainer">
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
import { LogLevel } from '@/emulator/event'
import { nextTick, onUpdated, shallowRef, reactive } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
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

<i18n>
{
  "en": {
    "title": "Log Output",
    "clear": "Clear Log Output"
  },
  "zh": {
    "title": "日志输出",
    "clear": "清空日志输出"
  }
}
</i18n>

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
  'bg-blue-4', // DEBUG
  'bg-purple-4' // TRACE
] as const

export const LOG_STR = ['off', 'error', 'warn', 'info', 'debug', 'trace'] as const
</script>
