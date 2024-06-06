<template>
  <div class="text-base lg:text-xl p-4">
    <div>
      GameboyMode: <span>{{ 'sgb' }}</span>
    </div>
    <div>
      Frequence: <span>{{ freq.toFixed(0) }}HZ</span>
    </div>
    <div>
      State: <span>{{ state }}</span>
    </div>
    <div>
      Volume: <span>{{ volume }}%</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useEmulator } from '@/emulator'

const emu = useEmulator()
const freq = emu.freqHz
const state = emu.state
const volume = emu.volume
</script>
<i18n>

</i18n>
