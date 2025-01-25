<template>
  <div>
    <div>
      Mode: <span>{{ 'sgb' }}</span>
    </div>
    <div>
      Freq: <span>{{ freqHz.toFixed(0) }}HZ</span>
    </div>
    <div>
      Cycles: <span>{{ cycles }}</span>
    </div>
    <div>
      State: <span>{{ STATE_STR[state] }}</span>
    </div>
    <div>
      Volume: <span>{{ volume }}%</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useEmulator } from '@/emulator'
import { STATE_STR } from '@/emulator/state'
const { freqHz, state, volume, cycles } = useEmulator()
</script>