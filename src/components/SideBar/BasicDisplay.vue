<template>
  <ul class="p-1">
    <li>
      <label>state: </label> <span>{{ STATE_STR[state] }}</span>
    </li>
    <li>
      <label>freq: </label> <span>{{ Math.floor(freqHz) }}HZ</span>
    </li>
    <li>
      <label>scale: </label>
      <span>{{ freqScale.toFixed(2) }}</span>
    </li>
    <li>
      <label>frame-rate: </label>
      <span>{{ frameRate.toFixed(2) }}</span>
    </li>
    <li>
      <label>actual-rate: </label>
      <span>{{ actualRate.toFixed(2) }}</span>
    </li>
    <li>
      <label>cycles: </label> <span>{{ cycles }}</span>
    </li>
    <li>
      <label>volume: </label><span>{{ volume }}</span>
    </li>
  </ul>
</template>

<script setup lang="ts">
import { useEmulator } from '@/emulator'
import { STATE_STR } from '@/emulator/constants'
const emu = useEmulator()
const { volume, freqHz, freqScale } = emu.config
const { cycles, state, frameRate, actualRate } = emu.stat
</script>
