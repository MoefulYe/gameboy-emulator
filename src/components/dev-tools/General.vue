<template>
  <div>
    <div>
      {{ t('mode') }}: <span>{{ 'sgb' }}</span>
    </div>
    <div>
      {{ t('freq') }}: <span>{{ freqHz.toFixed(0) }}HZ</span>
    </div>
    <div>
      {{ t('cycles') }}: <span>{{ cycles }}</span>
    </div>
    <div>
      {{ t('state') }}: <span>{{ STATE_STR[state] }}</span>
    </div>
    <div>
      {{ t('volume') }}: <span>{{ volume }}%</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useEmulator } from '@/emulator'
import { useI18n } from 'vue-i18n'
import { STATE_STR } from '@/emulator/state'
const { t } = useI18n()
const { freqHz, state, volume, cycles } = useEmulator()
</script>

<i18n>
{
  "en": {
    "mode": "Mode",
    "freq": "Freq",
    "cycles": "Cycles",
    "state": "State",
    "volume": "Volume"
  },
  "zh": {
    "mode": "模式",
    "freq": "频率",
    "cycles": "周期",
    "state": "状态",
    "volume": "音量"
  }
}
</i18n>
