<template>
  <ul class="p-1">
    <li>
      <label>state</label> <span>{{ STATE_STR[state] }}</span>
    </li>
    <li>
      <label>freq:</label> <span>{{ Math.floor(freqHz) }}</span>
      <span>(x{{ freqScale.toFixed(2) }})</span>
    </li>
    <li>
      <label>cycles</label> <span>{{ cycles }}</span>
    </li>
    <li>
      <label>volume: </label><span>{{ volume }}</span>
    </li>
    <li>
      <label>input-device: </label
      ><span @dblclick="toggleInputDevice">{{ INPUT_MODE_STR[gamepadMode] }}</span>
    </li>
  </ul>
</template>

<script setup lang="ts">
import { useEmulator } from '@/emulator'
import { STATE_STR } from '@/emulator/constants'
import { GamepadMode, INPUT_MODE_STR } from '@/emulator/input/gamepad'

const emu = useEmulator()
const { gamepadMode, volume, freqHz, freqScale } = emu.config
const { cycles, state } = emu.stat

const toggleInputDevice = () => {
  switch (gamepadMode.value) {
    case GamepadMode.Physical:
      gamepadMode.value = GamepadMode.Virtual
      break
    case GamepadMode.Virtual:
      gamepadMode.value = GamepadMode.Physical
      break
  }
}
</script>
