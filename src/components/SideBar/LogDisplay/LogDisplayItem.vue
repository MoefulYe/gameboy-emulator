<template>
  <div class="text-nowrap p-2">
    <span class="text-coolgray-50 rounded-md p-2" :class="[LOG_COLOR[source.level]]">{{
      LOG_STR[source.level]
    }}</span>
    {{ source.msg }}
  </div>
</template>

<script setup lang="ts">
import { type Log } from '@/emulator/logger'
import { type LogLevel, LOG_STR } from '@/emulator/constants'

const { source } = defineProps<{
  index: number
  source: Log
}>()
</script>

<script lang="ts">
const LOG_COLOR: Record<LogLevel, string> = [
  'bg-gray-3', // Off
  'bg-red-3', // error
  'bg-yellow-3', // warn
  'bg-green-3', // info
  'bg-blue-3' // debug
]
</script>
