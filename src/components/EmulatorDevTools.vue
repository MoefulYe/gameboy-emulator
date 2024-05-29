<template>
  <div class="text-gray-7">
    <nav class="relative px-2 overflow-x-auto flex gap-4 text-lg lg:text-xl border-b">
      <button
        v-for="{ title, icon, idx } of tabs"
        :key="idx"
        :class="{
          'p-2 flex items-center gap-2': true,
          'border-b-2 border-blue-5 text-blue-5': idx === tab,
          'hover:(border-blue-3 text-blue-3)': idx !== tab
        }"
        @click="tab = idx"
      >
        <span :class="['text-xl lg:text-2xl', icon]" />
        <span>{{ t(title) }}</span>
      </button>
    </nav>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { defineComponent } from 'vue'
import { useI18n } from 'vue-i18n'
const { t } = useI18n()
const tab = ref(0)
</script>

<script lang="ts">
defineComponent
interface Tab {
  title: string
  icon: string
  idx: number
}

const tabs: Tab[] = [
  {
    title: 'general',
    icon: 'i-solar:settings-minimalistic-outline',
    idx: 0
  },
  {
    title: 'cart',
    icon: 'i-solar:info-square-outline',
    idx: 1
  },
  {
    title: 'cpu',
    icon: 'i-solar:cpu-outline',
    idx: 2
  }
]
</script>

<i18n>
{
  "en": {
    "general": "General",
    "cart": "Cartridge",
    "cpu": "CPU"
  },
  "zh": {
    "general": "概要",
    "cart": "卡带",
    "cpu": "CPU"
  }
}
</i18n>
